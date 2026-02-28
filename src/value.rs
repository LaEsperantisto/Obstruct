use crate::env::Environment;
use crate::error;
use crate::expr::Expr;
use crate::span::Span;
use crate::type_env::{nil_type, Type, TypeEnvironment};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Value {
    pub value_type: Type,
    pub value: String,
    pub value_vec: Option<Vec<Value>>,
    pub body: Option<Func>,
    pub native: Option<fn(&mut Environment, &mut TypeEnvironment, Vec<Value>, Span) -> Value>,
    pub is_return: bool,
}

impl Value {
    pub fn is_true(&self) -> bool {
        if !self.value_type.has_tag("bool") {
            error(
                0,
                0,
                format!("Expected 'bool' but got '{}'", self.value_type).as_str(),
            );
        }
        self.value_type.has_tag("bool") && self.value == "`t"
    }

    #[inline(always)]
    pub fn is_false(&self) -> bool {
        !self.is_true()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value_type.has_tag("func") {
            if self.native.is_some() {
                panic!("Expected body for function");
            } else {
                write!(f, "{}", self.body.clone().unwrap().return_type)
            }
        } else if self.value_type.has_tag("vec") || self.value_type.has_tag("arr") {
            write!(f, "[")?;
            for v in self.value_vec.clone().unwrap() {
                write!(f, "{}, ", v)?;
            }
            write!(f, "]")
        } else {
            write!(f, "{}", self.value)
        }
    }
}

pub fn nil() -> Value {
    Value {
        value_type: nil_type(),
        value: "".to_string(),
        value_vec: Some(vec![]),
        body: None,
        native: None,
        is_return: false,
    }
}

pub fn func_val(func: Func) -> Value {
    Value {
        value_type: "func".into(),
        value: "".to_string(),
        value_vec: None,
        body: Some(func),
        native: None,
        is_return: false,
    }
}

pub fn native_func(
    f: fn(&mut Environment, &mut TypeEnvironment, Vec<Value>, Span) -> Value,
) -> Value {
    Value {
        value_type: "func".into(),
        value: "".to_string(),
        value_vec: None,
        body: None,
        native: Some(f),
        is_return: false,
    }
}
#[derive(Clone, Debug)]
pub struct Func {
    pub body: Box<Expr>,
    pub args: Vec<(String, Type)>,
    pub return_type: Type,
    pub gens: Vec<String>,
}

impl Func {
    pub fn new(
        body: Box<Expr>,
        args: Vec<(String, Type)>,
        return_type: Type,
        gens: Vec<String>,
    ) -> Func {
        Func {
            body,
            args,
            return_type,
            gens,
        }
    }
}

impl From<Func> for (Box<Expr>, Vec<(String, Type)>, Type, Vec<String>) {
    fn from(f: Func) -> (Box<Expr>, Vec<(String, Type)>, Type, Vec<String>) {
        (f.body, f.args, f.return_type, f.gens)
    }
}
