use crate::types::RuntimeValue;
use crate::vm::error::*;

trait UnsignedComparisons {
    fn compare_lt_u(self, x: &Self) -> i32;
    fn compare_gt_u(self, x: &Self) -> i32;
    fn compare_le_u(self, x: &Self) -> i32;
    fn compare_ge_u(self, x: &Self) -> i32;
}

trait SignedComparisons {
    fn compare_ge_s(self, x: &Self) -> i32;
    fn compare_lt_s(self, x: &Self) -> i32;
    fn compare_gt_s(self, x: &Self) -> i32;
    fn compare_le_s(self, x: &Self) -> i32;
}

impl UnsignedComparisons for i32 {
    fn compare_lt_u(self, x: &Self) -> i32 {
        (self as u32).lt(&(*x as u32)) as i32
    }

    fn compare_gt_u(self, x: &Self) -> i32 {
        (self as u32).gt(&(*x as u32)) as i32
    }

    fn compare_le_u(self, x: &Self) -> i32 {
        (self as u32).le(&(*x as u32)) as i32
    }

    fn compare_ge_u(self, x: &Self) -> i32 {
        (self as u32).ge(&(*x as u32)) as i32
    }
}

impl SignedComparisons for i32 {
    fn compare_ge_s(self, x: &Self) -> i32 {
        self.ge(x) as i32
    }

    fn compare_lt_s(self, x: &Self) -> i32 {
        self.lt(x) as i32
    }

    fn compare_gt_s(self, x: &Self) -> i32 {
        self.gt(x) as i32
    }

    fn compare_le_s(self, x: &Self) -> i32 {
        self.le(x) as i32
    }
}

impl SignedComparisons for i64 {
    fn compare_ge_s(self, x: &Self) -> i32 {
        self.ge(&x) as i32
    }

    fn compare_lt_s(self, x: &Self) -> i32 {
        self.lt(&x) as i32
    }

    fn compare_gt_s(self, x: &Self) -> i32 {
        self.gt(x) as i32
    }

    fn compare_le_s(self, x: &Self) -> i32 {
        self.le(x) as i32
    }
}

impl UnsignedComparisons for i64 {
    fn compare_lt_u(self, x: &Self) -> i32 {
        (self as u64).lt(&(*x as u64)) as i32
    }

    fn compare_gt_u(self, x: &Self) -> i32 {
        (self as u64).gt(&(*x as u64)) as i32
    }

    fn compare_le_u(self, x: &Self) -> i32 {
        (self as u64).le(&(*x as u64)) as i32
    }

    fn compare_ge_u(self, x: &Self) -> i32 {
        (self as u64).ge(&(*x as u64)) as i32
    }
}

impl SignedComparisons for f32 {
    fn compare_ge_s(self, x: &Self) -> i32 {
        self.ge(&x) as i32
    }

    fn compare_lt_s(self, x: &Self) -> i32 {
        self.lt(&x) as i32
    }

    fn compare_gt_s(self, x: &Self) -> i32 {
        self.gt(&x) as i32
    }

    fn compare_le_s(self, x: &Self) -> i32 {
        self.le(&x) as i32
    }
}

impl SignedComparisons for f64 {
    fn compare_ge_s(self, x: &Self) -> i32 {
        self.ge(&x) as i32
    }

    fn compare_lt_s(self, x: &Self) -> i32 {
        self.lt(&x) as i32
    }

    fn compare_gt_s(self, x: &Self) -> i32 {
        self.gt(&x) as i32
    }

    fn compare_le_s(self, x: &Self) -> i32 {
        self.le(&x) as i32
    }
}

macro_rules! compare_int_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::I32(l), RuntimeValue::I32(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        (RuntimeValue::I64(l), RuntimeValue::I64(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

macro_rules! compare_operation {
  ($fn_name: ident,$op: ident) => {
    pub fn $fn_name(&self, rhs: &Self) -> Result<Self, RuntimeError> {
      match (self, rhs) {
        (RuntimeValue::I32(l), RuntimeValue::I32(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        (RuntimeValue::I64(l), RuntimeValue::I64(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        (RuntimeValue::F32(l), RuntimeValue::F32(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        (RuntimeValue::F64(l), RuntimeValue::F64(r)) => Ok(RuntimeValue::I32(l.$op(r) as i32)),
        _ => Err(RuntimeError::TypeError),
      }
    }
  };
}

impl RuntimeValue {
    compare_operation!(eq, eq);
    compare_operation!(ne, ne);
    compare_operation!(lt_s, compare_lt_s);
    compare_int_operation!(lt_u, compare_lt_u);
    compare_operation!(ge_s, compare_ge_s);
    compare_operation!(gt_s, compare_gt_s);
    compare_int_operation!(gt_u, compare_gt_u);
    compare_operation!(le_s, compare_le_s);
    compare_int_operation!(le_u, compare_le_u);
    compare_int_operation!(ge_u, compare_ge_u);
}
