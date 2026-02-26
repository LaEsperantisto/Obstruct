use crate::env::Environment;
use crate::error;
use crate::expr::Expr;
use crate::expr::Expr::{Float, Int, Nothing, Str};
use crate::type_env::{nil_type, Type, TypeEnvironment};
use crate::value::{nil, Value};
use crate::variable::Variable;
use cobject::ccolor;
use std::io;

pub fn init(env: &mut Environment) {
    env.make_func(
        "i32::new",
        Box::new(Int(0)),
        "i32".into(),
        vec![],
        vec![],
        false,
    );
    env.make_func(
        "f64::new",
        Box::new(Float(0.0)),
        "f64".into(),
        vec![],
        vec![],
        false,
    );
    env.make_func(
        "arr::new",
        Box::new(Nothing()),
        "arr".into(),
        vec![],
        vec![],
        false,
    );
    env.make_func(
        "vec::new",
        Box::new(Expr::Custom(|_| Value {
            value: String::new(),
            value_vec: Some(vec![]),
            value_type: Type::with_generics("vec", vec![Type::generic("T")]),
            body: None,
            native: None,
            is_return: false,
        })),
        Type::with_generics("vec", vec![Type::generic("T")]),
        vec![],
        vec!["T".into()],
        false,
    );

    env.make_func(
        "str::new",
        Box::new(Str(String::new())),
        "str".into(),
        vec![],
        vec![],
        false,
    );

    env.declare_native("ptr::new", |env, _tenv, values| {
        if values.len() != 1 {
            error(
                0,
                0,
                format!("ptr::new expects exactly 1 argument, got {}.", values.len()).as_str(),
            );
            return nil();
        }

        let val = values[0].clone();

        let var = Variable::new(val.clone(), true);

        let id = env.new_ptr(var);

        Value {
            value: id.to_string(),
            value_vec: None,
            value_type: Type::with_generics("ptr", vec![val.value_type.clone()]),
            body: None,
            native: None,
            is_return: false,
        }
    });

    env.declare_native("ptr::deref", |env, _tenv, values| {
        if values.len() != 1 {
            error(
                0,
                0,
                format!(
                    "ptr::deref expects exactly 1 argument, got {}.",
                    values.len()
                )
                .as_str(),
            );
            return nil();
        }

        let id = values[0].clone();

        env.get_ptr(str::parse::<usize>(&id.value).unwrap()).value
    });

    env.declare_native("ptr::free", |env, _tenv, values| {
        if values.len() != 1 {
            error(
                0,
                0,
                format!(
                    "ptr::free expects exactly 1 argument, got {}.",
                    values.len()
                )
                .as_str(),
            );
            return nil();
        }

        let id = values[0].clone();

        env.del_ptr(str::parse::<usize>(&id.value).unwrap());

        nil()
    });

    env.declare_native("ref::new", |_env, _tenv, args| {
        if args.len() != 1 {
            error(0, 0, "ref::new expected exactly one argument");
            return nil();
        }

        let inner = args[0].value_type.clone();

        Value {
            value: args[0].value.clone(),
            value_vec: None,
            value_type: Type::with_generics("ref", vec![inner]),
            body: None,
            native: None,
            is_return: false,
        }
    });

    env.declare_native("ref::deref", |env, tenv, args| {
        if args.len() != 1 {
            error(
                0,
                0,
                format!("ref::deref expects exactly 1 argument, got {}.", args.len()).as_str(),
            );
            return nil();
        }

        let referer = args[0].clone();

        Expr::Variable(referer.value.clone()).value(env, tenv)
    });

    env.make_func(
        "quit",
        Box::new(Expr::Custom(|_| {
            error(0, 0, "Manual exit");
            nil()
        })),
        nil_type(),
        vec![],
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
        vec![],
        false,
    );

    env.declare_native("direct_nth", |_env, _tenv, values| {
        if values.len() != 2 {
            error(0, 0, "Expected exactly two arguments for direct_nth");
            return nil();
        }

        let value = values[0].clone();
        let index = str::parse::<usize>(&values[1].clone().value).unwrap_or(0);

        if value.value_vec.is_none() {
            error(
                0,
                0,
                "First argument of function direct_nth did not have a value_vec; could not index",
            );
            return nil();
        }

        if index >= value.value_vec.as_ref().unwrap().len() {
            error(0, 0, "Index out of bounds");

            return nil();
        }

        value.value_vec.unwrap()[index].clone()
    });

    env.declare_native("len", native_len);
    env.declare_native("str::nth", native_str_nth);
    env.declare_native("vec::nth", native_vec_nth);
    env.declare_native("vec::push", native_vec_push);
    env.declare_native("type", native_type_check);
    env.declare_native("init_window", native_init_window);
    env.declare_native("draw_window", native_draw_window);
    env.declare_native("is_window_open", native_is_window_open);
}

