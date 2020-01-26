use num_traits::*;
use std::io::{Cursor, Read};

use crate::reader::*;
use crate::types::*;
use crate::vm::Opcode;

use super::number::*;
use super::{DecodeError, Decoder};

pub type Instruction = (Opcode, Vec<Operand>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LocalEntry {
    pub count: u32,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub locals: Vec<LocalEntry>,
    pub decoded: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeSection {
    pub count: u32,
    pub bodies: Vec<FunctionBody>,
}

impl Decoder for CodeSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u32 = VarUint32::decode(reader)?.into();
        let mut bodies: Vec<FunctionBody> = vec![];
        for _ in 0..count {
            let body_size: usize = VarUint32::decode(reader)?.into();
            let mut body = Cursor::new(read_bytes(reader, body_size)?);
            let local_count: u32 = VarUint32::decode(&mut body)?.into();
            let mut locals: Vec<LocalEntry> = vec![];
            for _ in 0..local_count {
                let count = VarUint32::decode(&mut body)?.into();
                let value_type = ValueType::from_u8(read_next(&mut body)?)
                    .ok_or(DecodeError::InvalidValueTypeError)?;
                locals.push(LocalEntry { count, value_type });
            }
            let mut code: Vec<u8> = vec![];
            body.read_to_end(&mut code)?;
            code.pop();
            bodies.push(FunctionBody {
                locals,
                decoded: decode_function_body(&code)?,
            })
        }
        Ok(CodeSection { count, bodies })
    }
}

