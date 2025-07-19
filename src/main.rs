#[derive(Debug)]
pub enum TokenizationError {
    NoStringDelimiterFound,
    UnrecognizedToken,
}
pub struct Scanner {}

impl Scanner {
    pub fn scan<'s>(src: &'s str) -> Result<Vec<Token<'s>>, TokenizationError> {
        let mut line = 0;
        let mut iter = src.char_indices().peekable();
        let mut tokens = Vec::new();
        while let Some((index, c)) = iter.next() {
            match c {
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => line += 1,
                '/' => {
                    let next_token = iter.peek();
                    if Self::next_token_is(next_token, '/') {
                        while let Some((_, c)) = iter.next()
                            && c != '\n'
                        {}
                        line += 1;
                        ()
                    } else {
                        let token = Self::scan_token(src, iter.clone(), &mut line, index, c)?;
                        tokens.push(token);
                    }
                }
                _ => {
                    let token = Self::scan_token(src, iter.clone(), &mut line, index, c)?;
                    tokens.push(token);
                }
            }
        }
        return Ok(tokens);
    }

    pub fn scan_token<'s>(
        src: &'s str,
        mut iter: std::iter::Peekable<std::str::CharIndices<'s>>,
        line: &mut usize,
        start: usize,
        c: char,
    ) -> Result<Token<'s>, TokenizationError> {
        match c {
            '(' => Ok(Token::LeftParen { line: *line }),
            ')' => Ok(Token::RightParen { line: *line }),
            '!' => {
                let next_token = iter.peek();
                if Self::next_token_is(next_token, '=') {
                    iter.next();
                    Ok(Token::BangEqual { line: *line })
                } else {
                    Ok(Token::Bang { line: *line })
                }
            }
            '"' => {
                while let Some((current, c)) = iter.next() {
                    if c == '\n' {
                        *line += 1;
                    }
                    if c == '"' {
                        return Ok(Token::String {
                            line: *line,
                            // do not include the double quotes themselves so schrink by 1 on both sides
                            lexeme: &src[start + 1..current],
                        });
                    }
                }
                Err(TokenizationError::NoStringDelimiterFound)
            }
            '/' => Ok(Token::Slash),
            _ => Err(TokenizationError::UnrecognizedToken),
        }
    }

    pub fn next_token_is(token: Option<&(usize, char)>, expected: char) -> bool {
        if let Some((_, c)) = token {
            return *c == expected;
        } else {
            return false;
        }
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
    fn simple_scanner_test() -> Result<(), TokenizationError> {
        let src = r#"
            (
"test"
                "😂" // hello this is a comment
                ! // hello this is another comment
                !=
                /
                " multi-line text is 
                also ok"
            )
        "#;
        let tokens = Scanner::scan(&src)?;
        print_tokens(tokens.as_slice());
        Ok(())
    }
}
