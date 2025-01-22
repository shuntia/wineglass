use crate::symbol::SymbolKind;
use crate::SymbolInfo;
use std::collections::HashMap;

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    symbol_table: HashMap<&'a str, SymbolInfo>,
    children: Vec<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Self {
        Self {
            parent,
            symbol_table: HashMap::new(),
            children: Vec::new(),
        }
    }
    pub fn insert(&mut self, symbol: SymbolInfo) {
        self.symbol_table.insert(symbol.get_name(), symbol);
    }
    pub fn get(&self, name: &'a str) -> Option<&SymbolInfo> {
        match self.symbol_table.get(name) {
            Some(symbol) => Some(symbol),
            None => match self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }
}
