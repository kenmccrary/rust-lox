use crate::expr::{Expression, BooleanLiteral, NilLiteral, GroupingExpr, UnaryExpr, BinaryExpr, NumberLiteral, StringLiteral};
use crate::scanner::TokenType::*;
use crate::scanner::{Token, TokenType};
use crate::lib::report;
use std::rc::Rc;

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
 */

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expression>{
        Some(self.expression())
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(&[BangEqual, EqualEqual]) {
            let operator = (*self.previous()).clone();
            let right = self.term();
            expr = Rc::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Expression  {
        let mut expr = self.term();

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = (*self.previous()).clone();
            let right = self.term();
            expr = Rc::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(&[Plus, Minus]) {
            let operator = (*self.previous()).clone();
            let right = self.factor();
            expr = Rc::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(&[Slash, Star]) {
            let operator = (*self.previous()).clone();
            let right = self.unary();
            expr = Rc::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(&[Bang, Minus]) {
            let operator = (*self.previous()).clone();
            let right = self.unary();
            return Rc::new(UnaryExpr{operator, right});
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {

        if self.match_token(&[False]) {
            return Rc::new(BooleanLiteral{ value: false })
        }

        if self.match_token(&[True]) {
            return Rc::new(BooleanLiteral{ value: true })
        }

        if self.match_token(&[Nil]) {
            return Rc::new(NilLiteral{} )
        }

        if self.match_token(&[Number(0f32)]) {
            let number = (*self.previous()).clone();

           if let Number(value) = number.token_type() {
                return Rc::new(NumberLiteral{value : value});
            }
        }

        if self.match_token(&[StringLiteral("".parse().unwrap())]) {
            let string = (*self.previous()).clone();

            if let StringLiteral(value) = string.token_type() {
                return Rc::new(StringLiteral{value : value});
            }
        }

        if self.match_token(&[LeftParen]) {

            let group = GroupingExpr::new(self.expression());
            self.consume_token(RightParen, "Expect ')' after expression");
            return Rc::new(group);
        }

        // FIX
        let token = self.peek().clone();
        self.parser_error(&token, "Expect Expression".to_string());
        panic!("fix");
    }


    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume_token(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("Parser error: {}", message);
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        let found_type = self.peek().token_type();
        let answer = found_type == token_type;
        answer
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type() == Eof
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn parser_error(&mut self, token: &Token, message: String) {
        if token.token_type() == Eof {
            report(token.line(), "at end".to_string(), &message);
        } else {
            report(token.line(),
                   "at '".to_string() + token.lexeme() + "'" ,
                   &message);
        }
    }

    // TODO
    fn synchronize() {
        // In the book an exception is thrown in error recovery so we need an alternative in Rust
    }
}
