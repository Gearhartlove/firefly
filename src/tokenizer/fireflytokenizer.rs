use crate::token::{Token, TokenType};
use crate::token::TokenType::*;

pub struct Tokenizer<F> {
    start: u16,
    current: F,
    line : u16,
    source: String,
    tokens: Vec<Token<T>>,
}

impl Tokenizer<F> {
    /// Instantiate a Tokenizer Object with source code, then tokenize the source
    pub fn new(source: String) -> Self {
        // todo parse in the constructor
        let tokens = tokenize(&source);
        let mut current: u16 = 0;
        // test to see if this works?
        let get_current = | | -> u16 {
            value = current;
            current += 1;
            value
        };
        Self { source, tokens, start: 0, current: get_current, line: 1 }
    }

    fn tokenize(&mut self) {
        loop {
            if !isAtEnd() {
                start = self.current;
                scanTokens()
            }
            break;
        }
        // append EOF token (done reading the code)
        self.tokens.push(Token::new(EOF, String::new(),
                                    None, line));
    }

    /// Scan the next token and match depending on the next accepted lexeme
    fn scanToken(&mut self) {
        let c = advance();
        match c {
            '(' => self.addToken(LEFT_PAREN, None),
            ')' => self.addToken(RIGHT_PAREN, None),
            '{' => self.addToken(LEFT_BRACE, None),
            '}' => self.addToken(RIGHT_BRACE, None),
            ',' => self.addToken(COMMA, None),
            '.' => self.addToken(DOT, None),
            '-' => self.addToken(MINUS, None),
            '+' => self.addToken(PLUS, None),
            ';' => self.addToken(SEMICOLON, None),
            '*' => self.addToken(STAR, None),
            '!' => {
                self.addToken(check())
            }
            _ => println!("token not recognized?") // need to implement error still . . .
        }
    }

    // Helper Methods below
    /// Returns true if at the end of the file while tokenizing
    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }
    /// Consumes the next character in the source file and returns it
    fn advance(&mut self) -> char {
        // store current as a closure
        // is current going to increment how I think?
        self.source.char_indices().nth(self.current() as usize).unwrap().1
    }
    /// Grabs the text of the current lexeme and creates a new token for it
    fn addToken<T>(&mut self, token_type: TokenType, literal: Option<T>) {
        // possible error with range below
        let text = self.source.get(self.start..self.current);
        match literal {
            Some(L) => {
                self.tokens.push(Loken::new(token_type, text, L, self.line))
            },
            None => {
                self.tokens.push(Loken::new(token_type, text, None, self.line))
            },
        }
    }
}

impl Iterator for Tokenizer<F> {
    type Item = Token<T>;

    /// iterate over the
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
