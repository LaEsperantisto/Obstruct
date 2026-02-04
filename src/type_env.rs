use std::collections::HashMap;

#[derive(Clone)]
pub struct TypeEnvironment {
    scopes: Vec<HashMap<String, String>>,
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

    pub fn declare(&mut self, name: String, ty: String) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    pub fn get(&self, name: &str) -> String {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return t.clone();
            }
        }

        panic!("Type error: unknown variable {}", name);
    }
}
