#include "ecc/ast.hpp"

#include <iomanip>
#include <iostream>
#include <span>
#include <variant>

namespace ecc::ast {

class ExpressionPrinter {
public:
  auto operator()(const Identifier &identifier) -> void {
    std::cout << "Ident<" << std::quoted(identifier.name) << ">";
  }

  auto operator()(const IntegerLiteral &integer_literal) -> void {
    std::cout << "Int<" << integer_literal.value << '>';
  }
};

class StatementPrinter {
public:
  StatementPrinter(unsigned int indent_level) : m_indent_level(indent_level) {}

  auto operator()(const ReturnStatement &return_statement) -> void {
    for (unsigned int i = 0; i < m_indent_level; i++) {
      std::cout << '\t';
    }

    std::cout << "RETURN ";
    std::visit(ExpressionPrinter{}, return_statement.return_value);
    std::cout << '\n';
  }

private:
  unsigned int m_indent_level = 0;
};

static auto print_statements(std::span<const Statement> statements) -> void {
  for (const auto &statement : statements) {
    std::visit(StatementPrinter(2), statement);
  }
}

static auto print_function(const Function &function) -> void {
  std::cout << "FUN INT " << function.name << ":\n";
  std::cout << "\tparams: ()\n";
  std::cout << "\tbody:\n";

  print_statements(function.body);
}

auto print_ast(const Program &program) -> void {
  print_function(program.function);
}

} // namespace ecc::ast
