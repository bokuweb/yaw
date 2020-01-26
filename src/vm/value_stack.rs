use super::super::types::*;

#[derive(Debug, Default)]
pub struct ValueStack {
    buf: Vec<RuntimeValue>,
    sp: usize,
}

impl ValueStack {
    pub fn new() -> ValueStack {
        ValueStack { sp: 0, buf: vec![] }
    }

    pub fn push(&mut self, v: RuntimeValue) {
        self.buf.push(v);
    }

    pub fn pop(&mut self) -> Option<RuntimeValue> {
        self.buf.pop()
    }

    pub fn take_buf(&mut self) -> Vec<RuntimeValue> {
        ::std::mem::replace(&mut self.buf, vec![])
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.len() == 0
    }
}
