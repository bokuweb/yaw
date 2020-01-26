(module
  (type $t0 (func (result i32)))
  (type $t1 (func (result i32)))
  (func $call (export "call") (type $t0) (result i32)
    call $f1)
  (func $f1 (type $t1)
    i32.const 42))
