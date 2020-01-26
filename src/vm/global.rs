use std::cell::RefCell;
use std::rc::Rc;

use super::runtime_value::*;
use super::value_type::*;

use super::{ImportResolver, ImportType, RuntimeError, Sections};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Global {
    pub mutability: bool,
    pub value: RuntimeValue,
    pub value_type: ValueType,
}

impl Global {
    pub fn new(mutability: bool, value: RuntimeValue, value_type: ValueType) -> Global {
        Global {
            mutability,
            value,
            value_type,
        }
    }
}

pub type GlobalRef = Rc<RefCell<Global>>;

#[derive(Debug, Clone)]
pub struct Globals(Vec<GlobalRef>);

impl Globals {
    pub fn from_section<'a>(
        sections: &Sections,
        imports: Option<&'a dyn ImportResolver>,
    ) -> Result<Self, RuntimeError> {
        let mut globals = vec![];
        if let Some(import_section) = sections.import_section.as_ref() {
            if let Some(imports) = &imports {
                for entry in &import_section.entries {
                    if let ImportType::Global(_) = entry.import_type {
                        let global = imports
                            .resolve_global(entry.module_name.clone(), entry.field_name.clone())?;
                        globals.push(global);
                    }
                }
            }
        }

        if let Some(global) = sections.global_section.as_ref() {
            for g in &global.globals {
                globals.push(Rc::new(RefCell::new(Global::new(
                    g.global_type.mutability,
                    g.initial_value,
                    g.global_type.value_type,
                ))));
            }
        };
        Ok(Self(globals))
    }

    pub fn get(&self, index: usize) -> Option<&GlobalRef> {
        self.0.get(index)
    }
}
