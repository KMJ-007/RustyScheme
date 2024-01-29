use crate::parser::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    // will store variables here
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            // if asked var is found in the same scope, then return the same copy
            Some(value) => Some(value.clone()),
            // if we didn't find any value in local scope then check the parent scope recursively
            None => self
                .parent
                .as_ref()
                .and_then(|parent_env| parent_env.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
