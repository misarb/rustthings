use assert_cmd::Command;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("rprojcet").unwrap();
    cmd.assert().success().stdout("Hello orld");
}
