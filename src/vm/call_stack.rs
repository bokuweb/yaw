use std::rc::Rc;

use super::super::types::*;
use super::{Instruction, LabelStack};

#[derive(Debug)]
pub struct StackFrame {
    pub locals: Vec<RuntimeValue>,
    pub lstack: LabelStack,
    pub instructions: Rc<Vec<Instruction>>,
    pub pc: usize,
}

impl StackFrame {
    pub fn new(
        locals: Vec<RuntimeValue>,
        lstack: LabelStack,
        instructions: Rc<Vec<Instruction>>,
        pc: usize,
    ) -> Self {
        Self {
            locals,
            lstack,
            instructions,
            pc,
        }
    }
}

#[derive(Debug)]
pub struct CallStack {
    buf: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack { buf: vec![] }
    }

    pub fn push(&mut self, v: StackFrame) {
        self.buf.push(v);
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.buf.pop()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}
