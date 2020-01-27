use std::io::Read;
use std::str::from_utf8;

use crate::reader::*;

use super::number::*;
use super::{DecodeError, Decoder};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomSection {
	pub name: String,
	pub payload: Vec<u8>,
}

impl Decoder for CustomSection {
	type Error = DecodeError;

	fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let name_len: u32 = VarUint32::decode(reader)?.into();
		let bytes = read_bytes(reader, name_len as usize)?;
		dbg!(name_len, &bytes);
		let name = from_utf8(&bytes)?;
		dbg!(&name);
		let mut payload = vec![];
		reader.read_to_end(&mut payload)?;
		Ok(CustomSection {
			name: name.to_owned(),
			payload,
		})
	}
}
