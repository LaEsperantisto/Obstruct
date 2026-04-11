//! Scanner/Lexer tests
//! Tests tokenization of Obstruct source code

use crate::scanner::Scanner;
use crate::token_type::TokenType;

/// Helper to get the first token's type
fn tokenize_first(source: &str) -> TokenType {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    tokens.first().map(|t| t.token_type).unwrap_or(TokenType::EOF)
}

/// Helper to get all token types from source
fn tokenize_all(source: &str) -> Vec<TokenType> {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    tokens.iter().map(|t| t.token_type).collect()
}

/// Helper to get token literal value
fn tokenize_literal(source: &str) -> Option<String> {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    tokens.first().map(|t| t.literal.clone())
}

// ========== Keywords ==========

#[test]
fn test_keyword_fn() {
    assert_eq!(tokenize_first("fn"), TokenType::Fn);
}

#[test]
fn test_keyword_ret() {
    assert_eq!(tokenize_first("ret"), TokenType::Ret);
}

#[test]
fn test_keyword_for() {
    assert_eq!(tokenize_first("for"), TokenType::For);
}

#[test]
fn test_keyword_lam() {
    assert_eq!(tokenize_first("lam"), TokenType::Lam);
}

#[test]
fn test_keyword_del() {
    assert_eq!(tokenize_first("del"), TokenType::Del);
}

#[test]
fn test_keyword_use() {
    assert_eq!(tokenize_first("use"), TokenType::Use);
}

#[test]
fn test_keyword_std() {
    assert_eq!(tokenize_first("std"), TokenType::Std);
}

#[test]
fn test_keyword_cls() {
    assert_eq!(tokenize_first("cls"), TokenType::Cls);
}

#[test]
fn test_keyword_mac() {
    assert_eq!(tokenize_first("mac"), TokenType::Mac);
}

// ========== Operators ==========

#[test]
fn test_pound_operator() {
    // While loop operator
    assert_eq!(tokenize_first("£"), TokenType::Pound);
}

#[test]
fn test_question_mark_operator() {
    // If condition operator
    assert_eq!(tokenize_first("?"), TokenType::QuestionMark);
}

#[test]
fn test_tilde_question_mark() {
    // Else if operator
    assert_eq!(tokenize_first("~?"), TokenType::TildeQuestionMark);
}

#[test]
fn test_tilde_operator() {
    // Else operator
    assert_eq!(tokenize_first("~"), TokenType::Tilde);
}

#[test]
fn test_hash_operator() {
    // Variable declaration
    assert_eq!(tokenize_first("#"), TokenType::Hash);
}

#[test]
fn test_at_operator() {
    // Mutable marker
    assert_eq!(tokenize_first("@"), TokenType::At);
}

#[test]
fn test_dollar_operator() {
    // Print operator
    assert_eq!(tokenize_first("$"), TokenType::Dollar);
}

#[test]
fn test_dollar_dollar() {
    // Println - should parse as two dollar tokens
    let tokens = tokenize_all("$$");
    assert_eq!(tokens[0], TokenType::Dollar);
    assert_eq!(tokens[1], TokenType::Dollar);
}

#[test]
fn test_double_colon() {
    assert_eq!(tokenize_first("::"), TokenType::DoubleColon);
}

#[test]
fn test_double_arrow_generics() {
    assert_eq!(tokenize_first("<<"), TokenType::LessLess);
    assert_eq!(tokenize_first(">>"), TokenType::GreaterGreater);
}

#[test]
fn test_star_star() {
    // Power operator
    assert_eq!(tokenize_first("**"), TokenType::StarStar);
}

#[test]
fn test_minus_right_arrow() {
    assert_eq!(tokenize_first("->"), TokenType::MinusRight);
}

#[test]
fn test_bang_equal() {
    assert_eq!(tokenize_first("!="), TokenType::BangEqual);
}

