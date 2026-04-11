#![allow(dead_code)]
extern crate core;
const DEBUG: bool = true;

mod error;
mod expr;
mod init;
mod runtime_env;

mod transpiler {
    pub mod code_gen_context;
    pub mod compiletime_env;
    pub mod expr_to_c;
}
mod englich {
    pub mod englich_parser;
    pub mod englich_scanner;
    pub mod englich_token;
    pub mod englich_token_type;
}
mod parser;
mod scanner;
mod span;
mod token;
mod token_type;
mod type_env;
mod value;
mod variable;

#[cfg(test)]
mod tests;

// TODO Add generics
// TODO Add classes

use crate::englich::{englich_parser, englich_scanner};
use crate::error::ObstructError;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use colored::Colorize;
use image::GenericImageView;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::panic;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

// Paths
const STD_PATH: &str = "/home/aster/dev/obstruct/std/"; // end in a "/" !!!

// Global Variables
static SOURCES: Mutex<Vec<String>> = Mutex::new(vec![]);
static ERROR: Mutex<Result<(), ObstructError>> = Mutex::new(Ok(()));
static CALL_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PROGRAM_NAME: Mutex<String> = Mutex::new(String::new());

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

fn compile_c_file(path: &str, output: &str) {
    let status = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(output)
        .status()
        .expect("failed to compile C code");

    if !status.success() {
        panic!("C compilation failed");
    }
}

fn run_compiled_c_file(path: &str) {
    let status = Command::new(format!("./{}", path))
        .status()
        .expect("Failed to run compiled program");

    if !status.success() {
        panic!("Failed to run program");
    }
}

fn main() -> Result<(), ObstructError> {
    let result = run();

    if result.is_err() {
        std::process::exit(1);
    }

    let program_name = PROGRAM_NAME.lock().unwrap().clone();

    compile_c_file(&(program_name.clone() + ".c"), &program_name);
    run_compiled_c_file(&program_name);
    println!();

    result
}

fn run() -> Result<(), ObstructError> {
    let img = image::open("gfx/icon.png").unwrap();

    let size = if DEBUG {
        50
    } else {
        terminal_size::terminal_size().unwrap().0.0 / 2
    };

    let img = img.thumbnail(size as u32, size as u32);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            print!("{}", "  ".on_truecolor(r, g, b));
        }
        println!();
    }

    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut filepath = if DEBUG {
        "/home/aster/dev/obstruct/main.obs".to_string()
    } else {
        "".to_string()
    };
    let mut debug = true;
    let mut englich = false;

    for arg in &args {
        if arg == "--release" {
            debug = false;
        } else if arg == "--englich" {
            englich = true;
        } else {
            filepath = arg.clone();
        }
    }

    let program_name = Path::new(&filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(&filepath)
        .to_string();

    *PROGRAM_NAME.lock().unwrap() = program_name;

    let file = fs::read_to_string(&filepath);
    let source = file.unwrap();
    SOURCES.lock().unwrap().push(source.clone());

    let ast = if !englich {
        compile(source)
    } else {
        englich_compile(source)
    };

    let mut ctx = CodeGenContext::new();
    let mut cte = CompileTimeEnv::new(&mut ctx);

    ast.to_c(&mut cte, &mut ctx);

    let mut file = File::create(PROGRAM_NAME.lock().unwrap().clone() + ".c").unwrap();
    file.write_all(ctx.combine(&mut cte).as_bytes()).unwrap();

    SOURCES.lock().unwrap().pop();

    let error = ERROR.lock().unwrap().clone();
    error
}

pub fn error(span: Span, message: &str) {
    report(span.line, span.column, message);

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
    if line as isize - 2 > 0 {
        let prev_line = get_line(line - 2);
        println!("{CYAN}{:>3}{RESET} | {}", line - 2, prev_line);
    }

    if line as isize - 1 > 0 {
        let prev_line = get_line(line - 1);
        println!("{CYAN}{:>3}{RESET} | {}", line - 1, prev_line);
    }
    println!("{CYAN}{:>3}{RESET} | {}", line, source_line);

    let prefix_len = format!("{:>3}  | ", line).len();
    let caret_padding = " ".repeat((prefix_len + column).saturating_sub(3));

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

pub fn englich_compile(source: String) -> Expr {
    let mut scanner = englich_scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = englich_parser::Parser::new(tokens);
    let expr = parser.parse();

    expr
}

pub fn push_stack(name: &str) {
    CALL_STACK.lock().unwrap().push(name.to_string());
}

pub fn pop_stack() {
    CALL_STACK.lock().unwrap().pop();
}
