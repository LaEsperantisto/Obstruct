use crate::error;
use crate::span::Span;
use crate::type_env::Type;
use std::collections::HashMap;

pub struct CompileTimeEnv {
    types: Vec<Type>,
    scopes: Vec<HashMap<String, usize>>,
    current_scope: usize,

    next_var_id: usize,
    next_type_id: usize,
}

impl CompileTimeEnv {
    pub fn new() -> CompileTimeEnv {
        let mut this = CompileTimeEnv {
            types: Vec::new(),
            scopes: vec![HashMap::new()],
            current_scope: 0,
            next_var_id: 0,
            next_type_id: 0,
        };

        this.register_type(Type::simple("i32"));

        this
    }

    // Scope Management

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.current_scope += 1;
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        self.current_scope -= 1;
    }

    // Variable Handling

    pub fn declare_var(&mut self, name: String) -> usize {
        let id = self.next_var_id;
        self.next_var_id += 1;

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, id);

        id
    }

    fn resolve_var(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(id) = scope.get(name) {
                return Some(*id);
            }
        }
        None
    }

    pub fn var_exists(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if let Some(_id) = scope.get(name) {
                return true;
            }
        }
        false
    }

    pub fn c_var_name(&self, name: &str, span: Span) -> String {
        format!(
            "v_{}s_{}",
            self.current_scope,
            self.resolve_var(name).unwrap_or_else(|| {
                error(span, format!("Could not find variable '{}'", name).as_str());
                0
            })
        )
    }

    // Type Handling

    pub fn register_type(&mut self, ty: Type) -> usize {
        if let Some(index) = self.types.iter().position(|t| t == &ty) {
            return index;
        }

        let id = self.next_type_id;
        self.next_type_id += 1;

        self.types.push(ty);
        id
    }

    fn get_type_id(&self, ty: &Type) -> Option<usize> {
        self.types.iter().position(|t| t == ty)
    }

    pub fn c_type_name(&self, type_name: &Type, span: Span) -> String {
        format!(
            "t_{}",
            self.get_type_id(type_name).unwrap_or_else(|| {
                error(
                    span,
                    format!("Could not find type '{}'", type_name).as_str(),
                );
                0
            })
        )
    }
}
