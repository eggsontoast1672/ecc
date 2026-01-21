use std::fmt::Write;

use crate::ast;

/// Compile a program to assembly.
///
/// This function generates a string containing `x86_64` assembly code compiled from the given
/// abstract syntax tree. For now, it is guaranteed to link properly if the source code contains a
/// `main` function.
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

    fn finish(self) -> String {
        self.assembly
    }

    /// Compile a program.
    ///
    /// This method compiles a C program down to assembly. For now, a program consists of a single
    /// function declaration. That function's name can be anything and the compiler will work, but
    /// if the name is not `main` then the linker will complain.
    fn compile_program(&mut self, program: &ast::Program) {
        self.compile_function(&program.function);
    }

    /// Compile a function.
    ///
    /// This method generates a global instruction to expose the function's label to the linker.
    /// Then it generates a label corresponding to the function's name, followed by all of the code
    /// for the function.
    fn compile_function(&mut self, function: &ast::Function) {
        writeln_unwrap!(self.assembly, "\t.globl {}", function.name);
        writeln_unwrap!(self.assembly, "{}:", function.name);

        for statement in &function.body {
            self.compile_statement(statement);
        }
    }

    /// Compile a statement.
    ///
    /// This method compiles a single statement. The generated assembly (obviously) depends greatly
    /// on the type of statement being compiled.
    pub fn compile_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Return(expr) => self.compile_return(expr),
        }
    }

    /// Compile a return statement.
    ///
    /// This method generates a `movl` instruction, passing it the integer return value and the
    /// `%eax` register. In the future, functions will be able to return more than 32-bit integer
    /// values, but this is how it is for now. Naturally, the return statement is terminated with a
    /// `ret` instruction.
    pub fn compile_return(&mut self, return_value: &ast::Expression) {
        write_unwrap!(self.assembly, "\tmovl\t");
        self.compile_expression(return_value);
        writeln_unwrap!(self.assembly, ", %eax");
        writeln_unwrap!(self.assembly, "\tret");
    }

    /// Compile an expression.
    ///
    /// This method generates the corresponding expression in `at&t` syntax.
    pub fn compile_expression(&mut self, expr: &ast::Expression) {
        match expr {
            ast::Expression::Integer(value) => self.compile_integer(*value),
        }
    }

    /// Compile an integer literal.
    ///
    /// This method inserts the given integer into the assembly code directly after a dollar (`$`)
    /// sign, as is expected in `at&t`.
    pub fn compile_integer(&mut self, value: i32) {
        write_unwrap!(self.assembly, "${}", value);
    }
}
