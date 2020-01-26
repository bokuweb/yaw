use crate::types::RuntimeValue;
use crate::vm::error::*;

pub trait Reinterpret<T> {
    fn reinterpret(self) -> T;
}

impl Reinterpret<f32> for i32 {
    fn reinterpret(self) -> f32 {
        f32::from_bits(self as u32)
    }
}

impl Reinterpret<f64> for i64 {
    fn reinterpret(self) -> f64 {
        f64::from_bits(self as u64)
    }
}

impl Reinterpret<i32> for f32 {
    fn reinterpret(self) -> i32 {
        self.to_bits() as i32
    }
}

impl Reinterpret<i64> for f64 {
    fn reinterpret(self) -> i64 {
        self.to_bits() as i64
    }
}

impl RuntimeValue {
    pub fn reinterpret(&self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::F32(l.reinterpret())),
            RuntimeValue::I64(l) => Ok(RuntimeValue::F64(l.reinterpret())),
            RuntimeValue::F32(l) => Ok(RuntimeValue::I32(l.reinterpret())),
            RuntimeValue::F64(l) => Ok(RuntimeValue::I64(l.reinterpret())),
            _ => unimplemented!(),
        }
    }
}
