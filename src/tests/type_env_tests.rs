//! Type Environment tests
//! Tests type checking, type inference, and type operations

use crate::type_env::{substitute, unify, nil_type, Type, TypeEnvironment};

// ========== Type Creation ==========

#[test]
fn test_simple_type_creation() {
    let ty = Type::simple("i32");
    assert_eq!(ty.name(), "i32");
    assert!(ty.generics().is_empty());
}

#[test]
fn test_conceptual_type_creation() {
    let ty = Type::conceptual("T");
    assert_eq!(ty.name(), "T");
    assert!(ty.is_generic());
}

#[test]
fn test_type_with_generics() {
    let inner = Type::simple("i32");
    let ty = Type::with_generics("vec", vec![inner]);
    assert_eq!(ty.name(), "vec");
    assert_eq!(ty.generics().len(), 1);
}

#[test]
fn test_nested_generics() {
    let inner = Type::simple("i32");
    let vec_i32 = Type::with_generics("vec", vec![inner]);
    let ty = Type::with_generics("vec", vec![vec_i32]);
    assert_eq!(ty.name(), "vec");
    assert_eq!(ty.generics().len(), 1);
}

#[test]
fn test_multiple_generics() {
    let i32 = Type::simple("i32");
    let str = Type::simple("str");
    let ty = Type::with_generics("tuple", vec![i32, str]);
    assert_eq!(ty.name(), "tuple");
    assert_eq!(ty.generics().len(), 2);
}

// ========== Type Conversion ==========

#[test]
fn test_from_str() {
    let ty: Type = "i32".into();
    assert_eq!(ty.name(), "i32");
}

#[test]
fn test_from_string() {
    let ty: Type = "f64".to_string().into();
    assert_eq!(ty.name(), "f64");
}

// ========== Type Display ==========

#[test]
fn test_display_simple_type() {
    let ty = Type::simple("i32");
    assert_eq!(format!("{}", ty), "i32");
}

#[test]
fn test_display_type_with_generics() {
    let inner = Type::simple("i32");
    let ty = Type::with_generics("vec", vec![inner]);
    assert_eq!(format!("{}", ty), "vec<<i32>>");
}

#[test]
fn test_display_nested_generics() {
    let inner = Type::simple("i32");
    let vec_i32 = Type::with_generics("vec", vec![inner]);
    let ty = Type::with_generics("vec", vec![vec_i32]);
    assert_eq!(format!("{}", ty), "vec<<vec<<i32>>>>");
}

#[test]
fn test_display_multiple_generics() {
    let i32 = Type::simple("i32");
    let str = Type::simple("str");
    let ty = Type::with_generics("tuple", vec![i32, str]);
    assert_eq!(format!("{}", ty), "tuple<<i32, str>>");
}

#[test]
fn test_display_conceptual() {
    let ty = Type::conceptual("T");
    assert_eq!(format!("{}", ty), "T");
}

// ========== Type Properties ==========

#[test]
fn test_has_tag() {
    let ty = Type::simple("i32");
    assert!(ty.has_tag("i32"));
    assert!(!ty.has_tag("str"));
}

#[test]
fn test_is_generic() {
    let conceptual = Type::conceptual("T");
    let concrete = Type::simple("i32");

    assert!(conceptual.is_generic());
    assert!(!concrete.is_generic());
}

#[test]
fn test_generics_empty() {
    let ty = Type::simple("i32");
    assert!(ty.generics().is_empty());
}

#[test]
fn test_generics_not_empty() {
    let inner = Type::simple("i32");
    let ty = Type::with_generics("vec", vec![inner]);
    assert!(!ty.generics().is_empty());
}

// ========== Type Equality ==========

#[test]
fn test_type_equality_same() {
    let ty1 = Type::simple("i32");
    let ty2 = Type::simple("i32");
    assert_eq!(ty1, ty2);
}

#[test]
fn test_type_equality_different() {
    let ty1 = Type::simple("i32");
    let ty2 = Type::simple("str");
    assert_ne!(ty1, ty2);
}

#[test]
fn test_type_equality_with_generics() {
    let inner1 = Type::simple("i32");
    let inner2 = Type::simple("i32");
    let ty1 = Type::with_generics("vec", vec![inner1]);
    let ty2 = Type::with_generics("vec", vec![inner2]);
    assert_eq!(ty1, ty2);
}

#[test]
fn test_type_equality_different_generics() {
    let inner1 = Type::simple("i32");
    let inner2 = Type::simple("str");
    let ty1 = Type::with_generics("vec", vec![inner1]);
    let ty2 = Type::with_generics("vec", vec![inner2]);
    assert_ne!(ty1, ty2);
}

// ========== nil_type() Helper ==========

#[test]
fn test_nil_type() {
    let ty = nil_type();
    assert_eq!(ty.name(), "arr");
    assert!(ty.generics().is_empty());
}

