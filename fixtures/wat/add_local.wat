(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32) (local i32)
    i32.const 200
    set_local 2
    get_local $lhs
    get_local 2
    i32.add)
  (export "add" (func $add))
)