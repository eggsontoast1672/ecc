use crate::token::Token;
use crate::token::TokenKind;
use crate::token::check_keyword;

/// Tokenize a string of source code.
///
/// This function lexes a string of C source code into individual tokens. If the source code is not
/// ascii, you will get some very strange results.
pub fn tokenize(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut lexer = Lexer::new(bytes);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    tokens
}

struct Lexer<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Construct a lexer.
    ///
    /// This constructor initializes the source view to the given string, setting the current
    /// character index to the beginning of the string and the line and column to 1.
    fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Return true if the given character could be the start of an identifier. This includes
    /// uppercase and lowercase alphabetic characters and underscores.
    fn is_ident_start(c: u8) -> bool {
        (b'A' <= c && c <= b'Z') || (b'a' <= c && c <= b'z') || c == b'_'
    }

    /// Return true if the given character could be in the middle of an identifier. This includes
    /// every character from `is_ident_start` as well as numeric characters.
    fn is_ident(c: u8) -> bool {
        Self::is_ident_start(c) || Self::is_digit(c)
    }

    /// Return true if the given character is a digit, e.g. '0' to '9'.
    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    /// Get the current character.
    ///
    /// TODO: Write better description.
    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.source.get(self.current + 1).copied()
    }

    /// Advance the lexer by one character.
    ///
    /// This method advances the lexer state to point to the next character in the source string
    /// and returns that character. If the lexer was already at the end of the string, no operation
    /// is performed and [`None`] is returned.
    fn advance(&mut self) -> Option<u8> {
        let current = self.peek();
        if let Some(current) = current {
            self.current += 1;
            self.column += 1;

            if current == b'\n' {
                self.line += 1;
                self.column = 1;
            }
        }

        current
    }

    /// Skip past any whitespace and comments.
    ///
    /// This method advances the position of the lexer until the current character is not a
    /// whitespace character. If the next non-whitespace character is a slash followed by another
    /// slash, the comment will be skipped. Naturally, if that was already the case when the method
    /// was called, the lexer's state is not altered.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.advance();
                continue;
            }

            if c == b'/'
                && let Some(b'/') = self.peek_next()
            {
                while let Some(c) = self.peek()
                    && c != b'\n'
                {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    /// Make a token of the given type and advance.
    ///
    /// This method constructs a token with the given type, taking the line and column information
    /// from the lexer. The token is assumed to be one character long, so a single character
    /// substring is taken from the source.
    ///
    /// NOTE: This method is marked `#[must_use]`. If you just want to advance the lexer, use
    /// `advance`.
    #[must_use]
    fn make_token_and_advance(&mut self, kind: TokenKind) -> Token {
        let bytes = &self.source[self.current..self.current + 1];
        let lexeme = str::from_utf8(bytes).unwrap().to_string();
        let token = Token {
            kind,
            lexeme,
            line: self.line,
            column: self.column,
        };

        self.advance();

        token
    }

    /// Consume the next identifier from the source.
    ///
    /// This method assumes that the lexer's current character is the start of an identifier. If
    /// not, an exception is thrown.
    fn make_identifier(&mut self) -> Token {
        let Some(current) = self.peek() else {
            panic!("expected the start of an identifier");
        };

        if !Self::is_ident_start(current) {
            panic!("expected the start of an identifier");
        }

        let start = self.current;
        let column = self.column;
        let mut length = 1;

        self.advance();

        while let Some(current) = self.peek()
            && Self::is_ident(current)
        {
            length += 1;
            self.advance();
        }

        let lexeme = str::from_utf8(&self.source[start..start + length]).unwrap();
        let kind = check_keyword(lexeme);

        Token {
            kind,
            lexeme: lexeme.to_owned(),
            line: self.line,
            column,
        }
    }

    /// Consume the next number from the source.
    fn make_number(&mut self) -> Token {
        let Some(true) = self.peek().map(Self::is_digit) else {
            panic!("expected a digit");
        };

        let start = self.current;
        let column = self.column;
        let mut length = 1;

        self.advance();

        while let Some(current) = self.peek()
            && Self::is_digit(current)
        {
            length += 1;
            self.advance();
        }

        let lexeme = str::from_utf8(&self.source[start..start + length])
            .unwrap()
            .to_owned();

        Token {
            kind: TokenKind::LiteralInteger,
            lexeme,
            line: self.line,
            column: column,
        }
    }

    /// Extract the next token from the lexer.
    ///
    /// This method reads the next token from the source string. If the lexer has already read all
    /// of the tokens from the string (e.g. the source pointer is past the end of the string), then
    /// a null optional is returned.
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let current = self.peek()?;
        let token = match current {
            b'{' => self.make_token_and_advance(TokenKind::DelimBraceLeft),
            b'}' => self.make_token_and_advance(TokenKind::DelimBraceRight),
            b'(' => self.make_token_and_advance(TokenKind::DelimParenLeft),
            b')' => self.make_token_and_advance(TokenKind::DelimParenRight),
            b';' => self.make_token_and_advance(TokenKind::DelimSemicolon),
            b'!' => self.make_token_and_advance(TokenKind::OperatorBang),
            b'-' => self.make_token_and_advance(TokenKind::OperatorMinus),
            b'+' => self.make_token_and_advance(TokenKind::OperatorPlus),
            b'/' => self.make_token_and_advance(TokenKind::OperatorSlash),
            b'*' => self.make_token_and_advance(TokenKind::OperatorStar),
            b'~' => self.make_token_and_advance(TokenKind::OperatorTilde),
            _ => {
                if Self::is_ident_start(current) {
                    self.make_identifier()
                } else if Self::is_digit(current) {
                    self.make_number()
                } else {
                    self.make_token_and_advance(TokenKind::SpecialError)
                }
            }
        };

        Some(token)
    }
}
