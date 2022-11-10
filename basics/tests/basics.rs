#[test]
fn test_exampl1() {
    assert!(4 == 4);
    assert_eq!(4, 4);
    assert_ne!(4, 5);
}

#[test]
fn test_hello_bin() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn test_true_bin() {
    let mut cmd = assert_cmd::Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false_bin() {
    let mut cmd = assert_cmd::Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
