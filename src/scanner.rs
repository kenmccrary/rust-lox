use crate::scanner::TokenType::*;
use crate::lib::error;

use std::collections::HashMap;


#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    StringLiteral(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
    pub fn line(&self) -> usize {
        self.line
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), And);
        keywords.insert("class".to_string(), Class);
        keywords.insert("else".to_string(), Else);
        keywords.insert("false".to_string(), False);
        keywords.insert("for".to_string(), For);
        keywords.insert("fun".to_string(), Fun);
        keywords.insert("if".to_string(), If);
        keywords.insert("nil".to_string(), Nil);
        keywords.insert("or".to_string(), Or);
        keywords.insert("print".to_string(), Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("super".to_string(), Super);
        keywords.insert("this".to_string(), This);
        keywords.insert("true".to_string(), True);
        keywords.insert("var".to_string(), Var);
        keywords.insert("while".to_string(), While);

        Scanner {
            source,
            tokens: vec![],
            keywords,
            current: 0,
            start: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }

            '\"' => self.string_literal(),

            '0'..='9' => self.number_literal(),

            'a'..='z' | 'A'..='Z' => self.identifier(),

            ' ' | '\r' | '\t' => (), // Ignore whitespace

            '\n' => self.line = self.line + 1,

            _ => error(self.line, "Unexpected charater"),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text);

        if token_type.is_some() {
            self.add_token(token_type.unwrap().clone());
        } else {
            self.add_token(Identifier(text.to_string()));
        }
    }

    fn number_literal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number_text = &self.source[self.start..self.current];
        let value: f32 = number_text.parse().expect("Failed to parse number");
        self.add_token(Number(value));
    }

    fn string_literal(&mut self) {
        while self.peek() != '\"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }

            self.advance();
        }

        let value = &self.source[self.start + 1..=self.current - 1];
        self.add_token(StringLiteral(value.to_string()));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if !self.is_at_end() && self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c = self.source.as_str().chars().nth(self.current).unwrap();
        c
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        let c = self.source.as_str().chars().nth(self.current + 1).unwrap();
        c
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.current = self.current + 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token {
            token_type,
            lexeme: text.to_string(),
            line: self.line,
        };

        self.tokens.push(token);
    }
}
