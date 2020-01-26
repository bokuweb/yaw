pub mod error;
pub mod exports;
pub mod function;
pub mod global;
pub mod imports;
pub mod memory;
pub mod table;

pub(crate) mod call_stack;
pub(crate) mod instructions;
pub(crate) mod label;
pub(crate) mod value_stack;

use crate::decoder::*;
use crate::error::*;
use crate::types::*;

pub(crate) use instructions::*;

pub use error::*;
pub use exports::*;
pub use function::*;
pub use global::*;
pub use imports::*;
pub use memory::*;
pub use table::*;
pub use value_stack::*;

use std::rc::Rc;

use call_stack::{CallStack, StackFrame};
use label::*;

enum Next {
    None,
    Continue,
}

#[derive(Debug)]
pub struct VM<'a> {
    exports: Exports,
    functions: Functions,
    globals: Globals,
    table: TableRef,
    memories: Memories,
    func_types: Vec<FuncType>,
    func_resolver: Option<&'a dyn FunctionResolver>,
}

impl<'a> VM<'a> {
    pub fn from_section(
        sections: Sections,
        imports: Option<&'a dyn ImportResolver>,
    ) -> Result<Self, RuntimeError> {
        let func_resolver = VM::extract_func_resolver(sections.import_section.as_ref(), imports)?;
        let exports = Exports::from_section(sections.export_section.as_ref());
        let globals = Globals::from_section(&sections, imports)?;
        let functions = Functions::from_section(&sections)?;
        let memories = Memories::from_section(&sections, &globals, imports)?;
        let tables = Tables::from_section(&sections, &globals, &functions, imports)?;
        // Now only one table is supported.
        let table = tables.get_ref(0)?;
        let func_types = sections.type_section.unwrap_or_default().entries;
        Ok(Self {
            exports,
            globals,
            table,
            functions,
            memories,
            func_types,
            func_resolver,
        })
    }

    pub fn invoke(&self, name: &str, args: &[RuntimeValue]) -> Result<Vec<RuntimeValue>, YawError> {
        let index = self.exports.resolve(name)?;
        let func = self.functions.get_ref(index as usize)?;
        match &*func {
            FunctionInstance::InternalFunction(func) => self.invoke_internal(func, args),
            FunctionInstance::ExternalFunction(_) => unreachable!(),
        }
    }

    pub fn exports(&self) -> &Exports {
        &self.exports
    }

    pub fn resolve_table(&self) -> TableRef {
        self.table.clone()
    }

    pub fn resolve_global(&self, index: usize) -> Result<GlobalRef, RuntimeError> {
        let global = self
            .globals
            .get(index)
            .ok_or(RuntimeError::UndefinedGlobalError)?;
        Ok(Rc::clone(global))
    }

    pub fn resolve_function(&self, index: usize) -> Result<FunctionInstanceRef, RuntimeError> {
        self.functions.get_ref(index)
    }

    pub fn resolve_memory(&self) -> Result<MemoryRef, RuntimeError> {
        self.memories.get(0)
    }

    fn invoke_internal(
        &self,
        func: &InternalFunction,
        args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, YawError> {
        let mut locals = vec![];
        locals.extend(args);
        for local in &func.locals {
            for _ in 0..local.count {
                locals.push(local.value_type.into());
            }
        }
        let mut cstack = CallStack::new();
        let mut vstack = ValueStack::new();
        let lstack = LabelStack::new();
        let current_frame = StackFrame::new(locals, lstack, Rc::clone(&func.instructions), 0);
        cstack.push(current_frame);
        loop {
            match self.execute_function(&mut cstack, &mut vstack)? {
                Next::None => break,
                Next::Continue => {}
            }
        }
        let res = vstack.take_buf();
        Ok(res)
    }

    fn extract_func_resolver(
        import_section: Option<&ImportSection>,
        imports: Option<&'a dyn ImportResolver>,
    ) -> Result<Option<&'a dyn FunctionResolver>, RuntimeError> {
        let mut func_resolver: Option<&'a dyn FunctionResolver> = None;

        if let Some(import_section) = import_section {
            if let Some(imports) = &imports {
                for entry in &import_section.entries {
                    if let ImportType::Function(_) = entry.import_type {
                        func_resolver = Some(imports.resolve_function(/*entry.module_name */)?)
                    }
                }
            }
        }
        Ok(func_resolver)
    }

