# Build to a WASM component using the WASIp2 API

## Steps

1. rustup target add wasm32-wasip2  
  `wasm32-wasip2` is availabe with rust `1.82.0`.
1. cargo build --target wasm32-wasip2
1. copy the `target/wasm32-wasip2/debug/wasm_component.wasm` to the `wasmtime_sample/wasms` directory.

## Convert to a wat file

1. install wasm-tools  
   https://github.com/bytecodealliance/wasm-tools
2. wasm-tools print wasm_component.wasm -o wasm_component.wat
