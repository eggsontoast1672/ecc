use crate::token::TokenKind;

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
#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Compliment,
    NegateArith,
    NegateLogical,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
}

impl BinaryOp {
    /// Get the corresponding operator from the token kind.
    ///
    /// This is a useful function during the parsing stage, since it makes more sense to pass
    /// around token kinds in the parser then to pass around operators. The correspondence is
    /// pretty intuitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecc::ast::BinaryOp;
    /// use ecc::token::TokenKind;
    ///
    /// assert_eq!(from_token_kind(TokenKind::OperatorPlus), BinaryOp::Plus);
    /// assert_eq!(from_token_kind(TokenKind::OperatorMinus), BinaryOp::Minus);
    /// ```
    pub(crate) fn from_token_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::OperatorPlus => Some(Self::Plus),
            TokenKind::OperatorMinus => Some(Self::Minus),
            TokenKind::OperatorStar => Some(Self::Times),
            TokenKind::OperatorSlash => Some(Self::Divide),
            _ => None,
        }
    }
}

/// An expression.
///
/// Expressions are any part of the source code which can evaluate to a value. For example,
/// literals like integers, floating point numbers, or strings.
#[derive(Clone, Debug)]
pub enum Expr {
    /// An integer literal.
    Integer(i32),

    /// A unary expression.
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    /// A binary expression.
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// A statement.
///
/// As opposed to expressions, statements *do* something. They are like commands.
#[derive(Clone, Debug)]
pub enum Statement {
    /// A return statement.
    Return(Expr),
}
