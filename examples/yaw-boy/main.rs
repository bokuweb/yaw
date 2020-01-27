extern crate yaw;

use self::yaw::*;

fn main() -> Result<(), error::YawError> {
    let ins = instantiate(&include_bytes!("./gopher-boy/docs/main.wasm")[..], None)?;
    // let ret = ins.invoke("add", &[RuntimeValue::I32(1), RuntimeValue::I32(2)])?;
    // println!("1 + 2 = {:?}", ret);
    Ok(())
}
