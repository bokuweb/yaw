use num_traits::*;
use std::io::Read;
use std::str::from_utf8;

use crate::reader::*;

use super::number::*;
use super::types::*;
use super::{DecodeError, Decoder};
use super::{GlobalType, MemoryType, TableType};

#[derive(Debug, Clone, PartialEq)]
pub enum ImportType {
	Function(u32),
	Table(TableType),
	Memory(MemoryType),
	Global(GlobalType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportEntry {
	pub module_name: String,
	pub field_name: String,
	pub import_type: ImportType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSection {
	pub count: u32,
	pub entries: Vec<ImportEntry>,
}

impl Decoder for ImportSection {
	type Error = DecodeError;

	fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let count: u32 = VarUint32::decode(reader)?.into();
		let mut entries: Vec<ImportEntry> = vec![];
		for _ in 0..count {
			let module_len: usize = VarUint32::decode(reader)?.into();
			let bytes = read_bytes(reader, module_len)?;
			let module_name = from_utf8(&bytes)?;
			let field_len: usize = VarUint32::decode(reader)?.into();
			let bytes = read_bytes(reader, field_len)?;
			let field_name = from_utf8(&bytes)?;
			let kind = read_next(reader)?;
			let kind = ExternalKind::from_u8(kind).expect("should convert u8 to external kind");
			let import_type = match kind {
				ExternalKind::Function => {
					let index: u32 = VarUint32::decode(reader)?.into();
					ImportType::Function(index)
				}
				ExternalKind::Table => ImportType::Table(TableType::from_buffer(reader)?),
				ExternalKind::Memory => ImportType::Memory(MemoryType::new(reader)?),
				ExternalKind::Global => ImportType::Global(GlobalType::new(reader)?),
			};

			entries.push(ImportEntry {
				module_name: module_name.to_owned(),
				field_name: field_name.to_owned(),
				import_type,
			});
		}
		Ok(ImportSection { count, entries })
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::types::*;
	use std::io::Cursor;

	#[test]
	fn test_import_global() {
		// (import "spectest" "global_i32" (global $global0 i32))
		// [0x02, 0x18, 0x01, 0x08, 0x73, 0x70, 0x65, 0x63, 0x74, 0x65, 0x73, 0x74, 0x0A, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x5F, 0x69, 0x33, 0x32, 0x03, 0x7F, 0x00]
		let b = vec![
			0x01, 0x08, 0x73, 0x70, 0x65, 0x63, 0x74, 0x65, 0x73, 0x74, 0x0A, 0x67, 0x6C, 0x6F,
			0x62, 0x61, 0x6C, 0x5F, 0x69, 0x33, 0x32, 0x03, 0x7F, 0x00,
		];
		let mut cur = Cursor::new(b);
		let section = ImportSection::decode(&mut cur).unwrap();

		assert_eq!(
			section,
			ImportSection {
				count: 1,
				entries: vec![ImportEntry {
					module_name: "spectest".to_owned(),
					field_name: "global_i32".to_owned(),
					import_type: ImportType::Global(GlobalType {
						mutability: false,
						value_type: ValueType::I32,
					})
				},]
			}
		);
	}

	#[test]
	fn test_import_memory() {
		//  (import "env" "memory" (memory $memory0 1))
		// [0x02, 0x0F, 0x01, 0x03, 0x65, 0x6E, 0x76, 0x06, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79, 0x02, 0x00, 0x01]
		let b = vec![
			0x01, 0x03, 0x65, 0x6E, 0x76, 0x06, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79, 0x02, 0x00,
			0x01,
		];
		let mut cur = Cursor::new(b);
		let section = ImportSection::decode(&mut cur).unwrap();

		assert_eq!(
			section,
			ImportSection {
				count: 1,
				entries: vec![ImportEntry {
					module_name: "env".to_owned(),
					field_name: "memory".to_owned(),
					import_type: ImportType::Memory(MemoryType {
						limits: ResizableLimits::new(1, None),
					}),
				}]
			}
		)
	}
}
