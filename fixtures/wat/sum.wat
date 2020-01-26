(module
  (type $t0 (func (param i32) (result i32)))
  (func $sum (export "sum") (type $t0) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32)
    loop $L0
      block $B1
        get_local $l0
        get_local $p0
        i32.ge_s
        br_if $B1
        get_local $l0
        get_local $l1
        i32.add
        set_local $l1
        get_local $l0
        i32.const 1
        i32.add
        set_local $l0
        br $L0
      end
    end
    get_local $l1)
)