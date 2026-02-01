use crate::error;
use crate::value::{nil, Value};
use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
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
        self.values.get_mut(name).unwrap().value = value;
    }

    pub fn get(&self, name: &str) -> Variable {
        self.values.get(name).cloned().unwrap_or_else(|| {
            error(-1, format!("Undefined variable '{}'", name).as_str());
            Variable::new(nil(), false)
        })
    }
}
