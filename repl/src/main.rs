// Purpose: simple "hello" now; real REPL in Phase 8.
#![forbid(unsafe_code)]

fn main() {
    println!("repl::ok");
    println!("{}", runtime::ping());
}