use crate::error::YawError;

use crate::vm::pop::*;
use crate::vm::value_stack::ValueStack;

pub fn reinterpret(stack: &mut ValueStack) -> Result<(), YawError> {
    let v = pop(stack)?;
    stack.push(v.reinterpret()?);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;

    #[test]
    fn test_reinterpret() -> Result<(), YawError> {
        let mut stack = ValueStack::new();
        stack.push(RuntimeValue::F32(1.0));
        reinterpret(&mut stack)?;
        assert_eq!(stack.take_buf(), vec![RuntimeValue::I32(1_065_353_216)]);
        Ok(())
    }
}
