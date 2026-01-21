use crate::ast;
use crate::token::{Token, TokenKind};

/// An error that can be generated while parsing.
#[derive(Clone, Debug)]
pub struct ParseError {
    pub token: Option<Token>,
    pub message: &'static str,
}

/// A simple type alias for a [`Result`] whose [`Err`] variant contains a [`ParseError`].
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse a stream of tokens into a program.
pub fn parse_token_stream<T>(stream: T) -> ParseResult<ast::Program>
where
    T: IntoIterator<Item = Token>,
{
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut parser = Parser::new(tokens);

    parser.parse_program()
}

/// Advance the lexer if the pattern matches.
///
/// This helper macro checks the kind of the token that the lexer is currently pointing to. If the
/// kind matches the given pattern, then an [`Ok`] variant containing the peeked token is returned.
/// If the pattern did not match, an [`Err`] variant is returned.
macro_rules! advance_expect {
    ($lexer:expr, $kind:pat) => {
        match $lexer.peek() {
            Some(token @ Token { kind: $kind, .. }) => Ok(token),
            token => Err(ParseError {
                token: token.cloned(),
                message: "expected something different",
            }),
        }
    };
}

/// The parser.
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Get the token the parser is currently pointing to.
    ///
    /// If the parser has reached the end of the token stream and is pointing to nothing, a null
    /// optional is returned.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Parse a program.
    ///
    /// This method will parse a program (a single function declaration). After that, it asserts
    /// that there are no more tokens to be processed. If there are, an exception is thrown.
    fn parse_program(&mut self) -> ParseResult<ast::Program> {
        let function = self.parse_function()?;
        if let Some(token) = self.peek() {
            Err(ParseError {
                token: Some(token.clone()),
                message: "expected end of file",
            })
        } else {
            Ok(ast::Program { function })
        }
    }

    /// Parse a function declaration.
    ///
    /// This method parses the return type, function name, parameter list, and body of a function.
    fn parse_function(&mut self) -> ParseResult<ast::Function> {
        advance_expect!(self, TokenKind::KeywordInt)?;

        let name = self.parse_identifier()?;

        advance_expect!(self, TokenKind::SymbolParenLeft)?;
        advance_expect!(self, TokenKind::KeywordVoid)?;
        advance_expect!(self, TokenKind::SymbolParenRight)?;
        advance_expect!(self, TokenKind::SymbolBraceLeft)?;

        let return_statement = self.parse_statement()?;

        advance_expect!(self, TokenKind::SymbolBraceRight)?;

        Ok(ast::Function {
            name,
            body: vec![return_statement],
        })
    }

    /// Parse the next statement.
    ///
    /// This method looks at the next token in the stream and decides based on that what kind of
    /// statement to parse.
    fn parse_statement(&mut self) -> ParseResult<ast::Statement> {
        let token = self.peek();
        match token.map(|t| t.kind) {
            Some(TokenKind::KeywordReturn) => self.parse_return(),
            _ => Err(ParseError {
                token: token.cloned(),
                message: "expected statement",
            }),
        }
    }

    /// Parse the next return statement.
    ///
    /// This method expects a return keyword followed by an expression and then a semicolon.
    fn parse_return(&mut self) -> ParseResult<ast::Statement> {
        advance_expect!(self, TokenKind::KeywordReturn)?;
        let return_value = self.parse_expression()?;
        advance_expect!(self, TokenKind::SymbolSemicolon)?;
        Ok(ast::Statement::Return(return_value))
    }

    /// Parse the next expression.
    ///
    /// This method looks at the next token in the stream and decides based on that what kind of
    /// expression to parse. In the future, this method may take advantage of Pratt parsing.
    fn parse_expression(&mut self) -> ParseResult<ast::Expression> {
        let token = self.peek();
        match token.map(|t| t.kind) {
            Some(TokenKind::LiteralIdentifier) => todo!(),
            Some(TokenKind::LiteralInteger) => self.parse_integer(),
            _ => Err(ParseError {
                token: token.cloned(),
                message: "expected expression",
            }),
        }
    }

    /// Parse the next identifier.
    ///
    /// This method expects an identifier token.
    fn parse_identifier(&mut self) -> ParseResult<String> {
        let ident = advance_expect!(self, TokenKind::LiteralIdentifier)?;
        Ok(ident.lexeme.clone())
    }

    /// Parse the next integer literal.
    fn parse_integer(&mut self) -> ParseResult<ast::Expression> {
        let integer = advance_expect!(self, TokenKind::LiteralInteger)?;
        let value: i32 = integer.lexeme.parse().unwrap();
        Ok(ast::Expression::Integer(value))
    }
}
