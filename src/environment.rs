use crate::error;
use crate::expr::Expr;
use crate::value::{nil, Value};
use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Variable>,
    funcs: HashMap<String, (Box<Expr>, String)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: String, value: Value, is_mutable: bool) {
        if self.values.contains_key(&name) {
            error(-1, format!("Variable '{}' already defined", name).as_str());
        }
        self.values.insert(name, Variable::new(value, is_mutable));
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        if !self.get(name).is_mutable {
            error(-1, "Variable not mutable");
        }
        if self.get(name).value.value_type != value.value_type {
            error(
                -1,
                format!(
                    "Variable '{}' with type '{}' could not be assigned with type '{}'",
                    name,
                    self.get(name).value.value_type,
                    value.value_type
                )
                .as_str(),
            );
        }
        self.values.get_mut(name).unwrap().value = value;
    }

    pub fn get(&self, name: &str) -> Variable {
        self.values.get(name).cloned().unwrap_or_else(|| {
            error(-1, format!("Undefined variable '{}'", name).as_str());
            Variable::new(nil(), false)
        })
    }

    pub fn get_func(&self, name: &str) -> (Box<Expr>, String) {
        self.funcs.get(name).cloned().unwrap_or_else(|| {
            error(-1, format!("Undefined variable '{}'", name).as_str());
            nil_func()
        })
    }

    pub fn make_func(&mut self, name: &str, block: Box<Expr>, return_type: &str) {
        self.funcs
            .insert(name.to_string(), (block, return_type.to_string()));
    }
}

fn nil_func() -> (Box<Expr>, String) {
    (Box::new(Expr::StmtBlock(vec![])), "[]".to_string())
}
