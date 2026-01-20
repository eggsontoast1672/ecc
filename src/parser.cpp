#include "ecc/parser.hpp"

#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "ecc/ast.hpp"

namespace ecc {

Parser::Parser(const std::vector<Token> &tokens)
    : m_tokens(tokens), m_current(0) {}

auto Parser::has_more_tokens() const -> bool {
  return m_current < m_tokens.size();
}

auto Parser::get_current() const -> std::optional<Token> {
  if (has_more_tokens()) {
    return std::make_optional(m_tokens[m_current]);
  } else {
    return std::nullopt;
  }
}

auto Parser::consume_token(TokenType type) -> Token {
  const auto current = get_current();
  if (!current || current->type != type) {
    std::ostringstream message;
    message << "expected token of type " << type;
    throw std::runtime_error(message.str());
  } else {
    m_current += 1;
  }
  return *current;
}

auto Parser::parse_program() -> Program {
  const auto function = parse_function();
  if (has_more_tokens()) {
    throw std::runtime_error("expected end of file");
  }

  return Program{function};
}

auto Parser::parse_function() -> Function {
  consume_token(TokenType::KeywordInt);

  const auto name = parse_identifier();

  consume_token(TokenType::SymbolParenLeft);
  consume_token(TokenType::KeywordVoid);
  consume_token(TokenType::SymbolParenRight);
  consume_token(TokenType::SymbolBraceLeft);

  const auto return_statement = parse_statement();

  consume_token(TokenType::SymbolBraceRight);

  return Function{
      .name = name,
      .body = std::vector{return_statement},
  };
}

auto Parser::parse_statement() -> Statement {
  return parse_return_statement();
}

auto Parser::parse_return_statement() -> ReturnStatement {
  consume_token(TokenType::KeywordReturn);
  const auto return_value = parse_expression();
  consume_token(TokenType::SymbolSemicolon);
  return ReturnStatement{return_value};
}

auto Parser::parse_expression() -> Expression {
  const auto current = get_current();
  if (!current) {
    throw std::runtime_error("expected expression");
  }

  switch (current->type) {
  case TokenType::LiteralInteger:
    return parse_integer_literal();
  default:
    throw std::runtime_error("expected expression");
  }
}

auto Parser::parse_identifier() -> Identifier {
  const auto ident = consume_token(TokenType::LiteralIdentifier);
  return Identifier{std::string(ident.lexeme)};
}

auto Parser::parse_integer_literal() -> IntegerLiteral {
  const auto current = get_current();
  if (!current || current->type != TokenType::LiteralInteger) {
    throw std::runtime_error("expected integer literal");
  }

  m_current += 1;

  const auto lexeme = std::string(current->lexeme);
  const auto value = std::stoi(lexeme);
  return IntegerLiteral(value);
}

} // namespace ecc
