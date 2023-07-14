pub mod expressions;
pub mod statements;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub struct FuncParam {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum CmpOperators {
    Equal,
    NotEqual,
    GreaterThen,
    LessThen,
    GreaterThenOrEqual,
    LessThenOrEqual,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum Type {
    Char,
    Int8,
    Int16,
    Int32,
    Int64,
    Int,
    Float,
    Void,
    Unknown,
    Nil,
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "char" => Self::Char,
            "i8" => Self::Int8,
            "i16" => Self::Int16,
            "i32" => Self::Int32,
            "i64" => Self::Int64,
            "int" => Self::Int,
            "float" => Self::Float,
            "nil" | "null" => Self::Nil,
            "void" => Self::Void,
            _ => Self::Unknown,
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        todo!()
    }
}
