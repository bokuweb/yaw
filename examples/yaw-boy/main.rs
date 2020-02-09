extern crate yaw;

use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use self::yaw::*;

#[derive(Debug, Clone, Copy)]
enum BridgeValue {
  Undefined,
  NaN,
  Zero,
  Null,
  True,
  False,
  Global,
  This,
  Number(f64),
}

fn main() -> Result<(), error::YawError> {
  let go = Go::new();
  let mut imports = Imports::new();
  imports.add_function(&go);
  let ins = instantiate(
    &include_bytes!("./gopher-boy/docs/main.wasm")[..],
    Some(&imports),
  )?;
  go.run(ins);
  // let ret = ins.invoke("add", &[RuntimeValue::I32(1), RuntimeValue::I32(2)])?;
  // println!("1 + 2 = {:?}", ret);
  Ok(())
}

#[derive(Debug)]
struct Go<'a> {
  inst: Rc<RefCell<Option<VM<'a>>>>,
  // mem: Rc<RefCell<Option<MemoryRef>>>,
  argv: Vec<String>,
  exited: bool,
  values: Rc<RefCell<Vec<BridgeValue>>>,
  exports: Option<Rc<Exports>>,
}

impl<'a> FunctionResolver for Go<'a> {
  fn invoke(
    &self,
    _name: &str,
    field_name: &str,
    args: &[RuntimeValue],
  ) -> Result<Vec<RuntimeValue>, RuntimeError> {
    // if name == "spectest" {
    dbg!(field_name);
    return match field_name {
      "syscall/js.valueGet" => {
        dbg!(field_name, &args);
        let inst = self.inst.borrow();
        let inst = inst.as_ref().unwrap();
        let sp: u32 = args[0].into();
        let v = self.load_value(sp + 8, &self.values.borrow())?;
        let name: &str = &self.load_string(sp + 16).unwrap();
        // const result = Reflect.get(loadValue(sp + 8), loadString(sp + 16));
        // sp = this._inst.exports.getsp(); // see comment above
        // storeValue(sp + 32, result);
        let sp: u32 = inst.invoke("getsp", &[]).unwrap()[0].into();
        dbg!(v, &name, sp);
        match name {
          "O_WRONLY" | "O_RDWR" | "O_CREAT" | "O_TRUNC" | "O_APPEND" | "O_EXCL" => {
            self.store_value(sp + 32, BridgeValue::Number(-1.0))?;
            return Ok(vec![]);
          }
          _ => {}
        }
        self.store_value(sp + 32, v)?;
        Ok(vec![])
      }
      "syscall/js.valueCall" => {
        dbg!(field_name, &args);
        Ok(vec![])
      }
      _ => Ok(vec![]),
      //         "print_i32" => Ok(vec![]),
      //         "print_i32_f32" => Ok(vec![]),
      //         "print_f64_f64" => Ok(vec![]),
      //         "print_f32" => Ok(vec![]),
      //         "print_f64" => Ok(vec![]),
      //         _ => Ok(vec![]),
    };
    // }
    // let inner = self.inner.borrow();
    // let m = inner
    //     .modules
    //     .get(&Some(name.to_owned()))
    //     .expect("should get module");
    // let res = m.borrow().invoke(field_name, args).unwrap();
    Ok(vec![RuntimeValue::I32(0)])
  }
}

impl<'a> Go<'a> {
  fn new() -> Self {
    /*
    this.argv = ["js"];
    this.env = {};
    this.exit = code => {
      if (code !== 0) {
        console.warn("exit code:", code);
      }
    };
    this._exitPromise = new Promise(resolve => {
      this._resolveExitPromise = resolve;
    });
    this._pendingEvent = null;
    this._scheduledTimeouts = new Map();
    this._nextCallbackTimeoutID = 1;
    */
    Self {
      inst: Rc::new(RefCell::new(None)),
      // mem: Rc::new(RefCell::new(None)),
      argv: vec!["js".to_owned()],
      exited: false,
      exports: None,
      values: Rc::new(RefCell::new(vec![
        BridgeValue::NaN,
        BridgeValue::Zero,
        BridgeValue::Null,
        BridgeValue::True,
        BridgeValue::False,
        BridgeValue::Global,
        BridgeValue::This,
      ])),
    }
  }

  fn get_memory_ref(&self) -> MemoryRef {
    let i = self.inst.borrow();
    i.as_ref().unwrap().resolve_memory().unwrap()
  }

  fn load_string(&self, addr: u32) -> Result<String, RuntimeError> {
    let mem = self.get_memory_ref();
    let start = mem.i64_load(addr)? as usize;
    let len = mem.i64_load(addr + 8)? as usize;
    let s = mem.to_string(start, len)?;
    Ok(s)
  }

