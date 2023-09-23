use std::fmt::Display;

use crate::{Token, TokenType};

pub trait Node: std::fmt::Debug + std::fmt::Display {
    fn node_type(&self) -> NodeType;

    fn token_literal(&self) -> &str;

    fn token_type(&self) -> TokenType;

    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Program,
    LetStatement,
    Identifier,
    ReturnStatement,
    ExpressionStatement,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    Boolean,
    IfExpression,
    BlockStatement,
    FunctionLiteral,
    CallExpression,
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }

    pub fn add_statement(&mut self, stmt: Box<dyn Statement>) {
        self.statements.push(stmt);
    }

    pub fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

impl Node for Program {
    fn node_type(&self) -> NodeType {
        NodeType::Program
    }

    fn token_type(&self) -> TokenType {
        TokenType::Eof
    }

    fn token_literal(&self) -> &str {
        ""
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = self
            .statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<_>>();
        write!(f, "{}", vec.join(""))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> Self {
        LetStatement { token, name, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> &Box<dyn Expression> {
        &self.value
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

impl Node for LetStatement {
    fn node_type(&self) -> NodeType {
        NodeType::LetStatement
    }
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for LetStatement {}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: Token, value: impl ToString) -> Self {
        Identifier {
            token,
            value: value.to_string(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Node for Identifier {
    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Identifier {}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<dyn Expression>) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

impl Node for ReturnStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ReturnStatement
    }
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ReturnStatement {}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        ExpressionStatement { token, expression }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn expression(&self) -> &Box<dyn Expression> {
        &self.expression
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl Node for ExpressionStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ExpressionStatement
    }
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ExpressionStatement {}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        IntegerLiteral { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for IntegerLiteral {
    fn node_type(&self) -> NodeType {
        NodeType::IntegerLiteral
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: impl ToString, right: Box<dyn Expression>) -> Self {
        PrefixExpression {
            token,
            operator: operator.to_string(),
            right,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn right(&self) -> &Box<dyn Expression> {
        &self.right
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl Node for PrefixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for PrefixExpression {}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Box<dyn Expression>,
        operator: impl ToString,
        right: Box<dyn Expression>,
    ) -> Self {
        InfixExpression {
            token,
            left,
            operator: operator.to_string(),
            right,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn left(&self) -> &Box<dyn Expression> {
        &self.left
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn right(&self) -> &Box<dyn Expression> {
        &self.right
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Node for InfixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for InfixExpression {}

#[derive(Debug)]
pub struct Boolean {
    token: Token,
    value: bool,
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Self {
        Boolean { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "true" } else { "false" })
    }
}

impl Node for Boolean {
    fn node_type(&self) -> NodeType {
        NodeType::Boolean
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Boolean {}

#[derive(Debug)]
pub struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn condition(&self) -> &Box<dyn Expression> {
        &self.condition
    }

    pub fn consequence(&self) -> &BlockStatement {
        &self.consequence
    }

    pub fn alternative(&self) -> &Option<BlockStatement> {
        &self.alternative
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.consequence,)?;
        if let Some(alt) = &self.alternative {
            write!(f, " else {}", alt)?;
        }
        Ok(())
    }
}

impl Node for IfExpression {
    fn node_type(&self) -> NodeType {
        NodeType::IfExpression
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IfExpression {}

#[derive(Debug)]
pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Token) -> Self {
        BlockStatement {
            token,
            statements: Vec::new(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }

    pub fn add_statement(&mut self, stmt: Box<dyn Statement>) {
        self.statements.push(stmt);
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = self
            .statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<_>>();
        write!(f, "{{{}}}", vec.join(""))
    }
}

impl Node for BlockStatement {
    fn node_type(&self) -> NodeType {
        NodeType::BlockStatement
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for BlockStatement {}

#[derive(Debug)]
pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        FunctionLiteral {
            token,
            parameters,
            body,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>();
        write!(
            f,
            "{}({}) {}",
            self.token.literal(),
            params.join(", "),
            self.body
        )
    }
}

impl Node for FunctionLiteral {
    fn node_type(&self) -> NodeType {
        NodeType::FunctionLiteral
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for FunctionLiteral {}

#[derive(Debug)]
pub struct CallExpression {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        CallExpression {
            token,
            function,
            arguments,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn function(&self) -> &Box<dyn Expression> {
        &self.function
    }

    pub fn arguments(&self) -> &Vec<Box<dyn Expression>> {
        &self.arguments
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .arguments
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>();
        write!(f, "{}({})", self.function, args.join(", "))
    }
}

impl Node for CallExpression {
    fn node_type(&self) -> NodeType {
        NodeType::CallExpression
    }
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn token_type(&self) -> TokenType {
        self.token.token_type()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for CallExpression {}
