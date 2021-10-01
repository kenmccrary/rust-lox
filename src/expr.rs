use crate::scanner::{Token, TokenType};
use crate::lox_object::LoxObject;
use std::rc::Rc;

pub(crate) type Expression = Rc<Expr>;

pub trait Expr {
    fn attach(&self, expressionProcessor : &ExpressionProcessor) -> LoxObject;
}

// Binary Expressions --------------------------------------------------------------------------

pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

impl Expr for BinaryExpr {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processBinaryExpr(&self);
    }
}

impl BinaryExpr {
    pub fn new(left_expr: Rc<Expr>, operator: Token, right_expr: Rc<Expr>) -> BinaryExpr {
        BinaryExpr {
            left: left_expr,
            operator: operator,
            right: right_expr,
        }
    }
}

// Gruiping Expressions --------------------------------------------------------------------------

pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}


impl GroupingExpr {
    pub fn new(expression: Rc<Expr>) -> GroupingExpr {
        GroupingExpr {
            expression: expression,
        }
    }
}

//impl Expr for GroupingExpr {}

impl Expr for GroupingExpr {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processGroupingExpr(&self);
    }
}

// Literal Expressions --------------------------------------------------------------------------

pub trait LiteralExpr : Expr {}

// BooleanLiteral Expressions -------------------------------------------------------------------

pub struct BooleanLiteral {
    pub value:  bool,
}

//impl Expr for BooleanLiteral {}

impl Expr for BooleanLiteral {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        LoxObject::Boolean(true)
        //return expressionProcessor.processLiteralExpr((&self));
    }
}

impl LiteralExpr for BooleanLiteral {}


// StringLiteral Expressions -------------------------------------------------------------------

pub struct StringLiteral {
    value: String,
}

//impl Expr for StringLiteral {}

impl Expr for StringLiteral {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processStringLiteralExpr(&self);
    }
}

impl LiteralExpr for StringLiteral {}

// NumberLiteral Expressions -------------------------------------------------------------------

pub struct NumberLiteral {
    pub value: f32,
}

//impl Expr for NumberLiteral {}

impl Expr for NumberLiteral {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processNumberLiteralExpr(&self);
    }
}

impl LiteralExpr for NumberLiteral {}

// NilLiteral Expressions -------------------------------------------------------------------

pub struct NilLiteral {

}

//impl Expr for NilLiteral {}

impl Expr for NilLiteral {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processNilLiteralExpr((&self));
    }
}

impl LiteralExpr for NilLiteral {}


// Unary Expressions --------------------------------------------------------------------------

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

//impl Expr for UnaryExpr {}

impl Expr for UnaryExpr {
    fn attach(&self, expressionProcessor :  &ExpressionProcessor) -> LoxObject {
        return expressionProcessor.processUnaryExpr((&self));
    }
}

// ----------------------------------------------------------------------------------------------

pub trait ExpressionProcessor {
    fn processUnaryExpr(&self, unaryExpr: &UnaryExpr) -> LoxObject;
    fn processBinaryExpr(&self, binaryExpr: &BinaryExpr) -> LoxObject;
    fn processGroupingExpr(&self, groupingExpr: &GroupingExpr) -> LoxObject;
    fn processStringLiteralExpr(&self, stringLiteralExpr: &StringLiteral) -> LoxObject;
    fn processNumberLiteralExpr(&self, numberLiteralExpr: &NumberLiteral) -> LoxObject;
    fn processNilLiteralExpr(&self, nilLiteralExpr: &NilLiteral) -> LoxObject;
}