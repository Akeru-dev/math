/*
1. input math syntax
2. lexical and semantic analysis
3. equation identification and algorithm application
4. solving the equation(s)
5. formulating and storing the steps
6. displaying the output
*/

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Plus,      // +
    Minus,     // -
    Star,      // *
    Over,      // /
    Colon,     // :
    Semi,      // ;
    Up,        // ^
    Not,       // !
    Eq,        // =
    NotEq,     // !=
    Greater,   // >
    Less,      // <
    GreaterEq, // >=
    LessEq,    // <=
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    And,       // `and` keyword
    Or,        // `or` keyword
    State,     // `state` keyword
    Number,    // numeric value
    Ident,     // identifier
    EoF,       // end of the file
    Unknown,   // unknown character
}
use TokenType::*;

#[derive(Debug)]
pub struct Token {
    token: TokenType,
    lexeme: &'static str,
    lt: Option<f32>,
}

#[derive(Debug)]
pub enum Expression {
    Numeric {
        lt: f32,                       // 5
        sign: bool,                    // -32, +53 (true = +, false = -)
        base: usize,                   // 11101_2
        period: usize,                 // 3.(3)
        e: usize,                      // 5e12
        frac: Option<Box<Expression>>, // 1/2
    },
    Variable(Token), // x, y, speed, size
    Binary {
        lhs: Box<Expression>,
        op: Token,
        rhs: Box<Expression>,
    }, // 2 + 2, 4 * 3
    Grouping(Box<Expression>), // 2*(4 - 3)
    // sqrt{12}, frac{12}{3}
    Function {
        name: Token,
        args: Vec<Expression>,
    },
    // f(5)
    Call {
        name: Token,
        args: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    // 6 + 3, x + 4 = 4, 2x < 12
    Equation {
        lhs: Expression,
        op: Option<Token>,
        rhs: Option<Expression>,
    },
    // state x = 5
    State {
        name: Token,
        lt: Expression,
    },
    // f(x) = x + 1
    Function {
        name: Token,
        params: Vec<Token>,
        body: Expression,
    },
}

pub struct Syntax {
    src: &'static str,
    tokens: Vec<Token>,
    stmts: Vec<Statement>,
    keywords: HashMap<&'static str, TokenType>,
    line: usize,
    start: usize,
    current: usize,
    parser_current: usize,
    id: usize,
}

impl Syntax {
    pub fn new(src: &'static str) -> Self {
        return Self {
            src,
            tokens: Vec::new(),
            stmts: Vec::new(),
            keywords: HashMap::from([
                ("state", TokenType::State),
                ("and", TokenType::And),
                ("or", TokenType::Or),
            ]),
            line: 1,
            start: 0,
            current: 0,
            parser_current: 0,
            id: 0,
        };
    }

    pub fn analysis(&mut self) -> &Vec<Statement> {
        while !self.is_eof() {
            self.start = self.current;
            match self.advance_token() {
                '+' => self.push_token(Plus, None),
                '-' => self.push_token(Minus, None),
                '*' => self.push_token(Star, None),
                '/' => self.push_token(Over, None),
                ':' => self.push_token(Colon, None),
                ';' => self.push_token(Semi, None),
                '^' => self.push_token(Up, None),
                '!' => {
                    if self.peek() == '=' {
                        self.push_token(NotEq, None)
                    } else {
                        self.push_token(Not, None)
                    }
                }
                '=' => self.push_token(Eq, None),
                '>' => {
                    if self.peek() == '=' {
                        self.push_token(GreaterEq, None)
                    } else {
                        self.push_token(Greater, None)
                    }
                }
                '<' => {
                    if self.peek() == '=' {
                        self.push_token(LessEq, None)
                    } else {
                        self.push_token(Less, None)
                    }
                }
                '(' => self.push_token(LParen, None),
                ')' => self.push_token(RParen, None),
                '{' => self.push_token(LBrace, None),
                '}' => self.push_token(RBrace, None),
                c if c.is_ascii_digit() => self.lex_number(c),
                c if c.is_ascii_alphabetic() => self.lex_ident(),
                c => {
                    self.push_token(Unknown, None);
                    eprintln!("error: unknown character: '{}'", c)
                }
            }
        }
        self.tokens.push(Token {
            token: TokenType::EoF,
            lexeme: "\0",
            lt: None,
        });
        self.semantics()
    }

    fn lex_number(&mut self, c: char) {
        // TODO
    }

    fn lex_ident(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance_token();
        }
        let sub = &self.src[self.start..self.current];
        let token = self.keywords.get(sub).unwrap_or(&Ident);
        self.push_token(*token, None);
    }

    fn semantics(&mut self) -> &Vec<Statement> {
        // TODO: Statement / Expression semantics
        &self.stmts
    }

    #[inline]
    fn is_eof(&mut self) -> bool {
        self.current >= self.src.len()
    }

    #[inline]
    fn advance_token(&mut self) -> char {
        let c = self.peek();
        self.current += c.len_utf8();
        c
    }

    #[inline]
    fn push_token(&mut self, token: TokenType, lt: Option<f32>) {
        let lexeme = &self.src[self.start..self.current];
        self.tokens.push(Token { token, lexeme, lt });
    }

    #[inline]
    fn peek(&self) -> char {
        self.src[self.current..].chars().next().unwrap_or('\0')
    }
}

fn main() {
    println!("Hello, world!");
}
