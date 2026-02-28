use crate::env::Environment;
use crate::error;
use crate::expr::Expr;
use crate::expr::Expr::{Float, Int, Nothing, Str};
use crate::span::Span;
use crate::type_env::{nil_type, Type, TypeEnvironment};
use crate::value::{nil, Value};
use crate::variable::Variable;
use cobject::ccolor;
use std::io;

pub fn init(env: &mut Environment, _tenv: &mut TypeEnvironment) {
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

    env.declare_native("ptr::new", |env, _tenv, values, span| {
        if values.len() != 1 {
            error(
                span.line,
                span.column,
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

    env.declare_native("ptr::deref", |env, _tenv, values, span| {
        if values.len() != 1 {
            error(
                span.line,
                span.column,
                format!(
                    "ptr::deref expects exactly 1 argument, got {}.",
                    values.len()
                )
                .as_str(),
            );
            return nil();
        }

        let id = values[0].clone();

        env.get_ptr(str::parse::<usize>(&id.value).unwrap())
            .value
            .clone()
    });

    env.declare_native("ptr::free", |env, _tenv, values, span| {
        if values.len() != 1 {
            error(
                span.line,
                span.column,
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

    env.declare_native("ref::new", |env, _tenv, args, span| {
        if args.len() != 1 {
            error(
                span.line,
                span.column,
                "ref::new expected exactly one argument",
            );
            return nil();
        }

        // The argument must be a variable expression
        let var_name = args[0].value.clone();

        // Find the storage index of the variable
        let mut ptr_id: Option<usize> = None;

        for scope in env.scopes.iter().rev() {
            if let Some(id) = scope.get(&var_name) {
                ptr_id = Some(*id);
                break;
            }
        }

        let id = match ptr_id {
            Some(i) => i,
            None => {
                error(
                    span.line,
                    span.column,
                    "Cannot take reference of undefined variable",
                );
                return nil();
            }
        };

        let pointee = env.get_ptr(id);

        let pointee_type = pointee.value.value_type.clone();

        Value {
            value: id.to_string(),
            value_vec: None,
            value_type: Type::with_generics("ref", vec![pointee_type]),
            body: None,
            native: None,
            is_return: false,
        }
    });

    env.declare_native("ref::deref", |env, _tenv, args, span| {
        if args.len() != 1 {
            error(
                span.line,
                span.column,
                "ref::deref expects exactly 1 argument",
            );
            return nil();
        }

        let referer = &args[0];

        if !referer.value_type.has_tag("ref") {
            error(span.line, span.column, "Cannot dereference non-ref type");
            return nil();
        }

        let id = referer.value.parse::<usize>().unwrap();
        env.get_ptr(id).value.clone()
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

    env.declare_native("direct_nth", |_env, _tenv, values, span| {
        if values.len() != 2 {
            error(
                span.line,
                span.column,
                "Expected exactly two arguments for direct_nth",
            );
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

fn native_len(_: &mut Environment, _: &mut TypeEnvironment, args: Vec<Value>, span: Span) -> Value {
    if args.len() != 1 {
        error(span.line, span.column, "len() expects 1 argument");
        return nil();
    }

    let v = args.get(0).unwrap();

    Value {
        value_type: "i32".into(),
        value: if v.value_type.has_tag("str") {
            v.value.len().to_string()
        } else if v.value_type.has_tag("vec") {
            v.value_vec.iter().len().to_string()
        } else {
            error(
                span.line,
                span.column,
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

fn native_str_nth(
    _env: &mut Environment,
    _: &mut TypeEnvironment,
    args: Vec<Value>,
    span: Span,
) -> Value {
    if args.len() != 2 {
        error(
            span.line,
            span.column,
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
    if right.value_type.has_tag("i32") {
        error(
            span.line,
            span.column,
            "str_nth() expects an 'i32' as right argument",
        );
        return nil();
    }
    if left.value_type.has_tag("str") {
        error(
            span.line,
            span.column,
            "str_nth() expects an 'str' as left argument",
        );
        return nil();
    }

    if str::parse::<usize>(right.value.as_str()).unwrap() >= left.value.chars().count() {
        error(
            span.line,
            span.column,
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

fn native_vec_push(
    env: &mut Environment,
    _tenv: &mut TypeEnvironment,
    args: Vec<Value>,
    span: Span,
) -> Value {
    if args.len() != 2 {
        error(span.line, span.column, "vec::push() expects 2 arguments");
        return nil();
    }

    let ref_value = &args[0];
    let elem = args[1].clone();

    // Ensure first argument is ref<vec<T>>
    if ref_value.value_type.has_tag("ref") {
        error(
            span.line,
            span.column,
            "vec::push() expects ref as first argument",
        );
        return nil();
    }

    let ref_generics = ref_value.value_type.generics();
    if ref_generics.len() != 1 {
        error(span.line, span.column, "Malformed ref type");
        return nil();
    }

    let vec_type = &ref_generics[0];

    if vec_type.has_tag("vec") {
        error(
            span.line,
            span.column,
            "vec::push() expects ref<<vec>> as first argument",
        );
        return nil();
    }

    let vec_generics = vec_type.generics();
    if vec_generics.len() != 1 {
        error(span.line, span.column, "Malformed vec type");
        return nil();
    }

    let inner_type = &vec_generics[0];

    if &elem.value_type != inner_type {
        error(
            0,
            0,
            format!("vec::push expected {}, got {}", inner_type, elem.value_type).as_str(),
        );
        return nil();
    }

    // 🔥 REAL FIX STARTS HERE

    // Extract heap pointer index
    let ptr_id = match ref_value.value.parse::<usize>() {
        Ok(id) => id,
        Err(_) => {
            error(span.line, span.column, "Invalid ref pointer index");
            return nil();
        }
    };

    // Get mutable heap variable
    let heap_var = env.get_ptr(ptr_id);

    // Ensure stored value is actually a vector
    if heap_var.value.value_vec.is_none() {
        error(span.line, span.column, "Referenced value is not a vector");
        return nil();
    }

    // Mutate vector IN PLACE
    heap_var.value.value_vec.as_mut().unwrap().push(elem);

    nil()
}

fn native_vec_nth(
    _: &mut Environment,
    _: &mut TypeEnvironment,
    args: Vec<Value>,
    span: Span,
) -> Value {
    if args.len() != 2 {
        error(span.line, span.column, "vec::nth() expects 2 arguments");
        return nil();
    }

    let vec_val = &args[0];
    let index_val = &args[1];

    if vec_val.value_type.has_tag("vec") {
        error(
            span.line,
            span.column,
            "vec::nth() expects vec<T> as first argument",
        );
        return nil();
    }

    if index_val.value_type.has_tag("i32") {
        error(span.line, span.column, "vec::nth() expects i32 as index");
        return nil();
    }

    let vec = vec_val.value_vec.as_ref().unwrap();
    let index = index_val.value.parse::<usize>().unwrap();

    if index >= vec.len() {
        error(span.line, span.column, "vec::nth() index out of bounds");
        return nil();
    }

    vec[index].clone()
}

fn native_type_check(
    env: &mut Environment,
    tenv: &mut TypeEnvironment,
    args: Vec<Value>,
    _span: Span,
) -> Value {
    Str(args[0].value_type.clone().to_string()).value(env, tenv)
}

fn native_init_window(
    env: &mut Environment,
    _: &mut TypeEnvironment,
    args: Vec<Value>,
    span: Span,
) -> Value {
    if args.len() != 1 {
        error(span.line, span.column, "init_window() expects 1 argument");
    }
    env.make_window(args[0].value.clone());
    env.get_window().init();
    nil()
}

fn native_draw_window(
    env: &mut Environment,
    _: &mut TypeEnvironment,
    args: Vec<Value>,
    span: Span,
) -> Value {
    if !args.is_empty() {
        error(span.line, span.column, "show_window() expects no argument");
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
    span: Span,
) -> Value {
    if !args.is_empty() {
        error(
            span.line,
            span.column,
            "is_window_open() expects no argument",
        );
    }

    let window = env.get_window();
    Expr::Bool(window.is_open()).value(env, tenv)
}
