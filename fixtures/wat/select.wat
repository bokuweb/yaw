(module
  (func $select (result i32)
      i32.const 10
      i32.const 20
      i32.const 0
      select)
  (export "select" (func $select))
)