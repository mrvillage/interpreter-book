#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl ToString) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}
