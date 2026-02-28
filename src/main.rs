#![allow(dead_code)]
extern crate core;

mod env;
mod error;
mod expr;
mod init;
mod parser;
mod scanner;
mod span;
mod token;
mod token_type;
mod type_env;
mod value;
mod variable;
// TODO Add classes
// TODO Implement references

use crate::env::Environment;
use crate::error::ObstructError;
use crate::expr::Expr;
use crate::init::init;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::type_env::TypeEnvironment;
use std::fs;
use std::panic;
use std::sync::Mutex;
static SOURCES: Mutex<Vec<String>> = Mutex::new(vec![]);
static ERROR: Mutex<Result<(), ObstructError>> = Mutex::new(Ok(()));
static CALL_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());

// Basic Colors
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

// Bright Colors
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_YELLOW: &str = "\x1b[93m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BRIGHT_MAGENTA: &str = "\x1b[95m";
const BRIGHT_CYAN: &str = "\x1b[96m";

// Background Colors
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";

// Extra Ansi
const ERROR_COLOR: &str = BRIGHT_RED;
const WARNING_COLOR: &str = BRIGHT_YELLOW;

// Text Styles
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const UNDERLINE: &str = "\x1b[4m";
const BLINK: &str = "\x1b[5m";
const REVERSED: &str = "\x1b[7m";
const STRIKETHROUGH: &str = "\x1b[9m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<(), ObstructError> {
    let result = panic::catch_unwind(|| run());

    result.unwrap_or_else(|_| {
        let err = ERROR.lock().unwrap().clone();
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    })
}

fn run() -> Result<(), ObstructError> {
    let mut args = std::env::args().skip(1);

    let arg_len = args.len();

    let debug = if arg_len == 2 {
        let arg = args.next();
        if arg == Some("--release".into()) {
            false
        } else {
            true
        }
    } else {
        true
    };

    let arg1 = args.next();

    let filepath = match arg1 {
        Some(filename) => filename,
        _ => "/home/aster/dev/obstruct/main.obs".to_string(),
    };

    let mut env = Environment::new();
    let mut tenv = TypeEnvironment::new();
    Expr::DeclareAndAssign("DEBUG".into(), Box::new(Expr::Bool(debug)), false)
        .value(&mut env, &mut tenv);

    init(&mut env, &mut tenv);

    let source = fs::read_to_string(filepath).unwrap() + "\n\nmain();";

    {
        SOURCES.lock().unwrap().push(source.clone());

        let expr = compile(source);
        expr.value(&mut env, &mut tenv);

        SOURCES.lock().unwrap().pop();
    }

    let err = ERROR.lock().unwrap().clone();

    match err {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn error(line: usize, column: usize, message: &str) {
    report(line, column, message);

    panic::set_hook(Box::new(|_| {}));
}

fn get_line(line: usize) -> String {
    let src = SOURCES.lock().unwrap();
    if !src.is_empty() {
        let source = src.last().unwrap();
        source
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

pub fn report(line: usize, column: usize, message: &str) {
    let mut err = ERROR.lock().unwrap();

    println!("\n{BOLD}{ERROR_COLOR}error{RESET}{BOLD}: {message}{RESET}");

    println!("--> line {} column {}\n", line, column);

    let source_line = get_line(line);

    println!("    |");
    if line as isize - 1 > 0 {
        let prev_line = get_line(line - 1);
        println!("{CYAN}{:>3}{RESET} | {}", line - 1, prev_line);
    }
    println!("{CYAN}{:>3}{RESET} | {}", line, source_line);

    let prefix_len = format!("{:>3}  | ", line).len();
    let caret_padding = " ".repeat(prefix_len + column.saturating_sub(3));

    let mut caret_line = format!("{}{ERROR_COLOR}^{RESET} {message}", caret_padding);

    caret_line.replace_range(4..4, "|");

    println!("{}", caret_line);

    let stack = CALL_STACK.lock().unwrap();
    if !stack.is_empty() {
        println!("\n{BOLD}Stack trace:{RESET}");
        for func in stack.iter().rev() {
            println!("  {BRIGHT_YELLOW}->{BRIGHT_BLUE} {}", func);
        }
    }

    println!("{RESET}\n");

    *err = Err(ObstructError::new(line, column, message));
}

pub fn compile(source: String) -> Expr {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    expr
}

pub fn push_stack(name: &str) {
    CALL_STACK.lock().unwrap().push(name.to_string());
}

pub fn pop_stack() {
    CALL_STACK.lock().unwrap().pop();
}
