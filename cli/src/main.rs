#![forbid(unsafe_code)]

fn main() {
    println!("cli::ok");
    println!("{}", runtime::ping());
}
