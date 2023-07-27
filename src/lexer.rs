use std::fmt;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Num(i32),
    Plus,
}

#[derive(Debug)]
pub struct LexError(String);

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexical error: {}", self.0)
    }
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    lookahead: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
            lookahead: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.lookahead {
            self.lookahead = None;
            Some(c)
        } else {
            self.chars.next()
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        // If there are chars left, handle them
        if let Some(next_char) = self.next_char() {
            return match next_char {
                '+' => Ok(Some(Token::Plus)),
                '1'..='9' => {
                    let mut number = next_char.to_digit(10).unwrap() as i32;
                    let mut next = self.chars.next();

                    while let Some(next_char) = next {
                        if let Some(digit) = next_char.to_digit(10) {
                            number = number * 10 + digit as i32;
                            next = self.chars.next();
                        } else {
                            // Terminate loop and set lookahead
                            self.lookahead = Some(next_char);
                            break;
                        }
                    }
                    Ok(Some(Token::Num(number)))
                }
                ' ' => self.next_token(),
                _ => Err(LexError(format!(
                    "Invalid token starting with '{}'",
                    next_char
                ))),
            };
        }

        // Return a None representing EOF
        return Ok(None);
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_digit() {
        let mut lexer = Lexer::new("1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Num(1)]);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Num(123)]);
    }

    #[test]
    fn ignore_spaces() {
        let mut lexer = Lexer::new("2 57 120 1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Num(2),
                Token::Num(57),
                Token::Num(120),
                Token::Num(1)
            ]
        );
    }

    #[test]
    fn numbers_and_operators() {
        let mut lexer = Lexer::new("2+13");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Num(2), Token::Plus, Token::Num(13)]);
    }

    #[test]
    fn error() {
        let mut lexer = Lexer::new("2invalid");
        let tokens = lexer.tokenize();
        assert!(tokens.is_err());
    }
}
