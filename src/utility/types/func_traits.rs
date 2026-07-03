/* pub trait IntoFuncReturn {
    fn return_type() -> FuncReturnType;
    fn into_return_value(self) -> FuncReturnValue;
}
impl IntoFuncReturn for i32 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::Int32
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::Int32(self)
    }
}

impl IntoFuncReturn for f64 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::Float
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::Float(self)
    }
}

impl IntoFuncReturn for u32 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::UInt32
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::UInt32(self)
    }
}

impl IntoFuncReturn for i128 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::Int128
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::Int128(self)
    }
}
impl IntoFuncReturn for i8 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::Int8
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::Int8(self as i8)
    }
}
impl IntoFuncReturn for u8 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::UInt8
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::UInt8(self as u8)
    }
}
impl IntoFuncReturn for i16 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::Int16
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::Int16(self as i16)
    }
}
impl IntoFuncReturn for u16 {
    fn return_type() -> FuncReturnType {
        FuncReturnType::UInt16
    }
    fn into_return_value(self) -> FuncReturnValue {
        FuncReturnValue::UInt16(self as u16)
    }
}
 */
use crate::utility::types::errors::ErrType;
use crate::utility::types::func_return::{FuncReturnType, FuncReturnValue};

macro_rules! impl_into_func_return_int {
    ($($t:ty => $variant:ident),* $(,)?) => {
        $(
            impl IntoFuncReturn for $t {
                fn return_type() -> FuncReturnType {
                    FuncReturnType::Int(IntType::$variant)
                }
                fn into_return_value(self) -> FuncReturnValue {
                    FuncReturnValue::Int(IntValue::$variant(self))
                }
            }
        )*
    };
}

impl_into_func_return_int!(
    i8 => I8,
    u8 => U8,
    i16 => I16,
    u16 => U16,
    i32 => I32,
    u32 => U32,
    i128 => I128,
    f32 => F32,
    f64 => F64,
    f16 => F16,
);
