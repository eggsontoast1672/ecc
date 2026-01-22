use crate::ast;
use crate::token::{Token, TokenKind};

/// An error that can be generated while parsing.
#[derive(Clone, Debug)]
pub struct ParseError {
    pub token: Option<Token>,
    pub message: String,
}

impl ParseError {
    /// Create a new parse error.
    fn new(token: Option<Token>, message: impl Into<String>) -> Self {
        Self {
            token,
            message: message.into(),
        }
    }

    /// Create a new parse error at the specified token.
    fn at_token(token: Token, message: impl Into<String>) -> Self {
        Self::new(Some(token), message)
    }

    /// Create a new end of file parse error.
    fn end_of_file(message: impl Into<String>) -> Self {
        Self::new(None, message)
    }
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

/// A level of operator precedence.
///
/// The order of the members in this enum is very important. The members lower down are the
/// precedences that bind the tightest. For example, [`Precedence::Prefix`] is lower than
/// [`Precedence::Product`] since unary (prefix) operators bind more tightly than multiplication
/// and division.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Precedence {
    Lowest,
    Sum,
    Product,
    Prefix,
}

fn get_prefix_precedence(kind: TokenKind) -> Precedence {
    match kind {
        TokenKind::OperatorBang => Precedence::Prefix,
        TokenKind::OperatorMinus => Precedence::Prefix,
        TokenKind::OperatorTilde => Precedence::Prefix,
        _ => Precedence::Lowest,
    }
}

