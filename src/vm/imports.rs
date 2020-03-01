use super::memory::*;
use super::Global;
use super::TableRef;

use crate::types::RuntimeValue;
use crate::vm::*;
use std::cell::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::*;

pub trait ImportResolver: fmt::Debug {
    fn resolve_memory(
        &self,
        _module_name: String,
        _field_name: String,
    ) -> Result<MemoryRef, RuntimeError> {
        Err(RuntimeError::UndefinedMemoryError)
    }

    fn resolve_global(
        &self,
        _module_name: String,
        _field_name: String,
    ) -> Result<Rc<RefCell<Global>>, RuntimeError> {
        Err(RuntimeError::UndefinedGlobalError)
    }

    fn resolve_table(&self, _name: String, _field_name: String) -> Result<TableRef, RuntimeError> {
        Err(RuntimeError::UndefinedTableError)
    }

    fn resolve_function(
        &self, /*, _name: String */
    ) -> Result<&dyn FunctionResolver, RuntimeError> {
        Err(RuntimeError::UndefinedFunctionError)
    }
}

pub trait FunctionResolver: fmt::Debug {
    fn invoke(
        &self,
        _vm: &mut VM,
        _name: &str,
        _field_name: &str,
        _args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        Err(RuntimeError::UndefinedFunctionError)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Imports<'a> {
    func: Option<&'a dyn FunctionResolver>,
    memory: HashMap<String, HashMap<String, MemoryRef>>,
    // Please add `mutability` property when mutable global allowed.
    global: HashMap<String, HashMap<String, Rc<RefCell<Global>>>>,
    table: HashMap<String, HashMap<String, TableRef>>,
}

impl<'a> Imports<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_memory(
        &mut self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        memory: MemoryRef,
    ) {
        let mut m = HashMap::new();
        m.insert(field_name.into(), memory);
        self.memory.insert(name.into(), m);
    }

    pub fn add_function(&mut self, resolver: &'a dyn FunctionResolver) {
        self.func = Some(resolver);
    }

    pub fn add_global(
        &mut self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        value: Rc<RefCell<Global>>,
    ) {
        let module_name = name.into();
        let map = self.global.get_mut(&module_name);

        if let Some(m) = map {
            m.insert(field_name.into(), value);
        } else {
            let mut m = HashMap::new();
            m.insert(field_name.into(), value);
            self.global.insert(module_name, m);
        }
    }

    pub fn add_table(
        &mut self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        table: TableRef,
    ) {
        let mut m = HashMap::new();
        m.insert(field_name.into(), table);
        self.table.insert(name.into(), m);
    }
}

impl<'a> ImportResolver for Imports<'a> {
    fn resolve_memory(&self, name: String, field_name: String) -> Result<MemoryRef, RuntimeError> {
        let m = self.memory.get(&name);
        if let Some(map) = m {
            let mem = map.get(&field_name);
            if let Some(mem) = mem {
                return Ok(mem.clone());
            }
        }
        Err(RuntimeError::UndefinedMemoryError)
    }

    fn resolve_global(
        &self,
        name: String,
        field_name: String,
    ) -> Result<Rc<RefCell<Global>>, RuntimeError> {
        let m = self.global.get(&name);
        if let Some(map) = m {
            if let Some(value) = map.get(&field_name) {
                return Ok(Rc::clone(value));
            }
        }
        Err(RuntimeError::UndefinedGlobalError)
    }

    fn resolve_table(&self, name: String, field_name: String) -> Result<TableRef, RuntimeError> {
        let m = self.table.get(&name);
        if let Some(map) = m {
            if let Some(value) = map.get(&field_name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError::UndefinedTableError)
    }

    fn resolve_function(
        &self,
        // _name: String,
        // _field_name: String,
    ) -> Result<&dyn FunctionResolver, RuntimeError> {
        self.func.ok_or(RuntimeError::UndefinedFunctionError)
    }
}
