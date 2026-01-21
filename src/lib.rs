// #![warn(missing_docs)]
#![allow(dead_code)]

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::parser::ParseError;

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod token;

pub fn compile_source(source: &str) -> String {
    let tokens = lexer::tokenize(source);
    let tree = match parser::parse_token_stream(tokens) {
        Ok(tree) => tree,
        Err(e) => {
            print_parse_error(e, source);
            std::process::exit(1);
        }
    };

    compiler::compile_ast(&tree)
}

pub fn compile_and_link<P>(path: P)
where
    P: AsRef<Path>,
{
    // Compilation
    let path = path.as_ref();
    let source = std::fs::read_to_string(path).unwrap();
    let assembly = compile_source(&source);

    // Write to file
    let assembly_file = path.with_extension("s");
    eprintln!("writing assembly to {:?}", assembly_file);
    std::fs::write(assembly_file.clone(), assembly).unwrap();

    // Link
    let stem = path.with_extension("");
    eprintln!("writing binary to {:?}", stem);
    let output = Command::new("gcc")
        .args([assembly_file.as_os_str(), OsStr::new("-o"), stem.as_os_str()])
        .output()
        .unwrap();

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    // Remove assembly
    eprintln!("removing {:?}", assembly_file);
    std::fs::remove_file(assembly_file).unwrap();
}

fn print_parse_error(e: ParseError, source: &str) {
    eprintln!("message: {}", e.message);
    // eprintln!("debug: {:?}", e.token);

    let lines = source.lines();

    if let Some(token) = e.token {
        let line = lines
            .enumerate()
            .find(|(number, _)| *number == token.line - 1)
            .unwrap();

        eprintln!(" {:>4} | {}", token.line, line.1);
        eprint!("      | ");
        for _ in 0..token.column - 1 {
            eprint!(" ");
        }
        eprint!("^");
        for _ in 0..token.lexeme.len() - 1 {
            eprint!("~");
        }
        eprintln!();
    } else {
        let (number, line) = lines.enumerate().last().unwrap();
        eprintln!(" {:>4} | {}", number, line);
        eprint!("      | ");
        for _ in 0..line.len() {
            eprint!(" ");
        }
        eprintln!("^");
    }
}
