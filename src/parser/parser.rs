use crate::error_token;
use crate::ast::node::*;
use crate::token::token::Token;

// TODO: Add error handling with Result<Box<Expr>, ParseError> type
// Link: https://stackoverflow.com/questions/55755552/what-is-the-rust-equivalent-to-a-try-catch-statement/55758013#:~:text=There%20is%20no%20try%20catch%20statement%20in%20Rust.
// TODO: Unwind the stack if there is an error, and call synchronize()

pub struct Parser {
    tokens: Vec<Token>,
    // points to the next token to be parsed
    current: usize,
}

impl Parser {
    struct ParseError {}

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    // TODO: Implement parse method
    pub fn parse(&mut self) -> Box<Expr> {
    }


    fn match_token(&self, tokens_types: &[TokenType]) -> bool {
        for token_type in tokens_types {
            if check(token_type) {
                advance();
                true
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if is_at_end() {
            false
        } 
        peek().token_type == token_type.token_type
    }

    fn advance(&self) -> Token {
        if !is_at_end() {
            self.current += 1;
        }

        previous()
    }

    fn is_at_end(&self) -> bool {
        peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // most recently consumed token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }


    fn consume(&self, &token_type: TokenType, message: &str) -> Token {
        if check(token_type) {
            advance()
        } else {
            self.error(peek(), message);
        }
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        error_token(token, message);
        ParseError {};
    }

    fn synchronize() {
        advance();

        while !is_at_end() {
            match previous().token_type {
                TokenType::SEMICOLON => {
                    return;
                }
                _ => {
                    match peek().token_type {
                        TokenType::CLASS |
                        TokenType::FUN |
                        TokenType::VAR |
                        TokenType::FOR |
                        TokenType::IF |
                        TokenType::WHILE |
                        TokenType::PRINT |
                        TokenType::RETURN => return,
                        _ => advance(),
                    }
                }
            }
        }
    }

    // methods for parsing productions
    fn expression(&self) -> Box<Expr> {
        equality()
    }

    fn equality(&self) -> Box<Expr> {
        let mut expr = comparison();

        while match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = previous();
            let right = comparison();
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        expr
    }

    fn comparison(&self) -> Box<Expr> {
        let mut expr = term();

        while match_token(&[TokenType::GREATER,
                            TokenType::GREATER_EQUAL,
                            TokenType::LESS,
                            TokenType::LESS_EQUAL]) {
            let operator = previous();
            let right = term();
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        expr
    }

    fn term(&self) -> Box<Expr> {
        let mut expr = factor();

        while match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = previous();
            let right = factor();
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        expr
    }

    fn factor(&self) -> Box<Expr> {
        let mut expr = unary();

        while match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = previous();
            let right = unary();
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        expr
    }

    fn unary(&self) -> Box<Expr> {
        if match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = previous();
            let right = unary();
            Box::new(Expr::Unary(Unary { operator, right }));
        }

        primary() 
    }

    fn primary(&self) -> Box<Expr> {
        if match_token(&[TokenType::FALSE,
                         TokenType::TRUE,
                         TokenType::NIL,
                         TokenType::NUMBER,
                         TokenType::STRING]) {
            Box::new(Expr::Literal(Literal { value: previous().clone() }))
        }

        if match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            expr
        }
        error(self.peek(), "Expect expression.");
    }
}
