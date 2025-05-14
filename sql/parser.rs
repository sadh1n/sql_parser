use crate::tokens::{Token, Keyword};
use crate::expression::{Expression, BinaryOperator, UnaryOperator};
use crate::statement::Statement;

pub struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, position: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let tok = self.tokens[self.position].clone();
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }

    pub fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.next() {
            Some(ref t) if t == expected => Ok(()),
            Some(t) => Err(format!("Expected {:?} but found {:?}", expected, t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    pub fn expect_keyword(&mut self, keyword: Keyword) -> Result<(), String> {
        match self.next() {
            Some(Token::Keyword(k)) if k == keyword => Ok(()),
            Some(t) => Err(format!("Expected keyword {:?} but found {:?}", keyword, t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

pub fn build_statement(input: &str) -> Result<Statement, String> {
    use crate::tokenizer::tokenize_string;
    let tokens = tokenize_string(input)?;
    let mut stream = TokenStream::new(tokens);

    match stream.peek() {
        Some(Token::Keyword(Keyword::Select)) => parse_select(&mut stream),
        Some(Token::Keyword(Keyword::Create)) => parse_create(&mut stream),
        other => Err(format!("Unexpected start of statement: {:?}", other)),
    }
}

fn parse_select(stream: &mut TokenStream) -> Result<Statement, String> {
    stream.expect_keyword(Keyword::Select)?;

    let mut columns = Vec::new();
    loop {
        if let Some(Token::Identifier(name)) = stream.next() {
            columns.push(Expression::Identifier(name));
        } else {
            return Err("Expected column name".to_string());
        }

        match stream.peek() {
            Some(Token::Comma) => {
                stream.next();
            }
            _ => break,
        }
    }

    stream.expect_keyword(Keyword::From)?;

    let table_name = match stream.next() {
        Some(Token::Identifier(name)) => name,
        _ => return Err("Expected table name after FROM".to_string()),
    };

    let mut r#where = None;
    let mut orderby = Vec::new();

    loop {
        match stream.peek() {
            Some(Token::Keyword(Keyword::Where)) => {
                stream.next();
                // Placeholder for expression parser
                r#where = Some(Expression::Bool(true)); // dummy
            }
            Some(Token::Keyword(Keyword::Order)) => {
                stream.next();
                stream.expect_keyword(Keyword::By)?;
                // Placeholder for ORDER BY expressions
            }
            Some(Token::Semicolon) => {
                stream.next();
                break
