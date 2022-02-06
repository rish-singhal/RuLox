use crate::error;
use crate::token::token::Token;
use crate::token::token_type::{ TokenType, Literal };

struct Scanner {
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

    pub fn scan_tokens(&self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();    
        }
        self.tokens.push(Token{ 
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line 
        });
        return self.tokens;
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
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
                } else {
                    error(self.line, "Unexpected character.".to_string())
                }
            }
        };
    }

    fn advance(&self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current as usize - 1).unwrap();
    }

    fn add_token(&self, token: TokenType) {
        let text = self.source[self.start as usize..self.current as usize]
            .to_string();

        self.tokens.push(Token{
            token_type: token,
            lexeme: text,
            literal: None,
            line: self.line
        });
    }

    fn add_token_literal(&self, token: TokenType, literal: Literal) {
        let text = self.source[self.start as usize..self.current as usize]
            .to_string();

        self.tokens.push(Token{
            token_type: token,
            lexeme: text,
            literal: Some(literal),
            line: self.line
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn match_char(&self, expected: char) -> bool {
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

        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }

        return self.source.chars().nth((self.current + 1) as usize).unwrap();
    }

    // TODO: Implement string in Literal enum 
    fn string(&self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string".to_string());
        }

        // The closin "
        self.advance();

        let value = self.source[
            self.start as usize + 1..self.current as usize - 1
        ].to_string();
        self.add_token_literal(TokenType::STRING, literal: value);
    }

    fn number(&self) {

    }
}

