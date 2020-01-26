use crate::error::YawError;
use crate::types::*;
use crate::vm::value_stack::ValueStack;

pub fn i32_const(operands: &[Operand], stack: &mut ValueStack) -> Result<(), YawError> {
    let v: RuntimeValue = operands[0].into();
    stack.push(v);
    Ok(())
}

pub fn i64_const(operands: &[Operand], stack: &mut ValueStack) -> Result<(), YawError> {
    let v: RuntimeValue = operands[0].into();
    stack.push(v);
    Ok(())
}

pub fn f32_const(operands: &[Operand], stack: &mut ValueStack) -> Result<(), YawError> {
    let v: RuntimeValue = operands[0].into();
    stack.push(v);
    Ok(())
}

pub fn f64_const(operands: &[Operand], stack: &mut ValueStack) -> Result<(), YawError> {
    let v: RuntimeValue = operands[0].into();
    stack.push(v);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_i64const() -> Result<(), YawError> {
        let mut stack = ValueStack::new();
        i64_const(&[Operand::I64(42)], &mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I64(42)]);
        Ok(())
    }

    #[test]
    fn test_f64const() -> Result<(), YawError> {
        let mut stack = ValueStack::new();
        f64_const(&[Operand::F64(42.0)], &mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::F64(42.0)]);
        Ok(())
    }
}
