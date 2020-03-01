extern crate yaw;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Point;

use self::yaw::*;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

#[derive(Debug, Clone, PartialEq)]
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
  WrappedFunc(WrappedFunc),
  Uint8ArrayConstructor,
  Uint8Array(Uint8Array),
  Array,
  Object,
  Fs,
  Constants,
  Arguments(Vec<Arg>),
  Arg(Arg),
  PendingEvent(Option<PendingEvent>),
}

#[derive(Debug, Clone, PartialEq)]
enum Arg {
  Uint8Array(Rc<Vec<u8>>),
  Null,
  Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct WrappedFunc(EventId);

#[derive(Debug, Clone, PartialEq)]
struct PendingEvent {
  id: usize,
  args: Vec<Arg>,
  result: Box<Option<BridgeValue>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Uint8Array {
  buf: Vec<u8>,
}

impl Uint8Array {
  fn new(buf: Vec<u8>) -> Self {
    Self { buf }
  }

  fn get(&self, i: usize) -> u8 {
    self.buf[i]
  }
}

#[derive(Debug, Clone, PartialEq)]
struct EventId(usize);

fn main() -> Result<(), error::YawError> {
  let go = Go::new();
  let mut imports = Imports::new();
  imports.add_function(&go);
  let mut ins = instantiate(
    &include_bytes!("./gopher-boy/docs/main.wasm")[..],
    Some(&imports),
  )?;

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("yaw-boy", WIDTH, HEIGHT)
    .position_centered()
    .build()
    .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();
  go.run(&mut ins)?;
  go.create_gb(&mut ins);
  loop {
    go.next(&mut ins);
    let id = go.copied_id.get();
    let values = go.values.borrow();
    if let BridgeValue::Uint8Array(buf) = &values[id] {
      for i in 0..HEIGHT {
        for j in 0..WIDTH {
          let base = ((i * WIDTH + j) * 4) as usize;
          let r = buf.get(base);
          let g = buf.get(base + 1);
          let b = buf.get(base + 2);
          canvas.set_draw_color(Color::RGB(r, g, b));
          let _ = canvas.draw_point(Point::new(j as i32, i as i32));
        }
      }
    }
    canvas.present();
  }
}

#[derive(Debug)]
struct Go {
  argv: Vec<String>,
  exited: bool,
  values: RefCell<Vec<BridgeValue>>,
  gb: RefCell<Option<WrappedFunc>>,
  pending_event: RefCell<Option<PendingEvent>>,
  next: RefCell<Option<WrappedFunc>>,
  exports: Option<Rc<Exports>>,
  // HACK: Id to know where it was copied
  copied_id: Cell<usize>,
}

impl FunctionResolver for Go {
  fn invoke(
    &self,
    ins: &mut VM,
    _name: &str,
    field_name: &str,
    args: &[RuntimeValue],
  ) -> Result<Vec<RuntimeValue>, RuntimeError> {
    match field_name {
      "syscall/js.valueGet" => {
        let sp: u32 = args[0].into();
        let v = self.load_value(sp + 8, &self.values.borrow(), ins)?;
        let name: &str = &self.load_string(sp + 16, ins).unwrap();
        let sp = self.get_sp(ins)?;
        if let BridgeValue::PendingEvent(Some(e)) = v.clone() {
          match name {
            "id" => {
              self.store_value(sp + 32, BridgeValue::Number(e.id as f64), ins)?;
              return Ok(vec![]);
            }
            "this" => {
              self.store_value(sp + 32, BridgeValue::This, ins)?;
              return Ok(vec![]);
            }
            "args" => {
              self.store_value(sp + 32, BridgeValue::Arguments(e.args), ins)?;
              return Ok(vec![]);
            }
            _ => {}
          }
        }
        match name {
          "O_WRONLY" | "O_RDWR" | "O_CREAT" | "O_TRUNC" | "O_APPEND" | "O_EXCL" => {
            self.store_value(sp + 32, BridgeValue::Number(-1.0), ins)?;
            return Ok(vec![]);
          }
          "Uint8Array" => {
            self.store_value(sp + 32, BridgeValue::Uint8ArrayConstructor, ins)?;
            return Ok(vec![]);
          }
          "Array" => {
            self.store_value(sp + 32, BridgeValue::Array, ins)?;
            return Ok(vec![]);
          }
          "Object" => {
            self.store_value(sp + 32, BridgeValue::Object, ins)?;
            return Ok(vec![]);
          }
          "fs" => {
            self.store_value(sp + 32, BridgeValue::Fs, ins)?;
            return Ok(vec![]);
          }
          "process" => {
            self.store_value(sp + 32, BridgeValue::Undefined, ins)?;
            return Ok(vec![]);
          }
          "constants" => {
            self.store_value(sp + 32, BridgeValue::Constants, ins)?;
            return Ok(vec![]);
          }
          "_pendingEvent" => {
            let e = self.pending_event.borrow().clone();
            self.store_value(sp + 32, BridgeValue::PendingEvent(e), ins)?;
            return Ok(vec![]);
          }
          "length" => {
            if let BridgeValue::Arg(Arg::Uint8Array(a)) = &v {
              self.store_value(sp + 32, BridgeValue::Number(a.len() as f64), ins)?;
              return Ok(vec![]);
            }
            unreachable!();
          }
          _ => {}
        }
        self.store_value(sp + 32, v, ins)?;
        Ok(vec![])
      }
      "syscall/js.valueCall" => {
        let sp: u32 = args[0].into();
        self.value_call(sp, ins)?;
        Ok(vec![])
      }
      "syscall/js.valueSet" => {
        let sp: u32 = args[0].into();
        let _value = self.load_value(sp + 8, &self.values.borrow(), ins)?;
        let name: &str = &self.load_string(sp + 16, ins).unwrap();
        let value = &self.load_value(sp + 32, &self.values.borrow(), ins)?;
        match name {
          "GB" => {
            if let BridgeValue::WrappedFunc(f) = value.clone() {
              *self.gb.borrow_mut() = Some(f);
            }
          }
          "result" => {}
          "next" => {
            if let BridgeValue::WrappedFunc(f) = value.clone() {
              *self.next.borrow_mut() = Some(f);
            }
          }
          _ => {}
        }
        Ok(vec![])
      }
      "syscall/js.valueLength" => {
        let sp: u32 = args[0].into();
        let value = &self.load_value(sp + 8, &self.values.borrow(), ins)?;
        if let BridgeValue::Arguments(args) = value {
          self.set_int64(sp + 16, args.len() as u32, ins)?;
        } else {
          unimplemented!()
        }
        Ok(vec![])
      }
      "syscall/js.valueIndex" => {
        let sp: u32 = args[0].into();
        let value = self.load_value(sp + 8, &self.values.borrow(), ins)?;
        let index = self.get_int64(sp + 16, ins)?;
        match value {
          BridgeValue::Arguments(arg) => {
            self.store_value(sp + 24, BridgeValue::Arg(arg[index as usize].clone()), ins)?;
          }
          BridgeValue::Uint8Array(_u8arr) => unimplemented!(),
          BridgeValue::Arg(Arg::Uint8Array(u8arr)) => {
            self.store_value(
              sp + 24,
              BridgeValue::Number(u8arr[index as usize] as f64),
              ins,
            )?;
          }
          _ => unimplemented!(),
        }
        Ok(vec![])
      }
      "syscall/js.valueNew" => {
        let sp: u32 = args[0].into();
        let value = self.load_value(sp + 8, &self.values.borrow(), ins)?;
        let args = self.load_values(sp + 16, &self.values.borrow(), ins)?;
        match value {
          BridgeValue::Uint8ArrayConstructor => {
            if let BridgeValue::Number(arg) = args[0] {
              let b = vec![0; arg as usize];
              let result = Uint8Array::new(b);
              self.store_value(sp + 40, BridgeValue::Uint8Array(result), ins)?;
              let mem = self.get_memory_ref(ins);
              mem.u8_store(sp + 48, 1)?;
              return Ok(vec![]);
            }
          }
          _ => unreachable!(),
        }
        Ok(vec![])
      }
      "syscall/js.copyBytesToJS" => {
        let sp: u32 = args[0].into();
        let src = self.load_slice(sp + 16, ins)?;
        let len = src.len();
        self.update_value(sp + 8, BridgeValue::Uint8Array(Uint8Array::new(src)), ins)?;
        self.set_int64(sp + 40, len as u32, ins)?;
        let mem = self.get_memory_ref(ins);
        mem.u8_store(sp + 48, 1)?;
        Ok(vec![])
      }
      _ => Ok(vec![]),
    }
  }
}

impl Go {
  fn new() -> Self {
    Self {
      // inst: RefCell::new(None),
      argv: vec!["js".to_owned()],
      exited: false,
      exports: None,
      values: RefCell::new(vec![
        BridgeValue::NaN,
        BridgeValue::Zero,
        BridgeValue::Null,
        BridgeValue::True,
        BridgeValue::False,
        BridgeValue::Global,
        BridgeValue::This,
      ]),
      gb: RefCell::new(None),
      next: RefCell::new(None),
      pending_event: RefCell::new(None),
      copied_id: Cell::new(0),
    }
  }

