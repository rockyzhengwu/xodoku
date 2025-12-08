export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build --target web
cp pkg/* ../sudoku-web/app/wasm
