#pragma once

#include <cstddef>
#include <memory>
#include <optional>
#include <string_view>
#include <vector>

#include "ecc/ast.hpp"
#include "ecc/lexer.hpp"

namespace ecc {

/// The parser.
class Parser {
public:
  explicit Parser(const std::vector<Token> &tokens);

  /// Return true if there are more tokens to be processed (e.g. `get_current`
  /// would not return a null optional), and false otherwise.
  auto has_more_tokens() const -> bool;

  /// Get the token the parser is currently pointing to.
  ///
  /// If the parser has reached the end of the token stream and is pointing to
  /// nothing, a null optional is returned.
  auto get_current() const -> std::optional<Token>;

  /// Consume a token of the specified type.
  ///
  /// This method looks at the type of the current token, and if it matches the
  /// supplied type, the parser is advanced and the consumed token is returned.
  /// If the types do not match, an exception is thrown.
  auto consume_token(TokenType type) -> Token;

  /// Parse a program.
  ///
  /// This method will parse a program (a single function declaration). After
  /// that, it asserts that there are no more tokens to be processed. If there
  /// are, an exception is thrown.
  auto parse_program() -> ast::Program;

  auto parse_function() -> ast::Function;

  auto parse_statement() -> ast::Statement;

  auto parse_return_statement() -> ast::ReturnStatement;

  auto parse_expression() -> ast::Expression;

  auto parse_identifier() -> ast::Identifier;

  auto parse_integer_literal() -> ast::IntegerLiteral;

private:
  std::vector<Token> m_tokens;
  std::size_t m_current;
};

} // namespace ecc