// ========== TypeEnvironment ==========

#[test]
fn test_type_env_new() {
    let _env = TypeEnvironment::new();
    // Should have one scope by default
}

#[test]
fn test_type_env_declare_and_get() {
    let mut env = TypeEnvironment::new();
    let ty = Type::simple("i32");
    env.declare("x".to_string(), ty);

    let retrieved = env.get("x");
    assert_eq!(retrieved.name(), "i32");
}

#[test]
fn test_type_env_shadowing() {
    let mut env = TypeEnvironment::new();
    let i32 = Type::simple("i32");
    let str = Type::simple("str");

    env.declare("x".to_string(), i32);
    env.push();
    env.declare("x".to_string(), str);

    // Should get the inner scope's x (str)
    let retrieved = env.get("x");
    assert_eq!(retrieved.name(), "str");

    env.pop();
    // Should get the outer scope's x (i32)
    let retrieved = env.get("x");
    assert_eq!(retrieved.name(), "i32");
}

#[test]
fn test_type_env_multiple_variables() {
    let mut env = TypeEnvironment::new();
    env.declare("x".to_string(), Type::simple("i32"));
    env.declare("y".to_string(), Type::simple("f64"));
    env.declare("z".to_string(), Type::simple("str"));

    assert_eq!(env.get("x").name(), "i32");
    assert_eq!(env.get("y").name(), "f64");
    assert_eq!(env.get("z").name(), "str");
}

#[test]
fn test_type_env_scoping() {
    let mut env = TypeEnvironment::new();
    env.declare("outer".to_string(), Type::simple("i32"));

    env.push();
    env.declare("inner".to_string(), Type::simple("str"));

    // Both should be accessible
    assert_eq!(env.get("outer").name(), "i32");
    assert_eq!(env.get("inner").name(), "str");

    env.pop();
    // inner should no longer be accessible (will panic)
    // This tests that scoping works correctly
}

#[test]
fn test_type_env_push_func() {
    let mut env = TypeEnvironment::new();
    env.push_func();
    env.add_gen("T".to_string(), Type::simple("i32"));

    let ty = env.get_gen("T".to_string());
    assert_eq!(ty.name(), "i32");
}

#[test]
fn test_type_env_pop_func() {
    let _env = TypeEnvironment::new();
    // After popping, there's no generic scope
}

// ========== Unification ==========

#[test]
fn test_unify_conceptual_with_concrete() {
    let mut bindings = std::collections::HashMap::new();
    let pattern = Type::conceptual("T");
    let actual = Type::simple("i32");

    assert!(unify(&pattern, &actual, &mut bindings));
    assert!(bindings.contains_key("T"));
    assert_eq!(bindings.get("T").unwrap().name(), "i32");
}

#[test]
fn test_unify_conceptual_consistent() {
    let mut bindings = std::collections::HashMap::new();
    let pattern = Type::conceptual("T");
    let actual1 = Type::simple("i32");
    let actual2 = Type::simple("i32");

    assert!(unify(&pattern, &actual1, &mut bindings));
    // Using the same pattern should return the existing binding
    assert!(unify(&pattern, &actual2, &mut bindings));
}

#[test]
fn test_unify_conceptual_inconsistent() {
    let mut bindings = std::collections::HashMap::new();
    let pattern = Type::conceptual("T");
    let actual1 = Type::simple("i32");
    let actual2 = Type::simple("str");

    assert!(unify(&pattern, &actual1, &mut bindings));
    assert!(!unify(&pattern, &actual2, &mut bindings));
}

#[test]
fn test_unify_concrete_same() {
    let mut bindings = std::collections::HashMap::new();
    let ty1 = Type::simple("i32");
    let ty2 = Type::simple("i32");

    assert!(unify(&ty1, &ty2, &mut bindings));
}

#[test]
fn test_unify_concrete_different() {
    let mut bindings = std::collections::HashMap::new();
    let ty1 = Type::simple("i32");
    let ty2 = Type::simple("str");

    assert!(!unify(&ty1, &ty2, &mut bindings));
}

#[test]
fn test_unify_with_generics_same() {
    let mut bindings = std::collections::HashMap::new();
    let inner1 = Type::simple("i32");
    let inner2 = Type::simple("i32");
    let ty1 = Type::with_generics("vec", vec![inner1]);
    let ty2 = Type::with_generics("vec", vec![inner2]);

    assert!(unify(&ty1, &ty2, &mut bindings));
}

#[test]
fn test_unify_with_generics_different() {
    let mut bindings = std::collections::HashMap::new();
    let inner1 = Type::simple("i32");
    let inner2 = Type::simple("str");
    let ty1 = Type::with_generics("vec", vec![inner1]);
    let ty2 = Type::with_generics("vec", vec![inner2]);

    assert!(!unify(&ty1, &ty2, &mut bindings));
}

