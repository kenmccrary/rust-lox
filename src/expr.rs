use crate::scanner::{Token, TokenType};

pub(crate) type Expression = Box<Expr>;

pub trait Expr {}

// Binary Expressions --------------------------------------------------------------------------

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Expr for BinaryExpr {}

impl BinaryExpr {
    pub fn new(left_expr: Box<Expr>, operator: Token, right_expr: Box<Expr>) -> BinaryExpr {
        BinaryExpr {
            left: left_expr,
            operator: operator,
            right: right_expr,
        }
    }
}

// Gruiping Expressions --------------------------------------------------------------------------

pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Box<Expr>) -> GroupingExpr {
        GroupingExpr {
            expression: expression,
        }
    }
}

impl Expr for GroupingExpr {}

// Literal Expressions --------------------------------------------------------------------------

pub trait LiteralExpr : Expr {}

pub struct BooleanLiteral {
    pub value:  bool,
}

impl Expr for BooleanLiteral {}
impl LiteralExpr for BooleanLiteral {}

pub struct StringLiteral {
    value: String,
}

impl Expr for StringLiteral {}
impl LiteralExpr for StringLiteral {}

pub struct NumberLiteral {
    pub value: f32,
}

impl Expr for NumberLiteral {}
impl LiteralExpr for NumberLiteral {}

pub struct NilLiteral {

}

impl Expr for NilLiteral {}
impl LiteralExpr for NilLiteral {}


// Unary Expressions --------------------------------------------------------------------------

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Expr for UnaryExpr {}

// ----------------------------------------------------------------------------------------------
