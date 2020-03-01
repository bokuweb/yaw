extern crate yaw;

use self::yaw::*;

fn main() -> Result<(), error::YawError> {
    let mut ins = instantiate(&include_bytes!("../fixtures/wasm/const.wasm")[..], None)?;
    let ret = ins.invoke(&"c".to_owned(), &[]);
    println!("{:?}", ret);
    Ok(())
}
