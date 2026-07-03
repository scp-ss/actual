use crate::utility::types::errors::ErrType;
use crate::utility::types::func_return::{FuncReturnType, FuncReturnValue};

// ── Arg TYPE (declared shape of an argument) ──
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArgType {
    Int32,
    Float,
    String,
    Bool,
    Custom(String),
    Tuple(Vec<FuncArgType>),
    Array(Box<FuncArgType>, usize),
    Vec(Box<FuncArgType>),
}

// ── Arg VALUE (actual runtime argument data) ──
#[derive(Debug, Clone, PartialEq)]
pub enum FuncArgValue {
    Int32(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Custom(String),
    Tuple(Vec<FuncArgValue>),
    Array(Vec<FuncArgValue>),
    Vec(Vec<FuncArgValue>),
}

impl FuncArgValue {
    /// Derives the FuncArgType that matches this value — keeps type & value
    /// from ever disagreeing, since the type is computed, not stored twice.
    pub fn arg_type(&self) -> FuncArgType {
        match self {
            FuncArgValue::Int32(_) => FuncArgType::Int32,
            FuncArgValue::Float(_) => FuncArgType::Float,
            FuncArgValue::String(_) => FuncArgType::String,
            FuncArgValue::Bool(_) => FuncArgType::Bool,
            FuncArgValue::Custom(s) => FuncArgType::Custom(s.clone()),
            FuncArgValue::Tuple(items) => FuncArgType::Tuple(
                items
                    .iter()
                    .map(|v| v.arg_type())
                    .collect(),
            ),
            FuncArgValue::Array(items) => {
                let elem_type = items
                    .first()
                    .map(|v| v.arg_type())
                    .unwrap_or(FuncArgType::Custom("empty".to_string()));
                FuncArgType::Array(Box::new(elem_type), items.len())
            }
            FuncArgValue::Vec(items) => {
                let elem_type = items
                    .first()
                    .map(|v| v.arg_type())
                    .unwrap_or(FuncArgType::Custom("empty".to_string()));
                FuncArgType::Vec(Box::new(elem_type))
            }
        }
    }
}

// ── The function record itself ──
#[derive(Debug, Clone, PartialEq)]
pub struct FuncIdentifier {
    pub name: String,
    pub return_type: FuncReturnType,
    pub return_value: Result<FuncReturnValue, ErrType>,
    pub args: Vec<FuncArgValue>,
}

impl FuncIdentifier {
    pub fn validate(&self) -> Result<(), String> {
        let value = match &self.return_value {
            Ok(v) => v,
            Err(e) => return Err(format!("Function '{}' failed: {e}", self.name)),
        };

        let matches = match (&self.return_type, value) {
            (FuncReturnType::Void, FuncReturnValue::Void) => true,
            (FuncReturnType::Int32, FuncReturnValue::Int32(_)) => true,
            (FuncReturnType::UInt32, FuncReturnValue::UInt32(_)) => true,
            (FuncReturnType::Int128, FuncReturnValue::Int128(_)) => true,
            (FuncReturnType::Float, FuncReturnValue::Float(_)) => true,
            (FuncReturnType::String, FuncReturnValue::String(_)) => true,
            (FuncReturnType::Bool, FuncReturnValue::Bool(_)) => true,
            (FuncReturnType::Int32Vec, FuncReturnValue::Int32Vec(_)) => true,
            (FuncReturnType::Float64Vec, FuncReturnValue::Float64Vec(_)) => true,
            (FuncReturnType::Custom(t), FuncReturnValue::Custom(v)) => t == v,
            _ => false,
        };

        if matches {
            Ok(())
        } else {
            Err(format!(
                "Type mismatch in '{}': declared {:?}, got {:?}",
                self.name, self.return_type, value
            ))
        }
    }

    /// Returns the declared arg types, derived from the actual arg values.
    pub fn arg_types(&self) -> Vec<FuncArgType> {
        self.args
            .iter()
            .map(|v| v.arg_type())
            .collect()
    }
}
