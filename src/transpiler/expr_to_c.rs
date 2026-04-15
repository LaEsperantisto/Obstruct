use crate::expr::{Expr, UseKind};
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use crate::type_env::{nil_type, Type};
use crate::{error, STD_PATH};
use std::collections::HashMap;
use std::path::Path;

impl Expr {
    pub fn to_c(&self, cte: &mut CompileTimeEnv, ctx: &mut CodeGenContext) -> bool {
        // if true - requires a semicolon at end of statement
        match self {
            Expr::Int(n) => {
                ctx.body.push_str(&n.to_string());
                false
            }
            Expr::Float(n) => {
                if n.fract() == 0.0 {
                    ctx.body.push_str(&n.to_string());
                    ctx.body.push_str(".0");
                } else {
                    ctx.body.push_str(&n.to_string());
                }
                false
            }

            Expr::Bool(b) => {
                ctx.body.push_str(if *b { "1" } else { "0" });
                false
            }
            Expr::Char(c) => {
                ctx.body.push('\'');
                ctx.body.push_str(&c.to_string());
                ctx.body.push('\'');
                false
            }
            Expr::Str(s) => {
                ctx.body.push('"');
                for c in s.chars() {
                    match c {
                        '\n' => ctx.body.push_str("\\n"),
                        _ => ctx.body.push(c),
                    }
                }
                ctx.body.push('"');
                false
            }

            Expr::Add(l, r, span) => {
                Expr::CallFunc(
                    "_add".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::Div(l, r, span) => {
                Expr::CallFunc(
                    "_div".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::Less(l, r, span) => {
                Expr::CallFunc(
                    "_less".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::Power(l, r, span) => {
                Expr::CallFunc(
                    "_pow".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }
            Expr::Sub(l, r, span) => {
                Expr::CallFunc(
                    "_sub".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }
            Expr::Mult(l, r, span) => {
                Expr::CallFunc(
                    "_mult".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }
            Expr::EqualEqual(l, r, span) => {
                Expr::CallFunc(
                    "_equal".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }
            Expr::Greater(l, r, span) => {
                Expr::CallFunc(
                    "_greater".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }
            Expr::GreaterEqual(l, r, span) => {
                Expr::CallFunc(
                    "_greater_equal".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::LessEqual(l, r, span) => {
                Expr::CallFunc(
                    "_less_equal".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::BangEqual(l, r, span) => {
                Expr::CallFunc(
                    "_bang_equal".into(),
                    vec![l.get_type(cte)],
                    vec![l.clone(), r.clone()],
                    *span,
                )
                .to_c(cte, ctx);
                false
            }

            Expr::StmtBlock(exprs, _span) => {
                for expr in exprs {
                    if expr.to_c(cte, ctx) {
                        ctx.body.push(';');
                    }
                    ctx.body.push('\n');
                }
                false
            }

            Expr::If(if_cond, if_block, else_block, is_expr) => {
                if !is_expr {
                    ctx.body.push_str("if (");
                    if_cond.to_c(cte, ctx);
                    ctx.body.push_str(")");
                    if_block.to_c(cte, ctx);
                    if else_block.is_some() {
                        ctx.body.push_str(" else ");
                        else_block.clone().unwrap().to_c(cte, ctx);
                    }
                } else {
                    error(
                        if_cond.get_span(),
                        "EXPRESSION IF STATEMENTS NOT IMPLEMENTED",
                    );
                }

                let else_type = if let Some(ty) = else_block {
                    ty.get_type(cte)
                } else {
                    nil_type()
                };
                if if_block.get_type(cte) != else_type {
                    error(
                        if_block.get_span(),
                        "if and else blocks had different types",
                    );
                }

                false
            }

            Expr::StmtBlockWithScope(exprs, span) => {
                cte.push_scope();
                ctx.body.push_str("{\n");
                Expr::StmtBlock(exprs.clone(), *span).to_c(cte, ctx);
                ctx.body.push('}');
                cte.pop_scope();
                false
            }

            Expr::Print(expr, span) => {
                Expr::CallFunc(
                    "_print".into(),
                    vec![expr.get_type(cte)],
                    vec![expr.clone()],
                    *span,
                )
                .to_c(cte, ctx);
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

            Expr::This(span) => {
                ctx.body.push(' ');
                ctx.body.push_str(&cte.c_var_name(cte.this(), *span));
                ctx.body.push(' ');
                false
            }

            Expr::Declare(name, var_type, expr, is_mutable, span) => {
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

            Expr::CallFunc(name, gens, exprs, span) => {
                if !cte.var_exists(name) {
                    error(*span, &format!("Function '{}' does not exist", name));
                }
                ctx.body
                    .push_str(format!("{}(", cte.c_func_instance_name(name, gens, *span)).as_str());
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
                let mut arg_types = vec![];
                for arg in args {
                    arg_types.push(arg.1.clone());
                }
                let ret_type = block.returned_type(cte, *span);
                let return_type = if return_type.is_some() {
                    return_type.clone().unwrap()
                } else {
                    if let Some(ty) = ret_type.clone() {
                        ty
                    } else {
                        nil_type()
                    }
                };

                cte.add_func_type(return_type.clone(), arg_types.clone(), ctx, *span);
                ctx.body.push_str(
                    format!(
                        "{} {}(",
                        cte.c_type_name(&return_type, *span),
                        cte.c_func_instance_name(&name, &[], *span),
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

            Expr::Assign(name, expr, span) => {
                cte.push_this(name);
                ctx.body.push_str(&cte.c_var_name(&name, *span));
                ctx.body.push('=');
                expr.to_c(cte, ctx);
                cte.pop_this();
                true
            }

            Expr::Return(expr, _span) => {
                ctx.body.push_str("return ");
                expr.to_c(cte, ctx);
                true
            }

            Expr::Delete(name) => {
                cte.del_var(name);
                false
            }

            Expr::Nothing() => false,

            Expr::While(cond, block) => {
                ctx.body.push_str("while (");
                cond.to_c(cte, ctx);
                ctx.body.push_str("){\n");
                block.to_c(cte, ctx);
                ctx.body.push_str("}");
                false
            }

            Expr::Input(name) => {
                ctx.body.push_str("scanf(\"%d\",&");
                ctx.body.push_str(&cte.c_var_name(name, Span::empty()));
                ctx.body.push_str(")");
                true
            }

            Expr::Use { .. } => false,

            _ => panic!("unexpected expression (for transpilation) '{:?}'", self),
        }
    }

    pub fn pre_transpile(
        &self,
        cte: &mut CompileTimeEnv,
        ctx: &mut CodeGenContext,
        programs_to_transpile: &mut HashMap<String, bool>,
    ) {
        match self {
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
                let ret_type = block.returned_type(cte, *span);
                let return_type = if return_type.is_some() {
                    return_type.clone().unwrap()
                } else {
                    if let Some(ty) = ret_type.clone() {
                        ty
                    } else {
                        nil_type()
                    }
                };

                cte.add_func_type(return_type.clone(), arg_types.clone(), ctx, *span);
                // Declare the variable first so c_func_instance_name can find it
                cte.declare_var(
                    name.clone(),
                    false,
                    Type::with_generics("func", {
                        arg_types.push(return_type.clone());
                        let output = arg_types;
                        output
                    }),
                );
                ctx.declarations.push_str(
                    format!(
                        "{} {}(",
                        cte.c_type_name(&return_type, *span),
                        cte.c_func_instance_name(&name, &[], *span),
                    )
                    .as_str(),
                );

                for arg in args {
                    cte.declare_var(arg.0.clone(), false, arg.1.clone());
                    ctx.declarations.push_str(&cte.c_type_name(&arg.1, *span));
                    ctx.declarations.push(' ');
                    ctx.declarations.push_str(&cte.c_var_name(&arg.0, *span));
                    ctx.declarations.push_str(", ");
                }
                if ctx.declarations.ends_with(", ") {
                    ctx.declarations.pop();
                    ctx.declarations.pop();
                }

                ctx.declarations.push_str(");\n");

                block.pre_transpile(cte, ctx, programs_to_transpile);
            }
            Expr::Use { kind, path, span } => {
                let crate_path = match kind {
                    UseKind::Std => STD_PATH,
                    UseKind::Normal => "",
                };

                let full_path = format!("{}{}", crate_path, path);

                if !Path::new(&full_path).exists() {
                    error(*span, "File does not exist!");
                }

                programs_to_transpile.insert(full_path, false);
            }

            Expr::StmtBlock(exprs, _) | Expr::StmtBlockWithScope(exprs, _) => {
                for expr in exprs {
                    expr.pre_transpile(cte, ctx, programs_to_transpile);
                }
            }

            _ => {}
        }
    }

    fn get_type(&self, cte: &CompileTimeEnv) -> Type {
        match self {
            Expr::Int(_) => "i32".into(),
            Expr::Float(_) => "f64".into(),
            Expr::Str(_) => "str".into(),
            Expr::Bool(_)
            | Expr::EqualEqual(..)
            | Expr::BangEqual(..)
            | Expr::Less(..)
            | Expr::LessEqual(..)
            | Expr::Greater(..)
            | Expr::GreaterEqual(..) => "bool".into(),
            Expr::Char(_) => "char".into(),
            Expr::Add(l, _, _)
            | Expr::Sub(l, ..)
            | Expr::Mult(l, ..)
            | Expr::Power(l, ..)
            | Expr::Div(l, ..) => l.get_type(cte),
            Expr::Return(expr, _span) => expr.get_type(cte),
            Expr::CallFunc(name, _, _, span) => {
                let variable = cte
                    .get_var(name)
                    .unwrap_or_else(|| {
                        error(*span, "Variable not defined");
                        (false, nil_type())
                    })
                    .1;
                variable.generics()[0].clone()
            }
            Expr::StmtBlock(exprs, _span) => exprs.last().unwrap().get_type(cte),
            Expr::StmtBlockWithScope(exprs, _span) => exprs.last().unwrap().get_type(cte),
            Expr::Variable(name, span) => {
                cte.get_var(name)
                    .unwrap_or_else(|| {
                        error(*span, "Variable not defined");
                        (false, nil_type())
                    })
                    .1
            }
            Expr::Discard(..) => nil_type(),
            Expr::Print(expr, _) => expr.get_type(cte),
            _ => panic!("unexpected expression (for type check) '{:?}'", self),
        }
    }

    fn returned_type(&self, cte: &CompileTimeEnv, span: Span) -> Option<Type> {
        match self {
            Expr::Return(expr, _span) => Some(expr.get_type(cte)),
            Expr::Discard(expr) => expr.returned_type(cte, span),
            Expr::StmtBlock(exprs, span) => {
                let mut return_type = None;
                for expr in exprs {
                    let ret_type = expr.returned_type(cte, *span);
                    if let Some(returned_type) = ret_type {
                        if let Some(expected_type) = return_type.clone() {
                            if expected_type != returned_type {
                                error(
                                    {
                                        let expr_span = expr.get_span();
                                        if expr_span == Span::empty() {
                                            *span
                                        } else {
                                            expr_span
                                        }
                                    },
                                    "Returned type was different to previous returned type",
                                );
                            }
                        } else {
                            return_type = Some(returned_type);
                        }
                    }
                }
                return_type
            }
            Expr::StmtBlockWithScope(exprs, span) => {
                Expr::StmtBlock(exprs.clone(), *span).returned_type(cte, *span)
            }
            _ => None,
        }
    }

    fn get_span(&self) -> Span {
        match self {
            Expr::Add(_, _, span)
            | Expr::Sub(_, _, span)
            | Expr::Mult(_, _, span)
            | Expr::Div(_, _, span)
            | Expr::Power(_, _, span)
            | Expr::Mod(_, _, span)
            | Expr::BangEqual(_, _, span)
            | Expr::EqualEqual(_, _, span)
            | Expr::GreaterEqual(_, _, span)
            | Expr::LessEqual(_, _, span)
            | Expr::Less(_, _, span)
            | Expr::Greater(_, _, span)
            | Expr::Return(_, span) => *span,
            _ => Span::empty(),
        }
    }
}
