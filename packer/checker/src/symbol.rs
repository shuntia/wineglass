use std::hash::{self, Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct TypeInfo<'a> {
    name: &'a str,
    type_hash: u64,
    size: usize,
    alignment: usize,
}

impl<'a> TypeInfo<'a> {
    pub fn new(name: &'a str, size: usize, alignment: usize) -> Self {
        Self {
            name,
            type_hash: 0,
            size,
            alignment,
        }
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_alignment(&self) -> usize {
        self.alignment
    }

    pub fn get_type_hash(&self) -> u64 {
        self.type_hash
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SymbolInfo<'a> {
    name: &'a str,             // Name of the symbol (e.g., variable name)
    symbol_type: TypeInfo<'a>, // The type of the symbol (e.g., int, float, custom type)
    is_mutable: bool,          // Indicates if the symbol is mutable
    scope_level: usize,        // The scope level where the symbol is defined
    references: usize,         // Number of references to this symbol
    type_hash: u64,
    value: Option<&'a str>, // The value of the symbol
}

impl<'a> SymbolInfo<'a> {
    pub fn new(
        name: &'a str,
        symbol_type: TypeInfo<'a>,
        is_mutable: bool,
        scope_level: usize,
    ) -> Self {
        let type_hash = symbol_type.get_type_hash();
        Self {
            name,
            symbol_type,
            is_mutable,
            scope_level,
            references: 0,
            type_hash,
            value: None,
        }
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_type(&self) -> TypeInfo<'a> {
        self.symbol_type
    }

    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_scope_level(&self) -> usize {
        self.scope_level
    }

    pub fn is_initialized(&self) -> bool {
        self.value.is_none()
    }

    pub fn get_references(&self) -> usize {
        self.references
    }

    pub fn get_type_hash(&self) -> u64 {
        self.type_hash
    }
}
