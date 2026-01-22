// #![warn(missing_docs)]
#![allow(dead_code)]

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::parser::ParseError;
use crate::token::Token;

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod token;

/// Run the entire compilation pipeline, taking source code to assembly.
pub fn compile_source(source: &str) -> String {
    let tokens = lexer::tokenize(source);
    let tree = match parser::parse_token_stream(tokens) {
        Ok(tree) => tree,
        Err(e) => {
            print_parse_error(e, source);
            std::process::exit(1);
        }
    };

    compiler::compile_ast(tree)
}

pub fn compile_and_link<P>(path: P)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let source = std::fs::read_to_string(path).unwrap();
    let assembly = compile_source(&source);
    let assembly_file = path.with_extension("s");

    std::fs::write(assembly_file.clone(), assembly).unwrap();
    let result = link_program(&assembly_file);
    std::fs::remove_file(assembly_file).unwrap();
    if !result {
        std::process::exit(1);
    }
}

/// Run `gcc` on the given assembly file.
///
/// Since I do not really feel like writing my own linker and standard library, it seems like a
/// natural choice to link the program in this way. The return value indicates whether or not
/// linking was successful.
fn link_program<P>(assembly_file: P) -> bool
where
    P: AsRef<Path>,
{
    let assembly_file = assembly_file.as_ref();
    let without_extension = assembly_file.with_extension("");
    let output = Command::new("gcc")
        .args([
            OsStr::new("-o"),
            without_extension.as_os_str(),
            assembly_file.as_os_str(),
        ])
        .output()
        .unwrap();

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    output.status.success()
}

/// Print a pretty parse error.
fn print_parse_error(e: ParseError, source: &str) {
    match e.token {
        Some(token) => print_parse_error_at_token(token, &e.message, source),
        None => print_parse_error_at_eof(&e.message, source),
    }
}

fn print_parse_error_at_token(token: Token, message: &str, source: &str) {
    eprintln!("message: {message}");

    // The unwrap call here should never fail. This is because the token came from some line in the
    // source code, so if the lexer did its job correctly, there should exist a line whose number
    // mathes that of the token.
    let (_, line) = source
        .lines()
        .enumerate()
        .find(|(number, _)| *number == token.line - 1)
        .unwrap();

    let space_padding = token.column - 1;
    let tilde_padding = token.lexeme.len() - 1;

    eprintln!(" {:>4} | {line}", token.line);
    eprintln!("      | {: <space_padding$}^{:~<tilde_padding$}", "", "",);
}

fn print_parse_error_at_eof(message: &str, source: &str) {
    eprintln!("message: {}", message);

    let lines = source.lines();
    let (number, line) = lines.enumerate().last().unwrap();
    let padding = line.len();

    eprintln!(" {number:>4} | {line}");
    eprintln!("      | {: <padding$}^", "");
}
