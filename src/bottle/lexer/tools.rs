use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, Lines};

#[derive(Debug)]
pub enum Token {
    None,

    Eof,

    //# for bottle flags
    Sharp(String),

    //@ used for preprocessor
    At(String),

    //Unknown used for things that cannot be recognized by the lexer, therefore likely produces a syntax error
    Unknown(String),

    //Simple tokens
    Simple(SimpleToken),

    //Literals
    StringLiteral(String),
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralBool(bool),
    LiteralChar(char),
    LiteralNull,

    //Relating to objects
    Class(String),
    Struct(String),
    Enum(String),
    Interface(String),

    Identifier(String),

    //Keywords
    For,
    While,
    If,
    Else,
    Return,
    Break,
    Continue,
    Match,
    Case,

    //Flags
    Depend(String),
    Require(String),
    Import(String),

    Public,
    Private,
    Protected,

    //main
    Main,
}

#[derive(Debug)]
pub enum SimpleToken {
    //one-char tokens
    OpenParen,
    CloseParen,
    Comma,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Star,
    Semicolon,

    //Operators
    Plus,
    Minus,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
}

pub const SEPARATOR: [&str; 9] = ["(", ")", ",", "{", "}", "[", "]", ".", ";"];

pub const OPERATORS: [&str; 8] = ["+", "-", "/", "%", "=", "==", "!", "!="];

pub fn is_separator(c: char) -> bool {
    SEPARATOR.iter().any(|&x| x == c.to_string())
}

pub fn is_operator(c: char) -> bool {
    OPERATORS.iter().any(|&x| x == c.to_string())
}

pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

pub fn is_reserved(c: char) -> bool {
    is_separator(c) || is_operator(c) || is_whitespace(c)
}

pub struct LineReader {
    lines: Lines<BufReader<File>>,
    current_line: String,
    char_idx: usize,
}

impl LineReader {
    pub fn get_loc(&mut self) -> (usize, usize) {
        (self.lines.by_ref().count(), self.char_idx)
    }
    pub fn new(file: std::fs::File) -> io::Result<Self> {
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let current_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e),
            None => return Err(io::Error::new(io::ErrorKind::Other, "Empty file")),
        };
        Ok(Self {
            lines,
            current_line,
            char_idx: 0,
        })
    }

    fn load_next_line(&mut self) -> bool {
        self.current_line.clear();
        self.char_idx = 0;
        self.current_line = match self.lines.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => return false,
            None => return false,
        };
        true
    }
    pub fn remaining_line(&self) -> String {
        self.current_line[self.char_idx..].to_string()
    }
    pub fn last(&mut self) -> Option<Result<char,String>>
        where
            Self: Sized, {
            if self.char_idx==0{
                self.current_line=match self.lines.by_ref().last(){
                    Some(s)=>{
                        match s{
                            Ok(o)=>o,
                            Err(e)=>return Some(Err(e.to_string()))
                        }
                    }
                    None=>return None
                };
                self.char_idx=self.current_line.len();
            }
            self.char_idx-=1;
            return match self.current_line.chars().nth(self.char_idx){
                Some(c)=>Some(Ok(c)),
                None=>None
            };
    }
}

impl Iterator for LineReader {
    type Item = Result<char, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.char_idx < self.current_line.len() {
            let c = self.current_line.chars().nth(self.char_idx).unwrap();
            self.char_idx += 1;
            return Some(Ok(c));
        } else if self.load_next_line() {
            self.next(); // Retry with the new line.
        } else {
            return None; // No more characters to read.
        }

        if self.load_next_line() {
            self.next() // Retry with the new line.
        } else {
            None // No more characters to read.
        }
    }
}
