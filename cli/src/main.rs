#![forbid(unsafe_code)]

// import env and process for simple flag handling and exiting with code 0
use std::env;
use std::process;

// helper that prints a usage banner and exits nicely
fn print_usage_and_exit() -> ! {
    println!("Usage: cli [OPTIONS]\n\nOptions:\n -h, --help  Show help");
    process::exit(0)
}

fn main() {
    // handle -h/--help early; print Usage and exit 0 to satisfy the test
    if env::args().any(|a| a == "-h" || a == "--help") {
        print_usage_and_exit();
    }

    // otherwise, run the main logic
    println!("cli::ok");
    println!("{}", runtime::ping());
}
