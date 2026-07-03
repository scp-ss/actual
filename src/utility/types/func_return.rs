use crate::utility::types::errors::ErrType;

#[derive(Debug, Clone, PartialEq)]
pub enum FuncReturnType {
    Void,
    Int(IntValue),
    UInt32,
    Int128,
    Float,
    String,
    Bool,
    Int32Vec,
    Float64Vec,
    Custom(String),
    Optional(Box<FuncReturnType>),
    Fallible(Box<FuncReturnType>, ErrType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncReturnValue {
    Void,
    Int32(i32),
    UInt32(u32),
    Int128(i128),
    Float(f64),
    String(String),
    Bool(bool),
    Int32Vec(Vec<i32>),
    Float64Vec(Vec<f64>),
    Custom(String),
}
