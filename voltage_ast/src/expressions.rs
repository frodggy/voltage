use crate::Operator;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    StringLiteral { val: String },
    Identifier { val: String },
    IntLiteral { val: i64 },
    BooleanLiteral { val: bool },
    FloatLiteral { val: f64 },
    CharLiteral { val: char },
    
    FunctionCall { name: String, params: Vec<Expression> },
    BinaryExpr {
        op: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    UnaryExpr {
        op: Operator,
        child: Box<Expression>,
    },
}