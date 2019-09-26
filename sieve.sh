#!/bin/bash
wat2wasm --enable-tail-call -v sieve.wat -o sieve.wasm
wasm-interp --run-all-exports --enable-tail-call --host-print sieve.wasm
