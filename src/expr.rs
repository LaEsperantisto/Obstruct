use crate::environment::Environment;
use crate::value::{nil, Value};
use crate::{error, had_error};

#[derive(Debug, Clone)]
pub enum Expr {
    Nothing(),
    // Literals
    Num(f64),
    Bool(bool),
    Str(String),
    Char(String),

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

    // Control Flow
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if condition, if block, else block
    While(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn value(&self, env: &mut Environment) -> Value {
        match self {
            // ---- Literals ----
            Expr::Num(n) => Value {
                value_type: "f64".to_string(),
                value: n.to_string(),
                body: None,
            },
            Expr::Bool(b) => Value {
                value_type: "bool".to_string(),
                value: if *b {
                    "`t".to_string()
                } else {
                    "`f".to_string()
                },
                body: None,
            },
            Expr::Str(s) => Value {
                value_type: "str".to_string(),
                value: s.clone(),
                body: None,
            },
            Expr::Char(c) => Value {
                value_type: "str".to_string(),
                value: c.clone(),
                body: None,
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
                    },
                    ("str", "str") => Value {
                        value_type: "str".to_string(),
                        value: lv.value + &rv.value,
                        body: None,
                    },
                    ("f64", "str") => Value {
                        value_type: "str".to_string(),
                        value: lv.value + &rv.value,
                        body: None,
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
                }
            }
            Expr::Mult(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv * rv).to_string(),
                    body: None,
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
                }
            }
            Expr::Mod(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv % rv).to_string(),
                    body: None,
                }
            }
            Expr::Power(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: lv.powf(rv).to_string(),
                    body: None,
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
                }
            }
            Expr::Greater(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv > rv { "`t" } else { "`f" }.to_string(),
                    body: None,
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
                let value = expr.value(env);
                env.assign(name, value);
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
                        error(0,0, format!("Expected type '{}', but got '{}' with type '{}' in function call '{}'",arg_val.value_type,params[i].0,params[i].1,name).as_str())
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
                            "Return type of function is '{}', but got '{}' with type '{}'.",
                            return_type, result.value, result.value_type
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
            Expr::Nothing() => nil(),
        }
    }
}
