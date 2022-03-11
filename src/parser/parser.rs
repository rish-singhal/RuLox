use crate::error_token;
use crate::ast::node::*;
use crate::token::token::Token;
use crate::token::token_type;
use crate::token::token_type::TokenType;

// TODO: Check style guide for rust

struct ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    // points to the next token to be parsed
    current: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Ok(decl) = self.declaration() {
                statements.push(decl);
            }
        }
        Some(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::VAR]) {
            match self.var_declaration() {
                Ok(v) => return Ok(v),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            };
        };

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = match &self.peek().token_type {
            TokenType::LITERAL(token_type::Literal::IDENTIFIER(_))
            => Ok(self.advance().clone()),
            _ => Err(self.error(self.peek(), "Expect variable name."))
        };

        let name = name?;

        let mut initializer = None;
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(&TokenType::SEMICOLON, "Expected ';' after variable \
                                                declaration.")?;
        Ok(Stmt::Var(Var {name, initializer}))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError>  {
        let expression = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expected ';' after value.")?;
        Ok(Stmt::Print(Print { expression }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.expression()?;
        self.consume(&TokenType::SEMICOLON, "Expected ';' after expression.")?;
        Ok(Stmt::Expression(Expression { expression }))

    }

    fn match_token(&mut self, tokens_types: &[TokenType]) -> bool {
        for token_type in tokens_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, _token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *_token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // most recently consumed token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }


    fn consume(&mut self,
               token_type: &TokenType,
               message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
           return Err(self.error(self.peek(), message));
        }
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        error_token(token, message.to_string());
        ParseError {}
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::SEMICOLON => {
                    return;
                }
                _ => {
                    match self.peek().token_type {
                        TokenType::CLASS |
                        TokenType::FUN |
                        TokenType::VAR |
                        TokenType::FOR |
                        TokenType::IF |
                        TokenType::WHILE |
                        TokenType::PRINT |
                        TokenType::RETURN => return,
                        _ => self.advance(),
                    };
                }
            }
        }
    }

    // methods for parsing productions
    fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        self.assingment()
    }

    fn assingment(&mut self) -> Result<Box<Expr>, ParseError> {
        let expr = self.equality()?;

        if self.match_token(&[TokenType::EQUAL]) {
            let equals = self.previous().clone();
            let value = self.assingment()?;

            if let Expr::Variable(v) = *expr {
                Ok(Box::new(Expr::Assign(
                    Assign {
                        name: v.name,
                        value,
                    }
                )))
            } else {
                Err(self.error(&equals, "Invalid assignment type."))
            }
        } else {
            Ok(expr)
        }
    }


    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual,
                                 TokenType::EqualEqual]) {
            // https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenType::GREATER,
                            TokenType::GreaterEqual,
                            TokenType::LESS,
                            TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(
                Binary {
                    left: expr,
                    operator,
                    right,
                }
            ));
        }

        Ok(expr)
    }

    fn unary(&mut self) ->  Result<Box<Expr>, ParseError> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(
                        Unary {
                            operator,
                            right
                        })
                    )
                );
        }

        self.primary() 
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(&[TokenType::FALSE,
                              TokenType::TRUE,
                              TokenType::NIL,
                             ]) {
            return Ok(Box::new(Expr::Literal(
                    Literal { value: self.previous().clone() }
                    )
                ));
        }

        match &self.peek().token_type {
            TokenType::LITERAL(token_type::Literal::NUMBER(_))
            | TokenType::LITERAL(token_type::Literal::STRING(_))
            => {
                self.advance();
                Ok(Box::new(Expr::Literal(
                    Literal { value: self.previous().clone() }
                )))
            },
            TokenType::LITERAL(token_type::Literal::IDENTIFIER(_))
            => {
                self.advance();
                Ok(Box::new(Expr::Variable(
                    Variable { name: self.previous().clone() }
                )))
            }
            _ => {

                if self.match_token(&[TokenType::LeftParen]) {
                    let expr = self.expression()?;
                    self.consume(&TokenType::RightParen,
                                 "Expect ')' after expression.")?;
                    return Ok(expr);
                }

                Err(self.error(self.peek(), "Expect expression."))
            }
        }
    }
}

