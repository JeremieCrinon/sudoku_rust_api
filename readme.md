# sudoku_wasm
A sudoku resolver has a JS class with 2 ways of resolving :
- Using WASM written in Rust (link here : [https://github.com/JeremieCrinon/sudoku_wasm](https://github.com/JeremieCrinon/sudoku_wasm))
- Using an API written in Rust
The code is both this repository and the WASM are line-by-line commented.


This is the API part, it oppens a post route that gets a grid and returns it solved.
The entry point of the app is in src/main.rs, you might wanna start here.