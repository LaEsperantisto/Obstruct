use crate::expr::Expr;
use crate::value::{func_val, Value};
use std::fmt;

#[derive(Clone)]
pub struct Variable {
    pub value: Value,
    pub is_mutable: bool,
}

impl Variable {
    pub fn new(value: Value, is_mutable: bool) -> Variable {
        Variable { value, is_mutable }
    }
    pub fn new_func(
        block: Box<Expr>,
        parameters: Vec<(String, String)>,
        return_type: &str,
        is_mutable: bool,
    ) -> Variable {
        Variable {
            value: func_val((block, parameters, return_type.to_string())),
            is_mutable,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
