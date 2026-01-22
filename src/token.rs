/// The kind of a token.
///
/// This enum represents the kind associated with a token. While the lexer separates the source
/// code into tokens, it also assigns a kind to the lexeme so that the parser can check at a glance
/// what kind of token it is looking at.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq,Debug)]
pub enum TokenKind {
    DelimBraceLeft,
    DelimBraceRight,
    DelimParenLeft,
    DelimParenRight,
    DelimSemicolon,

    KeywordInt,
    KeywordReturn,
    KeywordVoid,

    LiteralIdentifier,
    LiteralInteger,

    OperatorBang,
    OperatorMinus,
    OperatorPlus,
    OperatorSlash,
    OperatorStar,
    OperatorTilde,

    SpecialError,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DelimBraceLeft => write!(f, "'{{'"),
            Self::DelimBraceRight => write!(f, "'}}'"),
            Self::DelimParenLeft => write!(f, "'('"),
            Self::DelimParenRight => write!(f, "')'"),
            Self::DelimSemicolon => write!(f, "';'"),

            Self::KeywordInt => write!(f, "'int'"),
            Self::KeywordReturn => write!(f, "'return'"),
            Self::KeywordVoid => write!(f, "'void'"),

            Self::LiteralIdentifier => write!(f, "identifier"),
            Self::LiteralInteger => write!(f, "integer literal"),

            Self::OperatorBang => write!(f, "'!'"),
            Self::OperatorMinus => write!(f, "'-'"),
            Self::OperatorPlus => write!(f, "'+'"),
            Self::OperatorSlash => write!(f, "'/'"),
            Self::OperatorStar => write!(f, "'*'"),
            Self::OperatorTilde => write!(f, "'~'"),

            Self::SpecialError => write!(f, "error token"),
        }
    }
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
