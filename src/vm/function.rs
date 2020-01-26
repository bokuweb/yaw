use crate::decoder::*;
use crate::types::*;

use std::rc::Rc;

use super::{ImportType, Instruction, RuntimeError, Sections};

#[derive(Debug)]
pub enum FunctionInstance {
    InternalFunction(InternalFunction),
    ExternalFunction(ExternalFunction),
}

pub type FunctionInstanceRef = Rc<FunctionInstance>;

#[derive(Debug, Clone)]
pub struct InternalFunction {
    pub args: Vec<ValueType>,
    pub instructions: Rc<Vec<Instruction>>,
    pub locals: Vec<LocalEntry>,
    pub results: Vec<ResultType>,
}

#[derive(Debug)]
pub struct ExternalFunction {
    pub module_name: String,
    pub field_name: String,
    pub args: Vec<ValueType>,
    pub results: Vec<ResultType>,
}

#[derive(Debug)]
pub struct Functions(Vec<FunctionInstanceRef>);

impl Functions {
    pub fn from_section(sections: &Sections) -> Result<Self, RuntimeError> {
        let mut functions = vec![];
        if let Some(import_section) = sections.import_section.as_ref() {
            for entry in &import_section.entries {
                if let ImportType::Function(type_index) = entry.import_type {
                    let type_section = sections
                        .type_section
                        .as_ref()
                        .expect("should has type section");
                    let args = type_section.entries[type_index as usize].args.clone();
                    let results = type_section.entries[type_index as usize].results.clone();
                    functions.push(Rc::new(FunctionInstance::ExternalFunction(
                        ExternalFunction {
                            module_name: entry.module_name.to_owned(),
                            field_name: entry.field_name.to_owned(),
                            args,
                            results,
                        },
                    )));
                }
            }
        }

        if let Some(code) = sections.code_section.as_ref() {
            let types = {
                if let Some(func_section) = sections.function_section.as_ref() {
                    func_section.types.clone()
                } else {
                    vec![]
                }
            };
            let type_section = sections
                .type_section
                .as_ref()
                .expect("should has type section");
            for (i, b) in code.bodies.iter().enumerate() {
                let instructions = Rc::new(b.decoded.clone());
                let locals = b.locals.clone();
                // Find type index from func section types
                // Then Find arg signature from type section entries.
                let type_index = types[i];
                let args = type_section.entries[type_index as usize].args.clone();
                let results = type_section.entries[type_index as usize].results.clone();
                functions.push(Rc::new(FunctionInstance::InternalFunction(
                    InternalFunction {
                        instructions,
                        locals,
                        args,
                        results,
                    },
                )));
            }
        }

        Ok(Self(functions))
    }

    pub fn get_ref(&self, index: usize) -> Result<FunctionInstanceRef, RuntimeError> {
        let t = self
            .0
            .get(index)
            .ok_or(RuntimeError::UndefinedFunctionError)?;
        Ok(Rc::clone(t))
    }

    pub fn into_inner(self) -> Vec<FunctionInstanceRef> {
        self.0
    }
}
