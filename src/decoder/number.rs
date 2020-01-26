use crate::reader::*;
use std::*;

use super::{DecodeError, Decoder};
use crate::types::RuntimeValue;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarInt32(i32);

impl From<VarInt32> for i32 {
	fn from(v: VarInt32) -> i32 {
		v.0
	}
}

impl From<VarInt32> for RuntimeValue {
	fn from(v: VarInt32) -> RuntimeValue {
		RuntimeValue::I32(v.0)
	}
}

impl Decoder for VarInt32 {
	type Error = DecodeError;

	fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut value = 0;
		let mut shift = 0;
		loop {
			if shift > 31 {
				return Err(DecodeError::InvalidVarInt32Error);
			}
			let b = u32::from(read_next(reader)?);
			value |= ((b & 0x7f) as i32)
				.checked_shl(shift)
				.ok_or(DecodeError::InvalidVarInt32Error)?;

			shift += 7;
			if (b >> 7) != 0 {
				continue;
			}
			if shift < 32 && b & 0b0100_0000 != 0 {
				value |= (1i32 << shift).wrapping_neg();
			}

			if shift > 32 && b & 0b0100_0000 != 0 && !(b as u8 | 0b1000_0000) >= 0x10 {
				return Err(DecodeError::InvalidVarInt32Error);
			}
			if shift > 32 && b & 0b0100_0000 == 0 && b >= 0x10 {
				return Err(DecodeError::InvalidVarInt32Error);
			}
			break;
		}
		Ok(VarInt32(i32::from_ne_bytes(value.to_ne_bytes())))
	}
}

#[derive(Debug, Copy, Clone)]
pub struct VarUint32(u32);

impl From<VarUint32> for usize {
	fn from(v: VarUint32) -> usize {
		v.0 as usize
	}
}

impl From<VarUint32> for u32 {
	fn from(v: VarUint32) -> u32 {
		v.0
	}
}

impl Decoder for VarUint32 {
	type Error = DecodeError;

	fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut value = 0;
		let mut i = 0;
		loop {
			let b = u32::from(read_next(reader)?);
			value |= (b & 0x7f)
				.checked_shl(i * 7)
				.ok_or(DecodeError::InvalidVarUint32Error)?;
			i += 1;
			if i > 5 {
				return Err(DecodeError::InvalidVarUint32Error);
			}
			if b & 0x80 == 0 {
				if i == 5 && b >= 0x10 {
					return Err(DecodeError::InvalidVarUint32Error);
				}
				break;
			}
		}
		Ok(VarUint32(value))
	}
}

#[derive(Debug, Copy, Clone)]
pub struct VarUint7(u8);

impl From<VarUint7> for u8 {
	fn from(v: VarUint7) -> u8 {
		v.0
	}
}

impl From<VarUint7> for usize {
	fn from(v: VarUint7) -> usize {
		v.0 as usize
	}
}

impl Decoder for VarUint7 {
	type Error = DecodeError;

	fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut u8buf = [0u8; 1];
		reader.read_exact(&mut u8buf)?;
		Ok(VarUint7(u8buf[0]))
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarInt64(i64);

impl From<VarInt64> for i64 {
	fn from(v: VarInt64) -> i64 {
		v.0
	}
}

impl From<VarInt64> for RuntimeValue {
	fn from(v: VarInt64) -> RuntimeValue {
		RuntimeValue::I64(v.0)
	}
}

impl Decoder for VarInt64 {
	type Error = DecodeError;

	fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut v: i64 = 0;
		let mut shift = 0;
		loop {
			if shift > 63 {
				return Err(DecodeError::InvalidVarInt64Error);
			}
			let b = u64::from(read_next(reader)?);
			v |= ((b & 0x7f) as i64)
				.checked_shl(shift)
				.ok_or(DecodeError::InvalidVarInt64Error)?;

			shift += 7;
			if (b >> 7) != 0 {
				continue;
			}
			if shift < 64 && b & 0b0100_0000 == 0b0100_0000 {
				// Fill 1 when less than 0
				v |= (1i64 << shift).wrapping_neg();
			}

			if shift > 64 {
				if b & 0b0100_0000 != 0 {
					if !(b as u8 | 0b1000_0000) > 0x01 {
						return Err(DecodeError::InvalidVarInt64Error);
					}
				} else if b > 0x01 {
					return Err(DecodeError::InvalidVarInt64Error);
				}
			}
			break;
		}
		Ok(VarInt64(v))
	}
}

