use crate::ast;
use crate::token::{Token, TokenKind};

/// An error that can be generated while parsing.
#[derive(Debug)]
pub enum ParseError {
    ExpectedDifferentToken,
    ExpectedEndOfFile,
    General(&'static str),
}

/// A simple type alias for a [`Result`] whose [`Err`] variant contains a [`ParseError`].
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse a stream of tokens into a program.
pub fn parse_token_stream<I>(stream: I) -> ParseResult<ast::Program>
where
    I: Iterator<Item = Token>,
{
    let tokens: Vec<_> = stream.collect();
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
            _ => Err(ParseError::ExpectedDifferentToken),
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

    /// Return true if there are more tokens to be processed (e.g. `get_current`
    /// would not return a null optional), and false otherwise.
    fn has_more_tokens(&self) -> bool {
        self.current < self.tokens.len()
    }

    /// Get the token the parser is currently pointing to.
    ///
    /// If the parser has reached the end of the token stream and is pointing to
    /// nothing, a null optional is returned.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Parse a program.
    ///
    /// This method will parse a program (a single function declaration). After
    /// that, it asserts that there are no more tokens to be processed. If there
    /// are, an exception is thrown.
    fn parse_program(&mut self) -> ParseResult<ast::Program> {
        let function = self.parse_function()?;
        if self.has_more_tokens() {
            Err(ParseError::ExpectedEndOfFile)
        } else {
            Ok(ast::Program { function })
        }
    }

    /// Parse a function declaration.
    ///
    /// This method parses the return type, function name, parameter list, and
    /// body of a function.
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
        match self.peek().map(|t| t.kind) {
            Some(TokenKind::KeywordReturn) => self.parse_return(),
            _ => Err(ParseError::General("expected statement")),
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
        match self.peek().map(|t| t.kind) {
            Some(TokenKind::LiteralIdentifier) => todo!(),
            Some(TokenKind::LiteralInteger) => self.parse_integer(),
            _ => Err(ParseError::General("expected expression")),
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
