use crate::atom::Atom;
use std::mem::discriminant;

#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    //Numbers
    Float(f64),
    //Brackets
    LParen,
    RParen,
    //Mathematical Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    //Logical Operators
    True,
    False,
    Equal,
    NotEqual,
    Not,
    Then,
    GreaterThan,
    LessThan,
    //Keywords,
    If,
    Else,
    ElseIf,
    End,
    //WhiteSpace
    NewLine,

    //Assignment
    Assign,
    //Unknown things
    EOF,
    UnknownToken(String),
}

//This block should be moved to the parser file?
impl Token {
    pub fn to_atom(&self) -> Atom {
        match self {
            Token::Float(value) => Atom::Float(*value),
            Token::Plus => Atom::Plus,
            Token::Power => Atom::Power,
            Token::Multiply => Atom::Multiply,
            Token::Divide => Atom::Divide,
            Token::Assign => Atom::Assign,
            Token::True => Atom::True,
            Token::False => Atom::False,
            Token::Equal => Atom::Equal,
            Token::NotEqual => Atom::NotEqual,
            Token::Not => Atom::Not,
            Token::GreaterThan => Atom::GreaterThan,
            Token::LessThan => Atom::LessThan,
            _ => panic!("Error cannot convert token {:?} to atom", self),
        }
    }

    pub fn precedence(&self) -> isize {
        match self {
            Self::GreaterThan => 10,
            Self::LessThan => 10,
            Self::Plus => 20,
            Self::Minus => 20,
            Self::Multiply => 30,
            Self::Divide => 30,
            Self::Power => 40,
            Self::Assign => 50,
            _ => 0,
        }
    }
}

pub struct Lexer {
    tokens: Vec<String>,
    idx: usize,
}

impl Lexer {
    pub fn new(tokens: Vec<String>) -> Self {
        Lexer { tokens, idx: 0 }
    }

    pub fn next(&mut self) -> Token {
        if self.idx < self.tokens.len() {
            let token = str_to_token(&self.tokens[self.idx]);

            if token == Token::NewLine {
                self.idx += 1;
                return self.next();
            }

            let next_token = self.peek(1);

            match (&token, &next_token) {
                (Token::Assign, Token::Assign) => {
                    self.idx += 2;
                    Token::Equal
                }
                (Token::Not, Token::Assign) => {
                    self.idx += 2;
                    Token::NotEqual
                }
                _ => {
                    self.idx += 1;
                    token
                }
            }
        } else {
            Token::EOF
        }
    }

    pub fn peek(&self, n: usize) -> Token {
        if self.idx + n < self.tokens.len() {
            let token = str_to_token(&self.tokens[self.idx + n]);

            if token == Token::NewLine {
                return self.peek(n + 1);
            } else {
                token
            }
        } else {
            Token::EOF
        }
    }

    pub fn expect(&mut self, token: &Token) -> Token {
        let next_token = self.next();
        if discriminant(token) != discriminant(&next_token) {
            panic!(
                "Unexpected token given {:?} expected {:?}",
                token, next_token
            )
        }
        next_token
    }

    pub fn all(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while self.peek(0) != Token::EOF {
            tokens.push(self.next())
        }
        tokens
    }
}

fn str_to_token(token: &str) -> Token {
    match token {
        //Mathematical operators
        "+" => Token::Plus,
        "-" => Token::Minus,
        "/" => Token::Divide,
        "*" => Token::Multiply,
        "^" => Token::Power,
        //Logical Operators
        "!" => Token::Not,
        "<" => Token::LessThan,
        ">" => Token::GreaterThan,
        //Assignemnt
        "=" => Token::Assign,
        //Brackets
        "(" => Token::LParen,
        ")" => Token::RParen,
        //Keywords
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "elseif" => Token::ElseIf,
        "end" => Token::End,
        "\n" => Token::NewLine,
        "false" => Token::False,
        "true" => Token::True,

        //Literals
        token if is_f64(token) => Token::Float(token.parse::<f64>().unwrap()),
        token if is_valid_symbol(token) => Token::Symbol(token.to_string()),
        token => return Token::UnknownToken(token.to_string()),
    }
}

fn is_f64(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn is_i64(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn is_valid_symbol(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;
    use indoc::indoc;

    fn setup(s: &str) -> Vec<Token> {
        let tokens = tokenize(s).expect("Error in tokenizing input");
        let mut lexer = Lexer::new(tokens);
        let tokens = lexer.all();
        tokens
    }

    #[test]
    fn brackets_and_symbols() {
        let tokens = setup("( hello ) mate");
        let ans = vec![
            Token::LParen,
            Token::Symbol("hello".to_string()),
            Token::RParen,
            Token::Symbol("mate".to_string()),
        ];
        assert_eq!(ans, tokens);
    }

    #[test]
    fn assigment() {
        let tokens = setup("x = 100");
        let ans = vec![
            Token::Symbol("x".to_string()),
            Token::Assign,
            Token::Float(100.0),
        ];
        assert_eq!(ans, tokens);
    }

    #[test]
    fn equals() {
        let tokens = setup("x != y");
        let ans = vec![
            Token::Symbol("x".to_string()),
            Token::NotEqual,
            Token::Symbol("y".to_string()),
        ];
        assert_eq!(ans, tokens);
    }

    #[test]
    fn if_else_block() {
        let s = indoc!(
            "
            if x > y then
                x
            else
                y
            end
        "
        );
        let tokens = setup(s);
        let ans = vec![
            Token::If,
            Token::Symbol("x".to_string()),
            Token::GreaterThan,
            Token::Symbol("y".to_string()),
            Token::Then,
            Token::Symbol("x".to_string()),
            Token::Else,
            Token::Symbol("y".to_string()),
            Token::End,
        ];
        assert_eq!(ans, tokens);
    }

    #[test]
    fn elseif_block() {
        let s = indoc!(
            "
            if x > y then
                x
            elseif y > x then
                y
            end
        "
        );
        let tokens = setup(s);
        let ans = vec![
            Token::If,
            Token::Symbol("x".to_string()),
            Token::GreaterThan,
            Token::Symbol("y".to_string()),
            Token::Then,
            Token::Symbol("x".to_string()),
            Token::ElseIf,
            Token::Symbol("y".to_string()),
            Token::GreaterThan,
            Token::Symbol("x".to_string()),
            Token::Then,
            Token::Symbol("y".to_string()),
            Token::End,
        ];
        assert_eq!(ans, tokens);
    }
}
