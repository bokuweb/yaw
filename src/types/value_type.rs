use super::runtime_value::*;
use num_derive::*;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ValueType {
  I32 = 0x7F,
  I64 = 0x7E,
  F32 = 0x7D,
  F64 = 0x7C,
}

impl From<ValueType> for RuntimeValue {
  fn from(v: ValueType) -> RuntimeValue {
    match v {
      ValueType::I32 => RuntimeValue::I32(0),
      ValueType::I64 => RuntimeValue::I64(0),
      ValueType::F32 => RuntimeValue::F32(0.0),
      ValueType::F64 => RuntimeValue::F64(0.0),
    }
  }
}
