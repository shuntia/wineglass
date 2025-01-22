use ast::*;
use std::collections::HashMap;
mod scope;
mod symbol;
use slotmap::{new_key_type, SlotMap};
use symbol::{SymbolInfo, TypeInfo};
use thiserror::Error;

new_key_type! { pub struct ScopeId; }
new_key_type! { pub struct TypeId; }

//struct that actually checks
pub struct Checker<'a> {
    context: SemanticContext<'a>,
    dependancies: HashMap<String, SemanticContext<'a>>,
    program: &'a AST<'a>,
}

impl<'a> Checker<'a> {
    fn new(program: &'a AST<'a>) -> Self {
        Checker {
            context: SemanticContext::new(),
            dependancies: HashMap::new(),
            program,
        }
    }
    fn add_dependancy(&mut self, name: String, context: SemanticContext<'a>) {
        self.dependancies.insert(name, context);
    }
    fn check(&self) {
        let mut context = SemanticContext::new();
        let global_scope = context.enter_scope();
        //traverse nodes and check for declaration errors etc
    }
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
pub struct SemanticContext<'a> {
    pub symbol_table: HashMap<String, SymbolInfo<'a>>, // Flat mapping for symbols
    pub types: SlotMap<TypeId, TypeInfo<'a>>,          // Storage for type information
    pub scopes: SlotMap<ScopeId, HashMap<String, SymbolInfo<'a>>>, // Scopes for symbols
    pub current_scope: ScopeId,                        // Current active scope
    pub errors: Vec<SemanticError>,                    // Accumulated errors
}

impl<'a> SemanticContext<'a> {
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
    pub fn add_symbol(&mut self, name: String, ty: TypeInfo) {
        let current_scope = self.scopes.get_mut(self.current_scope).unwrap();
        if current_scope.contains_key(&name) {
            self.errors.push(SemanticError::DuplicateDeclaration(name));
        } else {
            current_scope.insert(
                name.clone(),
                SymbolInfo::new_variable(&name, ty, false, self.current_scope, None),
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

fn check(program: &ast::AST) {
    let mut context = SemanticContext::new();
    let global_scope = context.enter_scope();
    //traverse nodes and check for declaration errors etc
}
