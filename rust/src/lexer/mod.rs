mod tests;
mod token;

pub use token::*;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut this = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        this.read_char();
        this
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position..].chars().next().unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            None => Token::new(TokenType::Eof, "".to_string()),
            Some(ch) => match ch {
                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Token::new(TokenType::Eq, "==".to_string())
                    } else {
                        Token::new(TokenType::Assign, ch.to_string())
                    }
                },
                '+' => Token::new(TokenType::Plus, ch.to_string()),
                '-' => Token::new(TokenType::Minus, ch.to_string()),
                '!' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Token::new(TokenType::NotEq, "!=".to_string())
                    } else {
                        Token::new(TokenType::Bang, ch.to_string())
                    }
                },
                '/' => Token::new(TokenType::Slash, ch.to_string()),
                '*' => Token::new(TokenType::Asterisk, ch.to_string()),
                '<' => Token::new(TokenType::Lt, ch.to_string()),
                '>' => Token::new(TokenType::Gt, ch.to_string()),
                ';' => Token::new(TokenType::Semicolon, ch.to_string()),
                '(' => Token::new(TokenType::Lparen, ch.to_string()),
                ')' => Token::new(TokenType::Rparen, ch.to_string()),
                ',' => Token::new(TokenType::Comma, ch.to_string()),
                '{' => Token::new(TokenType::Lbrace, ch.to_string()),
                '}' => Token::new(TokenType::Rbrace, ch.to_string()),
                'a'..='z' | 'A'..='Z' | '_' => {
                    return match self.read_ident_literal() {
                        "fn" => Token::new(TokenType::Function, "fn"),
                        "let" => Token::new(TokenType::Let, "let"),
                        "true" => Token::new(TokenType::True, "true"),
                        "false" => Token::new(TokenType::False, "false"),
                        "if" => Token::new(TokenType::If, "if"),
                        "else" => Token::new(TokenType::Else, "else"),
                        "return" => Token::new(TokenType::Return, "return"),
                        ident => Token::new(TokenType::Ident, ident),
                    }
                },
                ' ' | '\t' | '\n' | '\r' => {
                    self.read_char();
                    return self.next_token();
                },
                '0'..='9' => return Token::new(TokenType::Int, self.read_int_literal()),
                _ => Token::new(TokenType::Illegal, ch.to_string()),
            },
        };
        self.read_char();
        tok
    }

    pub fn read_ident_literal(&mut self) -> &str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if ch.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        &self.input[position..self.position]
    }

    pub fn read_int_literal(&mut self) -> &str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if ch.is_numeric() {
                self.read_char();
            } else {
                break;
            }
        }
        &self.input[position..self.position]
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position..].chars().next().unwrap())
        }
    }
}
