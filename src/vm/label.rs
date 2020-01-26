use num_derive::*;

use crate::types::*;

#[derive(Debug, Copy, Clone)]
pub struct Label {
    pub position: usize,
    pub block_type: BlockType,
    pub result_type: ResultType,
    pub sp: usize,
}

impl Label {
    pub fn new(
        position: usize,
        block_type: BlockType,
        result_type: ResultType,
        sp: usize,
    ) -> Label {
        Label {
            position,
            block_type,
            result_type,
            sp,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BlockType {
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
}

#[derive(Debug)]
pub struct LabelStack(Vec<Label>);

impl LabelStack {
    pub fn new() -> LabelStack {
        LabelStack(vec![])
    }

    pub fn push(&mut self, v: Label) {
        self.0.push(v);
    }

    pub fn pop(&mut self) -> Option<Label> {
        self.0.pop()
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn take_before(&mut self, depth: u32) -> Vec<Label> {
        let depth = if depth as usize > self.0.len() {
            self.0.len()
        } else {
            depth as usize
        };
        self.0.split_off(self.0.len() - depth as usize)
    }
}
