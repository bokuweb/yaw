# yaw

[![GitHub Actions Status](https://github.com/bokuweb/yaw/workflows/Continuous%20Integration/badge.svg)](https://github.com/bokuweb/yaw/actions)

## Installation

For now, please install from github.

```toml
[dependencies]
yaw = { git = "https://github.com/bokuweb/yaw.git" }
```

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

### More examples

- [const](https://github.com/bokuweb/yaw/blob/master/examples/const.rs)
- [add](https://github.com/bokuweb/yaw/blob/master/examples/add/main.rs)
- [yaw-boy(gameboy emulator written in go)](https://github.com/bokuweb/yaw/tree/master/examples/yaw-boy) 

## TODO

- [x] Run gameboy emulator
- [ ] Add validator
- [ ] Support WASI
- [ ] Run NES emulator
- [ ] Support no_std
- [ ] Support ARM core

## LICENSE

MIT
