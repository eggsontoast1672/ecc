#include "ecc/lexer.h"

#define SINGLE_CHARACTER(c, kind_)                                             \
  case c:                                                                      \
    token.kind = (kind_);                                                      \
    token.lexeme = lexer->current;                                             \
    token.lexeme_length = 1;                                                   \
    lexer->current++;                                                          \
    break;

token_t lexer_next_token(lexer_t *lexer) {
  token_t token = {0};
  lexer_skip_whitespace(lexer);
  switch (*lexer->current) {
    SINGLE_CHARACTER('*', TOKEN_STAR);
    SINGLE_CHARACTER('/', TOKEN_SLASH);
    SINGLE_CHARACTER(':', TOKEN_COLON);
    SINGLE_CHARACTER(',', TOKEN_COMMA);
    SINGLE_CHARACTER('.', TOKEN_DOT);
    SINGLE_CHARACTER(';', TOKEN_SEMICOLON);
    SINGLE_CHARACTER('{', TOKEN_BRACE_LEFT);
    SINGLE_CHARACTER('}', TOKEN_BRACE_RIGHT);
    SINGLE_CHARACTER('[', TOKEN_BRACKET_LEFT);
    SINGLE_CHARACTER(']', TOKEN_BRACKET_RIGHT);
    SINGLE_CHARACTER('(', TOKEN_PAREN_LEFT);
    SINGLE_CHARACTER(')', TOKEN_PAREN_RIGHT);

    DOUBLE_CHARACTER('!', '=', TOKEN_BANG, TOKEN_BANG_EQUAL);
  }
}
