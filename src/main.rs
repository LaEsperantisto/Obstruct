mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;

use crate::parser::Parser;
use crate::scanner::Scanner;
use std::sync::atomic::AtomicBool;
use std::{env, fs};

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

    println!("{}", expr.value());

    Ok(())
}

pub fn error(line: isize, message: &str) {
    report(line, "".into(), message);
}

pub fn report(line: isize, place: String, message: &str) {
    println!("[line {} ] Error{}: {}", line, place, message);
    // println!("\n{} | {}", line.into(), get_line(line));
    HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
}
