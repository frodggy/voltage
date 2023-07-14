use std::collections::HashMap;

use voltage_ast::{statements::Statement, FuncParam, Type};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Envoirment {
    pub global_variables: HashMap<String, Value>,
    pub r#return: Option<Box<Value>>,
}

impl Envoirment {
    pub fn set(&mut self, name: String, value: Value) {
        self.global_variables.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<&Value> {
        self.global_variables.get(&name)
    }
}

impl PartialOrd for Envoirment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let len: usize = self.global_variables.len();

        let mut cmp_to: Vec<usize> = Vec::with_capacity(len);
        cmp_to.fill(0);

        let mut cmp: Vec<usize> = Vec::new();

        for (k, v) in self.global_variables.clone() {
            if Some(&v) == other.get(k) {
                cmp.push(0);
            }
        };

        if cmp_to.len() == cmp.len(){
            cmp_to.partial_cmp(&cmp)
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum Value {
    Null,
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
        r#type: FunctionType, 
        params: Vec<FuncParam>,
        body: Vec<Statement>,
        env: Option<Envoirment>,
        return_type: Type,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum FunctionType {
    Native,
    Function
}
