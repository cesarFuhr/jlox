use super::errors::error;
use super::syntax_tree::{Binary, Expr, Grouping, Literal, Ternary, Unary};
use super::tokens::{LiteralType, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        if let Ok(expr) = self.expression() {
            return Some(expr);
        }

        None
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.r#match(vec![TokenType::Comma]) {
            let op = self.previous()?;
            let right = self.ternary()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.r#match(vec![TokenType::Question]) {
            let condition = expr;
            let then = self.ternary()?;

            // Consume until we find a colon (should be only once).
            // Should this be another while?
            let _ = self.consume(
                TokenType::Colon,
                "Expect ':' after ternary condition.".to_string(),
            );

            let r#else = self.ternary()?;

            expr = Expr::Ternary(Box::new(Ternary::new(condition, then, r#else)))
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous()?;
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous()?;
            let right = self.term()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous()?;
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous()?;
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous()?;
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(Unary::new(op, right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.r#match(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Bool(false))));
        }
        if self.r#match(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Bool(true))));
        }
        if self.r#match(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::new(LiteralType::Nil)));
        }

        if self.r#match(vec![TokenType::Number, TokenType::String]) {
            let prev = self.previous()?;
            return Ok(Expr::Literal(Literal::new(prev.literal.unwrap())));
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(
                TokenType::RightParen,
                "Expect ')' after expression".to_string(),
            )?;
            return Ok(Expr::Grouping(Box::new(Grouping::new(expr))));
        }

        let e = Parser::error(self.peek(), "expect expression".to_string());
        Err(e)
    }

    fn r#match(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                let _ = self.advance();
                return true;
            }
        }

        false
    }

    fn synchronize(&mut self) -> Result<(), ParseError> {
        let _ = self.advance();

        while !self.is_at_end() {
            let current = self.previous()?;
            if current.r#type == TokenType::Semicolon {
                return Ok(());
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return Ok(()),
                _ => (),
            }

            let _ = self.advance();
        }

        Ok(())
    }

    fn consume(&mut self, t: TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(t) {
            return self.advance();
        }

        let e = Parser::error(self.peek(), message);
        panic!("{:?}", e);
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        self.peek().r#type == t
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        matches!(self.peek().r#type, TokenType::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.get(self.current).unwrap().to_owned()
    }

    fn previous(&self) -> Result<Token, ParseError> {
        let token = self.tokens.get(self.current - 1);

        if token.is_none() {
            return Err(ParseError {
                token: Token {
                    r#type: TokenType::Nil,
                    lexeme: "".to_string(),
                    literal: None,
                    line: 0,
                },
                message: "unexpected absense of token".to_string(),
            });
        }

        Ok(token.unwrap().to_owned())
    }

    fn error(token: Token, message: String) -> ParseError {
        error(&token, &message);
        ParseError { token, message }
    }
}

#[cfg(test)]
mod test {
    use crate::tree_walker::scanner::Scanner;

    use super::*;

