extern crate yaw;

use self::yaw::*;

use std::fs;
use std::io::Read;

fn main() -> Result<(), error::YawError> {
    better_panic::install();
    let mut file = fs::File::open("./fixtures/wasm/sum.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let ins = instantiate(&buf, None)?;
    let ret = ins.invoke(&"sum".to_owned(), &[RuntimeValue::I32(10)]);
    println!("{:?}", ret);
    Ok(())
}
