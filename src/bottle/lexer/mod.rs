use log::*;
use std::fs::File;
pub mod tools;

pub struct Lexer {
    reader: tools::LineReader,
    buf: String,
    target: std::path::PathBuf,
}

impl Lexer {
    fn error(&mut self, msg: &str) {
        error!("{:?}: {}", self.reader.get_loc(), msg);
    }
    pub fn from_path<P: AsRef<std::path::Path>>(input: P) -> Result<Lexer, String> {
        let path = input.as_ref().to_path_buf();
        let owned_file_handle = match File::open(input.as_ref()) {
            Ok(f) => f,
            Err(_) => return Err("Failed to open file!(@bottle::lexer)".to_owned()),
        };
        Ok(Lexer {
            reader: tools::LineReader::new(owned_file_handle).unwrap(),
            buf: String::new(),
            target: path.to_path_buf(),
        })
    }
    fn peek_to(&mut self, c: char) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.peek() {
            match next {
                Ok(ch) => {
                    if { ch } == c {
                        return Some(buf);
                    } else {
                        buf.push(ch);
                    }
                }
                Err(_) => return None,
            }
        }
        None
    }
    ///returns the string until the character c is found, inclusively.
    fn next_to(&mut self, c: char) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.by_ref() {
            match next {
                Ok(ch) => {
                    if ch == c {
                        buf.push(ch);
                        return Some(buf);
                    } else {
                        buf.push(ch);
                    }
                }
                Err(_) => return None,
            }
        }
        None
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
                        if { *ch } == c {
                            return Some(buf);
                        } else {
                            buf.push(*ch);
                        }
                        it.next();
                    }
                    Err(_) => return None,
                },
                None => return None,
            }
        }
    }
    /// Returns the string until a reserved character is found.
    fn next_to_reserved(&mut self) -> Option<String> {
        let mut buf = String::new();
        let mut it = self.reader.by_ref().peekable();
        loop {
            let next = it.peek();
            match next {
                Some(s) => match s {
                    Ok(ch) => {
                        if tools::is_reserved(*ch) {
                            let _tmp = self.reader.by_ref().last();
                            return Some(buf);
                        } else {
                            buf.push(*ch);
                            it.next(); // Consume the character only if it's not reserved
                        }
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        return None;
                    }
                },
                None => {
                    it.next();
                    return self.next_to_reserved();
                }
            }
        }
    }
    fn next_line(&mut self) -> Option<String> {
        let mut buf = String::new();
        for next in self.reader.by_ref() {
            match next {
                Ok(ch) => {
                    if ch == '\n' {
                        return Some(buf);
                    } else {
                        buf.push(ch);
                    }
                }
                Err(_) => return None,
            }
        }
        None
    }
}

