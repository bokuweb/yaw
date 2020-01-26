use crate::types::RuntimeValue;
use crate::vm::error::*;
use core::ops::{BitAnd, BitOr, BitXor, Neg};

trait IntegerNumeric {
    fn div_s(&self, rhs: Self) -> Result<Self, RuntimeError>
    where
        Self: Sized;
    fn div_u(&self, rhs: Self) -> Result<Self, RuntimeError>
    where
        Self: Sized;
    fn rem_s(&self, rhs: Self) -> Result<Self, RuntimeError>
    where
        Self: Sized;
    fn rem_u(&self, rhs: Self) -> Result<Self, RuntimeError>
    where
        Self: Sized;
    fn shl(&self, x: Self) -> Self
    where
        Self: Sized;
    fn shr_s(&self, rhs: Self) -> Self;
    fn shr_u(&self, rhs: Self) -> Self;
    fn rotl(&self, rhs: Self) -> Self;
    fn rotr(&self, rhs: Self) -> Self;
}

impl IntegerNumeric for i32 {
    fn div_s(&self, rhs: Self) -> Result<Self, RuntimeError> {
        if rhs == 0 {
            return Err(RuntimeError::DivisionByZeroError);
        }
        match (*self).overflowing_div(rhs) {
            (_, true) => Err(RuntimeError::DivisionOverflowError),
            (res, false) => Ok(res),
        }
    }

    fn div_u(&self, rhs: Self) -> Result<Self, RuntimeError> {
        if rhs == 0 {
            return Err(RuntimeError::DivisionByZeroError);
        }
        match (*self as u32).overflowing_div(rhs as u32) {
            (_, true) => Err(RuntimeError::DivisionOverflowError),
            (res, false) => Ok(res as i32),
        }
    }

    fn rem_s(&self, rhs: Self) -> Result<Self, RuntimeError> {
        match rhs {
            0 => Err(RuntimeError::DivisionByZeroError),
            _ => {
                let (res, _) = self.overflowing_rem(rhs);
                Ok(res)
            }
        }
    }

    fn rem_u(&self, rhs: Self) -> Result<Self, RuntimeError> {
        match rhs {
            0 => Err(RuntimeError::DivisionByZeroError),
            _ => {
                let (res, _) = (*self as u32).overflowing_rem(rhs as u32);
                Ok(res as i32)
            }
        }
    }

    fn shl(&self, rhs: Self) -> i32 {
        self.wrapping_shl(rhs as u32) as i32
    }

    fn shr_s(&self, rhs: Self) -> i32 {
        self.wrapping_shr(rhs as u32)
    }

    fn shr_u(&self, rhs: Self) -> i32 {
        (*self as u32).wrapping_shr(rhs as u32) as i32
    }

    fn rotl(&self, rhs: Self) -> i32 {
        self.rotate_left(rhs as u32) as i32
    }

    fn rotr(&self, rhs: Self) -> i32 {
        self.rotate_right(rhs as u32) as i32
    }
}

impl IntegerNumeric for i64 {
    fn div_s(&self, rhs: Self) -> Result<Self, RuntimeError> {
        if rhs == 0 {
            return Err(RuntimeError::DivisionByZeroError);
        }
        match self.overflowing_div(rhs) {
            (_, true) => Err(RuntimeError::DivisionOverflowError),
            (res, false) => Ok(res as i64),
        }
    }

    fn div_u(&self, rhs: Self) -> Result<Self, RuntimeError> {
        if rhs == 0 {
            return Err(RuntimeError::DivisionByZeroError);
        }
        match (*self as u64).overflowing_div(rhs as u64) {
            (_, true) => Err(RuntimeError::DivisionOverflowError),
            (res, false) => Ok(res as i64),
        }
    }

    fn rem_s(&self, rhs: Self) -> Result<Self, RuntimeError> {
        match rhs {
            0 => Err(RuntimeError::DivisionByZeroError),
            _ => {
                let (res, _) = self.overflowing_rem(rhs);
                Ok(res)
            }
        }
    }

    fn rem_u(&self, rhs: Self) -> Result<Self, RuntimeError> {
        match rhs {
            0 => Err(RuntimeError::DivisionByZeroError),
            _ => {
                let (res, _) = (*self as u64).overflowing_rem(rhs as u64);
                Ok(res as i64)
            }
        }
    }

    fn shl(&self, rhs: Self) -> i64 {
        self.wrapping_shl(rhs as u32) as i64
    }

    fn shr_s(&self, rhs: Self) -> Self {
        self.wrapping_shr(rhs as u32)
    }

    fn shr_u(&self, rhs: Self) -> Self {
        (*self as u64).wrapping_shr(rhs as u32) as i64
    }

    fn rotl(&self, rhs: Self) -> i64 {
        self.rotate_left(rhs as u32) as i64
    }

    fn rotr(&self, rhs: Self) -> i64 {
        self.rotate_right(rhs as u32) as i64
    }
}

