(module
  (import "env" "memory" (memory 1))
  (func $load (param $lhs i32) (param $rhs i32) (result i32)
    i32.const 0
    i32.load)
  (export "load" (func $load))
)