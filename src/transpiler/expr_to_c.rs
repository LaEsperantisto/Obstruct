use crate::expr::{Expr, UseKind};
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use crate::type_env::{nil_type, Type};
use crate::{error, STD_PATH};
use std::collections::HashMap;
use std::path::Path;

/// Register local variable declarations from a block so they are visible to
/// returned_type() / get_type() lookups. This only declares variables (no
/// C code is generated).
fn declare_locals_from_block(cte: &mut CompileTimeEnv, block: &Expr) {
    match block {
        Expr::StmtBlock(exprs, _) | Expr::StmtBlockWithScope(exprs, _) => {
            for expr in exprs {
                // Handle both bare Declare and Declare wrapped in Discard
                match expr.as_ref() {
                    Expr::Declare(name, var_type, expr_opt, is_mutable, _) => {
                        let ty = if let Some(vt) = var_type {
                            vt.clone()
                        } else if let Some(e) = expr_opt {
                            e.get_type(cte)
                        } else {
                            nil_type()
                        };
                        cte.declare_var(name.clone(), *is_mutable, ty);
                    }
                    Expr::Discard(inner) => {
                        if let Expr::Declare(name, var_type, expr_opt, is_mutable, _) =
                            inner.as_ref()
                        {
                            let ty = if let Some(vt) = var_type {
                                vt.clone()
                            } else if let Some(e) = expr_opt {
                                e.get_type(cte)
                            } else {
                                nil_type()
                            };
                            cte.declare_var(name.clone(), *is_mutable, ty);
                        }
                    }
                    _ => {}
                }
                // Recurse into nested blocks and function bodies
                declare_locals_from_block(cte, expr);
            }
        }
        Expr::Discard(inner) => declare_locals_from_block(cte, inner),
        _ => {}
    }
}

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
                ctx.body.push_str(if *b { "true" } else { "false" });
                false
            }
            Expr::Char(c) => {
                ctx.body.push('\'');
                let c = c.chars().collect::<Vec<char>>()[0];
                match c {
                    '\n' => ctx.body.push_str("\\n"),
                    '\t' => ctx.body.push_str("\\t"),
                    '\0' => ctx.body.push_str("\\0"),
                    _ => ctx.body.push(c),
                }
                ctx.body.push('\'');
                false
            }
            Expr::Str(s) => {
                ctx.body.push('"');
                for c in s.chars() {
                    match c {
                        '\n' => ctx.body.push_str("\\n"),
                        '\t' => ctx.body.push_str("\\t"),
                        '\0' => ctx.body.push_str("\\0"),
                        _ => ctx.body.push(c),
                    }
                }
                ctx.body.push('"');
                false
            }

            Expr::Add(l, r, span) => {
                let left_type = l.get_type(cte);
                let right_type = r.get_type(cte);
                if left_type.name() == "strlit" && right_type.name() == "char" {
                    // strlit + char: use dedicated strlit+char helper
                    ctx.body.push_str("v_1s_0Ct_6Ct_4CDD(");
                    l.to_c(cte, ctx);
                    ctx.body.push_str(",");
                    r.to_c(cte, ctx);
                    ctx.body.push_str(")");
                } else {
                    Expr::CallFunc(
                        "_add".into(),
                        vec![left_type],
                        vec![l.clone(), r.clone()],
                        *span,
                    )
                    .to_c(cte, ctx);
                }
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

            Expr::Not(expr, span) => {
                Expr::CallFunc("not".into(), vec![], vec![expr.clone()], *span).to_c(cte, ctx);
                false
            }

            Expr::Or(l, r, span) => {
                Expr::CallFunc("or".into(), vec![], vec![l.clone(), r.clone()], *span)
                    .to_c(cte, ctx);
                false
            }

            Expr::And(l, r, span) => {
                Expr::CallFunc("and".into(), vec![], vec![l.clone(), r.clone()], *span)
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
                        "transpiling",
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
                        "transpiling",
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

                let var_type_name = cte.c_type_name(&var_type, ctx, *span);
                let var_name = cte.c_var_name(&name, *span);
                ctx.body += format!("{} {}", var_type_name, var_name).as_str();

                if expr.is_some() {
                    ctx.body.push('=');
                    expr.clone().unwrap().to_c(cte, ctx);
                }
                true
            }

            Expr::CallFunc(name, gens, exprs, span) => {
                if !cte.var_exists(name) {
                    error(
                        *span,
                        &format!("Function '{}' does not exist", name),
                        "transpiling",
                    );
                }
                ctx.body
                    .push_str(format!("{}(", cte.c_func_instance_name(name, gens, *span)).as_str());
                for expr in exprs.iter() {
                    // Check if this argument is a struct — if so, pass by pointer
                    let arg_type = expr.get_type(cte);
                    if cte.is_class(&arg_type) {
                        ctx.body.push_str("&");
                        expr.to_c(cte, ctx);
                    } else {
                        expr.to_c(cte, ctx);
                    }
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

                // Push scope and declare parameters. Struct params use pointers in C.
                cte.push_scope();
                for arg in args {
                    cte.declare_var(arg.0.clone(), arg.2, arg.1.clone());
                }

                // Infer return type from return statements in the body.
                // Parameters are already declared in scope. We also need to declare
                // local variables from the body so they're resolvable.
                declare_locals_from_block(cte, block);
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

                let return_type_name = cte.c_type_name(&return_type, ctx, *span);
                let func_name = cte.c_func_instance_name(&name, &[], *span);
                ctx.body
                    .push_str(&format!("{} {}(", return_type_name, func_name));

                let mut params: Vec<String> = Vec::new();
                for arg in args {
                    let param = cte.c_param_type(&arg.1, ctx, *span);
                    let var = cte.c_var_name(&arg.0, *span);
                    params.push(format!("{} {}", param, var));
                }
                ctx.body.push_str(&params.join(", "));
                if ctx.body.ends_with(", ") {
                    ctx.body.pop();
                    ctx.body.pop();
                }

                ctx.body.push(')');

                block.to_c(cte, ctx);

                cte.pop_scope();

                false
            }

            Expr::Assign(left, right, span) => {
                match left.as_ref() {
                    Expr::Variable(name, _) => {
                        let var_info = cte.get_var(name).unwrap_or_else(|| {
                            error(
                                *span,
                                &format!("Could not find variable '{}'", name),
                                "transpiling",
                            );
                            (false, nil_type())
                        });

                        cte.push_this(name);

                        if var_info.1.name() == "ref" {
                            ctx.body.push_str("(*");
                            ctx.body.push_str(&cte.c_var_name(name, *span));
                            ctx.body.push(')');
                        } else {
                            ctx.body.push_str(&cte.c_var_name(name, *span));
                        }

                        ctx.body.push('=');
                        right.to_c(cte, ctx);
                        cte.pop_this();
                    }
                    Expr::Member(expr, member, span) => {
                        let var_type = expr.get_type(cte);

                        let member_type =
                            cte.get_member_type(&var_type, member).unwrap_or_else(|| {
                                error(
                                    *span,
                                    &format!("Could not find member '{}'", member),
                                    "transpiling",
                                );
                                nil_type()
                            });

                        if member_type.name() == "ref" {
                            ctx.body.push_str("(*");
                        }

                        expr.to_c(cte, ctx);

                        if member_type.name() == "ref" {
                            ctx.body.push(')');
                        }

                        ctx.body.push('.');

                        ctx.body
                            .push_str(&cte.c_member_name(&var_type, member, *span));

                        ctx.body.push('=');
                        right.to_c(cte, ctx);
                    }
                    _ => {
                        error(*span, "Invalid assignment target", "transpiler");
                    }
                }
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

            Expr::Use { .. } => false,

            Expr::Class(ty, members, span) => {
                let c_type_name = cte.c_type_name(ty, ctx, *span);
                ctx.types.push_str("struct ");
                ctx.types.push_str(&c_type_name);
                ctx.types.push_str(" {\n");
                let member_lines: Vec<String> = members
                    .iter()
                    .map(|(name, member_type)| {
                        let ty_name = cte.c_type_name(member_type, ctx, *span);
                        let m_name = cte.c_member_name(ty, name, *span);
                        format!("{} {}", ty_name, m_name)
                    })
                    .collect();
                ctx.types.push_str(&member_lines.join(";\n"));
                ctx.types.push_str(";\n");
                if ctx.types.ends_with("; \n") {
                    ctx.types.pop();
                    ctx.types.pop();
                    ctx.types.push('\n');
                }
                ctx.types.push_str("};\n");

                true
            }

            Expr::Member(expr, member, span) => {
                let var_type = expr.get_type(cte);
                expr.to_c(cte, ctx);

                ctx.body.push('.');

                ctx.body
                    .push_str(&cte.c_member_name(&var_type, member, *span));
                false
            }

            Expr::Ref(expr, _span) => {
                ctx.body.push('&');
                expr.to_c(cte, ctx);
                false
            }
            Expr::Deref(expr, _span) => {
                ctx.body.push_str("(*");
                expr.to_c(cte, ctx);
                ctx.body.push(')');
                false
            }

            Expr::Nth(left, right, _span) => {
                left.to_c(cte, ctx);

                ctx.body.push('[');
                right.to_c(cte, ctx);
                ctx.body.push(']');

                false
            }

            _ => panic!("unexpected expression (for transpilation) '{:?}'", self),
        }
    }

    pub fn pre_transpile(
        &self,
        cte: &mut CompileTimeEnv,
        ctx: &mut CodeGenContext,
        programs_to_transpile: &mut HashMap<String, bool>,
        current_file_dir: &str,
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
                        .as_str(),"pre-transpiling"
                );
                }
                let mut arg_types = vec![];
                for arg in args {
                    arg_types.push(arg.1.clone());
                }

                // Push scope and declare parameters.
                cte.push_scope();
                for arg in args {
                    cte.declare_var(arg.0.clone(), arg.2, arg.1.clone());
                }

                // Process the body first so local variable declarations (like #x = ...)
                // are registered in the scope. This makes them visible to returned_type()
                // and get_type() calls that resolve variable names.
                block.pre_transpile(cte, ctx, programs_to_transpile, current_file_dir);

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
                cte.declare_global_var(
                    name.clone(),
                    false,
                    Type::with_generics("func", {
                        arg_types.push(return_type.clone());
                        let output = arg_types;
                        output
                    }),
                );

                let return_type_name = cte.c_type_name(&return_type, ctx, *span);
                let func_name = cte.c_func_instance_name(&name, &[], *span);

                let mut decl_params: Vec<String> = Vec::new();
                for arg in args {
                    let param = cte.c_param_type(&arg.1, ctx, *span);
                    let var = cte.c_var_name(&arg.0, *span);
                    decl_params.push(format!("{} {}", param, var));
                }
                ctx.declarations
                    .push_str(&format!("{} {}(", return_type_name, func_name));
                ctx.declarations.push_str(&decl_params.join(", "));
                ctx.declarations.push_str(");\n");

                cte.pop_scope();
            }
            Expr::Use { kind, path, span } => {
                let full_path = match kind {
                    UseKind::Std => format!("{}{}", STD_PATH, path),
                    UseKind::Normal => {
                        if current_file_dir.is_empty() {
                            path.to_string()
                        } else {
                            format!("{}/{}", current_file_dir, path)
                        }
                    }
                };

                if !Path::new(&full_path).exists() {
                    error(*span, &format!("File at path '{}' does not exist!", full_path), "pre-transpiling");
                    return;
                }

                if !programs_to_transpile.contains_key(&full_path) {
                    programs_to_transpile.insert(full_path.clone(), true);
                    // Process this file immediately so its functions are available to subsequent
                    // declarations in the current file
                    let source = std::fs::read_to_string(&full_path)
                        .unwrap_or_else(|_| panic!("Failed to read {}", full_path));
                    let imported_ast = {
                        let mut scanner = crate::scanner::Scanner::new(source.clone());
                        let tokens = scanner.scan_tokens();
                        let mut parser = crate::parser::Parser::new(tokens);
                        parser.parse()
                    };
                    let imported_dir = std::path::Path::new(&full_path)
                        .parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    imported_ast.pre_transpile(cte, ctx, programs_to_transpile, &imported_dir);
                }
            }

            Expr::StmtBlock(exprs, _) | Expr::StmtBlockWithScope(exprs, _) => {
                for expr in exprs {
                    expr.pre_transpile(cte, ctx, programs_to_transpile, current_file_dir);
                }
            }

            Expr::Discard(expr) => {
                expr.pre_transpile(cte, ctx, programs_to_transpile, current_file_dir)
            }

            Expr::Class(ty, members, span) => {
                cte.register_class(ty.clone());

                for member in members {
                    cte.declare_member(member.0.clone(), member.1.clone(), ty.clone());
                }

                let c_type_name = cte.c_type_name(ty, ctx, *span);
                ctx.include.push_str("struct ");
                ctx.include.push_str(&c_type_name);
                ctx.include.push_str(";\n");

                ctx.types.push_str("\ntypedef struct ");
                ctx.types.push_str(&c_type_name);
                ctx.types.push(' ');
                ctx.types.push_str(&c_type_name);
                ctx.types.push_str(";\n");
            }

            Expr::Declare(name, var_type, expr, is_mutable, _span) => {
                if let Some(var_type) = var_type {
                    cte.declare_var(name.clone(), *is_mutable, var_type.clone());
                } else if let Some(expr_box) = expr {
                    let t = expr_box.get_type(cte);
                    cte.declare_var(name.clone(), *is_mutable, t);
                }
                if let Some(expr_box) = expr {
                    expr_box.pre_transpile(cte, ctx, programs_to_transpile, current_file_dir);
                }
            }

            _ => {}
        }
    }

    fn get_type(&self, cte: &mut CompileTimeEnv) -> Type {
        match self {
            Expr::Int(_) => "i32".into(),
            Expr::Float(_) => "f64".into(),
            Expr::Str(_) => "strlit".into(),
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
            Expr::Return(_, _span) => nil_type(),
            Expr::Nth(..) => "char".into(),
            Expr::CallFunc(name, _, _, span) => {
                let function = cte
                    .get_var(name)
                    .unwrap_or_else(|| {
                        error(
                            *span,
                            &format!("Function not defined: {}", name),
                            "type checker",
                        );
                        (false, nil_type())
                    })
                    .1;
                function.generics().last().cloned().unwrap_or_else(nil_type)
            }
            Expr::StmtBlock(exprs, _span) => exprs.last().unwrap().get_type(cte),
            Expr::StmtBlockWithScope(exprs, _span) => exprs.last().unwrap().get_type(cte),
            Expr::Variable(name, span) => {
                cte.get_var(name)
                    .unwrap_or_else(|| {
                        error(
                            *span,
                            &format!("Variable not defined: {}", name),
                            "type checker",
                        );
                        (false, nil_type())
                    })
                    .1
            }
            Expr::Discard(..) => nil_type(),
            Expr::Print(expr, _) => expr.get_type(cte),
            Expr::If(_, block, ..) => block.get_type(cte),
            Expr::Member(expr, member_name, span) => {
                let var_type = expr.get_type(cte);

                let member = cte.get_member_type(&var_type, member_name);

                let member_type = if member.is_none() {
                    error(
                        *span,
                        &format!(
                            "Member '{}' not defined for type '{}'",
                            member_name, var_type
                        ),
                        "type checker",
                    );
                    nil_type()
                } else {
                    member.unwrap()
                };

                member_type
            }

            Expr::Ref(expr, _span) => {
                let expr_type = expr.get_type(cte);
                cte.register_type(expr_type.clone());
                Type::with_generics("ref", vec![expr_type])
            }
            Expr::Deref(expr, span) => {
                let expr_type = expr.get_type(cte);
                if expr_type.name() != "ref" {
                    error(*span, "Could not deref non-reference", "type checker");
                    return nil_type();
                }
                expr_type.generics()[0].clone()
            }
            _ => panic!("unexpected expression (for type check) '{:?}'", self),
        }
    }

    fn returned_type(&self, cte: &mut CompileTimeEnv, span: Span) -> Option<Type> {
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
                                    "returned type checker",
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
