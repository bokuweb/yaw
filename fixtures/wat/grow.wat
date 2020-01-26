(module
(memory 2)
  (func $memory (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add
    grow_memory
    drop
    current_memory)
  (export "grow" (func $memory))
)