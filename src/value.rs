use crate::error;
use std::fmt;

#[derive(Clone)]
pub struct Value {
    pub value_type: String,
    pub value: String,
}

impl Value {
    pub fn is_true(&self) -> bool {
        if self.value_type != "bool" {
            error(
                -1,
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
        write!(f, "{}", self.value)
    }
}

pub fn nil() -> Value {
    Value {
        value_type: "[]".to_string(),
        value: "".to_string(),
    }
}
