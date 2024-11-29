use ast::*;
use std::collections::HashMap;
mod scope;
mod symbol;
use slotmap::{new_key_type, SlotMap};
use thiserror::Error;

new_key_type! { pub struct ScopeId; }
new_key_type! { pub struct TypeId; }

/// Information about a symbol in the program
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub ty: TypeId,
    pub scope: ScopeId,
}

/// Information about a type in the program
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub size: usize,                     // Example: for primitive types or structs
    pub fields: HashMap<String, TypeId>, // For composite types like structs
}

/// Enum to represent semantic errors
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Duplicate declaration of symbol `{0}` in the same scope")]
    DuplicateDeclaration(String),
    #[error("Use of undeclared symbol `{0}`")]
    UndeclaredSymbol(String),
    #[error("Type mismatch: expected `{expected}`, found `{found}`")]
    TypeMismatch { expected: String, found: String },
    #[error("Scope `{0:?}` not found")]
    MissingScope(ScopeId),
}

/// Semantic context used for analysis
pub struct SemanticContext {
    pub symbol_table: HashMap<String, SymbolInfo>, // Flat mapping for symbols
    pub types: SlotMap<TypeId, TypeInfo>,          // Storage for type information
    pub scopes: SlotMap<ScopeId, HashMap<String, SymbolInfo>>, // Scopes for symbols
    pub current_scope: ScopeId,                    // Current active scope
    pub errors: Vec<SemanticError>,                // Accumulated errors
}

impl SemanticContext {
    /// Creates a new semantic context with a global scope
    pub fn new() -> Self {
        let mut scopes = SlotMap::with_key();
        let global_scope = scopes.insert(HashMap::new());
        SemanticContext {
            symbol_table: HashMap::new(),
            types: SlotMap::with_key(),
            scopes,
            current_scope: global_scope,
            errors: Vec::new(),
        }
    }

    /// Enters a new scope and sets it as the current scope
    pub fn enter_scope(&mut self) -> ScopeId {
        let new_scope = self.scopes.insert(HashMap::new());
        self.current_scope = new_scope;
        new_scope
    }

    /// Leaves the current scope, reverting to the parent or reporting an error
    pub fn leave_scope(&mut self) -> Result<(), SemanticError> {
        // Note: In a real implementation, you'd track parent scopes.
        if let Some(parent_scope) = self.scopes.keys().next() {
            self.current_scope = parent_scope;
            Ok(())
        } else {
            Err(SemanticError::MissingScope(self.current_scope))
        }
    }

    /// Adds a symbol to the current scope
    pub fn add_symbol(&mut self, name: String, ty: TypeId) {
        let current_scope = self.scopes.get_mut(self.current_scope).unwrap();
        if current_scope.contains_key(&name) {
            self.errors.push(SemanticError::DuplicateDeclaration(name));
        } else {
            current_scope.insert(
                name.clone(),
                SymbolInfo {
                    name,
                    ty,
                    scope: self.current_scope,
                },
            );
        }
    }

    /// Finds a symbol in the current or parent scopes
    pub fn find_symbol(&self, name: &str) -> Option<&SymbolInfo> {
        self.scopes
            .get(self.current_scope)
            .and_then(|scope| scope.get(name))
    }

    /// Registers a new type and returns its ID
    pub fn register_type(
        &mut self,
        name: String,
        size: usize,
        fields: HashMap<String, TypeId>,
    ) -> TypeId {
        self.types.insert(TypeInfo { name, size, fields })
    }

    /// Retrieves type information by ID
    pub fn get_type(&self, ty_id: TypeId) -> Option<&TypeInfo> {
        self.types.get(ty_id)
    }
}
