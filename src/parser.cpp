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

auto Parser::parse_program() -> ast::Program {
  auto function = parse_function();
  if (has_more_tokens()) {
    throw std::runtime_error("expected end of file");
  }
  return {.function = std::move(function)};
}

auto Parser::parse_function() -> ast::Function {
  consume_token(TokenType::KeywordInt);

  const auto name = parse_identifier();

  consume_token(TokenType::SymbolParenLeft);
  consume_token(TokenType::KeywordVoid);
  consume_token(TokenType::SymbolParenRight);
  consume_token(TokenType::SymbolBraceLeft);

  std::vector<ast::Statement> body;
  auto statement = parse_statement();
  body.push_back(std::move(statement));

  consume_token(TokenType::SymbolBraceRight);

  return {
      .name = name.name,
      .body = std::move(body),
  };
}

auto Parser::parse_statement() -> ast::Statement {
  return parse_return_statement();
}

auto Parser::parse_return_statement() -> ast::ReturnStatement {
  consume_token(TokenType::KeywordReturn);
  auto return_value = parse_expression();
  consume_token(TokenType::SymbolSemicolon);
  return ast::ReturnStatement{return_value};
}

auto Parser::parse_expression() -> ast::Expression {
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

auto Parser::parse_identifier() -> ast::Identifier {
  const auto ident = consume_token(TokenType::LiteralIdentifier);
  return ast::Identifier{ident.lexeme};
}

auto Parser::parse_integer_literal() -> ast::IntegerLiteral {
  const auto current = get_current();
  if (!current || current->type != TokenType::LiteralInteger) {
    throw std::runtime_error("expected integer literal");
  }

  m_current += 1;

  const auto lexeme = std::string(current->lexeme);
  const auto value = std::stoi(lexeme);
  return ast::IntegerLiteral(value);
}

} // namespace ecc
