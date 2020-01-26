use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct StartSection {
    pub index: u32,
}

impl Decoder for StartSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let index: u32 = VarUint32::decode(reader)?.into();
        Ok(StartSection { index })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_start_section() {
        let b = vec![0x00];
        let mut cur = Cursor::new(b);
        let section = StartSection::decode(&mut cur).unwrap();
        assert_eq!(section, StartSection { index: 0 });
    }
}
