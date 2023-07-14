use voltage_ast::{expressions::Expression, statements::Statement, Operator};

use self::envoirment::{Envoirment, Value};

mod envoirment;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Engine {
    pub env: Envoirment,
}

impl Engine {
    pub fn new() -> Engine {
        println!("Running with Built in interptor");
        Self {
            env: Envoirment::default(),
        }
    }

    pub fn exectute(&mut self, ast: Vec<Statement>) -> String {
        for statement in ast {
            self.run_statement(statement, None)
        };
        #[cfg(feature = "json_abi")]
        return serde_json::to_string_pretty(&self).unwrap();
        #[cfg(not(feature = "json_abi"))]
        return String::new()
    }

    pub fn run_statement(&mut self, statement: Statement, function: Option<&mut Envoirment>) {
        match statement {
            Statement::VariableDeclaration {
                name,
                value,
                r#type,
            } => {
                // type check
                // convert expression to value
                let value = self.expression_to_value(value);
                // instert to global variables
                match function {
                    Some(env) => env.set(name, value),
                    None => self.env.set(name, value),
                }
            }
            Statement::FunctionDeclaration {
                name,
                params,
                body,
                return_type,
            } => {
                let mut env = Envoirment::default();
                for statement in body.clone() {
                    self.run_statement(statement, Some(&mut env))
                }
                self.env.set(
                name.clone(),
                Value::Function {
                    name,
                    params,
                    body,
                    env: Some(env),
                    return_type,
                })
            },
        }
    }

    pub fn expression_to_value(&mut self, expr: Expression) -> Value {
        let value = match expr {
            voltage_ast::expressions::Expression::StringLiteral { val } => {
                Value::String { value: val }
            }
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val).cloned() {
                    Some(value) => value,
                    None => todo!(),
                }
            }
            voltage_ast::expressions::Expression::IntLiteral { val } => Value::Int { value: val },
            voltage_ast::expressions::Expression::BooleanLiteral { val } => {
                Value::Bool { value: val }
            }
            voltage_ast::expressions::Expression::FloatLiteral { val } => {
                Value::Float { value: val }
            }
            voltage_ast::expressions::Expression::CharLiteral { val } => Value::Char { value: val },
            voltage_ast::expressions::Expression::FunctionCall { name, params } => {
                unimplemented!()
            }
            voltage_ast::expressions::Expression::BinaryExpr { op, lhs, rhs } => {
                self.run_binary_op(lhs, op, rhs)
            }
            voltage_ast::expressions::Expression::UnaryExpr { op, child } => todo!(),
        };

        value
    }

    pub fn run_binary_op(&self, lhs: Box<Expression>, op: Operator, rhs: Box<Expression>) -> Value {
        let lhs = match unbox(lhs) {
            Expression::IntLiteral { val } => Value::Int { value: val },
            Expression::FloatLiteral { val } => Value::Float { value: val },
            Expression::BinaryExpr { op, lhs, rhs } => self.run_binary_op(lhs, op, rhs),
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val.clone()).cloned() {
                    Some(value) => value,
                    None => panic!("No variable {val} found"),
                }
            }
            x => panic!("Can not add {:?}", x),
        };
        let rhs = match unbox(rhs) {
            Expression::IntLiteral { val } => Value::Int { value: val },
            Expression::FloatLiteral { val } => Value::Float { value: val },
            Expression::BinaryExpr { op, lhs, rhs } => self.run_binary_op(lhs, op, rhs),
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val.clone()).cloned() {
                    Some(value) => value,
                    None => panic!("No variable {val} found"),
                }
            }
            x => panic!("Can not add {:?}", x),
        };

        match op {
            // Add
            voltage_ast::Operator::Plus => {
                if matches!(lhs, Value::Int { .. }) && matches!(rhs, Value::Int { .. }) {
                    let mut x = 0;
                    let mut y = 0;
                    if let Value::Int { value } = lhs {
                        x = value
                    }
                    if let Value::Int { value } = rhs {
                        y = value;
                    }
                    return Value::Int { value: x + y };
                } else if matches!(lhs, Value::Float { .. }) && matches!(rhs, Value::Float { .. }) {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    if let Value::Float { value } = lhs {
                        x = value
                    }
                    if let Value::Float { value } = rhs {
                        y = value;
                    }
                    return Value::Float { value: x + y };
                } else {
                    panic!("Can not add {:?} with {:?}", lhs, rhs)
                }
            }
            // Subtract
            voltage_ast::Operator::Minus => {
                if matches!(lhs, Value::Int { .. }) && matches!(rhs, Value::Int { .. }) {
                    let mut x = 0;
                    let mut y = 0;
                    if let Value::Int { value } = lhs {
                        x = value
                    }
                    if let Value::Int { value } = rhs {
                        y = value;
                    }
                    return Value::Int { value: x - y };
                } else if matches!(lhs, Value::Float { .. }) && matches!(rhs, Value::Float { .. }) {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    if let Value::Float { value } = lhs {
                        x = value
                    }
                    if let Value::Float { value } = rhs {
                        y = value;
                    }
                    return Value::Float { value: x - y };
                } else {
                    panic!("Can not subtract {:?} with {:?}", lhs, rhs)
                }
            }
            // Multiply
            voltage_ast::Operator::Multiplication => {
                if matches!(lhs, Value::Int { .. }) && matches!(rhs, Value::Int { .. }) {
                    let mut x = 0;
                    let mut y = 0;
                    if let Value::Int { value } = lhs {
                        x = value
                    }
                    if let Value::Int { value } = rhs {
                        y = value;
                    }
                    return Value::Int { value: x * y };
                } else if matches!(lhs, Value::Float { .. }) && matches!(rhs, Value::Float { .. }) {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    if let Value::Float { value } = lhs {
                        x = value
                    }
                    if let Value::Float { value } = rhs {
                        y = value;
                    }
                    return Value::Float { value: x * y };
                } else {
                    panic!("Can not multiply {:?} with {:?}", lhs, rhs)
                }
            }
            // Divide
            voltage_ast::Operator::Division => {
                if matches!(lhs, Value::Int { .. }) && matches!(rhs, Value::Int { .. }) {
                    let mut x = 0;
                    let mut y = 0;
                    if let Value::Int { value } = lhs {
                        x = value
                    }
                    if let Value::Int { value } = rhs {
                        y = value;
                    }
                    return Value::Int { value: x / y };
                } else if matches!(lhs, Value::Float { .. }) && matches!(rhs, Value::Float { .. }) {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    if let Value::Float { value } = lhs {
                        x = value
                    }
                    if let Value::Float { value } = rhs {
                        y = value;
                    }
                    return Value::Float { value: x / y };
                } else {
                    panic!("Can not divide {:?} with {:?}", lhs, rhs)
                }
            }
        }
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}
