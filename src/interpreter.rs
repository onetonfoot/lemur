use crate::atom::{Atom, AST};
use crate::enviroment::Enviroment;
use crate::object::Object;

struct Interpreter {
    env: Enviroment,
}

impl Interpreter {
    fn new(env: Enviroment) -> Self {
        Interpreter { env }
    }

    //Should return a result to an object and remove panic / expects!
    fn eval(&mut self, node: &AST) -> Object {
        match node {
            AST::Node(node) => match &node.head {
                Atom::Plus | Atom::Minus | Atom::Divide | Atom::Multiply | Atom::Power => {
                    let right = self.eval(&node.tail[0]);
                    let left = self.eval(&node.tail[1]);
                    eval_math(&node.head, &left, &right)
                }

                Atom::Assign => {
                    let key = &node.tail[0];
                    let value = self.eval(&node.tail[1]);

                    let varname = match key {
                        AST::Atom(Atom::Symbol(k)) => {
                            self.env.insert(&k, value);
                            k
                        }
                        _ => panic!("Invalid key {:?}", key),
                    };
                    Object::Symbol(varname.to_string())
                }

                Atom::GreaterThan => {
                    let a = self.eval(&node.tail[0]);
                    let b = self.eval(&node.tail[1]);
                    eval_gt(&a, &b)
                }
                Atom::Block => {
                    let iter = node.tail.iter();
                    let n = iter.len() - 1;

                    for (i, ast) in iter.enumerate() {
                        if i == n {
                            return self.eval(ast);
                        }
                        self.eval(ast);
                    }

                    panic!("An error occured excuting a block");
                }
                Atom::If | Atom::ElseIf => match self.eval(&node.tail[0]) {
                    Object::Bool(true) => self.eval(&node.tail[1]),
                    Object::Bool(false) => self.eval(&node.tail[2]),
                    o => panic!("Non boolean value given to if statement {:?}", o),
                },
                op => panic!("Unknown operation {:?}", op),
            },
            AST::Atom(atom) => match atom {
                Atom::Float(value) => Object::Float(*value),
                Atom::Symbol(key) => {
                    let object = self
                        .env
                        .get(key)
                        .expect(&format!("Object with name {:? } doens't exists", key))
                        .clone();
                    //Here actually just need to return a string that is the graphical representation of the object...
                    object
                }
                Atom::False => Object::Bool(false),
                Atom::True => Object::Bool(true),
                Atom::Nothing => Object::Nothing,
                atom => panic!("Cannot interpret this atom {:?}", atom),
            },
        }
    }
}

fn eval_math(atom: &Atom, x: &Object, y: &Object) -> Object {
    let (x, y) = match (x, y) {
        (Object::Float(x), Object::Float(y)) => (x, y),
        (x, y) => panic!("Cannot evaluate maths with {:?} {:?}", x, y),
    };

    match atom {
        Atom::Plus => Object::Float(x + y),
        Atom::Minus => Object::Float(x - y),
        Atom::Divide => Object::Float(x / y),
        Atom::Multiply => Object::Float(x * y),
        Atom::Power => Object::Float(y.powf(*x)),
        _ => panic!("Atom {:?} has math no operation implemented", atom),
    }
}

fn eval_gt(a: &Object, b: &Object) -> Object {
    match (a, b) {
        (Object::Float(x), Object::Float(y)) => Object::Bool(x > y),
        _ => panic!(
            "Greater than not implemented for objects of type {:?} and {:?}",
            a, b
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::tokenizer::tokenize;

    fn setup(s: &str) -> (Interpreter, AST) {
        //Stage 1
        let tokens = tokenize(s).expect("Setup failed to tokenize the string");
        let lexer = Lexer::new(tokens);
        //Stage 2
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        //Stage 3
        let env = Enviroment::new();
        let interpreter = Interpreter::new(env);
        (interpreter, ast)
    }

    fn setup_block(s: &str) -> (Interpreter, AST) {
        //Stage 1
        let tokens = tokenize(s).expect("Setup failed to tokenize the string");
        let lexer = Lexer::new(tokens);
        //Stage 2
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_block();
        //Stage 3
        let env = Enviroment::new();
        let interpreter = Interpreter::new(env);
        (interpreter, ast)
    }

    #[test]
    fn power() {
        let (mut interpreter, ast) = setup("3 ^ 2 * 10");
        let result = interpreter.eval(&ast);
        let ans = Object::Float(90.0);
        assert_eq!(result, ans);
    }

    #[test]
    fn booleans() {
        let (mut interpreter, ast) = setup("false");
        let result = interpreter.eval(&ast);
        let ans = Object::Bool(false);
        assert_eq!(result, ans);
    }

    #[test]
    fn assignment() {
        let (mut interpreter, ast) = setup("x = 10");
        interpreter.eval(&ast);
        let val = interpreter
            .env
            .get("x")
            .expect(&format!("No value x in env: {:?}", interpreter.env));
        let ans = Object::Float(10.0);
        assert_eq!(val, &ans);
    }

    #[test]
    fn gt() {
        let (mut interpreter, ast) = setup("5 > 10");
        let res = interpreter.eval(&ast);
        let ans = Object::Bool(false);
        assert_eq!(&res, &ans);
    }

    #[test]
    fn execute_block() {
        let (mut interpreter, ast) = setup_block(
            "
        x = 10
        y = 11
        y > x
        ",
        );

        let res = interpreter.eval(&ast);
        let ans = Object::Bool(true);
        assert_eq!(&res, &ans);
    }

    #[test]
    fn if_statement() {
        let (mut interpreter, ast) = setup_block(
            "
        x = 10
        if x > 5 then
            y = 2
        end
        ",
        );

        let res = interpreter.eval(&ast);

        let val = interpreter
            .env
            .get("y")
            .expect(&format!("No value y in env: {:?}", interpreter.env));
        assert_eq!(val, &Object::Float(2.0));
        assert_eq!(&res, &Object::Symbol("y".to_string()));
    }

    #[test]
    fn if_else_statement() {
        let (mut interpreter, ast) = setup_block(
            "
        x = 1
        if x > 5 then
            y = 2
        else
            y = 10
        end
        ",
        );

        let res = interpreter.eval(&ast);

        let val = interpreter
            .env
            .get("y")
            .expect(&format!("No value y in env: {:?}", interpreter.env));
        assert_eq!(val, &Object::Float(10.0));
        assert_eq!(&res, &Object::Symbol("y".to_string()));
    }

    #[test]
    fn elseif_statement() {
        let (mut interpreter, ast) = setup_block(
            "
        if false then
            10
        elseif true then
            20
        end
        ",
        );

        println!("AST {:?}", ast);

        let res = interpreter.eval(&ast);
        assert_eq!(&res, &Object::Float(20.0));
    }

    #[test]
    fn if_else_elseif_statement() {
        let (mut interpreter, ast) = setup_block(
            "
        if 0 > 5 then
            10
        elseif 1 > 2 then
            20
        else
            30
        end
        ",
        );

        let res = interpreter.eval(&ast);
        assert_eq!(&res, &Object::Float(30.0));
    }
}
