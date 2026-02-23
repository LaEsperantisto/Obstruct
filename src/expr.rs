use crate::env::Environment;
use crate::type_env::{nil_type, substitute, unify, Type, TypeEnvironment};
use crate::value::{func_val, nil, Value};
use crate::{error, had_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Nothing(),
    // Literals
    Float(f64),
    Int(i32),
    Bool(bool),
    Str(String),
    Char(String),
    Type(Type),

    // Binary Operators
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    EqualEqual(Box<Expr>, Box<Expr>),
    BangEqual(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    Nth(Box<Expr>, Box<Expr>),

    // Unary Operators
    Not(Box<Expr>),

    // Statements
    StmtBlock(Vec<Box<Expr>>),
    Print(Box<Expr>),
    Discard(Box<Expr>),

    // Functions
    Function(
        String,
        Box<Expr>,
        Type,
        bool,
        Vec<(String, Type)>,
        Vec<String>,
    ),
    CallFunc(String, Vec<Type>, Vec<Box<Expr>>),
    Return(Box<Expr>),

    // Variables
    Variable(String),
    DeclareAndAssign(String, Box<Expr>, bool),
    Declare(String, Type, bool),
    Assign(String, Box<Expr>),
    Delete(String),
    This(),

    // Control Flow
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if condition, if block, else block
    While(Box<Expr>, Box<Expr>),

    // Others
    Custom(fn(&mut Environment) -> Value),
    Custom2(fn(&mut Environment, Vec<Value>) -> Value),
    Value(Value),
}

impl Expr {
    pub fn value(&self, env: &mut Environment, tenv: &mut TypeEnvironment) -> Value {
        if had_error() {
            return nil();
        }

        match self {
            // ---- Literals ----
            Expr::Float(n) => Value {
                value_type: "f64".into(),
                value: n.to_string(),
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            },
            Expr::Int(n) => Value {
                value_type: "i32".into(),
                value: n.to_string(),
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            },
            Expr::Bool(b) => Value {
                value_type: "bool".into(),
                value: if *b { "`t".into() } else { "`f".into() },
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            },
            Expr::Str(s) => Value {
                value_type: "str".into(),
                value: s.clone(),
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            },
            Expr::Char(c) => Value {
                value_type: "str".into(),
                value: c.clone(),
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            },

            // ---- Binary Operators ----
            Expr::Add(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => Value {
                        value_type: "f64".into(),
                        value: (lv.value.parse::<f64>().unwrap_or(0.0)
                            + rv.value.parse::<f64>().unwrap_or(0.0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("i32", "i32") => Value {
                        value_type: "i32".into(),
                        value: (lv.value.parse::<i32>().unwrap_or(0)
                            + rv.value.parse::<i32>().unwrap_or(0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("str", _) | (_, "str") => Value {
                        value_type: "str".into(),
                        value: lv.value + &rv.value,
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    _ => {
                        let func_name = format!("_add<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            Expr::Sub(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => Value {
                        value_type: "f64".into(),
                        value: (lv.value.parse::<f64>().unwrap_or(0.0)
                            - rv.value.parse::<f64>().unwrap_or(0.0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("i32", "i32") => Value {
                        value_type: "i32".into(),
                        value: (lv.value.parse::<i32>().unwrap_or(0)
                            - rv.value.parse::<i32>().unwrap_or(0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    _ => {
                        let func_name = format!("_sub<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            Expr::Mult(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => Value {
                        value_type: "f64".into(),
                        value: (lv.value.parse::<f64>().unwrap_or(0.0)
                            * rv.value.parse::<f64>().unwrap_or(0.0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("i32", "i32") => Value {
                        value_type: "i32".into(),
                        value: (lv.value.parse::<i32>().unwrap_or(0)
                            * rv.value.parse::<i32>().unwrap_or(0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    _ => {
                        let func_name = format!("_mul<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            Expr::Divide(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => {
                        let rv_num = rv.value.parse::<f64>().unwrap_or(0.0);
                        let result = if rv_num == 0.0 {
                            error(0, 0, "Undefined dividing by 0");
                            0.0
                        } else {
                            lv.value.parse::<f64>().unwrap_or(0.0) / rv_num
                        };
                        Value {
                            value_type: "f64".into(),
                            value: result.to_string(),
                            value_vec: None,
                            body: None,
                            native: None,
                            is_return: false,
                        }
                    }
                    ("i32", "i32") => {
                        let rv_num = rv.value.parse::<i32>().unwrap_or(0);
                        let result = if rv_num == 0 {
                            error(0, 0, "Undefined dividing by 0");
                            0
                        } else {
                            lv.value.parse::<i32>().unwrap_or(0) / rv_num
                        };
                        Value {
                            value_type: "i32".into(),
                            value: result.to_string(),
                            value_vec: None,
                            body: None,
                            native: None,
                            is_return: false,
                        }
                    }
                    _ => {
                        let func_name = format!("_div<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            Expr::Mod(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => Value {
                        value_type: "f64".into(),
                        value: (lv.value.parse::<f64>().unwrap_or(0.0)
                            % rv.value.parse::<f64>().unwrap_or(1.0))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("i32", "i32") => Value {
                        value_type: "i32".into(),
                        value: (lv.value.parse::<i32>().unwrap_or(0)
                            % rv.value.parse::<i32>().unwrap_or(1))
                        .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    _ => {
                        let func_name = format!("_mod<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            Expr::Power(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);

                match (lv.value_type.name(), rv.value_type.name()) {
                    ("f64", "f64") => Value {
                        value_type: "f64".into(),
                        value: lv
                            .value
                            .parse::<f64>()
                            .unwrap_or(0.0)
                            .powf(rv.value.parse::<f64>().unwrap_or(0.0))
                            .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    ("i32", "i32") => Value {
                        value_type: "i32".into(),
                        value: lv
                            .value
                            .parse::<i32>()
                            .unwrap_or(0)
                            .pow(rv.value.parse::<u32>().unwrap_or(0))
                            .to_string(),
                        value_vec: None,
                        body: None,
                        native: None,
                        is_return: false,
                    },
                    _ => {
                        let func_name = format!("_pow<{},{}>", lv.value_type, rv.value_type);
                        Expr::CallFunc(
                            func_name,
                            vec![],
                            vec![Box::new(Expr::Value(lv)), Box::new(Expr::Value(rv))],
                        )
                        .value(env, tenv)
                    }
                }
            }

            // ---- Comparison and logical operators ----
            Expr::EqualEqual(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);
                Value {
                    value_type: "bool".into(),
                    value: if lv.value == rv.value && lv.value_type == rv.value_type {
                        "`t".into()
                    } else {
                        "`f".into()
                    },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }
            Expr::BangEqual(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);
                Value {
                    value_type: "bool".into(),
                    value: if lv.value != rv.value {
                        "`t".into()
                    } else {
                        "`f".into()
                    },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }
            Expr::GreaterEqual(l, r)
            | Expr::Greater(l, r)
            | Expr::LessEqual(l, r)
            | Expr::Less(l, r) => {
                let lv = l.value(env, tenv).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env, tenv).value.parse::<f64>().unwrap_or(0.0);
                let result = match self {
                    Expr::Greater(_, _) => lv > rv,
                    Expr::GreaterEqual(_, _) => lv >= rv,
                    Expr::Less(_, _) => lv < rv,
                    Expr::LessEqual(_, _) => lv <= rv,
                    _ => false,
                };
                Value {
                    value_type: "bool".into(),
                    value: if result { "`t".into() } else { "`f".into() },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }

            Expr::And(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);
                Value {
                    value_type: "bool".into(),
                    value: if lv.value != "`f" && rv.value != "`f" {
                        "`t".into()
                    } else {
                        "`f".into()
                    },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }
            Expr::Or(l, r) => {
                let lv = l.value(env, tenv);
                let rv = r.value(env, tenv);
                Value {
                    value_type: "bool".into(),
                    value: if lv.value != "`f" || rv.value != "`f" {
                        "`t".into()
                    } else {
                        "`f".into()
                    },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }

            // ---- Unary ----
            Expr::Not(r) => {
                let rv = r.value(env, tenv);
                Value {
                    value_type: "bool".into(),
                    value: if rv.value == "`f" {
                        "`t".into()
                    } else {
                        "`f".into()
                    },
                    value_vec: None,
                    body: None,
                    native: None,
                    is_return: false,
                }
            }

            // ---- Statements, Variables, Functions, Control Flow ----
            Expr::Print(r) => {
                let val = r.value(env, tenv);
                if !had_error() {
                    print!("{}", val.to_string());
                }
                val
            }
            Expr::StmtBlock(stmts) => {
                env.push_scope();
                let mut val = nil();
                for stmt in stmts {
                    val = stmt.value(env, tenv);
                    if val.is_return {
                        break;
                    }
                }
                env.pop_scope();
                val
            }
            Expr::Variable(var) => env.get(var).value,
            Expr::DeclareAndAssign(name, expr, is_mutable) => {
                let value = expr.value(env, tenv);
                env.declare(name.clone(), value.clone(), *is_mutable);
                value
            }
            Expr::Assign(name, expr) => {
                env.new_this(name);
                let value = expr.value(env, tenv);
                env.assign(name, value);
                env.end_this();
                nil()
            }
            Expr::If(cond, if_block, else_block) => {
                if cond.value(env, tenv).is_true() {
                    if_block.value(env, tenv)
                } else if let Some(else_block) = else_block {
                    else_block.value(env, tenv)
                } else {
                    nil()
                }
            }
            Expr::Declare(name, var_type, is_mutable) => {
                let var_type = if var_type.is_generic() {
                    tenv.get(var_type.name())
                } else {
                    var_type.clone()
                };

                let func = env.get_func(format!("{}::new", var_type).as_str());
                let val = func.0.value(env, tenv);

                env.declare(name.to_string(), val, *is_mutable);
                nil()
            }
            Expr::Function(name, block, return_type, is_mutable, parameters, gens) => {
                env.make_func(
                    name,
                    block.clone(),
                    return_type.clone(),
                    (*parameters).clone(),
                    vec![],
                    *is_mutable,
                );
                func_val((
                    block.clone(),
                    parameters.clone(),
                    return_type.clone(),
                    gens.clone(),
                ))
            }
            Expr::CallFunc(name, explicit_gens, arguments) => {
                let var = env.get(name);

                if let Some(native) = var.value.native {
                    let args = arguments.iter().map(|a| a.value(env, tenv)).collect();
                    return native(env, tenv, args);
                }

                let (body, params, return_type, gens) = env.get_func(name);

                if params.len() != arguments.len() {
                    error(
                        0,
                        0,
                        format!(
                            "Got {} arguments for function '{}', expected {}",
                            arguments.len(),
                            name,
                            params.len()
                        )
                        .as_str(),
                    );
                }
                env.push_scope();

                let mut bindings = HashMap::new();

                for i in 0..params.len() {
                    let arg_value = arguments[i].value(env, tenv);
                    let arg_type = arg_value.value_type.clone();

                    if !unify(&params[i].1, &arg_type, &mut bindings) {
                        panic!("Type mismatch: expected {}, got {}", params[i].1, arg_type);
                    }

                    env.declare(
                        params[i].0.clone(), // parameter name
                        arg_value,           // actual value
                        false,
                    );
                }

                let real_return = substitute(&return_type, &bindings);

                let mut result = body.value(env, tenv);

                env.pop_scope();

                if result.is_return {
                    result.is_return = false;
                }

                if result.value_type != real_return {
                    error(
                        0,
                        0,
                        format!(
                            "Return type expected {}, got {}",
                            real_return, result.value_type
                        )
                        .as_str(),
                    );
                }

                result
            }

            Expr::Discard(expr) => {
                expr.value(env, tenv);
                nil()
            }
            Expr::Delete(expr) => {
                env.delete(expr);
                nil()
            }
            Expr::While(cond, body) => {
                while cond.value(env, tenv).is_true() {
                    body.value(env, tenv);
                }
                nil()
            }
            Expr::Nth(l, r) => {
                let val = l.value(env, tenv);
                Expr::CallFunc(
                    format!("{}::nth", val.value_type),
                    vec![],
                    vec![l.clone(), r.clone()],
                )
                .value(env, tenv)
            }
            Expr::This() => Expr::Variable(env.this()).value(env, tenv),
            Expr::Nothing() => nil(),
            Expr::Custom(func) => func(env),
            Expr::Custom2(func) => func(env, vec![]),
            Expr::Value(v) => v.clone(),

            Expr::Return(expr) => {
                let mut v = expr.value(env, tenv);
                v.is_return = true;
                v
            }

            Expr::Type(t) => Expr::Str(t.name().to_string()).value(env, tenv),
        }
    }

    pub fn type_of(&self, tenv: &mut TypeEnvironment) -> Type {
        match self {
            Expr::Float(_) => "f64".into(),
            Expr::Int(_) => "i32".into(),
            Expr::Bool(_) => "bool".into(),
            Expr::Str(_) | Expr::Char(_) => "str".into(),

            Expr::Add(l, r) => {
                let lt = l.type_of(tenv);
                let rt = r.type_of(tenv);
                match (lt.to_string().as_str(), rt.to_string().as_str()) {
                    ("f64", "f64") => "f64".into(),
                    ("i32", "i32") => "i32".into(),
                    ("str", _) | (_, "str") => "str".into(),
                    _ => format!("{}_{}", lt, rt).into(), // fallback for mixed/custom types
                }
            }

            Expr::Sub(l, r)
            | Expr::Mult(l, r)
            | Expr::Divide(l, r)
            | Expr::Mod(l, r)
            | Expr::Power(l, r) => {
                let lt = l.type_of(tenv);
                let rt = r.type_of(tenv);
                match (lt.to_string().as_str(), rt.to_string().as_str()) {
                    ("f64", "f64") => "f64".into(),
                    ("i32", "i32") => "i32".into(),
                    _ => format!("{}_{}", lt, rt).into(), // fallback
                }
            }

            Expr::EqualEqual(_, _)
            | Expr::BangEqual(_, _)
            | Expr::Greater(_, _)
            | Expr::GreaterEqual(_, _)
            | Expr::Less(_, _)
            | Expr::LessEqual(_, _) => "bool".into(),

            Expr::And(l, r) | Expr::Or(l, r) => {
                if l.type_of(tenv) != "bool".into() || r.type_of(tenv) != "bool".into() {
                    panic!("Type error: logical ops require bool");
                }
                "bool".into()
            }

            Expr::Not(e) => {
                if e.type_of(tenv) != "bool".into() {
                    panic!("Type error: ! requires bool");
                }
                "bool".into()
            }

            Expr::Variable(name) => tenv.get(name),

            Expr::DeclareAndAssign(name, expr, _) => {
                let t = expr.type_of(tenv);
                tenv.declare(name.clone(), t.clone());
                t
            }

            Expr::Declare(name, ty, _) => {
                tenv.declare(name.clone(), ty.clone());
                nil_type()
            }

            Expr::Assign(name, expr) => {
                let expected = tenv.get(name);
                let got = expr.type_of(tenv);
                if expected != got {
                    panic!("Type error: expected {}, got {}", expected, got);
                }
                expected
            }

            Expr::StmtBlock(stmts) => {
                tenv.push();
                let mut last = nil_type();
                for s in stmts {
                    last = s.type_of(tenv);
                }
                tenv.pop();
                last
            }

            Expr::If(cond, a, b) => {
                if cond.type_of(tenv) != "bool".into() {
                    panic!("if condition must be bool");
                }
                let t1 = a.type_of(tenv);
                let t2 = b.as_ref().map(|x| x.type_of(tenv)).unwrap_or(nil_type());
                if t1 != t2 {
                    panic!("if branches return different types");
                }
                t1
            }

            Expr::While(cond, _) => {
                let t = cond.type_of(tenv);
                if t != "bool".into() {
                    error(
                        0,
                        0,
                        format!("While condition must be bool, type was '{}'", t).as_str(),
                    );
                }
                nil_type()
            }

            Expr::Function(name, body, return_type, _, params, _gens) => {
                tenv.declare(name.clone(), "func".into());
                tenv.push();
                for (p, t) in params {
                    tenv.declare(p.clone(), t.clone());
                }
                let actual = body.type_of(tenv);
                tenv.pop();
                if &actual != return_type {
                    error(
                        0,
                        0,
                        format!(
                            "Function {} should return {}, got {}",
                            name, return_type, actual
                        )
                        .as_str(),
                    );
                }
                "func".into()
            }

            Expr::CallFunc(_, _, _) => "unknown".into(),

            Expr::Nothing() => nil_type(),

            Expr::Custom(_) | Expr::Custom2(_) => "unknown".into(),
            Expr::Value(v) => v.value_type.clone(),

            Expr::Nth(_, _) => "unknown".into(),
            Expr::This() => "unknown".into(),
            Expr::Print(_) | Expr::Discard(_) | Expr::Delete(_) => nil_type(),

            _ => "unknown".into(),
        }
    }
}
