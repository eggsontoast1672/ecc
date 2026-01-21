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

#[allow(unused)]
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
    fn compile_statement(&mut self, statement: &ast::Statement) {
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
    fn compile_return(&mut self, return_value: &ast::Expression) {
        self.compile_expression(return_value);
        writeln_unwrap!(self.assembly, "\tret");
    }

    /// Compile an expression.
    ///
    /// For now, all manipulation of expressions happens in the `eax` register. This is because the
    /// only meaningful thing that we can do is return an integer from `main`, and since that
    /// integer must be stored in `eax` according to the calling convention, it is a logical
    /// register to use for operations.
    fn compile_expression(&mut self, expr: &ast::Expression) {
        match expr {
            ast::Expression::Integer(value) => self.compile_integer(*value),
            ast::Expression::Unary { operator, operand } => self.compile_unary(*operator, operand),
        }
    }

    /// Compile an integer literal.
    ///
    /// This method loads the given integer into the `eax` register.
    fn compile_integer(&mut self, value: i32) {
        writeln_unwrap!(self.assembly, "\tmovl\t${}, %eax", value);
    }

    /// Compile a unary expression.
    fn compile_unary(&mut self, operator: ast::UnaryOperator, operand: &ast::Expression) {
        self.compile_expression(operand);

        use ast::UnaryOperator as UO; // 'Sco Ducks

        match operator {
            UO::Compliment => writeln_unwrap!(self.assembly, "\tnot\t%eax"),
            UO::NegateArith => writeln_unwrap!(self.assembly, "\tneg\t%eax"),
            UO::NegateLogical => {
                writeln_unwrap!(self.assembly, "\tcmpl\t$0, %eax");
                writeln_unwrap!(self.assembly, "\tmovl\t$0, %eax");
                writeln_unwrap!(self.assembly, "\tsete\t%al");
            }
        }
    }
}
