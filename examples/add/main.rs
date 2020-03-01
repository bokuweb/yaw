extern crate yaw;

use self::yaw::*;

fn main() -> Result<(), error::YawError> {
    let mut ins = instantiate(&include_bytes!("../../fixtures/wasm/add.wasm")[..], None)?;
    let ret = ins.invoke("add", &[RuntimeValue::I32(1), RuntimeValue::I32(2)])?;
    println!("1 + 2 = {:?}", ret);
    Ok(())
}
