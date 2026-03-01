use crate::error;
use crate::expr::Expr;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use crate::type_env::{nil_type, Type};

impl Expr {
    pub fn to_c(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext) {
        match self {
            Expr::Int(n) => {
                ctx.output.push_str(&n.to_string());
            }
            Expr::Add(l, r) => {
                self.func(cte, ctx, &[l, r], "_add");
            }
            Expr::Sub(l, r) => {
                self.func(cte, ctx, &[l, r], "_sub");
            }
            Expr::Mult(l, r) => {
                self.func(cte, ctx, &[l, r], "_mult");
            }
            Expr::Div(l, r) => {
                self.func(cte, ctx, &[l, r], "_div");
            }
            Expr::StmtBlockNoScope(exprs) => {
                for expr in exprs {
                    expr.to_c(cte, ctx);
                    ctx.output.push('\n');
                }
            }
            Expr::StmtBlock(exprs) => {
                ctx.output.push_str("{\n");
                for expr in exprs {
                    expr.to_c(cte, ctx);
                    ctx.output.push('\n');
                }
                ctx.output.push('}');
            }

            Expr::Print(expr) => self.func(cte, ctx, &[expr], "_print"),

            Expr::Discard(expr) => {
                expr.to_c(cte, ctx);
                ctx.output.push(';');
            }

            Expr::Variable(name, span) => {
                ctx.output.push_str(&cte.c_var_name(name, *span));
            }

            Expr::Declare(name, var_type, expr, is_mutable, span) => {
                if cte.var_exists(name) {
                    error(
                        *span,
                        format!("Variable '{}' already exists", name).as_str(),
                    );
                }
                cte.declare_var(name.clone());
                ctx.output += format!(
                    "{} {} {} = ",
                    if *is_mutable { "" } else { "const" },
                    cte.c_type_name(&var_type.clone().unwrap(), *span),
                    cte.c_var_name(&name, *span),
                )
                .as_str();
                expr.clone().unwrap().to_c(cte, ctx);
                ctx.output.push_str(";\n");
            }

            Expr::CallFunc(name, _gens, exprs, span) => {
                if !cte.var_exists(name) {
                    error(*span, &format!("Function '{}' does not exist", name));
                }
                ctx.output
                    .push_str(format!("{}(", cte.c_var_name(name, *span)).as_str());
                for expr in exprs {
                    expr.to_c(cte, ctx);
                    ctx.output.push(',');
                }
                if exprs.len() >= 1 {
                    ctx.output.pop();
                }

                ctx.output.push_str(")");
            }

            Expr::Stmt(expr) => {
                expr.to_c(cte, ctx);
                ctx.output.push_str(";\n");
            }

            Expr::DeclareFunction(name, block, return_type, _is_mutable, args, _gens, span) => {
                if cte.var_exists(name) {
                    error(
                        *span,
                        format!(
                            "Variable '{}' already exists, could not declare function",
                            name
                        )
                        .as_str(),
                    );
                }
                cte.declare_var(name.clone());
                ctx.output.push_str(
                    format!(
                        "{} {}(",
                        cte.c_type_name(return_type, *span),
                        cte.c_var_name(&name, *span),
                    )
                    .as_str(),
                );

                for arg in args {
                    cte.declare_var(arg.0.clone());
                    ctx.output.push_str(&cte.c_type_name(&arg.1, *span));
                    ctx.output.push(' ');
                    ctx.output.push_str(&cte.c_var_name(&arg.0, *span));
                    ctx.output.push_str(", ");
                }
                if ctx.output.ends_with(", ") {
                    ctx.output.pop();
                    ctx.output.pop();
                }

                ctx.output.push(')');

                block.to_c(cte, ctx);
                // ctx.output.push_str("}\n\n");
            }

            Expr::Return(expr) => {
                ctx.output.push_str("return ");
                expr.to_c(cte, ctx);
            }

            _ => panic!("unexpected expression '{:?}'", self),
        }
    }
    fn func(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext, args: &[&Expr], name: &str) {
        ctx.output.push_str(name);
        ctx.output.push('(');
        args[0].to_c(cte, ctx);
        for arg in args[1..].iter() {
            ctx.output.push(',');
            arg.to_c(cte, ctx);
        }

        ctx.output.push(')');
    }
    pub fn get_type(&self, _cte: &CompileTimeEnv) -> Type {
        match self {
            _ => nil_type(),
        }
    }
}
