use num_traits::*;
use std::io::Read;

use crate::reader::*;

use super::number::*;
use super::types::*;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct ExportEntry {
    pub name: String,
    pub kind: ExternalKind,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSection {
    pub count: u32,
    pub entries: Vec<ExportEntry>,
}

impl Decoder for ExportSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u32 = VarUint32::decode(reader)?.into();
        let mut entries: Vec<ExportEntry> = vec![];
        for _ in 0..count {
            let field_len = VarUint32::decode(reader)?.into();
            let name = read_bytes(reader, field_len)?;
            let name = String::from_utf8(name).expect("should convert name to utf8");
            let kind = read_next(reader)?;
            let kind = ExternalKind::from_u8(kind).expect("should convert u8 to external kind");
            let index: u32 = VarUint32::decode(reader)?.into();
            entries.push(ExportEntry { name, kind, index });
        }
        Ok(ExportSection { count, entries })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_export_add() {
        let b = vec![0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00];
        let mut cur = Cursor::new(b);
        let section = ExportSection::decode(&mut cur).unwrap();
        assert_eq!(
            section,
            ExportSection {
                count: 1,
                entries: vec![ExportEntry {
                    name: "add".to_owned(),
                    kind: ExternalKind::Function,
                    index: 0,
                }]
            }
        );
    }
}
