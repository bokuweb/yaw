use num_derive::*;

#[derive(Debug, PartialEq)]
pub struct TypeSection {
	count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ExternalKind {
	Function = 0x00,
	Table = 0x01,
	Memory = 0x02,
	Global = 0x03,
}
