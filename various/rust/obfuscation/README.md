# obfuscation

- About `etc` folder
  - Placing decompile artifacts of obfuscation-aware wasm module

```bash
wasm2wat target/wasm32-unknown-unknown/release/raw_url.wasm > etc/src/raw_url.wat
wasm2wat target/wasm32-unknown-unknown/release/from_hex_str.wasm > etc/src/from_hex_str.wat
wasm2wat target/wasm32-unknown-unknown/release/from_binary.wasm > etc/src/from_binary.wat
wasm2wat target/wasm32-unknown-unknown/release/from_divided_hex_str.wasm > etc/src/from_divided_hex_str.wat

for value in `ls src`
do
  wasm2wat "target/wasm32-unknown-unknown/release/$value.wasm" > "etc/$value.wat"
done
```