    #[test]
    fn grouping_unary() {
        let tokens = Scanner::new("(-1)".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Grouping(Box::new(Grouping::new(Expr::Unary(Box::new(Unary::new(
            Token {
                line: 1,
                lexeme: "-".to_string(),
                r#type: TokenType::Minus,
                literal: None,
            },
            Expr::Literal(Literal::new(LiteralType::Number(1.0))),
        ))))));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn comma_separated_expressions() {
        let tokens = Scanner::new("1+1,1-1,1==1".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Binary(Box::new(Binary::new(
            Expr::Binary(Box::new(Binary::new(
                Expr::Binary(Box::new(Binary::new(
                    Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                    Token {
                        line: 1,
                        lexeme: "+".to_string(),
                        r#type: TokenType::Plus,
                        literal: None,
                    },
                    Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                ))),
                Token {
                    line: 1,
                    lexeme: ",".to_string(),
                    r#type: TokenType::Comma,
                    literal: None,
                },
                Expr::Binary(Box::new(Binary::new(
                    Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                    Token {
                        line: 1,
                        lexeme: "-".to_string(),
                        r#type: TokenType::Minus,
                        literal: None,
                    },
                    Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                ))),
            ))),
            Token {
                line: 1,
                lexeme: ",".to_string(),
                r#type: TokenType::Comma,
                literal: None,
            },
            Expr::Binary(Box::new(Binary::new(
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                Token {
                    line: 1,
                    lexeme: "==".to_string(),
                    r#type: TokenType::EqualEqual,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
            ))),
        )));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn grouping_plus() {
        let tokens = Scanner::new("(1+1)".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Grouping(Box::new(Grouping::new(Expr::Binary(Box::new(
            Binary::new(
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                Token {
                    line: 1,
                    lexeme: "+".to_string(),
                    r#type: TokenType::Plus,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
            ),
        )))));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn ternary() {
        let tokens = Scanner::new("1 == 1 ? 2 : 3".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Ternary(Box::new(Ternary::new(
            Expr::Binary(Box::new(Binary::new(
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                Token {
                    line: 1,
                    lexeme: "==".to_string(),
                    r#type: TokenType::EqualEqual,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
            ))),
            Expr::Literal(Literal::new(LiteralType::Number(2.0))),
            Expr::Literal(Literal::new(LiteralType::Number(3.0))),
        )));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn ternary_complex() {
        let tokens = Scanner::new("5 * 20 == 99 ? 10 : 3 < 2 ? 1 : 0".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Ternary(Box::new(Ternary::new(
            Expr::Binary(Box::new(Binary::new(
                Expr::Binary(Box::new(Binary::new(
                    Expr::Literal(Literal::new(LiteralType::Number(5.0))),
                    Token {
                        line: 1,
                        lexeme: "*".to_string(),
                        r#type: TokenType::Star,
                        literal: None,
                    },
                    Expr::Literal(Literal::new(LiteralType::Number(20.0))),
                ))),
                Token {
                    line: 1,
                    lexeme: "==".to_string(),
                    r#type: TokenType::EqualEqual,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(99.0))),
            ))),
            Expr::Literal(Literal::new(LiteralType::Number(10.0))),
            Expr::Ternary(Box::new(Ternary::new(
                Expr::Binary(Box::new(Binary::new(
                    Expr::Literal(Literal::new(LiteralType::Number(3.0))),
                    Token {
                        line: 1,
                        lexeme: "<".to_string(),
                        r#type: TokenType::Less,
                        literal: None,
                    },
                    Expr::Literal(Literal::new(LiteralType::Number(2.0))),
                ))),
                Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                Expr::Literal(Literal::new(LiteralType::Number(0.0))),
            ))),
        )));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn equality() {
        let tokens = Scanner::new("1 == 1".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Binary(Box::new(Binary::new(
            Expr::Literal(Literal::new(LiteralType::Number(1.0))),
            Token {
                line: 1,
                lexeme: "==".to_string(),
                r#type: TokenType::EqualEqual,
                literal: None,
            },
            Expr::Literal(Literal::new(LiteralType::Number(1.0))),
        )));

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn complex_grouping() {
        let tokens = Scanner::new("(1+10)/10+2 < 10*2".to_string()).scan_tokens();

        let mut parser = Parser::new(tokens);
        let expected = Expr::Binary(Box::new(Binary::new(
            Expr::Binary(Box::new(Binary::new(
                Expr::Binary(Box::new(Binary::new(
                    Expr::Grouping(Box::new(Grouping::new(Expr::Binary(Box::new(
                        Binary::new(
                            Expr::Literal(Literal::new(LiteralType::Number(1.0))),
                            Token {
                                line: 1,
                                lexeme: "+".to_string(),
                                r#type: TokenType::Plus,
                                literal: None,
                            },
                            Expr::Literal(Literal::new(LiteralType::Number(10.0))),
                        ),
                    ))))),
                    Token {
                        line: 1,
                        lexeme: "/".to_string(),
                        r#type: TokenType::Slash,
                        literal: None,
                    },
                    Expr::Literal(Literal::new(LiteralType::Number(10.0))),
                ))),
                Token {
                    line: 1,
                    lexeme: "+".to_string(),
                    r#type: TokenType::Plus,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(2.0))),
            ))),
            Token {
                line: 1,
                lexeme: "<".to_string(),
                r#type: TokenType::Less,
                literal: None,
            },
            Expr::Binary(Box::new(Binary::new(
                Expr::Literal(Literal::new(LiteralType::Number(10.0))),
                Token {
                    line: 1,
                    lexeme: "*".to_string(),
                    r#type: TokenType::Star,
                    literal: None,
                },
                Expr::Literal(Literal::new(LiteralType::Number(2.0))),
            ))),
        )));

        let actual = parser.parse().unwrap();

        assert_eq!(actual, expected);
    }
}
