use crate::atom::{Atom, Node, AST};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    fn nud(&mut self, token: Token) -> AST {
        match token {
            Token::False => AST::Atom(Atom::False),
            Token::True => AST::Atom(Atom::True),
            Token::Float(value) => AST::Atom(Atom::Float(value)),
            Token::LParen => {
                let inner = self._parse(token.precedence());
                self.lexer.expect(&Token::RParen);
                inner
            }
            Token::Plus => self._parse(token.precedence()),
            Token::Minus => {
                let tail = vec![self._parse(token.precedence())];
                let head = Atom::Symbol("-".to_string());
                let node = Node { head, tail };
                AST::Node(node)
            }

            Token::If => {
                let condition = self.parse();
                self.lexer.expect(&Token::Then);
                let block = self.parse_block();
                let next_token = self.lexer.peek(0);

                match next_token {
                    Token::End => {
                        self.lexer.next();
                        AST::Node(Node {
                            head: Atom::If,
                            tail: vec![condition, block, AST::Atom(Atom::Nothing)],
                        })
                    }
                    Token::Else | Token::ElseIf => AST::Node(Node {
                        head: Atom::If,
                        tail: vec![condition, block, self.parse()],
                    }),
                    _ => panic!("Unexpected token after if block {:?}", next_token),
                }
            }

            Token::ElseIf => {
                let elseif_condition = self.parse();
                self.lexer.expect(&Token::Then);
                let elseif_block = self.parse_block();

                let other_block = match self.lexer.peek(0) {
                    Token::End => {
                        self.lexer.next();
                        AST::Atom(Atom::Nothing)
                    }
                    Token::ElseIf | Token::Else => self.parse(),
                    token => panic!("Unexpected token after elseif {:?}", token),
                };

                AST::Node(Node {
                    head: Atom::ElseIf,
                    tail: vec![elseif_condition, elseif_block, other_block],
                })
            }

            Token::Else => {
                let ast = self.parse_block();
                self.lexer.expect(&Token::End);
                ast
            }
            Token::Symbol(value) => AST::Atom(Atom::Symbol(value)),
            _ => panic!("The token {:?} doesn't have a nud", token),
        }
    }

    fn led(&mut self, left: AST, token: Token) -> AST {
        match token {
            Token::Plus
            | Token::Minus
            | Token::Divide
            | Token::Multiply
            | Token::Assign
            | Token::GreaterThan
            | Token::LessThan
            | Token::Equal
            | Token::NotEqual => {
                let right = self._parse(token.precedence());
                let head = token.to_atom();
                let node = Node {
                    head,
                    tail: vec![left, right],
                };
                AST::Node(node)
            }
            Token::Power => {
                let right = self._parse(token.precedence() - 1);
                let head = token.to_atom();
                let node = Node {
                    head,
                    tail: vec![left, right],
                };
                AST::Node(node)
            }
            token => panic!("The token {:?} has no led implemented", token),
        }
    }

    pub fn parse_block(&mut self) -> AST {
        let head = Atom::Block;
        let mut tail = vec![];

        loop {
            match self.lexer.peek(0) {
                Token::EOF | Token::End | Token::Else | Token::ElseIf => break,
                _ => (),
            }

            let ast = self.parse();
            tail.push(ast);
        }
        AST::Node(Node { head, tail })
    }

    pub fn parse(&mut self) -> AST {
        self._parse(0)
    }

    fn _parse(&mut self, precedence: isize) -> AST {
        let mut token = self.lexer.next();
        let mut left = self.nud(token);
        while self.lexer.peek(0).precedence() > precedence {
            token = self.lexer.next();
            if token == Token::EOF {
                break;;
            }
            left = self.led(left, token);
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::tokenizer::tokenize;
    use indoc::indoc;

    #[test]
    fn plus() {
        let tokens = tokenize("1 + 2").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let ans = AST::Node(Node {
            head: Atom::Plus,
            tail: vec![AST::Atom(Atom::Float(1.0)), AST::Atom(Atom::Float(2.0))],
        });
        assert_eq!(result, ans);
    }

    #[test]
    fn muti() {
        let tokens = tokenize("3 + 2 * 10").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let n1 = AST::Node(Node {
            head: Atom::Multiply,
            tail: vec![AST::Atom(Atom::Float(2.0)), AST::Atom(Atom::Float(10.0))],
        });

        let n2 = AST::Node(Node {
            head: Atom::Plus,
            tail: vec![AST::Atom(Atom::Float(3.0)), n1],
        });

        assert_eq!(result, n2);
    }

    #[test]
    fn power() {
        let tokens = tokenize("3 ^ 2 * 10").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let n1 = AST::Node(Node {
            head: Atom::Power,
            tail: vec![AST::Atom(Atom::Float(3.0)), AST::Atom(Atom::Float(2.0))],
        });

        let n2 = AST::Node(Node {
            head: Atom::Multiply,
            tail: vec![n1, AST::Atom(Atom::Float(10.0))],
        });

        assert_eq!(result, n2);
    }

    #[test]
    fn brackets() {
        let tokens = tokenize("3 ^ (2 * 10)").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let n1 = AST::Node(Node {
            head: Atom::Multiply,
            tail: vec![AST::Atom(Atom::Float(2.0)), AST::Atom(Atom::Float(10.0))],
        });

        let n2 = AST::Node(Node {
            head: Atom::Power,
            tail: vec![AST::Atom(Atom::Float(3.0)), n1],
        });

        assert_eq!(result, n2);
    }

    #[test]
    fn assignment() {
        let tokens = tokenize("x = 10").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let ans = AST::Node(Node {
            head: Atom::Assign,
            tail: vec![
                AST::Atom(Atom::Symbol("x".to_string())),
                AST::Atom(Atom::Float(10.0)),
            ],
        });

        assert_eq!(result, ans);
    }

    #[test]
    fn equals() {
        let tokens = tokenize("x == 10").unwrap();
        let mut lexer = Lexer::new(tokens);
        println!("LEXER: {:?}", lexer.all());
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let ans = AST::Node(Node {
            head: Atom::Equal,
            tail: vec![
                AST::Atom(Atom::Symbol("x".to_string())),
                AST::Atom(Atom::Float(10.0)),
            ],
        });

        assert_eq!(result, ans);
    }

    #[test]
    fn not_equals() {
        let tokens = tokenize("x != 10").unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let result = parser.parse();
        let ans = AST::Node(Node {
            head: Atom::NotEqual,
            tail: vec![
                AST::Atom(Atom::Symbol("x".to_string())),
                AST::Atom(Atom::Float(10.0)),
            ],
        });

        assert_eq!(result, ans);
    }

    #[test]
    fn parse_block_simple() {
        let s = indoc!(
            "
            y = 5
            y * 8
        "
        );

        let tokens = tokenize(s).unwrap();
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);

        let n1 = AST::Node(Node {
            head: Atom::Assign,
            tail: vec![
                AST::Atom(Atom::Symbol("y".to_string())),
                AST::Atom(Atom::Float(5.0)),
            ],
        });

        let n2 = AST::Node(Node {
            head: Atom::Multiply,
            tail: vec![
                AST::Atom(Atom::Symbol("y".to_string())),
                AST::Atom(Atom::Float(8.0)),
            ],
        });

        let result = parser.parse_block();
        let ans = AST::Node(Node {
            head: Atom::Block,
            tail: vec![n1, n2],
        });

        assert_eq!(result, ans);
    }
}