fn get_infix_precedence(kind: TokenKind) -> Precedence {
    match kind {
        TokenKind::OperatorPlus => Precedence::Sum,
        TokenKind::OperatorMinus => Precedence::Sum,
        TokenKind::OperatorStar => Precedence::Product,
        TokenKind::OperatorSlash => Precedence::Product,
        _ => Precedence::Lowest,
    }
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

    /// Advance the parser and return the next token.
    ///
    /// If the parser has reached the end of the token stream, [`None`] is returned.
    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).cloned()?;
        self.current += 1;
        Some(token)
    }

    /// Advance the lexer if the pattern matches.
    ///
    /// This helper macro checks the kind of the token that the lexer is currently pointing to. If the
    /// kind matches the given pattern, then an [`Ok`] variant containing the peeked token is returned.
    /// If the pattern did not match, an [`Err`] variant is returned.
    fn advance_expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        let message = format!("expected {kind}");

        let Some(token) = self.peek() else {
            return Err(ParseError::end_of_file(message));
        };

        if token.kind != kind {
            return Err(ParseError::at_token(token.clone(), message));
        }

        Ok(self.advance().unwrap())
    }

    /// Get the next token, or an error if there is none.
    fn advance_expect_anything(&mut self, message: impl Into<String>) -> ParseResult<Token> {
        self.advance().ok_or(ParseError::end_of_file(message))
    }

    /// Get the token the parser is currently pointing to.
    ///
    /// If the parser has reached the end of the token stream and is pointing to nothing, a null
    /// optional is returned.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_expect_anything(&self, message: String) -> ParseResult<&Token> {
        self.peek().ok_or(ParseError::end_of_file(message))
    }

    /// Parse a program.
    ///
    /// This method will parse a program (a single function declaration). After that, it asserts
    /// that there are no more tokens to be processed. If there are, an exception is thrown.
    fn parse_program(&mut self) -> ParseResult<ast::Program> {
        let function = self.parse_function()?;
        if let Some(token) = self.peek() {
            Err(ParseError::at_token(token.clone(), "expected end of file"))
        } else {
            Ok(ast::Program { function })
        }
    }

    /// Parse a function declaration.
    ///
    /// This method parses the return type, function name, parameter list, and body of a function.
    fn parse_function(&mut self) -> ParseResult<ast::Function> {
        self.advance_expect(TokenKind::KeywordInt)?;

        let name = self.parse_identifier()?;

        self.advance_expect(TokenKind::DelimParenLeft)?;
        self.advance_expect(TokenKind::KeywordVoid)?;
        self.advance_expect(TokenKind::DelimParenRight)?;
        self.advance_expect(TokenKind::DelimBraceLeft)?;

        let return_statement = self.parse_statement()?;

        self.advance_expect(TokenKind::DelimBraceRight)?;

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
            _ => Err(ParseError::new(token.cloned(), "expected statement")),
        }
    }

    /// Parse the next return statement.
    ///
    /// This method expects a return keyword followed by an expression and then a semicolon.
    fn parse_return(&mut self) -> ParseResult<ast::Statement> {
        self.advance_expect(TokenKind::KeywordReturn)?;
        let return_value = self.parse_expression(Precedence::Lowest)?;
        self.advance_expect(TokenKind::DelimSemicolon)?;
        Ok(ast::Statement::Return(return_value))
    }

    /// Parse the next expression.
    ///
    /// This method looks at the next token in the stream and decides based on that what kind of
    /// expression to parse. In the future, this method may take advantage of Pratt parsing.
    fn parse_expression(&mut self, prec: Precedence) -> ParseResult<ast::Expr> {
        let token = self.peek_expect_anything("expected expression".to_string())?;
        let mut left = self.parse_prefix(token.clone())?;

        while let Some(token) = self.peek()
            && prec < get_infix_precedence(token.kind)
        {
            left = self.parse_infix(token.clone(), left)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self, token: Token) -> ParseResult<ast::Expr> {
        match token.kind {
            TokenKind::DelimParenLeft => self.parse_group(),
            TokenKind::LiteralIdentifier => todo!(),
            TokenKind::LiteralInteger => self.parse_integer(),
            TokenKind::OperatorBang => self.parse_unary(ast::UnaryOp::NegateLogical),
            TokenKind::OperatorMinus => self.parse_unary(ast::UnaryOp::NegateArith),
            TokenKind::OperatorTilde => self.parse_unary(ast::UnaryOp::Compliment),
            _ => Err(ParseError::at_token(token, "expected prefix operator")),
        }
    }

    /// Parse an infix expression.
    ///
    /// The `kind` is the kind of token that the parser is currently looking at. The `left` is the
    /// portion of the expression that has been parsed so far, e.g. the left half of the binary
    /// operation.
    fn parse_infix(&mut self, token: Token, left: ast::Expr) -> ParseResult<ast::Expr> {
        match token.kind {
            TokenKind::OperatorMinus => self.parse_binary(ast::BinaryOp::Minus, left),
            TokenKind::OperatorPlus => self.parse_binary(ast::BinaryOp::Plus, left),
            TokenKind::OperatorSlash => self.parse_binary(ast::BinaryOp::Divide, left),
            TokenKind::OperatorStar => self.parse_binary(ast::BinaryOp::Times, left),
            _ => Err(ParseError::at_token(token, "expected infix operator")),
        }
    }

    /// Parse the next unary expression.
    ///
    /// This method parses a unary expression with the given operator. The next token is skipped
    /// (it is assumed to correspond to the operator passed) and an expression is parsed. From the
    /// operator and the parsed expression, a new unary expression is constructed.
    fn parse_unary(&mut self, op: ast::UnaryOp) -> ParseResult<ast::Expr> {
        let token = self.advance_expect_anything("expected unary operator")?;
        let prec = get_prefix_precedence(token.kind);
        let operand = self.parse_expression(prec)?;

        Ok(ast::Expr::Unary {
            operator: op,
            operand: Box::new(operand),
        })
    }

    /// Parse the next binary expression.
    ///
    /// This method recieves the binary operation that is currently being parsed as well as the
    /// left hand side of the expression. It assumes that the parser is currently pointing to a
    /// binary operator token which corresponds to the given `op`.
    fn parse_binary(&mut self, op: ast::BinaryOp, left: ast::Expr) -> ParseResult<ast::Expr> {
        let token = self.advance_expect_anything("expected binary operator")?;
        let prec = get_infix_precedence(token.kind);
        let right = self.parse_expression(prec)?;

        Ok(ast::Expr::Binary {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Parse the next group expression.
    ///
    /// This method parses an opening parenthesis, followed by an expression with reset precedence,
    /// and then a closing parenthesis. This has the effect of considering the parenthesized
    /// expression as a single unit.
    fn parse_group(&mut self) -> ParseResult<ast::Expr> {
        self.advance_expect(TokenKind::DelimParenLeft)?;
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.advance_expect(TokenKind::DelimParenRight)?;
        Ok(expr)
    }

    /// Parse the next identifier.
    ///
    /// This method expects an identifier token.
    fn parse_identifier(&mut self) -> ParseResult<String> {
        let ident = self.advance_expect(TokenKind::LiteralIdentifier)?;
        Ok(ident.lexeme.clone())
    }

    /// Parse the next integer literal.
    fn parse_integer(&mut self) -> ParseResult<ast::Expr> {
        let integer = self.advance_expect(TokenKind::LiteralInteger)?;
        let value: i32 = integer.lexeme.parse().unwrap();
        Ok(ast::Expr::Integer(value))
    }
}
