pub struct Error {
    pub line: u64,
    pub message: String,
    pub place: String,
}

pub fn report(e: Error) {
    eprintln!("[line {}] Error: {}: {}", e.line, e.place, e.message);
}
