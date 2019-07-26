use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn it_runs() {
    Command::cargo_bin("todo").unwrap().assert().success();
}

#[test]
fn it_prints_hello_world() {
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.assert().stdout("Hello, world!\n");
}

fn it_prints_hello_world() {
    // Call the command
    let mut cmd = Command::cargo_bin("todo")
        .arg("add")
        .arg("Test todo")
        .unwrap();
    // Assert stdout
    // Check that the todo text is in the file
}
