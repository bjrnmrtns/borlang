use std::io::{BufReader, Cursor, Read};

pub struct Scanner {}

impl Scanner {
    pub fn scan<R: Read>(&self, mut reader: R) -> std::io::Result<Vec<Token>> {
        let tokens = Vec::new();
        Ok(tokens)
    }
}

pub fn error(line: usize, message: &str) {
    println!("[line {}]: {}", line, message);
}

#[derive(Debug)]
pub struct TokenLoc {
    line: usize,
    start: usize,
    len: usize,
}

impl TokenLoc {
    pub fn new(line: usize, start: usize, len: usize) -> Self {
        Self { line, start, len }
    }
}

#[derive(Debug)]
pub enum Token {
    LeftParen(TokenLoc),
    RightParen(TokenLoc),
    LeftBrace(TokenLoc),
    RightBrace(TokenLoc),
    Comma(TokenLoc),
    Dot(TokenLoc),
    Minus(TokenLoc),
    Plus(TokenLoc),
    SemiColon(TokenLoc),
    Slash(TokenLoc),
    Star(TokenLoc),
    Equal(TokenLoc),
    EqualEqual(TokenLoc),
    Greater(TokenLoc),
    GreaterEqual(TokenLoc),
    Less(TokenLoc),
    LessEqual(TokenLoc),
    Identifier(TokenLoc),
    String(TokenLoc),
    Number(TokenLoc),
    And(TokenLoc),
    Struct(TokenLoc),
    If(TokenLoc),
    Else(TokenLoc),
    True(TokenLoc),
    False(TokenLoc),
    Function(TokenLoc),
    For(TokenLoc),
    Print(TokenLoc),
    Return(TokenLoc),
    Let(TokenLoc),
    While(TokenLoc),
    EOF(TokenLoc),
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_tokens(tokens: &[Token]) {
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn simple_scanner_test() {
        let src = r#"
    let x = 5;
    let name = "henk";
        "#;
        let scanner = Scanner {};
        let reader = Cursor::new(src.as_bytes());
        let tokens = scanner.scan(reader).expect("tokenization failed");
        print_tokens(tokens.as_slice());
    }
}
