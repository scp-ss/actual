#[derive(Debug, Clone, PartialEq)]
pub enum NoneReason {
    NotFound(String),
    EmptyInput,
    OutOfBounds(String),
    ConditionNeverMet(String),
    Unknown(String),
    StdIo(String),
    StdParse(String),
}

impl std::fmt::Display for NoneReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NoneReason::NotFound(what) => write!(f, "not found: {what}"),
            NoneReason::EmptyInput => write!(f, "input was empty"),
            NoneReason::OutOfBounds(msg) => write!(f, "out of bounds: {msg}"),
            NoneReason::ConditionNeverMet(cond) => write!(f, "condition never met: {cond}"),
            NoneReason::Unknown(msg) => write!(f, "unknown reason: {msg}"),
            NoneReason::StdIo(msg) => write!(f, "I/O error: {msg}"),
            NoneReason::StdParse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrType {
    DivideByZero,
    FileMissing(String),
    TypeMismatch(String),
    TypeConversion { from: String, to: String },
    NoneReason(NoneReason),
    Custom(String),
}

impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrType::DivideByZero => write!(f, "cannot divide by zero"),
            ErrType::FileMissing(path) => write!(f, "file missing: {path}"),
            ErrType::TypeMismatch(msg) => write!(f, "type mismatch: {msg}"),
            ErrType::TypeConversion { from, to } => write!(f, "failed to convert {from} to {to}"),
            ErrType::NoneReason(reason) => write!(f, "{reason}"),
            ErrType::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<std::io::Error> for ErrType {
    fn from(e: std::io::Error) -> Self {
        ErrType::NoneReason(NoneReason::StdIo(e.to_string()))
    }
}

impl From<std::num::ParseIntError> for ErrType {
    fn from(e: std::num::ParseIntError) -> Self {
        ErrType::NoneReason(NoneReason::StdParse(e.to_string()))
    }
}
