use core::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TypeEnvironment {
    scopes: Vec<HashMap<String, Type>>,
    gens: Vec<HashMap<String, Type>>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            gens: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn push_func(&mut self) {
        self.gens.push(HashMap::new());
    }

    pub fn pop_func(&mut self) {
        self.gens.pop();
    }

    pub fn get_gen(&mut self, name: String) -> Type {
        self.gens.last().unwrap().get(&name).unwrap().clone()
    }

    pub fn add_gen(&mut self, name: String, ty: Type) {
        self.gens.last_mut().unwrap().insert(name, ty);
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
pub enum Type {
    Concrete { name: String, generics: Vec<Type> },
    Generic(String), // T, U, etc
}

impl Type {
    pub fn simple(name: &str) -> Self {
        Type::Concrete {
            name: name.into(),
            generics: vec![],
        }
    }

    pub fn is_generic(&self) -> bool {
        matches!(self, Type::Generic(_))
    }

    pub fn generic(name: &str) -> Self {
        Type::Generic(name.into())
    }

    pub fn generics(&self) -> Vec<Type> {
        if let Type::Concrete { name: _, generics } = &self {
            generics.clone()
        } else {
            vec![]
        }
    }

    pub fn with_generics(name: &str, gens: Vec<Type>) -> Self {
        Type::Concrete {
            name: name.into(),
            generics: gens,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Type::Concrete { name, .. } => name,
            Type::Generic(n) => n,
        }
    }
}

impl From<&str> for Type {
    fn from(name: &str) -> Self {
        Type::simple(name)
    }
}

impl From<String> for Type {
    fn from(name: String) -> Self {
        Type::simple(&name)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Generic(n) => write!(f, "{}", n),
            Type::Concrete { name, generics } => {
                if generics.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let g = generics
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "{}<{}>", name, g)
                }
            }
        }
    }
}

pub fn nil_type() -> Type {
    Type::simple("arr")
}

pub fn unify(pattern: &Type, actual: &Type, bindings: &mut HashMap<String, Type>) -> bool {
    match (pattern, actual) {
        (Type::Generic(name), t) => {
            if let Some(bound) = bindings.get(name) {
                bound == t
            } else {
                bindings.insert(name.clone(), t.clone());
                true
            }
        }

        (
            Type::Concrete {
                name: a,
                generics: ag,
            },
            Type::Concrete {
                name: b,
                generics: bg,
            },
        ) => {
            if a != b || ag.len() != bg.len() {
                return false;
            }

            for (x, y) in ag.iter().zip(bg.iter()) {
                if !unify(x, y, bindings) {
                    return false;
                }
            }
            true
        }

        _ => false,
    }
}

pub fn substitute(t: &Type, map: &HashMap<String, Type>) -> Type {
    match t {
        Type::Generic(n) => map.get(n).cloned().unwrap_or(t.clone()),

        Type::Concrete { name, generics } => Type::Concrete {
            name: name.clone(),
            generics: generics.iter().map(|g| substitute(g, map)).collect(),
        },
    }
}
