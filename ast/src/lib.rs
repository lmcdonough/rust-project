#![forbid(unsafe_code)]

/// Expresion nodes for our Python subset.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Name(String),
    // placeholder: binary ops, calls, etc.
}

/// Statement nodes for our Python subset.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    // placehoder: assignment, if, while, def, return...
}
