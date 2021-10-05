use crate::scanner::{Token};
use crate::lox_object::LoxObject;
use std::rc::Rc;

pub(crate) type Expression = Rc<dyn Expr>;

pub trait Expr {
    fn attach(&self, expression_processor: &dyn ExpressionProcessor) -> LoxObject;
}

// Binary Expressions --------------------------------------------------------------------------

pub struct BinaryExpr {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}

impl Expr for BinaryExpr {
    fn attach(&self, expression_processor:  &dyn ExpressionProcessor) -> LoxObject {
        return expression_processor.process_binary_expr(&self);
    }
}

impl BinaryExpr {
    pub fn new(left_expr: Rc<dyn Expr>, operator: Token, right_expr: Rc<dyn Expr>) -> BinaryExpr {
        BinaryExpr {
            left: left_expr,
            operator: operator,
            right: right_expr,
        }
    }
}

// Gruiping Expressions --------------------------------------------------------------------------

pub struct GroupingExpr {
    pub expression: Rc<dyn Expr>,
}


impl GroupingExpr {
    pub fn new(expression: Rc<dyn Expr>) -> GroupingExpr {
        GroupingExpr {
            expression: expression,
        }
    }
}

//impl Expr for GroupingExpr {}

impl Expr for GroupingExpr {
    fn attach(&self, expression_processor:  &dyn ExpressionProcessor) -> LoxObject {
        return expression_processor.process_grouping_expr(&self);
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
    fn attach(&self, expression_processor:  &dyn ExpressionProcessor) -> LoxObject {
        LoxObject::Boolean(true)
        //return expressionProcessor.processLiteralExpr((&self));
    }
}

impl LiteralExpr for BooleanLiteral {}


// StringLiteral Expressions -------------------------------------------------------------------

pub struct StringLiteral {
    pub(crate) value: String,
}

//impl Expr for StringLiteral {}

impl Expr for StringLiteral {
    fn attach(&self, expression_processor:  &dyn ExpressionProcessor) -> LoxObject {
        return expression_processor.process_string_literal_expr(&self);
    }
}

impl LiteralExpr for StringLiteral {}

// NumberLiteral Expressions -------------------------------------------------------------------

pub struct NumberLiteral {
    pub value: f32,
}

//impl Expr for NumberLiteral {}

impl Expr for NumberLiteral {
    fn attach(&self, expresion_processor:  &dyn ExpressionProcessor) -> LoxObject {
        return expresion_processor.process_number_literal_expr(&self);
    }
}

impl LiteralExpr for NumberLiteral {}

// NilLiteral Expressions -------------------------------------------------------------------

pub struct NilLiteral {

}

//impl Expr for NilLiteral {}

impl Expr for NilLiteral {
    fn attach(&self, expression_processor :  &dyn ExpressionProcessor) -> LoxObject {
        return expression_processor.process_nil_literal_expr(&self);
    }
}

impl LiteralExpr for NilLiteral {}


// Unary Expressions --------------------------------------------------------------------------

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}

//impl Expr for UnaryExpr {}

impl Expr for UnaryExpr {
    fn attach(&self, expression_processor :  &dyn ExpressionProcessor) -> LoxObject {
        return expression_processor.process_unary_expr(&self);
    }
}

// ----------------------------------------------------------------------------------------------

pub trait ExpressionProcessor {
    fn process_unary_expr(&self, unary_expr: &UnaryExpr) -> LoxObject;
    fn process_binary_expr(&self, binary_expr: &BinaryExpr) -> LoxObject;
    fn process_grouping_expr(&self, grouping_expr: &GroupingExpr) -> LoxObject;
    fn process_string_literal_expr(&self, string_literal_expr: &StringLiteral) -> LoxObject;
    fn process_number_literal_expr(&self, number_literal_expr: &NumberLiteral) -> LoxObject;
    fn process_nil_literal_expr(&self, nil_literal_expr: &NilLiteral) -> LoxObject;
}