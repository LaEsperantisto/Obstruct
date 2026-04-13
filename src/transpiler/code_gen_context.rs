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
    /// The constructor for CodeGenContext
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

    /// Combines all the parts of the variable into one single String.
    pub fn combine(&mut self, cte: &mut CompileTimeEnv) -> String {
        self.include.push_str(
            "
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef int32_t t_0CD; // i32
typedef void t_1CD; // []
typedef double t_2CD; // f64
typedef bool t_3CD; // bool
typedef char t_4CD; // char
// typedef func t_5; // func - commented out as func is not a C type

t_1CD v_0s_0Ct_0CDD(t_0CD i) { // print i32
    printf(\"%d\",i);
}

t_0CD v_1s_0CD(t_0CD n1, t_0CD n2) { // add
    return n1 + n2;
}

t_0CD v_2s_0CD(t_0CD n1, t_0CD n2) { // less
    return n1 < n2;
}

t_0CD v_3s_0CD(t_0CD n1, t_0CD n2) { // sub
    return n1 - n2;
}

",
        );
        self.body.push_str(
            "
int main() {\n    ",
        );
        self.body
            .push_str(&cte.c_func_instance_name("main", &[], Span::empty()));
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
