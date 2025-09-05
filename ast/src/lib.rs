/// A library for working with abstract syntax trees (AST).
#![forbid(unsafe_code)]

// we'll grow these enums alongside the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Name(String),
    // placeholder: binary ops, calls, etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    // placehoder: assignment, if, while, def, return...
}