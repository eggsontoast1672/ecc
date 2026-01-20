#pragma once

#include <sstream>
#include <string>

#include "ecc/ast.hpp"

namespace ecc {

/// The compiler.
///
/// This class is responsible for turining an abstract syntax tree into
/// assembly.
class Compiler {
public:
  auto compile_program(const Program &program) -> void;
  auto compile_function(const Function &function) -> void;
  
  auto operator()(const Identifier &identifier) -> void;
  auto operator()(const IntegerLiteral &integer_literal) -> void;
  auto operator()(const ReturnStatement &return_statement) -> void;

  auto get_code() const -> std::string;

private:
  std::ostringstream m_assembly;
};

} // namespace ecc
