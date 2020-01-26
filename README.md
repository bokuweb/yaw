# yaw

[![GitHub Actions Status](https://github.com/bokuweb/yaw/workflows/Continuous%20Integration/badge.svg)](https://github.com/bokuweb/yaw/actions)

## Example

```rust
use yaw::*;

fn main() -> Result<(), error::YawError> {
    let ins = instantiate(&include_bytes!("./add.wasm")[..], None)?;
    let ret = ins.invoke("add", &[RuntimeValue::I32(1), RuntimeValue::I32(2)])?;
    println!("1 + 2 = {:?}", ret);
    Ok(())
}

```
