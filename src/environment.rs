use crate::error;
use crate::expr::Expr;
use crate::value::{func_val, nil, Value};
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
            error(-1, format!("Variable '{}' not mutable", name).as_str());
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
        self.values.get_mut(name).unwrap().value = value.clone();
    }

    pub fn get(&self, name: &str) -> Variable {
        self.values.get(name).cloned().unwrap_or_else(|| {
            error(-1, format!("Undefined variable '{}'", name).as_str());
            Variable::new(nil(), false)
        })
    }

    pub fn get_func(&self, name: &str) -> (Box<Expr>, Vec<(String, String)>, String) {
        self.values
            .get(name)
            .cloned()
            .unwrap_or_else(|| {
                error(-1, format!("Undefined variable '{}'", name).as_str());
                nil_func()
            })
            .value
            .body
            .unwrap_or_else(|| {
                error(-1, format!("Variable '{}' not function", name).as_str());
                nil_func().value.body.unwrap()
            })
    }

    pub fn make_func(
        &mut self,
        name: &str,
        block: Box<Expr>,
        return_type: &str,
        parameters: Vec<(String, String)>,
        is_mutable: bool,
    ) {
        if self.values.contains_key(name) && !self.values.get(name).unwrap().is_mutable {
            error(
                -1,
                format!("Variable '{}' already defined as immutable", name).as_str(),
            );
        } else if self.values.contains_key(name)
            && self.values.get(name).unwrap().value.value_type != "func"
        {
            error(
                -1,
                format!(
                    "Variable '{}' already declared with type '{}'; could not define function.",
                    name,
                    self.values.get(name).unwrap().value.value_type
                )
                .as_str(),
            );
        }
        self.values.insert(
            name.to_string(),
            Variable::new_func(block, parameters, return_type, is_mutable),
        );
    }
}

fn nil_func() -> Variable {
    Variable {
        value: func_val((Box::new(Expr::StmtBlock(vec![])), vec![], "[]".to_string())),
        is_mutable: false,
    }
}
