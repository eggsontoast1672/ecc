/// The kind of a token.
///
/// This enum represents the kind associated with a token. While the lexer separates the source
/// code into tokens, it also assigns a kind to the lexeme so that the parser can check at a glance
/// what kind of token it is looking at.
#[allow(missing_docs)]
#[derive(Clone, Debug, Copy)]
pub enum TokenKind {
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
}

/// Check if the given lexeme is a keyword.
///
/// If the lexeme is a keyword, the returned token type will indicate which one it is. Otherwise,
/// the returned token type is [`TokenKind::LiteralIdentifier`].
pub fn check_keyword(lexeme: &str) -> TokenKind {
    match lexeme {
        "int" => TokenKind::KeywordInt,
        "return" => TokenKind::KeywordReturn,
        "void" => TokenKind::KeywordVoid,
        _ => TokenKind::LiteralIdentifier,
    }
}

/// A source code token.
///
/// Tokens are the smallest unit of lexical information. They are analogous to words in spoken
/// language. A token contains its kind, the corresponding substring of the source code (the
/// lexeme), and the line and column info.
#[derive(Clone, Debug)]
pub struct Token {
    /// The kind of token this is. This information is helpful for the parser.
    pub kind: TokenKind,

    /// The corresponding string in the source code from which this token came.
    pub lexeme: String,

    /// The line of the source code that this token was on.
    pub line: usize,

    /// The column of the source code that this token was on.
    pub column: usize,
}
