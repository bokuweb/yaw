use crate::types::RuntimeValue;
use crate::vm::error::RuntimeError;
use crate::vm::value_stack::ValueStack;

use super::pop::*;

macro_rules! compare {
    ($name: ident) => {
        pub fn $name(stack: &mut ValueStack) -> Result<(), RuntimeError> {
            let (left, right) = pop_lr(stack)?;
            stack.push(left.$name(&right)?);
            Ok(())
        }
    };
}

pub fn eqz(stack: &mut ValueStack) -> Result<(), RuntimeError> {
    let res = pop(stack)?.is_zero() as i32;
    stack.push(RuntimeValue::I32(res));
    Ok(())
}

compare!(eq);
compare!(ne);
compare!(lt_s);
compare!(lt_u);
compare!(ge_s);
compare!(gt_s);
compare!(gt_u);
compare!(le_s);
compare!(le_u);
compare!(ge_u);

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn eqz_should_push_0() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x01));
        eqz(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)]);
        Ok(())
    }

    #[test]
    fn eqz_should_push_1() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I64(0x00));
        eqz(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)]);
        Ok(())
    }

    #[test]
    fn i32_eq_should_push_1() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x01));
        stack.push(RuntimeValue::I32(0x01));
        eq(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)]);
        Ok(())
    }

    #[test]
    fn f32_eq_should_push_1() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(1.23));
        stack.push(RuntimeValue::F32(1.23));
        eq(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)]);
        Ok(())
    }

    #[test]
    fn f64_eq_should_push_0() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(1.23));
        stack.push(RuntimeValue::F64(1.22));
        eq(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)]);
        Ok(())
    }

    #[test]
    fn i32_ne_should_push_1() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x01));
        stack.push(RuntimeValue::I32(0x00));
        ne(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)]);
        Ok(())
    }

    #[test]
    fn f64_ne_should_push_0() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(0.01));
        stack.push(RuntimeValue::F64(0.01));
        ne(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)]);
        Ok(())
    }

    #[test]
    fn test_ge_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        ge_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)],);
        Ok(())
    }

    #[test]
    fn test_lt_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        lt_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)],);
        Ok(())
    }

    #[test]
    fn test_lt_u_i32() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(1));
        stack.push(RuntimeValue::I32(-1));
        lt_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)],);
        Ok(())
    }

    #[test]
    fn test_lt_u_i64() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I64(-1));
        stack.push(RuntimeValue::I64(1));
        lt_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)],);
        Ok(())
    }

    #[test]
    fn test_gt_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        gt_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)],);
        Ok(())
    }

    #[test]
    fn test_gt_u() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        gt_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)],);
        Ok(())
    }

    #[test]
    fn test_le_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        le_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)],);
        Ok(())
    }

    #[test]
    fn test_ge_u() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(2));
        stack.push(RuntimeValue::I32(-2));
        ge_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0)],);
        Ok(())
    }
}
