use crate::ast::{Expression, BinaryOperator, UnaryOperator, Token, Keyword};

pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }
}

pub fn parse_expression(stream: &mut TokenStream, min_prec: u8) -> Result<Expression, String> {
    let mut left = match stream.next() {
        Some(Token::Number(n)) => Expression::Number(n),
        Some(Token::String(s)) => Expression::String(s),
        Some(Token::Bool(b)) => Expression::Bool(b),
        Some(Token::Identifier(name)) => Expression::Identifier(name),
        Some(Token::Minus) => {
            let right = parse_expression(stream, 100)?;
            Expression::UnaryOperation {
                operand: Box::new(right),
                operator: UnaryOperator::Minus,
            }
        }
        Some(Token::Keyword(Keyword::Not)) => {
            let right = parse_expression(stream, 100)?;
            Expression::UnaryOperation {
                operand: Box::new(right),
                operator: UnaryOperator::Not,
            }
        }
        Some(Token::LeftParentheses) => {
            let expr = parse_expression(stream, 0)?;
            match stream.next() {
                Some(Token::RightParentheses) => expr,
                _ => return Err("Expected ')'".to_string()),
            }
        }
        Some(tok) => return Err(format!("Unexpected token: {:?}", tok)),
        None => return Err("Unexpected end of input".to_string()),
    };

    while let Some(op) = stream.peek() {
        let (prec, right_assoc, operator) = match op {
            Token::Plus => (10, false, BinaryOperator::Plus),
            Token::Minus => (10, false, BinaryOperator::Minus),
            Token::Multiply => (20, false, BinaryOperator::Multiply),
            Token::Divide => (20, false, BinaryOperator::Divide),
            Token::GreaterThan => (5, false, BinaryOperator::GreaterThan),
            Token::GreaterThanOrEqual => (5, false, BinaryOperator::GreaterThanOrEqual),
            Token::LessThan => (5, false, BinaryOperator::LessThan),
            Token::LessThanOrEqual => (5, false, BinaryOperator::LessThanOrEqual),
            Token::Equal => (5, false, BinaryOperator::Equal),
            Token::NotEqual => (5, false, BinaryOperator::NotEqual),
            Token::Keyword(Keyword::And) => (3, false, BinaryOperator::And),
            Token::Keyword(Keyword::Or) => (2, false, BinaryOperator::Or),
            _ => break,
        };

        if prec < min_prec {
            break;
        }

        stream.next(); // consume operator
        let right = parse_expression(stream, if right_assoc { prec } else { prec + 1 })?;

        left = Expression::BinaryOperation {
            left_operand: Box::new(left),
            operator,
            right_operand: Box::new(right),
        };
    }

    Ok(left)
}