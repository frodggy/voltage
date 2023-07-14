use voltage_ast::{
    expressions::Expression, statements::Statement, CmpOperators, FuncParam, Operator, Type,
};
use voltage_lexer::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input_token: Vec<Token>) -> Self {
        Self {
            tokens: input_token,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut ast = vec![];

        while let Some(statement) = self.parse_statement() {
            ast.push(statement)
        }

        ast
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.next_token() {
            Some(Token::Let) => {
                let identifier = match self.next_token() {
                    Some(id) => match id {
                        Token::Identifier { val } => String::from_iter(val),
                        _ => panic!("Expected identifier"),
                    },
                    None => panic!(),
                };

                if !matches!(self.peak_next_token(), Some(Token::Colon { .. })) {
                    panic!("Expected ':' for type declartion");
                }

                self.next_token();

                let raw_type = match self.next_token() {
                    Some(id) => match id {
                        Token::Identifier { val } => String::from_iter(val),
                        _ => panic!("Expected identifier"),
                    },
                    None => panic!(),
                };

                let r#type = Type::from(raw_type.as_str());

                if !matches!(self.peak_next_token(), Some(Token::Assign { .. })) {
                    panic!("Expected '=' for assignment operator");
                }

                self.next_token();

                let value = self.parse_expression(0).unwrap();

                return Some(Statement::VariableDeclaration {
                    name: identifier,
                    value,
                    r#type,
                });
            }
            Some(Token::Function) => {
                let mut params: Vec<FuncParam> = vec![];

                let identifier = match self.next_token() {
                    Some(id) => match id {
                        Token::Identifier { val } => String::from_iter(val),
                        _ => panic!("Expected identifier"),
                    },
                    None => panic!(),
                };

                if !matches!(self.peak_next_token(), Some(Token::LParen { .. })) {
                    panic!("")
                }

                self.next_token();

                while !matches!(self.peak_next_token(), Some(Token::RParen { .. })) {
                    let curr_token = self.next_token();

                    let comma = matches!(curr_token, Some(Token::Comma { .. }));

                    let identifier = if comma {
                        match self.next_token() {
                            Some(id) => match id {
                                Token::Identifier { val } => String::from_iter(val),
                                x => panic!("Expected identifier found {:?}", x),
                            },
                            None => panic!(),
                        }
                    } else {
                        match curr_token {
                            Some(id) => match id {
                                Token::Identifier { val } => String::from_iter(val),
                                x => panic!("Expected identifier found {:?}", x),
                            },
                            None => panic!(),
                        }
                    };

                    if !matches!(self.peak_next_token(), Some(Token::Colon { .. })) {
                        panic!("Expected ':' for type declartion");
                    }

                    self.next_token();

                    let raw_type = match self.next_token() {
                        Some(id) => match id {
                            Token::Identifier { val } => String::from_iter(val),
                            _ => panic!("Expected identifier"),
                        },
                        None => panic!(),
                    };

                    let r#type = Type::from(raw_type.as_str());

                    params.push(FuncParam {
                        name: identifier,
                        r#type,
                    })
                }

                self.next_token();

                let raw_type = if !matches!(self.peak_next_token(), Some(Token::Colon { .. })) {
                    String::from("void")
                } else {
                    self.next_token();
                    match self.next_token() {
                        Some(id) => match id {
                            Token::Identifier { val } => String::from_iter(val),
                            _ => panic!("Expected identifier"),
                        },
                        None => panic!(),
                    }
                };

                let block = self.parse_block(Token::End);

                let r#type = raw_type.as_str();

                return Some(Statement::FunctionDeclaration {
                    name: identifier,
                    params,
                    body: block,
                    return_type: Type::from(r#type),
                });
            }
            Some(Token::If) => {
                let expr1 = self.parse_expression(0).unwrap();
                let cmp_op = self.parse_cmp_op();
                let expr2 = self.parse_expression(0).unwrap();

                if !matches!(self.peak_next_token(), Some(Token::LBrace { .. })) {
                    panic!("");
                }

                self.next_token();

                let body = self.parse_block(Token::RBrace { val: '}' });

                return Some(Statement::IfStatement {
                    expr1,
                    cmp_op,
                    expr2,
                    body,
                });
            }
            Some(Token::Return) => {
                let ret = self.parse_expression(0).unwrap();
                return Some(Statement::Return { value: ret });
            }
            _ => {
                let expr = match self.parse_expression(0) {
                    Some(expr) => expr,
                    None => return None,
                };
                return Some(Statement::ExprStatement { expr });
            }
        };
    }

    pub fn parse_cmp_op(&mut self) -> voltage_ast::CmpOperators {
        match self.next_token().unwrap() {
            Token::Lt { .. } => CmpOperators::LessThen,
            Token::LtOrEq { .. } => CmpOperators::LessThenOrEqual,
            Token::Gt { .. } => CmpOperators::GreaterThen,
            Token::GtOrEq { .. } => CmpOperators::GreaterThenOrEqual,
            Token::Eq { .. } => CmpOperators::Equal,
            Token::NotEq { .. } => CmpOperators::NotEqual,
            _ => panic!("Invalid comperasion operator"),
        }
    }

    pub fn parse_block(&mut self, delimiter: Token) -> Vec<Statement> {
        let mut block = vec![];

        loop {
            if self.peak_next_token() == Some(delimiter.clone()) {
                self.next_token();
                break;
            }

            block.push(self.parse_statement().unwrap());
        }

        block
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut tokens = self.tokens.clone().into_iter();

        let token = tokens.next().clone();

        let tokens_vec = tokens.collect::<Vec<Token>>();
        self.tokens = tokens_vec;

        token
    }

    pub fn peak_next_token(&mut self) -> Option<Token> {
        let mut tokens = self.tokens.iter();

        let token = match tokens.next().clone() {
            Some(token) => token,
            None => return None,
        };

        tokens.next_back();

        Some(token.clone())
    }

    pub fn forward(&mut self, times: usize) -> Option<Token> {
        let mut tokens = self.tokens.iter();

        let mut cur_time = 0;

        loop {
            if cur_time >= times - 1 {
                break;
            }

            tokens.next();

            cur_time += 1;
        }

        let token = match tokens.next().clone() {
            Some(token) => token,
            None => return None,
        };

        let mut cur_time = 0;

        loop {
            if cur_time >= times {
                break;
            }

            tokens.next_back();

            cur_time += 1;
        }

        Some(token.clone())
    }

    pub fn parse_expression(&mut self, bp: u8) -> Option<Expression> {
        let mut lhs = match self.next_token() {
            Some(Token::String { val }) => {
                let val: String = val.into_iter().collect();
                Expression::StringLiteral { val }
            }
            Some(Token::Identifier { val }) => {
                let val: String = val.into_iter().collect();
                Expression::Identifier { val }
            }
            Some(Token::Int { val }) => {
                let val: String = val.into_iter().collect();
                Expression::IntLiteral {
                    val: val.parse().unwrap(),
                }
            }
            Some(Token::Float { val }) => {
                let val: String = val.into_iter().collect();
                Expression::FloatLiteral {
                    val: val.parse().unwrap(),
                }
            }
            Some(Token::Char { val }) => Expression::CharLiteral { val },
            Some(Token::True) => Expression::BooleanLiteral { val: true },
            Some(Token::False) => Expression::BooleanLiteral { val: false },
            x => return None,
        };

        loop {
            let infix = if let Some(infix) = self.peak_next_token() {
                if matches!(self.forward(1), Some(Token::LParen { .. })) {
                    self.next_token();

                    let mut params: Vec<Expression> = vec![];

                    loop {
                        if self.peak_next_token() == Some(Token::RParen { val: ')' }) {
                            self.next_token();
                            break;
                        }

                        if self.peak_next_token() == Some(Token::Comma { val: ',' }) {
                            self.next_token();
                            params.push(self.parse_expression(0).unwrap());
                        } else {
                            params.push(self.parse_expression(0).unwrap());
                        }
                    }

                    lhs = Expression::FunctionCall {
                        name: Box::new(lhs),
                        params,
                    }
                }

                infix
            } else {
                break;
            };

            if let Some((lbp, rbp)) = infix_binding_power(infix) {
                if lbp < bp {
                    break;
                }

                let next_op = self.next_token().unwrap().clone();

                let rhs = self.parse_expression(rbp);

                lhs = make_infix_expr(lhs.clone(), next_op, rhs.unwrap());

                continue;
            }

            break;
        }

        Some(lhs)
    }
}

fn infix_binding_power(token: Token) -> Option<(u8, u8)> {
    let bp = match token {
        Token::Multiplication { .. } | Token::Division { .. } => (8, 9),
        Token::Plus { .. } | Token::Minus { .. } => (6, 7),
        _ => return None,
    };

    Some(bp)
}

fn make_infix_expr(lhs: Expression, op: Token, rhs: Expression) -> Expression {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    match op {
        Token::Plus { .. } => Expression::BinaryExpr {
            op: Operator::Plus,
            lhs,
            rhs,
        },
        Token::Multiplication { .. } => Expression::BinaryExpr {
            op: Operator::Multiplication,
            lhs,
            rhs,
        },
        Token::Minus { .. } => Expression::BinaryExpr {
            op: Operator::Minus,
            lhs,
            rhs,
        },
        Token::Division { .. } => Expression::BinaryExpr {
            op: Operator::Division,
            lhs,
            rhs,
        },
        _ => unimplemented!(),
    }
}
