(module
  (func (export "fizzbuzz") (param $1 i32) (result i32)
    block $default
      block $fizz
        block $buzz
          block $fizzbuzz
            get_local 0
            i32.const 15
            i32.rem_u
            br_table $fizzbuzz $default $default $fizz    $default
                     $buzz     $fizz    $default $default $fizz
                     $buzz     $default $fizz    $default 
          end
          i32.const 3
          return
        end
        i32.const 2
        return
      end
      i32.const 1
      return
    end
    i32.const 0
    return
  )
)