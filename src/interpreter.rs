use crate::expr::{Expression, UnaryExpr, ExpressionProcessor, BinaryExpr, GroupingExpr, StringLiteral, NumberLiteral, NilLiteral};
use crate::lox_object::LoxObject;
use crate::scanner::TokenType::{*};
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

            TokenType::Minus => return LoxObject::Number(-object.to_number()),

            TokenType::Bang => return LoxObject::Boolean(!object.is_truthy()),

            _ => panic!("processUnaryExpr")
        }


    }

    fn processBinaryExpr(&self, binaryExpr: &BinaryExpr) -> LoxObject {
        let left = self.evaluate(binaryExpr.left.clone());
        let right = self.evaluate(binaryExpr.right.clone());

        let x = binaryExpr.operator.token_type();
        match binaryExpr.operator.token_type() {

            TokenType::Minus => return LoxObject::Number(left.to_number() - right.to_number()),

            // TODO handle String type
            TokenType::Plus => {
                /*   let leftNum = left.to_number();
                let rightNum = right.to_number();
                return LoxObject::Number(leftNum + rightNum); */

                if let LoxObject::Number(leftNum) = left {
                    if let LoxObject::Number(rightNum) = right {
                        return LoxObject::Number(leftNum + rightNum);
                    }
                    todo!() // syntax error
                }

                if let LoxObject::String(leftStr) = left {
                    if let LoxObject::String(rightStr) = right {
                        return LoxObject::String(format!("{}{}", leftStr, rightStr));
                    }
                    todo!() // syntax error
                }

                todo!() // syntax error
            }


            TokenType::Slash => return LoxObject::Number(left.to_number() / right.to_number()),

            TokenType::Star => return LoxObject::Number(left.to_number() * right.to_number()),

            _ => panic!("SDfsdf"),
        }
    }

    fn processGroupingExpr(&self, groupingExpr: &GroupingExpr) -> LoxObject {
        return self.evaluate(groupingExpr.expression.clone());
    }

    fn processStringLiteralExpr(&self, stringLiteralExpr: &StringLiteral) -> LoxObject {
        return LoxObject::String(stringLiteralExpr.value.clone());
    }

    fn processNumberLiteralExpr(&self, numberLiteralExpr: &NumberLiteral) -> LoxObject {
        return LoxObject::Number(numberLiteralExpr.value);
    }

    fn processNilLiteralExpr(&self, nilLiteralExpr: &NilLiteral) -> LoxObject {
        todo!()
    }

}