use crate::types::RuntimeValue;
use crate::vm::error::*;

trait Trunc<T> {
    fn trunc_s(self) -> Result<T, RuntimeError>;
    fn trunc_u(self) -> Result<T, RuntimeError>;
}

trait Convert<T> {
    fn convert_s(self) -> Result<T, RuntimeError>;
    fn convert_u(self) -> Result<T, RuntimeError>;
}

macro_rules! trunc_impl {
    ($from: ty,$signed_to: ty,$unsigned_to: ty) => {
        impl Trunc<$signed_to> for $from {
            fn trunc_s(self) -> Result<$signed_to, RuntimeError> {
                match self {
                    _n if self.is_nan() => Err(RuntimeError::UnrepresentableIntegerError),
                    _n if self.is_infinite() => Err(RuntimeError::IntegerOverflowError),
                    n => {
                        let res = n as $signed_to;
                        if ((res as $from) - n.trunc()).abs() > 0.0 {
                            return Err(RuntimeError::IntegerOverflowError);
                        }
                        Ok(res)
                    }
                }
            }

            fn trunc_u(self) -> Result<$signed_to, RuntimeError> {
                match self {
                    _n if self.is_nan() => Err(RuntimeError::UnrepresentableIntegerError),
                    _n if self.is_infinite() => Err(RuntimeError::IntegerOverflowError),
                    n => {
                        let res = n as $unsigned_to;
                        if ((res as $from) - n.trunc()).abs() > 0.0 {
                            return Err(RuntimeError::IntegerOverflowError);
                        }
                        Ok(res as $signed_to)
                    }
                }
            }
        }
    };
}

trunc_impl!(f32, i32, u32);
trunc_impl!(f64, i32, u32);
trunc_impl!(f32, i64, u64);
trunc_impl!(f64, i64, u64);

macro_rules! convert_impl {
    ($from: ty, $mid: ty, $to: ty) => {
        impl Convert<$to> for $from {
            fn convert_s(self) -> Result<$to, RuntimeError> {
                Ok(self as $to)
            }

            fn convert_u(self) -> Result<$to, RuntimeError> {
                Ok(self as $mid as $to)
            }
        }
    };
}

convert_impl!(i32, u32, f32);
convert_impl!(i64, u64, f32);
convert_impl!(i32, u32, f64);
convert_impl!(i64, u64, f64);

impl RuntimeValue {
    pub fn trunc_s_toi32(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F32(l) => Ok(RuntimeValue::I32(l.trunc_s()?)),
            RuntimeValue::F64(l) => Ok(RuntimeValue::I32(l.trunc_s()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn trunc_u_toi32(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F32(l) => Ok(RuntimeValue::I32(l.trunc_u()?)),
            RuntimeValue::F64(l) => Ok(RuntimeValue::I32(l.trunc_u()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn trunc_s_toi64(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F32(l) => Ok(RuntimeValue::I64(l.trunc_s()?)),
            RuntimeValue::F64(l) => Ok(RuntimeValue::I64(l.trunc_s()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn trunc_u_toi64(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F32(l) => Ok(RuntimeValue::I64(l.trunc_u()?)),
            RuntimeValue::F64(l) => Ok(RuntimeValue::I64(l.trunc_u()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn extend_s(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::I64(i64::from(l))),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn extend_u(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::I64(i64::from(l as u32))),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn convert_s_tof32(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::F32(l.convert_s()?)),
            RuntimeValue::I64(l) => Ok(RuntimeValue::F32(l.convert_s()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn convert_u_tof32(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::F32(l.convert_u()?)),
            RuntimeValue::I64(l) => Ok(RuntimeValue::F32(l.convert_u()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn convert_s_tof64(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::F64(l.convert_s()?)),
            RuntimeValue::I64(l) => Ok(RuntimeValue::F64(l.convert_s()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn convert_u_tof64(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I32(l) => Ok(RuntimeValue::F64(l.convert_u()?)),
            RuntimeValue::I64(l) => Ok(RuntimeValue::F64(l.convert_u()?)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn wrap(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::I64(n) => Ok(RuntimeValue::I32(n as i32)),
            _ => Err(RuntimeError::TypeMismatchOperationError),
        }
    }

    pub fn demote(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F64(l) => Ok(RuntimeValue::F32(l as f32)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn promote(self) -> Result<Self, RuntimeError> {
        match self {
            RuntimeValue::F32(l) => Ok(RuntimeValue::F64(f64::from(l))),
            _ => Err(RuntimeError::TypeError),
        }
    }
}
