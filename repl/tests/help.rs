// integration test: ensure the repl binary runs with --help and exits 0
// uses the same CARGO_BIN_EXE_<name> trick as the cli test
use std::process::Command;

#[test]
fn help_prints_and_exits_zero() {
    // resolve the compiled repl binary path
    let exe = env!("CARGO_BIN_EXE_repl");

    // run the binary with --help
    let output = std::process::Command::new(exe)
        .arg("--help")
        .output()
        .expect("failed to spawn repl --help");

    // assert it succeeded
    assert!(output.status.success(), "repl --help failed: {:?}", output.status);

    // basic content smoke check
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    assert!(
        stdout.contains("usage") || stdout.contains("help"),
        "help text missing expected keywords, got: {}",
        stdout
    );
}
