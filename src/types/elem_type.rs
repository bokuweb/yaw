use num_derive::*;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ElemType {
  AnyFunc = 0x70,
}

