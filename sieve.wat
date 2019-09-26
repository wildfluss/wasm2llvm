(module
  (import "host" "print" (func $imported (param i32) (result i32)))

  (memory (data "hello")) ;; 104 101 108 108 111

  (func (export "f") (result i32)
    (i32.load8_s (i32.const 1)) ;; 101
    return_call $imported
    unreachable
  )
)
