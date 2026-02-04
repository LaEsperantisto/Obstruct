use crate::env::Environment;
use crate::type_env::TypeEnvironment;
use crate::value::{nil, Value};
use crate::{error, had_error};

#[derive(Debug, Clone)]
pub enum Expr {
    Nothing(),
    // Literals
    Float(f64),
    Int(i32),
    Bool(bool),
    Str(String),
    Char(String),

    // Binary Operators
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
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
    Function(String, Box<Expr>, String, bool, Vec<(String, String)>),
    CallFunc(String, Vec<Box<Expr>>),

    // Variables
    Variable(String),
    DeclareAndAssign(String, Box<Expr>, bool),
    Declare(String, String, bool),
    Assign(String, Box<Expr>),
    Delete(String),
    This(),

    // Control Flow
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if condition, if block, else block
    While(Box<Expr>, Box<Expr>),

    Custom(fn(&mut Environment) -> Value),
    Custom2(fn(&mut Environment, Vec<Value>) -> Value),
}

impl Expr {
    pub fn value(&self, env: &mut Environment) -> Value {
        if had_error() {
            return nil();
        }
        match self {
            // ---- Literals ----
            Expr::Float(n) => Value {
                value_type: "f64".to_string(),
                value: n.to_string(),
                body: None,
                native: None,
            },
            Expr::Int(n) => Value {
                value_type: "i32".to_string(),
                value: n.to_string(),
                body: None,
                native: None,
            },
            Expr::Bool(b) => Value {
                value_type: "bool".to_string(),
                value: if *b {
                    "`t".to_string()
                } else {
                    "`f".to_string()
                },
                body: None,
                native: None,
            },
            Expr::Str(s) => Value {
                value_type: "str".to_string(),
                value: s.clone(),
                body: None,
                native: None,
            },
            Expr::Char(c) => Value {
                value_type: "str".to_string(),
                value: c.clone(),
                body: None,
                native: None,
            },
            // ---- Binary Operators ----
            Expr::Add(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                match (&lv.value_type[..], &rv.value_type[..]) {
                    ("f64", "f64") => Value {
                        value_type: "f64".to_string(),
                        value: (lv.value.parse::<f64>().unwrap_or(0.0)
                            + rv.value.parse::<f64>().unwrap_or(0.0))
                        .to_string(),
                        body: None,
                        native: None,
                    },
                    ("str", "str") => Value {
                        value_type: "str".to_string(),
                        value: lv.value + &rv.value,
                        body: None,
                        native: None,
                    },
                    (_, "str") | ("str", _) => Value {
                        value_type: "str".to_string(),
                        value: lv.value + &rv.value,
                        body: None,
                        native: None,
                    },
                    _ => {
                        error(
                            0,
                            0,
                            format!("Could not add '{}' with '{}'", lv.value_type, rv.value_type)
                                .as_str(),
                        );
                        nil()
                    }
                }
            }
            Expr::Sub(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv - rv).to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::Mul(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv * rv).to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::Divide(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                let result = if rv == 0.0 {
                    error(0, 0, "Undefined dividing by 0");
                    0.0
                } else {
                    lv / rv
                };
                Value {
                    value_type: "f64".to_string(),
                    value: result.to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::Mod(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv % rv).to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::Power(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: lv.powf(rv).to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::EqualEqual(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv.value == rv.value && lv.value_type == rv.value_type {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::BangEqual(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv.value != rv.value {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::GreaterEqual(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv >= rv {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::LessEqual(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv <= rv {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::Less(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv < rv {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::Greater(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv > rv { "`t" } else { "`f" }.to_string(),
                    body: None,
                    native: None,
                }
            }
            Expr::And(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv.value != "`f" && rv.value != "`f" {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::Or(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv.value != "`f" || rv.value != "`f" {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }

            // ---- Unary ----
            Expr::Not(r) => {
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if rv.value == "`f" {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
                    body: None,
                    native: None,
                }
            }
            Expr::Print(r) => {
                let val = r.value(env);
                if !had_error() {
                    print!("{}", val.to_string());
                }
                val
            }
            Expr::StmtBlock(stmts) => {
                env.push_scope();

                let mut val = nil();
                for stmt in stmts {
                    val = stmt.value(env);
                }

                env.pop_scope();
                val
            }
            Expr::Variable(var) => env.get(&var).value,
            Expr::DeclareAndAssign(name, expr, is_mutable) => {
                let value = expr.value(env);
                env.declare(name.clone(), value.clone(), *is_mutable);
                value
            }
            Expr::Assign(name, expr) => {
                env.new_this(name);
                let value = expr.value(env);
                env.assign(name, value);
                env.end_this();
                nil()
            }
            Expr::If(if_cond, if_block, else_block) => {
                if if_cond.value(env).is_true() {
                    if_block.value(env)
                } else if else_block.is_some() {
                    else_block.clone().unwrap().value(env)
                } else {
                    nil()
                }
            }
            Expr::Declare(name, var_type, is_mutable) => {
                let func = env.get_func(format!("{}::new", var_type).as_str());
                let val = func.0.value(env);
                env.declare(name.to_string(), val, *is_mutable);
                nil()
            }
            Expr::Function(name, block, return_type, is_mutable, parameters) => {
                env.make_func(
                    name,
                    block.clone(),
                    return_type,
                    (*parameters).clone(),
                    *is_mutable,
                );
                nil()
            }
            Expr::CallFunc(name, arguments) => {
                let var = env.get(name);

                if let Some(native) = var.value.native {
                    let mut args = vec![];

                    for arg in arguments {
                        args.push(arg.value(env));
                    }

                    return native(env, args);
                }

                let (body, params, return_type) = env.get_func(name);

                if params.len() != arguments.len() {
                    error(
                        0,
                        0,
                        format!(
                            "Got {} arguments for function '{}', which wanted {} arguments.",
                            arguments.len(),
                            name,
                            params.len()
                        )
                        .as_str(),
                    );
                }

                env.push_scope();

                for i in 0..params.len() {
                    let arg_val = arguments[i].value(env);
                    if arg_val.value_type != params[i].1 {
                        error(0, 0, format!("Expected type '{}', but got '{}' with type '{}' in function call '{}'", arg_val.value_type, params[i].0, params[i].1, name).as_str())
                    }
                    env.declare(params[i].0.clone(), arg_val, false);
                }

                let result = body.value(env);

                env.pop_scope();

                if result.value_type != return_type {
                    error(
                        0,
                        0,
                        format!(
                            "Return type of function is '{}', but got type '{}'.",
                            return_type, result.value_type
                        )
                        .as_str(),
                    );
                }

                result
            }

            Expr::Discard(expr) => {
                expr.value(env);
                nil()
            }

            Expr::Delete(expr) => {
                env.delete(&expr);
                nil()
            }

            Expr::While(cond, body) => {
                while cond.value(env).is_true() {
                    body.value(env);
                }
                nil()
            }

            Expr::Nth(l, r) => {
                let val = l.value(env);
                Expr::CallFunc(
                    format!("{}::nth", val.value_type).to_string(),
                    vec![l.clone(), r.clone()],
                )
                .value(env)
            }

            Expr::This() => Expr::Variable(env.this()).value(env),

            Expr::Nothing() => nil(),

            Expr::Custom(func) => func(env),

            Expr::Custom2(func) => func(env, vec![]),
        }
    }

    pub fn type_of(&self, tenv: &mut TypeEnvironment) -> String {
        match self {
            Expr::Float(_) => "f64".into(),
            Expr::Int(_) => "i32".into(),
            Expr::Bool(_) => "bool".into(),
            Expr::Str(_) => "str".into(),
            Expr::Char(_) => "str".into(),

            Expr::Add(l, r) => {
                let lt = l.type_of(tenv);
                let rt = r.type_of(tenv);

                match (lt.as_str(), rt.as_str()) {
                    ("f64", "f64") => "f64".into(),
                    ("str", _) | (_, "str") => "str".into(),
                    _ => panic!("Type error: cannot add {} + {}", lt, rt),
                }
            }

            Expr::Sub(l, r)
            | Expr::Mul(l, r)
            | Expr::Divide(l, r)
            | Expr::Mod(l, r)
            | Expr::Power(l, r) => {
                let lt = l.type_of(tenv);
                let rt = r.type_of(tenv);

                if lt != "f64" || rt != "f64" {
                    panic!("Type error: arithmetic requires f64");
                }

                "f64".into()
            }

            Expr::EqualEqual(_, _)
            | Expr::BangEqual(_, _)
            | Expr::Greater(_, _)
            | Expr::GreaterEqual(_, _)
            | Expr::Less(_, _)
            | Expr::LessEqual(_, _) => "bool".into(),

            Expr::And(l, r) | Expr::Or(l, r) => {
                if l.type_of(tenv) != "bool" || r.type_of(tenv) != "bool" {
                    panic!("Type error: logical ops need bool");
                }
                "bool".into()
            }

            Expr::Not(e) => {
                if e.type_of(tenv) != "bool" {
                    panic!("Type error: ! needs bool");
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
                "[]".into()
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
                let mut last = "[]".to_string();
                for s in stmts {
                    last = s.type_of(tenv);
                }
                tenv.pop();
                last
            }

            Expr::If(cond, a, b) => {
                if cond.type_of(tenv) != "bool" {
                    panic!("if condition must be bool");
                }

                let t1 = a.type_of(tenv);
                let t2 = b.as_ref().map(|x| x.type_of(tenv)).unwrap_or("[]".into());

                if t1 != t2 {
                    panic!("if branches return different types");
                }

                t1
            }

            Expr::While(cond, _) => {
                if cond.type_of(tenv) != "bool" {
                    panic!("while condition must be bool");
                }
                "[]".into()
            }

            Expr::Function(name, body, return_type, _, params) => {
                tenv.declare(name.clone(), "func".into());

                tenv.push();
                for (p, t) in params {
                    tenv.declare(p.clone(), t.clone());
                }

                let actual = body.type_of(tenv);
                tenv.pop();

                if &actual != return_type {
                    panic!(
                        "Function {} should return {}, got {}",
                        name, return_type, actual
                    );
                }

                "func".into()
            }

            Expr::CallFunc(_, _) => {
                // you can later store full function signatures
                "unknown".into()
            }

            Expr::Nothing() => "[]".into(),

            _ => "[]".into(),
        }
    }
}
