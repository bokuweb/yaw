use num_traits::*;
use std::io::Read;

use crate::reader::*;
use crate::types::*;

use super::number::*;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
	pub args: Vec<ValueType>,
	pub results: Vec<ResultType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSection {
	pub count: u32,
	pub entries: Vec<FuncType>,
}

impl Default for TypeSection {
	fn default() -> Self {
		Self {
			count: 0,
			entries: vec![],
		}
	}
}

impl Decoder for TypeSection {
	type Error = DecodeError;

	fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let count: u32 = VarUint32::decode(reader)?.into();
		let mut entries: Vec<FuncType> = vec![];
		for _ in 0..count {
			if read_next(reader)? != 0x60 {
				return Err(DecodeError::InvalidTypeSectionError);
			}
			let arg_len = read_next(reader)?;
			let mut args: Vec<ValueType> = vec![];
			for _ in 0..arg_len {
				let arg = read_next(reader)?;
				args.push(ValueType::from_u8(arg).expect("should convert u8 to value type"));
			}
			let result_len = read_next(reader)?;
			let mut results: Vec<ResultType> = vec![];
			for _ in 0..result_len {
				let result = read_next(reader)?;
				results
					.push(ResultType::from_u8(result).expect("should convert u8 to result type"));
			}
			entries.push(FuncType { args, results });
		}
		Ok(TypeSection { count, entries })
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use std::io::Cursor;

	#[test]
	fn i32_two_args_i32_result() {
		// (i32, i32): (i32)
		let b = vec![0x01, 0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F];
		let mut cur = Cursor::new(b);
		let section = TypeSection::decode(&mut cur).unwrap();
		assert_eq!(
			section,
			TypeSection {
				count: 1,
				entries: vec![FuncType {
					args: vec![ValueType::I32, ValueType::I32],
					results: vec![ResultType::I32]
				}]
			}
		);
	}
}
