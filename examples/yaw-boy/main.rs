extern crate yaw;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use self::yaw::*;

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

fn load_string(mem: &MemoryRef, addr: u32) -> Result<String, RuntimeError> {
  let start = mem.i64_load(addr)? as usize;
  let len = mem.i64_load(addr + 8)? as usize;
  let s = mem.to_string(start, len)?;
  dbg!(&s);
  Ok(s)
}

#[derive(Debug)]
struct Go {
  // inst: Option<VM<'a>>,
  mem: Rc<RefCell<Option<MemoryRef>>>,
  argv: Vec<String>,
  exited: bool,
}

impl FunctionResolver for Go {
  fn invoke(
    &self,
    name: &str,
    field_name: &str,
    args: &[RuntimeValue],
  ) -> Result<Vec<RuntimeValue>, RuntimeError> {
    // if name == "spectest" {
    dbg!(field_name);
    return match field_name {
      "syscall/js.valueGet" => {
        dbg!(field_name, &args);
        let m = self.mem.borrow();
        let m = m.as_ref().unwrap();
        let sp: u32 = args[0].into();
        load_string(m, sp + 16).unwrap();
        // const result = Reflect.get(loadValue(sp + 8), loadString(sp + 16));
        // sp = this._inst.exports.getsp(); // see comment above
        // storeValue(sp + 32, result);
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

impl Go {
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
      mem: Rc::new(RefCell::new(None)),
      argv: vec!["js".to_owned()],
      exited: false,
    }
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

  fn run<'a>(&self, inst: VM<'a>) -> () {
    // self.inst = Some(inst);
    let mut mem = inst.resolve_memory().unwrap();
    // let mem = mem.as_mut().unwrap();
    {
      let mut m = self.mem.borrow_mut();
      *m = Some(mem);
    }
    let v = vec![0];
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
    inst
      .invoke(
        "run",
        &[
          RuntimeValue::I32(argc as i32),
          RuntimeValue::I32(argv as i32),
        ],
      )
      .unwrap();
    dbg!("end");
    // await this._exitPromise;
    loop {}
  }
}

/*
          "runtime.nanotime": sp => {
            setInt64(sp + 8, (timeOrigin + performance.now()) * 1000000);
          },
*/
