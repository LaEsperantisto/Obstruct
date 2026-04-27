//! Parser tests
//! Tests AST construction from tokens

// ========== Statement Block ==========

use crate::expr::Expr;

fn parse_source(source: &str) -> crate::expr::Expr {
    crate::parse(source.to_string())
}

#[test]
fn test_parse_empty_block() {
    let expr = parse_source("{};");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => {
            assert_eq!(statements.len(), 1);
        }
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_multiple_statements() {
    let expr = parse_source("#x = 1; #y = 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => {
            assert_eq!(statements.len(), 2);
        }
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Variable Declarations ==========

#[test]
fn test_parse_immutable_declaration() {
    let expr = parse_source("#x = 42;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Declare(name, _, expr_value, mutable, _) => {
                assert_eq!(name, "x");
                assert!(!*mutable);
                match expr_value {
                    Some(boxed) => {
                        if let crate::expr::Expr::Int(42) = boxed.as_ref() {
                        } else {
                            panic!("Expected Int(42)");
                        }
                    }
                    _ => panic!("Expected Some value"),
                }
            }
            _ => panic!("Expected Declare, got {:?}", statements[0]),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_mutable_declaration() {
    let expr = parse_source("#@y = 3.14;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Declare(name, _, expr_value, mutable, _) => {
                assert_eq!(name, "y");
                assert!(*mutable);
                match expr_value {
                    Some(boxed) => {
                        if let crate::expr::Expr::Float(f) = boxed.as_ref() {
                            assert!((f - 3.14).abs() < 0.0001);
                        } else {
                            panic!("Expected Float(3.14)");
                        }
                    }
                    _ => panic!("Expected Some value"),
                }
            }
            _ => panic!("Expected Declare"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_typed_declaration_without_value() {
    let expr = parse_source("#x: i32;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Declare(name, var_type, expr_value, _, _) => {
                assert_eq!(name, "x");
                assert!(var_type.is_some());
                assert!(expr_value.is_none());
            }
            _ => panic!("Expected Declare"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_declaration_with_type_and_value() {
    let expr = parse_source("#x: i32 = 42;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Declare(name, var_type, expr_value, _, _) => {
                assert_eq!(name, "x");
                assert!(var_type.is_some());
                assert!(expr_value.is_some());
            }
            _ => panic!("Expected Declare"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Assignment ==========

#[test]
fn test_parse_assignment() {
    let expr = parse_source("x = 10;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => match inner.as_ref() {
                crate::expr::Expr::Assign(_name, expr_value, _) => {
                    if let crate::expr::Expr::Int(10) = expr_value.as_ref() {
                    } else {
                        panic!("Expected Int(10)");
                    }
                }
                _ => panic!("Expected Assign"),
            },
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== If/Else If/Else ==========

#[test]
fn test_parse_if_statement() {
    let expr = parse_source("? x > 5 { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::If(_, _, _, _) => {}
            _ => panic!("Expected If"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_if_else_if() {
    let expr = parse_source("? x > 5 { ret; } ~? x == 5 { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::If(_, _, else_block, _) => {
                assert!(else_block.is_some());
            }
            _ => panic!("Expected If"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_if_else() {
    let expr = parse_source("? x > 5 { ret; } ~ { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::If(_, _, else_block, _) => {
                assert!(else_block.is_some());
            }
            _ => panic!("Expected If"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== While Loop ==========

#[test]
fn test_parse_while_loop() {
    let expr = parse_source("£ x < 10 { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::While(_, _) => {}
            _ => panic!("Expected While"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== For Loop ==========

#[test]
fn test_parse_for_loop() {
    let expr = parse_source("for i: 0..5 { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::For(_, _, _, _) => {}
            _ => panic!("Expected For"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Print Statements ==========

#[test]
fn test_parse_print() {
    let expr = parse_source(r#"$"Hello";"#);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Print(_, _) => {}
            _ => panic!("Expected Print"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_println() {
    let expr = parse_source(r#"$$"Hello";"#);
    match expr {
        Expr::StmtBlock(statements, _) => {
            if let Expr::StmtBlock(statements, ..) = statements[0].as_ref() {
                match statements[0].as_ref() {
                    Expr::Print(..) => {}
                    _ => panic!(
                        "Expected Print in first statement, found: {:?}",
                        statements[0].as_ref()
                    ),
                }
                match statements[1].as_ref() {
                    Expr::Print(expr, ..) => {
                        if let Expr::Str(_) = &**expr {
                        } else {
                            panic!("Expected Print(Str(_))");
                        }
                    }
                    _ => panic!("Expected Print in second statement"),
                }
            } else {
                panic!("Expected StmtBlock");
            }
        }
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Return Statement ==========

#[test]
fn test_parse_return() {
    let expr = parse_source("ret x;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Return(_, _) => {}
            _ => panic!("Expected Return"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_return_value() {
    let expr = parse_source("ret 42;");
    match expr {
        Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            Expr::Return(inner, _) => {
                if let crate::expr::Expr::Int(42) = inner.as_ref() {
                } else {
                    panic!("Expected Int(42)");
                }
            }
            _ => panic!("Expected Return"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Delete Statement ==========

#[test]
fn test_parse_delete() {
    let expr = parse_source("del x;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Delete(name) => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected Delete"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Function Definition ==========

#[test]
fn test_parse_function_definition() {
    let expr = parse_source("fn add(a: i32, b: i32) i32 { ret a + b; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::DeclareFunction(name, _, _, params, _, _) => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].0, "a");
                assert_eq!(params[1].0, "b");
            }
            _ => panic!("Expected DeclareFunction"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_function_without_return_type() {
    let expr = parse_source("fn print() { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::DeclareFunction(name, _, return_type, _, _, _) => {
                assert_eq!(name, "print");
                assert!(return_type.is_none());
            }
            _ => panic!("Expected DeclareFunction"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_function_with_generics() {
    let expr = parse_source("fn <<T>> push(v: vec<<T>>, item: T) { ret; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::DeclareFunction(_, _, _, _, generics, _) => {
                assert_eq!(generics.len(), 1);
                assert_eq!(generics[0], "T");
            }
            _ => panic!("Expected DeclareFunction"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Lambda ==========

#[test]
fn test_parse_lambda() {
    let expr = parse_source("#f = lam { ret 42; };");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Declare(_, _, Some(inner), _, _) => {
                if let crate::expr::Expr::Function(_, _, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Function");
                }
            }
            _ => panic!("Expected Declare with lambda"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Binary Operators ==========

#[test]
fn test_parse_addition() {
    let expr = parse_source("1 + 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Add(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Add");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_subtraction() {
    let expr = parse_source("1 - 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Sub(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Sub");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_multiplication() {
    let expr = parse_source("1 * 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Mult(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Mult");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_division() {
    let expr = parse_source("1 / 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Div(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Div");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_modulo() {
    let expr = parse_source("5 % 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Mod(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Mod");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_power() {
    let expr = parse_source("2 ** 3;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Power(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Power");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Comparison Operators ==========

#[test]
fn test_parse_equal() {
    let expr = parse_source("1 == 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::EqualEqual(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected EqualEqual");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_not_equal() {
    let expr = parse_source("1 != 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::BangEqual(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected BangEqual");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_less_than() {
    let expr = parse_source("1 < 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Less(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Less");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_greater_than() {
    let expr = parse_source("2 > 1;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Greater(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Greater");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_less_equal() {
    let expr = parse_source("1 <= 2;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::LessEqual(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected LessEqual");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_greater_equal() {
    let expr = parse_source("2 >= 1;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::GreaterEqual(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected GreaterEqual");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Logical Operators ==========

#[test]
fn test_parse_and() {
    let expr = parse_source("`t & `f;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::And(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected And");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_or() {
    let expr = parse_source("`t | `f;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Or(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Or");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Unary Operators ==========

#[test]
fn test_parse_negation() {
    let expr = parse_source("!x;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Not(..) = inner.as_ref() {
                } else {
                    panic!("Expected Not");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_unary_minus() {
    let expr = parse_source("-5;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Sub(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Sub for unary minus");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Function Calls ==========

#[test]
fn test_parse_function_call() {
    let expr = parse_source("add(1, 2);");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::CallFunc(name, _, args, _) = inner.as_ref() {
                    assert_eq!(name, "add");
                    assert_eq!(args.len(), 2);
                } else {
                    panic!("Expected CallFunc");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_generic_function_call() {
    let expr = parse_source("push<<i32>>(v, 5);");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::CallFunc(name, generics, _, _) = inner.as_ref() {
                    assert_eq!(name, "push");
                    assert_eq!(generics.len(), 1);
                } else {
                    panic!("Expected CallFunc");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Array/Indexing ==========

#[test]
fn test_parse_array_indexing() {
    let expr = parse_source("my_arr[0];");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Nth(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Nth");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_array_literal() {
    let expr = parse_source("[1, 2, 3];");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Array(exprs) = inner.as_ref() {
                    assert_eq!(exprs.len(), 3);
                } else {
                    panic!("Expected Array");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Vector Literal ==========

#[test]
fn test_parse_vector_literal() {
    let expr = parse_source(r#"\{1, 2, 3};"#);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Vector(exprs) = inner.as_ref() {
                    assert_eq!(exprs.len(), 3);
                } else {
                    panic!("Expected Vector");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Literals ==========

#[test]
fn test_parse_integer() {
    let expr = parse_source("42;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Int(42) = inner.as_ref() {
                } else {
                    panic!("Expected Int(42)");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_float() {
    let expr = parse_source("3.14;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Float(f) = inner.as_ref() {
                    assert!((f - 3.14).abs() < 0.0001);
                } else {
                    panic!("Expected Float");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_string() {
    let expr = parse_source(r#""hello";"#);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Str(s) = inner.as_ref() {
                    assert_eq!(s, "hello");
                } else {
                    panic!("Expected Str");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_bool_true() {
    let expr = parse_source("`t;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Bool(true) = inner.as_ref() {
                } else {
                    panic!("Expected Bool(true)");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_bool_false() {
    let expr = parse_source("`f;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Bool(false) = inner.as_ref() {
                } else {
                    panic!("Expected Bool(false)");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_char() {
    let expr = parse_source("'a';");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Char(c) = inner.as_ref() {
                    assert_eq!(c, "a");
                } else {
                    panic!("Expected Char");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Parenthesized Expressions ==========

#[test]
fn test_parse_parenthesized_expr() {
    let expr = parse_source("(1 + 2);");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::Add(_, _, _) = inner.as_ref() {
                } else {
                    panic!("Expected Add (parentheses should be stripped)");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Operator Precedence ==========

#[test]
fn test_operator_precedence() {
    // 1 + 2 * 3 should be parsed as 1 + (2 * 3)
    let expr = parse_source("1 + 2 * 3;");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => {
            match statements[0].as_ref() {
                crate::expr::Expr::Stmt(inner) => {
                    if let crate::expr::Expr::Add(_, right, _) = inner.as_ref() {
                        // Right should be Mult
                        if let crate::expr::Expr::Mult(_, _, _) = right.as_ref() {
                        } else {
                            panic!("Expected Mult as right operand");
                        }
                    } else {
                        panic!("Expected Add at top level");
                    }
                }
                _ => panic!("Expected Stmt"),
            }
        }
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Block Expressions ==========

#[test]
fn test_parse_block_expression() {
    let expr = parse_source("{ 1; 2; 3; }");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Stmt(inner) => {
                if let crate::expr::Expr::StmtBlockWithScope(inner_stmts, _) = inner.as_ref() {
                    assert_eq!(inner_stmts.len(), 3);
                } else {
                    panic!("Expected StmtBlockWithScope");
                }
            }
            _ => panic!("Expected Stmt"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Use Statement ==========

#[test]
fn test_parse_use_statement() {
    let expr = parse_source(r#"use "mymodule";"#);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Use { kind, path, .. } => {
                assert_eq!(path, "mymodule");
                assert!(matches!(kind, crate::expr::UseKind::Normal));
            }
            _ => panic!("Expected Use"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

#[test]
fn test_parse_use_std() {
    let expr = parse_source(r#"use std "math.obs";"#);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => match statements[0].as_ref() {
            crate::expr::Expr::Use { kind, path, .. } => {
                assert_eq!(path, "math.obs");
                assert!(matches!(kind, crate::expr::UseKind::Std));
            }
            _ => panic!("Expected Use"),
        },
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Complete Program ==========

#[test]
fn test_parse_complete_function() {
    let source = r#"
fn main(args: vec<<str>>) {
    #x = 42;
    $x;
    ret;
}
"#;
    let expr = parse_source(source);
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => {
            assert!(statements.len() >= 1);
        }
        _ => panic!("Expected StmtBlock"),
    }
}

// ========== Expression If Statement ==========

#[test]
fn test_parse_expression_if() {
    let expr = parse_source("ret ? x > 0 { 1 } ~ { 2 };");
    match expr {
        crate::expr::Expr::StmtBlock(statements, _) => {
            match statements[0].as_ref() {
                crate::expr::Expr::Return(inner, _) => {
                    // The return value should be an If expression
                    if let crate::expr::Expr::If(_, _, _, true) = inner.as_ref() {
                    } else {
                        panic!("Expected If expression");
                    }
                }
                _ => panic!("Expected Return"),
            }
        }
        _ => panic!("Expected StmtBlock"),
    }
}
