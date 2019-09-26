(module
  (import "host" "print" (func $imported (param i32) (result i32)))

  (func (export "f") (result i32)
    i32.const 42
    return_call $imported
    unreachable
  )
)
