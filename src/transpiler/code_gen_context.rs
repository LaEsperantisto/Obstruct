use crate::span::Span;
use crate::transpiler::compiletime_env::CompileTimeEnv;

pub struct CodeGenContext {
    pub include: String,
    pub types: String,
    pub declarations: String,
    pub unnamed: String,
    pub body: String,
}

impl CodeGenContext {
    pub fn new() -> CodeGenContext {
        let ctx = CodeGenContext {
            include: String::new(),
            types: String::new(),
            body: String::new(),
            declarations: String::new(),
            unnamed: String::new(),
        };
        ctx
    }

    pub fn combine(&mut self, cte: &mut CompileTimeEnv) -> String {
        self.include.push_str(
            "
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef int32_t t_0; // i32
typedef void t_1; // []
typedef double t_2; // f64
typedef bool t_3; // bool
typedef char t_4; // char

t_1 v_0s_0(t_0 i) { // print i32
    printf(\"%d\",i);
}

t_0 v_1s_0(t_0 n1, t_0 n2) { // add
    return n1 + n2;
}

t_0 v_2s_0(t_0 n1, t_0 n2) { // less
    return n1 < n2;
}

t_0 v_3s_0(t_0 n1, t_0 n2) { // sub
    return n1 - n2;
}

",
        );
        self.body.push_str(
            "
int main() {\n    ",
        );
        self.body.push_str(&cte.c_var_name("main", Span::empty()));
        self.body.push_str("();\n}");

        self.include.clone()
            + "\n\n"
            + &self.types
            + "\n\n"
            + &self.declarations
            + "\n\n"
            + &self.unnamed
            + "\n\n"
            + &self.body
    }
}
