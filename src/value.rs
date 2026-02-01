use std::fmt;

#[derive(Clone)]
pub struct Value {
    pub value_type: String,
    pub value: String,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value_type == "str" {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

pub fn nil() -> Value {
    Value {
        value_type: "nil".to_string(),
        value: "".to_string(),
    }
}
