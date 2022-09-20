use assert_cmd::Command;
use predicates::prelude::*;
type TestResult = Result<(),Box<dyn std::error::Error>>;
#[test]
fn test_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}
// #[test]
// fn runs() -> TestResult {
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.arg("hello").assert().success();
//     Ok(())
// }
