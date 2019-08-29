#[derive(PartialEq, Debug)]
pub struct Symbol(pub String);

pub enum Keyword {
    If,
    Else,
    Fn,
}

#[derive(PartialEq, Debug)]
pub enum Atom {
    //Literals
    Symbol(String),
    String(String),
    Float(f64),
    True,
    False,
    //Maths
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    //Logic
    If,
    Else,
    ElseIf,
    Equal,
    NotEqual,
    Not,
    GreaterThan,
    LessThan,
    //Semanitcs
    Assign,
    Block,
    Nothing,
    End,
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pub head: Atom,
    pub tail: Vec<AST>,
}

#[derive(PartialEq, Debug)]
pub enum AST {
    Node(Node),
    Atom(Atom),
}