#[test]
fn test_equal_equal() {
    assert_eq!(tokenize_first("=="), TokenType::EqualEqual);
}

#[test]
fn test_less_equal() {
    assert_eq!(tokenize_first("<="), TokenType::LessEqual);
}

#[test]
fn test_greater_equal() {
    assert_eq!(tokenize_first(">="), TokenType::GreaterEqual);
}

// ========== Arithmetic Operators ==========

#[test]
fn test_arithmetic_operators() {
    assert_eq!(tokenize_first("+"), TokenType::Plus);
    assert_eq!(tokenize_first("-"), TokenType::Minus);
    assert_eq!(tokenize_first("*"), TokenType::Star);
    assert_eq!(tokenize_first("/"), TokenType::Slash);
    assert_eq!(tokenize_first("%"), TokenType::Mod);
}

// ========== Literals ==========

#[test]
fn test_integer_literal() {
    assert_eq!(tokenize_first("42"), TokenType::Int);
    assert_eq!(tokenize_literal("42"), Some("42".to_string()));
}

#[test]
fn test_float_literal() {
    assert_eq!(tokenize_first("3.14"), TokenType::Float);
    assert_eq!(tokenize_literal("3.14"), Some("3.14".to_string()));
}

#[test]
fn test_string_literal() {
    assert_eq!(tokenize_first(r#""hello""#), TokenType::String);
    assert_eq!(tokenize_literal(r#""hello""#), Some("hello".to_string()));
}

#[test]
fn test_string_with_escape() {
    assert_eq!(tokenize_first("\"hello\\nworld\""), TokenType::String);
    assert_eq!(
        tokenize_literal("\"hello\\nworld\""),
        Some("hello\nworld".to_string())
    );
}

#[test]
fn test_char_literal() {
    assert_eq!(tokenize_first("'a'"), TokenType::Char);
    assert_eq!(tokenize_literal("'a'"), Some("a".to_string()));
}

#[test]
fn test_char_with_escape() {
    // Test escaped newline in character literal
    assert_eq!(tokenize_first("'\\n'"), TokenType::Char);
    assert_eq!(tokenize_literal("'\\n'"), Some("\n".to_string()));
}

#[test]
fn test_true_literal() {
    assert_eq!(tokenize_first("`t"), TokenType::True);
}

#[test]
fn test_false_literal() {
    assert_eq!(tokenize_first("`f"), TokenType::False);
}

#[test]
fn test_this_literal() {
    assert_eq!(tokenize_first("`v"), TokenType::This);
}

#[test]
fn test_empty_string_literal() {
    assert_eq!(tokenize_first("`s"), TokenType::String);
}

// ========== Identifiers ==========

#[test]
fn test_identifier() {
    let tokens = tokenize_all("my_var");
    assert_eq!(tokens[0], TokenType::Ident);
}

#[test]
fn test_identifier_with_underscore() {
    let tokens = tokenize_all("_private");
    assert_eq!(tokens[0], TokenType::Ident);
}

// ========== Delimiters ==========

#[test]
fn test_delimiters() {
    assert_eq!(tokenize_first("("), TokenType::LeftParen);
    assert_eq!(tokenize_first(")"), TokenType::RightParen);
    assert_eq!(tokenize_first("{"), TokenType::LeftBrace);
    assert_eq!(tokenize_first("}"), TokenType::RightBrace);
    assert_eq!(tokenize_first("["), TokenType::LeftBrack);
    assert_eq!(tokenize_first("]"), TokenType::RightBrack);
    assert_eq!(tokenize_first(","), TokenType::Comma);
    assert_eq!(tokenize_first(";"), TokenType::Semicolon);
    assert_eq!(tokenize_first(":"), TokenType::Colon);
    assert_eq!(tokenize_first("."), TokenType::Dot);
    assert_eq!(tokenize_first("&"), TokenType::And);
    assert_eq!(tokenize_first("|"), TokenType::Or);
}

#[test]
fn test_backslash() {
    // Vector literal marker
    assert_eq!(tokenize_first("\\"), TokenType::BackSlash);
}

// ========== Comments ==========

#[test]
fn test_single_line_comment() {
    // Comment should be skipped, next token is EOF
    let tokens = tokenize_all("// this is a comment");
    assert_eq!(tokens[0], TokenType::EOF);
}

#[test]
fn test_multi_line_comment() {
    // Block comment should be skipped
    let tokens = tokenize_all("/* this is a block comment */");
    assert_eq!(tokens[0], TokenType::EOF);
}

#[test]
fn test_code_after_comment() {
    // Should skip comment and find the identifier
    let tokens = tokenize_all("// comment\nmy_var");
    assert_eq!(tokens[0], TokenType::Ident);
}

// ========== Complex Expressions ==========

#[test]
fn test_variable_declaration_syntax() {
    // #my_var = 42;
    let tokens = tokenize_all("#my_var = 42;");
    assert_eq!(tokens[0], TokenType::Hash);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::Equal);
    assert_eq!(tokens[3], TokenType::Int);
    assert_eq!(tokens[4], TokenType::Semicolon);
}

#[test]
fn test_mutable_declaration_syntax() {
    // #@my_var = 42;
    let tokens = tokenize_all("#@my_var = 42;");
    assert_eq!(tokens[0], TokenType::Hash);
    assert_eq!(tokens[1], TokenType::At);
    assert_eq!(tokens[2], TokenType::Ident);
    assert_eq!(tokens[3], TokenType::Equal);
    assert_eq!(tokens[4], TokenType::Int);
}

#[test]
fn test_if_condition_syntax() {
    // ? x > 5 {
    let tokens = tokenize_all("? x > 5 {");
    assert_eq!(tokens[0], TokenType::QuestionMark);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::Greater);
    assert_eq!(tokens[3], TokenType::Int);
    assert_eq!(tokens[4], TokenType::LeftBrace);
}

#[test]
fn test_else_if_syntax() {
    // ~? x == 5 {
    let tokens = tokenize_all("~? x == 5 {");
    assert_eq!(tokens[0], TokenType::TildeQuestionMark);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::EqualEqual);
    assert_eq!(tokens[3], TokenType::Int);
}

#[test]
fn test_while_loop_syntax() {
    // £ x < 10 {
    let tokens = tokenize_all("£ x < 10 {");
    assert_eq!(tokens[0], TokenType::Pound);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::Less);
    assert_eq!(tokens[3], TokenType::Int);
}

