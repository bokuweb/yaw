use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    U32(u32),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    ResultType(ResultType),
}

impl From<Operand> for RuntimeValue {
    fn from(v: Operand) -> RuntimeValue {
        match v {
            Operand::I32(v) => RuntimeValue::I32(v),
            Operand::I64(v) => RuntimeValue::I64(v),
            Operand::F32(v) => RuntimeValue::F32(v),
            Operand::F64(v) => RuntimeValue::F64(v),
            _ => panic!("should convert operand to runtime value"),
        }
    }
}

impl From<Operand> for usize {
    fn from(v: Operand) -> usize {
        match v {
            Operand::U32(v) => v as usize,
            Operand::I32(v) => v as usize,
            Operand::I64(v) => v as usize,
            Operand::F32(v) => v as usize,
            Operand::F64(v) => v as usize,
            _ => panic!("should convert operand to runtime value"),
        }
    }
}

impl From<Operand> for u32 {
    fn from(v: Operand) -> u32 {
        match v {
            Operand::U32(v) => v as u32,
            Operand::I32(v) => v as u32,
            Operand::I64(v) => v as u32,
            Operand::F32(v) => v as u32,
            Operand::F64(v) => v as u32,
            _ => panic!("should convert operand to runtime value"),
        }
    }
}
