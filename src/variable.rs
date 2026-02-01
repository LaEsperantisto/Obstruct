use crate::value::Value;
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
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
