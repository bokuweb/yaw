(module
  (func $if (param $lhs i32) (result i32)
    get_local $lhs
    if (result i32)
      i32.const 10
    else
      i32.const 20
    end)
  (export "if_else" (func $if))
)