impl Iterator for Lexer {
    type Item = Result<tools::Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        //outputs next token in the input provided at Lexer::new()
        match self.reader.next() {
            Some(res) => {
                let c = match res {
                    Ok(ch) => ch,
                    Err(_) => {
                        return Some(Err(
                            "Failed to read character from file!(@bottle::lexer)".to_string()
                        ))
                    }
                };
                match c {
                    '#' => match self.next_line() {
                        Some(s) => Some(Ok(tools::Token::Sharp(s))),
                        None => Some(Err(
                            "Failed to read character from file!(@bottle::lexer)".to_string()
                        )),
                    },
                    '@' => match self.next_line() {
                        Some(s) => {
                            return Some(Ok(tools::Token::At(match s.strip_suffix('\n') {
                                Some(s) => s.to_string(),
                                None => s.to_string(),
                            })))
                        }
                        None => Some(Err(
                            "Failed to read character from file!(@bottle::lexer)".to_string()
                        )),
                    },
                    '(' => Some(Ok(tools::Token::Simple(tools::SimpleToken::OpenParen))),
                    ')' => Some(Ok(tools::Token::Simple(tools::SimpleToken::CloseParen))),
                    ',' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Comma))),
                    '{' => Some(Ok(tools::Token::Simple(tools::SimpleToken::OpenBrace))),
                    '}' => Some(Ok(tools::Token::Simple(tools::SimpleToken::CloseBrace))),
                    '[' => Some(Ok(tools::Token::Simple(tools::SimpleToken::OpenBracket))),
                    ']' => Some(Ok(tools::Token::Simple(tools::SimpleToken::CloseBracket))),
                    '.' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Dot))),
                    '*' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Star))),
                    '+' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Plus))),
                    '-' => {
                        if match self.reader.peek().unwrap() {
                            Ok(ch) => ch,
                            Err(_) => ' ',
                        } == '>'
                        {
                            self.reader.next();
                            return Some(Ok(tools::Token::Simple(tools::SimpleToken::RtArrow)));
                        }
                        return Some(Ok(tools::Token::Simple(tools::SimpleToken::Minus)));
                    }
                    '/' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Slash))),
                    '%' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Percent))),
                    '=' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Equal))),
                    '!' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Bang))),
                    ';' => Some(Ok(tools::Token::Simple(tools::SimpleToken::Semicolon))),
                    '0'..='9' => {
                        self.buf = c.to_string();
                        while self.reader.peek().unwrap().unwrap().is_numeric() || self.reader.peek().unwrap().unwrap() == '.' {
                            self.buf.push(self.reader.next().unwrap().unwrap());
                        }
                        match self.buf.parse::<i64>() {
                            Ok(n) => Some(Ok(tools::Token::LiteralInt(n))),
                            Err(_) => match self.buf.parse::<f64>() {
                                Ok(f) => Some(Ok(tools::Token::LiteralFloat(f))),
                                Err(_) => {
                                    self.error(format!("Invalid float: {}",self.buf).as_str());
                                    Some(Err("Invalid Float".to_string()))
                                }
                            },
                        }
                    }
                    '"' => match self.next_to_exclusive('"') {
                        Some(s) => Some(Ok(tools::Token::StringLiteral(s))),
                        None => {
                            self.error("Failed to parse String Literal!(@bottle::lexer)");
                            Some(Err("Invalid String Literal!".to_string()))
                        }
                    },
                    c => {
                        if c.is_alphabetic() || c == '_' {
                            self.buf = c.to_string();
                            let next_reserved = match self.next_to_reserved() {
                                Some(s) => s,
                                None => {
                                    self.error("Failed to read identifier!");
                                    return Some(Err(
                                        "EOF encountered while searching for identifier"
                                            .to_string(),
                                    ));
                                }
                            };
                            self.buf.push_str(&next_reserved);
                            match self.buf.as_str() {
                                "let" => Some(Ok(tools::Token::Let)),
                                "main" => Some(Ok(tools::Token::Main)),
                                "for" => Some(Ok(tools::Token::For)),
                                "while" => Some(Ok(tools::Token::While)),
                                "if" => Some(Ok(tools::Token::If)),
                                "else" => Some(Ok(tools::Token::Else)),
                                "return" => Some(Ok(tools::Token::Return)),
                                "break" => Some(Ok(tools::Token::Break)),
                                "continue" => Some(Ok(tools::Token::Continue)),
                                "match" => Some(Ok(tools::Token::Match)),
                                "case" => Some(Ok(tools::Token::Case)),
                                "true" => Some(Ok(tools::Token::LiteralBool(true))),
                                "false" => Some(Ok(tools::Token::LiteralBool(false))),
                                "null" => Some(Ok(tools::Token::LiteralNull)),
                                "depend" => {
                                    Some(Ok(tools::Token::Depend(self.reader.remaining_line())))
                                }
                                "require" => {
                                    Some(Ok(tools::Token::Require(self.reader.remaining_line())))
                                }
                                "import" => {
                                    Some(Ok(tools::Token::Import(self.reader.remaining_line())))
                                }
                                "public" => Some(Ok(tools::Token::Public)),
                                "private" => Some(Ok(tools::Token::Private)),
                                "protected" => Some(Ok(tools::Token::Protected)),
                                s => Some(Ok(tools::Token::Identifier(s.to_string()))),
                            }
                        } else if c == ' ' || c == '\t' || c == '\n' {
                            return self.next();
                        } else {
                            Some(Err("Unrecognized Token!(@bottle::lexer)".to_string()))
                        }
                    }
                }
            }
            None => None,
        }
    }
}
