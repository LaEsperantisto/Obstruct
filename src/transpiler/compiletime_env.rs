use crate::error;
use crate::span::Span;
use crate::type_env::{nil_type, Type};
use std::collections::HashMap;

pub struct CompileTimeEnv {
    types: Vec<Type>,
    scopes: Vec<HashMap<String, (usize, bool, Type)>>, // id, is_mutable, type
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
        this.register_type(Type::simple("arr"));
        this.register_type(Type::simple("f64"));
        this.declare_var(
            "_print".to_string(),
            false,
            Type::with_generics("func", vec![nil_type(), Type::simple("i32")]),
        );

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

    pub fn declare_var(&mut self, name: String, is_mutable: bool, var_type: Type) -> usize {
        let id = self.next_var_id;
        self.next_var_id += 1;

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, (id, is_mutable, var_type));

        id
    }

    fn resolve_var(&self, name: &str) -> Option<(usize, usize)> {
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(id) = scope.get(name) {
                return Some((id.0, idx));
            }
        }
        None
    }

    pub fn get_var(&self, name: &str) -> Option<(bool, Type)> {
        for scope in self.scopes.iter().rev() {
            if let Some((id, is_mutable, var_type)) = scope.get(name) {
                return Some((*is_mutable, var_type.clone()));
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
        let (id, scope) = self.resolve_var(name).unwrap_or_else(|| {
            error(span, format!("Could not find variable '{}'", name).as_str());
            (0, 0)
        });
        format!("v_{}s_{}", id, scope,)
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
