# wasm2llvm

Lift WASM to LLVM IR.

Example from [testsuite/i32.wast](testsuite/i32.wast)

```
(module
  (func (export "add") (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y)))
```

`wasm2llvm` uses [wabt](https://crates.io/crates/wabt) to get module binary representation corresponding to text format to e.g.:

```
(func (export "add") (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y))
```
`wasm2ll` takes name of WebAssembly function definition e.g. `add` and corresponding WebAssembly bytecode and outputs LLVM IR e.g.:

```
define i32 @add(i32 %a, i32 %b) #0 {
  %1 = add nsw i32 %a, %b
  ret i32 %1
}
```

## How to compile Rust to WebAssembly and get text representation

Install target for WebAssembly 

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Create a simple program that always returs 1.

```
echo "fn main(){1;}" > test.rs
```

Compile it to WebAssembly 

```
rustup run nightly rustc --target=wasm32-unknown-unknown test.rs
```

Set up wabt.

Convert the assembly into the text representation.

```
wasm2wat test.wasm -o test.wat
```

Run it with wasmtime

```
wasmtime test.wasm
```


