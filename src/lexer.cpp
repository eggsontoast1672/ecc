#include "ecc/lexer.hpp"

#include <cctype>
#include <cstddef>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string_view>

namespace ecc {

auto operator<<(std::ostream &stream, TokenType type) -> std::ostream & {
  switch (type) {
#define X(token_type)                                                          \
  case TokenType::token_type:                                                  \
    stream << "TokenType::" #token_type;                                       \
    break;
    TOKEN_TYPES
#undef X
  }

  return stream;
}

auto check_keyword(std::string_view lexeme) -> TokenType {
  // This implementation is probably not the most efficient, but do I care?
  // Definitely not until this becomes an observable performance bottleneck.
  if (lexeme == "int") {
    return TokenType::KeywordInt;
  } else if (lexeme == "return") {
    return TokenType::KeywordReturn;
  } else if (lexeme == "void") {
    return TokenType::KeywordVoid;
  } else {
    return TokenType::LiteralIdentifier;
  }
}

LexError::LexError(const char *what_arg) : std::runtime_error(what_arg) {}

Lexer::Lexer(std::string_view source)
    : m_source(source), m_current(0), m_line(1), m_column(1) {}

auto Lexer::is_ident_start(char c) -> bool {
  return ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z') || c == '_';
}

auto Lexer::is_ident(char c) -> bool {
  return is_ident_start(c) || is_digit(c);
}

auto Lexer::is_digit(char c) -> bool { return '0' <= c && c <= '9'; }

auto Lexer::get_current() const -> std::optional<char> {
  if (m_current < m_source.size()) {
    return std::make_optional(m_source[m_current]);
  } else {
    return std::nullopt;
  }
}

auto Lexer::advance() -> void {
  const auto current = get_current();
  if (!current) {
    return;
  }

  m_current += 1;
  m_column += 1;

  if (*current == '\n') {
    m_line += 1;
    m_column = 1;
  }
}

auto Lexer::skip_whitespace() -> void {
  while (true) {
    const auto current = get_current();
    if (!current || !std::isspace(*current)) {
      return;
    }

    advance();
  }
}

auto Lexer::make_token_and_advance(TokenType type) -> Token {
  const Token token = {
      TokenType::SymbolBraceLeft,
      m_source.substr(m_current, 1),
      m_line,
      m_column,
  };
  advance();
  return token;
}

auto Lexer::make_identifier() -> Token {
  const auto current = get_current();
  if (!current || !is_ident_start(*current)) {
    throw LexError("expected the start of an identifier");
  }

  const std::size_t start = m_current;
  std::size_t length = 0;

  advance();

  while (true) {
    const auto current = get_current();
    if (!current || !is_ident(*current)) {
      const auto lexeme = m_source.substr(start, length);
      const auto type = check_keyword(lexeme);
      return {
          .type = type,
          .lexeme = lexeme,
          .line = m_line,
          .column = m_column,
      };
    }

    length += 1;
    advance();
  }
}

auto Lexer::make_number() -> Token {
  const auto current = get_current();
  if (!current || !is_digit(*current)) {
    throw LexError("expected a digit");
  }

  const std::size_t start = m_current;
  std::size_t length = 0;

  advance();

  while (true) {
    const auto current = get_current();
    if (!current || !is_digit(*current)) {
      return {
          .type = TokenType::LiteralInteger,
          .lexeme = m_source.substr(start, length),
          .line = m_line,
          .column = m_column,
      };
    }

    length += 1;
    advance();
  }
}

auto Lexer::make_error() -> Token {
  using namespace std::literals;

  const Token token = {
      .type = TokenType::SpecialError,
      .lexeme = "unrecognized character"s,
      .line = m_line,
      .column = m_column,
  };
  advance();
  return token;
}

auto Lexer::next_token() -> std::optional<Token> {
  skip_whitespace();

  const auto current = get_current();
  if (!current) {
    return std::nullopt;
  }

  switch (*current) {
  case '{':
    return make_token_and_advance(TokenType::SymbolBraceLeft);
  case '}':
    return make_token_and_advance(TokenType::SymbolBraceRight);
  case '(':
    return make_token_and_advance(TokenType::SymbolParenLeft);
  case ')':
    return make_token_and_advance(TokenType::SymbolParenRight);
  case ';':
    return make_token_and_advance(TokenType::SymbolSemicolon);
  default:
    if (is_ident_start(*current)) {
      return make_identifier();
    } else if (is_digit(*current)) {
      return make_number();
    } else {
      return make_error();
    }
  }
}

} // namespace ecc
