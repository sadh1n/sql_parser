use crate::ast::Token;
use crate::ast::Keyword;
use crate::errors::{Result, SQLError};

pub fn tokenize_string(input: &str) -> Result<Vec<Token>> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); },
            ',' => { tokens.push(Token::Comma); chars.next(); },
            ';' => { tokens.push(Token::Semicolon); chars.next(); },
            '(' => { tokens.push(Token::LeftParentheses); chars.next(); },
            ')' => { tokens.push(Token::RightParentheses); chars.next(); },
            '=' => { tokens.push(Token::Equal); chars.next(); },
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') { chars.next(); tokens.push(Token::GreaterThanOrEqual); }
                else { tokens.push(Token::GreaterThan); }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') { chars.next(); tokens.push(Token::LessThanOrEqual); }
                else { tokens.push(Token::LessThan); }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') { chars.next(); tokens.push(Token::NotEqual); }
                else { return Err(SQLError::TokenizerError("Unexpected '!' without '='".into())); }
            }
            '+' => { tokens.push(Token::Plus); chars.next(); },
            '-' => { tokens.push(Token::Minus); chars.next(); },
            '*' => { tokens.push(Token::Multiply); chars.next(); },
            '/' => { tokens.push(Token::Divide); chars.next(); },
            '"' | '\'' => {
                let quote = chars.next().unwrap();
                let mut string = String::new();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == quote { break; }
                    string.push(next);
                }
                tokens.push(Token::String(string));
            }
            c if c.is_ascii_digit() => {
                let mut number = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(next);
                        chars.next();
                    } else { break; }
                }
                let value = number.parse::<u64>().unwrap();
                tokens.push(Token::Number(value));
            }
            c if c.is_alphabetic() => {
                let mut word = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        word.push(next);
                        chars.next();
                    } else { break; }
                }
                match word.to_uppercase().as_str() {
                    "SELECT" => tokens.push(Token::Keyword(Keyword::Select)),
                    "FROM" => tokens.push(Token::Keyword(Keyword::From)),
                    "WHERE" => tokens.push(Token::Keyword(Keyword::Where)),
                    "ORDER" => tokens.push(Token::Keyword(Keyword::Order)),
                    "BY" => tokens.push(Token::Keyword(Keyword::By)),
                    "ASC" => tokens.push(Token::Keyword(Keyword::Asc)),
                    "DESC" => tokens.push(Token::Keyword(Keyword::Desc)),
                    "AND" => tokens.push(Token::Keyword(Keyword::And)),
                    "OR" => tokens.push(Token::Keyword(Keyword::Or)),
                    "NOT" => tokens.push(Token::Keyword(Keyword::Not)),
                    "TRUE" => tokens.push(Token::Bool(true)),
                    "FALSE" => tokens.push(Token::Bool(false)),
                    _ => tokens.push(Token::Identifier(word)),
                }
            }
            other => {
                return Err(SQLError::TokenizerError(format!("Invalid character: {}", other)));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}