    fn execute_function(
        &self,
        cstack: &mut CallStack,
        vstack: &mut ValueStack,
    ) -> Result<Next, YawError> {
        let StackFrame {
            instructions,
            mut locals,
            mut lstack,
            mut pc,
        } = cstack.pop().ok_or(RuntimeError::StackPopError)?;
        loop {
            if instructions.len() == pc {
                if cstack.len() > 0 {
                    return Ok(Next::Continue);
                }
                return Ok(Next::None);
            }
            let inst = &instructions[pc];
            pc += 1;
            match inst.0 {
                Opcode::Unreachable => panic!("unreachable"),
                Opcode::Select => select(vstack)?,
                Opcode::Drop => drop(vstack)?,
                Opcode::Call => {
                    let index: usize = inst.1[0].into();
                    let func = self.functions.get_ref(index)?;
                    match &*func {
                        FunctionInstance::InternalFunction(func) => {
                            let instrs = Rc::clone(&instructions);
                            let frame = StackFrame::new(locals, lstack, instrs, pc);
                            // Save current context
                            cstack.push(frame);
                            cstack.push(self.create_new_frame(func, vstack)?);
                            return Ok(Next::Continue);
                        }
                        FunctionInstance::ExternalFunction(func) => {
                            self.execute_external_function(func, vstack)?
                        }
                    }
                }
                Opcode::CallIndirect => {
                    let type_index: usize = inst.1[0].into();
                    // Reserved
                    let entry_index: usize = pop(vstack)?.into();
                    if let Some(fn_ref) = self.table.borrow().entries.get(entry_index) {
                        if let Some(fn_ref) = fn_ref {
                            let func = Rc::clone(fn_ref);
                            match &*func {
                                FunctionInstance::InternalFunction(func) => {
                                    self.validate_call_indirect(func, type_index)?;
                                    let instrs = Rc::clone(&instructions);
                                    let frame = StackFrame::new(locals, lstack, instrs, pc);
                                    // Save current context
                                    cstack.push(frame);
                                    cstack.push(self.create_new_frame(func, vstack)?);
                                    return Ok(Next::Continue);
                                }
                                FunctionInstance::ExternalFunction(func) => {
                                    self.execute_external_function(func, vstack)?
                                }
                            }
                        } else {
                            return Err(RuntimeError::UnInitializedElementError.into());
                        }
                    } else {
                        return Err(RuntimeError::UndefinedElementError.into());
                    }
                }
                Opcode::If => pc = r#if(&inst.1, &instructions, pc, vstack, &mut lstack)?,
                Opcode::Else => pc = r#else(&instructions, pc, &mut lstack)?,
                Opcode::Nop => {}
                Opcode::Loop => r#loop(&inst.1, pc, vstack, &mut lstack)?,
                Opcode::Block => block(&inst.1, pc, vstack, &mut lstack)?,
                Opcode::BrIf => pc = br_if(&inst.1, &instructions, pc, vstack, &mut lstack)?,
                Opcode::Br => pc = br(&inst.1, &instructions, pc, vstack, &mut lstack)?,
                Opcode::BrTable => pc = br_table(&inst.1, &instructions, pc, vstack, &mut lstack)?,
                Opcode::Return => {
                    if cstack.len() == 0 {
                        return Ok(Next::None);
                    }
                    return Ok(Next::Continue);
                }
                Opcode::End => {
                    lstack.take_before(1);
                }
                // Variable access
                Opcode::GetLocal => get_local(&inst.1, vstack, &locals)?,
                Opcode::SetLocal => set_local(&inst.1, vstack, &mut locals)?,
                Opcode::TeeLocal => tee_local(&inst.1, vstack, &mut locals)?,
                Opcode::GetGlobal => get_global(&inst.1, vstack, &self.globals)?,
                Opcode::SetGlobal => set_global(&inst.1, vstack, &self.globals)?,
                // Memory related operations
                Opcode::I32Load => i32_load(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load => i64_load(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::F32Load => f32_load(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::F64Load => f64_load(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Load8S => i32_load8_s(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Load8U => i32_load8_u(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Load16S => i32_load16_s(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Load16U => i32_load16_u(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load8S => i64_load8_s(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load8U => i64_load8_u(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load16S => i64_load16_s(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load16U => i64_load16_u(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load32S => i64_load32_s(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Load32U => i64_load32_u(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Store => i32_store(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Store => i64_store(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::F32Store => f32_store(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::F64Store => f64_store(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Store8 => i32_store8(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I32Store16 => i32_store16(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Store8 => i64_store8(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Store16 => i64_store16(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::I64Store32 => i64_store32(&inst.1, vstack, &self.resolve_memory()?)?,
                Opcode::CurrentMemory => current(vstack, &self.resolve_memory()?)?,
                Opcode::GrowMemory => grow(vstack, &self.resolve_memory()?)?,
                // Constants
                Opcode::I32Const => i32_const(&inst.1, vstack)?,
                Opcode::I64Const => i64_const(&inst.1, vstack)?,
                Opcode::F32Const => f32_const(&inst.1, vstack)?,
                Opcode::F64Const => f64_const(&inst.1, vstack)?,
                // Comparison operations
                Opcode::I32Eqz | Opcode::I64Eqz => eqz(vstack)?,
                Opcode::I32Eq | Opcode::I64Eq | Opcode::F32Eq | Opcode::F64Eq => eq(vstack)?,
                Opcode::I32Ne | Opcode::I64Ne | Opcode::F32Ne | Opcode::F64Ne => ne(vstack)?,
                Opcode::I32LtS | Opcode::I64LtS | Opcode::F32Lt | Opcode::F64Lt => lt_s(vstack)?,
                Opcode::I32LtU | Opcode::I64LtU => lt_u(vstack)?,
                Opcode::I32GtS | Opcode::I64GtS | Opcode::F32Gt | Opcode::F64Gt => gt_s(vstack)?,
                Opcode::I32GtU | Opcode::I64GtU => gt_u(vstack)?,
                Opcode::I32LeS | Opcode::I64LeS | Opcode::F32Le | Opcode::F64Le => le_s(vstack)?,
                Opcode::I32LeU | Opcode::I64LeU => le_u(vstack)?,
                Opcode::I32GeU | Opcode::I64GeU => ge_u(vstack)?,
                Opcode::I32GeS | Opcode::I64GeS | Opcode::F32Ge | Opcode::F64Ge => ge_s(vstack)?,
                // Numeric operations
                Opcode::I32Clz | Opcode::I64Clz => clz(vstack)?,
                Opcode::I32Ctz | Opcode::I64Ctz => ctz(vstack)?,
                Opcode::I32Popcnt | Opcode::I64Popcnt => popcnt(vstack)?,
                Opcode::I32Add | Opcode::I64Add | Opcode::F32Add | Opcode::F64Add => add(vstack)?,
                Opcode::I32Sub | Opcode::I64Sub | Opcode::F32Sub | Opcode::F64Sub => sub(vstack)?,
                Opcode::I32Mul | Opcode::I64Mul | Opcode::F32Mul | Opcode::F64Mul => mul(vstack)?,
                Opcode::I32DivS | Opcode::I64DivS => div_s(vstack)?,
                Opcode::I32DivU | Opcode::I64DivU => div_u(vstack)?,
                Opcode::I32RemS | Opcode::I64RemS => rem_s(vstack)?,
                Opcode::I32RemU | Opcode::I64RemU => rem_u(vstack)?,
                Opcode::I32And | Opcode::I64And => and(vstack)?,
                Opcode::I32Or | Opcode::I64Or => or(vstack)?,
                Opcode::I32Xor | Opcode::I64Xor => xor(vstack)?,
                Opcode::I32Shl | Opcode::I64Shl => shl(vstack)?,
                Opcode::I32ShrS | Opcode::I64ShrS => shr_s(vstack)?,
                Opcode::I32ShrU | Opcode::I64ShrU => shr_u(vstack)?,
                Opcode::I32Rotl | Opcode::I64Rotl => rotl(vstack)?,
                Opcode::I32Rotr | Opcode::I64Rotr => rotr(vstack)?,
                Opcode::F32Abs | Opcode::F64Abs => abs(vstack)?,
                Opcode::F32Neg | Opcode::F64Neg => neg(vstack)?,
                Opcode::F32Ceil | Opcode::F64Ceil => ceil(vstack)?,
                Opcode::F32Floor | Opcode::F64Floor => floor(vstack)?,
                Opcode::F32Trunc | Opcode::F64Trunc => trunc(vstack)?,
                Opcode::F32Nearest | Opcode::F64Nearest => nearest(vstack)?,
                Opcode::F32Sqrt | Opcode::F64Sqrt => sqrt(vstack)?,
                Opcode::F32Div | Opcode::F64Div => div(vstack)?,
                Opcode::F32Min | Opcode::F64Min => min(vstack)?,
                Opcode::F32Max | Opcode::F64Max => max(vstack)?,
                Opcode::F32Copysign | Opcode::F64Copysign => copysign(vstack)?,
                // Conversions
                Opcode::I32WrapI64 => wrap(vstack)?,
                Opcode::I32TruncSF32 | Opcode::I32TruncSF64 => trunc_s_toi32(vstack)?,
                Opcode::I64TruncSF32 | Opcode::I64TruncSF64 => trunc_s_toi64(vstack)?,
                Opcode::I32TruncUF32 | Opcode::I32TruncUF64 => trunc_u_toi32(vstack)?,
                Opcode::I64TruncUF32 | Opcode::I64TruncUF64 => trunc_u_toi64(vstack)?,
                Opcode::I64ExtendSI32 => extend_s(vstack)?,
                Opcode::I64ExtendUI32 => extend_u(vstack)?,
                Opcode::F32ConvertSI32 | Opcode::F32ConvertSI64 => convert_s_tof32(vstack)?,
                Opcode::F32ConvertUI32 | Opcode::F32ConvertUI64 => convert_u_tof32(vstack)?,
                Opcode::F64ConvertSI32 | Opcode::F64ConvertSI64 => convert_s_tof64(vstack)?,
                Opcode::F64ConvertUI32 | Opcode::F64ConvertUI64 => convert_u_tof64(vstack)?,
                Opcode::F32DemoteF64 => demote(vstack)?,
                Opcode::F64PromoteF32 => promote(vstack)?,
                // Reinterpretations
                Opcode::F32ReinterpretI32
                | Opcode::F64ReinterpretI64
                | Opcode::I32ReinterpretF32
                | Opcode::I64ReinterpretF64 => reinterpret(vstack)?,
            };
        }
    }

    fn execute_external_function(
        &self,
        func: &ExternalFunction,
        vstack: &mut ValueStack,
    ) -> Result<(), RuntimeError> {
        if let Some(resolver) = &self.func_resolver {
            let mut args: Vec<RuntimeValue> = vec![];
            for _ in 0..func.args.len() {
                args.push(pop(vstack)?);
            }
            let result = resolver.invoke(&func.module_name, &func.field_name, &args)?;
            for r in result {
                vstack.push(r);
            }
        }
        Ok(())
    }

    fn validate_call_indirect(
        &self,
        func: &InternalFunction,
        type_index: usize,
    ) -> Result<(), RuntimeError> {
        if func.args.len() != self.func_types[type_index].args.len() {
            return Err(RuntimeError::IndirectCallTypeMismatchError);
        }
        for i in 0..func.args.len() {
            if func.args[i] != self.func_types[type_index].args[i] {
                return Err(RuntimeError::IndirectCallTypeMismatchError);
            }
        }

        if func.results.len() != self.func_types[type_index].results.len() {
            return Err(RuntimeError::IndirectCallTypeMismatchError);
        }
        for i in 0..func.results.len() {
            if func.results[i] != self.func_types[type_index].results[i] {
                return Err(RuntimeError::IndirectCallTypeMismatchError);
            }
        }
        Ok(())
    }

    fn create_new_frame(
        &self,
        func: &InternalFunction,
        vstack: &mut ValueStack,
    ) -> Result<StackFrame, RuntimeError> {
        let mut locals = vec![];
        for _ in &func.args {
            locals.push(vstack.pop().expect("should pop"));
        }
        locals.reverse();
        for local in &func.locals {
            for _ in 0..local.count {
                locals.push(local.value_type.into());
            }
        }
        let lstack = LabelStack::new();
        let frame = StackFrame::new(locals, lstack, Rc::clone(&func.instructions), 0);
        Ok(frame)
    }
}
