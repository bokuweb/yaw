use std::io::Read;

use super::number::*;
use super::InitExpr;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct ElementSegment {
    pub table_index: u32,
    pub offset: InitExpr,
    pub elems: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementSection {
    pub entries: Vec<ElementSegment>,
}

impl Decoder for ElementSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u32 = VarUint32::decode(reader)?.into();
        let mut entries: Vec<ElementSegment> = vec![];
        for _ in 0..count {
            let table_index: u32 = VarUint32::decode(reader)?.into();
            let offset = InitExpr::new(reader)?;
            let num: u32 = VarUint32::decode(reader)?.into();
            let mut elems: Vec<u32> = vec![];
            for _ in 0..num {
                elems.push(VarUint32::decode(reader)?.into())
            }
            entries.push(ElementSegment {
                table_index,
                offset,
                elems,
            })
        }
        Ok(ElementSection { entries })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::RuntimeValue;
    use std::io::Cursor;

    #[test]
    fn test_elem_section() {
        // (elem (i32.const 0) $f1 $f2)
        let b = vec![0x01, 0x00, 0x41, 0x00, 0x0B, 0x02, 0x00, 0x01];
        let mut cur = Cursor::new(b);
        let section = ElementSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            ElementSection {
                entries: vec![ElementSegment {
                    table_index: 0,
                    offset: InitExpr::RuntimeValue(RuntimeValue::I32(0)),
                    elems: vec![0x00, 0x01]
                }]
            }
        );
    }
}
