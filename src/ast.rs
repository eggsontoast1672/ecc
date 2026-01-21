/// A program.
///
/// This node represents a C program. For now, a program consists of a single function declaration.
/// It can technically be called whatever, but if the name of the function is not `main`, the
/// linker will yell at you.
#[derive(Clone, Debug)]
pub struct Program {
    /// The single function of the program.
    pub function: Function,
}

/// A function node.
///
/// Functions act as reusable blocks of code that can be parameterized. For now, a function
/// consists only of a name and a body. The return type is assumed to be `int` and the parameter
/// list is assumed to be `void`. The name can be any identifier, but the linker will generate an
/// error if there is no `main` function defined.
#[derive(Clone, Debug)]
pub struct Function {
    /// The function's name.
    pub name: String,

    /// The body of the function.
    pub body: Vec<Statement>,
}

/// An operator that can appear in a unary expression.
#[derive(Clone, Debug, Copy)]
pub enum UnaryOperator {
    Compliment,
    NegateArith,
    NegateLogical,
}

/// An expression.
///
/// Expressions are any part of the source code which can evaluate to a value. For example,
/// literals like integers, floating point numbers, or strings.
#[derive(Clone, Debug)]
pub enum Expression {
    /// An integer literal.
    Integer(i32),

    /// A unary expression.
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
}

/// A statement.
///
/// As opposed to expressions, statements *do* something. They are like commands.
#[derive(Clone, Debug)]
pub enum Statement {
    /// A return statement.
    Return(Expression),
}
