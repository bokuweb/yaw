use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use wabt::script::{Action, Command, CommandKind, ScriptParser, Value};
use yaw::types::*;
use yaw::{
    ExternalKind, FunctionResolver, Global, ImportResolver, MemoryDescriptor, MemoryRef,
    RuntimeError, TableInstance, TableRef, ValueType, VM,
};

#[derive(Debug, Clone)]
struct SpecModuleInternal<'a> {
    func: Option<&'a dyn FunctionResolver>,
    memory: HashMap<String, HashMap<String, MemoryRef>>,
    // Please add `mutability` property when mutable global allowed.
    global: HashMap<String, HashMap<String, Rc<RefCell<Global>>>>,
    table: HashMap<String, HashMap<String, TableRef>>,
    modules: HashMap<Option<String>, Rc<RefCell<VM<'a>>>>,
}

#[derive(Debug, Clone)]
struct SpecModule<'a> {
    inner: Rc<RefCell<SpecModuleInternal<'a>>>,
}

impl<'a> SpecModule<'a> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SpecModuleInternal {
                func: None,
                memory: HashMap::new(),
                global: HashMap::new(),
                table: HashMap::new(),
                modules: HashMap::new(),
            })),
        }
    }
    pub fn add_memory(
        &self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        memory: MemoryRef,
    ) {
        let mut m = HashMap::new();
        m.insert(field_name.into(), memory);
        self.inner.borrow_mut().memory.insert(name.into(), m);
    }

    pub fn add_global(
        &self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        value: Rc<RefCell<Global>>,
    ) {
        let module_name = name.into();
        let mut inner = self.inner.borrow_mut();
        let map = inner.global.get_mut(&module_name);
        if map.is_some() {
            if let Some(m) = map {
                m.insert(field_name.into(), value);
            }
        } else {
            let mut m = HashMap::new();
            m.insert(field_name.into(), value);
            inner.global.insert(module_name, m);
        }
    }

    pub fn add_table(
        &self,
        name: impl Into<String>,
        field_name: impl Into<String>,
        table: TableRef,
    ) {
        let mut m = HashMap::new();
        m.insert(field_name.into(), table);
        self.inner.borrow_mut().table.insert(name.into(), m);
    }

    pub fn add_module(&self, name: Option<String>, module_ref: Rc<RefCell<VM<'a>>>) {
        self.inner.borrow_mut().modules.insert(name, module_ref);
    }
}

impl<'a> FunctionResolver for SpecModule<'a> {
    fn invoke(
        &self,
        _ins: &mut VM,
        name: &str,
        field_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Vec<RuntimeValue>, RuntimeError> {
        if name == "spectest" {
            return match field_name {
                "print" => Ok(vec![]),
                "print_i32" => Ok(vec![]),
                "print_i32_f32" => Ok(vec![]),
                "print_f64_f64" => Ok(vec![]),
                "print_f32" => Ok(vec![]),
                "print_f64" => Ok(vec![]),
                _ => Ok(vec![]),
            };
        }
        let inner = self.inner.borrow();
        let mut m = inner
            .modules
            .get(&Some(name.to_owned()))
            .expect("should get module");
        let res = m.borrow_mut().invoke(field_name, args).unwrap();
        Ok(res)
    }
}

impl<'a> ImportResolver for SpecModule<'a> {
    fn resolve_memory(&self, name: String, field_name: String) -> Result<MemoryRef, RuntimeError> {
        let inner = self.inner.borrow();
        let m = inner.memory.get(&name);
        if let Some(map) = m {
            let mem = map.get(&field_name);
            if let Some(mem) = mem {
                return Ok(mem.clone());
            }
        }
        Err(RuntimeError::UndefinedMemoryError)
    }

    fn resolve_global(
        &self,
        name: String,
        field_name: String,
    ) -> Result<Rc<RefCell<Global>>, RuntimeError> {
        let inner = self.inner.borrow();
        let m = inner.global.get(&name);
        if let Some(map) = m {
            if let Some(value) = map.get(&field_name) {
                return Ok(Rc::clone(value));
            }
        }
        Err(RuntimeError::UndefinedGlobalError)
    }

