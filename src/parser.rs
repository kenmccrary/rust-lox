use crate::expr::{Expression, Expr, BooleanLiteral, NilLiteral, GroupingExpr, UnaryExpr, BinaryExpr, NumberLiteral};
use crate::scanner::TokenType::*;
use crate::scanner::{Token, TokenType};
use crate::lib::report;

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

    // TODO finish expression and equality
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(&[BangEqual, EqualEqual]) {
            let operator = (*self.previous()).clone();
            let right = self.term();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Expression  {
        let mut expr = self.term();

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = (*self.previous()).clone();
            let right = self.term();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(&[Plus, Minus]) {
            let operator = (*self.previous()).clone();
            let right = self.factor();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(&[Slash, Star]) {
            let operator = (*self.previous()).clone();
            let right = self.unary();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(&[Bang, Minus]) {
            let operator = (*self.previous()).clone();
            let right = self.unary();
            return Box::new(UnaryExpr{operator, right});
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {

        if self.match_token(&[False]) {
            return Box::new(BooleanLiteral{ value: false })
        }

        if self.match_token(&[True]) {
            return Box::new(BooleanLiteral{ value: true })
        }

        if self.match_token(&[Nil]) {
            return Box::new(NilLiteral{} )
        }

        if self.match_token(&[Number(0f32)]) {
            let number = (*self.previous()).clone();

           if let Number(value) = number.token_type() {
                return Box::new(NumberLiteral{value : *value});
            }
        }

        if self.match_token(&[StringLiteral("".parse().unwrap())]) {
            let number = (*self.previous()).clone();

            if let Number(value) = number.token_type() {
                return Box::new(NumberLiteral{value : *value});
            }
        }


        if self.match_token(&[LeftParen]) {

            let group = GroupingExpr::new(self.expression());
            self.consume_token(RightParen, "Expect ')' after expression");
            return Box::new(group);
        }

        panic!("Parser failed.")
    }


    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume_token(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        }

        panic!("Parser error: {}", message);
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        *self.peek().token_type() == Eof
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn parser_error(token: Token, message: String) {
        if *token.token_type() == Eof {
            report(token.line(), "at end".to_string(), &message);
        } else {
            report(token.line(),
                   "at '".to_string() + token.lexeme() + "'" ,
                   &message);
        }
    }
}
