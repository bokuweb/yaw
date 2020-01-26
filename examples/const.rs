extern crate yaw;

use self::yaw::*;

use std::fs;
use std::io::Read;

fn main() -> Result<(), error::YawError> {
    better_panic::install();
    let mut file = fs::File::open("./fixtures/wasm/const.wasm")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let ins = instantiate(&buf, None)?;
    let ret = ins.invoke(&"c".to_owned(), &[]);
    println!("{:?}", ret);
    Ok(())
}
