#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "ecc/ast.hpp"
#include "ecc/lexer.hpp"
#include "ecc/parser.hpp"

static auto read_file(const char *path) -> std::string {
  std::ifstream file(path);
  if (!file.is_open()) {
    throw std::runtime_error("failed to open file");
  }

  std::ostringstream contents;
  contents << file.rdbuf();
  return contents.str();
}

static auto debug_tokens(std::string_view source) -> void {
  std::cout << source << '\n';

  ecc::Lexer lexer(source);

  while (true) {
    const auto token = lexer.next_token();
    if (!token) {
      break;
    }

    std::cout << "Token{ " << token->type << ", \"" << token->lexeme << "\", "
              << token->line << ", " << token->column << " }\n";
  }
}

auto main(int argc, char **argv) -> int {
  if (argc < 2) {
    std::cout << "Usage: " << argv[0] << " [file]\n";
    return 1;
  }

  const auto source = read_file(argv[1]);
  ecc::Lexer lexer(source);
  std::vector<ecc::Token> tokens;
  while (true) {
    const auto token = lexer.next_token();
    if (token) {
      tokens.push_back(*token);
    } else {
      break;
    }
  }

  ecc::Parser parser(tokens);
  const auto program = parser.parse_program();
  ecc::ast::print_ast(program);
}
