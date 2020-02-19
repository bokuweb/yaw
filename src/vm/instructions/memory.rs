use std::mem;

use crate::types::*;
use crate::vm::error::RuntimeError;
use crate::vm::memory::{MemoryRef, PAGE_SIZE};
use crate::vm::value_stack::ValueStack;

use super::pop::*;

fn validate_bounds(addr: u64, type_size: usize, current_size: usize) -> Result<(), RuntimeError> {
    if addr + type_size as u64 > current_size as u64 * PAGE_SIZE as u64 {
        dbg!(addr, current_size, PAGE_SIZE);
        return Err(RuntimeError::OutOfBoundsMemoryAccessError);
    }
    Ok(())
}

macro_rules! load {
    ($name: ident, $loader: ident, $convert_type: ty, $ret_type: ty, $ret_runtime_value: expr) => {
        pub fn $name(
            operands: &[Operand],
            stack: &mut ValueStack,
            memory_ref: &MemoryRef,
        ) -> Result<(), RuntimeError> {
            // index 0 is unused flags
            let offset: u32 = operands[1].into();
            let addr: u32 = pop(stack)?.into();
            validate_bounds(
                addr as u64 + offset as u64,
                mem::size_of::<$convert_type>(),
                memory_ref.current(),
            )?;
            let v = memory_ref.$loader(addr + offset)? as $convert_type;
            stack.push($ret_runtime_value(v as $ret_type));
            Ok(())
        }
    };
}

load!(i32_load, i32_load, i32, i32, RuntimeValue::I32);
load!(i64_load, i64_load, i64, i64, RuntimeValue::I64);
load!(f32_load, f32_load, f32, f32, RuntimeValue::F32);
load!(f64_load, f64_load, f64, f64, RuntimeValue::F64);

load!(i32_load8_s, i8_load, i8, i32, RuntimeValue::I32);
load!(i32_load8_u, i8_load, u8, i32, RuntimeValue::I32);
load!(i32_load16_s, i16_load, i16, i32, RuntimeValue::I32);
load!(i32_load16_u, i16_load, u16, i32, RuntimeValue::I32);

load!(i64_load8_s, i8_load, i8, i64, RuntimeValue::I64);
load!(i64_load8_u, i8_load, u8, i64, RuntimeValue::I64);
load!(i64_load16_s, i16_load, i16, i64, RuntimeValue::I64);
load!(i64_load16_u, i16_load, u16, i64, RuntimeValue::I64);
load!(i64_load32_s, i32_load, i32, i64, RuntimeValue::I64);
load!(i64_load32_u, i32_load, u32, i64, RuntimeValue::I64);

macro_rules! i32_store {
    ($name: ident, $writer: ident, $convert_type: ty) => {
        pub fn $name(
            operands: &[Operand],
            stack: &mut ValueStack,
            memory_ref: &MemoryRef,
        ) -> Result<(), RuntimeError> {
            // index 0 is unused flags
            let offset: u32 = operands[1].into();
            let data = pop(stack)?;
            let addr: u32 = pop(stack)?.into();
            validate_bounds(
                addr as u64 + offset as u64,
                mem::size_of::<$convert_type>(),
                memory_ref.current(),
            )?;
            if let RuntimeValue::I32(v) = data {
                memory_ref.$writer(addr + offset, v as $convert_type)?;
                Ok(())
            } else {
                Err(RuntimeError::TypeMismatchOperationError)
            }
        }
    };
}

i32_store!(i32_store, i32_store, i32);
i32_store!(i32_store8, i8_store, i8);
i32_store!(i32_store16, i16_store, i16);

macro_rules! i64_store {
    ($name: ident, $writer: ident, $convert_type: ty) => {
        pub fn $name(
            operands: &[Operand],
            stack: &mut ValueStack,
            memory_ref: &MemoryRef,
        ) -> Result<(), RuntimeError> {
            // index 0 is unused flags
            let offset: u32 = operands[1].into();
            let data = pop(stack)?;
            let addr: u32 = pop(stack)?.into();
            validate_bounds(
                addr as u64 + offset as u64,
                mem::size_of::<$convert_type>(),
                memory_ref.current(),
            )?;
            if let RuntimeValue::I64(v) = data {
                memory_ref.$writer(addr + offset, v as $convert_type)?;
                return Ok(());
            }
            Err(RuntimeError::TypeMismatchOperationError)
        }
    };
}

i64_store!(i64_store, i64_store, i64);
i64_store!(i64_store8, i8_store, i8);
i64_store!(i64_store16, i16_store, i16);
i64_store!(i64_store32, i32_store, i32);

pub fn f32_store(
    operands: &[Operand],
    stack: &mut ValueStack,
    memory_ref: &MemoryRef,
) -> Result<(), RuntimeError> {
    // index 0 is unused flags
    let offset: u32 = operands[1].into();
    let data = pop(stack)?;
    let addr: u32 = pop(stack)?.into();
    validate_bounds(
        u64::from(addr) + u64::from(offset),
        mem::size_of::<f32>(),
        memory_ref.current(),
    )?;
    if let RuntimeValue::F32(v) = data {
        memory_ref.f32_store(addr + offset, v)?;
        return Ok(());
    }
    Err(RuntimeError::TypeMismatchOperationError)
}

pub fn f64_store(
    operands: &[Operand],
    stack: &mut ValueStack,
    memory_ref: &MemoryRef,
) -> Result<(), RuntimeError> {
    // index 0 is unused flags
    let offset: u32 = operands[1].into();
    let data = pop(stack)?;
    let addr: u32 = pop(stack)?.into();
    validate_bounds(
        u64::from(addr) + u64::from(offset),
        mem::size_of::<f64>(),
        memory_ref.current(),
    )?;
    if let RuntimeValue::F64(v) = data {
        memory_ref.f64_store(addr + offset, v)?;
        return Ok(());
    }
    Err(RuntimeError::TypeMismatchOperationError)
}

pub fn current(stack: &mut ValueStack, memory_ref: &MemoryRef) -> Result<(), RuntimeError> {
    let current = memory_ref.current();
    stack.push(RuntimeValue::I32(current as i32));
    Ok(())
}

pub fn grow(stack: &mut ValueStack, memory_ref: &MemoryRef) -> Result<(), RuntimeError> {
    let size: u32 = pop(stack)?.into();
    let res = memory_ref.grow(size);
    stack.push(RuntimeValue::I32(res));
    Ok(())
}
