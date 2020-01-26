use crate::types::RuntimeValue;
use crate::vm::value_stack::ValueStack;
use crate::vm::error::RuntimeError;

pub fn pop_lr(stack: &mut ValueStack) -> Result<(RuntimeValue, RuntimeValue), RuntimeError> {
    let r = stack.pop().ok_or(RuntimeError::StackPopError)?;
    let l = stack.pop().ok_or(RuntimeError::StackPopError)?;
    Ok((l, r))
}

pub fn pop(stack: &mut ValueStack) -> Result<RuntimeValue, RuntimeError> {
    let v = stack.pop().ok_or(RuntimeError::StackPopError)?;
    Ok(v)
}
