use crate::span::Span;
use crate::transpiler::compiletime_env::CompileTimeEnv;

pub struct CodeGenContext {
    pub include: String,
    pub declarations: String,
    pub unnamed: String,
    pub body: String,
}

impl CodeGenContext {
    pub fn new() -> CodeGenContext {
        let mut ctx = CodeGenContext {
            include: String::new(),
            body: String::new(),
            declarations: String::new(),
            unnamed: String::new(),
        };
        ctx.body.push_str(
            "
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef int32_t t_0;
typedef void t_1;
typedef double t_2;
typedef bool t_3;

void v_0s_0(t_0 i) {
    printf(\"%d\",i);
}

",
        );
        ctx
    }

    pub fn combine(&mut self, cte: &mut CompileTimeEnv) -> String {
        self.body.push_str(
            "
int main() {\n    ",
        );
        self.body.push_str(&cte.c_var_name("main", Span::empty()));
        self.body.push_str("();\n}");

        self.include.clone() + &self.declarations + &self.unnamed + &self.body
    }
}
