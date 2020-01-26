use std::io::Read;

use super::number::*;
use super::InitExpr;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSegment {
    pub index: u32,
    pub offset: InitExpr,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    pub segments: Vec<DataSegment>,
}

impl Decoder for DataSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u8 = VarUint7::decode(reader)?.into();
        let mut segments: Vec<DataSegment> = vec![];
        for _ in 0..count {
            let index: u32 = VarUint32::decode(reader)?.into();
            let offset = InitExpr::new(reader)?;
            let size: usize = VarUint32::decode(reader)?.into();
            let mut body = vec![0u8; size];
            reader.read_exact(&mut body)?;
            segments.push(DataSegment {
                index,
                offset,
                data: body,
            });
        }
        Ok(DataSection { segments })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;
    use std::io::Cursor;

    #[test]
    fn test_global_section_with_i32() {
        let b = vec![0x01, 0x00, 0x41, 0x1, 0x0B, 0x01, 0xA5];
        let mut cur = Cursor::new(b);
        let section = DataSection::decode(&mut cur).unwrap();

        assert_eq!(
            section,
            DataSection {
                segments: vec![DataSegment {
                    offset: InitExpr::RuntimeValue(RuntimeValue::I32(1)),
                    index: 0x00,
                    data: vec![0xA5],
                }],
            }
        );
    }
}