#[derive(Debug, Copy, Clone)]
pub struct VarUint64(u64);

impl From<VarUint64> for u64 {
	fn from(v: VarUint64) -> u64 {
		v.0 as u64
	}
}

impl Decoder for VarUint64 {
	type Error = DecodeError;

	fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut v: u64 = 0;
		let mut shift = 0;
		loop {
			if shift > 63 {
				return Err(DecodeError::InvalidVarUint64Error);
			}
			let b = u64::from(read_next(reader)?);
			v |= ((b & 0x7f) as u64)
				.checked_shl(shift)
				.ok_or(DecodeError::InvalidVarUint64Error)?;

			shift += 7;
			if b & 0x80 != 0 {
				continue;
			}
			if shift > 64 && b > 0x01 {
				return Err(DecodeError::InvalidVarUint64Error);
			}
			break;
		}
		Ok(VarUint64(v))
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use std::io::Cursor;
	#[test]
	fn byte_to_varuint32() {
		let b = vec![0x08];
		let mut cur = Cursor::new(b);
		let v: usize = VarUint32::decode(&mut cur).unwrap().into();
		assert_eq!(v, 0x08);
	}

	#[test]
	fn ff_to_varuint32() {
		let b = vec![0xFF, 0x01];
		let mut cur = Cursor::new(b);
		let v: usize = VarUint32::decode(&mut cur).unwrap().into();
		assert_eq!(v, 0xFF);
	}

	// Please see https://en.wikipedia.org/wiki/LEB128
	#[test]
	fn three_bytes_to_varuint32() {
		let b = vec![0xE5, 0x8E, 0x26];
		let mut cur = Cursor::new(b);
		let v: usize = VarUint32::decode(&mut cur).unwrap().into();
		assert_eq!(v, 624_485);
	}

	#[test]
	fn four_bytes_to_varuint32() {
		let b = vec![0x80, 0x80, 0x80, 0x3F];
		let mut cur = Cursor::new(b);
		let v: usize = VarUint32::decode(&mut cur).unwrap().into();
		assert_eq!(v, 0x07E0_0000);
	}

	#[test]
	fn maxu32_to_varuint32() {
		let b = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F];
		let mut cur = Cursor::new(b);
		let v: u32 = VarUint32::decode(&mut cur).unwrap().into();
		assert_eq!(v, 0xFFFF_FFFF);
	}

	#[test]
	fn invalid32_to_varuint32() {
		let b = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x10];
		let mut cur = Cursor::new(b);
		let v = VarUint32::decode(&mut cur);
		assert!(v.is_err());
	}
	#[test]
	fn invalid_varuint32_format() {
		let b = vec![0x80, 0x80, 0x80, 0x80, 0x80];
		let mut cur = Cursor::new(b);
		let v = VarUint32::decode(&mut cur);
		assert!(v.is_err());
	}

	#[test]
	fn four_bytes_to_varint64() {
		let b = vec![0xDF, 0xF9, 0x6A];
		let mut cur = Cursor::new(b);
		let v: i64 = VarInt64::decode(&mut cur).unwrap().into();
		assert_eq!(v, -344_865);
	}

	#[test]
	fn min_i64_to_varint64() {
		let b = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F];
		let mut cur = Cursor::new(b);
		let v: i64 = VarInt64::decode(&mut cur).unwrap().into();
		assert_eq!(v, -9_223_372_036_854_775_808);
	}

	#[test]
	fn invalid_varint64_format() {
		let b = vec![
			0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F,
		];
		let mut cur = Cursor::new(b);
		let v = VarInt64::decode(&mut cur);
		assert!(v.is_err());
	}

	#[test]
	fn invalid_varuint64_format() {
		let b = vec![
			0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F,
		];
		let mut cur = Cursor::new(b);
		let v = VarUint64::decode(&mut cur);
		assert!(v.is_err());
	}

	#[test]
	fn u64_to_varuint64() {
		let b = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
		let mut cur = Cursor::new(b);
		let v: u64 = VarUint64::decode(&mut cur).unwrap().into();
		assert_eq!(v, 0xFFFF_FFFF_FFFF_FFFF);
	}
}
