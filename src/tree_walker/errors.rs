use super::tokens::{Token, TokenType};

pub struct Error {
    pub line: u64,
    pub message: String,
    pub place: String,
}

pub fn report(e: Error) {
    eprintln!("[line {}] Error: {}: {}", e.line, e.place, e.message);
}

pub fn error(token: &Token, message: &String) {
    if token.r#type == TokenType::Eof {
        let e = Error {
            line: token.line.to_owned(),
            place: " at the end".to_string(),
            message: message.to_owned(),
        };
        report(e)
    }

    let e = Error {
        line: token.line.to_owned(),
        message: message.to_owned(),
        place: " at '".to_string() + &token.lexeme.to_owned() + "'",
    };
    report(e)
}
