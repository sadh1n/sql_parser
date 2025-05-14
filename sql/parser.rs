use crate::tokenizer::tokenize_string;
use crate::statement::{Statement, TableColumn, DBType, Constraint};
use crate::ast::{Token, Keyword};
use crate::pratt::{TokenStream, parse_expression};
use crate::errors::{Result, SQLError};

pub fn build_statement(input: &str) -> Result<Statement> {
    let tokens = tokenize_string(input)?;
    let mut stream = TokenStream::new(tokens);

    match stream.peek() {
        Some(Token::Keyword(Keyword::Select)) => parse_select(&mut stream),
        Some(Token::Keyword(Keyword::Create)) => parse_create_table(&mut stream),
        other => Err(SQLError::ParserError(format!("Unknown start of statement: {:?}", other))),
    }
}

fn parse_select(stream: &mut TokenStream) -> Result<Statement> {
    stream.expect_keyword(Keyword::Select)?;
    let mut columns = Vec::new();

    loop {
        let expr = parse_expression(stream, 0).map_err(|e| SQLError::ParserError(e))?;
        columns.push(expr);

        if matches!(stream.peek(), Some(Token::Comma)) {
            stream.next(); // consume comma
        } else {
            break;
        }
    }

    stream.expect_keyword(Keyword::From)?;
    let table_name = match stream.next() {
        Some(Token::Identifier(name)) => name,
        _ => return Err(SQLError::ParserError("Expected table name after FROM".into())),
    };

    let mut r#where = None;
    let mut orderby = Vec::new();

    loop {
        match stream.peek() {
            Some(Token::Keyword(Keyword::Where)) => {
                stream.next();
                r#where = Some(parse_expression(stream, 0).map_err(|e| SQLError::ParserError(e))?);
            }
            Some(Token::Keyword(Keyword::Order)) => {
                stream.next();
                stream.expect_keyword(Keyword::By)?;
                loop {
                    let expr = parse_expression(stream, 0).map_err(|e| SQLError::ParserError(e))?;
                    orderby.push(expr);
                    if matches!(stream.peek(), Some(Token::Comma)) {
                        stream.next();
                    } else {
                        break;
                    }
                }
            }
            Some(Token::Semicolon) | Some(Token::Eof) => {
                stream.next();
                break;
            }
            _ => break,
        }
    }

    Ok(Statement::Select {
        columns,
        from: table_name,
        r#where,
        orderby,
    })
}

fn parse_create_table(stream: &mut TokenStream) -> Result<Statement> {
    stream.expect_keyword(Keyword::Create)?;
    stream.expect_keyword(Keyword::Table)?;

    let table_name = match stream.next() {
        Some(Token::Identifier(name)) => name,
        _ => return Err(SQLError::ParserError("Expected table name".into())),
    };

    stream.expect_token(Token::LeftParentheses)?;
    let mut column_list = Vec::new();

    loop {
        let column_name = match stream.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(SQLError::ParserError("Expected column name".into())),
        };

        let column_type = match stream.next() {
            Some(Token::Keyword(Keyword::Int)) => DBType::Int,
            Some(Token::Keyword(Keyword::Bool)) => DBType::Bool,
            Some(Token::Keyword(Keyword::Varchar)) => {
                stream.expect_token(Token::LeftParentheses)?;
                let length = match stream.next() {
                    Some(Token::Number(n)) => n,
                    _ => return Err(SQLError::ParserError("Expected VARCHAR length".into())),
                };
                stream.expect_token(Token::RightParentheses)?;
                DBType::Varchar(length as usize)
            }
            _ => return Err(SQLError::ParserError("Expected column type".into())),
        };

        let mut constraints = Vec::new();
        loop {
            match stream.peek() {
                Some(Token::Keyword(Keyword::Primary)) => {
                    stream.next();
                    stream.expect_keyword(Keyword::Key)?;
                    constraints.push(Constraint::PrimaryKey);
                }
                Some(Token::Keyword(Keyword::Not)) => {
                    stream.next();
                    stream.expect_keyword(Keyword::Null)?;
                    constraints.push(Constraint::NotNull);
                }
                Some(Token::Keyword(Keyword::Check)) => {
                    stream.next();
                    stream.expect_token(Token::LeftParentheses)?;
                    let expr = parse_expression(stream, 0).map_err(|e| SQLError::ParserError(e))?;
                    stream.expect_token(Token::RightParentheses)?;
                    constraints.push(Constraint::Check(expr));
                }
                _ => break,
            }
        }

        column_list.push(TableColumn {
            column_name,
            column_type,
            constraints,
        });

        match stream.peek() {
            Some(Token::Comma) => {
                stream.next();
            }
            Some(Token::RightParentheses) => {
                stream.next();
                break;
            }
            _ => return Err(SQLError::ParserError("Expected ',' or ')'".into())),
        }
    }

    stream.expect_token(Token::Semicolon)?;

    Ok(Statement::CreateTable {
        table_name,
        column_list,
    })
}

trait ExpectExt {
    fn expect_token(&mut self, expected: Token) -> Result<()>;
    fn expect_keyword(&mut self, keyword: Keyword) -> Result<()>;
}

impl ExpectExt for TokenStream {
    fn expect_token(&mut self, expected: Token) -> Result<()> {
        match self.next() {
            Some(t) if t == expected => Ok(()),
            Some(t) => Err(SQLError::ParserError(format!("Expected {:?}, found {:?}", expected, t))),
            None => Err(SQLError::ParserError("Unexpected end of input".into())),
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Result<()> {
        match self.next() {
            Some(Token::Keyword(k)) if k == keyword => Ok(()),
            Some(t) => Err(SQLError::ParserError(format!("Expected keyword {:?}, found {:?}", keyword, t))),
            None => Err(SQLError::ParserError("Unexpected end of input".into())),
        }
    }
}
