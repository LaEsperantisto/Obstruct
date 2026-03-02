use crate::error;
use crate::expr::Expr;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use crate::type_env::{nil_type, Type};

impl Expr {
    pub fn to_c(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext) -> bool {
        // if returns true - requires a semicolon at end of statement
        match self {
            Expr::Int(n) => {
                ctx.body.push_str(&n.to_string());
                false
            }
            Expr::Bool(b) => {
                ctx.body.push_str(if *b { "1" } else { "0" });
                false
            }

            Expr::Add(l, r, span) => {
                Expr::CallFunc("_add".into(), vec![], vec![l.clone(), r.clone()], *span)
                    .to_c(cte, ctx);
                false
            }
            Expr::If(if_cond, if_block, else_block) => {
                ctx.body.push_str("if (");
                if_cond.to_c(cte, ctx);
                ctx.body.push_str(")");
                if_block.to_c(cte, ctx);
                if else_block.is_some() {
                    ctx.body.push_str(" else ");
                    else_block.clone().unwrap().to_c(cte, ctx);
                }

                false
            }
            Expr::Sub(l, r, span) => {
                Expr::CallFunc("_sub".into(), vec![], vec![l.clone(), r.clone()], *span)
                    .to_c(cte, ctx);
                false
            }
            Expr::Mult(l, r, span) => {
                Expr::CallFunc("_mult".into(), vec![], vec![l.clone(), r.clone()], *span)
                    .to_c(cte, ctx);
                false
            }
            Expr::Div(l, r, span) => {
                Expr::CallFunc("_div".into(), vec![], vec![l.clone(), r.clone()], *span)
                    .to_c(cte, ctx);
                false
            }
            Expr::StmtBlockNoScope(exprs) => {
                for expr in exprs {
                    if expr.to_c(cte, ctx) {
                        ctx.body.push(';');
                    }
                    ctx.body.push('\n');
                }
                false
            }
            Expr::StmtBlock(exprs) => {
                cte.push_scope();
                ctx.body.push_str("{\n");
                Expr::StmtBlockNoScope(exprs.clone()).to_c(cte, ctx);
                ctx.body.push('}');
                cte.pop_scope();
                false
            }

            Expr::Print(expr, span) => {
                Expr::CallFunc("_print".into(), vec![], vec![expr.clone()], *span).to_c(cte, ctx);
                true
            }

            Expr::Discard(expr) => {
                expr.to_c(cte, ctx);
                true
            }

            Expr::Variable(name, span) => {
                ctx.body.push_str(&cte.c_var_name(name, *span));
                false
            }

            Expr::Declare(name, var_type, expr, is_mutable, span) => {
                if cte.var_exists(name) {
                    error(
                        *span,
                        format!("Variable '{}' already exists", name).as_str(),
                    );
                }
                let var_type = if var_type.is_some() {
                    cte.declare_var(name.clone(), *is_mutable, var_type.clone().unwrap());
                    var_type.clone().unwrap()
                } else {
                    let t = expr.clone().unwrap().get_type(cte);
                    cte.declare_var(name.clone(), *is_mutable, t.clone());
                    t
                };

                ctx.body += format!(
                    "{} {}=",
                    cte.c_type_name(&var_type, *span),
                    cte.c_var_name(&name, *span),
                )
                .as_str();
                expr.clone().unwrap().to_c(cte, ctx);
                true
            }

            Expr::CallFunc(name, _gens, exprs, span) => {
                if !cte.var_exists(name) {
                    error(*span, &format!("Function '{}' does not exist", name));
                }
                ctx.body
                    .push_str(format!("{}(", cte.c_var_name(name, *span)).as_str());
                for expr in exprs {
                    expr.to_c(cte, ctx);
                    ctx.body.push(',');
                }
                if exprs.len() >= 1 {
                    ctx.body.pop();
                }

                ctx.body.push_str(")");
                true
            }

            Expr::Stmt(expr) => {
                expr.to_c(cte, ctx);
                true
            }

            Expr::DeclareFunction(name, block, return_type, args, _gens, span) => {
                if cte.get_var(name).is_some() {
                    error(
                        *span,
                        format!(
                            "Variable '{}' already exists and is immutable, could not declare function",
                            name
                        )
                        .as_str(),
                    );
                }
                let mut arg_types = vec![];
                for arg in args {
                    arg_types.push(arg.1.clone());
                }
                let return_type = if return_type.is_some() {
                    return_type.clone().unwrap()
                } else {
                    block.get_type(cte)
                };

                cte.declare_var(
                    name.clone(),
                    false,
                    Type::with_generics("func", {
                        let old_arg_types = arg_types.clone();
                        arg_types.push(return_type.clone());
                        let output = arg_types;
                        arg_types = old_arg_types;
                        output
                    }),
                );
                ctx.body.push_str(
                    format!(
                        "{} {}(",
                        cte.c_type_name(&return_type, *span),
                        cte.c_var_name(&name, *span),
                    )
                    .as_str(),
                );

                for arg in args {
                    cte.declare_var(arg.0.clone(), false, arg.1.clone());
                    ctx.body.push_str(&cte.c_type_name(&arg.1, *span));
                    ctx.body.push(' ');
                    ctx.body.push_str(&cte.c_var_name(&arg.0, *span));
                    ctx.body.push_str(", ");
                }
                if ctx.body.ends_with(", ") {
                    ctx.body.pop();
                    ctx.body.pop();
                }

                ctx.body.push(')');

                block.to_c(cte, ctx);
                false
            }

            Expr::Return(expr) => {
                ctx.body.push_str("return ");
                expr.to_c(cte, ctx);
                true
            }

            _ => panic!("unexpected expression '{:?}'", self),
        }
    }
    pub fn get_type(&self, cte: &CompileTimeEnv) -> Type {
        match self {
            Expr::Int(_) => "i32".into(),
            Expr::Float(_) => "f64".into(),
            Expr::Str(_) => "str".into(),
            Expr::Bool(_) => "bool".into(),
            Expr::Return(expr) => expr.get_type(cte),
            Expr::CallFunc(name, _, _, _) => {
                let variable = cte.get_var(name).unwrap().1;
                variable.generics()[0].clone()
            }
            Expr::StmtBlockNoScope(exprs) => exprs.last().unwrap().get_type(cte),
            Expr::StmtBlock(exprs) => exprs.last().unwrap().get_type(cte),
            Expr::Variable(name, _) => cte.get_var(name).unwrap().1,
            Expr::Discard(expr) => expr.get_type(cte),
            _ => nil_type(),
        }
    }
}
