#![allow(dead_code)]

mod environment;
mod expr;
mod init;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;
mod variable;

use crate::environment::Environment;
use crate::expr::Expr;
use crate::init::init;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, fs, io};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

// static mut SOURCE: *mut String = std::ptr::null_mut();

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);

    let arg1 = args.next();

    let filepath = match arg1 {
        Some(filename) => filename,
        _ => "/home/aster/main.obs".to_string(),
    };

    let mut env = Environment::new();
    init(&mut env);

    let source = fs::read_to_string(filepath)? + "\n\nmain();";

    let expr = compile(source);

    if !had_error() {
        expr.value(&mut env);
        println!();
        if had_error() {
            Err(io::Error::new(ErrorKind::Other, "Error during execution"))
        } else {
            Ok(())
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

// fn get_source() -> String {
//     unsafe { (*SOURCE).clone() }
// }

pub fn compile(source: String) -> Expr {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    expr
}
