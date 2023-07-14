use crate::{expressions::Expression, Type, FuncParam, CmpOperators};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
        r#type: Type,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<FuncParam>,
        body: Vec<Statement>,
        return_type: Type,
    },
    IfStatement {
        expr1: Expression,
        cmp_op: CmpOperators,
        expr2: Expression
    }
}
