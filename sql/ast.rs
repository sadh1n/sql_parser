#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryOperation {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    UnaryOperation {
        operand: Box<Expression>,
        operator: UnaryOperator,
    },
    Number(u64),
    Bool(bool),
    Identifier(String),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
    Asc,
    Desc,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Bool(bool),
    RightParentheses,
    LeftParentheses,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Multiply,
    Divide,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Eof,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Keyword {
    Select,
    Create,
    Table,
    Where,
    Order,
    By,
    Asc,
    Desc,
    From,
    And,
    Or,
    Not,
    True,
    False,
    Primary,
    Key,
    Check,
    Int,
    Bool,
    Varchar,
    Null,
}