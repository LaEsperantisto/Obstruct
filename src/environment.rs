use crate::error;
use crate::expr::Expr;
use crate::value::{func_val, nil, Value};
use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Variable>>,
    this: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            this: vec![],
        }
    }

    // ---------- SCOPES ----------

    pub fn new_this(&mut self, this: &str) {
        self.this.push(this.to_string());
    }

    pub fn end_this(&mut self) {
        self.this.pop();
    }

    pub fn this(&self) -> String {
        self.this.last().unwrap().clone()
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    // ---------- VARIABLES ----------

    pub fn declare(&mut self, name: String, value: Value, is_mutable: bool) {
        let scope = self.scopes.last_mut().unwrap();

        if scope.contains_key(&name) {
            error(
                0,
                0,
                format!("Variable '{}' already defined", name).as_str(),
            );
            return;
        }

        scope.insert(name, Variable::new(value, is_mutable));
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                if !var.is_mutable {
                    error(0, 0, format!("Variable '{}' not mutable", name).as_str());
                    return;
                }
                var.value = value;
                return;
            }
        }

        error(0, 0, format!("Undefined variable '{}'", name).as_str());
    }

    pub fn get(&self, name: &str) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return v.clone();
            }
        }

        error(0, 0, format!("Undefined variable '{}'", name).as_str());
        Variable::new(nil(), false)
    }

    pub fn delete(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.remove(name).is_some() {
                return;
            }
        }

        error(0, 0, format!("Undefined variable '{}'", name).as_str());
    }

    // ---------- FUNCTIONS ----------

    pub fn make_func(
        &mut self,
        name: &str,
        block: Box<Expr>,
        return_type: &str,
        parameters: Vec<(String, String)>,
        is_mutable: bool,
    ) {
        let scope = self.scopes.last_mut().unwrap();

        if let Some(existing) = scope.get(name) {
            if !existing.is_mutable {
                error(
                    0,
                    0,
                    format!("Variable '{}' already defined as immutable", name).as_str(),
                );
                return;
            }

            if existing.value.value_type != "func" {
                error(
                    0,
                    0,
                    format!(
                        "Variable '{}' already declared with type '{}'; could not define function.",
                        name, existing.value.value_type
                    )
                    .as_str(),
                );
                return;
            }
        }

        scope.insert(
            name.to_string(),
            Variable::new_func(block, parameters, return_type, is_mutable),
        );
    }

    pub fn get_func(&self, name: &str) -> (Box<Expr>, Vec<(String, String)>, String) {
        let var = self.get(name);

        var.value.body.unwrap_or_else(|| {
            error(
                0,
                0,
                format!("Variable '{}' is not a function", name).as_str(),
            );
            nil_func().value.body.unwrap()
        })
    }
}

// ---------- INTERNAL ----------

fn nil_func() -> Variable {
    Variable {
        value: func_val((Box::new(Expr::StmtBlock(vec![])), vec![], "[]".to_string())),
        is_mutable: false,
    }
}
