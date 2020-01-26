use std::collections::HashMap;

use crate::decoder::*;
use crate::RuntimeError;

#[derive(Debug, Clone)]
pub struct ExportType {
    pub kind: ExternalKind,
    pub index: u32,
}

impl ExportType {
    pub fn new(kind: ExternalKind, index: u32) -> ExportType {
        ExportType { kind, index }
    }
}

#[derive(Debug, Clone)]
pub struct Exports(HashMap<String, ExportType>);

impl Exports {
    pub fn from_section(export_section: Option<&ExportSection>) -> Self {
        let mut exports = HashMap::new();
        if let Some(export) = export_section {
            let entries = &export.entries;
            for entry in entries {
                exports.insert(
                    entry.name.to_owned(),
                    ExportType::new(entry.kind, entry.index),
                );
            }
        }
        Self(exports)
    }

    pub fn resolve(&self, name: &str) -> Result<u32, RuntimeError> {
        let e = self.0.get(name).ok_or(RuntimeError::UndefinedExportError)?;
        Ok(e.index)
    }

    pub fn into_inner(self) -> HashMap<String, ExportType> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<String, ExportType> {
        &self.0
    }
}
