use crate::token::Token;

pub struct Tokenizer {
    source: String,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        // todo parse in the constructor
        Self { source }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {}
}
