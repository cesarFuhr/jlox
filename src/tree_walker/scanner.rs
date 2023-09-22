use std::collections::HashMap;
use std::fmt;

use super::errors::{report, Error};
use super::tokens::{LiteralType, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.extend([Token {
            r#type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            literal: None,
        }]);

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            ':' => self.add_token(TokenType::Colon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '>' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '<' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '/' => {
                if self.next_matches('/') {
                    // This is here to detect commented lines.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.next_matches('*') {
                    // This is here to detect commented blocks.
                    while !(self.is_at_end() || (self.peek() == '*' && self.peek_next() == '/')) {
                        self.advance();
                    }

                    // Advance twice to skip "*/"
                    self.advance();
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '"' => self.string(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    report(Error {
                        line: self.line,
                        message: fmt::format(format_args!("Unexpected character: {}", c)),
                        place: String::new(),
                    })
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, t: TokenType, l: Option<LiteralType>) {
        self.tokens.extend([Token {
            r#type: t,
            lexeme: self.source[self.start..self.current].to_string(),
            line: self.line,
            literal: l,
        }]);
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            report(Error {
                line: self.line,
                message: fmt::format(format_args!("Unterminated string")),
                place: String::new(),
            })
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();

        self.add_token(TokenType::String, Some(LiteralType::String(value)));
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) {
        while !self.is_at_end() && Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while !self.is_at_end() && Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        while !self.is_at_end() && Scanner::is_digit(self.peek()) {
            self.advance();
        }

        let literal = self.source[self.start..self.current].to_string();
        let value: f64 = literal.parse().unwrap();
        self.add_token(TokenType::Number, Some(LiteralType::Number(value)));
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let tkn_type = self
            .keywords
            .get(&text)
            .unwrap_or(&TokenType::Identifier)
            .to_owned();

        self.add_token(tkn_type, None);
    }
}

#[cfg(test)]
mod test {
    use crate::tree_walker::tokens::*;

    use super::Scanner;

    #[test]
    fn grouping() {
        let mut scanner = Scanner::new("(()){}".to_string());

        assert_eq!(
            scanner.scan_tokens(),
            vec![
                Token {
                    r#type: TokenType::LeftParen,
                    lexeme: "(".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::LeftParen,
                    lexeme: "(".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::RightParen,
                    lexeme: ")".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::RightParen,
                    lexeme: ")".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::LeftBrace,
                    lexeme: "{".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::RightBrace,
                    lexeme: "}".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1,
                    literal: None,
                },
            ]
        );
    }

    #[test]
    fn operators() {
        let mut scanner = Scanner::new("!*+-/=<> <= ==".to_string());

        assert_eq!(
            scanner.scan_tokens(),
            vec![
                Token {
                    r#type: TokenType::Bang,
                    lexeme: "!".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Star,
                    lexeme: "*".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Plus,
                    lexeme: "+".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Slash,
                    lexeme: "/".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Equal,
                    lexeme: "=".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Less,
                    lexeme: "<".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Greater,
                    lexeme: ">".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::LessEqual,
                    lexeme: "<=".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::EqualEqual,
                    lexeme: "==".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1,
                    literal: None,
                },
            ]
        );
    }

    #[test]
    fn string_literals() {
        let mut scanner = Scanner::new("\"this is a string literal\"()".to_string());

        assert_eq!(
            scanner.scan_tokens(),
            vec![
                Token {
                    r#type: TokenType::String,
                    lexeme: "\"this is a string literal\"".to_string(),
                    line: 1,
                    literal: Some(LiteralType::String("this is a string literal".to_string())),
                },
                Token {
                    r#type: TokenType::LeftParen,
                    lexeme: "(".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::RightParen,
                    lexeme: ")".to_string(),
                    line: 1,
                    literal: None,
                },
                Token {
                    r#type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1,
                    literal: None,
                },
            ]
        );
    }

    #[test]
    fn number_literals() {
        let mut scanner = Scanner::new("123.45".to_string());

        assert_eq!(
            scanner.scan_tokens(),
            vec![
                Token {
                    r#type: TokenType::Number,
                    lexeme: "123.45".to_string(),
                    line: 1,
                    literal: Some(LiteralType::Number(123.45)),
                },
                Token {
                    r#type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1,
                    literal: None,
                },
            ]
        );
    }

    #[test]
    fn ingnores_comment_blocks() {
        let mut scanner = Scanner::new("/* \nthis \nis \na \ncomment */123.45".to_string());

        assert_eq!(
            scanner.scan_tokens(),
            vec![
                Token {
                    r#type: TokenType::Number,
                    lexeme: "123.45".to_string(),
                    line: 1,
                    literal: Some(LiteralType::Number(123.45)),
                },
                Token {
                    r#type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1,
                    literal: None,
                },
            ]
        );
    }
}
