extern crate yaw;

use self::yaw::*;

#[derive(Debug)]
struct Wasi32Functions {}

impl FunctionResolver for Wasi32Functions {
    fn invoke(
        &self,
        ins: &mut VM,
        _name: &str,
        field_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        match field_name {
            "fd_write" => {
                let mut ctx = WasiCtx {};
                let m = ins.resolve_memory()?;
                dbg!(&args);
                let fd: u32 = args[0].into();
                let ptr: u32 = args[1].into();
                let len: u32 = args[2].into();
                let written: u32 = args[3].into();
                unsafe {
                    fd_write(&mut ctx, &m, Fd::new(fd), ptr, len, written);
                }
                Ok(vec![RuntimeValue::I32(0)])
            }
            _ => Ok(vec![RuntimeValue::I32(0)]),
        }
    }
}

fn main() -> Result<(), error::YawError> {
    let wasi_fn = Wasi32Functions {};
    let mut imports = Imports::new();
    imports.add_function(&wasi_fn);
    let mut ins = instantiate(
        &include_bytes!("../../fixtures/wasm/wasi_hello.wasm")[..],
        Some(&imports),
    )?;
    let ret = ins.invoke("_start", &[RuntimeValue::I32(1), RuntimeValue::I32(2)])?;
    Ok(())
}
