// Purpose: quick smoke test of the workspace wiring.
#![forbid(unsafe_code)]

fn main() {
    println!("cli::ok");
    println!("{}", runtime::ping());
}