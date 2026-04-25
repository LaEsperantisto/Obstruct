#![allow(dead_code)]
extern crate core;
pub const DEBUG: bool = false;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PATH: &str = "/home/aster/dev/obstruct/main.obs";

mod error;
mod expr;
mod init;
mod runtime_env;

mod transpiler {
    pub mod code_gen_context;
    pub mod compiletime_env;
    pub mod expr_to_c;
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

// FIXME
//  Rework generic types

// TODO
//  Add "Vec"
//  Add "str"
//  Add conversion (a function) from "strlit" to "&str"
//  Add generic functions:
//      Create function instances for each generic variant
//  Add generic classes
//  Add checks for "\x"
//  Let function calls accept any expression as left hand expression, not just directly calling a variable

use crate::error::ObstructError;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::transpiler::compiletime_env::CompileTimeEnv;
use colored::Colorize;
use image::GenericImageView;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::panic;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;

// Paths
pub const STD_PATH: &str = "/home/aster/dev/rust/Obstruct/std/"; // end in a "/" !!!

// Global Variables
static SOURCES: Mutex<Vec<String>> = Mutex::new(vec![]);
static ERROR: Mutex<Result<(), ObstructError>> = Mutex::new(Ok(()));
static CALL_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PROGRAM_NAME: Mutex<String> = Mutex::new(String::new());
static RUNNING_TESTS: Mutex<bool> = Mutex::new(true);

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
const HELP_COLOR: &str = BRIGHT_GREEN;

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
        .arg("-lm")
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
    let args: Vec<String> = std::env::args().collect();

    let (command, filepath) = if DEBUG {
        ("run".to_string(), Some(DEFAULT_PATH))
    } else {
        if args.len() < 2 {
            eprintln!("{BOLD}{RED}error:{RESET} missing arguments");
            eprintln!("Usage: {} <command> <file> [--release]", args[0]);
            eprintln!("Commands: transpile, build, run, cleanup, --version");
            std::process::exit(1);
        }
        let cmd = args[1].clone();
        let needs_file = cmd == "transpile" || cmd == "build" || cmd == "run";
        let filepath = if needs_file && args.len() >= 3 {
            Some(args[2].as_str())
        } else if needs_file {
            eprintln!("{BOLD}{RED}error:{RESET} expected path after '{}'", cmd);
            std::process::exit(1);
        } else {
            None
        };
        (cmd, filepath)
    };

    match command.as_str() {
        "transpile" | "build" | "run" | "cleanup" | "--version" => {}
        _ => {
            eprintln!("{BOLD}{RED}error:{RESET} unknown command '{}'", command);
            eprintln!("Usage: {} <transpile|build|run|cleanup> <file>", args[0]);
            std::process::exit(1);
        }
    }

    if let Some(fp) = filepath {
        if !Path::new(fp).exists() {
            eprintln!("{BOLD}{RED}error:{RESET} file not found: {}", fp);
            std::process::exit(1);
        }
    }

    if command == "build" || command == "run" || command == "transpile" {
        let start = Instant::now();
        *RUNNING_TESTS.lock().unwrap() = false;
        let result = run(filepath.unwrap_or_else(|| {
            eprintln!("{BOLD}{RED}error:{RESET} expected path after '{}'", command);
            DEFAULT_PATH
        }));

        if result.is_err() {
            let err = result.unwrap_err();
            error(err.span, &err.message, "transpilation");
            std::process::exit(1);
        }

        let program_name = PROGRAM_NAME.lock().unwrap().clone();
        let transpile_time = start.elapsed();
        println!(
            "{} Took {:?} to transpile into C.",
            "Done:".green().bold(),
            transpile_time
        );

        if command == "build" || command == "run" {
            compile_c_file(&(program_name.clone() + ".c"), &program_name);
            let compile_time = start.elapsed();
            println!(
                "{} Took {:?} to compile C code.",
                "Done:".green().bold(),
                compile_time - transpile_time
            );
        }

        if command == "run" {
            println!(
                "{} Running {}...\n",
                "Execute:".bright_blue().bold(),
                program_name
            );
            run_compiled_c_file(&program_name);
        }

        result
    } else {
        match command.as_str() {
            "cleanup" => {
                let dir = Path::new(".");

                for entry in fs::read_dir(dir).unwrap().filter_map(Result::ok) {
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "c" {
                                fs::remove_file(&path).unwrap();
                            }
                        }
                    }
                }

                Ok(())
            }
            "--version" => {
                println!("obstruct {VERSION}");
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

fn run(filepath: &str) -> Result<(), ObstructError> {
    let img = image::open("/home/aster/dev/rust/Obstruct/gfx/icon.png").unwrap();
    let size = 50;
    let img = img.thumbnail(size as u32, size as u32);
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            print!("{}", "  ".on_truecolor(pixel[0], pixel[1], pixel[2]));
        }
        println!();
    }

