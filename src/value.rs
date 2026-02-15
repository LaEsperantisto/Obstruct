use crate::env::Environment;
use crate::error;
use crate::expr::Expr;
use crate::type_env::{nil_type, Type, TypeEnvironment};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Value {
    pub value_type: Type,
    pub value: String,
    pub value_vec: Option<Vec<Value>>,
    pub body: Option<(Box<Expr>, Vec<(String, Type)>, Type)>,
    pub native: Option<fn(&mut Environment, &mut TypeEnvironment, Vec<Value>) -> Value>,
    pub is_return: bool,
}

impl Value {
    pub fn is_true(&self) -> bool {
        if self.value_type.name() != "bool" {
            error(
                0,
                0,
                format!("Expected 'bool' but got '{}'", self.value_type).as_str(),
            );
        }
        self.value_type.name() == "bool" && self.value == "`t"
    }

    #[inline(always)]
    pub fn is_false(&self) -> bool {
        !self.is_true()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value_type.name() == "func" {
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
        value_type: nil_type(),
        value: "".to_string(),
        value_vec: None,
        body: None,
        native: None,
        is_return: false,
    }
}

pub fn func_val(body: (Box<Expr>, Vec<(String, Type)>, Type)) -> Value {
    Value {
        value_type: "func".into(),
        value: "".to_string(),
        value_vec: None,
        body: Some(body),
        native: None,
        is_return: false,
    }
}

pub fn native_func(f: fn(&mut Environment, &mut TypeEnvironment, Vec<Value>) -> Value) -> Value {
    Value {
        value_type: "func".into(),
        value: "".to_string(),
        value_vec: None,
        body: None,
        native: Some(f),
        is_return: false,
    }
}
