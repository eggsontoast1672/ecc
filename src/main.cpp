#include <iostream>

#include "ecc/lexer.hpp"

int main(int argc, char **argv) {
  if (argc < 2) {
    std::cout << "Usage: " << argv[0] << " [source]\n";
    return 1;
  }

  const auto source = argv[1];
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