  fn load_value(&self, addr: u32, values: &[BridgeValue]) -> Result<BridgeValue, RuntimeError> {
    let mem = self.get_memory_ref();
    let f = mem.f64_load(addr)? as f64;
    if f == 0.0 {
      return Ok(BridgeValue::Undefined);
    }
    if !f.is_nan() {
      return Ok(BridgeValue::Number(f));
    }
    let id = mem.i32_load(addr)? as usize;
    Ok(values[id])
  }

  fn store_value(&self, addr: u32, value: BridgeValue) -> Result<(), RuntimeError> {
    let mem = self.get_memory_ref();
    let nan_head = 0x7ff8_0000;

    match value {
      BridgeValue::Number(n) => {
        if n == 0.0 {
          mem.u32_store(addr + 4, nan_head)?;
          mem.u32_store(addr, 1)?;
          return Ok(());
        }
        mem.f64_store(addr, n)?;
        return Ok(());
      }
      BridgeValue::NaN => {
        mem.u32_store(addr + 4, nan_head)?;
        mem.u32_store(addr, 0)?;
        return Ok(());
      }
      BridgeValue::Undefined => {
        mem.f64_store(addr, 0.0)?;
        return Ok(());
      }
      BridgeValue::Null => {
        mem.u32_store(addr + 4, nan_head)?;
        mem.u32_store(addr, 2)?;
        return Ok(());
      }
      BridgeValue::True => {
        mem.u32_store(addr + 4, nan_head)?;
        mem.u32_store(addr, 3)?;
        return Ok(());
      }
      BridgeValue::False => {
        mem.u32_store(addr + 4, nan_head)?;
        mem.u32_store(addr, 4)?;
        return Ok(());
      }
      _ => {}
    };
    let type_flag = 0;
    let len = self.values.borrow().len();
    self.values.borrow_mut().push(value);
    /*

    let ref = this._refs.get(v);
    if (ref === undefined) {
      ref = this._values.length;
      this._values.push(v);
      this._refs.set(v, ref);
    }
    switch (typeof v) {
      case "string":
        typeFlag = 1;
        break;
      case "symbol":
        typeFlag = 2;
        break;
      case "function":
        typeFlag = 3;
        break;
    }
        */
    mem.u32_store(addr + 4, nan_head | type_flag);
    mem.u32_store(addr, len as u32);

    Ok(())
  }

  fn strPtr(&self) {
    /*
    const strPtr = str => {
      const ptr = offset;
      const bytes = encoder.encode(str + "\0");
      new Uint8Array(mem.buffer, offset, bytes.length).set(bytes);
      offset += bytes.length;
      if (offset % 8 !== 0) {
        offset += 8 - (offset % 8);
      }
      return ptr;
    };
    */
  }

  fn run(&self, inst: VM<'a>) -> () {
    // self.inst = Rc::new(RefCell::new(Some(inst)));
    // let mut mem = inst.resolve_memory().unwrap();
    // let mem = mem.as_mut().unwrap();
    {
      let mut m = self.inst.borrow_mut();
      *m = Some(inst);
    }
    //   // self.exports = Some(Rc::clone(Rc::new(exports)));
    // let v = vec![0];
    // mem.set(&v, 0);
    /*
    this._inst = instance;
    this._values = [
      // TODO: garbage collection
      NaN,
      0,
      null,
      true,
      false,
      global,
      this
    ];
    this._refs = new Map();

    const mem = new DataView(this._inst.exports.mem.buffer);
    // Pass command line arguments and environment variables to WebAssembly by writing them to the linear memory.
    */
    let offset = 4096;
    let argc = self.argv.len();
    // let mut argvPtrs = vec![];

    // for arg in self.argv {
    // argvPtrs.push(self.strPtr(arg));
    // }
    let argv = offset;

    // argvPtrs.forEach(ptr => {
    //   mem.setUint32(offset, ptr, true);
    //   mem.setUint32(offset + 4, 0, true);
    //   offset += 8;
    // });
    // this._inst.exports.run(argc, argv);
    // inst
    //   .invoke(
    //     "run",
    //     &[
    //       RuntimeValue::I32(argc as i32),
    //       RuntimeValue::I32(argv as i32),
    //     ],
    //   )
    //   .unwrap();
    // dbg!("end");
    // // await this._exitPromise;
    // loop {}
    self.start(argc as i32, argv as i32);
  }

  fn start(&self, argc: i32, argv: i32) -> Result<(), RuntimeError> {
    let inst = self.inst.borrow();
    let inst = inst.as_ref().unwrap();
    inst
      .invoke("run", &[RuntimeValue::I32(argc), RuntimeValue::I32(argv)])
      .unwrap();
    Ok(())
  }
}

/*
          "runtime.nanotime": sp => {
            setInt64(sp + 8, (timeOrigin + performance.now()) * 1000000);
          },
*/
