(module
  (import "env" "memory" (memory 1))
  (func $store (param $lhs i32) (param $rhs i32) (result i32)
    i32.const 2
    i32.const 0x5a5aa5a5
    i32.store
    i32.const 2
    i32.load)
  (export "store" (func $store))
)