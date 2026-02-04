use crate::environment::Environment;
use crate::expr::Expr;
use crate::expr::Expr::{Num, Str};
use crate::value::{nil, Value};
use crate::error;
use std::io;

pub fn init(env: &mut Environment) {
    env.make_func("i32::new", Box::new(Num(0.0)), "i32", vec![], false);
    env.make_func("f64::new", Box::new(Num(0.0)), "f64", vec![], false);
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
            }
        })),
        "str",
        vec![],
        false,
    );
    // env.make_func(
    //     "str¬f64",
    //     Box::new({
    //         compile(
    //             "\
    //         \
    //         \
    //         \
    //         \
    //         \
    //         "
    //             .to_string(),
    //         )
    //     }),
    //     "f64",
    //     vec![("s".to_string(), "str".to_string())],
    //     false,
    // );
}
