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





",
        );
        ctx
    }
}
