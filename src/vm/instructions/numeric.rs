use crate::vm::pop::*;
use crate::vm::value_stack::ValueStack;
use crate::vm::RuntimeError;

macro_rules! unary {
    ($name: ident) => {
        pub fn $name(stack: &mut ValueStack) -> Result<(), RuntimeError> {
            let v = pop(stack)?;
            stack.push(v.$name()?);
            Ok(())
        }
    };
}

macro_rules! binary {
    ($name: ident) => {
        pub fn $name(stack: &mut ValueStack) -> Result<(), RuntimeError> {
            let (left, right) = pop_lr(stack)?;
            stack.push(left.$name(&right)?);
            Ok(())
        }
    };
}

unary!(clz);
unary!(ctz);
unary!(popcnt);
binary!(add);
binary!(sub);
binary!(mul);
binary!(div_s);
binary!(div_u);
binary!(rem_s);
binary!(rem_u);
binary!(and);
binary!(or);
binary!(xor);
binary!(shl);
binary!(shr_s);
binary!(shr_u);
binary!(rotl);
binary!(rotr);
unary!(abs);
unary!(neg);
unary!(ceil);
unary!(floor);
unary!(trunc);
unary!(nearest);
unary!(sqrt);
binary!(div);
binary!(min);
binary!(max);
binary!(copysign);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;

    #[test]
    fn test_i32clz() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x0000_FFFF));
        clz(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(16)]);
        Ok(())
    }

    #[test]
    fn test_i32ctz() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x0000_FF00));
        ctz(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(8)]);
        Ok(())
    }

    #[test]
    fn test_i32popcnt() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(0x5A5A_A5A5));
        popcnt(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(16)]);
        Ok(())
    }

    #[test]
    fn test_f64add() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(10.0));
        stack.push(RuntimeValue::F64(20.1));
        add(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(30.1)]);
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(4));
        stack.push(RuntimeValue::I32(-2));
        add(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(2)],);
        Ok(())
    }

    #[test]
    fn test_i32mul() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(100));
        stack.push(RuntimeValue::I32(200));
        mul(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(20_000)]);
        Ok(())
    }

    #[test]
    fn test_f32mul() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(0.1));
        stack.push(RuntimeValue::F32(0.2));
        mul(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F32(0.020_000_001)]);
        Ok(())
    }

    #[test]
    fn test_i32div_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        div_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-2)]);
        Ok(())
    }

    #[test]
    fn test_i32div_u() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        div_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(2_147_483_646)]);
        Ok(())
    }

    #[test]
    fn test_i32rem_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-37));
        stack.push(RuntimeValue::I32(10));
        rem_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-7)]);
        Ok(())
    }

    #[test]
    fn test_i32rem_u() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-37));
        stack.push(RuntimeValue::I32(10));
        rem_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(9)]);
        Ok(())
    }

    #[test]
    fn test_i32shr_s() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        shr_s(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-1)]);
        Ok(())
    }

    #[test]
    fn test_i32shr_u() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        shr_u(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1_073_741_823)]);
        Ok(())
    }

    #[test]
    fn test_i32rotl() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        rotl(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(-13)]);
        Ok(())
    }

    #[test]
    fn test_i32rotr() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(-4));
        stack.push(RuntimeValue::I32(2));
        rotr(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1_073_741_823)]);
        Ok(())
    }

    #[test]
    fn test_f64abs() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(-4.5));
        abs(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(4.5)]);
        Ok(())
    }

    #[test]
    fn test_f64neg() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(4.5));
        neg(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(-4.5)]);
        Ok(())
    }

    #[test]
    fn test_f64ceil() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(4.5));
        ceil(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(5.0)]);
        Ok(())
    }

    #[test]
    fn test_f64floor() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(4.79));
        floor(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(4.0)]);
        Ok(())
    }

    #[test]
    fn test_f64nearest() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(4.49));
        nearest(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(4.0)]);
        Ok(())
    }

    #[test]
    fn test_f64trunc() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(-4.49));
        trunc(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(-4.0)]);
        Ok(())
    }

    #[test]
    fn test_f64sqrt() -> Result<(), RuntimeError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F64(1.44));
        sqrt(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(1.2)]);
        Ok(())
    }
}
