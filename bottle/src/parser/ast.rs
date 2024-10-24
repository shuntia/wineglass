#[derive(Debug, PartialEq)]
pub struct AST {
    pub head: AstNode,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    BinaryExpr {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    UnaryExpr {
        op: String,
        expr: Box<AstNode>,
    },
    Literal {
        value: String,
    },
    Identifier {
        name: String,
    },
    Root {
        children: Vec<AstNode>,
    },
    Function {
        name: String,
        params: Vec<AstNode>,
        body: Vec<AstNode>,
    },
    Call {
        name: String,
        args: Vec<AstNode>,
    },
    Return {
        value: Box<AstNode>,
    },
    Assignment {
        identifier: Box<AstNode>,
        value: Box<AstNode>,
    },
    Type {
        name: String,
    },
    Sharp {
        args: Vec<String>,
    },
    At {
        args: Vec<String>,
    },
    BottleCall {
        name: String,
        params: Vec<AstNode>,
        body: Vec<AstNode>,
    },
    Declaration {
        name: String,
        value: Box<AstNode>,
    },
    Variable {
        name: String,
    },
    Eof,
    None,
}

impl Default for AST {
    fn default() -> Self {
        Self::new()
    }
}

impl AST {
    pub fn new() -> AST {
        AST {
            head: AstNode::new(AstNode::Root {
                children: Vec::new(),
            }),
        }
    }
    pub fn is_empty(&self) -> bool {
        true
    }
}

impl AstNode {
    pub fn new(node: AstNode) -> AstNode {
        node
    }
}
