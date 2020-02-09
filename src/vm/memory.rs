use std::cell::RefCell;
use std::io::{Cursor, Read, Write};
use std::rc::Rc;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{Globals, ImportResolver, ImportType, RuntimeError, Sections};

#[derive(Debug)]
pub struct MemoryRef(Rc<RefCell<Memory>>);

#[derive(Debug)]
pub struct Memory {
    buf: Vec<u8>,
    maximum: Option<u32>,
}

#[derive(Debug)]
pub struct MemoryDescriptor {
    initial: u32,
    maximum: Option<u32>,
}

impl MemoryDescriptor {
    pub fn new(initial: u32, maximum: Option<u32>) -> MemoryDescriptor {
        MemoryDescriptor { initial, maximum }
    }
}

pub const PAGE_SIZE: usize = 0x10000;

impl Clone for MemoryRef {
    fn clone(&self) -> MemoryRef {
        MemoryRef(Rc::clone(&self.0))
    }
}

macro_rules! load {
    ($name: ident, $reader: ident, $ret_type: ty) => {
        impl MemoryRef {
            pub fn $name(&self, addr: u32) -> Result<$ret_type, RuntimeError> {
                let m = self.0.borrow();
                let mut cur = Cursor::new(&m.buf);
                cur.set_position(addr as u64);
                let v = cur.$reader::<LittleEndian>()?;
                Ok(v)
            }
        }
    };
}

macro_rules! store {
    ($name: ident, $writer: ident, $data_type: ty) => {
        impl MemoryRef {
            pub fn $name(&self, addr: u32, data: $data_type) -> Result<(), RuntimeError> {
                let mut m = self.0.borrow_mut();
                let mut cur = Cursor::new(&mut m.buf);
                cur.set_position(addr as u64);
                &mut cur.$writer::<LittleEndian>(data)?;
                Ok(())
            }
        }
    };
}

impl MemoryRef {
    pub fn new(desc: MemoryDescriptor) -> MemoryRef {
        MemoryRef(Rc::new(RefCell::new(Memory {
            buf: vec![0; desc.initial as usize * PAGE_SIZE],
            maximum: desc.maximum,
        })))
    }

    pub fn to_string(&self, start: usize, len: usize) -> Result<String, std::str::Utf8Error> {
        let b = self.0.borrow();
        let b = &b.buf[start..start + len];
        let s = std::str::from_utf8(b)?;
        Ok(s.to_owned())
    }

    pub fn grow(&self, delta: u32) -> i32 {
        let mut m = self.0.borrow_mut();
        let current = m.buf.len() / PAGE_SIZE;
        if let Some(max) = m.maximum {
            if current + delta as usize > max as usize {
                return -1;
            }
        }
        let len = m.buf.len() + (delta as usize * PAGE_SIZE);
        m.buf.resize(len, 0);
        current as i32
    }

    pub fn current(&self) -> usize {
        let m = self.0.borrow();
        m.buf.len() / PAGE_SIZE
    }

    pub fn set(&self, data: &[u8], offset: usize) -> Result<(), RuntimeError> {
        let mut m = self.0.borrow_mut();
        let mut cur = Cursor::new(&mut m.buf);
        cur.set_position(offset as u64);
        for d in data {
            let b = [*d as u8; 1];
            cur.write_all(&b)?;
        }
        Ok(())
    }

    pub fn i8_load(&self, addr: u32) -> Result<i8, RuntimeError> {
        let m = self.0.borrow();
        let mut cur = Cursor::new(&m.buf);
        cur.set_position(u64::from(addr));
        let mut b = [0u8; 1];
        cur.read_exact(&mut b)?;
        Ok(b[0] as i8)
    }

    pub fn i8_store(&self, addr: u32, data: i8) -> Result<(), RuntimeError> {
        let mut m = self.0.borrow_mut();
        let mut cur = Cursor::new(&mut m.buf);
        cur.set_position(u64::from(addr));
        let b = [data as u8; 1];
        cur.write_all(&b)?;
        Ok(())
    }
}

load!(i16_load, read_i16, i16);
load!(i32_load, read_i32, i32);
load!(i64_load, read_i64, i64);
load!(f32_load, read_f32, f32);
load!(f64_load, read_f64, f64);

store!(i32_store, write_i32, i32);
store!(u32_store, write_u32, u32);
store!(i64_store, write_i64, i64);
store!(f32_store, write_f32, f32);
store!(f64_store, write_f64, f64);
store!(i16_store, write_i16, i16);

#[derive(Debug)]
pub struct Memories(Vec<MemoryRef>);

impl Memories {
    pub fn from_section<'a>(
        sections: &Sections,
        globals: &Globals,
        imports: Option<&'a dyn ImportResolver>,
    ) -> Result<Self, RuntimeError> {
        let mut memories = vec![];
        let mut segments = &vec![];
        if let Some(data) = sections.data_section.as_ref() {
            segments = &data.segments
        }

        if let Some(import_section) = sections.import_section.as_ref() {
            if let Some(imports) = &imports {
                for entry in &import_section.entries {
                    if let ImportType::Memory(_m) = &entry.import_type {
                        let memory_ref = imports
                            .resolve_memory(entry.module_name.clone(), entry.field_name.clone())?;
                        // For now support only one memory
                        for s in segments {
                            memory_ref.set(&s.data, s.offset.eval(globals)?.into())?;
                        }
                        memories.push(memory_ref.clone());
                    }
                }
            }
        }

        if let Some(m) = sections.memory_section.as_ref() {
            let m = &m.entries[0];
            let m = MemoryRef::new(MemoryDescriptor::new(m.limits.initial, m.limits.maximum));
            // For now support only one memory
            for s in segments {
                m.set(&s.data, s.offset.eval(globals)?.into())?;
            }
            memories.push(m)
        }
        Ok(Self(memories))
    }

    pub fn get(&self, index: usize) -> Result<MemoryRef, RuntimeError> {
        let m = self
            .0
            .get(index)
            .ok_or(RuntimeError::UndefinedMemoryError)?;
        Ok(m.clone())
    }
}
