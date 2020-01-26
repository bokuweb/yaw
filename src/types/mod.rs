pub mod elem_type;
pub mod memory_type;
pub mod operand;
pub mod runtime_value;
pub mod table_type;

pub(crate) mod resizable_limits;
pub(crate) mod value_type;

pub use elem_type::*;
pub use operand::*;
pub use resizable_limits::*;
pub use runtime_value::*;
pub use table_type::*;
pub use value_type::*;

use num_derive::*;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ResultType {
  I32 = 0x7F,
  I64 = 0x7E,
  F32 = 0x7D,
  F64 = 0x7C,
  Empty = 0x40,
}
