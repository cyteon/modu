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
fn comparisons() {
    run_test("comparisons");
}

#[test]
fn crypto() {
    run_test("crypto");
}

#[test]
fn encoding() {
    run_test("encoding");
}

#[test]
fn http() {
    run_test("http");
}

#[test]
fn if_statements() {
    run_test("if_statements");
}

#[test]
fn loops() {
    run_test("loops");
}

#[test]
fn math() {
    run_test("math");
}

#[test]
fn minskys_machine() {
    run_test("minskys_machine");
}

#[test]
fn nesting_funcs() {
    run_test("nesting_funcs");
}

#[test]
fn type_conversion() {
    run_test("type_conversion");
}