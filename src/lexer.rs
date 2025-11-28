use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(String),
    Keyword(String),
    Symbol(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let identifier = Regex::new(r"[A-Za-z0-9\-]+").unwrap();

    for line in input.lines() {
        for cap in identifier.find_iter(line) {
            let value = cap.as_str().to_uppercase();

            let token = match value.as_str() {
                "IDENTIFICATION" | "DIVISION" | "PROGRAM-ID" |
                "DISPLAY" | "STOP" | "RUN" => Token::Keyword(value),
                _ => Token::Identifier(value),
            };

            tokens.push(token);
        }
    }

    tokens
}