  fn create_gb(&self, ins: &mut VM) {
    if let Some(func) = self.gb.borrow().clone() {
      let e = func.0;
      let id = e.0;
      let rom = Rc::new(Vec::from(&include_bytes!("./gopher-boy/docs/tobu.gb")[..]));
      *self.pending_event.borrow_mut() = Some(PendingEvent {
        id,
        args: vec![Arg::Uint8Array(rom)],
        result: Box::new(None),
      });
    }
    self.resume(ins);
  }

  fn next(&self, ins: &mut VM) {
    let buf = Rc::new(vec![0; 160 * 144 * 4]);
    if let Some(func) = self.next.borrow().clone() {
      let e = func.0;
      let id = e.0;
      *self.pending_event.borrow_mut() = Some(PendingEvent {
        id,
        args: vec![Arg::Uint8Array(buf)],
        result: Box::new(None),
      });
    }
    self.resume(ins);
  }

  fn resume(&self, ins: &mut VM) {
    ins.invoke("resume", &[]).unwrap();
  }

  fn get_memory_ref(&self, ins: &mut VM) -> MemoryRef {
    ins.resolve_memory().unwrap()
  }

  fn load_string(&self, addr: u32, ins: &mut VM) -> Result<String, RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let start = mem.i64_load(addr)? as usize;
    let len = mem.i64_load(addr + 8)? as usize;
    let s = mem.to_string(start, len)?;
    Ok(s)
  }

