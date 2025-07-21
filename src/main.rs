mod multipeek;
use crate::multipeek::multipeek;

#[derive(Debug)]
pub enum TokenizationError {
    NoStringDelimiterFound,
    UnrecognizedToken { line: usize },
}
pub struct Scanner<'s> {
    src: &'s str,
    iter: multipeek::MultiPeek<std::iter::Peekable<std::str::CharIndices<'s>>>,
    line: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            iter: multipeek(src.char_indices().peekable()),
            line: 0,
        }
    }
    pub fn scan(&mut self) -> Result<Vec<Token<'s>>, TokenizationError> {
        self.line = 0;
        let mut tokens = Vec::new();
        while let Some((index, c)) = self.iter.next() {
            match c {
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => self.line += 1,
                _ => {
                    let token = self.scan_token(index, c)?;
                    tokens.push(token);
                }
            }
        }
        return Ok(tokens);
    }

    pub fn scan_token(&mut self, start: usize, c: char) -> Result<Token<'s>, TokenizationError> {
        match c {
            '(' => Ok(Token::LeftParen { line: self.line }),
            ')' => Ok(Token::RightParen { line: self.line }),
            '!' => {
                if let Some(_) = self.next_if_char_is('=') {
                    Ok(Token::BangEqual { line: self.line })
                } else {
                    Ok(Token::Bang { line: self.line })
                }
            }
            '"' => {
                while let Some((current, c)) = self.iter.next() {
                    if c == '\n' {
                        self.line += 1;
                    }
                    if c == '"' {
                        return Ok(Token::String {
                            line: self.line,
                            // do not include the double quotes themselves so schrink by 1 on both sides
                            lexeme: &self.src[start..current + 1],
                        });
                    }
                }
                Err(TokenizationError::NoStringDelimiterFound)
            }
            '/' => {
                if let Some(_) = self.next_if_char_is('/') {
                    let line_comment = self.line;
                    while let Some((index, c)) = self.iter.next() {
                        if c == '\n' {
                            self.line += 1;
                            return Ok(Token::Comment {
                                line: line_comment,
                                lexeme: &self.src[start..index + 1],
                            });
                        }
                    }
                    return Ok(Token::Comment {
                        line: line_comment,
                        lexeme: &self.src[start..],
                    });
                } else {
                    Ok(Token::Slash)
                }
            }
            c if Self::is_digit(c) => {
                let mut last_index = start;
                while let Some(index) = self.next_if_char_is_f(Self::is_digit) {
                    last_index = index;
                }
                self.iter.reset_peek();
                if let Some(index) = self.next_if_char_is('.') {
                    last_index = index;
                    while let Some(index) = self.next_if_char_is_f(Self::is_digit) {
                        last_index = index;
                    }
                }
                return Ok(Token::Number {
                    line: self.line,
                    lexeme: &self.src[start..last_index + 1],
                });
            }
            _ => Err(TokenizationError::UnrecognizedToken { line: self.line }),
        }
    }

    pub fn next_if_char_is(&mut self, expected: char) -> Option<usize> {
        let index = match self.iter.peek() {
            Some((index, c)) => {
                if expected == *c {
                    Some(*index)
                } else {
                    None
                }
            }
            _ => None,
        };
        if index.is_some() {
            self.iter.next();
        }
        index
    }

    pub fn next_if_char_is_f<F>(&mut self, f: F) -> Option<usize>
    where
        F: Fn(char) -> bool,
    {
        let index = match self.iter.peek() {
            Some((index, c)) => {
                if f(*c) {
                    Some(*index)
                } else {
                    None
                }
            }
            _ => None,
        };
        if index.is_some() {
            self.iter.next();
        }
        index
    }

    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
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
    Number { line: usize, lexeme: &'s str },

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

    // No tokens
    Eof,
    Comment { line: usize, lexeme: &'s str },
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
    fn number_with_dot_test() -> Result<(), TokenizationError> {
        let src = r#"3234.1245"#;
        let mut scanner = Scanner::new(&src);
        let tokens = scanner.scan()?;
        print_tokens(tokens.as_slice());
        Ok(())
    }

    #[test]
    fn simple_scanner_test() -> Result<(), TokenizationError> {
        let src = r#"
            (
"test"
                "😂" // hello this is a comment
                ! // hello this is another comment
                !=
                /
                312455()
                3234.1245
                " multi-line text is 
                also ok"
            )
        //this is the last comment of the file"#;
        let mut scanner = Scanner::new(&src);
        let tokens = scanner.scan()?;
        print_tokens(tokens.as_slice());
        Ok(())
    }
}
