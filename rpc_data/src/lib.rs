use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType{
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Array(Vec<DataType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MathOp{
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodCall{
    pub method: MathOp,
    pub params: Vec<DataType>,
}

impl MethodCall{
    pub fn new(method: MathOp, params: Vec<DataType>) -> MethodCall{
        MethodCall{
            method,
            params,
        }
    }
}