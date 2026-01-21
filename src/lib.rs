// #![warn(missing_docs)]

use std::{ffi::OsStr, path::Path, process::Command};

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod token;

pub fn compile_source(source: &str) -> String {
    let tokens = lexer::tokenize(source);
    let tree = parser::parse_token_stream(tokens).unwrap();
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
    std::fs::write(assembly_file.clone(), assembly).unwrap();

    // Link
    let stem = path.file_stem().unwrap();
    Command::new("ld")
        .args([assembly_file.as_os_str(), OsStr::new("-o"), stem])
        .output()
        .unwrap();

    // Remove assembly
    std::fs::remove_file(assembly_file).unwrap();
}
