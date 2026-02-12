use crate::env::Environment;
use crate::expr::Expr;
use crate::expr::Expr::{Float, Int, Nothing, Str};
use crate::value::{nil, Value};
use crate::{error, value};
use cobject::ccolor;
use std::io;

pub fn init(env: &mut Environment) {
    env.make_func("i32::new", Box::new(Int(0)), "i32".into(), vec![], false);
    env.make_func(
        "f64::new",
        Box::new(Float(0.0)),
        "f64".into(),
        vec![],
        false,
    );
    env.make_func("arr::new", Box::new(Nothing()), "arr".into(), vec![], false);
    env.make_func(
        "vec::new",
        Box::new(Expr::Value(value::Value {
            value: String::new(),
            value_vec: Some(vec![]),
            value_type: "vec".into(),
            body: None,
            native: None,
            is_return: false,
        })),
        "vec".into(),
        vec![],
        false,
    );
    env.make_func(
        "str::new",
        Box::new(Str(String::new())),
        "str".into(),
        vec![],
        false,
    );
    env.make_func(
        "quit",
        Box::new(Expr::Custom(|_| {
            error(0, 0, "Manual exit");
            nil()
        })),
        "[]".into(),
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
                value_type: "str".into(),
                value: input,
                value_vec: None,
                body: None,
                native: None,
                is_return: false,
            }
        })),
        "str".into(),
        vec![],
        false,
    );

    env.declare_native("len", native_len);
    env.declare_native("str::nth", native_str_nth);
    env.declare_native("vec::nth", native_vec_nth);
    env.declare_native("vec::push", native_push);
    env.declare_native("type", native_type_check);
    env.declare_native("init_window", native_init_window);
    env.declare_native("show_window", native_show_window);
    env.declare_native("is_window_open", native_is_window_open);
}

fn native_len(_: &mut Environment, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        error(0, 0, "len() expects 1 argument");
        return nil();
    }

    Value {
        value_type: "f64".into(),
        value: args[0].value.len().to_string(),
        value_vec: None,
        body: None,
        native: None,
        is_return: false,
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
    if right.value_type.name() != "i32" {
        error(0, 0, "str_nth() expects an 'i32' as right argument");
        return nil();
    }
    if left.value_type.name() != "str" {
        error(0, 0, "str_nth() expects an 'str' as left argument");
        return nil();
    }

    if str::parse::<usize>(right.value.as_str()).unwrap() >= left.value.chars().count() {
        error(
            0,
            0,
            format!(
                "Out of bounds index '{}' with str of len '{}'",
                str::parse::<usize>(right.value.as_str()).unwrap(),
                left.value.chars().count()
            )
            .as_str(),
        );
        return nil();
    }

    Value {
        value_type: "char".into(),
        value: left
            .value
            .chars()
            .nth(str::parse::<usize>(right.value.as_str()).unwrap())
            .unwrap()
            .to_string(),
        value_vec: None,
        body: None,
        native: None,
        is_return: false,
    }
}

fn native_push(env: &mut Environment, args: Vec<Value>) -> Value {
    Value {
        value: String::new(),
        value_type: "vec".into(),
        value_vec: Some({
            let mut v = args[0].value_vec.clone().unwrap();
            v.push(Str(args[1].clone().value).value(env));
            v
        }),
        body: None,
        native: None,
        is_return: false,
    }
}

fn native_vec_nth(_env: &mut Environment, args: Vec<Value>) -> Value {
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
    if right.value_type.name() != "i32" {
        error(0, 0, "vec_nth() expects an 'i32' as right argument");
        return nil();
    }
    if left.value_type.name() != "vec" {
        error(0, 0, "vec_nth() expects an 'vec' as left argument");
        return nil();
    }

    if str::parse::<usize>(right.value.as_str()).unwrap() >= left.value_vec.clone().unwrap().len() {
        error(
            0,
            0,
            format!(
                "Out of bounds index '{}' with vec of len '{}'",
                str::parse::<usize>(right.value.as_str()).unwrap(),
                left.value.chars().count()
            )
            .as_str(),
        );
        return nil();
    }

    Value {
        value_type: "unknown".into(),
        value: left.clone().value_vec.unwrap()[str::parse::<usize>(right.value.as_str()).unwrap()]
            .to_string(),
        value_vec: None,
        body: None,
        native: None,
        is_return: false,
    }
}

fn native_type_check(env: &mut Environment, args: Vec<Value>) -> Value {
    Str(args[0].value_type.clone().name().to_string()).value(env)
}

fn native_init_window(env: &mut Environment, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        error(0, 0, "init_window() expects 1 argument");
    }
    env.make_window(args[0].value.clone());
    env.get_window().init();
    nil()
}

fn native_show_window(env: &mut Environment, args: Vec<Value>) -> Value {
    if !args.is_empty() {
        error(0, 0, "show_window() expects no argument");
    }

    let window = env.get_window();

    window.poll_input();
    window.update();

    window.fill(ccolor::BLACK);
    window.show_window();

    nil()
}

fn native_is_window_open(env: &mut Environment, args: Vec<Value>) -> Value {
    if !args.is_empty() {
        error(0, 0, "is_window_open() expects no argument");
    }

    let window = env.get_window();
    Expr::Bool(window.is_open()).value(env)
}
