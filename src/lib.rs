#[macro_use]
extern crate failure;

pub mod error;
pub mod types;
pub mod vm;

mod decoder;
mod reader;

use std::io::Read;

pub use decoder::ExternalKind;
pub use error::YawError;
pub use types::*;
pub use vm::{
    ExportType, Exports, FunctionResolver, Global, ImportResolver, Imports, Memory,
    MemoryDescriptor, MemoryRef, RuntimeError, TableInstance, TableRef, VM,
};

pub fn instantiate<B: AsRef<[u8]>>(
    buf: B,
    imports: Option<&dyn ImportResolver>,
) -> Result<VM<'_>, error::YawError> {
    let mut magic_number = [0; 4];
    let mut reader = buf.as_ref();
    reader.read_exact(&mut magic_number)?;
    let magic_number = String::from_utf8(magic_number.to_vec())?;
    if magic_number != "\0asm" {
        return Err(error::YawError::InvalidFileError);
    }
    let mut ver = [0; 4];
    reader.read_exact(&mut ver)?;
    if u32::from_ne_bytes(ver) != 0x0000_0001 {
        return Err(error::YawError::InvalidFileError);
    }
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let sections = decoder::decode(&buf)?;

    Ok(VM::from_section(sections, imports)?)
}
