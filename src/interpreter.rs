use crate::expr::{Expression, UnaryExpr, ExpressionProcessor, BinaryExpr, GroupingExpr, StringLiteral, NumberLiteral, NilLiteral};
use crate::lox_object::LoxObject;
//use crate::scanner::TokenType::{*};
use crate::scanner::{TokenType};

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

    fn process_unary_expr(&self, unary_expr: &UnaryExpr) -> LoxObject {
        let object = self.evaluate(unary_expr.right.clone());

        match unary_expr.operator.token_type() {

            TokenType::Minus => return LoxObject::Number(-object.to_number()),

            TokenType::Bang => return LoxObject::Boolean(!object.is_truthy()),

            _ => panic!("processUnaryExpr")
        }


    }

    fn process_binary_expr(&self, binary_expr: &BinaryExpr) -> LoxObject {
        let left = self.evaluate(binary_expr.left.clone());
        let right = self.evaluate(binary_expr.right.clone());

        //let x = binaryExpr.operator.token_type();
        match binary_expr.operator.token_type() {

            // Arithmetic binary operations
            TokenType::Minus => return LoxObject::Number(left.to_number() - right.to_number()),

            // TODO handle String type
            TokenType::Plus => {

                if let LoxObject::Number(left_num) = left {
                    if let LoxObject::Number(right_num) = right {
                        return LoxObject::Number(left_num + right_num);
                    }
                    todo!() // syntax error
                }

                if let LoxObject::String(left_str) = left {
                    if let LoxObject::String(right_str) = right {
                        return LoxObject::String(format!("{}{}", left_str, right_str));
                    }
                    todo!() // syntax error
                }

                todo!() // syntax error
            }

            TokenType::Slash => return LoxObject::Number(left.to_number() / right.to_number()),

            TokenType::Star => return LoxObject::Number(left.to_number() * right.to_number()),

            // Comparison binary operations
            TokenType::Greater => return LoxObject::Boolean(left.to_number() > right.to_number()),
            TokenType::GreaterEqual => return LoxObject::Boolean(left.to_number() >= right.to_number()),
            TokenType::Less => return LoxObject::Boolean(left.to_number() < right.to_number()),
            TokenType::LessEqual => return LoxObject::Boolean(left.to_number() <= right.to_number()),

            // Equality
            TokenType::EqualEqual => return LoxObject::Boolean(left == right),
            TokenType::BangEqual => return LoxObject::Boolean(left != right),

            _ => panic!("interpreter internal error"),
        }
    }

    fn process_grouping_expr(&self, grouping_expr: &GroupingExpr) -> LoxObject {
        return self.evaluate(grouping_expr.expression.clone());
    }

    fn process_string_literal_expr(&self, string_literal_expr: &StringLiteral) -> LoxObject {
        return LoxObject::String(string_literal_expr.value.clone());
    }

    fn process_number_literal_expr(&self, number_literal_expr: &NumberLiteral) -> LoxObject {
        return LoxObject::Number(number_literal_expr.value);
    }

    fn process_nil_literal_expr(&self, nil_literal_expr: &NilLiteral) -> LoxObject {
        return LoxObject::Nil;
    }

}