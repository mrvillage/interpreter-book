mod tests;

use crate::{
    BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
    Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, Lexer,
    PrefixExpression, Program, ReturnStatement, Statement, Token, TokenType,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut this = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
        };
        this.next_token();
        this.next_token();
        this
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        while let Some(token) = &self.cur_token {
            match token.token_type() {
                TokenType::Eof => break,
                _ => program.add_statement(self.parse_statement()?),
            }
            self.next_token();
        }
        Ok(program)
    }

    pub fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        match self.cur_token {
            Some(Token {
                token_type: TokenType::Let,
                ..
            }) => self.parse_let_statement(),
            Some(Token {
                token_type: TokenType::Return,
                ..
            }) => self.parse_return_statement(),
            Some(_) => self.parse_expression_statement(),
            _ => Err(format!("unexpected token: {:?}", self.cur_token)),
        }
    }

    pub fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.take().unwrap();
        if !self.expect_peek(TokenType::Ident) {
            return Err(format!(
                "expected next token to be Ident, got {:?} instead",
                self.peek_token
            ));
        }

        let ident = self.cur_token.take().unwrap();
        let name = ident.literal().to_string();
        let name = Identifier::new(ident, name);

        if !self.expect_peek(TokenType::Assign) {
            return Err(format!(
                "expected next token to be Assign, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Box::new(LetStatement::new(token, name, expr)))
    }

    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        match self.cur_token {
            Some(Token { token_type: t, .. }) => t == token_type,
            None => false,
        }
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        match self.peek_token {
            Some(Token { token_type: t, .. }) => t == token_type,
            None => false,
        }
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.take().unwrap();
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Box::new(ReturnStatement::new(token, expr)))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone().unwrap();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Box::new(ExpressionStatement::new(token, expr)))
    }

    pub fn prefix_parse_fn(
        &mut self,
        token_type: TokenType,
    ) -> Result<Box<dyn Expression>, String> {
        match token_type {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::True | TokenType::False => self.parse_boolean(),
            TokenType::Lparen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_literal(),
            _ => Err(format!("no prefix parse function for {:?}", token_type)),
        }
    }

    pub fn infix_parse_fn(
        &mut self,
        token_type: TokenType,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, String> {
        match token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Asterisk
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Gt => self.parse_infix_expression(left),
            TokenType::Lparen => self.parse_call_expression(left),
            _ => Err(format!("no infix parse function for {:?}", token_type)),
        }
    }

    pub fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.prefix_parse_fn(self.cur_token.as_ref().unwrap().token_type())?;
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let token_type = self.peek_token.as_ref().unwrap().token_type();
            self.next_token();
            expr = self.infix_parse_fn(token_type, expr)?;
        }
        Ok(expr)
    }

    pub fn parse_identifier(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let value = token.literal().to_string();
        Ok(Box::new(Identifier::new(token, value)))
    }

    pub fn parse_integer_literal(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let value = token.literal().parse::<i64>().unwrap();
        Ok(Box::new(IntegerLiteral::new(token, value)))
    }

    pub fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let operator = token.literal().to_string();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Box::new(PrefixExpression::new(token, operator, right)))
    }

    pub fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let operator = token.literal().to_string();
        let precedence = Self::get_precedence(token.token_type());
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Box::new(InfixExpression::new(token, left, operator, right)))
    }

    pub fn get_precedence(t: TokenType) -> Precedence {
        use TokenType::*;

        match t {
            Eq | NotEq => Precedence::Equals,
            Lt | Gt => Precedence::LessGreater,
            Plus | Minus => Precedence::Sum,
            Slash | Asterisk => Precedence::Product,
            Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Some(Token { token_type: t, .. }) => Self::get_precedence(t),
            None => Precedence::Lowest,
        }
    }

    pub fn cur_precedence(&self) -> Precedence {
        match self.cur_token {
            Some(Token { token_type: t, .. }) => Self::get_precedence(t),
            None => Precedence::Lowest,
        }
    }

    pub fn parse_boolean(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let value = match token.token_type() {
            TokenType::True => true,
            TokenType::False => false,
            _ => unreachable!(),
        };
        Ok(Box::new(Boolean::new(token, value)))
    }

    pub fn parse_grouped_expression(&mut self) -> Result<Box<dyn Expression>, String> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return Err(format!(
                "expected next token to be Rparen, got {:?} instead",
                self.peek_token
            ));
        }
        Ok(expr)
    }

    pub fn parse_if_expression(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        if !self.expect_peek(TokenType::Lparen) {
            return Err(format!(
                "expected next token to be Lparen, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return Err(format!(
                "expected next token to be Rparen, got {:?} instead",
                self.peek_token
            ));
        }
        if !self.expect_peek(TokenType::Lbrace) {
            return Err(format!(
                "expected next token to be Lbrace, got {:?} instead",
                self.peek_token
            ));
        }
        let consequence = self.parse_block_statement()?;
        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if !self.expect_peek(TokenType::Lbrace) {
                return Err(format!(
                    "expected next token to be Lbrace, got {:?} instead",
                    self.peek_token
                ));
            }
            Some(self.parse_block_statement()?)
        } else {
            None
        };
        Ok(Box::new(IfExpression::new(
            token,
            condition,
            consequence,
            alternative,
        )))
    }

    pub fn parse_block_statement(&mut self) -> Result<BlockStatement, String> {
        let token = self.cur_token.take().unwrap();
        let mut block = BlockStatement::new(token);
        self.next_token();
        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            block.add_statement(self.parse_statement()?);
            self.next_token();
        }
        Ok(block)
    }

    pub fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        if !self.expect_peek(TokenType::Lparen) {
            return Err(format!(
                "expected next token to be Lparen, got {:?} instead",
                self.peek_token
            ));
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::Lbrace) {
            return Err(format!(
                "expected next token to be Lbrace, got {:?} instead",
                self.peek_token
            ));
        }
        let body = self.parse_block_statement()?;
        Ok(Box::new(FunctionLiteral::new(token, parameters, body)))
    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, String> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Ok(identifiers);
        }
        self.next_token();

        let token = self.cur_token.take().unwrap();
        let literal = token.literal().to_string();
        let ident = Identifier::new(token, literal);
        identifiers.push(ident);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            let token = self.cur_token.take().unwrap();
            let literal = token.literal().to_string();
            let ident = Identifier::new(token, literal);
            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::Rparen) {
            return Err(format!(
                "expected next token to be Rparen, got {:?} instead",
                self.peek_token
            ));
        }
        Ok(identifiers)
    }

    pub fn parse_call_expression(
        &mut self,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, String> {
        let token = self.cur_token.take().unwrap();
        let arguments = self.parse_call_arguments()?;
        Ok(Box::new(CallExpression::new(token, left, arguments)))
    }

    pub fn parse_call_arguments(&mut self) -> Result<Vec<Box<dyn Expression>>, String> {
        let mut args = Vec::new();

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.peek_token_is(TokenType::Rparen) {
            return Err(format!(
                "expected next token to be Rparen, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        Ok(args)
    }
}
