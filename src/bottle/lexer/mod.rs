use log::*;
use serde_json::error;
use std::fs::File;
use std::io::{BufReader, Read};
pub mod tools;

pub struct Lexer {
    location: String,
    ntoken: tools::Token,
    identifier: bool,
    reader: tools::LineReader,
    buf: String,
    target: File,
}

impl Lexer {
    fn error(&mut self, msg: &str) {
        error!("{:?}: {}", self.reader.get_loc(), msg);
    }
    pub fn from_file(input: &File) -> Result<Lexer, &str> {
        let owned_file_handle = match input.try_clone() {
            Ok(f) => f,
            Err(_) => return Err("Failed to clone File Handle for Lexer!(@bottle::lexer)".into()),
        };
        let reader_handle = match owned_file_handle.try_clone() {
            Ok(f) => f,
            Err(_) => {
                return Err("Failed to clone File Handle for BufReader!(@bottle::lexer)".into())
            }
        };
        Ok(Lexer {
            location: String::new(),
            ntoken: tools::Token::None,
            identifier: false,
            reader: tools::LineReader::new(reader_handle).unwrap(),
            buf: String::new(),
            target: owned_file_handle,
        })
    }
    fn peek_to(&mut self, c: char) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.by_ref().peekable().peek() {
            match next {
                Ok(ch) => {
                    if *ch as char == c {
                        return Some(buf);
                    } else {
                        buf.push(*ch as char);
                    }
                }
                Err(_) => return None,
            }
        }
        return None;
    }
    ///returns the string until the character c is found, inclusively.
    fn next_to(&mut self, c: char) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.by_ref() {
            match next {
                Ok(ch) => {
                    if ch as char == c {
                        buf.push(ch as char);
                        return Some(buf);
                    } else {
                        buf.push(ch as char);
                    }
                }
                Err(_) => return None,
            }
        }
        return None;
    }
    ///returns the string until the character c is found, exclusively.
    fn next_to_exclusive(&mut self, c: char) -> Option<String> {
        let mut buf = String::new();
        let mut it = self.reader.by_ref().peekable();
        loop {
            let next = it.peek();
            match next {
                Some(s) => match s {
                    Ok(ch) => {
                        if *ch as char == c {
                            return Some(buf);
                        } else {
                            buf.push(*ch as char);
                        }
                        it.next();
                    }
                    Err(_) => return None,
                },
                None => return None,
            }
        }
    }
    ///returns the string until a reserved character is found.
    fn next_to_reserved(&mut self) -> Option<String> {
        let mut buf = String::new();
        let mut it = self.reader.by_ref().peekable();
        loop {
            let next = it.peek();
            match next {
                Some(s) => match s {
                    Ok(ch) => {
                        if tools::is_reserved(*ch as char) {
                            return Some(buf);
                        } else {
                            buf.push(*ch as char);
                        }
                        it.next();
                    }
                    Err(_) => return None,
                },
                None => return None,
            }
        }
    }
    fn next_line(&mut self) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.by_ref() {
            match next {
                Ok(ch) => {
                    if ch as char == '\n' {
                        return Some(buf);
                    } else {
                        buf.push(ch as char);
                    }
                }
                Err(_) => return None,
            }
        }
        return None;
    }
}

