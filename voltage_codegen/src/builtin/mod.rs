use std::collections::HashMap;

use voltage_ast::{expressions::Expression, statements::Statement, Operator};

use self::envoirment::{Envoirment, FunctionType, Value};

mod envoirment;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Engine {
    pub env: Envoirment,
    pub globals: HashMap<String, Value>,
}

impl Engine {
    pub fn new() -> Engine {
        println!("Running with Built in interptor");
        Self {
            env: Envoirment::default(),
            globals: HashMap::new(),
        }
    }

    pub fn exectute(&mut self, ast: Vec<Statement>) -> String {
        for statement in ast {
            self.run_statement(statement, None)
        }
        #[cfg(feature = "json_abi")]
        return serde_json::to_string_pretty(&self).unwrap();
        #[cfg(not(feature = "json_abi"))]
        return String::new();
    }

    pub fn run_statement(&mut self, statement: Statement, external_env: Option<&mut Envoirment>) {
        match statement {
            Statement::VariableDeclaration {
                name,
                value,
                r#type,
            } => {
                // type check
                // convert expression to value
                let value = self.expression_to_value(value, None).unwrap();
                println!("{value:?}");
                // instert to global variables
                match external_env {
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
                let env = Envoirment::default();
                // for statement in body.clone() {
                //     self.run_statement(statement, Some(&mut env))
                // }
                self.globals.insert(
                    name.clone(),
                    Value::Function {
                        name,
                        params,
                        r#type: FunctionType::Function,
                        body,
                        env: Some(env),
                        return_type,
                    },
                );
            }
            Statement::IfStatement {
                expr1,
                cmp_op,
                expr2,
                body,
            } => {
                let mut env = Envoirment {
                    global_variables: HashMap::new(),
                    r#return: Some(Box::new(Value::Null)),
                };

                let lhs;
                let rhs;

                match external_env {
                    Some(external_env) => {
                        lhs = self.expression_to_value(expr1, Some(external_env));
                        rhs = self.expression_to_value(expr2, Some(external_env));
                    }
                    None => {
                        lhs = self.expression_to_value(expr1, None);
                        rhs = self.expression_to_value(expr2, None);
                    }
                }

                match cmp_op {
                    voltage_ast::CmpOperators::Equal => {
                        if lhs == rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                    voltage_ast::CmpOperators::NotEqual => {
                        if lhs != rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                    voltage_ast::CmpOperators::GreaterThen => {
                        if lhs > rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                    voltage_ast::CmpOperators::LessThen => {
                        if lhs < rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                    voltage_ast::CmpOperators::GreaterThenOrEqual => {
                        if lhs != rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                    voltage_ast::CmpOperators::LessThenOrEqual => {
                        if lhs != rhs {
                            for statement in body {
                                self.run_statement(statement, Some(&mut env))
                            }
                        }
                    }
                }

                println!("{:?} {:?}", lhs, rhs);
            }
            Statement::Return { value } => {
                let env = external_env.unwrap();
                let ret = self.expression_to_value(value, Some(env)).unwrap();

                env.r#return = Some(Box::new(ret));
            }
            Statement::ExprStatement { expr } => {
                self.expression_to_value(expr, external_env);
            }
        }
    }

    pub fn expression_to_value(
        &mut self,
        expr: Expression,
        external_env: Option<&mut Envoirment>,
    ) -> Option<Value> {
        let value = match expr {
            voltage_ast::expressions::Expression::StringLiteral { val } => {
                Value::String { value: val }
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
                let func_params = params;

                let handle_get_value =
                    || match self.expression_to_value(*name.clone(), external_env) {
                        Some(value) => value,
                        None => self
                            .expression_to_value(
                                *name,
                                Some(&mut Envoirment {
                                    global_variables: self.globals.clone(),
                                    r#return: None,
                                }),
                            )
                            .unwrap(),
                    };

                if let Value::Function {
                    name,
                    params,
                    body,
                    mut env,
                    return_type,
                    r#type,
                } = handle_get_value()
                {
                    if name.ends_with("$") && matches!(r#type, FunctionType::Native) {
                        return Some(Value::Null);
                    } else {

                        for (k, _) in env.clone().unwrap().global_variables {
                            env.as_mut().unwrap().global_variables.remove(&k).unwrap();
                        }

                        match env.as_mut().unwrap().r#return {
                            Some(_) => env.as_mut().unwrap().r#return = None,
                            None => env.as_mut().unwrap().r#return = None,
                        }

                        let mut true_params: Vec<Value> = vec![];
                        for param in func_params {
                            true_params
                                .push(self.expression_to_value(param, env.as_mut()).unwrap());
                        }

                        let mut idx = 0;
                        for param in params {
                            env.as_mut()
                                .unwrap()
                                .set(param.name, true_params[idx].clone());

                            idx += 1;
                        }

                        for statement in body {
                            self.run_statement(statement, env.as_mut());
                        }

                        println!("{:?}", env.clone());

                        for (k, _) in env.clone().unwrap().global_variables {
                            env.as_mut().unwrap().global_variables.remove(&k).unwrap();
                        }

                        println!("{:?}", env.clone());

                        return Some(*env.unwrap().r#return.unwrap());
                    }
                };

                Value::Null
            }
            voltage_ast::expressions::Expression::BinaryExpr { op, lhs, rhs } => {
                self.run_binary_op(lhs, op, rhs, external_env)
            }
            voltage_ast::expressions::Expression::UnaryExpr { op, child } => todo!(),
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val.clone()).cloned() {
                    Some(value) => value,
                    None => match external_env {
                        Some(env) => match env.get(val.clone()).cloned() {
                            Some(val) => val,
                            None => panic!("Variable {} not found", val.clone()),
                        },
                        None => return None,
                    },
                }
            }
        };

        Some(value)
    }

    pub fn run_binary_op(
        &self,
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
        external_env: Option<&mut Envoirment>,
    ) -> Value {
        let lhs = match unbox(lhs) {
            Expression::IntLiteral { val } => Value::Int { value: val },
            Expression::FloatLiteral { val } => Value::Float { value: val },
            Expression::BinaryExpr { op, lhs, rhs } => self.run_binary_op(lhs, op, rhs, None),
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val.clone()).cloned() {
                    Some(value) => value,
                    None => match external_env.as_ref().unwrap().get(val.clone()) {
                        Some(value) => value,
                        None => panic!("No variable {val} found"),
                    }
                    .clone(),
                }
            }
            x => panic!("Can not add {:?}", x),
        };
        let rhs = match unbox(rhs) {
            Expression::IntLiteral { val } => Value::Int { value: val },
            Expression::FloatLiteral { val } => Value::Float { value: val },
            Expression::BinaryExpr { op, lhs, rhs } => self.run_binary_op(lhs, op, rhs, None),
            voltage_ast::expressions::Expression::Identifier { val } => {
                match self.env.get(val.clone()).cloned() {
                    Some(value) => value,
                    None => match external_env.cloned().unwrap().get(val.clone()) {
                        Some(value) => value,
                        None => panic!("No variable {val} found"),
                    }
                    .clone(),
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
