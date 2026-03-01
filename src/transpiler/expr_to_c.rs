use crate::expr::Expr;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;

impl Expr {
    pub fn to_c(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext) {
        match self {
            Expr::Int(n) => {
                ctx.output.push_str(&n.to_string());
            }
            Expr::Add(l, r) => {
                self.op(cte, ctx, l, r, "+");
            }
            Expr::Sub(l, r) => {
                self.op(cte, ctx, l, r, "-");
            }
            Expr::Mult(l, r) => {
                self.op(cte, ctx, l, r, "*");
            }
            Expr::Div(l, r) => {
                self.op(cte, ctx, l, r, "/");
            }
            Expr::StmtBlockNoScope(exprs) => {
                for expr in exprs {
                    expr.to_c(cte, ctx);
                }
            }
            Expr::StmtBlock(exprs) => {
                ctx.output.push('{');
                for expr in exprs {
                    expr.to_c(cte, ctx);
                }
                ctx.output.push('}');
            }
            // … other cases …
            _ => {}
        }
    }
    fn op(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext, l: &Expr, r: &Expr, op: &str) {
        ctx.output.push('(');
        l.to_c(cte, ctx);
        ctx.output.push_str(op);
        r.to_c(cte, ctx);
        ctx.output.push(')');
    }
}