#[test]
fn test_function_definition_syntax() {
    // fn add(a: i32, b: i32) i32 {
    let tokens = tokenize_all("fn add(a: i32, b: i32) i32 {");
    assert_eq!(tokens[0], TokenType::Fn);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::LeftParen);
    assert_eq!(tokens[3], TokenType::Ident);
    assert_eq!(tokens[4], TokenType::Colon);
    assert_eq!(tokens[5], TokenType::Ident);
}

#[test]
fn test_print_syntax() {
    // $"Hello";
    let tokens = tokenize_all("$\"Hello\";");
    assert_eq!(tokens[0], TokenType::Dollar);
    assert_eq!(tokens[1], TokenType::String);
    assert_eq!(tokens[2], TokenType::Semicolon);
}

#[test]
fn test_return_syntax() {
    // ret x;
    let tokens = tokenize_all("ret x;");
    assert_eq!(tokens[0], TokenType::Ret);
    assert_eq!(tokens[1], TokenType::Ident);
    assert_eq!(tokens[2], TokenType::Semicolon);
}

#[test]
fn test_lambda_syntax() {
    // lam {
    let tokens = tokenize_all("lam {");
    assert_eq!(tokens[0], TokenType::Lam);
    assert_eq!(tokens[1], TokenType::LeftBrace);
}

