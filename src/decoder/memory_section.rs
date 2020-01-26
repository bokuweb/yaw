use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

use crate::types::ResizableLimits;

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}

impl MemoryType {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let flags: u8 = VarUint7::decode(reader)?.into();
        let initial: u32 = VarUint32::decode(reader)?.into();
        let maximum: Option<u32> = if flags != 0 {
            let m: u32 = VarUint32::decode(reader)?.into();
            Some(m)
        } else {
            None
        };
        let limits = ResizableLimits::new(initial, maximum);
        Ok(MemoryType { limits })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemorySection {
    pub entries: Vec<MemoryType>,
}

impl Decoder for MemorySection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u8 = VarUint7::decode(reader)?.into();
        if count > 1 {
            return Err(DecodeError::InvalidMemoryCountError);
        }
        let mut entries = vec![];
        for _ in 0..count {
            let memory_type = MemoryType::new(reader)?;
            entries.push(memory_type)
        }
        Ok(MemorySection { entries })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_memory_section() {
        // (memory $M0 3 10))
        // [0x05, 0x04, 0x01, 0x01, 0x03, 0x0A]
        let b = vec![0x01, 0x70, 0x03, 0x0A];
        let mut cur = Cursor::new(b);
        let section = MemorySection::decode(&mut cur).unwrap();

        assert_eq!(
            section,
            MemorySection {
                entries: vec![MemoryType {
                    limits: ResizableLimits::new(3, Some(10)),
                }],
            },
        );
    }
}
