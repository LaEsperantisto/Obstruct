use core::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TypeEnvironment {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String, ty: Type) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Type {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return t.clone();
            }
        }

        panic!("Type error: unknown variable {}", name);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    name: String,
    generics: Vec<Type>,
}

impl Type {
    pub fn new(name: String) -> Self {
        if name == "[]" {
            Self {
                name: "arr".into(),
                generics: Vec::new(),
            }
        } else {
            Self {
                name,
                generics: Vec::new(),
            }
        }
    }

    pub fn add_generic(&mut self, t: Type) {
        self.generics.push(t);
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn generics(&self) -> &[Type] {
        &self.generics
    }

    pub fn is_nil(&self) -> bool {
        self.generics.is_empty() && self.name == "arr"
    }

    pub fn is_generic(&self) -> bool {
        !self.generics.is_empty()
    }
}

impl From<&str> for Type {
    fn from(name: &str) -> Self {
        Self::new(name.into())
    }
}

impl From<String> for Type {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_generic() {
            write!(f, "{}<{}>", self.name, join(&self.generics))
        } else {
            write!(f, "{}", self.name)
        }
    }
}

fn join(vec: &Vec<Type>) -> String {
    let mut output = String::new();
    for t in vec {
        output.push_str(format!("{}, ", t).as_str());
    }
    output
}
