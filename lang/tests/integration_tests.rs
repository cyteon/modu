use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

fn run_test(name: &str) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("cases");
    let modu_file = dir.join(format!("{}.modu", name));
    let expected_file = dir.join(format!("{}.expected", name));

    let expected_output = fs::read_to_string(&expected_file)
        .expect("Failed to read expected output file");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("Failed to find binary")
        .arg("run")
        .arg(modu_file)
        .assert()
        .success()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn basic_print() {
    run_test("basic_print");
}

#[test]
fn nesting_funcs() {
    run_test("nesting_funcs");
}

#[test]
fn type_conversion() {
    run_test("type_conversion");
}

#[test]
fn file_reading() {
    run_test("file_reading");
}

#[test]
fn http_requests() {
    run_test("http");
}