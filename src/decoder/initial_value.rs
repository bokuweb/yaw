use num_traits::*;
use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

use crate::reader::*;
use crate::types::*;
use crate::vm::instructions::Opcode;

pub fn read_initial_value<R: Read>(reader: &mut R) -> Result<RuntimeValue, DecodeError> {
    let opcode = Opcode::from_u8(read_next(reader)?).unwrap();
    let v = match opcode {
        Opcode::I32Const => {
            let v: RuntimeValue = VarInt32::decode(reader)?.into();
            v
        }
        Opcode::I64Const => {
            let v: RuntimeValue = VarInt64::decode(reader)?.into();
            v
        }
        Opcode::F32Const => {
            let v = read_u32(reader)?;
            let v = f32::from_bits(v);
            RuntimeValue::F32(v)
        }
        Opcode::F64Const => {
            let v = read_u64(reader)?;
            let v = f64::from_bits(v);
            RuntimeValue::F64(v)
        }
        _ => return Err(DecodeError::InvalidInitializerError),
    };
    // Read 0x0B
    let _ = read_next(reader)?;
    Ok(v)
}
