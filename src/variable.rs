use crate::expr::Expr;
use crate::type_env::Type;
use crate::value::{func_val, Func, Value};
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
        parameters: Vec<(String, Type)>,
        return_type: Type,
        gens: Vec<String>,
        is_mutable: bool,
    ) -> Variable {
        Variable {
            value: func_val(Func::new(block, parameters, return_type.into(), gens)),
            is_mutable,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.value_type.name() == "vec" {
            write!(f, "{}", self.value.value_vec.clone().unwrap()[0])
        } else {
            write!(f, "{}", self.value)
        }
    }
}
