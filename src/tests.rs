//! Comprehensive test suite for Obstruct language
//!
//! This module contains tests for all major components:
//! - Scanner/Lexer: Tokenization of source code
//! - Parser: AST construction from tokens
//! - Type System: Type checking and inference
//! - Transpiler: C code generation

pub mod scanner_tests;
pub mod parser_tests;
pub mod type_env_tests;
pub mod transpiler_tests;
