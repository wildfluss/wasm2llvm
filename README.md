# wasm2llvm

Lift WASM to LLVM IR.

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


