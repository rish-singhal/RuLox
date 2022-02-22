#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    LITERAL(Literal), 

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::LITERAL(literal) => write!(f, "{:?}", literal),
            _ => write!(f, "{:?}", self)
        }
    }
}

pub fn get_token_type(literal: String) -> TokenType {
    match &literal[..] {
        "and" => TokenType::AND,
        "class" =>TokenType::CLASS,
        "else" => TokenType::ELSE,
        "false" => TokenType::FALSE,
        "fun" => TokenType::FUN,
        "for" => TokenType::FOR,
        "if" => TokenType::IF,
        "nil" => TokenType::NIL,
        "or" => TokenType::OR,
        "print" => TokenType::PRINT,
        "return" => TokenType::RETURN,
        "super" => TokenType::SUPER,
        "this" => TokenType::THIS,
        "true" => TokenType::TRUE,
        "var" => TokenType::VAR,
        "while" => TokenType::WHILE,
        // assuming IDENTIFIER is returned as this function is called only
        // for identifier & keyword segregation
        _ => TokenType::LITERAL(Literal::IDENTIFIER(literal.to_string()))
    }
}