#[test]
fn test_generic_syntax() {
    // fn <<T>> push(v: vec<<T>>)
    let tokens = tokenize_all("fn <<T>> push");
    assert_eq!(tokens[0], TokenType::Fn);
    assert_eq!(tokens[1], TokenType::LessLess);
    assert_eq!(tokens[2], TokenType::Ident);
    assert_eq!(tokens[3], TokenType::GreaterGreater);
    assert_eq!(tokens[4], TokenType::Ident);
}

#[test]
fn test_vector_literal_syntax() {
    // \{1, 2, 3}
    let tokens = tokenize_all("\\{1, 2, 3}");
    assert_eq!(tokens[0], TokenType::BackSlash);
    assert_eq!(tokens[1], TokenType::LeftBrace);
    assert_eq!(tokens[2], TokenType::Int);
}

#[test]
fn test_array_literal_syntax() {
    // [1, 2, 3]
    let tokens = tokenize_all("[1, 2, 3]");
    assert_eq!(tokens[0], TokenType::LeftBrack);
    assert_eq!(tokens[1], TokenType::Int);
    assert_eq!(tokens[2], TokenType::Comma);
    assert_eq!(tokens[3], TokenType::Int);
    assert_eq!(tokens[4], TokenType::Comma);
    assert_eq!(tokens[5], TokenType::Int);
    assert_eq!(tokens[6], TokenType::RightBrack);
}

// ========== Line and Column Tracking ==========

#[test]
fn test_line_tracking() {
    let mut scanner = Scanner::new("a\nb\nc".to_string());
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].lexeme, "a");
    assert_eq!(tokens[0].line, 1);

    assert_eq!(tokens[1].lexeme, "b");
    assert_eq!(tokens[1].line, 2);

    assert_eq!(tokens[2].lexeme, "c");
    assert_eq!(tokens[2].line, 3);
}

#[test]
fn test_column_tracking() {
    let mut scanner = Scanner::new("   x".to_string());
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].lexeme, "x");
    assert_eq!(tokens[0].column, 4);
}

// ========== Whitespace Handling ==========

#[test]
fn test_whitespace_ignored() {
    let tokens = tokenize_all("  \t\n  my_var  \t\n  ");
    assert_eq!(tokens[0], TokenType::Ident);
    assert_eq!(tokens[1], TokenType::EOF);
}

#[test]
fn test_mixed_operators_and_whitespace() {
    let tokens = tokenize_all("a + b - c");
    assert_eq!(tokens[0], TokenType::Ident);
    assert_eq!(tokens[1], TokenType::Plus);
    assert_eq!(tokens[2], TokenType::Ident);
    assert_eq!(tokens[3], TokenType::Minus);
    assert_eq!(tokens[4], TokenType::Ident);
}

// ========== Edge Cases ==========

#[test]
fn test_empty_source() {
    let tokens = tokenize_all("");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenType::EOF);
}

#[test]
fn test_only_comments() {
    let tokens = tokenize_all("// comment\n/* block */");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenType::EOF);
}

#[test]
fn test_nested_block_comment() {
    // Note: Obstruct doesn't support nested comments
    // The first */ closes the comment, leaving " outer */" after it
    // "outer" after first */ becomes an identifier
    let tokens = tokenize_all("/* outer /* inner */ outer */");
    // "outer" becomes Ident, then */ is just two tokens, then EOF
    assert!(tokens.contains(&TokenType::Ident));
    assert!(tokens.contains(&TokenType::EOF));
}

#[test]
fn test_not_sign_operator() {
    assert_eq!(tokenize_first("¬"), TokenType::NotSign);
}

#[test]
fn test_up_arrow() {
    assert_eq!(tokenize_first("^"), TokenType::UpArrow);
}

#[test]
fn test_double_up_arrow() {
    assert_eq!(tokenize_first("^^"), TokenType::DoubleUpArrow);
}

#[test]
fn test_dollar_question_mark() {
    assert_eq!(tokenize_first("$?"), TokenType::DollarQuestionMark);
}
