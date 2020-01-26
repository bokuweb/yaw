extern crate yaw;

mod spectest;

use spectest::*;
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::rc::Rc;

use yaw::types::*;
use yaw::{
    FunctionResolver, Global, Imports, MemoryDescriptor, MemoryRef, RuntimeError, ValueType,
};

#[test]
fn const_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/const.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("c", &[])?;
    assert_eq!(vec![RuntimeValue::I32(42)], ret);
    Ok(())
}

#[test]
fn add_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/add.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("add", &[RuntimeValue::I32(3), RuntimeValue::I32(7)])?;
    assert_eq!(vec![RuntimeValue::I32(10)], ret);
    Ok(())
}

#[test]
fn sub_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/sub.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("sub", &[RuntimeValue::I32(3), RuntimeValue::I32(7)])?;
    assert_eq!(vec![RuntimeValue::I32(-4)], ret);
    Ok(())
}

#[test]
fn add_f64_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/add_f64.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("add_f64", &[RuntimeValue::F64(1.1), RuntimeValue::F64(2.2)])?;
    assert_eq!(vec![RuntimeValue::F64(3.300_000_000_000_000_3)], ret);
    Ok(())
}

#[test]
fn sum_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/sum.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("sum", &[RuntimeValue::I32(10)])?;
    assert_eq!(vec![RuntimeValue::I32(45)], ret);
    Ok(())
}

#[test]
fn call_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/call.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("call", &[])?;
    assert_eq!(vec![RuntimeValue::I32(42)], ret);
    Ok(())
}

#[test]
fn call_loop_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/call_loop.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("call_loop", &[RuntimeValue::I32(10)])?;
    assert_eq!(vec![RuntimeValue::I32(45)], ret);
    Ok(())
}

#[test]
fn if_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/if.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("if", &[])?;
    assert_eq!(vec![RuntimeValue::I32(10)], ret);
    Ok(())
}

#[test]
fn if_else_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/if-else.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("if_else", &[RuntimeValue::I32(0)])?;
    assert_eq!(vec![RuntimeValue::I32(20)], ret);
    let ret = ins.invoke("if_else", &[RuntimeValue::I32(1)])?;
    assert_eq!(vec![RuntimeValue::I32(10)], ret);
    Ok(())
}

#[test]
fn fib_wasm() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/fib.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("fib", &[RuntimeValue::I32(10)])?;
    assert_eq!(vec![RuntimeValue::I32(89)], ret);
    Ok(())
}

#[test]
fn i32_load() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/load.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let mem = MemoryRef::new(MemoryDescriptor::new(1, None));
    mem.i32_store(0, 0x1234_5678)?;
    let mut imports = Imports::new();
    imports.add_memory("env", "memory", mem.clone());
    let ins = yaw::instantiate(&buf, Some(&imports))?;
    let ret = ins.invoke("load", &[])?;
    assert_eq!(vec![RuntimeValue::I32(0x1234_5678)], ret);
    mem.i32_store(0, 0x5A5A_A5A5)?;
    let ret = ins.invoke("load", &[])?;
    assert_eq!(vec![RuntimeValue::I32(0x5A5A_A5A5)], ret);
    Ok(())
}

#[test]
fn i32_store() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/store.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let mem = MemoryRef::new(MemoryDescriptor::new(1, None));
    let mut imports = Imports::new();
    imports.add_memory("env", "memory", mem);
    let ins = yaw::instantiate(&buf, Some(&imports))?;
    let ret = ins.invoke("store", &[])?;
    assert_eq!(vec![RuntimeValue::I32(0x5A5A_A5A5)], ret);
    Ok(())
}

#[test]
fn memory() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/memory.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("memory", &[RuntimeValue::I32(10), RuntimeValue::I32(20)])?;
    assert_eq!(vec![RuntimeValue::I32(30)], ret);
    Ok(())
}

#[test]
fn global() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/global.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("global", &[RuntimeValue::I32(1)])?;
    assert_eq!(vec![RuntimeValue::F32(5.1)], ret);
    Ok(())
}

#[test]
fn i32_store_without_import() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/store_without_import.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("store_without_import", &[])?;
    assert_eq!(vec![RuntimeValue::I32(0x5A5A_A5A5)], ret);
    Ok(())
}

#[test]
fn grow() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/grow.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("grow", &[RuntimeValue::I32(3), RuntimeValue::I32(1)])?;
    assert_eq!(vec![RuntimeValue::I32(6)], ret);
    Ok(())
}

#[test]
fn fizzbuzz() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/fizzbuzz.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("fizzbuzz", &[RuntimeValue::I32(0)])?;
    assert_eq!(vec![RuntimeValue::I32(3)], ret);
    let ret = ins.invoke("fizzbuzz", &[RuntimeValue::I32(1)])?;
    assert_eq!(vec![RuntimeValue::I32(0)], ret);
    let ret = ins.invoke("fizzbuzz", &[RuntimeValue::I32(3)])?;
    assert_eq!(vec![RuntimeValue::I32(1)], ret);
    let ret = ins.invoke("fizzbuzz", &[RuntimeValue::I32(5)])?;
    assert_eq!(vec![RuntimeValue::I32(2)], ret);
    let ret = ins.invoke("fizzbuzz", &[RuntimeValue::I32(15)])?;
    assert_eq!(vec![RuntimeValue::I32(3)], ret);
    Ok(())
}

