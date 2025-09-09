// integration test: ensure the cli binary runs with --help and exits 0
// we use CARGO_BIN_EXE <name> so we dont need extra dev-dependencies
// this env var is auto set by cargo for bin targets in this package
#[test]
fn help_prints_and_exits_zero() {
    // resolve the compiled cli binary path from env
    let exe = env!("CARGO_BIN_EXE_cli");

    // run the binary with --help
    let output = std::process::Command::new(exe)
        .arg("--help")
        .output()
        .expect("failed to spawn cli --help");

    // assert it succeeded
    assert!(output.status.success(), "cli --help failed: {:?}", output.status);

    // basic content smoke check
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    assert!(
        stdout.contains("usage") || stdout.contains("help"),
        "help text missing expected keywords, got: {}", stdout
    );
}