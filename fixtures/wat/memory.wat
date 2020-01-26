(module
(memory 2 3)
  (func $memory (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add)
  (export "memory" (func $memory))
)