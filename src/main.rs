mod environment;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;
mod variable;

use crate::environment::Environment;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, fs, io};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);

    let arg1 = args.next();

    let filepath = match arg1 {
        Some(filename) => filename,
        _ => "/home/aster/main.obs".to_string(),
    };

    let source = fs::read_to_string(filepath)?;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    let mut env = Environment::new();
    if !HAD_ERROR.load(Ordering::Relaxed) {
        expr.value(&mut env);
        println!();
        if HAD_ERROR.load(Ordering::Relaxed) {
            Err(io::Error::new(ErrorKind::Other, "Error during execution"))
        } else {
            Ok(())
        }
    } else {
        Err(io::Error::new(ErrorKind::Other, "Error during parsing"))
    }
}

pub fn error(line: isize, message: &str) {
    report(line, "".into(), message);
}

pub fn report(line: isize, place: String, message: &str) {
    println!("[line {line}] Error{place}: {message}");
    // println!("\n{} | {}", line.into(), get_line(line));
    HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
}