#[test]
fn sum_rs() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/sum-rs.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("sum", &[RuntimeValue::I32(10)])?;
    assert_eq!(vec![RuntimeValue::I32(10)], ret);
    Ok(())
}

#[test]
fn minimum_call_indirect() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/call_indirect.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let ins = yaw::instantiate(&buf, None)?;
    let ret = ins.invoke("callByIndex", &[RuntimeValue::I32(0)])?;
    assert_eq!(vec![RuntimeValue::I32(42)], ret);
    let ret = ins.invoke("callByIndex", &[RuntimeValue::I32(1)])?;
    assert_eq!(vec![RuntimeValue::I32(13)], ret);

    Ok(())
}

#[derive(Debug, Clone)]
struct ImportFuncTest;

impl FunctionResolver for ImportFuncTest {
    fn invoke(
        &self,
        _name: &str,
        _field_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        let v: usize = args[0].into();
        Ok(vec![RuntimeValue::I32((v * 2) as i32)])
    }
}

#[test]
fn import_func() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/import_func.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let r = ImportFuncTest {};
    let mut imports = Imports::new();
    imports.add_function(&r);
    let ins = yaw::instantiate(&buf, Some(&imports))?;
    let ret = ins.invoke("exported_func", &[RuntimeValue::I32(0)])?;
    assert_eq!(vec![RuntimeValue::I32(84)], ret);
    Ok(())
}

#[test]
fn import_global() -> Result<(), yaw::error::YawError> {
    let mut file = fs::File::open("./fixtures/wasm/global_import.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let mut imports = Imports::new();
    imports.add_global(
        "env",
        "global1",
        Rc::new(RefCell::new(Global::new(
            false,
            RuntimeValue::I32(42),
            ValueType::I32,
        ))),
    );
    let ins = yaw::instantiate(&buf, Some(&imports))?;
    let ret = ins.invoke("global", &[RuntimeValue::I32(0)])?;
    assert_eq!(vec![RuntimeValue::I32(42)], ret);
    Ok(())
}

#[test]
fn testsuite_i32() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/i32.wast")
}

#[test]
fn testsuite_i64() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/i64.wast")
}

#[test]
fn testsuite_local_get() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/local_get.wast")
}

#[test]
fn testsuite_local_set() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/local_set.wast")
}

#[test]
fn testsuite_local_tee() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/local_tee.wast")
}

#[test]
fn testsuite_loop() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/loop.wast")
}

#[test]
fn testsuite_nop() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/nop.wast")
}

#[test]
fn testsuite_block() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/block.wast")
}

#[test]
fn testsuite_const() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/const.wast")
}

#[test]
fn testsuite_conversions() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/conversions.wast")
}

#[test]
fn testsuite_custom() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/custom.wast")
}

#[test]
fn testsuite_data() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/data.wast")
}

#[test]
fn testsuite_elem() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/elem.wast")
}

#[test]
fn testsuite_imports() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/imports.wast")
}

#[test]
fn testsuite_inline_module() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/inline-module.wast")
}

#[test]
fn testsuite_br() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/br.wast")
}

#[test]
fn testsuite_if() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/if.wast")
}

#[test]
fn testsuite_br_if() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/br_if.wast")
}

#[test]
fn testsuite_store() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/store.wast")
}

#[test]
fn testsuite_f32() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f32.wast")
}

#[test]
fn testsuite_f64() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f64.wast")
}

#[test]
fn testsuite_f32_cmp() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f32_cmp.wast")
}

#[test]
fn testsuite_f32_bitwise() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f32_bitwise.wast")
}

#[test]
fn testsuite_f64_cmp() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f64_cmp.wast")
}

#[test]
fn testsuite_f64_bitwise() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/f64_bitwise.wast")
}

#[test]
fn testsuite_float_literals() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/float_literals.wast")
}

#[test]
fn testsuite_address() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/address.wast")
}

#[test]
fn testsuite_align() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/align.wast")
}

#[test]
fn testsuite_binary_leb128() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/binary-leb128.wast")
}

#[test]
fn testsuite_binary() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/binary.wast")
}

#[test]
fn testsuite_switch() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/switch.wast")
}

#[test]
fn testsuite_float_misc() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/float_misc.wast")
}

#[test]
fn testsuite_br_table() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/br_table.wast")
}

#[test]
fn testsuite_break_drop() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/break-drop.wast")
}

#[test]
fn testsuite_call_indirect() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/call_indirect.wast")
}

#[test]
fn testsuite_call_test() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/call.wast")
}

#[test]
fn testsuite_comments() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/comments.wast")
}

#[test]
fn testsuite_endianness() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/endianness.wast")
}

#[test]
fn testsuite_exports() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/exports.wast")
}

#[test]
fn testsuite_fac() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/fac.wast")
}

#[test]
fn testsuite_int_exprs() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/int_exprs.wast")
}

#[test]
fn testsuite_int_literals() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/int_literals.wast")
}

#[test]
fn testsuite_labels() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/labels.wast")
}

#[test]
fn testsuite_left_to_right() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/left-to-right.wast")
}

#[test]
fn testsuite_load() -> Result<(), yaw::error::YawError> {
    exec_testsuite("./testsuite/load.wast")
}
