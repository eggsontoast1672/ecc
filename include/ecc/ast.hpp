#pragma once

#include <string>
#include <variant>
#include <vector>

namespace ecc {

struct Identifier;
struct IntegerLiteral;

/// The expression type.
///
/// This type is a big union of all of the possible expression structs.
using Expression = std::variant<Identifier, IntegerLiteral>;

/// An identifier.
struct Identifier {
  std::string name;
};

/// An integer literal node.
///
/// This structure represents a literal integer in the code. It evaluates to
/// itself, nothing too complicated.
struct IntegerLiteral {
  int value;
};

struct ReturnStatement;

/// The base class for all statement nodes.
using Statement = std::variant<ReturnStatement>;

/// A return statement node.
///
/// Return statements stop execution of the current function and yield the
/// specified value as the function's return value.
struct ReturnStatement {
  Expression return_value;
};

/// A function node.
///
/// Functions act as reusable blocks of code that can be parameterized. For
/// now, a function consists only of a name and a body. The return type is
/// assumed to be `int` and the parameter list is assumed to be `void`. The
/// name can be any identifier, but the linker will generate an error if there
/// is no `main` function defined.
struct Function {
  Identifier name;
  std::vector<Statement> body;
};

/// A program.
///
/// This node represents a C program. For now, a program consists of a single
/// function declaration. It can technically be called whatever, but if the
/// name of the function is not `main`, the linker will yell at you.
struct Program {
  Function function;
};

/// Pretty print the abstract syntax tree.
auto print_ast(const Program &program) -> void;

} // namespace ecc
