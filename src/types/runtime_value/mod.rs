mod comparisons;
mod conversions;
mod numeric;
mod reinterpret;

use super::ValueType;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RuntimeValue {
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  V128(u128),
}

impl From<RuntimeValue> for u32 {
  fn from(v: RuntimeValue) -> u32 {
    match v {
      RuntimeValue::I32(x) => x as u32,
      RuntimeValue::I64(x) => x as u32,
      RuntimeValue::F32(x) => x as u32,
      RuntimeValue::F64(x) => x as u32,
      RuntimeValue::V128(x) => x as u32,
    }
  }
}

impl From<RuntimeValue> for u64 {
  fn from(v: RuntimeValue) -> u64 {
    match v {
      RuntimeValue::I32(x) => x as u64,
      RuntimeValue::I64(x) => x as u64,
      RuntimeValue::F32(x) => x as u64,
      RuntimeValue::F64(x) => x as u64,
      RuntimeValue::V128(x) => x as u64,
    }
  }
}

impl From<RuntimeValue> for usize {
  fn from(v: RuntimeValue) -> usize {
    match v {
      RuntimeValue::I32(x) => x as usize,
      RuntimeValue::I64(x) => x as usize,
      RuntimeValue::F32(x) => x as usize,
      RuntimeValue::F64(x) => x as usize,
      RuntimeValue::V128(x) => x as usize,
    }
  }
}

impl From<RuntimeValue> for ValueType {
  fn from(v: RuntimeValue) -> ValueType {
    match v {
      RuntimeValue::I32(_) => ValueType::I32,
      RuntimeValue::I64(_) => ValueType::I64,
      RuntimeValue::F32(_) => ValueType::F32,
      RuntimeValue::F64(_) => ValueType::F64,
      RuntimeValue::V128(_) => unimplemented!("V128 is not implemented."),
    }
  }
}
