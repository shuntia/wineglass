pub mod ast;
use std::any::Any;

use crate::lexer::{self, tools::SimpleToken, tools::Token};

pub struct Parser {
    pub lexer: lexer::Lexer,
    ast: ast::AST,
    errors: Vec<ParseError>,
}

pub enum ErrorType {
    SyntaxError(String),
    UnexpectedToken(Token, String),
    UnexpectedEOF(String),
    GeneralError(String),
}

pub struct ParseError {
    pub error: ErrorType,
    pub position: lexer::tools::Position,
}

pub enum ParseResult {
    Success(ast::AST),
    Failure(ParseError),
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Parser {
        let ast = ast::AST::new();
        Parser {
            lexer,
            ast,
            errors: Vec::new(),
        }
    }

    pub fn error<T: Into<ErrorType>>(&mut self, error: T) {
        self.errors.push(ParseError {
            error: error.into(),
            position: self.lexer.get_loc(),
        });
    }

    pub fn parse(&mut self) -> ast::AST {
        ast::AST { head: self.root() }
    }

    pub fn check_eof(&mut self) -> Result<(), ()> {
        if self.ast.is_empty() {
            self.error(ErrorType::UnexpectedEOF("Unexpected EOF".to_string()));
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn root(&mut self) -> ast::AstNode {
        let mut children = Vec::new();
        loop {
            let node = self.classify();
            if node == ast::AstNode::Eof {
                break;
            }
            children.push(node);
        }
        ast::AstNode::Root { children }
    }

    pub fn function(&mut self) -> ast::AstNode {
        let params: Vec<ast::AstNode> = Vec::new();
        let body: Vec<ast::AstNode> = Vec::new();
        let name = match self.lexer.next() {
            Some(Ok(Token::Identifier(name))) => name,
            None => {
                self.error(ErrorType::UnexpectedEOF("Unexpected EOF".to_string()));
                return ast::AstNode::None;
            }
            t => {
                self.error(ErrorType::UnexpectedToken(
                    t.clone().unwrap().unwrap(),
                    format!(
                        "Expected Identifier, instead got: {:?}",
                        t.unwrap().unwrap()
                    ),
                ));
                return ast::AstNode::None;
            }
        };

        ast::AstNode::Function { name, params, body }
    }

    pub fn args(&mut self) -> Vec<ast::AstNode> {
        let mut args: Vec<ast::AstNode> = Vec::new();
        loop {
            match self.lexer.peek_next() {
                Some(Ok(Token::Simple(SimpleToken::CloseParen))) => return args,
                Some(Err(e)) => {
                    self.error(ErrorType::SyntaxError(e));
                    self.lexer.next();
                }
                None => {
                    self.error(ErrorType::UnexpectedEOF(
                        "Unexpected EOF: EOF encountered while parsing args.".to_string(),
                    ));
                    return args;
                }
                _ => {}
            }
            let node = self.classify();
            args.push(node);
        }
    }

    pub fn none(&mut self) -> ast::AstNode {
        ast::AstNode::None
    }
    ///This function is used to classify the tokens into their respective types. They need to be called after one statement.
    pub fn classify(&mut self) -> ast::AstNode {
        match self.lexer.peek_next() {
            Some(s) => match s {
                Ok(t) => match t {
                    Token::Identifier(s) => match self.lexer.peek_n(1) {
                        Some(Ok(Token::Simple(SimpleToken::OpenParen))) => self.call(),
                        Some(Ok(Token::Identifier(_))) => self.declaration(),
                        Some(Err(e)) => {
                            self.error(ErrorType::SyntaxError(e));
                            ast::AstNode::None
                        }
                        None => {
                            self.error(ErrorType::UnexpectedEOF("Unexpected EOF: Expected FUNCTION or DECLARATION, ASSIGNMENT after Identifier.".to_string()));
                            ast::AstNode::Eof
                        }
                        Some(Ok(Token::Eof)) => {
                            self.error(ErrorType::UnexpectedEOF("Unexpected EOF: Expected FUNCTION or DECLARATION, ASSIGNMENT after Identifier.".to_string()));
                            ast::AstNode::Eof
                        }
                        Some(Ok(Token::Simple(SimpleToken::OpenBrace))) => self.construct(),
                        _ => ast::AstNode::Variable { name: s },
                    },
                    Token::None => self.none(),
                    _ => ast::AstNode::None,
                },
                Err(e) => {
                    self.error(ErrorType::SyntaxError(e));
                    ast::AstNode::None
                }
            },
            None => ast::AstNode::Eof,
        }
    }
    pub fn declaration(&mut self) -> ast::AstNode {
        ast::AstNode::None
    }
    pub fn call(&mut self) -> ast::AstNode {
        ast::AstNode::None
    }
    pub fn construct(&mut self) -> ast::AstNode {
        ast::AstNode::None
    }
    pub fn parseTo(&mut self, node: ast::AstNode) -> Vec<ast::AstNode> {
        let mut nodes = Vec::new();
        loop {
            let t = self.classify();
            if t.type_id() == node.type_id() {
                break;
            }
            nodes.push(t);
        }
        nodes
    }
}
