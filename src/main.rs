#![allow(dead_code)]
extern crate core;

mod env;
mod expr;
mod init;
mod parser;
mod scanner;
mod token;
mod token_type;
mod type_env;
mod value;
mod variable;

use crate::env::Environment;
use crate::expr::Expr;
use crate::init::init;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::type_env::TypeEnvironment;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, io};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

// static mut SOURCE: *mut String = std::ptr::null_mut();

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    let arg1 = args.next();

    let filepath = match arg1 {
        Some(filename) => filename,
        _ => "/home/aster/dev/obstruct/main.obs".to_string(),
    };

    let mut env = Environment::new();
    let mut tenv = TypeEnvironment::new();
    init(&mut env);

    let source = fs::read_to_string(filepath)? + "\n\nmain();";

    let expr = compile(source);

    if !had_error() {
        // expr.type_of(&mut tenv);

        if had_error() {
            Err(io::Error::new(
                ErrorKind::Other,
                "Error during type verification",
            ))
        } else {
            expr.value(&mut env, &mut tenv);
            println!();
            if had_error() {
                Err(io::Error::new(ErrorKind::Other, "Error during execution"))
            } else {
                Ok(())
            }
        }
    } else {
        Err(io::Error::new(ErrorKind::Other, "Error during parsing"))
    }
}

pub fn error(line: usize, column: usize, message: &str) {
    report(line, column, "".into(), message);
}

fn get_line(_line: usize, _column: usize) -> String {
    String::new()
}

pub fn report(line: usize, column: usize, place: String, message: &str) {
    println!("\n\n[line {line} column {column}] Error{place}: {message}");
    // println!("\n{} | {}", line, get_line(line, column));
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn compile(source: String) -> Expr {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    expr
}
