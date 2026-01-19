#pragma once

#include <memory>
#include <string_view>
#include <vector>

namespace ecc {

/// The base class for all expression nodes.
struct Expression {
public:
  virtual ~Expression() = 0;
};

/// An integer literal node.
///
/// This structure represents a literal integer in the code. It evaluates to
/// itself, nothing too complicated.
struct IntegerLiteral : public Expression {
  int value;
  ~IntegerLiteral() override = default;
};

/// The base class for all statement nodes.
struct Statement {
public:
  virtual ~Statement() = 0;
};

/// A return statement node.
///
/// Return statements stop execution of the current function and yield the
/// specified value as the function's return value.
struct ReturnStatement : public Statement {
  std::unique_ptr<Expression> return_value;
  ~ReturnStatement() override = default;
};

/// A function node.
///
/// Functions act as reusable blocks of code that can be parameterized. For
/// now, a function consists only of a name and a body. The return type is
/// assumed to be `int` and the parameter list is assumed to be `void`. The
/// name can be any identifier, but the linker will generate an error if there
/// is no `main` function defined.
struct Function {
  std::string_view name;
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

} // namespace ecc