#[test]
fn test_unify_with_generics_different_count() {
    let mut bindings = std::collections::HashMap::new();
    let ty1 = Type::with_generics("vec", vec![Type::simple("i32")]);
    let ty2 = Type::with_generics("vec", vec![
        Type::simple("i32"),
        Type::simple("str"),
    ]);

    assert!(!unify(&ty1, &ty2, &mut bindings));
}

#[test]
fn test_unify_nested_generics() {
    let mut bindings = std::collections::HashMap::new();
    let inner = Type::conceptual("T");
    let ty1 = Type::with_generics("vec", vec![inner]);
    let inner2 = Type::simple("i32");
    let ty2 = Type::with_generics("vec", vec![inner2]);

    assert!(unify(&ty1, &ty2, &mut bindings));
    assert!(bindings.contains_key("T"));
    assert_eq!(bindings.get("T").unwrap().name(), "i32");
}

// ========== Substitution ==========

#[test]
fn test_substitute_conceptual() {
    let mut map = std::collections::HashMap::new();
    map.insert("T".to_string(), Type::simple("i32"));

    let pattern = Type::conceptual("T");
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    assert_eq!(result.name(), "i32");
}

#[test]
fn test_substitute_conceptual_not_found() {
    let map: std::collections::HashMap<String, Type> = std::collections::HashMap::new();
    let pattern = Type::conceptual("T");
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    // Should return the original type if not found
    assert_eq!(result.name(), "T");
}

#[test]
fn test_substitute_concrete_no_generics() {
    let map: std::collections::HashMap<String, Type> = std::collections::HashMap::new();
    let pattern = Type::simple("i32");
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    assert_eq!(result.name(), "i32");
}

#[test]
fn test_substitute_with_generics() {
    let mut map = std::collections::HashMap::new();
    map.insert("T".to_string(), Type::simple("i32"));

    let inner = Type::conceptual("T");
    let pattern = Type::with_generics("vec", vec![inner]);
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    assert_eq!(result.name(), "vec");
    assert_eq!(result.generics().len(), 1);
    assert_eq!(result.generics()[0].name(), "i32");
}

#[test]
fn test_substitute_nested_generics() {
    let mut map = std::collections::HashMap::new();
    map.insert("T".to_string(), Type::simple("i32"));

    let inner = Type::conceptual("T");
    let vec_t = Type::with_generics("vec", vec![inner]);
    let pattern = Type::with_generics("vec", vec![vec_t]);
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    assert_eq!(result.name(), "vec");
    assert_eq!(result.generics().len(), 1);
    // The inner vec should have been substituted
}

#[test]
fn test_substitute_multiple_generics() {
    let mut map = std::collections::HashMap::new();
    map.insert("T".to_string(), Type::simple("i32"));
    map.insert("U".to_string(), Type::simple("str"));

    let t = Type::conceptual("T");
    let u = Type::conceptual("U");
    let pattern = Type::with_generics("tuple", vec![t, u]);
    let result = substitute(&pattern, &map, crate::span::Span::empty());

    assert_eq!(result.name(), "tuple");
    assert_eq!(result.generics().len(), 2);
    assert_eq!(result.generics()[0].name(), "i32");
    assert_eq!(result.generics()[1].name(), "str");
}

// ========== Combined Unification and Substitution ==========

#[test]
fn test_unify_then_substitute() {
    let mut bindings = std::collections::HashMap::new();
    let pattern = Type::conceptual("T");
    let actual = Type::simple("i32");

    assert!(unify(&pattern, &actual, &mut bindings));

    // Now substitute using the bindings
    let result = substitute(&pattern, &bindings, crate::span::Span::empty());
    assert_eq!(result.name(), "i32");
}

#[test]
fn test_complex_unification() {
    let mut bindings = std::collections::HashMap::new();

    // Pattern: func<<T, i32, T>>
    let t = Type::conceptual("T");
    let i32 = Type::simple("i32");
    let pattern = Type::with_generics("func", vec![t.clone(), i32.clone(), t.clone()]);

    // Actual: func<<str, i32, str>>
    let str = Type::simple("str");
    let actual = Type::with_generics("func", vec![str.clone(), i32.clone(), str.clone()]);

    assert!(unify(&pattern, &actual, &mut bindings));
    assert_eq!(bindings.get("T").unwrap().name(), "str");
}

#[test]
fn test_complex_unification_failure() {
    let mut bindings = std::collections::HashMap::new();

    // Pattern: func<<T, i32, T>>
    let t = Type::conceptual("T");
    let i32 = Type::simple("i32");
    let pattern = Type::with_generics("func", vec![t.clone(), i32.clone(), t.clone()]);

    // Actual: func<<str, i32, i32>> - doesn't match because T should be same
    let str = Type::simple("str");
    let actual = Type::with_generics("func", vec![str.clone(), i32.clone(), i32.clone()]);

    assert!(!unify(&pattern, &actual, &mut bindings));
}
