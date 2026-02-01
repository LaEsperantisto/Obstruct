use crate::error;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Expr {
    // Operators
    Num(f64),
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
    Not(Box<Expr>),
}

impl Expr {
    pub fn value(&self) -> Value {
        match self {
            Expr::Num(n) => Value {
                value_type: "Number".to_string(),
                value: n.to_string(),
            },
            Expr::Add(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Number".to_string(),
                    value: (lv + rv).to_string(),
                }
            }
            Expr::Sub(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Number".to_string(),
                    value: (lv - rv).to_string(),
                }
            }
            Expr::Mult(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Number".to_string(),
                    value: (lv * rv).to_string(),
                }
            }
            Expr::Divide(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                let result = if rv == 0.0 {
                    error(-1, "Undefined dividing by 0");
                    0.0
                } else {
                    lv / rv
                };
                Value {
                    value_type: "Number".to_string(),
                    value: result.to_string(),
                }
            }
            Expr::Mod(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Number".to_string(),
                    value: (lv % rv).to_string(),
                }
            }
            Expr::Power(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Number".to_string(),
                    value: lv.powf(rv).to_string(),
                }
            }
            Expr::EqualEqual(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv == rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::BangEqual(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv != rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::GreaterEqual(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv >= rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::LessEqual(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv <= rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::Less(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv < rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::Greater(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv > rv {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::And(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv != 0.0 && rv != 0.0 {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::Or(l, r) => {
                let lv = l.value().value.parse::<f64>().unwrap_or(0.0);
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if lv != 0.0 || rv != 0.0 {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Expr::Not(r) => {
                let rv = r.value().value.parse::<f64>().unwrap_or(0.0);
                Value {
                    value_type: "Boolean".to_string(),
                    value: if rv == 0.0 {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
        }
    }
}
