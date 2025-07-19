#[derive(Debug)]
pub enum TokenizationError {
    NoStringDelimiterFound,
    UnrecognizedToken,
}
pub struct Scanner<'s> {
    src: &'s str,
    iter: std::iter::Peekable<std::str::CharIndices<'s>>,
    line: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            iter: src.char_indices().peekable(),
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
                let next_char = self.iter.peek();
                if Self::next_char_is(next_char, '=') {
                    self.iter.next();
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
                let next_char = self.iter.peek();
                if Self::next_char_is(next_char, '/') {
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
                while let Some((index, c)) = self.iter.peek() {
                    if !Self::is_digit(*c) {
                        return Ok(Token::Number {
                            line: self.line,
                            lexeme: &self.src[start..*index],
                        });
                    } else {
                        self.iter.next();
                    }
                }
                return Ok(Token::Number {
                    line: self.line,
                    lexeme: &self.src[start..],
                });
            }
            _ => Err(TokenizationError::UnrecognizedToken),
        }
    }

    pub fn next_char_is(c: Option<&(usize, char)>, expected: char) -> bool {
        if let Some((_, c)) = c {
            return *c == expected;
        } else {
            return false;
        }
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
    fn simple_scanner_test() -> Result<(), TokenizationError> {
        let src = r#"
            (
"test"
                "😂" // hello this is a comment
                ! // hello this is another comment
                !=
                /
                312455()
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
