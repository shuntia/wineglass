use typed_arena::Arena;

#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    BinaryExpr {
        left: &'a AstNode<'a>,
        op: String,
        right: &'a AstNode<'a>,
    },
    UnaryExpr {
        op: String,
        expr: &'a AstNode<'a>,
    },
    StrLiteral {
        value: String,
    },
    IntLiteral {
        value: i64,
    },
    FloatLiteral {
        value: f64,
    },
    Identifier {
        name: String,
    },
    Root {
        children: Vec<&'a AstNode<'a>>,
    },
    Function {
        name: String,
        params: Vec<&'a AstNode<'a>>,
        return_type: String,
        body: Vec<&'a AstNode<'a>>,
    },
    Call {
        name: String,
        args: Vec<&'a AstNode<'a>>,
    },
    BangCall {
        name: String,
        args: Vec<&'a AstNode<'a>>,
    },
    Return {
        value: &'a AstNode<'a>,
    },
    Assignment {
        identifier: &'a AstNode<'a>,
        value: &'a AstNode<'a>,
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
        params: Vec<&'a AstNode<'a>>,
        body: Vec<&'a AstNode<'a>>,
    },
    Declaration {
        struct_type: String,
        name: String,
        value: &'a AstNode<'a>,
    },
    Variable {
        name: String,
    },
    Unknown {
        stmt: String,
    },
    Skip,
    Eof,
    None,
}

pub struct AST<'a> {
    pub(crate) head: &'a AstNode<'a>,
    pub(crate) arena: &'a Arena<AstNode<'a>>,
}

impl std::fmt::Debug for AST<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.head)
    }
}

impl<'a> AST<'a> {
    pub fn new(arena: &'a Arena<AstNode<'a>>) -> AST<'a> {
        let head = arena.alloc(AstNode::Root { children: vec![] });
        AST { head, arena }
    }
    pub fn is_empty(&self) -> bool {
        match self.head {
            AstNode::Root { children } => children.is_empty(),
            _ => panic!("Root node was expected! The AST initialization is improper!"),
        }
    }
}

impl<'a> AstNode<'a> {
    pub fn new(node: AstNode) -> AstNode {
        node
    }
}
