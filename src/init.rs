use crate::environment::Environment;
use crate::expr::Expr;
use crate::expr::Expr::{Num, Str};
use crate::value::nil;
use crate::error;

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
        Box::new(Expr::Custom(|| {
            error(0, 0, "Manual exit");
            nil()
        })),
        "[]",
        vec![],
        false,
    );
}
