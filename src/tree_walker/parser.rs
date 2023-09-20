use super::errors::error;
use super::syntax_tree::{Binary, Expr, Grouping, Literal, Unary};
use super::tokens::{LiteralType, Token, TokenType};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        if let Ok(expr) = self.expression() {
            return Some(expr);
        }

        None
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
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
