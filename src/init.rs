use crate::env::Environment;
use crate::error;
use crate::expr::Expr;
use crate::expr::Expr::{Float, Nothing, Str};
use crate::value::{nil, Value};
use std::io;

pub fn init(env: &mut Environment) {
    env.make_func("i32::new", Box::new(Float(0.0)), "i32", vec![], false);
    env.make_func("f64::new", Box::new(Float(0.0)), "f64", vec![], false);
    env.make_func("[]::new", Box::new(Nothing()), "[]", vec![], false);
    env.make_func(
        "str::new",
        Box::new(Str(String::new())),
        "str",
        vec![],
        false,
    );
    env.make_func(
        "quit",
        Box::new(Expr::Custom(|_| {
            error(0, 0, "Manual exit");
            nil()
        })),
        "[]",
        vec![],
        false,
    );
    env.make_func(
        "in",
        Box::new(Expr::Custom(|_| {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to readline");

            Value {
                value_type: "str".to_string(),
                value: input,
                body: None,
                native: None,
            }
        })),
        "str",
        vec![],
        false,
    );

    env.declare_native("len", native_len);
    env.declare_native("str::nth", native_str_nth);
}

fn native_len(_: &mut Environment, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        error(0, 0, "len() expects 1 argument");
        return nil();
    }

    Value {
        value_type: "f64".to_string(),
        value: args[0].value.len().to_string(),
        body: None,
        native: None,
    }
}

fn native_str_nth(_env: &mut Environment, args: Vec<Value>) -> Value {
    if args.len() != 2 {
        error(
            0,
            0,
            format!(
                "str::nth() expects 2 arguments but got {} argument/s",
                args.len()
            )
            .as_str(),
        );
        return nil();
    }

    let left = args.get(0).unwrap();
    let right = args.get(1).unwrap();
    if right.value_type != "i32" {
        error(0, 0, "str_nth() expects an 'f64' as right argument");
        return nil();
    }
    if left.value_type != "str" {
        error(0, 0, "str_nth() expects an 'str' as left argument");
        return nil();
    }

    if str::parse::<usize>(right.value.as_str()).unwrap() >= left.value.chars().count() {
        error(
            0,
            0,
            format!(
                "Out of bounds index '{}' with list of len '{}'",
                str::parse::<usize>(right.value.as_str()).unwrap(),
                left.value.chars().count()
            )
            .as_str(),
        );
        return nil();
    }

    Value {
        value_type: "char".to_string(),
        value: left
            .value
            .chars()
            .nth(str::parse::<usize>(right.value.as_str()).unwrap())
            .unwrap()
            .to_string(),
        body: None,
        native: None,
    }
}
