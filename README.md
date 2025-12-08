# Xodoku

## About The Project

Existing tools like Hodoku and Sudoku Explainer are useful, they are often proprietary or use older technologies. I want to leverage Rust's performance and safety to create a modern one", with some AI feature , such as sudoku scanner.

the app is [xodoku](xodoku.com)

- [sudoku-rs](./sodoku-rs) is the sudoku core , scanner part and the app will release soon.
- [sudoku-wasm](./sudoku-wasm/): sudoku-rs wasm api
- [sudoku-web](./sudoku-web/): sudoku player and scanner, build on sudoku-wasm and onnx-runtime
