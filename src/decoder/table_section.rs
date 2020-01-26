use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

use crate::types::{ElemType, ResizableLimits};

#[derive(Debug, Clone, PartialEq)]
pub struct TableType {
    pub elem_type: ElemType,
    pub limits: ResizableLimits,
}

impl TableType {
    // pub fn new(initial: u32, max: Option<u32>) -> TableType {
    //     let elem_type = ElemType::AnyFunc;
    //     let limits = ResizableLimits::new(initial, max);
    //     TableType { elem_type, limits }
    // }
    pub(crate) fn from_buffer<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let elem_type: u8 = VarUint7::decode(reader)?.into();
        let elem_type = num_traits::FromPrimitive::from_u8(elem_type)
            .ok_or(DecodeError::InvalidElementTypeError)?;
        let flags: u8 = VarUint7::decode(reader)?.into();
        let initial: u32 = VarUint32::decode(reader)?.into();
        let maximum: Option<u32> = if flags != 0 {
            let m: u32 = VarUint32::decode(reader)?.into();
            Some(m)
        } else {
            None
        };
        let limits = ResizableLimits::new(initial, maximum);
        Ok(TableType { elem_type, limits })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableSection {
    pub entries: Vec<TableType>,
}

impl Decoder for TableSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u8 = VarUint7::decode(reader)?.into();
        if count > 1 {
            return Err(DecodeError::InvalidTableCountError);
        }
        let mut entries = vec![];
        for _ in 0..count {
            let table_type = TableType::from_buffer(reader)?;
            entries.push(table_type)
        }
        Ok(TableSection { entries })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_table_section_with_maximum() {
        // (table $T0 1 1 anyfunc)
        // [0x04, 0x05, 0x01, 0x70, 0x01, 0x01, 0x01]
        let b = vec![0x01, 0x70, 0x01, 0x01, 0x01];
        let mut cur = Cursor::new(b);
        let section = TableSection::decode(&mut cur).unwrap();

        assert_eq!(
            section,
            TableSection {
                entries: vec![TableType {
                    elem_type: ElemType::AnyFunc,
                    limits: ResizableLimits::new(1, Some(1)),
                }]
            }
        );
    }
}
