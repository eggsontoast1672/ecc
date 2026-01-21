use std::fmt::Write;

use crate::ast;

pub fn compile_ast(program: &ast::Program) -> String {
    let mut compiler = Compiler::new();
    compiler.compile_program(program);
    compiler.finish()
}

macro_rules! write_unwrap {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).unwrap()
    }
}

macro_rules! writeln_unwrap {
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*).unwrap()
    }
}

/// The compiler.
///
/// This class is responsible for turining an abstract syntax tree into
/// assembly.
pub struct Compiler {
    assembly: String,
}

impl Compiler {
    /// Create a new compiler with empty assembly buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecc::compiler::Compiler;
    ///
    /// let compiler = Compiler::new();
    ///
    /// assert_eq!(compiler.get_code(), String::new());
    /// ```
    fn new() -> Self {
        Self {
            assembly: String::new(),
        }
    }

    pub fn get_code(&self) -> &str {
        &self.assembly
    }

    fn finish(self) -> String {
        self.assembly
    }

    /// Compile a program.
    ///
    /// This method compiles a C program down to assembly. For now, a program consists of a single
    /// function declaration. That function's name can be anything and the compiler will work, but
    /// if the name is not `main` then the linker will complain.
    pub fn compile_program(&mut self, program: &ast::Program) {
        self.compile_function(&program.function);
    }

    pub fn compile_function(&mut self, function: &ast::Function) {
        writeln_unwrap!(self.assembly, "\t.globl {}", function.name);
        writeln_unwrap!(self.assembly, "{}:", function.name);

        for statement in &function.body {
            self.compile_statement(statement);
        }
    }

    pub fn compile_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Return(expr) => self.compile_return(expr),
        }
    }

    pub fn compile_return(&mut self, return_value: &ast::Expression) {
        write_unwrap!(self.assembly, "\tmovl\t");
        self.compile_expression(return_value);
        writeln_unwrap!(self.assembly, ", %eax");
        writeln_unwrap!(self.assembly, "\tret");
    }

    pub fn compile_expression(&mut self, expr: &ast::Expression) {
        match expr {
            ast::Expression::Integer(value) => self.compile_integer(*value),
        }
    }

    pub fn compile_integer(&mut self, value: i32) {
        writeln_unwrap!(self.assembly, "${}", value);
    }
}
