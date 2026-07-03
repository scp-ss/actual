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
