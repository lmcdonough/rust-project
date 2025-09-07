 2 # Rust based Python Compiler and Interpreter
 3
 4 ## Quickstart

 5 ```bash
 6 # one-stop dev loop (fmt → clippy → check)
 7 make dev
 8 # or via Cargo alias
 9 cargo dev
10 # or individual steps
11 make fmt && make lint && make check
12```
 13
14 ## Binaries
15 ```bash
16 cargo run -p cli
17 cargo run -p repl
18```
19
20 ## Notes
21 - Keep crate layering clean: ast → lexer → parser → runtime → cli/repl.
22 - Use `cargo fmt`, `cargo clippy`, and `cargo check` at the workspace level for consistency.
