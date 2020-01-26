use crate::types::*;

use super::FunctionInstanceRef;
use super::{Functions, Globals, ImportResolver, ImportType, RuntimeError, Sections};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TableInstance {
    pub entries: Vec<Option<FunctionInstanceRef>>,
    pub limits: ResizableLimits,
}

impl TableInstance {
    pub fn new(initial: u32, max: Option<u32>) -> Self {
        Self {
            entries: vec![None; initial as usize],
            limits: ResizableLimits::new(initial, max),
        }
    }

    pub fn get(&self, index: usize) -> FunctionInstanceRef {
        self.entries[index].as_ref().unwrap().clone()
    }

    pub fn set(&mut self, index: usize, value: Option<FunctionInstanceRef>) {
        self.entries[index] = value;
    }
}

pub type TableRef = Rc<RefCell<TableInstance>>;

#[derive(Debug, Clone)]
pub struct Tables(Vec<TableRef>);

impl Tables {
    pub fn from_section<'a>(
        sections: &Sections,
        globals: &Globals,
        functions: &Functions,
        imports: Option<&'a dyn ImportResolver>,
    ) -> Result<Self, RuntimeError> {
        let mut tables = vec![];

        let mut table = Rc::new(RefCell::new(TableInstance::new(0, None)));
        let mut table_entries: Vec<Option<u32>> = vec![];
        if let Some(import_section) = sections.import_section.as_ref() {
            if let Some(imports) = &imports {
                for entry in &import_section.entries {
                    if let ImportType::Table(_) = entry.import_type {
                        table = imports
                            .resolve_table(entry.module_name.clone(), entry.field_name.clone())?;
                        table_entries = vec![None; table.borrow().limits.initial as usize];
                        if let Some(elem_section) = sections.element_section.as_ref() {
                            for i in 0..elem_section.entries.len() {
                                let offset: usize =
                                    elem_section.entries[i].offset.eval(&globals)?.into();

                                for index in 0..elem_section.entries[i].elems.len() {
                                    table_entries[offset + index as usize] =
                                        Some(elem_section.entries[i].elems[index]);
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(t) = sections.table_section.as_ref() {
            let t = &t.entries[0];
            let len = t.limits.initial;
            let max = t.limits.maximum;
            if let Some(elem_section) = sections.element_section.as_ref() {
                table = Rc::new(RefCell::new(TableInstance::new(len, max)));
                table_entries = vec![None; len as usize];
                for i in 0..elem_section.entries.len() {
                    let offset: usize = elem_section.entries[i].offset.eval(&globals)?.into();
                    for index in 0..elem_section.entries[i].elems.len() {
                        table_entries[offset + index as usize] =
                            Some(elem_section.entries[i].elems[index]);
                    }
                }
            }
        };

        for (i, item) in table_entries.iter().enumerate() {
            if let Some(fn_index) = item {
                table.borrow_mut().entries[i] = Some(functions.get_ref(*fn_index as usize)?);
            }
        }

        tables.push(table);
        Ok(Self(tables))
    }

    pub fn get_ref(&self, index: usize) -> Result<TableRef, RuntimeError> {
        let t = self.0.get(index).ok_or(RuntimeError::UndefinedTableError)?;
        Ok(Rc::clone(t))
    }
}
