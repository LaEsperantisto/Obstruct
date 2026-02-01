use crate::environment::Environment;
use crate::error;
use crate::value::{nil, Value};

#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    Num(f64),
    Bool(bool),
    Str(String),

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

    // Variables
    Variable(String),
    Declare(String, Box<Expr>, bool),
    Assign(String, Box<Expr>),
}

impl Expr {
    pub fn value(&self, env: &mut Environment) -> Value {
        match self {
            // ---- Literals ----
            Expr::Num(n) => Value {
                value_type: "f64".to_string(),
                value: n.to_string(),
            },
            Expr::Bool(b) => Value {
                value_type: "bool".to_string(),
                value: if *b {
                    "`t".to_string()
                } else {
                    "`f".to_string()
                },
            },
            Expr::Str(s) => Value {
                value_type: "str".to_string(),
                value: s.clone(),
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
                    },
                    _ => Value {
                        value_type: "String".to_string(),
                        value: lv.value + &rv.value,
                    },
                }
            }
            Expr::Sub(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv - rv).to_string(),
                }
            }
            Expr::Mult(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv * rv).to_string(),
                }
            }
            Expr::Divide(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                let result = if rv == 0.0 {
                    error(-1, "Undefined dividing by 0");
                    0.0
                } else {
                    lv / rv
                };
                Value {
                    value_type: "f64".to_string(),
                    value: result.to_string(),
                }
            }
            Expr::Mod(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: (lv % rv).to_string(),
                }
            }
            Expr::Power(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "f64".to_string(),
                    value: lv.powf(rv).to_string(),
                }
            }
            Expr::EqualEqual(l, r) => {
                let lv = l.value(env);
                let rv = r.value(env);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv.value == rv.value {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
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
                }
            }
            Expr::Greater(l, r) => {
                let lv = l.value(env).value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value(env).value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "bool".to_string(),
                    value: if lv > rv {
                        "`t".to_string()
                    } else {
                        "`f".to_string()
                    },
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
                }
            }
            Expr::Print(r) => {
                let val = r.value(env);
                print!("{}", val.to_string());
                val
            }
            Expr::StmtBlock(stmts) => {
                let mut val = nil();
                for stmt in stmts {
                    val = stmt.value(env);
                }
                val
            }
            Expr::Variable(var) => env.get(&var).value,
            Expr::Declare(name, expr, is_mutable) => {
                let value = expr.value(env);
                env.declare(name.clone(), value.clone(), *is_mutable);
                value
            }
            Expr::Assign(name, expr) => {
                let value = expr.value(env);
                env.assign(name, value);
                nil()
            }
            Expr::Discard(expr) => {
                expr.value(env);
                nil()
            }
        }
    }
}
