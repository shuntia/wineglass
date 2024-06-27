
use std::fs;
use std::vec;
use std::collections::BTreeMap;

struct TokenLookup{
    KEYWORD: BTreeMap,
    FLAG: BTreeMap,
    DEF_OPERAND: BTreeMap,
}
impl token_lookup {
    pub fn new(self)-> ! {
        
    }
}

pub struct Token{
    content: String,
    content_type: u64,
}
impl Token { 
    fn display(&self) -> &'static str {
        let TOKEN_DICT = fs::read_to_string("src/TOKEN_DICT.txt").expect("TOKEN_DICT Could not be read").split("\n").collect();
        return TOKEN_DICT[self.content_type]
    }
}
pub struct ProgramNode{
    parent: ProgramNode,
    children: Vec<ProgramNode>,
    content: Token
}
pub struct ProgramTree{
    head: ProgramNode,
}