fn decode_function_body(code: &[u8]) -> Result<Vec<Instruction>, DecodeError> {
    let mut decoded = vec![];
    let mut reader = Cursor::new(code);
    loop {
        if reader.get_ref().len() == reader.position() as usize {
            break;
        }
        let mut operands: Vec<Operand> = vec![];
        let next = read_next(&mut reader)?;
        let i = Opcode::from_u8(next).ok_or(DecodeError::InvalidOpcodeError)?;
        match i {
            Opcode::Block | Opcode::Loop | Opcode::If => {
                let result_type = ResultType::from_u8(read_next(&mut reader)?)
                    .ok_or(DecodeError::InvalidResultTypeError)?;
                operands.push(Operand::ResultType(result_type));
                decoded.push((i, operands))
            }
            Opcode::Br
            | Opcode::BrIf
            | Opcode::Call
            | Opcode::GetLocal
            | Opcode::SetLocal
            | Opcode::TeeLocal
            | Opcode::GetGlobal
            | Opcode::SetGlobal => {
                let v: u32 = VarUint32::decode(&mut reader)?.into();
                operands.push(Operand::U32(v));
                decoded.push((i, operands))
            }
            Opcode::BrTable => {
                let count: usize = VarUint32::decode(&mut reader)?.into();
                operands.push(Operand::U32(count as u32));
                for _ in 0..count {
                    let v: u32 = VarUint32::decode(&mut reader)?.into();
                    operands.push(Operand::U32(v));
                }
                // Default target
                let v: u32 = VarUint32::decode(&mut reader)?.into();
                operands.push(Operand::U32(v));
                decoded.push((i, operands))
            }
            Opcode::CallIndirect => {
                let v: u32 = VarUint32::decode(&mut reader)?.into();
                operands.push(Operand::U32(v));
                decoded.push((i, operands));
                // Reserved
                let _ = VarUint32::decode(&mut reader)?;
            }
            Opcode::I32Load
            | Opcode::I64Load
            | Opcode::F32Load
            | Opcode::F64Load
            | Opcode::I32Load8S
            | Opcode::I32Load8U
            | Opcode::I32Load16S
            | Opcode::I32Load16U
            | Opcode::I64Load8S
            | Opcode::I64Load8U
            | Opcode::I64Load16S
            | Opcode::I64Load16U
            | Opcode::I64Load32S
            | Opcode::I64Load32U
            | Opcode::I32Store
            | Opcode::I64Store
            | Opcode::F32Store
            | Opcode::F64Store
            | Opcode::I32Store8
            | Opcode::I32Store16
            | Opcode::I64Store8
            | Opcode::I64Store16
            | Opcode::I64Store32 => {
                let flags: u32 = VarUint32::decode(&mut reader)?.into();
                let offset: u32 = VarUint32::decode(&mut reader)?.into();
                operands.push(Operand::U32(flags));
                operands.push(Operand::U32(offset));
                decoded.push((i, operands))
            }
            Opcode::CurrentMemory | Opcode::GrowMemory => {
                // Reserved
                let _ = VarUint32::decode(&mut reader)?;
                decoded.push((i, operands))
            }
            Opcode::I32Const => {
                let v: i32 = VarInt32::decode(&mut reader)?.into();
                operands.push(Operand::I32(v));
                decoded.push((i, operands))
            }
            Opcode::I64Const => {
                let v: i64 = VarInt64::decode(&mut reader)?.into();
                operands.push(Operand::I64(v));
                decoded.push((i, operands))
            }
            Opcode::F32Const => {
                let v = read_u32(&mut reader)?;
                let v = f32::from_bits(v);
                operands.push(Operand::F32(v));
                decoded.push((i, operands))
            }
            Opcode::F64Const => {
                let v = read_u64(&mut reader)?;
                let v = f64::from_bits(v);
                operands.push(Operand::F64(v));
                decoded.push((i, operands))
            }
            _ => decoded.push((i, operands)),
        };
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_decode_add() {
        // (func $func0 (param $var0 i32) (param $var1 i32) (result i32)
        //   get_local $var0
        //   get_local $var1
        //   i32.add
        // )
        let b = vec![0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6A, 0x0B];
        let mut cur = Cursor::new(b);
        let section = CodeSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            CodeSection {
                count: 1,
                bodies: vec![FunctionBody {
                    decoded: vec![
                        (Opcode::GetLocal, vec![Operand::U32(0)]),
                        (Opcode::GetLocal, vec![Operand::U32(1)]),
                        (Opcode::I32Add, vec![])
                    ],
                    locals: vec![],
                }],
            }
        );
    }

    #[test]
    fn it_decode_add_local() {
        // (func $add (param $lhs i32) (param $rhs i32) (result i32) (local i32)
        //   i32.const 200
        //   set_local 2
        //   get_local $lhs
        //   get_local 2
        //   i32.add)
        let b = vec![
            0x01, 0x0E, 0x01, 0x01, 0x7F, 0x41, 0xC8, 0x01, 0x21, 0x02, 0x20, 0x00, 0x20, 0x02,
            0x6A, 0x0B,
        ];
        let mut cur = Cursor::new(b);
        let section = CodeSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            CodeSection {
                count: 1,
                bodies: vec![FunctionBody {
                    decoded: vec![
                        (Opcode::I32Const, vec![Operand::I32(200)]),
                        (Opcode::SetLocal, vec![Operand::U32(2)]),
                        (Opcode::GetLocal, vec![Operand::U32(0)]),
                        (Opcode::GetLocal, vec![Operand::U32(2)]),
                        (Opcode::I32Add, vec![])
                    ],
                    locals: vec![LocalEntry {
                        count: 1,
                        value_type: ValueType::I32,
                    }],
                }],
            }
        );
    }

    #[test]
    fn it_decode_sum() {
        // (func $func0 (param $var0 i32) (result i32)
        //   (local $var1 i32) (local $var2 i32)
        //   loop $label1
        //     block $label0
        //       get_local $var1
        //       get_local $var0
        //       i32.ge_s
        //       br_if $label0
        //       get_local $var1
        //       get_local $var2
        //       i32.add
        //       set_local $var2
        //       get_local $var1
        //       i32.const 1
        //       i32.add
        //       set_local $var1
        //       br $label1
        //     end $label0
        //   end $label1
        //   get_local $var2
        // )
        let b = vec![
            0x01, 0x23, 0x01, 0x02, 0x7F, 0x03, 0x40, 0x02, 0x40, 0x20, 0x01, 0x20, 0x00, 0x4E,
            0x0D, 0x00, 0x20, 0x01, 0x20, 0x02, 0x6A, 0x21, 0x02, 0x20, 0x01, 0x41, 0x01, 0x6A,
            0x21, 0x01, 0x0C, 0x01, 0x0B, 0x0B, 0x20, 0x02, 0x0B,
        ];
        let mut cur = Cursor::new(b);
        let section = CodeSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            CodeSection {
                count: 1,
                bodies: vec![FunctionBody {
                    decoded: vec![
                        (Opcode::Loop, vec![Operand::ResultType(ResultType::Empty)]),
                        (Opcode::Block, vec![Operand::ResultType(ResultType::Empty)]),
                        (Opcode::GetLocal, vec![Operand::U32(1)]),
                        (Opcode::GetLocal, vec![Operand::U32(0)]),
                        (Opcode::I32GeS, vec![]),
                        (Opcode::BrIf, vec![Operand::U32(0)]),
                        (Opcode::GetLocal, vec![Operand::U32(1)]),
                        (Opcode::GetLocal, vec![Operand::U32(2)]),
                        (Opcode::I32Add, vec![]),
                        (Opcode::SetLocal, vec![Operand::U32(2)]),
                        (Opcode::GetLocal, vec![Operand::U32(1)]),
                        (Opcode::I32Const, vec![Operand::I32(1)]),
                        (Opcode::I32Add, vec![]),
                        (Opcode::SetLocal, vec![Operand::U32(1)]),
                        (Opcode::Br, vec![Operand::U32(1)]),
                        (Opcode::End, vec![]),
                        (Opcode::End, vec![]),
                        (Opcode::GetLocal, vec![Operand::U32(2)]),
                    ],
                    locals: vec![LocalEntry {
                        count: 2,
                        value_type: ValueType::I32,
                    }],
                }],
            }
        );
    }
}
