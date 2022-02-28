use crate::token::Token;
use crate::token::TokenType::EOF;

pub struct Tokenizer {
    start: u16,
    end: u16,
    source: String,
    tokens: Vec<Token<T>>,
}

impl Tokenizer {
    /// Instantiate a Tokenizer Object with source code, then tokenize the source
    pub fn new(source: String) -> Self {
        // todo parse in the constructor
        let tokens = tokenize(&source);
        Self { source, tokens }
    }

    fn tokenize(&self, source: &String) -> Vec<Token<T>> {
        loop {
            if !isAtEnd() {
                start = current;
                scanToken();
            }
        }

        self.tokens.append(Token::new(EOF, String::new(""), null(), line));
    }
}

impl Iterator for Tokenizer {
    type Item = Token<T>;

    /// iterate over the
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