    let is_release = std::env::args().any(|a| a == "--release");
    let debug_val = !is_release;

    let main_program_name = Path::new(&filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(&filepath)
        .to_string();

    if main_program_name.is_empty() {
        eprintln!("Invalid program name");
        std::process::exit(1);
    }

    *PROGRAM_NAME.lock().unwrap() = main_program_name.clone();

    let mut ctx = CodeGenContext::new();
    let mut cte = CompileTimeEnv::new(&mut ctx);

    Expr::Discard(Box::new(Expr::Declare(
        "DEBUG".to_string(),
        None,
        Some(Box::new(Expr::Bool(debug_val))),
        false,
        Span::empty(),
    )))
    .to_c(&mut cte, &mut ctx);
    ctx.body.push_str(";\n");

    let mut programs_to_transpile = HashMap::new();
    programs_to_transpile.insert(filepath.to_string(), false);

    loop {
        let next_file = programs_to_transpile
            .iter()
            .find(|(_, transpiled)| !**transpiled)
            .map(|(name, _)| name.clone());

        match next_file {
            Some(name) => {
                programs_to_transpile.insert(name.clone(), true);
                let source = fs::read_to_string(&name)
                    .map_err(|_| ObstructError::file_not_found(name.to_string()))?;
                SOURCES.lock().unwrap().push(source.clone());
                let ast = parse(source);
                let current_dir = Path::new(&name)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                ast.pre_transpile(&mut cte, &mut ctx, &mut programs_to_transpile, &current_dir);
                SOURCES.lock().unwrap().pop();
            }
            None => break,
        }
    }

    for programs in programs_to_transpile.keys() {
        let source = fs::read_to_string(&programs)
            .map_err(|_| ObstructError::file_not_found(programs.clone()))?;
        SOURCES.lock().unwrap().push(source.clone());
        let ast = parse(source);
        ast.to_c(&mut cte, &mut ctx);
        SOURCES.lock().unwrap().pop();
    }

    let mut file = File::create(PROGRAM_NAME.lock().unwrap().clone() + ".c").unwrap();
    file.write_all(ctx.combine(&mut cte).as_bytes()).unwrap();

    let error = ERROR.lock().unwrap().clone();
    error
}

pub fn error(span: Span, message: &str, place: &str) {
    if !RUNNING_TESTS.lock().unwrap().clone() {
        report(span.line, span.column, message, place);

        panic::set_hook(Box::new(|_| {}));
    }
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

pub fn report(line: usize, column: usize, message: &str, place: &str) {
    let mut err = ERROR.lock().unwrap();

    println!(
        "\n{BOLD}{ERROR_COLOR}error{RESET} {HELP_COLOR}({place}){RESET}{BOLD}: {message}{RESET}"
    );

    println!("--> line {} column {}\n", line, column);

    let source_line = get_line(line);

    println!("    |");
    if line as isize - 3 > 0 {
        let prev_line = get_line(line - 3);
        println!("{CYAN}{:>3}{RESET} | {}", line - 3, prev_line);
    }

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

pub fn parse(source: String) -> Expr {
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
