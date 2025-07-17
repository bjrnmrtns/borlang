pub struct Scanner<'s> {
    src: &'s str,
    iter: std::iter::Peekable<std::str::CharIndices<'s>>,
    tokens: Vec<Token<'s>>,
    line: usize,
    current: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            iter: src.char_indices().peekable(),
            tokens: Vec::new(),
            line: 0,
            current: 0,
        }
    }
    pub fn scan(&mut self) {
        while let Some(_) = self.iter.peek() {
            self.scan_token();
        }
    }
    pub fn scan_token(&mut self) {
        let (start, c) = self.iter.next().unwrap();
        match c {
            '(' => self.add_token(Token::LeftParen { line: self.line }),
            ')' => self.add_token(Token::RightParen { line: self.line }),
            '!' => {
                if let Some((current, _)) = self.match_next_token('=') {
                    self.current = current;
                    self.add_token(Token::BangEqual { line: self.line });
                } else {
                    self.add_token(Token::Bang { line: self.line });
                }
            }
            '"' => {
                while let Some((current, c)) = self.iter.next() {
                    self.current = current;
                    if c == '\n' {
                        self.line += 1;
                    }
                    if c == '"' {
                        break;
                    }
                }
                self.add_token(Token::String {
                    line: self.line,
                    // do not include the double quotes themselves so schrink by 1 on both sides
                    lexeme: &self.src[start + 1..self.current],
                });
            }
            '/' => {
                if let Some(_) = self.match_next_token('/') {
                    while let Some((_, c)) = self.iter.next()
                        && c != '\n'
                    {}
                    self.line += 1;
                } else {
                    self.add_token(Token::Slash);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            _ => (),
        }
    }

    pub fn add_token(&mut self, token: Token<'s>) {
        self.tokens.push(token);
    }

    pub fn match_next_token(&mut self, expected: char) -> Option<(usize, char)> {
        self.iter.next_if(|(_, c)| *c == expected)
    }
}

pub fn error(line: usize, message: &str) {
    println!("[line {}]: {}", line, message);
}

#[derive(Debug, PartialEq)]
pub enum Token<'s> {
    // Single
    LeftParen { line: usize },
    RightParen { line: usize },
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // Single/Double
    Bang { line: usize },
    BangEqual { line: usize },
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String { line: usize, lexeme: &'s str },
    Number,

    // Keywords
    And,
    Or,
    If,
    Else,
    True,
    False,
    Function,
    For,
    Print,
    Return,
    Let,
    While,
    Eof,
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
            (
"test"
                😂 // hello this is a comment
                ! // hello this is another comment
                !=
                /
                " multi-line text is 
                also ok"
            )
        "#;
        let mut scanner = Scanner::new(&src);
        scanner.scan();
        print_tokens(scanner.tokens.as_slice());
    }
}
