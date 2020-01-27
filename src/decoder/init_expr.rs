use num_traits::*;
use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

use crate::reader::*;
use crate::types::*;
use crate::vm::instructions::Opcode;
use crate::vm::Globals;

#[derive(Debug, Clone, PartialEq)]
pub enum InitExpr {
    RuntimeValue(RuntimeValue),
    GlobalIndex(usize),
}

impl InitExpr {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<InitExpr, DecodeError> {
        let opcode = Opcode::from_u8(read_next(reader)?).unwrap();
        let v = match opcode {
            Opcode::I32Const => {
                let v: RuntimeValue = VarInt32::decode(reader)?.into();
                InitExpr::RuntimeValue(v)
            }
            Opcode::I64Const => {
                let v: RuntimeValue = VarInt64::decode(reader)?.into();
                InitExpr::RuntimeValue(v)
            }
            Opcode::F32Const => {
                let v = read_u32(reader)?;
                let v = f32::from_bits(v);
                InitExpr::RuntimeValue(RuntimeValue::F32(v))
            }
            Opcode::F64Const => {
                let v = read_u64(reader)?;
                let v = f64::from_bits(v);
                InitExpr::RuntimeValue(RuntimeValue::F64(v))
            }
            Opcode::GetGlobal => {
                let index: usize = VarUint32::decode(reader)?.into();
                InitExpr::GlobalIndex(index)
            }
            _ => return Err(DecodeError::InvalidInitializerError),
        };
        // Read 0x0B
        let _ = read_next(reader)?;
        Ok(v)
    }

    pub(crate) fn eval(&self, globals: &Globals) -> Result<RuntimeValue, DecodeError> {
        match self {
            Self::RuntimeValue(v) => Ok(*v),
            Self::GlobalIndex(index) => {
                if let Some(v) = globals.get(*index) {
                    return Ok(v.borrow().value);
                }
                Err(DecodeError::InvalidInitializerError)
            }
        }
    }
}