trait FloatingNumeric {
    fn wrapping_add(self, x: Self) -> Self;
    fn wrapping_sub(self, x: Self) -> Self;
    fn wrapping_mul(self, x: Self) -> Self;

    fn div(self, x: Self) -> Self;
    fn copysign(self, x: Self) -> Self;
    fn nearest(self) -> Self;
}

macro_rules! impl_float_operation {
    ($t: ident) => {
        impl FloatingNumeric for $t {
            fn wrapping_add(self, x: Self) -> Self {
                self + x
            }

            fn wrapping_sub(self, x: Self) -> Self {
                self - x
            }

            fn wrapping_mul(self, x: Self) -> Self {
                self * x
            }

            fn div(self, x: Self) -> Self {
                self / x
            }

            fn copysign(self, x: Self) -> Self {
                if (self.signum() - x.signum()).abs() > 0.0 {
                    return self.neg();
                }
                self
            }

            fn nearest(self) -> Self {
                if self.abs() > 0.0 && self.abs() <= 0.5 {
                    0.0
                } else {
                    let rounded = self.round();
                    match rounded as i64 % 2 {
                        r if r == 1 => self.floor(),
                        r if r == -1 => self.ceil(),
                        _ => rounded,
                    }
                }
            }
        }
    };
}

impl_float_operation!(f32);
impl_float_operation!(f64);

macro_rules! binary_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::I32(l), RuntimeValue::I32(r)) => Ok(RuntimeValue::I32(l.$op(*r))),
        (RuntimeValue::I64(l), RuntimeValue::I64(r)) => Ok(RuntimeValue::I64(l.$op(*r))),
        (RuntimeValue::F32(l), RuntimeValue::F32(r)) => Ok(RuntimeValue::F32(l.$op(*r))),
        (RuntimeValue::F64(l), RuntimeValue::F64(r)) => Ok(RuntimeValue::F64(l.$op(*r))),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! unary_int_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self) -> Result<Self, RuntimeError> {
      match self {
        RuntimeValue::I32(l) => Ok(RuntimeValue::I32(l.$op() as i32)),
        RuntimeValue::I64(l) => Ok(RuntimeValue::I64(l.$op() as i64)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! unary_float_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self) -> Result<Self, RuntimeError> {
      match self {
        RuntimeValue::F32(l) => Ok(RuntimeValue::F32(l.$op() as f32)),
        RuntimeValue::F64(l) => Ok(RuntimeValue::F64(l.$op() as f64)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! binary_int_div_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::I32(l), RuntimeValue::I32(r)) => Ok(RuntimeValue::I32(l.$op(*r)? as i32)),
        (RuntimeValue::I64(l), RuntimeValue::I64(r)) => Ok(RuntimeValue::I64(l.$op(*r)? as i64)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! binary_int_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::I32(l), RuntimeValue::I32(r)) => Ok(RuntimeValue::I32(l.$op(*r) as i32)),
        (RuntimeValue::I64(l), RuntimeValue::I64(r)) => Ok(RuntimeValue::I64(l.$op(*r) as i64)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! binary_float_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::F32(l), RuntimeValue::F32(r)) => Ok(RuntimeValue::F32(l.$op(*r) as f32)),
        (RuntimeValue::F64(l), RuntimeValue::F64(r)) => Ok(RuntimeValue::F64(l.$op(*r) as f64)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

impl RuntimeValue {
    unary_int_operation!(clz, leading_zeros);
    unary_int_operation!(ctz, trailing_zeros);
    unary_int_operation!(popcnt, count_ones);
    binary_operation!(add, wrapping_add);
    binary_operation!(sub, wrapping_sub);
    binary_operation!(mul, wrapping_mul);
    binary_int_div_operation!(div_s, div_s);
    binary_int_div_operation!(div_u, div_u);
    binary_int_div_operation!(rem_s, rem_s);
    binary_int_div_operation!(rem_u, rem_u);
    binary_int_operation!(and, bitand);
    binary_int_operation!(or, bitor);
    binary_int_operation!(xor, bitxor);
    binary_int_operation!(shl, shl);
    binary_int_operation!(shr_s, shr_s);
    binary_int_operation!(shr_u, shr_u);
    binary_int_operation!(rotl, rotl);
    binary_int_operation!(rotr, rotr);
    unary_float_operation!(abs, abs);
    unary_float_operation!(neg, neg);
    unary_float_operation!(ceil, ceil);
    unary_float_operation!(floor, floor);
    unary_float_operation!(trunc, trunc);
    unary_float_operation!(nearest, nearest);
    unary_float_operation!(sqrt, sqrt);
    binary_float_operation!(div, div);
    binary_float_operation!(min, min);
    binary_float_operation!(max, max);
    binary_float_operation!(copysign, copysign);

    pub fn is_zero(self) -> bool {
        match self {
            RuntimeValue::I32(v) => v == 0,
            RuntimeValue::I64(v) => v == 0,
            RuntimeValue::F32(v) => v == 0.0,
            RuntimeValue::F64(v) => v == 0.0,
            RuntimeValue::V128(v) => v == 0,
        }
    }
}
