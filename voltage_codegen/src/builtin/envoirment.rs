use std::collections::HashMap;

use voltage_ast::{statements::Statement, FuncParam, Type};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Envoirment {
    pub global_variables: HashMap<String, Value>,
}

impl Envoirment {
    pub fn set(&mut self, name: String, value: Value) {
        self.global_variables.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<&Value> {
        self.global_variables.get(&name)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Value {
    String {
        value: String,
    },
    Int {
        value: i64,
    },
    Float {
        value: f64,
    },
    Bool {
        value: bool,
    },
    Char {
        value: char,
    },
    Function {
        name: String,
        params: Vec<FuncParam>,
        body: Vec<Statement>,
        env: Option<Envoirment>,
        return_type: Type,
    },
}
