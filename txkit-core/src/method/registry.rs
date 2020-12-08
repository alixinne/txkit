//! Definition of the method registry type

use std::collections::HashMap;

use super::Method;

/// Type of a method constructor
pub type MethodConstructor = Box<dyn Fn() -> Box<dyn Method>>;

/// Registry to declare methods by name
#[derive(Default)]
pub struct MethodRegistry {
    method_constructors: HashMap<String, MethodConstructor>,
}

impl MethodRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &str, constructor: MethodConstructor) {
        self.method_constructors
            .insert(name.to_string(), constructor);
    }

    pub fn build(&self, name: &str) -> Option<Box<dyn Method>> {
        self.method_constructors.get(name).and_then(|v| Some(v()))
    }
}
