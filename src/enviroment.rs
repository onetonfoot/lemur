use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Enviroment {
    parent: Option<Rc<Enviroment>>,
    state: HashMap<String, Object>,
}

impl Enviroment {
    pub fn new() -> Self {
        let state = HashMap::new();
        Enviroment {
            parent: None,
            state,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        match self.state.get(key) {
            Some(value) => Some(value),
            None => match &self.parent {
                None => None,
                Some(parent) => parent.get(key),
            },
        }
    }

    pub fn insert(&mut self, key: &str, value: Object) {
        self.state.insert(key.to_string(), value);
    }
}
