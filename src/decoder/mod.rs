pub mod export_section;
pub mod number;

mod code_section;
mod custom_section;
mod data_section;
mod element_section;
mod function_section;
mod global_section;
mod import_section;
mod init_expr;
mod initial_value;
mod memory_section;
mod start_section;
mod table_section;
mod type_section;
mod types;

pub use code_section::{CodeSection, Instruction, LocalEntry};
pub use custom_section::CustomSection;
pub use data_section::DataSection;
pub use element_section::ElementSection;
pub use export_section::ExportSection;
pub use function_section::FunctionSection;
pub use global_section::{GlobalSection, GlobalType};
pub use import_section::{ImportSection, ImportType};
pub use init_expr::InitExpr;
pub use memory_section::{MemorySection, MemoryType};
pub use number::*;
pub use start_section::StartSection;
pub use table_section::{TableSection, TableType};
pub use type_section::{FuncType, TypeSection};
pub use types::*;

use initial_value::*;
use num_derive::*;
use std::io::{Cursor, Error, Read};
use std::str::Utf8Error;

#[derive(Debug, Fail)]
pub enum DecodeError {
    #[fail(display = "Invalid VarUint32 format Error")]
    InvalidVarUint32Error,

    #[fail(display = "Invalid VarUint64 format Error")]
    InvalidVarUint64Error,

    #[fail(display = "Invalid VarInt32 format Error")]
    InvalidVarInt32Error,

    #[fail(display = "Invalid VarInt64 format Error")]
    InvalidVarInt64Error,

    #[fail(display = "Invalid Section Kind format Error")]
    InvalidSectionKindFormatError,

    #[fail(display = "Invalid Type Section format Error")]
    InvalidTypeSectionError,

    #[fail(display = "Invalid ContentType Error")]
    InvalidContentTypeError,

    #[fail(display = "Invalid Initializer Error")]
    InvalidInitializerError,

    #[fail(display = "Invalid element Type Error")]
    InvalidElementTypeError,

    #[fail(display = "Invalid table count Error: table count must one or less in MVP")]
    InvalidTableCountError,

    #[fail(display = "Invalid memory count Error: memory count must one or less in MVP")]
    InvalidMemoryCountError,

    #[fail(display = "invalid value type error")]
    InvalidValueTypeError,

    #[fail(display = "invalid result type error")]
    InvalidResultTypeError,

    #[fail(display = "invalid opcode error")]
    InvalidOpcodeError,

    #[fail(display = "Some I/O Error: {:?}", error)]
    IOError { error: Error },

    #[fail(display = "utf-8 8Error: {:?}", error)]
    Utf8Error { error: Utf8Error },
}

impl From<Error> for DecodeError {
    fn from(error: Error) -> Self {
        DecodeError::IOError { error }
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(error: Utf8Error) -> Self {
        DecodeError::Utf8Error { error }
    }
}

pub trait Decoder: Sized {
    type Error: From<Error>;
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SectionKind {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Table = 0x04,
    Memory = 0x05,
    Global = 0x06,
    Export = 0x07,
    Start = 0x08,
    Element = 0x09,
    Code = 0x0A,
    Data = 0x0B,
}

#[derive(Debug)]
pub struct Sections {
    pub(crate) custom_section: Option<CustomSection>,
    pub(crate) type_section: Option<TypeSection>,
    pub(crate) import_section: Option<ImportSection>,
    pub(crate) function_section: Option<FunctionSection>,
    pub(crate) table_section: Option<TableSection>,
    pub(crate) memory_section: Option<MemorySection>,
    pub(crate) global_section: Option<GlobalSection>,
    pub(crate) export_section: Option<ExportSection>,
    pub(crate) start_section: Option<StartSection>,
    pub(crate) code_section: Option<CodeSection>,
    pub(crate) data_section: Option<DataSection>,
    pub(crate) element_section: Option<ElementSection>,
}

pub fn decode(buf: &[u8]) -> Result<Sections, DecodeError> {
    let mut cur = Cursor::new(buf);
    let mut sections = Sections {
        custom_section: None,
        type_section: None,
        import_section: None,
        function_section: None,
        table_section: None,
        memory_section: None,
        global_section: None,
        export_section: None,
        element_section: None,
        start_section: None,
        code_section: None,
        data_section: None,
    };
    loop {
        let len = cur.get_ref().len() as u64;
        if cur.position() == len {
            break;
        }
        let kind: u8 = VarUint7::decode(&mut cur)?.into();
        let len: usize = VarUint32::decode(&mut cur)?.into();
        let mut body = vec![0u8; len];
        cur.read_exact(&mut body)?;
        let mut buf = Cursor::new(body);
        dbg!(kind, len);
        match num_traits::FromPrimitive::from_u8(kind)
            .ok_or(DecodeError::InvalidSectionKindFormatError)?
        {
            SectionKind::Custom => sections.custom_section = Some(CustomSection::decode(&mut buf)?),
            SectionKind::Type => sections.type_section = Some(TypeSection::decode(&mut buf)?),
            SectionKind::Import => sections.import_section = Some(ImportSection::decode(&mut buf)?),
            SectionKind::Function => {
                sections.function_section = Some(FunctionSection::decode(&mut buf)?)
            }
            SectionKind::Table => sections.table_section = Some(TableSection::decode(&mut buf)?),
            SectionKind::Memory => sections.memory_section = Some(MemorySection::decode(&mut buf)?),
            SectionKind::Global => sections.global_section = Some(GlobalSection::decode(&mut buf)?),
            SectionKind::Element => {
                sections.element_section = Some(ElementSection::decode(&mut buf)?)
            }
            SectionKind::Export => sections.export_section = Some(ExportSection::decode(&mut buf)?),
            SectionKind::Start => sections.start_section = Some(StartSection::decode(&mut buf)?),
            SectionKind::Code => sections.code_section = Some(CodeSection::decode(&mut buf)?),
            SectionKind::Data => sections.data_section = Some(DataSection::decode(&mut buf)?),
        }
    }
    Ok(sections)
}
