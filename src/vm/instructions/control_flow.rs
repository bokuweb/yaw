use crate::vm::error::*;
use crate::vm::value_stack::ValueStack;

use crate::decoder::*;
use crate::types::*;
use crate::vm::instructions::*;
use crate::vm::label::*;

pub fn br_table(
    operands: &[Operand],
    instrs: &[Instruction],
    mut pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<usize, RuntimeError> {
    let index: usize = pop(vstack)?.into();
    let count: u32 = operands[0].into();
    let count = count as usize;
    let indexes = {
        let mut indexes = vec![];
        for i in 0..count {
            let index: usize = operands[i + 1].into();
            indexes.push(index);
        }
        indexes
    };
    let default_index: usize = operands[count + 1].into();
    let before = if let Some(i) = indexes.get(index) {
        i + 1
    } else {
        default_index + 1
    };
    let should_exit = lstack.len() < before as usize;
    let labels = lstack.take_before(before as u32);
    let label = labels.get(0).expect("should take label");
    match label.block_type {
        BlockType::Block | BlockType::If => {
            pc = skip_until_end(instrs, pc, before)?;
            if label.result_type == ResultType::Empty {
                while vstack.len() > label.sp {
                    vstack.pop();
                }
            } else {
                let first = vstack.pop().expect("should pop");
                while vstack.len() > label.sp {
                    vstack.pop();
                }
                vstack.push(first);
            }
        }
        BlockType::Loop => pc = do_loop(instrs, label, should_exit)?,
    }
    Ok(pc)
}

pub fn br(
    operands: &[Operand],
    instrs: &[Instruction],
    mut pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<usize, RuntimeError> {
    let depth: usize = operands[0].into();
    let before = depth + 1;
    let should_exit = lstack.len() < before;
    let labels = lstack.take_before(before as u32);
    let label = labels.get(0).expect("should take label");
    match label.block_type {
        BlockType::Block | BlockType::If => {
            pc = skip_until_end(instrs, pc, depth as usize + 1)?;
            if label.result_type == ResultType::Empty {
                while vstack.len() > label.sp {
                    vstack.pop();
                }
            } else {
                let first = vstack.pop().expect("should pop");
                while vstack.len() > label.sp {
                    vstack.pop();
                }
                vstack.push(first);
            }
        }
        BlockType::Loop => pc = do_loop(instrs, label, should_exit)?,
    }
    Ok(pc)
}

pub fn br_if(
    operands: &[Operand],
    instrs: &[Instruction],
    mut pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<usize, RuntimeError> {
    let depth: usize = operands[0].into();
    let opland = pop(vstack)?;
    if !opland.is_zero() {
        let before = depth + 1;
        let should_exit = lstack.len() < before as usize;
        let labels = lstack.take_before(before as u32);
        let label = labels.get(0).expect("should take label");
        match label.block_type {
            BlockType::Block | BlockType::If => {
                pc = skip_until_end(instrs, pc, depth as usize + 1)?;
                if label.result_type == ResultType::Empty {
                    while vstack.len() > label.sp {
                        vstack.pop();
                    }
                } else {
                    let first = vstack.pop().expect("should pop");
                    while vstack.len() > label.sp {
                        vstack.pop();
                    }
                    vstack.push(first);
                }
            }
            BlockType::Loop => pc = do_loop(instrs, label, should_exit)?,
        }
    }
    Ok(pc)
}

fn do_loop(
    instrs: &[Instruction],
    label: &Label,
    should_exit: bool,
) -> Result<usize, RuntimeError> {
    let pos = label.position;
    let mut pc = pos as usize;
    // Please see. br_if.wast's as-loop_last
    // I guess we need to skip to end in this case.
    //
    // (func (export "as-loop-last") (param i32)
    //   (loop (call $dummy) (br_if 1 (local.get 0)))
    // )
    if should_exit {
        pc = skip_until_end(instrs, pc, 0)?;
    }
    Ok(pc)
}

pub fn block(
    operands: &[Operand],
    pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<(), RuntimeError> {
    if let Operand::ResultType(rtype) = operands[0] {
        lstack.push(Label::new(pc - 1, BlockType::Block, rtype, vstack.len()));
    }
    Ok(())
}

pub fn r#loop(
    operands: &[Operand],
    pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<(), RuntimeError> {
    if let Operand::ResultType(rtype) = operands[0] {
        lstack.push(Label::new(pc - 1, BlockType::Loop, rtype, vstack.len()));
    }
    Ok(())
}

pub fn r#if(
    operands: &[Operand],
    instrs: &[Instruction],
    mut pc: usize,
    vstack: &mut ValueStack,
    lstack: &mut LabelStack,
) -> Result<usize, RuntimeError> {
    if let Operand::ResultType(rtype) = operands[0] {
        lstack.push(Label::new(pc - 1, BlockType::If, rtype, vstack.len()));
        if vstack.pop().expect("should pop").is_zero() {
            pc = skip_until_else_or_end(instrs, pc)?;
            if instrs[pc - 1].0 == Opcode::End {
                // POP If label when end.
                lstack.pop();
            }
        }
    }
    Ok(pc)
}

pub fn r#else(
    instrs: &[Instruction],
    mut pc: usize,
    lstack: &mut LabelStack,
) -> Result<usize, RuntimeError> {
    pc = skip_until_end(instrs, pc, 1)?;
    lstack.pop();
    Ok(pc)
}

fn skip_until_else_or_end(instr: &[Instruction], mut pc: usize) -> Result<usize, RuntimeError> {
    let mut block_depth = 1;
    loop {
        let (op, _) = instr[pc];
        pc += 1;
        match op {
            Opcode::End => block_depth -= 1,
            Opcode::If | Opcode::Block | Opcode::Loop => {
                block_depth += 1;
            }
            _ => (),
        };
        if block_depth == 0 || (op == Opcode::Else && block_depth == 1) {
            return Ok(pc);
        }
    }
}

fn skip_until_end(
    instr: &[Instruction],
    mut pc: usize,
    mut block_depth: usize,
) -> Result<usize, RuntimeError> {
    loop {
        let (op, _) = instr[pc];
        pc += 1;
        match op {
            Opcode::End => block_depth -= 1,
            Opcode::If | Opcode::Block | Opcode::Loop => block_depth += 1,
            _ => (),
        };
        if block_depth == 0 {
            return Ok(pc);
        }
    }
}
