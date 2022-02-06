use super::token_type::*;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

// TODO: Implement str version of token_type
impl Token {
    pub fn to_string(&self) -> String {
        return format!("{} {} {}", self.token_type, self.lexeme, self.literal)
            .to_string();
    }
}
