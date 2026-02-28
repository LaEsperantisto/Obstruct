use crate::error;
use crate::expr::Expr;
use crate::span::Span;
use crate::type_env::{nil_type, Type, TypeEnvironment};
use crate::value::{func_val, native_func, nil, Func, Value};
use crate::variable::Variable;
use cobject::CWindow;
use std::collections::HashMap;

pub struct Environment {
    pub(crate) scopes: Vec<HashMap<String, usize>>,
    this: Vec<String>,
    window: Option<CWindow>,
    storage: Vec<Option<Variable>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            this: vec![],
            window: None,
            storage: vec![],
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
        for pointer in self.scopes.last().unwrap().values() {
            self.storage[*pointer] = None;
        }

        self.scopes.pop();
    }

    // ----------- POINTERS ------------

    pub fn new_ptr(&mut self, item: Variable) -> usize {
        let id = self.storage.len();
        self.storage.push(Some(item));
        id
    }

    pub fn del_ptr(&mut self, id: usize) {
        if let Some(slot) = self.storage.get_mut(id) {
            *slot = None;
        } else {
            error(0, 0, "Invalid pointer ID, could not delete.");
        }
    }

    pub fn set_ptr(&mut self, id: usize, val: Value) {
        if let Some(slot) = self.storage.get_mut(id) {
            let variable = slot.as_mut().unwrap();
            if !variable.is_mutable {
                error(0, 0, "Variable not mutable, could not set pointee value");
            }
            variable.value = val;
        } else {
            error(0, 0, "Invalid pointer ID, could not set value.");
        }
    }

    pub fn get_ptr(&mut self, id: usize) -> &mut Variable {
        match self.storage.get_mut(id) {
            Some(Some(var)) => var,
            _ => {
                error(0, 0, "Invalid or freed pointer dereference.");
                panic!("Invalid pointer dereference");
            }
        }
    }

    // ---------- VARIABLES ----------

    pub fn declare(&mut self, name: String, value: Value, is_mutable: bool) {
        let id = self.alloc_var(Variable::new(value, is_mutable));

        let scope = self.scopes.last_mut().unwrap();

        if scope.contains_key(&name) {
            error(
                0,
                0,
                format!("Variable '{}' already defined", name).as_str(),
            );
            return;
        }

        scope.insert(name, id);
    }

    pub fn assign(&mut self, name: &str, value: Value, span: Span) {
        for scope in self.scopes.iter().rev() {
            if let Some(&id) = scope.get(name) {
                if let Some(Some(var)) = self.storage.get_mut(id) {
                    if !var.is_mutable {
                        error(
                            span.line,
                            span.column,
                            format!("Variable '{}' not mutable", name).as_str(),
                        );
                        return;
                    }

                    var.value = value;
                    return;
                }
            }
        }

        error(
            span.line,
            span.column,
            format!("Undefined variable '{}'.", name).as_str(),
        );
    }

    pub fn get(&self, name: &str, span: Span) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(&id) = scope.get(name) {
                if let Some(Some(var)) = self.storage.get(id) {
                    return var.clone();
                }
            }
        }

        error(
            span.line,
            span.column,
            format!("Undefined variable '{}'.", name).as_str(),
        );
        Variable::new(nil(), false)
    }

    pub fn delete(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(id) = scope.remove(name) {
                if let Some(slot) = self.storage.get_mut(id) {
                    *slot = None;
                }
                return;
            }
        }

        error(0, 0, format!("Undefined variable '{}'.", name).as_str());
    }

    // ---------- FUNCTIONS ----------

    pub fn make_func(
        &mut self,
        name: &str,
        block: Box<Expr>,
        return_type: Type,
        parameters: Vec<(String, Type)>,
        gens: Vec<String>,
        is_mutable: bool,
        span: Span,
    ) {
        let scope = self.scopes.last_mut().unwrap();

        if scope.contains_key(name) {
            error(
                span.line,
                span.column,
                format!("'{}' already defined", name).as_str(),
            );
            return;
        }

        let func_var = Variable::new_func(block, parameters, return_type, gens, is_mutable);

        let id = self.storage.len();
        self.storage.push(Some(func_var));
        scope.insert(name.to_string(), id);
    }

    pub fn declare_native(
        &mut self,
        name: &str,
        func: fn(&mut Environment, &mut TypeEnvironment, Vec<Value>, Span) -> Value,
    ) {
        let scope = self.scopes.last_mut().unwrap();

        let id = self.storage.len();
        self.storage
            .push(Some(Variable::new(native_func(func), false)));

        scope.insert(name.to_string(), id);
    }

    pub fn get_func(&self, name: &str, span: Span) -> Func {
        let var = self.get(name, span);

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

    fn alloc_var(&mut self, var: Variable) -> usize {
        for (i, slot) in self.storage.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(var);
                return i;
            }
        }

        let id = self.storage.len();
        self.storage.push(Some(var));
        id
    }
}

// ---------- INTERNAL ----------

fn nil_func() -> Variable {
    Variable {
        value: func_val(Func::new(
            Box::new(Expr::StmtBlock(vec![])),
            vec![],
            nil_type(),
            vec![],
        )),
        is_mutable: false,
    }
}
