use assert_cmd::Command;
use predicates::prelude::*;
#[test]
fn test_no_arg(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stdout(predicate::str::contains("USAGE"));
}
#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}