fn native_len(_: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        error(0, 0, "len() expects 1 argument");
        return nil();
    }

    let v = args.get(0).unwrap();

    Value {
        value_type: "i32".into(),
        value: if v.value_type.name() == "str" {
            v.value.len().to_string()
        } else if v.value_type.name() == "vec" {
            v.value_vec.iter().len().to_string()
        } else {
            error(
                0,
                0,
                format!("len() could not find length of type {}", v.value_type).as_str(),
            );
            String::new()
        },
        value_vec: None,
        body: None,
        native: None,
        is_return: false,
    }
}

fn native_str_nth(_env: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
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

fn native_vec_push(env: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
    if args.len() != 2 {
        error(0, 0, "vec::push() expects 2 arguments");
    }

    let id = str::parse::<usize>(&args[0].value).unwrap();

    let mut v = env.get_ptr(id).value.value_vec.unwrap();

    let elem = args[1].clone();

    let vec_type = args[0].value_type.clone();

    let inner = &vec_type.generics()[0].generics()[0];

    if &elem.value_type != inner {
        error(
            0,
            0,
            format!("vec::push expected {}, got {}", inner, elem.value_type).as_str(),
        );
        return nil();
    }

    v.push(elem);

    env.set_ptr(
        id,
        Value {
            value_type: vec_type.generics()[0].clone(),
            value: String::new(),
            value_vec: Some(v),
            body: None,
            native: None,
            is_return: false,
        },
    );

    nil()
}

fn native_vec_nth(_: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
    if args.len() != 2 {
        error(0, 0, "vec::nth() expects 2 arguments");
        return nil();
    }

    let vec_val = &args[0];
    let index_val = &args[1];

    if vec_val.value_type.name() != "vec" {
        error(0, 0, "vec::nth() expects vec<T> as first argument");
        return nil();
    }

    if index_val.value_type.name() != "i32" {
        error(0, 0, "vec::nth() expects i32 as index");
        return nil();
    }

    let vec = vec_val.value_vec.as_ref().unwrap();
    let index = index_val.value.parse::<usize>().unwrap();

    if index >= vec.len() {
        error(0, 0, "vec::nth() index out of bounds");
        return nil();
    }

    vec[index].clone()
}

fn native_type_check(env: &mut Environment, tenv: &mut TypeEnvironment, args: Vec<Value>) -> Value {
    Str(args[0].value_type.clone().to_string()).value(env, tenv)
}

fn native_init_window(env: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        error(0, 0, "init_window() expects 1 argument");
    }
    env.make_window(args[0].value.clone());
    env.get_window().init();
    nil()
}

fn native_draw_window(env: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>) -> Value {
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

fn native_is_window_open(
    env: &mut Environment,
    tenv: &mut TypeEnvironment,
    args: Vec<Value>,
) -> Value {
    if !args.is_empty() {
        error(0, 0, "is_window_open() expects no argument");
    }

    let window = env.get_window();
    Expr::Bool(window.is_open()).value(env, tenv)
}
