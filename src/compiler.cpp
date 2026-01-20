#include "ecc/compiler.hpp"

#include <stdexcept>
#include <string>
#include <variant>

#include "ecc/ast.hpp"

namespace ecc {

auto Compiler::compile_program(const Program &program) -> void {
  compile_function(program.function);
}

auto Compiler::compile_function(const Function &function) -> void {
  m_assembly << "\t.globl " << function.name.name << '\n';
  m_assembly << function.name.name << ":\n";

  for (const auto &statement : function.body) {
    std::visit(*this, statement);
  }
}

auto Compiler::operator()(const Identifier &identifier) -> void {
  throw std::runtime_error("todo");
}

auto Compiler::operator()(const IntegerLiteral &integer_literal) -> void {
  m_assembly << '$' << integer_literal.value;
}

auto Compiler::operator()(const ReturnStatement &return_statement) -> void {
  m_assembly << "\tmovl\t";
 
  std::visit(*this, return_statement.return_value);

  m_assembly << ", %eax\n";
  m_assembly << "\tret\n";
}

auto Compiler::get_code() const -> std::string {
  return m_assembly.str();
}

} // namespace ecc
