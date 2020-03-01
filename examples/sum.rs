extern crate yaw;

use self::yaw::*;

fn main() -> Result<(), error::YawError> {
    let mut ins = instantiate(&include_bytes!("../fixtures/wasm/sum.wasm")[..], None)?;
    let ret = ins.invoke(&"sum".to_owned(), &[RuntimeValue::I32(10)]);
    println!("{:?}", ret);
    Ok(())
}
