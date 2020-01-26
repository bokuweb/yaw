use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSection {
    pub count: u32,
    pub types: Vec<u32>,
}

impl Decoder for FunctionSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u32 = VarUint32::decode(reader)?.into();
        let mut types: Vec<u32> = vec![];
        for _ in 0..count {
            let index: u32 = VarUint32::decode(reader)?.into();
            types.push(index);
        }
        Ok(FunctionSection { count, types })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_function_section() {
        let b = vec![0x06, 0x00, 0x01, 0x02, 0x00, 0x02, 0x01];
        let mut cur = Cursor::new(b);
        let section = FunctionSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            FunctionSection {
                count: 6,
                types: vec![0x00, 0x01, 0x02, 0x00, 0x02, 0x01],
            }
        );
    }
}
