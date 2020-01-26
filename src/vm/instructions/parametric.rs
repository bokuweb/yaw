use crate::error::YawError;
use crate::vm::value_stack::ValueStack;
use crate::vm::RuntimeError;

use super::pop;

pub fn select(stack: &mut ValueStack) -> Result<(), YawError> {
    let op3 = stack.pop().unwrap();
    let op2 = stack.pop().unwrap();
    let op1 = stack.pop().unwrap();
    if op3.is_zero() {
        stack.push(op2);
    } else {
        stack.push(op1);
    }
    Ok(())
}

pub fn drop(stack: &mut ValueStack) -> Result<(), RuntimeError> {
    pop(stack)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;

    #[test]
    fn test_select() -> Result<(), YawError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(10));
        stack.push(RuntimeValue::I32(20));
        stack.push(RuntimeValue::I32(0));
        select(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(20)]);
        Ok(())
    }

    #[test]
    fn test_drop() -> Result<(), YawError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::I32(4));
        drop(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![],);
        Ok(())
    }
}