  fn load_slice(&self, addr: u32, ins: &mut VM) -> Result<Vec<u8>, RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let start = self.get_int64(addr, ins)?;
    let len = self.get_int64(addr + 8, ins)?;
    let s = mem.slice(start as usize, len as usize)?;
    Ok(s)
  }

  fn load_values(
    &self,
    addr: u32,
    values: &[BridgeValue],
    ins: &mut VM,
  ) -> Result<Vec<BridgeValue>, RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let start = mem.i64_load(addr)? as usize;
    let len = mem.i64_load(addr + 8)? as usize;
    let mut a = vec![];
    for i in 0..len {
      a.push(self.load_value((start + i * 8) as u32, values, ins)?);
    }
    Ok(a)
  }
  fn load_value(
    &self,
    addr: u32,
    values: &[BridgeValue],
    ins: &mut VM,
  ) -> Result<BridgeValue, RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let f = mem.f64_load(addr)? as f64;
    if f == 0.0 {
      return Ok(BridgeValue::Undefined);
    }
    if !f.is_nan() {
      return Ok(BridgeValue::Number(f));
    }
    let id = mem.i32_load(addr)? as usize;
    Ok(values[id].clone())
  }

  fn update_value(&self, addr: u32, value: BridgeValue, ins: &mut VM) -> Result<(), RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let id = mem.i32_load(addr)? as usize;
    let mut v = self.values.borrow_mut();
    v[id] = value;
    self.copied_id.set(id);
    Ok(())
  }

  fn store_value(&self, addr: u32, value: BridgeValue, ins: &mut VM) -> Result<(), RuntimeError> {
    let mem = self.get_memory_ref(ins);
    let nan_head = 0x7ff8_0000;

    match value {
      BridgeValue::Arg(Arg::Number(n)) | BridgeValue::Number(n) => {
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
      BridgeValue::Arg(Arg::Null) | BridgeValue::Null => {
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
    let mut type_flag = 0;
    let len = self.values.borrow().len();
    self.values.borrow_mut().push(value.clone());
    if let BridgeValue::WrappedFunc(_) = value {
      type_flag = 3;
    }
    mem.u32_store(addr + 4, nan_head | type_flag)?;
    mem.u32_store(addr, len as u32)?;

    Ok(())
  }

  fn set_int64(&self, addr: u32, v: u32, ins: &mut VM) -> Result<(), RuntimeError> {
    let mem = self.get_memory_ref(ins);
    mem.u32_store(addr, v)?;
    mem.u32_store(addr + 4, (v as f64 / 4_294_967_296.0).floor() as u32)?;
    Ok(())
  }

  fn get_int64(&self, addr: u32, ins: &mut VM) -> Result<i64, RuntimeError> {
    let mem = self.get_memory_ref(ins);
    Ok(mem.i64_load(addr)?)
  }

  fn value_call(&self, sp: u32, ins: &mut VM) -> Result<(), RuntimeError> {
    let _ = self.load_value(sp + 8, &self.values.borrow(), ins);
    let s: &str = &self.load_string(sp + 16, ins)?;
    let args = self.load_values(sp + 32, &self.values.borrow(), ins)?;
    let result = match s {
      "_makeFuncWrapper" => {
        if let BridgeValue::Number(id) = args[0] {
          BridgeValue::WrappedFunc(WrappedFunc(EventId(id as usize)))
        } else {
          unreachable!();
        }
      }
      "write" => {
        if let BridgeValue::WrappedFunc(WrappedFunc(EventId(id))) = &args[5] {
          if let BridgeValue::Uint8Array(u8_arr) = &args[1] {
            let id = *id;
            *self.pending_event.borrow_mut() = Some(PendingEvent {
              id,
              args: vec![Arg::Null, Arg::Number(u8_arr.buf.len() as f64)],
              result: Box::new(None),
            });
            self.resume(ins);
            BridgeValue::Undefined
          } else {
            unreachable!();
          }
        } else {
          unreachable!();
        }
      }
      _ => unimplemented!("{}", s),
    };
    let sp = self.get_sp(ins)?;
    self.store_value(sp + 56, result, ins)?;
    Ok(())
  }

  fn get_sp(&self, ins: &mut VM) -> Result<u32, RuntimeError> {
    let sp: u32 = ins.invoke("getsp", &[]).unwrap()[0].into();
    Ok(sp)
  }

  fn run(&self, ins: &mut VM) -> Result<(), RuntimeError> {
    let offset = 4096;
    let argc = self.argv.len();
    let argv = offset;
    self.start(argc as i32, argv as i32, ins)?;
    Ok(())
  }

  fn start(&self, argc: i32, argv: i32, ins: &mut VM) -> Result<(), RuntimeError> {
    ins
      .invoke("run", &[RuntimeValue::I32(argc), RuntimeValue::I32(argv)])
      .unwrap();
    Ok(())
  }
}