impl Iterator for Lexer {
    type Item = Result<tools::Token, &'static str>;
    fn next(&mut self) -> Option<Self::Item> {
        //outputs next token in the input provided at Lexer::new()
        match self.reader.next() {
            Some(res) => {
                let c = match res {
                    Ok(ch) => ch as char,
                    Err(_) => {
                        return Some(Err(
                            "Failed to read character from file!(@bottle::lexer)".into()
                        ))
                    }
                };
                match c {
                    '#' => {
                        return match self.next_line() {
                            Some(s) => Some(Ok(tools::Token::Sharp(s))),
                            None => {
                                Some(Err("Failed to read character from file!(@bottle::lexer)"))
                            }
                        }
                    }
                    '@' => match self.next_line() {
                        Some(s) => {
                            return Some(Ok(tools::Token::At(match s.strip_suffix("\n") {
                                Some(s) => s.to_string(),
                                None => s.to_string(),
                            })))
                        }
                        None => {
                            return Some(Err("Failed to read character from file!(@bottle::lexer)"))
                        }
                    },
                    '(' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::OpenParen))),
                    ')' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::CloseParen))),
                    ',' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Comma))),
                    '{' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::OpenBrace))),
                    '}' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::CloseBrace))),
                    '.' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Dot))),
                    '*' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Star))),
                    '+' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Plus))),
                    '-' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Minus))),
                    '/' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Slash))),
                    '%' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Percent))),
                    '=' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Equal))),
                    '!' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Bang))),
                    ';' => return Some(Ok(tools::Token::Simple(tools::SimpleToken::Semicolon))),
                    '0'..='9' => match self.next_to_reserved() {
                        Some(s) => match s.parse() {
                            Ok(n) => {
                                if self.reader.by_ref().peekable().peek() == Some(&Ok('.')) {
                                    self.reader.next();
                                    match self.next_to_reserved() {
                                        Some(s) => match s.parse::<f64>() {
                                            Ok(f) => {
                                                return Some(Ok(tools::Token::LiteralFloat(
                                                    n as f64 + f,
                                                )));
                                            }
                                            Err(_) => {
                                                self.error(
                                                    "Invalid number literal!(@bottle::lexer)",
                                                );
                                                return None;
                                            }
                                        },
                                        None => {
                                            self.error("Invalid number literal!(@bottle::lexer)");
                                            return None;
                                        }
                                    }
                                } else {
                                    return Some(Ok(tools::Token::LiteralInt(n)));
                                }
                            }
                            Err(_) => {
                                self.error("Invalid number literal!(@bottle::lexer)");
                                return None;
                            }
                        },
                        None => {
                            self.error("Invalid number literal!(@bottle::lexer)");
                            return None;
                        }
                    },
                    '"' => {
                        return match self.next_to('"') {
                            Some(s) => Some(Ok(tools::Token::StringLiteral(s))),
                            None => {
                                self.error("Failed to parse String Literal!(@bottle::lexer)");
                                return None;
                            }
                        }
                    }
                    c => {
                        if c.is_alphabetic() || c == '_' {
                            self.buf = match self.next_to_reserved() {
                                Some(s) => s,
                                None => {
                                    self.error("Failed to read identifier!(@bottle::lexer)");
                                    return None;
                                }
                            };
                            match self.buf.as_str() {
                                "for" => return Some(Ok(tools::Token::For)),
                                "while" => return Some(Ok(tools::Token::While)),
                                "if" => return Some(Ok(tools::Token::If)),
                                "else" => return Some(Ok(tools::Token::Else)),
                                "return" => return Some(Ok(tools::Token::Return)),
                                "break" => return Some(Ok(tools::Token::Break)),
                                "continue" => return Some(Ok(tools::Token::Continue)),
                                "match" => return Some(Ok(tools::Token::Match)),
                                "case" => return Some(Ok(tools::Token::Case)),
                                "true" => return Some(Ok(tools::Token::LiteralBool(true))),
                                "false" => return Some(Ok(tools::Token::LiteralBool(false))),
                                "null" => return Some(Ok(tools::Token::LiteralNull)),
                                "depend" => {
                                    return Some(Ok(tools::Token::Depend(
                                        self.reader.remaining_line(),
                                    )))
                                }
                                "require" => {
                                    return Some(Ok(tools::Token::Require(
                                        self.reader.remaining_line(),
                                    )))
                                }
                                "import" => {
                                    return Some(Ok(tools::Token::Import(
                                        self.reader.remaining_line(),
                                    )))
                                }
                                "public" => return Some(Ok(tools::Token::Public)),
                                "private" => return Some(Ok(tools::Token::Private)),
                                "protected" => return Some(Ok(tools::Token::Protected)),
                                s => return Some(Ok(tools::Token::Identifier(s.to_string()))),
                            }
                        } else {
                            return Some(Err("Unrecognized tools::Token!(@bottle::lexer)"));
                        }
                    }
                }
            }
            None => {
                if self.buf.len() == 0 {
                    return Some(Ok(tools::Token::Eof));
                } else {
                    return None;
                }
            }
        }
    }
}
