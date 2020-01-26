(module
  (type $t0 (func))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)))
  (type $t3 (func (param i32 i32)))
  (type $t4 (func (param i32) (result i32)))
  (func $<alloc::raw_vec::RawVec<T__A>>::allocate_in::__closure__::h8c98326fcf61324e (type $t0)
    call $alloc::raw_vec::capacity_overflow::h034ca36241ac64a2
    unreachable)
  (func $alloc::raw_vec::capacity_overflow::h034ca36241ac64a2 (type $t0)
    i32.const 1548
    call $core::panicking::panic::haf7d7779169c0743
    unreachable)
  (func $<alloc::raw_vec::RawVec<T__A>>::allocate_in::__closure__::hd2ed2b152d4b34da (type $t0)
    call $alloc::raw_vec::capacity_overflow::h034ca36241ac64a2
    unreachable)
  (func $std::panicking::rust_panic_with_hook::h9b1c029d1ceaded2 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 1
    set_local $l0
    block $B0
      block $B1
        block $B2
          i32.const 0
          i32.load offset=1024
          i32.const 1
          i32.ne
          br_if $B2
          i32.const 0
          i32.const 0
          i32.load offset=1028
          i32.const 1
          i32.add
          tee_local $l0
          i32.store offset=1028
          get_local $l0
          i32.const 3
          i32.lt_u
          br_if $B1
          br $B0
        end
        i32.const 0
        i64.const 4294967297
        i64.store offset=1024
      end
      i32.const 0
      i32.load offset=1036
      tee_local $l1
      i32.const -1
      i32.le_s
      br_if $B0
      i32.const 0
      get_local $l1
      i32.store offset=1036
      get_local $l0
      i32.const 2
      i32.lt_u
      drop
    end
    unreachable
    unreachable)
  (func $rust_begin_unwind (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) (param $p6 i32) (param $p7 i32) (param $p8 i32) (param $p9 i32)
    (local $l0 i32)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 20
    i32.add
    get_local $p3
    i32.store
    get_local $l0
    i32.const 28
    i32.add
    get_local $p5
    i32.store
    get_local $l0
    get_local $p1
    i32.store offset=12
    get_local $l0
    get_local $p0
    i32.store offset=8
    get_local $l0
    get_local $p2
    i32.store offset=16
    get_local $l0
    get_local $p4
    i32.store offset=24
    get_local $l0
    get_local $p7
    i32.store offset=36
    get_local $l0
    get_local $p6
    i32.store offset=32
    get_local $l0
    get_local $p8
    i32.store offset=40
    get_local $l0
    get_local $p9
    i32.store offset=44
    get_local $l0
    i32.const 8
    i32.add
    get_local $l0
    i32.const 32
    i32.add
    call $std::panicking::begin_panic_fmt::h29d4906ca23d78a0
    unreachable)
  (func $std::panicking::begin_panic_fmt::h29d4906ca23d78a0 (type $t3) (param $p0 i32) (param $p1 i32)
    get_local $p1
    call $std::panicking::rust_panic_with_hook::h9b1c029d1ceaded2
    unreachable)
  (func $rust_oom (type $t3) (param $p0 i32) (param $p1 i32)
    unreachable
    unreachable)
  (func $dlmalloc::dlmalloc::Dlmalloc::malloc::hce1b00d5aca5677c (type $t4) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i64)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    block $B8
                      block $B9
                        block $B10
                          block $B11
                            block $B12
                              block $B13
                                block $B14
                                  block $B15
                                    block $B16
                                      block $B17
                                        block $B18
                                          block $B19
                                            block $B20
                                              block $B21
                                                block $B22
                                                  block $B23
                                                    block $B24
                                                      block $B25
                                                        block $B26
                                                          block $B27
                                                            block $B28
                                                              block $B29
                                                                block $B30
                                                                  block $B31
                                                                    block $B32
                                                                      block $B33
                                                                        block $B34
                                                                          block $B35
                                                                            block $B36
                                                                              get_local $p0
                                                                              i32.const 244
                                                                              i32.gt_u
                                                                              br_if $B36
                                                                              i32.const 0
                                                                              i32.load offset=1044
                                                                              tee_local $l0
                                                                              i32.const 16
                                                                              get_local $p0
                                                                              i32.const 11
                                                                              i32.add
                                                                              i32.const -8
                                                                              i32.and
                                                                              get_local $p0
                                                                              i32.const 11
                                                                              i32.lt_u
                                                                              select
                                                                              tee_local $l1
                                                                              i32.const 3
                                                                              i32.shr_u
                                                                              tee_local $l2
                                                                              i32.const 31
                                                                              i32.and
                                                                              tee_local $l3
                                                                              i32.shr_u
                                                                              tee_local $p0
                                                                              i32.const 3
                                                                              i32.and
                                                                              i32.eqz
                                                                              br_if $B35
                                                                              get_local $p0
                                                                              i32.const -1
                                                                              i32.xor
                                                                              i32.const 1
                                                                              i32.and
                                                                              get_local $l2
                                                                              i32.add
                                                                              tee_local $l1
                                                                              i32.const 3
                                                                              i32.shl
                                                                              tee_local $l3
                                                                              i32.const 1060
                                                                              i32.add
                                                                              i32.load
                                                                              tee_local $p0
                                                                              i32.const 8
                                                                              i32.add
                                                                              set_local $l4
                                                                              get_local $p0
                                                                              i32.load offset=8
                                                                              tee_local $l2
                                                                              get_local $l3
                                                                              i32.const 1052
                                                                              i32.add
                                                                              tee_local $l3
                                                                              i32.eq
                                                                              br_if $B34
                                                                              get_local $l2
                                                                              get_local $l3
                                                                              i32.store offset=12
                                                                              get_local $l3
                                                                              i32.const 8
                                                                              i32.add
                                                                              get_local $l2
                                                                              i32.store
                                                                              br $B33
                                                                            end
                                                                            i32.const 0
                                                                            set_local $l2
                                                                            get_local $p0
                                                                            i32.const -64
                                                                            i32.ge_u
                                                                            br_if $B7
                                                                            get_local $p0
                                                                            i32.const 11
                                                                            i32.add
                                                                            tee_local $p0
                                                                            i32.const -8
                                                                            i32.and
                                                                            set_local $l1
                                                                            i32.const 0
                                                                            i32.load offset=1048
                                                                            tee_local $l5
                                                                            i32.eqz
                                                                            br_if $B26
                                                                            i32.const 0
                                                                            set_local $l6
                                                                            block $B37
                                                                              get_local $p0
                                                                              i32.const 8
                                                                              i32.shr_u
                                                                              tee_local $p0
                                                                              i32.eqz
                                                                              br_if $B37
                                                                              i32.const 31
                                                                              set_local $l6
                                                                              get_local $l1
                                                                              i32.const 16777215
                                                                              i32.gt_u
                                                                              br_if $B37
                                                                              get_local $l1
                                                                              i32.const 38
                                                                              get_local $p0
                                                                              i32.clz
                                                                              tee_local $p0
                                                                              i32.sub
                                                                              i32.const 31
                                                                              i32.and
                                                                              i32.shr_u
                                                                              i32.const 1
                                                                              i32.and
                                                                              i32.const 31
                                                                              get_local $p0
                                                                              i32.sub
                                                                              i32.const 1
                                                                              i32.shl
                                                                              i32.or
                                                                              set_local $l6
                                                                            end
                                                                            i32.const 0
                                                                            get_local $l1
                                                                            i32.sub
                                                                            set_local $l2
                                                                            get_local $l6
                                                                            i32.const 2
                                                                            i32.shl
                                                                            i32.const 1316
                                                                            i32.add
                                                                            i32.load
                                                                            tee_local $p0
                                                                            i32.eqz
                                                                            br_if $B29
                                                                            i32.const 0
                                                                            set_local $l3
                                                                            get_local $l1
                                                                            i32.const 0
                                                                            i32.const 25
                                                                            get_local $l6
                                                                            i32.const 1
                                                                            i32.shr_u
                                                                            i32.sub
                                                                            i32.const 31
                                                                            i32.and
                                                                            get_local $l6
                                                                            i32.const 31
                                                                            i32.eq
                                                                            select
                                                                            i32.shl
                                                                            set_local $l0
                                                                            i32.const 0
                                                                            set_local $l4
                                                                            loop $L38
                                                                              block $B39
                                                                                get_local $p0
                                                                                i32.load offset=4
                                                                                i32.const -8
                                                                                i32.and
                                                                                tee_local $l7
                                                                                get_local $l1
                                                                                i32.lt_u
                                                                                br_if $B39
                                                                                get_local $l7
                                                                                get_local $l1
                                                                                i32.sub
                                                                                tee_local $l7
                                                                                get_local $l2
                                                                                i32.ge_u
                                                                                br_if $B39
                                                                                get_local $l7
                                                                                set_local $l2
                                                                                get_local $p0
                                                                                set_local $l4
                                                                                get_local $l7
                                                                                i32.eqz
                                                                                br_if $B31
                                                                              end
                                                                              get_local $p0
                                                                              i32.const 20
                                                                              i32.add
                                                                              i32.load
                                                                              tee_local $l7
                                                                              get_local $l3
                                                                              get_local $l7
                                                                              get_local $p0
                                                                              get_local $l0
                                                                              i32.const 29
                                                                              i32.shr_u
                                                                              i32.const 4
                                                                              i32.and
                                                                              i32.add
                                                                              i32.const 16
                                                                              i32.add
                                                                              i32.load
                                                                              tee_local $p0
                                                                              i32.ne
                                                                              select
                                                                              get_local $l3
                                                                              get_local $l7
                                                                              select
                                                                              set_local $l3
                                                                              get_local $l0
                                                                              i32.const 1
                                                                              i32.shl
                                                                              set_local $l0
                                                                              get_local $p0
                                                                              br_if $L38
                                                                            end
                                                                            get_local $l3
                                                                            i32.eqz
                                                                            br_if $B30
                                                                            get_local $l3
                                                                            set_local $p0
                                                                            br $B28
                                                                          end
                                                                          get_local $l1
                                                                          i32.const 0
                                                                          i32.load offset=1444
                                                                          i32.le_u
                                                                          br_if $B26
                                                                          get_local $p0
                                                                          i32.eqz
                                                                          br_if $B32
                                                                          get_local $p0
                                                                          get_local $l3
                                                                          i32.shl
                                                                          i32.const 2
                                                                          get_local $l3
                                                                          i32.shl
                                                                          tee_local $p0
                                                                          i32.const 0
                                                                          get_local $p0
                                                                          i32.sub
                                                                          i32.or
                                                                          i32.and
                                                                          tee_local $p0
                                                                          i32.const 0
                                                                          get_local $p0
                                                                          i32.sub
                                                                          i32.and
                                                                          i32.ctz
                                                                          tee_local $l2
                                                                          i32.const 3
                                                                          i32.shl
                                                                          tee_local $l4
                                                                          i32.const 1060
                                                                          i32.add
                                                                          i32.load
                                                                          tee_local $p0
                                                                          i32.load offset=8
                                                                          tee_local $l3
                                                                          get_local $l4
                                                                          i32.const 1052
                                                                          i32.add
                                                                          tee_local $l4
                                                                          i32.eq
                                                                          br_if $B24
                                                                          get_local $l3
                                                                          get_local $l4
                                                                          i32.store offset=12
                                                                          get_local $l4
                                                                          i32.const 8
                                                                          i32.add
                                                                          get_local $l3
                                                                          i32.store
                                                                          br $B23
                                                                        end
                                                                        i32.const 0
                                                                        get_local $l0
                                                                        i32.const -2
                                                                        get_local $l1
                                                                        i32.rotl
                                                                        i32.and
                                                                        i32.store offset=1044
                                                                      end
                                                                      get_local $p0
                                                                      get_local $l1
                                                                      i32.const 3
                                                                      i32.shl
                                                                      tee_local $l1
                                                                      i32.const 3
                                                                      i32.or
                                                                      i32.store offset=4
                                                                      get_local $p0
                                                                      get_local $l1
                                                                      i32.add
                                                                      tee_local $p0
                                                                      get_local $p0
                                                                      i32.load offset=4
                                                                      i32.const 1
                                                                      i32.or
                                                                      i32.store offset=4
                                                                      get_local $l4
                                                                      return
                                                                    end
                                                                    i32.const 0
                                                                    i32.load offset=1048
                                                                    tee_local $p0
                                                                    i32.eqz
                                                                    br_if $B26
                                                                    get_local $p0
                                                                    i32.const 0
                                                                    get_local $p0
                                                                    i32.sub
                                                                    i32.and
                                                                    i32.ctz
                                                                    i32.const 2
                                                                    i32.shl
                                                                    i32.const 1316
                                                                    i32.add
                                                                    i32.load
                                                                    tee_local $l0
                                                                    i32.load offset=4
                                                                    i32.const -8
                                                                    i32.and
                                                                    get_local $l1
                                                                    i32.sub
                                                                    set_local $l2
                                                                    get_local $l0
                                                                    set_local $l3
                                                                    get_local $l0
                                                                    i32.load offset=16
                                                                    tee_local $p0
                                                                    i32.eqz
                                                                    br_if $B11
                                                                    i32.const 0
                                                                    set_local $l8
                                                                    br $B10
                                                                  end
                                                                  i32.const 0
                                                                  set_local $l2
                                                                  get_local $p0
                                                                  set_local $l4
                                                                  br $B28
                                                                end
                                                                get_local $l4
                                                                br_if $B27
                                                              end
                                                              i32.const 0
                                                              set_local $l4
                                                              i32.const 2
                                                              get_local $l6
                                                              i32.const 31
                                                              i32.and
                                                              i32.shl
                                                              tee_local $p0
                                                              i32.const 0
                                                              get_local $p0
                                                              i32.sub
                                                              i32.or
                                                              get_local $l5
                                                              i32.and
                                                              tee_local $p0
                                                              i32.eqz
                                                              br_if $B26
                                                              get_local $p0
                                                              i32.const 0
                                                              get_local $p0
                                                              i32.sub
                                                              i32.and
                                                              i32.ctz
                                                              i32.const 2
                                                              i32.shl
                                                              i32.const 1316
                                                              i32.add
                                                              i32.load
                                                              tee_local $p0
                                                              i32.eqz
                                                              br_if $B26
                                                            end
                                                            loop $L40
                                                              get_local $p0
                                                              i32.load offset=4
                                                              i32.const -8
                                                              i32.and
                                                              tee_local $l3
                                                              get_local $l1
                                                              i32.ge_u
                                                              get_local $l3
                                                              get_local $l1
                                                              i32.sub
                                                              tee_local $l7
                                                              get_local $l2
                                                              i32.lt_u
                                                              i32.and
                                                              set_local $l0
                                                              block $B41
                                                                get_local $p0
                                                                i32.load offset=16
                                                                tee_local $l3
                                                                br_if $B41
                                                                get_local $p0
                                                                i32.const 20
                                                                i32.add
                                                                i32.load
                                                                set_local $l3
                                                              end
                                                              get_local $p0
                                                              get_local $l4
                                                              get_local $l0
                                                              select
                                                              set_local $l4
                                                              get_local $l7
                                                              get_local $l2
                                                              get_local $l0
                                                              select
                                                              set_local $l2
                                                              get_local $l3
                                                              set_local $p0
                                                              get_local $l3
                                                              br_if $L40
                                                            end
                                                            get_local $l4
                                                            i32.eqz
                                                            br_if $B26
                                                          end
                                                          i32.const 0
                                                          i32.load offset=1444
                                                          tee_local $p0
                                                          get_local $l1
                                                          i32.lt_u
                                                          br_if $B25
                                                          get_local $l2
                                                          get_local $p0
                                                          get_local $l1
                                                          i32.sub
                                                          i32.lt_u
                                                          br_if $B25
                                                        end
                                                        block $B42
                                                          block $B43
                                                            block $B44
                                                              block $B45
                                                                i32.const 0
                                                                i32.load offset=1444
                                                                tee_local $l2
                                                                get_local $l1
                                                                i32.ge_u
                                                                br_if $B45
                                                                i32.const 0
                                                                i32.load offset=1448
                                                                tee_local $p0
                                                                get_local $l1
                                                                i32.le_u
                                                                br_if $B44
                                                                i32.const 0
                                                                get_local $p0
                                                                get_local $l1
                                                                i32.sub
                                                                tee_local $l2
                                                                i32.store offset=1448
                                                                i32.const 0
                                                                i32.const 0
                                                                i32.load offset=1456
                                                                tee_local $p0
                                                                get_local $l1
                                                                i32.add
                                                                tee_local $l3
                                                                i32.store offset=1456
                                                                get_local $l3
                                                                get_local $l2
                                                                i32.const 1
                                                                i32.or
                                                                i32.store offset=4
                                                                get_local $p0
                                                                get_local $l1
                                                                i32.const 3
                                                                i32.or
                                                                i32.store offset=4
                                                                get_local $p0
                                                                i32.const 8
                                                                i32.add
                                                                return
                                                              end
                                                              i32.const 0
                                                              i32.load offset=1452
                                                              set_local $p0
                                                              get_local $l2
                                                              get_local $l1
                                                              i32.sub
                                                              tee_local $l3
                                                              i32.const 16
                                                              i32.ge_u
                                                              br_if $B43
                                                              i32.const 0
                                                              i32.const 0
                                                              i32.store offset=1452
                                                              i32.const 0
                                                              i32.const 0
                                                              i32.store offset=1444
                                                              get_local $p0
                                                              get_local $l2
                                                              i32.const 3
                                                              i32.or
                                                              i32.store offset=4
                                                              get_local $p0
                                                              get_local $l2
                                                              i32.add
                                                              tee_local $l2
                                                              i32.const 4
                                                              i32.add
                                                              set_local $l1
                                                              get_local $l2
                                                              i32.load offset=4
                                                              i32.const 1
                                                              i32.or
                                                              set_local $l2
                                                              br $B42
                                                            end
                                                            i32.const 0
                                                            set_local $l2
                                                            get_local $l1
                                                            i32.const 65583
                                                            i32.add
                                                            tee_local $l3
                                                            i32.const 16
                                                            i32.shr_u
                                                            grow_memory
                                                            tee_local $p0
                                                            i32.const -1
                                                            i32.eq
                                                            br_if $B7
                                                            get_local $p0
                                                            i32.const 16
                                                            i32.shl
                                                            tee_local $l0
                                                            i32.eqz
                                                            br_if $B7
                                                            i32.const 0
                                                            i32.const 0
                                                            i32.load offset=1460
                                                            get_local $l3
                                                            i32.const -65536
                                                            i32.and
                                                            tee_local $l7
                                                            i32.add
                                                            tee_local $p0
                                                            i32.store offset=1460
                                                            i32.const 0
                                                            i32.const 0
                                                            i32.load offset=1464
                                                            tee_local $l2
                                                            get_local $p0
                                                            get_local $p0
                                                            get_local $l2
                                                            i32.lt_u
                                                            select
                                                            i32.store offset=1464
                                                            i32.const 0
                                                            i32.load offset=1456
                                                            tee_local $l2
                                                            i32.eqz
                                                            br_if $B18
                                                            i32.const 1468
                                                            set_local $p0
                                                            loop $L46
                                                              get_local $p0
                                                              i32.load
                                                              tee_local $l3
                                                              get_local $p0
                                                              i32.load offset=4
                                                              tee_local $l4
                                                              i32.add
                                                              get_local $l0
                                                              i32.eq
                                                              br_if $B17
                                                              get_local $p0
                                                              i32.load offset=8
                                                              tee_local $p0
                                                              br_if $L46
                                                              br $B9
                                                            end
                                                          end
                                                          i32.const 0
                                                          get_local $l3
                                                          i32.store offset=1444
                                                          i32.const 0
                                                          get_local $p0
                                                          get_local $l1
                                                          i32.add
                                                          tee_local $l0
                                                          i32.store offset=1452
                                                          get_local $l0
                                                          get_local $l3
                                                          i32.const 1
                                                          i32.or
                                                          i32.store offset=4
                                                          get_local $p0
                                                          get_local $l2
                                                          i32.add
                                                          get_local $l3
                                                          i32.store
                                                          get_local $l1
                                                          i32.const 3
                                                          i32.or
                                                          set_local $l2
                                                          get_local $p0
                                                          i32.const 4
                                                          i32.add
                                                          set_local $l1
                                                        end
                                                        get_local $l1
                                                        get_local $l2
                                                        i32.store
                                                        get_local $p0
                                                        i32.const 8
                                                        i32.add
                                                        return
                                                      end
                                                      get_local $l4
                                                      call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                                                      get_local $l2
                                                      i32.const 15
                                                      i32.gt_u
                                                      br_if $B22
                                                      get_local $l4
                                                      get_local $l2
                                                      get_local $l1
                                                      i32.add
                                                      tee_local $p0
                                                      i32.const 3
                                                      i32.or
                                                      i32.store offset=4
                                                      get_local $l4
                                                      get_local $p0
                                                      i32.add
                                                      tee_local $p0
                                                      get_local $p0
                                                      i32.load offset=4
                                                      i32.const 1
                                                      i32.or
                                                      i32.store offset=4
                                                      br $B12
                                                    end
                                                    i32.const 0
                                                    get_local $l0
                                                    i32.const -2
                                                    get_local $l2
                                                    i32.rotl
                                                    i32.and
                                                    i32.store offset=1044
                                                  end
                                                  get_local $p0
                                                  i32.const 8
                                                  i32.add
                                                  set_local $l3
                                                  get_local $p0
                                                  get_local $l1
                                                  i32.const 3
                                                  i32.or
                                                  i32.store offset=4
                                                  get_local $p0
                                                  get_local $l1
                                                  i32.add
                                                  tee_local $l0
                                                  get_local $l2
                                                  i32.const 3
                                                  i32.shl
                                                  tee_local $l2
                                                  get_local $l1
                                                  i32.sub
                                                  tee_local $l1
                                                  i32.const 1
                                                  i32.or
                                                  i32.store offset=4
                                                  get_local $p0
                                                  get_local $l2
                                                  i32.add
                                                  get_local $l1
                                                  i32.store
                                                  i32.const 0
                                                  i32.load offset=1444
                                                  tee_local $p0
                                                  i32.eqz
                                                  br_if $B19
                                                  get_local $p0
                                                  i32.const 3
                                                  i32.shr_u
                                                  tee_local $l4
                                                  i32.const 3
                                                  i32.shl
                                                  i32.const 1052
                                                  i32.add
                                                  set_local $l2
                                                  i32.const 0
                                                  i32.load offset=1452
                                                  set_local $p0
                                                  i32.const 0
                                                  i32.load offset=1044
                                                  tee_local $l7
                                                  i32.const 1
                                                  get_local $l4
                                                  i32.const 31
                                                  i32.and
                                                  i32.shl
                                                  tee_local $l4
                                                  i32.and
                                                  i32.eqz
                                                  br_if $B21
                                                  get_local $l2
                                                  i32.load offset=8
                                                  set_local $l4
                                                  br $B20
                                                end
                                                get_local $l4
                                                get_local $l1
                                                i32.const 3
                                                i32.or
                                                i32.store offset=4
                                                get_local $l4
                                                get_local $l1
                                                i32.add
                                                tee_local $p0
                                                get_local $l2
                                                i32.const 1
                                                i32.or
                                                i32.store offset=4
                                                get_local $p0
                                                get_local $l2
                                                i32.add
                                                get_local $l2
                                                i32.store
                                                get_local $l2
                                                i32.const 255
                                                i32.gt_u
                                                br_if $B16
                                                get_local $l2
                                                i32.const 3
                                                i32.shr_u
                                                tee_local $l2
                                                i32.const 3
                                                i32.shl
                                                i32.const 1052
                                                i32.add
                                                set_local $l1
                                                i32.const 0
                                                i32.load offset=1044
                                                tee_local $l3
                                                i32.const 1
                                                get_local $l2
                                                i32.const 31
                                                i32.and
                                                i32.shl
                                                tee_local $l2
                                                i32.and
                                                i32.eqz
                                                br_if $B14
                                                get_local $l1
                                                i32.const 8
                                                i32.add
                                                set_local $l3
                                                get_local $l1
                                                i32.load offset=8
                                                set_local $l2
                                                br $B13
                                              end
                                              i32.const 0
                                              get_local $l7
                                              get_local $l4
                                              i32.or
                                              i32.store offset=1044
                                              get_local $l2
                                              set_local $l4
                                            end
                                            get_local $l2
                                            i32.const 8
                                            i32.add
                                            get_local $p0
                                            i32.store
                                            get_local $l4
                                            get_local $p0
                                            i32.store offset=12
                                            get_local $p0
                                            get_local $l2
                                            i32.store offset=12
                                            get_local $p0
                                            get_local $l4
                                            i32.store offset=8
                                          end
                                          i32.const 0
                                          get_local $l0
                                          i32.store offset=1452
                                          i32.const 0
                                          get_local $l1
                                          i32.store offset=1444
                                          get_local $l3
                                          return
                                        end
                                        block $B47
                                          block $B48
                                            i32.const 0
                                            i32.load offset=1488
                                            tee_local $p0
                                            i32.eqz
                                            br_if $B48
                                            get_local $p0
                                            get_local $l0
                                            i32.le_u
                                            br_if $B47
                                          end
                                          i32.const 0
                                          get_local $l0
                                          i32.store offset=1488
                                        end
                                        i32.const 0
                                        set_local $p0
                                        i32.const 0
                                        get_local $l7
                                        i32.store offset=1472
                                        i32.const 0
                                        get_local $l0
                                        i32.store offset=1468
                                        i32.const 0
                                        i32.const 4095
                                        i32.store offset=1492
                                        i32.const 0
                                        i32.const 0
                                        i32.store offset=1480
                                        loop $L49
                                          get_local $p0
                                          i32.const 1060
                                          i32.add
                                          get_local $p0
                                          i32.const 1052
                                          i32.add
                                          tee_local $l2
                                          i32.store
                                          get_local $p0
                                          i32.const 1064
                                          i32.add
                                          get_local $l2
                                          i32.store
                                          get_local $p0
                                          i32.const 8
                                          i32.add
                                          tee_local $p0
                                          i32.const 256
                                          i32.ne
                                          br_if $L49
                                        end
                                        get_local $l0
                                        get_local $l7
                                        i32.const -40
                                        i32.add
                                        tee_local $p0
                                        i32.const 1
                                        i32.or
                                        i32.store offset=4
                                        i32.const 0
                                        get_local $l0
                                        i32.store offset=1456
                                        i32.const 0
                                        i32.const 2097152
                                        i32.store offset=1484
                                        i32.const 0
                                        get_local $p0
                                        i32.store offset=1448
                                        get_local $l0
                                        get_local $p0
                                        i32.add
                                        i32.const 40
                                        i32.store offset=4
                                        br $B8
                                      end
                                      get_local $p0
                                      i32.load offset=12
                                      i32.eqz
                                      br_if $B15
                                      br $B9
                                    end
                                    get_local $p0
                                    get_local $l2
                                    call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
                                    br $B12
                                  end
                                  get_local $l0
                                  get_local $l2
                                  i32.le_u
                                  br_if $B9
                                  get_local $l3
                                  get_local $l2
                                  i32.gt_u
                                  br_if $B9
                                  get_local $p0
                                  i32.const 4
                                  i32.add
                                  get_local $l4
                                  get_local $l7
                                  i32.add
                                  i32.store
                                  i32.const 0
                                  i32.load offset=1456
                                  tee_local $p0
                                  i32.const 15
                                  i32.add
                                  i32.const -8
                                  i32.and
                                  tee_local $l2
                                  i32.const -8
                                  i32.add
                                  tee_local $l3
                                  i32.const 0
                                  i32.load offset=1448
                                  get_local $l7
                                  i32.add
                                  tee_local $l0
                                  get_local $l2
                                  get_local $p0
                                  i32.const 8
                                  i32.add
                                  i32.sub
                                  i32.sub
                                  tee_local $l2
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  i32.const 0
                                  i32.const 2097152
                                  i32.store offset=1484
                                  i32.const 0
                                  get_local $l3
                                  i32.store offset=1456
                                  i32.const 0
                                  get_local $l2
                                  i32.store offset=1448
                                  get_local $p0
                                  get_local $l0
                                  i32.add
                                  i32.const 40
                                  i32.store offset=4
                                  br $B8
                                end
                                i32.const 0
                                get_local $l3
                                get_local $l2
                                i32.or
                                i32.store offset=1044
                                get_local $l1
                                i32.const 8
                                i32.add
                                set_local $l3
                                get_local $l1
                                set_local $l2
                              end
                              get_local $l3
                              get_local $p0
                              i32.store
                              get_local $l2
                              get_local $p0
                              i32.store offset=12
                              get_local $p0
                              get_local $l1
                              i32.store offset=12
                              get_local $p0
                              get_local $l2
                              i32.store offset=8
                            end
                            get_local $l4
                            i32.const 8
                            i32.add
                            set_local $l2
                            br $B7
                          end
                          i32.const 1
                          set_local $l8
                        end
                        loop $L50
                          block $B51
                            block $B52
                              block $B53
                                block $B54
                                  block $B55
                                    block $B56
                                      block $B57
                                        block $B58
                                          block $B59
                                            block $B60
                                              block $B61
                                                block $B62
                                                  block $B63
                                                    block $B64
                                                      block $B65
                                                        block $B66
                                                          block $B67
                                                            block $B68
                                                              get_local $l8
                                                              br_table $B68 $B67 $B66 $B64 $B63 $B62 $B60 $B59 $B58 $B61 $B65 $B65
                                                            end
                                                            get_local $p0
                                                            i32.load offset=4
                                                            i32.const -8
                                                            i32.and
                                                            get_local $l1
                                                            i32.sub
                                                            tee_local $l0
                                                            get_local $l2
                                                            get_local $l0
                                                            get_local $l2
                                                            i32.lt_u
                                                            tee_local $l0
                                                            select
                                                            set_local $l2
                                                            get_local $p0
                                                            get_local $l3
                                                            get_local $l0
                                                            select
                                                            set_local $l3
                                                            get_local $p0
                                                            tee_local $l0
                                                            i32.load offset=16
                                                            tee_local $p0
                                                            br_if $B57
                                                            i32.const 1
                                                            set_local $l8
                                                            br $L50
                                                          end
                                                          get_local $l0
                                                          i32.const 20
                                                          i32.add
                                                          i32.load
                                                          tee_local $p0
                                                          br_if $B56
                                                          i32.const 2
                                                          set_local $l8
                                                          br $L50
                                                        end
                                                        get_local $l3
                                                        call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                                                        get_local $l2
                                                        i32.const 16
                                                        i32.ge_u
                                                        br_if $B55
                                                        i32.const 10
                                                        set_local $l8
                                                        br $L50
                                                      end
                                                      get_local $l3
                                                      get_local $l2
                                                      get_local $l1
                                                      i32.add
                                                      tee_local $p0
                                                      i32.const 3
                                                      i32.or
                                                      i32.store offset=4
                                                      get_local $l3
                                                      get_local $p0
                                                      i32.add
                                                      tee_local $p0
                                                      get_local $p0
                                                      i32.load offset=4
                                                      i32.const 1
                                                      i32.or
                                                      i32.store offset=4
                                                      br $B51
                                                    end
                                                    get_local $l3
                                                    get_local $l1
                                                    i32.const 3
                                                    i32.or
                                                    i32.store offset=4
                                                    get_local $l3
                                                    get_local $l1
                                                    i32.add
                                                    tee_local $l1
                                                    get_local $l2
                                                    i32.const 1
                                                    i32.or
                                                    i32.store offset=4
                                                    get_local $l1
                                                    get_local $l2
                                                    i32.add
                                                    get_local $l2
                                                    i32.store
                                                    i32.const 0
                                                    i32.load offset=1444
                                                    tee_local $p0
                                                    i32.eqz
                                                    br_if $B54
                                                    i32.const 4
                                                    set_local $l8
                                                    br $L50
                                                  end
                                                  get_local $p0
                                                  i32.const 3
                                                  i32.shr_u
                                                  tee_local $l4
                                                  i32.const 3
                                                  i32.shl
                                                  i32.const 1052
                                                  i32.add
                                                  set_local $l0
                                                  i32.const 0
                                                  i32.load offset=1452
                                                  set_local $p0
                                                  i32.const 0
                                                  i32.load offset=1044
                                                  tee_local $l7
                                                  i32.const 1
                                                  get_local $l4
                                                  i32.const 31
                                                  i32.and
                                                  i32.shl
                                                  tee_local $l4
                                                  i32.and
                                                  i32.eqz
                                                  br_if $B53
                                                  i32.const 5
                                                  set_local $l8
                                                  br $L50
                                                end
                                                get_local $l0
                                                i32.load offset=8
                                                set_local $l4
                                                br $B52
                                              end
                                              i32.const 0
                                              get_local $l7
                                              get_local $l4
                                              i32.or
                                              i32.store offset=1044
                                              get_local $l0
                                              set_local $l4
                                              i32.const 6
                                              set_local $l8
                                              br $L50
                                            end
                                            get_local $l0
                                            i32.const 8
                                            i32.add
                                            get_local $p0
                                            i32.store
                                            get_local $l4
                                            get_local $p0
                                            i32.store offset=12
                                            get_local $p0
                                            get_local $l0
                                            i32.store offset=12
                                            get_local $p0
                                            get_local $l4
                                            i32.store offset=8
                                            i32.const 7
                                            set_local $l8
                                            br $L50
                                          end
                                          i32.const 0
                                          get_local $l1
                                          i32.store offset=1452
                                          i32.const 0
                                          get_local $l2
                                          i32.store offset=1444
                                          i32.const 8
                                          set_local $l8
                                          br $L50
                                        end
                                        get_local $l3
                                        i32.const 8
                                        i32.add
                                        return
                                      end
                                      i32.const 0
                                      set_local $l8
                                      br $L50
                                    end
                                    i32.const 0
                                    set_local $l8
                                    br $L50
                                  end
                                  i32.const 3
                                  set_local $l8
                                  br $L50
                                end
                                i32.const 7
                                set_local $l8
                                br $L50
                              end
                              i32.const 9
                              set_local $l8
                              br $L50
                            end
                            i32.const 6
                            set_local $l8
                            br $L50
                          end
                          i32.const 8
                          set_local $l8
                          br $L50
                        end
                      end
                      i32.const 0
                      i32.const 0
                      i32.load offset=1488
                      tee_local $p0
                      get_local $l0
                      get_local $p0
                      get_local $l0
                      i32.lt_u
                      select
                      i32.store offset=1488
                      get_local $l0
                      get_local $l7
                      i32.add
                      set_local $l3
                      i32.const 1468
                      set_local $p0
                      block $B69
                        block $B70
                          block $B71
                            block $B72
                              block $B73
                                loop $L74
                                  get_local $p0
                                  i32.load
                                  get_local $l3
                                  i32.eq
                                  br_if $B73
                                  get_local $p0
                                  i32.load offset=8
                                  tee_local $p0
                                  br_if $L74
                                  br $B72
                                end
                              end
                              get_local $p0
                              i32.load offset=12
                              i32.eqz
                              br_if $B71
                            end
                            i32.const 1468
                            set_local $p0
                            block $B75
                              loop $L76
                                block $B77
                                  get_local $p0
                                  i32.load
                                  tee_local $l3
                                  get_local $l2
                                  i32.gt_u
                                  br_if $B77
                                  get_local $l3
                                  get_local $p0
                                  i32.load offset=4
                                  i32.add
                                  tee_local $l3
                                  get_local $l2
                                  i32.gt_u
                                  br_if $B75
                                end
                                get_local $p0
                                i32.load offset=8
                                set_local $p0
                                br $L76
                              end
                            end
                            get_local $l0
                            get_local $l7
                            i32.const -40
                            i32.add
                            tee_local $p0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            get_local $l0
                            get_local $p0
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            get_local $l2
                            get_local $l3
                            i32.const -32
                            i32.add
                            i32.const -8
                            i32.and
                            i32.const -8
                            i32.add
                            tee_local $l4
                            get_local $l4
                            get_local $l2
                            i32.const 16
                            i32.add
                            i32.lt_u
                            select
                            tee_local $l4
                            i32.const 27
                            i32.store offset=4
                            i32.const 0
                            get_local $l0
                            i32.store offset=1456
                            i32.const 0
                            i32.const 2097152
                            i32.store offset=1484
                            i32.const 0
                            get_local $p0
                            i32.store offset=1448
                            i32.const 0
                            i64.load offset=1468 align=4
                            set_local $l9
                            get_local $l4
                            i32.const 16
                            i32.add
                            i32.const 0
                            i64.load offset=1476 align=4
                            i64.store align=4
                            get_local $l4
                            get_local $l9
                            i64.store offset=8 align=4
                            i32.const 0
                            get_local $l7
                            i32.store offset=1472
                            i32.const 0
                            get_local $l0
                            i32.store offset=1468
                            i32.const 0
                            get_local $l4
                            i32.const 8
                            i32.add
                            i32.store offset=1476
                            i32.const 0
                            i32.const 0
                            i32.store offset=1480
                            get_local $l4
                            i32.const 28
                            i32.add
                            set_local $p0
                            loop $L78
                              get_local $p0
                              i32.const 7
                              i32.store
                              get_local $l3
                              get_local $p0
                              i32.const 4
                              i32.add
                              tee_local $p0
                              i32.gt_u
                              br_if $L78
                            end
                            get_local $l4
                            get_local $l2
                            i32.eq
                            br_if $B8
                            get_local $l4
                            get_local $l4
                            i32.load offset=4
                            i32.const -2
                            i32.and
                            i32.store offset=4
                            get_local $l2
                            get_local $l4
                            get_local $l2
                            i32.sub
                            tee_local $p0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            get_local $l4
                            get_local $p0
                            i32.store
                            block $B79
                              get_local $p0
                              i32.const 255
                              i32.gt_u
                              br_if $B79
                              get_local $p0
                              i32.const 3
                              i32.shr_u
                              tee_local $l3
                              i32.const 3
                              i32.shl
                              i32.const 1052
                              i32.add
                              set_local $p0
                              i32.const 0
                              i32.load offset=1044
                              tee_local $l0
                              i32.const 1
                              get_local $l3
                              i32.const 31
                              i32.and
                              i32.shl
                              tee_local $l3
                              i32.and
                              i32.eqz
                              br_if $B70
                              get_local $p0
                              i32.load offset=8
                              set_local $l3
                              br $B69
                            end
                            get_local $l2
                            get_local $p0
                            call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
                            br $B8
                          end
                          get_local $p0
                          get_local $l0
                          i32.store
                          get_local $p0
                          get_local $p0
                          i32.load offset=4
                          get_local $l7
                          i32.add
                          i32.store offset=4
                          get_local $l0
                          get_local $l1
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          get_local $l0
                          get_local $l1
                          i32.add
                          set_local $p0
                          get_local $l3
                          get_local $l0
                          i32.sub
                          get_local $l1
                          i32.sub
                          set_local $l1
                          i32.const 0
                          i32.load offset=1456
                          get_local $l3
                          i32.eq
                          br_if $B6
                          i32.const 0
                          i32.load offset=1452
                          get_local $l3
                          i32.eq
                          br_if $B5
                          get_local $l3
                          i32.load offset=4
                          tee_local $l2
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if $B1
                          get_local $l2
                          i32.const -8
                          i32.and
                          tee_local $l4
                          i32.const 255
                          i32.gt_u
                          br_if $B4
                          get_local $l3
                          i32.load offset=12
                          tee_local $l7
                          get_local $l3
                          i32.load offset=8
                          tee_local $l6
                          i32.eq
                          br_if $B3
                          get_local $l6
                          get_local $l7
                          i32.store offset=12
                          get_local $l7
                          get_local $l6
                          i32.store offset=8
                          br $B2
                        end
                        i32.const 0
                        get_local $l0
                        get_local $l3
                        i32.or
                        i32.store offset=1044
                        get_local $p0
                        set_local $l3
                      end
                      get_local $p0
                      i32.const 8
                      i32.add
                      get_local $l2
                      i32.store
                      get_local $l3
                      get_local $l2
                      i32.store offset=12
                      get_local $l2
                      get_local $p0
                      i32.store offset=12
                      get_local $l2
                      get_local $l3
                      i32.store offset=8
                    end
                    i32.const 0
                    set_local $l2
                    i32.const 0
                    i32.load offset=1448
                    tee_local $p0
                    get_local $l1
                    i32.le_u
                    br_if $B7
                    i32.const 0
                    get_local $p0
                    get_local $l1
                    i32.sub
                    tee_local $l2
                    i32.store offset=1448
                    i32.const 0
                    i32.const 0
                    i32.load offset=1456
                    tee_local $p0
                    get_local $l1
                    i32.add
                    tee_local $l3
                    i32.store offset=1456
                    get_local $l3
                    get_local $l2
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    get_local $p0
                    get_local $l1
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    get_local $p0
                    i32.const 8
                    i32.add
                    return
                  end
                  get_local $l2
                  return
                end
                i32.const 0
                get_local $p0
                i32.store offset=1456
                i32.const 0
                i32.const 0
                i32.load offset=1448
                get_local $l1
                i32.add
                tee_local $l1
                i32.store offset=1448
                get_local $p0
                get_local $l1
                i32.const 1
                i32.or
                i32.store offset=4
                br $B0
              end
              get_local $p0
              i32.const 0
              i32.load offset=1444
              get_local $l1
              i32.add
              tee_local $l1
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              get_local $p0
              i32.store offset=1452
              i32.const 0
              get_local $l1
              i32.store offset=1444
              get_local $p0
              get_local $l1
              i32.add
              get_local $l1
              i32.store
              br $B0
            end
            get_local $l3
            call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
            br $B2
          end
          i32.const 0
          i32.const 0
          i32.load offset=1044
          i32.const -2
          get_local $l2
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store offset=1044
        end
        get_local $l4
        get_local $l1
        i32.add
        set_local $l1
        get_local $l3
        get_local $l4
        i32.add
        set_local $l3
      end
      get_local $l3
      get_local $l3
      i32.load offset=4
      i32.const -2
      i32.and
      i32.store offset=4
      get_local $p0
      get_local $l1
      i32.const 1
      i32.or
      i32.store offset=4
      get_local $p0
      get_local $l1
      i32.add
      get_local $l1
      i32.store
      block $B80
        block $B81
          block $B82
            get_local $l1
            i32.const 255
            i32.gt_u
            br_if $B82
            get_local $l1
            i32.const 3
            i32.shr_u
            tee_local $l2
            i32.const 3
            i32.shl
            i32.const 1052
            i32.add
            set_local $l1
            i32.const 0
            i32.load offset=1044
            tee_local $l3
            i32.const 1
            get_local $l2
            i32.const 31
            i32.and
            i32.shl
            tee_local $l2
            i32.and
            i32.eqz
            br_if $B81
            get_local $l1
            i32.const 8
            i32.add
            set_local $l3
            get_local $l1
            i32.load offset=8
            set_local $l2
            br $B80
          end
          get_local $p0
          get_local $l1
          call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
          br $B0
        end
        i32.const 0
        get_local $l3
        get_local $l2
        i32.or
        i32.store offset=1044
        get_local $l1
        i32.const 8
        i32.add
        set_local $l3
        get_local $l1
        set_local $l2
      end
      get_local $l3
      get_local $p0
      i32.store
      get_local $l2
      get_local $p0
      i32.store offset=12
      get_local $p0
      get_local $l1
      i32.store offset=12
      get_local $p0
      get_local $l2
      i32.store offset=8
    end
    get_local $l0
    i32.const 8
    i32.add)
  (func $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    get_local $p0
    i32.load offset=24
    set_local $l0
    block $B0
      block $B1
        block $B2
          block $B3
            get_local $p0
            i32.load offset=12
            tee_local $l1
            get_local $p0
            i32.eq
            br_if $B3
            get_local $p0
            i32.load offset=8
            tee_local $l2
            get_local $l1
            i32.store offset=12
            get_local $l1
            get_local $l2
            i32.store offset=8
            get_local $l0
            br_if $B2
            br $B1
          end
          block $B4
            get_local $p0
            i32.const 20
            i32.add
            tee_local $l2
            get_local $p0
            i32.const 16
            i32.add
            get_local $l2
            i32.load
            select
            tee_local $l3
            i32.load
            tee_local $l2
            i32.eqz
            br_if $B4
            block $B5
              loop $L6
                get_local $l3
                set_local $l4
                block $B7
                  get_local $l2
                  tee_local $l1
                  i32.const 20
                  i32.add
                  tee_local $l3
                  i32.load
                  tee_local $l2
                  i32.eqz
                  br_if $B7
                  get_local $l2
                  br_if $L6
                  br $B5
                end
                get_local $l1
                i32.const 16
                i32.add
                set_local $l3
                get_local $l1
                i32.load offset=16
                tee_local $l2
                br_if $L6
              end
            end
            get_local $l4
            i32.const 0
            i32.store
            get_local $l0
            br_if $B2
            br $B1
          end
          i32.const 0
          set_local $l1
          get_local $l0
          i32.eqz
          br_if $B1
        end
        block $B8
          block $B9
            get_local $p0
            i32.load offset=28
            tee_local $l3
            i32.const 2
            i32.shl
            i32.const 1316
            i32.add
            tee_local $l2
            i32.load
            get_local $p0
            i32.eq
            br_if $B9
            get_local $l0
            i32.const 16
            i32.add
            get_local $l0
            i32.const 20
            i32.add
            get_local $l0
            i32.load offset=16
            get_local $p0
            i32.eq
            select
            get_local $l1
            i32.store
            get_local $l1
            br_if $B8
            br $B1
          end
          get_local $l2
          get_local $l1
          i32.store
          get_local $l1
          i32.eqz
          br_if $B0
        end
        get_local $l1
        get_local $l0
        i32.store offset=24
        block $B10
          get_local $p0
          i32.load offset=16
          tee_local $l2
          i32.eqz
          br_if $B10
          get_local $l1
          get_local $l2
          i32.store offset=16
          get_local $l2
          get_local $l1
          i32.store offset=24
        end
        get_local $p0
        i32.const 20
        i32.add
        i32.load
        tee_local $l2
        i32.eqz
        br_if $B1
        get_local $l1
        i32.const 20
        i32.add
        get_local $l2
        i32.store
        get_local $l2
        get_local $l1
        i32.store offset=24
      end
      return
    end
    i32.const 0
    i32.const 0
    i32.load offset=1048
    i32.const -2
    get_local $l3
    i32.rotl
    i32.and
    i32.store offset=1048)
  (func $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad (type $t3) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    i32.const 0
    set_local $l0
    block $B0
      get_local $p1
      i32.const 8
      i32.shr_u
      tee_local $l1
      i32.eqz
      br_if $B0
      i32.const 31
      set_local $l0
      get_local $p1
      i32.const 16777215
      i32.gt_u
      br_if $B0
      get_local $p1
      i32.const 38
      get_local $l1
      i32.clz
      tee_local $l0
      i32.sub
      i32.const 31
      i32.and
      i32.shr_u
      i32.const 1
      i32.and
      i32.const 31
      get_local $l0
      i32.sub
      i32.const 1
      i32.shl
      i32.or
      set_local $l0
    end
    get_local $p0
    get_local $l0
    i32.store offset=28
    get_local $p0
    i64.const 0
    i64.store offset=16 align=4
    get_local $l0
    i32.const 2
    i32.shl
    i32.const 1316
    i32.add
    set_local $l1
    block $B1
      block $B2
        block $B3
          block $B4
            i32.const 0
            i32.load offset=1048
            tee_local $l2
            i32.const 1
            get_local $l0
            i32.const 31
            i32.and
            i32.shl
            tee_local $l3
            i32.and
            i32.eqz
            br_if $B4
            get_local $l1
            i32.load
            tee_local $l2
            i32.load offset=4
            i32.const -8
            i32.and
            get_local $p1
            i32.ne
            br_if $B3
            get_local $l2
            set_local $l0
            br $B2
          end
          get_local $l1
          get_local $p0
          i32.store
          i32.const 0
          get_local $l2
          get_local $l3
          i32.or
          i32.store offset=1048
          get_local $p0
          get_local $l1
          i32.store offset=24
          get_local $p0
          get_local $p0
          i32.store offset=8
          get_local $p0
          get_local $p0
          i32.store offset=12
          return
        end
        get_local $p1
        i32.const 0
        i32.const 25
        get_local $l0
        i32.const 1
        i32.shr_u
        i32.sub
        i32.const 31
        i32.and
        get_local $l0
        i32.const 31
        i32.eq
        select
        i32.shl
        set_local $l1
        loop $L5
          get_local $l2
          get_local $l1
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          tee_local $l3
          i32.load
          tee_local $l0
          i32.eqz
          br_if $B1
          get_local $l1
          i32.const 1
          i32.shl
          set_local $l1
          get_local $l0
          set_local $l2
          get_local $l0
          i32.load offset=4
          i32.const -8
          i32.and
          get_local $p1
          i32.ne
          br_if $L5
        end
      end
      get_local $l0
      i32.load offset=8
      tee_local $l1
      get_local $p0
      i32.store offset=12
      get_local $l0
      get_local $p0
      i32.store offset=8
      get_local $p0
      get_local $l0
      i32.store offset=12
      get_local $p0
      get_local $l1
      i32.store offset=8
      get_local $p0
      i32.const 0
      i32.store offset=24
      return
    end
    get_local $l3
    get_local $p0
    i32.store
    get_local $p0
    get_local $l2
    i32.store offset=24
    get_local $p0
    get_local $p0
    i32.store offset=12
    get_local $p0
    get_local $p0
    i32.store offset=8)
  (func $dlmalloc::dlmalloc::Dlmalloc::free::h4c32f8306a59a4b8 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    get_local $p0
    i32.const -8
    i32.add
    tee_local $l0
    get_local $p0
    i32.const -4
    i32.add
    i32.load
    tee_local $l1
    i32.const -8
    i32.and
    tee_local $p0
    i32.add
    set_local $l2
    block $B0
      block $B1
        get_local $l1
        i32.const 1
        i32.and
        br_if $B1
        get_local $l1
        i32.const 3
        i32.and
        i32.eqz
        br_if $B0
        get_local $l0
        i32.load
        tee_local $l1
        get_local $p0
        i32.add
        set_local $p0
        block $B2
          block $B3
            block $B4
              i32.const 0
              i32.load offset=1452
              get_local $l0
              get_local $l1
              i32.sub
              tee_local $l0
              i32.eq
              br_if $B4
              get_local $l1
              i32.const 255
              i32.gt_u
              br_if $B3
              get_local $l0
              i32.load offset=12
              tee_local $l3
              get_local $l0
              i32.load offset=8
              tee_local $l4
              i32.eq
              br_if $B2
              get_local $l4
              get_local $l3
              i32.store offset=12
              get_local $l3
              get_local $l4
              i32.store offset=8
              br $B1
            end
            get_local $l2
            i32.load offset=4
            tee_local $l1
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if $B1
            i32.const 0
            get_local $p0
            i32.store offset=1444
            get_local $l2
            i32.const 4
            i32.add
            get_local $l1
            i32.const -2
            i32.and
            i32.store
            get_local $l0
            get_local $p0
            i32.const 1
            i32.or
            i32.store offset=4
            get_local $l0
            get_local $p0
            i32.add
            get_local $p0
            i32.store
            return
          end
          get_local $l0
          call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
          br $B1
        end
        i32.const 0
        i32.const 0
        i32.load offset=1044
        i32.const -2
        get_local $l1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store offset=1044
      end
      block $B5
        block $B6
          block $B7
            block $B8
              block $B9
                block $B10
                  block $B11
                    block $B12
                      block $B13
                        get_local $l2
                        i32.load offset=4
                        tee_local $l1
                        i32.const 2
                        i32.and
                        br_if $B13
                        i32.const 0
                        i32.load offset=1456
                        get_local $l2
                        i32.eq
                        br_if $B12
                        i32.const 0
                        i32.load offset=1452
                        get_local $l2
                        i32.eq
                        br_if $B11
                        get_local $l1
                        i32.const -8
                        i32.and
                        tee_local $l3
                        get_local $p0
                        i32.add
                        set_local $p0
                        get_local $l3
                        i32.const 255
                        i32.gt_u
                        br_if $B10
                        get_local $l2
                        i32.load offset=12
                        tee_local $l3
                        get_local $l2
                        i32.load offset=8
                        tee_local $l2
                        i32.eq
                        br_if $B9
                        get_local $l2
                        get_local $l3
                        i32.store offset=12
                        get_local $l3
                        get_local $l2
                        i32.store offset=8
                        br $B8
                      end
                      get_local $l2
                      i32.const 4
                      i32.add
                      get_local $l1
                      i32.const -2
                      i32.and
                      i32.store
                      get_local $l0
                      get_local $p0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      get_local $l0
                      get_local $p0
                      i32.add
                      get_local $p0
                      i32.store
                      br $B5
                    end
                    i32.const 0
                    get_local $l0
                    i32.store offset=1456
                    i32.const 0
                    i32.const 0
                    i32.load offset=1448
                    get_local $p0
                    i32.add
                    tee_local $p0
                    i32.store offset=1448
                    get_local $l0
                    get_local $p0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block $B14
                      get_local $l0
                      i32.const 0
                      i32.load offset=1452
                      i32.ne
                      br_if $B14
                      i32.const 0
                      i32.const 0
                      i32.store offset=1444
                      i32.const 0
                      i32.const 0
                      i32.store offset=1452
                    end
                    i32.const 0
                    i32.load offset=1484
                    get_local $p0
                    i32.ge_u
                    br_if $B0
                    block $B15
                      get_local $p0
                      i32.const 41
                      i32.lt_u
                      br_if $B15
                      i32.const 1468
                      set_local $p0
                      loop $L16
                        block $B17
                          get_local $p0
                          i32.load
                          tee_local $l2
                          get_local $l0
                          i32.gt_u
                          br_if $B17
                          get_local $l2
                          get_local $p0
                          i32.load offset=4
                          i32.add
                          get_local $l0
                          i32.gt_u
                          br_if $B15
                        end
                        get_local $p0
                        i32.load offset=8
                        tee_local $p0
                        br_if $L16
                      end
                    end
                    i32.const 0
                    set_local $l0
                    i32.const 0
                    i32.load offset=1476
                    tee_local $p0
                    i32.eqz
                    br_if $B7
                    loop $L18
                      get_local $l0
                      i32.const 1
                      i32.add
                      set_local $l0
                      get_local $p0
                      i32.load offset=8
                      tee_local $p0
                      br_if $L18
                    end
                    get_local $l0
                    i32.const 4095
                    get_local $l0
                    i32.const 4095
                    i32.gt_u
                    select
                    set_local $l0
                    br $B6
                  end
                  i32.const 0
                  get_local $l0
                  i32.store offset=1452
                  i32.const 0
                  i32.const 0
                  i32.load offset=1444
                  get_local $p0
                  i32.add
                  tee_local $p0
                  i32.store offset=1444
                  get_local $l0
                  get_local $p0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  get_local $l0
                  get_local $p0
                  i32.add
                  get_local $p0
                  i32.store
                  return
                end
                get_local $l2
                call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                br $B8
              end
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const -2
              get_local $l1
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=1044
            end
            get_local $l0
            get_local $p0
            i32.const 1
            i32.or
            i32.store offset=4
            get_local $l0
            get_local $p0
            i32.add
            get_local $p0
            i32.store
            get_local $l0
            i32.const 0
            i32.load offset=1452
            i32.ne
            br_if $B5
            i32.const 0
            get_local $p0
            i32.store offset=1444
            return
          end
          i32.const 4095
          set_local $l0
        end
        i32.const 0
        i32.const -1
        i32.store offset=1484
        i32.const 0
        get_local $l0
        i32.store offset=1492
        return
      end
      block $B19
        block $B20
          block $B21
            block $B22
              block $B23
                get_local $p0
                i32.const 255
                i32.gt_u
                br_if $B23
                get_local $p0
                i32.const 3
                i32.shr_u
                tee_local $l2
                i32.const 3
                i32.shl
                i32.const 1052
                i32.add
                set_local $p0
                i32.const 0
                i32.load offset=1044
                tee_local $l1
                i32.const 1
                get_local $l2
                i32.const 31
                i32.and
                i32.shl
                tee_local $l2
                i32.and
                i32.eqz
                br_if $B22
                get_local $p0
                i32.const 8
                i32.add
                set_local $l1
                get_local $p0
                i32.load offset=8
                set_local $l2
                br $B21
              end
              get_local $l0
              get_local $p0
              call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
              i32.const 0
              i32.const 0
              i32.load offset=1492
              i32.const -1
              i32.add
              tee_local $l0
              i32.store offset=1492
              get_local $l0
              br_if $B0
              i32.const 0
              i32.load offset=1476
              tee_local $p0
              i32.eqz
              br_if $B20
              i32.const 0
              set_local $l0
              loop $L24
                get_local $l0
                i32.const 1
                i32.add
                set_local $l0
                get_local $p0
                i32.load offset=8
                tee_local $p0
                br_if $L24
              end
              get_local $l0
              i32.const 4095
              get_local $l0
              i32.const 4095
              i32.gt_u
              select
              set_local $l0
              br $B19
            end
            i32.const 0
            get_local $l1
            get_local $l2
            i32.or
            i32.store offset=1044
            get_local $p0
            i32.const 8
            i32.add
            set_local $l1
            get_local $p0
            set_local $l2
          end
          get_local $l1
          get_local $l0
          i32.store
          get_local $l2
          get_local $l0
          i32.store offset=12
          get_local $l0
          get_local $p0
          i32.store offset=12
          get_local $l0
          get_local $l2
          i32.store offset=8
          return
        end
        i32.const 4095
        set_local $l0
      end
      i32.const 0
      get_local $l0
      i32.store offset=1492
    end)
  (func $core::panicking::panic::haf7d7779169c0743 (type $t1) (param $p0 i32)
    (local $l0 i32) (local $l1 i64) (local $l2 i64) (local $l3 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p0
    i64.load offset=16 align=4
    set_local $l1
    get_local $p0
    i64.load offset=8 align=4
    set_local $l2
    get_local $p0
    i64.load align=4
    set_local $l3
    get_local $l0
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    get_local $l3
    i64.store offset=24
    get_local $l0
    i64.const 1
    i64.store offset=4 align=4
    get_local $l0
    i32.const 1544
    i32.store offset=16
    get_local $l0
    get_local $l0
    i32.const 24
    i32.add
    i32.store
    get_local $l0
    get_local $l2
    i64.store offset=32
    get_local $l0
    get_local $l1
    i64.store offset=40
    get_local $l0
    get_local $l0
    i32.const 32
    i32.add
    call $core::panicking::panic_fmt::h29e5105b4d53bc05
    unreachable)
  (func $core::panicking::panic_fmt::h29e5105b4d53bc05 (type $t3) (param $p0 i32) (param $p1 i32)
    get_local $p0
    i32.load
    get_local $p0
    i32.load offset=4
    get_local $p0
    i32.load offset=8
    get_local $p0
    i32.const 12
    i32.add
    i32.load
    get_local $p0
    i32.load offset=16
    get_local $p0
    i32.const 20
    i32.add
    i32.load
    get_local $p1
    i32.load
    get_local $p1
    i32.load offset=4
    get_local $p1
    i32.load offset=8
    get_local $p1
    i32.load offset=12
    call $rust_begin_unwind
    unreachable)
  (func $sum (export "sum") (type $t4) (param $p0 i32) (result i32)
    (local $l0 i64) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    block $B0
      block $B1
        get_local $p0
        i64.extend_u/i32
        tee_local $l0
        i64.const 30
        i64.shr_u
        i32.wrap/i64
        br_if $B1
        get_local $l0
        i64.const 2
        i64.shl
        i32.wrap/i64
        tee_local $l1
        i32.const -1
        i32.le_s
        br_if $B0
        block $B2
          block $B3
            get_local $l1
            i32.eqz
            br_if $B3
            get_local $l1
            call $dlmalloc::dlmalloc::Dlmalloc::malloc::hce1b00d5aca5677c
            tee_local $l2
            br_if $B2
            get_local $l1
            i32.const 4
            call $rust_oom
            unreachable
          end
          i32.const 4
          set_local $l2
        end
        block $B4
          block $B5
            block $B6
              block $B7
                block $B8
                  get_local $p0
                  i32.const 2
                  i32.lt_u
                  br_if $B8
                  get_local $l2
                  set_local $l1
                  i32.const 1
                  set_local $l3
                  loop $L9
                    get_local $l1
                    i32.const 1
                    i32.store
                    get_local $l1
                    i32.const 4
                    i32.add
                    set_local $l1
                    get_local $l3
                    tee_local $l4
                    i32.const 1
                    i32.add
                    tee_local $l3
                    get_local $p0
                    i32.lt_u
                    br_if $L9
                  end
                  get_local $l1
                  i32.const 1
                  i32.store
                  get_local $l4
                  i32.const 1
                  i32.add
                  tee_local $l1
                  i32.eqz
                  br_if $B6
                  get_local $l2
                  get_local $l1
                  i32.const 2
                  i32.shl
                  i32.add
                  set_local $l4
                  br $B7
                end
                get_local $p0
                i32.eqz
                br_if $B4
                get_local $l2
                i32.const 1
                i32.store
                get_local $l2
                i32.const 4
                i32.add
                set_local $l4
              end
              i32.const 0
              set_local $l3
              get_local $l2
              set_local $l1
              loop $L10
                get_local $l1
                i32.load
                get_local $l3
                i32.add
                set_local $l3
                get_local $l4
                get_local $l1
                i32.const 4
                i32.add
                tee_local $l1
                i32.ne
                br_if $L10
              end
              get_local $p0
              br_if $B5
              get_local $l3
              return
            end
            i32.const 0
            set_local $l3
          end
          get_local $l2
          call $dlmalloc::dlmalloc::Dlmalloc::free::h4c32f8306a59a4b8
          get_local $l3
          return
        end
        i32.const 0
        return
      end
      call $<alloc::raw_vec::RawVec<T__A>>::allocate_in::__closure__::h8c98326fcf61324e
      unreachable
    end
    call $<alloc::raw_vec::RawVec<T__A>>::allocate_in::__closure__::hd2ed2b152d4b34da
    unreachable)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17)
  (global $g0 (mut i32) (i32.const 1050160))
  (data (i32.const 1024) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00")
  (data (i32.const 1500) "\00\00\00\00\00liballoc/raw_vec.rscapacity overflow\00\00\00\00")
  (data (i32.const 1548) "\f4\05\00\00\11\00\00\00\e1\05\00\00\13\00\00\00\ee\02\00\00\05\00\00\00"))
