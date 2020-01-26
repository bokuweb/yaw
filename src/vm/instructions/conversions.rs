use crate::vm::error::*;
use crate::vm::pop::*;
use crate::vm::value_stack::ValueStack;

macro_rules! unary {
    ($name: ident) => {
        pub fn $name(stack: &mut ValueStack) -> Result<(), RuntimeError> {
            let v = pop(stack)?;
            stack.push(v.$name()?);
            Ok(())
        }
    };
}

unary!(wrap);
unary!(trunc_s_toi32);
unary!(trunc_u_toi32);
unary!(trunc_s_toi64);
unary!(trunc_u_toi64);
unary!(extend_s);
unary!(extend_u);
unary!(convert_s_tof32);
unary!(convert_u_tof32);
unary!(convert_s_tof64);
unary!(convert_u_tof64);
unary!(demote);
unary!(promote);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;

    #[test]
    fn test_reinterpret() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I64(-6_510_698_342_184_737_371)); // 0xA5A5_5A5A_5A5A_A5A5
        wrap(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(0x5A5A_A5A5)]);
        Ok(())
    }

    #[test]
    fn test_f32_trunc_s_toi32() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(-1.123));
        trunc_s_toi32(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-1)]);
        Ok(())
    }

    #[test]
    fn test_f64_trunc_s_toi32() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(-2.423));
        trunc_s_toi32(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-2)]);
        Ok(())
    }

    #[test]
    fn test_f32_trunc_u_toi32() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(1.123));
        trunc_s_toi32(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1)]);
        Ok(())
    }

    #[test]
    fn test_f64_trunc_s_toi64() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(-2.423));
        trunc_s_toi64(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I64(-2)]);
        Ok(())
    }

    #[test]
    fn test_f32_trunc_u_toi64() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(1.123));
        trunc_s_toi64(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I64(1)]);
        Ok(())
    }
}
