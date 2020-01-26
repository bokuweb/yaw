use num_traits::*;
use std::io::Read;

use super::number::*;
use super::{DecodeError, Decoder};

use super::read_initial_value;
use crate::reader::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalType {
    pub mutability: bool,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariable {
    pub global_type: GlobalType,
    pub initial_value: RuntimeValue,
}

impl GlobalType {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let content_type = read_next(reader)?;
        let content_type = ValueType::from_u8(content_type).unwrap();
        let mutability = read_next(reader)? != 0;
        Ok(GlobalType {
            mutability,
            value_type: content_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSection {
    pub globals: Vec<GlobalVariable>,
}

impl Decoder for GlobalSection {
    type Error = DecodeError;

    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let count: u8 = VarUint7::decode(reader)?.into();
        let mut globals: Vec<GlobalVariable> = vec![];
        for _ in 0..count {
            let global_type = GlobalType::new(reader)?;
            // TODO: fix to use initializer
            let initial_value = read_initial_value(reader)?;
            globals.push(GlobalVariable {
                global_type,
                initial_value,
            });
        }
        Ok(GlobalSection { globals })
    }
}

#[cfg(test)]
mod tests {

    use super::{Decoder, GlobalSection, GlobalType, GlobalVariable, RuntimeValue, ValueType};
    use std::io::Cursor;

    #[test]
    fn test_global_section_with_i32() {
        let b = vec![0x01, 0x7F, 0x00, 0x41, 0x01, 0x0B];
        let mut cur = Cursor::new(b);
        let section = GlobalSection::decode(&mut cur).unwrap();

        assert_eq!(
            section,
            GlobalSection {
                globals: vec![GlobalVariable {
                    global_type: GlobalType {
                        mutability: false,
                        value_type: ValueType::I32,
                    },
                    initial_value: RuntimeValue::I32(1),
                }],
            }
        );
    }

    #[test]
    fn test_global_section_with_f32_mut() {
        let b = vec![0x01, 0x7D, 0x01, 0x43, 0x7B, 0x14, 0x8E, 0x3F, 0x0B];
        let mut cur = Cursor::new(b);
        let section = GlobalSection::decode(&mut cur).unwrap();

        assert_eq!(
            section,
            GlobalSection {
                globals: vec![GlobalVariable {
                    global_type: GlobalType {
                        mutability: true,
                        value_type: ValueType::F32,
                    },
                    initial_value: RuntimeValue::F32(1.11),
                }],
            }
        );
    }
}
