use std::path::Path;

use assert_cmd::Command;

fn cargo_test(current_dir: impl AsRef<Path>) {
    let current_dir = current_dir.as_ref();
    Command::new("cargo")
        .args(["build", "--workspace"])
        .current_dir(current_dir)
        .assert()
        .success();
    Command::new("cargo")
        .args(["xtask", "dist"])
        .current_dir(current_dir)
        .assert()
        .success();
}

#[test]
fn example_simple() {
    cargo_test("examples/simple");
}

#[test]
fn example_with_command() {
    cargo_test("examples/with_command");
}
