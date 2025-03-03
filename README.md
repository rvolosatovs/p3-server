```
$ cargo build --target wasm32-unknown-unknown --release
$ wasm-tools component new --skip-validation ./target/wasm32-unknown-unknown/release/p3_server.wasm -o component.wasm
$ wasmtime run -Sinherit-network=y component.wasm 8080
```

```
$ echo 'hello' | nc 127.0.0.1 8080
```
