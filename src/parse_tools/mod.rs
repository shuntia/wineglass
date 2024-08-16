
use std::fs::File;
use std::vec;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub enum Token{
    Eof(),
    Keyword(String),
    Literal(String),
    Punctuator(String),
    LiteralType(String),
    LiteralValue(String),
    Identifier(String),
    Operator(String),
    Separator(String),
    Bracket(String),
    Import(String),
    Option(String),
    Variable(String),
    Type(String),
    Function(String),
    Macro(String),
    Comment(String),
    Expression(String),
    Attribute(String),
    Structure(String),
    Trait(String),
    Enum(String),
    TraitAlias(String),
}