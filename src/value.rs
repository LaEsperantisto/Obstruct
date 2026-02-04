use crate::environment::Environment;
use crate::error;
use crate::expr::Expr;
use std::fmt;

#[derive(Clone)]
pub struct Value {
    pub value_type: String,
    pub value: String,
    pub body: Option<(Box<Expr>, Vec<(String, String)>, String)>,
    pub native: Option<fn(&mut Environment, Vec<Value>) -> Value>,
}

impl Value {
    pub fn is_true(&self) -> bool {
        if self.value_type != "bool" {
            error(
                0,
                0,
                format!("Expected 'bool' but got '{}'", self.value_type).as_str(),
            );
        }
        self.value_type == "bool" && self.value == "`t"
    }

    #[inline(always)]
    pub fn is_false(&self) -> bool {
        !self.is_true()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value_type == "func" {
            if self.native.is_some() {
                write!(f, "")
            } else {
                write!(f, "{}", self.body.clone().unwrap().2)
            }
        } else {
            write!(f, "{}", self.value)
        }
    }
}

pub fn nil() -> Value {
    Value {
        value_type: "[]".to_string(),
        value: "".to_string(),
        body: None,
        native: None,
    }
}

pub fn func_val(body: (Box<Expr>, Vec<(String, String)>, String)) -> Value {
    Value {
        value_type: "func".to_string(),
        value: "".to_string(),
        body: Some(body),
        native: None,
    }
}

pub fn native_func(f: fn(&mut Environment, Vec<Value>) -> Value) -> Value {
    Value {
        value_type: "func".to_string(),
        value: "".to_string(),
        body: None,
        native: Some(f),
    }
}
