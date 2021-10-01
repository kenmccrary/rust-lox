use crate::expr::{Expression, UnaryExpr, ExpressionProcessor, BinaryExpr, GroupingExpr, StringLiteral, NumberLiteral, NilLiteral};
use crate::lox_object::LoxObject;
use crate::scanner::TokenType::*;
use crate::scanner::{Token, TokenType};

pub struct Interpreter {
    // expression: Expression,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter {  }
    }

    pub fn interpret(&self, expresion : Expression) {
        let object = self.evaluate(expresion);

        println!("{:?}", object)
    }

    fn evaluate(&self, expression : Expression)  -> LoxObject {
        return expression.attach(self);
    }

}

impl ExpressionProcessor for Interpreter {

    fn processUnaryExpr(&self, unaryExpr: &UnaryExpr) -> LoxObject {
        let object = self.evaluate(unaryExpr.right.clone());

        match unaryExpr.operator.token_type() {

            MINUS => return LoxObject::Number(-object.to_number()),

            BANG => return LoxObject::Boolean(!object.is_truthy()),

        }

        panic!("oh hell");

    }

    fn processBinaryExpr(&self, binaryExpr: &BinaryExpr) -> LoxObject {
        let left = self.evaluate(binaryExpr.left.clone());
        let right = self.evaluate(binaryExpr.right.clone());

        match binaryExpr.operator.token_type() {

            MINUS => return LoxObject::Number(left.to_number() - right.to_number()),

            // TODO handle String type
            PLUS => {
                let leftNum = left.to_number();
                let rightNum = right.to_number();
                return LoxObject::Number(leftNum + rightNum);
            },



            SLASH => return LoxObject::Number(left.to_number() / right.to_number()),

            STAR => return LoxObject::Number(left.to_number() * right.to_number()),

            _ => panic!("SDfsdf"),
        }
    }

    fn processGroupingExpr(&self, groupingExpr: &GroupingExpr) -> LoxObject {
        return self.evaluate(groupingExpr.expression.clone());
    }

    fn processStringLiteralExpr(&self, stringLiteralExpr: &StringLiteral) -> LoxObject {
        todo!()
    }

    fn processNumberLiteralExpr(&self, numberLiteralExpr: &NumberLiteral) -> LoxObject {
        return LoxObject::Number(numberLiteralExpr.value);
    }

    fn processNilLiteralExpr(&self, nilLiteralExpr: &NilLiteral) -> LoxObject {
        todo!()
    }

}