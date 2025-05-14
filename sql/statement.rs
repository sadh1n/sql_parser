// statement.rs

use crate::ast::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Select {
        columns: Vec<Expression>,
        from: String,
        r#where: Option<Expression>,
        orderby: Vec<Expression>,
    },
    CreateTable {
        table_name: String,
        column_list: Vec<TableColumn>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableColumn {
    pub column_name: String,
    pub column_type: DBType,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DBType {
    Int,
    Varchar(usize),
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constraint {
    NotNull,
    PrimaryKey,
    Check(Expression),
}
