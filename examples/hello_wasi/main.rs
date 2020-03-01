extern crate yaw;

use self::yaw::*;

#[derive(Debug)]
struct Wasi32Functions {}

impl FunctionResolver for Wasi32Functions {
    fn invoke(
        &self,
        _vm: &mut VM,
        _name: &str,
        field_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        match field_name {
            "fd_write" => {
                let mut ctx = WasiCtx {};
                let m = MemoryRef::new(MemoryDescriptor::new(1, None));
                unsafe {
                    fd_write(&mut ctx, &m, Fd::new(1), 0, 0, 0);
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
