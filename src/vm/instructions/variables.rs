use crate::types::*;
use crate::vm::{Globals, RuntimeError};

use super::super::value_stack::ValueStack;
use super::pop;

pub fn get_local(
    operands: &[Operand],
    stack: &mut ValueStack,
    locals: &[RuntimeValue],
) -> Result<(), RuntimeError> {
    let index: usize = operands[0].into();
    let value = locals[index];
    stack.push(value);
    Ok(())
}

pub fn set_local(
    operands: &[Operand],
    stack: &mut ValueStack,
    locals: &mut [RuntimeValue],
) -> Result<(), RuntimeError> {
    let index: usize = operands[0].into();
    let value = pop(stack)?;
    locals[index] = value;
    Ok(())
}

pub fn tee_local(
    operands: &[Operand],
    stack: &mut ValueStack,
    locals: &mut [RuntimeValue],
) -> Result<(), RuntimeError> {
    let index: usize = operands[0].into();
    let value = pop(stack)?;
    locals[index] = value;
    stack.push(value);
    Ok(())
}

pub fn get_global(
    operands: &[Operand],
    stack: &mut ValueStack,
    globals: &Globals,
) -> Result<(), RuntimeError> {
    let index: usize = operands[0].into();
    let value = globals
        .get(index)
        .ok_or(RuntimeError::UndefinedGlobalError)?;
    let value = value.borrow();
    stack.push(value.value);
    Ok(())
}

pub fn set_global(
    operands: &[Operand],
    stack: &mut ValueStack,
    globals: &Globals,
) -> Result<(), RuntimeError> {
    let index: usize = operands[0].into();
    let v = pop(stack)?;
    let value = globals
        .get(index)
        .ok_or(RuntimeError::UndefinedGlobalError)?;
    let mut g = value.borrow_mut();
    g.value = v;
    Ok(())
}
