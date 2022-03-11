use crate::error;
use crate::token::token::Token;
use crate::token::token_type::{ TokenType, Literal, get_token_type };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token { 
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            line: self.line 
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.reserved_words();
                }
                else {
                    error(self.line, "Unexpected character.".to_string());
                }
            }
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        if let Some(c) = self.source.chars().nth(self.current as usize - 1) {
            c
        } else {
            eprintln!("Error: Reached end of source but advance was called.");
            '\0'
        }
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source[self.start as usize..self.current as usize]
            .to_string();

        self.tokens.push(Token{
            token_type: token,
            lexeme: text,
            line: self.line
        });
    }

    fn add_token_literal(&mut self, token: TokenType) {
        let mut text = self.source[self.start as usize..self.current as usize]
            .to_string();

        if let TokenType::LITERAL(Literal::STRING(_)) = token {
            text.remove(0);
            text.remove(text.len() - 1);
        }

        self.tokens.push(Token{
            token_type: token,
            lexeme: text,
            line: self.line
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn is_alpha(&self, c: char) -> bool {
        ('a'..='z').contains(&c)|| ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
           return false;
        }

        let c = self.source.chars().nth(self.current as usize).unwrap();
        if c != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        // unwrapping assuming above statement is true
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    // scanner just look ahead atmost one character
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }

        // unwrapping as above statement is true
        return self.source.chars().nth((self.current + 1) as usize).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string".to_string());
            return;
        }

        // The closin "
        self.advance();

        let value = self.source[
            self.start as usize + 1..self.current as usize - 1
        ].to_string();

        self.add_token_literal(TokenType::LITERAL(Literal::STRING(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        // assuming that the number is valid~ that's why using unwrap()
        // without error handling
        let value = self.source[
            self.start as usize..self.current as usize
        ].parse::<f64>().unwrap();

        self.add_token_literal(TokenType::LITERAL(Literal::NUMBER(value)));
    }

    fn reserved_words(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text =
            self.source[self.start as usize..self.current as usize]
            .to_string();

        self.add_token(get_token_type(text));
    }
}