    fn resolve_table(&self, name: String, field_name: String) -> Result<TableRef, RuntimeError> {
        let inner = self.inner.borrow();
        let m = inner.table.get(&name);
        if let Some(map) = m {
            if let Some(value) = map.get(&field_name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError::UndefinedTableError)
    }

    fn resolve_function(&self) -> Result<&dyn FunctionResolver, RuntimeError> {
        Ok(self)
    }
}

pub fn exec_testsuite(p: impl AsRef<Path>) -> Result<(), yaw::error::YawError> {
    let m = SpecModule::new();
    m.add_table(
        "spectest",
        "table",
        Rc::new(RefCell::new(TableInstance::new(10, Some(20)))),
    );
    m.add_global(
        "spectest",
        "global_i32",
        Rc::new(RefCell::new(Global::new(
            false,
            RuntimeValue::I32(666),
            ValueType::I32,
        ))),
    );
    m.add_global(
        "spectest",
        "global_i64",
        Rc::new(RefCell::new(Global::new(
            false,
            RuntimeValue::I64(666),
            ValueType::I64,
        ))),
    );
    m.add_global(
        "spectest",
        "global_f32",
        Rc::new(RefCell::new(Global::new(
            false,
            RuntimeValue::F32(666.0),
            ValueType::F32,
        ))),
    );
    m.add_global(
        "spectest",
        "global_f64",
        Rc::new(RefCell::new(Global::new(
            false,
            RuntimeValue::F64(666.0),
            ValueType::F64,
        ))),
    );
    m.add_memory(
        "spectest",
        "memory",
        MemoryRef::new(MemoryDescriptor::new(1, Some(2))),
    );

    let mut buf = vec![];
    let mut file = fs::File::open(p.as_ref())?;
    file.read_to_end(&mut buf)?;
    let s = String::from_utf8(buf).unwrap();
    let mut parser = ScriptParser::from_str(&s).unwrap();
    while let Some(Command { kind, .. }) = parser.next().unwrap() {
        match kind {
            CommandKind::Module { module, name } => {
                let module_binary = module.into_vec();
                let ins = Rc::new(RefCell::new(
                    yaw::instantiate(&module_binary, Some(&m)).unwrap(),
                ));
                m.add_module(None, ins.clone());
                m.add_module(name.clone(), ins.clone());
            }
            CommandKind::AssertReturn { action, expected } => {
                if let Action::Invoke {
                    field,
                    args,
                    module,
                    ..
                } = action
                {
                    let ins_ref = m.inner.borrow().modules[&module].clone();
                    let mut ins = ins_ref.borrow_mut();
                    let args: Vec<RuntimeValue> = args
                        .into_iter()
                        .map(|arg| match arg {
                            Value::I32(v) => RuntimeValue::I32(v),
                            Value::I64(v) => RuntimeValue::I64(v),
                            Value::F32(v) => RuntimeValue::F32(v),
                            Value::F64(v) => RuntimeValue::F64(v),
                            Value::V128(v) => RuntimeValue::V128(v),
                        })
                        .collect();
                    let ret = ins.invoke(&field.to_string(), &args)?;
                    if !ret.is_empty() {
                        match ret[ret.len() - 1] {
                            RuntimeValue::I32(v) => assert_eq!(expected, vec![Value::I32(v)]),
                            RuntimeValue::I64(v) => assert_eq!(expected, vec![Value::I64(v)]),
                            RuntimeValue::F32(v) => {
                                if let Value::F32(e) = expected[0] {
                                    if e.is_nan() {
                                        assert!(v.is_nan())
                                    } else {
                                        assert_eq!(expected, vec![Value::F32(v)])
                                    }
                                }
                            }
                            RuntimeValue::F64(v) => {
                                if let Value::F64(e) = expected[0] {
                                    if e.is_nan() {
                                        assert!(v.is_nan())
                                    } else {
                                        assert_eq!(expected, vec![Value::F64(v)])
                                    }
                                }
                            }
                            RuntimeValue::V128(_v) => unreachable!(),
                        }
                    }
                }
            }
            CommandKind::AssertTrap { action, message } => {
                if let Action::Invoke {
                    field,
                    args,
                    module,
                    ..
                } = action
                {
                    let ins_ref = m.inner.borrow().modules[&module].clone();
                    let mut ins = ins_ref.borrow_mut();
                    let args: Vec<RuntimeValue> = args
                        .into_iter()
                        .map(|arg| match arg {
                            Value::I32(v) => RuntimeValue::I32(v),
                            Value::I64(v) => RuntimeValue::I64(v),
                            Value::F32(v) => RuntimeValue::F32(v),
                            Value::F64(v) => RuntimeValue::F64(v),
                            Value::V128(v) => RuntimeValue::V128(v),
                        })
                        .collect();
                    let ret = ins.invoke(&field.to_string(), &args);
                    match ret {
                        Err(err) => assert_eq!(message, err.to_string()),
                        Ok(_) => panic!("error"),
                    }
                }
                // }
            }
            CommandKind::AssertInvalid { .. } => {
                // TODO: We need to add validation when loading.RuntimeValue
                //       Then add invalid assertion
            }
            CommandKind::AssertMalformed { .. } => {
                // TODO:
            }
            CommandKind::AssertReturnCanonicalNan { .. } => {
                // TODO:
            }
            CommandKind::AssertReturnArithmeticNan { .. } => {
                // TODO:
            }
            CommandKind::AssertExhaustion { .. } => {
                // TODO:
            }
            CommandKind::AssertUnlinkable { .. } => {
                // TODO:
            }
            CommandKind::Register { name, as_name, .. } => {
                let module = Rc::clone(
                    m.inner
                        .borrow()
                        .modules
                        .get(&name)
                        .expect("There are no module"),
                );
                let mo = module.borrow();
                let exports = mo.exports();
                m.add_module(Some(as_name.clone()), Rc::clone(&module));

                for (k, v) in exports.inner() {
                    match v.kind {
                        ExternalKind::Table => {
                            m.add_table(as_name.clone(), k.clone(), mo.resolve_table());
                        }
                        ExternalKind::Global => {
                            m.add_global(
                                as_name.clone(),
                                k.clone(),
                                mo.resolve_global(v.index as usize)?,
                            );
                        }
                        ExternalKind::Memory => {
                            m.add_memory(
                                as_name.clone(),
                                k.clone(),
                                mo.resolve_memory().unwrap().clone(),
                            );
                        }
                        _ => {}
                    }
                }
            }
            _ => panic!("there are no other commands apart from that defined above"),
        }
    }
    Ok(())
}
