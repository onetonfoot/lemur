use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    String(String),
    Symbol(String),
    Float(f64),
    Bool(bool),
    Nothing,
}

// impl fmt::Display for Object {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         Object
//     }
// }
