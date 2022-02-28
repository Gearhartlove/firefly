use std::fmt;
use std::fmt::Formatter;

pub struct Token<T> {
    token_type: TokenType,
    lexeme: String,
    literal: T,
    line: u16,
}

impl Token<T> {
    pub fn new(token_type: TokenType, lexeme: String, literal: T, line: u16) -> Self {
        Self { token_type, lexeme, literal, line}
    }
}

impl ToString for Token<T> {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
