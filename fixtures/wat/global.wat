(module
(global i32 (i32.const 1))
(global $foo i64 (i64.const 100))
(global $bar (mut f32) (f32.const 1.11))
  (func $global (param $lhs i32) (result f32)
    get_global 0
    f32.convert_s/i32
    f32.const 4.1
    f32.add
    set_global 2
    get_global 2)
  (export "global" (func $global))
)