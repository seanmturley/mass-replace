use assert_cmd::Command;

#[test]
fn cli_requires_input_file() {
    let mut cmd = Command::cargo_bin("mass-replace").unwrap();
    cmd.assert().failure().stderr(predicates::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
fn cli_help_output() {
    let mut cmd = Command::cargo_bin("mass-replace").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("mass-replace"));
}
