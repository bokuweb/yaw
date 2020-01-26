(module
  (func $sub (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.sub)
  (export "sub" (func $sub))
)