pub struct CodeGenContext {
    pub output: String,
}

impl CodeGenContext {
    pub fn new() -> CodeGenContext {
        let mut ctx = CodeGenContext {
            output: String::new(),
        };
        ctx.output.push_str(
            "
#include <stdint.h>
#include <stdio.h>

typedef int32_t t_0;
typedef const char* t_1;
typedef double t_2;
",
        );
        ctx
    }
}
