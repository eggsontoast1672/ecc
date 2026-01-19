#pragma once

#include <cstddef>
#include <optional>
#include <stdexcept>
#include <string_view>

namespace ecc {

/// The type of a token.
///
/// This type represents the type associated with a token. This makes it easier
/// for the parser since it can look at the type of the token rather than
/// working directly with the lexeme.
enum class TokenType {
  KeywordInt,
  KeywordReturn,
  KeywordVoid,
  LiteralIdentifier,
  LiteralInteger,
  SpecialError,
  SymbolBraceLeft,
  SymbolBraceRight,
  SymbolParenLeft,
  SymbolParenRight,
  SymbolSemicolon,
};

/// Check if the given lexeme is a keyword.
///
/// If the lexeme is a keyword, the returned token type will indicate which one
/// it is. Otherwise, the returned token type is
/// `TokenType::LiteralIdentifier`.
auto check_keyword(std::string_view lexeme) -> TokenType;

/// A source code token.
///
/// Tokens are the smallest unit of lexical information. They are analogous to
/// words in spoken language. A token contains its type, the corresponding
/// substring of the source code (the lexeme), and the line and column info.
struct Token {
  TokenType type;
  std::string_view lexeme;
  std::size_t line;
  std::size_t column;
};

/// An error generated while lexing.
///
/// This error is reserved for situations where a lexer method is called, but
/// the state of the lexer is not appropriate. If you are only calling the
/// `next_token` method in a loop, you should never encounter this exception.
class LexError : public std::runtime_error {
public:
  /// Construct a lex error.
  explicit LexError(const char *what_arg);
};

class Lexer {
public:
  /// Construct a lexer.
  ///
  /// This constructor initializes the source view to the given string, setting
  /// the current character index to the beginning of the string and the line
  /// and column to 1.
  explicit Lexer(std::string_view source);

  /// Return true if the given character could be the start of an identifier.
  /// This includes uppercase and lowercase alphabetic characters and
  /// underscores.
  static auto is_ident_start(char c) -> bool;

  /// Return true if the given character could be in the middle of an
  /// identifier. This includes every character from `is_ident_start` as well
  /// as numeric characters.
  static auto is_ident(char c) -> bool;

  /// Return true if the given character is a digit, e.g. '0' to '9'.
  static auto is_digit(char c) -> bool;

  /// Get the current character.
  ///
  /// TODO: Write better description.
  [[nodiscard]] auto get_current() const -> std::optional<char>;

  /// Advance the lexer by one character.
  ///
  /// This method advances the lexer state to point to the next character in
  /// the source string. If the lexer was already at the end of the string, no
  /// operation is performed.
  auto advance() -> void;

  /// Skip past any whitespace.
  ///
  /// This method advances the position of the lexer until the current
  /// character is not a whitespace character. Naturally, if that was already
  /// the case when the method was called, the lexer's state is not altered.
  auto skip_whitespace() -> void;

  /// Make a token of the given type and advance.
  ///
  /// This method constructs a token with the given type, taking the line and
  /// column information from the lexer. The token is assumed to be one
  /// character long, so a single character substring is taken from the source.
  ///
  /// NOTE: This method is marked `[[nodiscard]]`. If you just want to advance
  /// the lexer, use `advance`.
  [[nodiscard]] auto make_token_and_advance(TokenType type) -> Token;

  /// Consume the next identifier from the source.
  ///
  /// This method assumes that the lexer's current character is the start of an
  /// identifier. If not, an exception is thrown.
  auto make_identifier() -> Token;

  /// Consume the next number from the source.
  auto make_number() -> Token;

  /// Create an error token.
  ///
  /// Error tokens represent unrecognized characters.
  auto make_error() -> Token;

  /// Extract the next token from the lexer.
  ///
  /// This method reads the next token from the source string. If the lexer has
  /// already read all of the tokens from the string (e.g. the source pointer
  /// is past the end of the string), then a null optional is returned.
  auto next_token() -> std::optional<Token>;

private:
  std::string_view m_source;
  std::size_t m_current;
  std::size_t m_line;
  std::size_t m_column;
};

} // namespace ecc
