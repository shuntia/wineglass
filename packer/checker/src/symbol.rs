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
pub enum SymbolKind<'a> {
    Variable,
    Function {
        return_type: TypeInfo<'a>,
        parameter_types: &'a [TypeInfo<'a>],
    },
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolInfo<'a> {
    name: &'a str,             // Name of the symbol (e.g., variable or function name)
    symbol_type: TypeInfo<'a>, // The type of the symbol (e.g., int, float, custom type)
    is_mutable: bool,          // Indicates if the symbol is mutable (for variables)
    scope_level: usize,        // The scope level where the symbol is defined
    references: usize,         // Number of references to this symbol
    kind: SymbolKind<'a>,      // Kind of the symbol (variable or function)
    value: Option<&'a str>,    // The value of the symbol (for variables)
}

impl<'a> SymbolInfo<'a> {
    pub fn new_variable(
        name: &'a str,
        symbol_type: TypeInfo<'a>,
        is_mutable: bool,
        scope_level: usize,
        value: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            symbol_type,
            is_mutable,
            scope_level,
            references: 0,
            kind: SymbolKind::Variable,
            value,
        }
    }

    pub fn new_function(
        name: &'a str,
        return_type: TypeInfo<'a>,
        parameter_types: &'a [TypeInfo<'a>],
        scope_level: usize,
    ) -> Self {
        Self {
            name,
            symbol_type: return_type,
            is_mutable: false,
            scope_level,
            references: 0,
            kind: SymbolKind::Function {
                return_type,
                parameter_types,
            },
            value: None,
        }
    }
    pub fn get_name(&self) -> &'a str {
        self.name
    }
}
