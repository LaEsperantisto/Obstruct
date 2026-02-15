use crate::error;
use crate::expr::Expr;
use crate::type_env::{Type, TypeEnvironment};
use crate::value::{func_val, native_func, nil, Value};
use crate::variable::Variable;
use cobject::CWindow;
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Variable>>,
    this: Vec<String>,
    window: Option<CWindow>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            this: vec![],
            window: None,
        }
    }

    // ------------ THIS -----------

    pub fn end_this(&mut self) {
        self.this.pop();
    }

    pub fn this(&self) -> String {
        self.this.last().unwrap().clone()
    }

    pub fn new_this(&mut self, this: &str) {
        self.this.push(this.to_string());
    }

    // ---------- SCOPES ----------

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    // ---------- VARIABLES ----------

    pub fn declare(&mut self, name: String, value: Value, is_mutable: bool) {
        let scope = self.scopes.last_mut().unwrap();

        if scope.contains_key(&name) {
            error(
                0,
                0,
                format!("Variable '{}' already defined", name).as_str(),
            );
            return;
        }

        scope.insert(name, Variable::new(value, is_mutable));
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                if !var.is_mutable {
                    error(0, 0, format!("Variable '{}' not mutable", name).as_str());
                    return;
                }
                var.value = value;
                return;
            }
        }

        error(
            0,
            0,
            format!("Undefined variable '{}'. Could not assign.", name).as_str(),
        );
    }

    pub fn get(&self, name: &str) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return v.clone();
            }
        }

        error(
            0,
            0,
            format!("Undefined variable '{}'. Could not get value.", name).as_str(),
        );
        Variable::new(nil(), false)
    }

    pub fn delete(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.remove(name).is_some() {
                return;
            }
        }

        error(
            0,
            0,
            format!("Undefined variable '{}'. Could not delete.", name).as_str(),
        );
    }

    // ---------- FUNCTIONS ----------

    pub fn make_func(
        &mut self,
        name: &str,
        block: Box<Expr>,
        return_type: Type,
        parameters: Vec<(String, Type)>,
        is_mutable: bool,
    ) {
        let scope = self.scopes.last_mut().unwrap();

        if let Some(existing) = scope.get(name) {
            if !existing.is_mutable {
                error(
                    0,
                    0,
                    format!("Variable '{}' already defined as immutable", name).as_str(),
                );
                return;
            }

            if existing.value.value_type.name() != "func" {
                error(
                    0,
                    0,
                    format!(
                        "Variable '{}' already declared with type '{}'; could not define function.",
                        name, existing.value.value_type
                    )
                    .as_str(),
                );
                return;
            }
        }

        scope.insert(
            name.to_string(),
            Variable::new_func(block, parameters, return_type, is_mutable),
        );
    }
    pub fn declare_native(
        &mut self,
        name: &str,
        func: fn(&mut Environment, &mut TypeEnvironment, Vec<Value>) -> Value,
    ) {
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.to_string(), Variable::new(native_func(func), false));
    }

    pub fn get_func(&self, name: &str) -> (Box<Expr>, Vec<(String, Type)>, Type) {
        let var = self.get(name);

        var.value.body.unwrap_or_else(|| {
            error(
                0,
                0,
                format!("Variable '{}' is not a function", name).as_str(),
            );
            nil_func().value.body.unwrap()
        })
    }

    pub fn make_window(&mut self, name: String) {
        self.window = Some(CWindow::new(800, 800, name));
    }

    pub fn get_window(&mut self) -> &mut CWindow {
        self.window
            .as_mut()
            .expect("Window doesn't exist, could not fetch window")
    }
}

// ---------- INTERNAL ----------

fn nil_func() -> Variable {
    Variable {
        value: func_val((Box::new(Expr::StmtBlock(vec![])), vec![], "[]".into())),
        is_mutable: false,
    }
}
