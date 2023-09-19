use super::syntax_tree::{Binary, Expr, Grouping, Literal, Unary};
use super::tokens::{LiteralType, Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.r#match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.r#match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.r#match(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.r#match(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(Binary::new(expr, op, right)));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(Box::new(Unary::new(op, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.r#match(vec![TokenType::False]) {
            return Expr::Literal(Literal::new(LiteralType::Bool(false)));
        }
        if self.r#match(vec![TokenType::True]) {
            return Expr::Literal(Literal::new(LiteralType::Bool(true)));
        }
        if self.r#match(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::new(LiteralType::Nil));
        }

        if self.r#match(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(Literal::new(self.previous().literal.unwrap()));
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            //self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Expr::Grouping(Box::new(Grouping::new(expr)));
        }

        Expr::Literal(Literal::new(LiteralType::Nil))
    }

    fn r#match(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        matches!(self.peek().r#type, t)
    }

    fn advance(&mut self) -> Token {
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

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().to_owned()
    }
}
