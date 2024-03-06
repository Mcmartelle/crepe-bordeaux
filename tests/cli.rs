use assert_cmd::Command;

#[test]
fn clear_copy_paste_and_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_clear = Command::cargo_bin("cb")?;
    let assert_clear = cmd_clear.arg("clear-all").assert();
    assert_clear.success().stdout("");

    let mut cmd_copy = Command::cargo_bin("cb")?;
    let assert_copy = cmd_copy
        .arg("memorable-name")
        .write_stdin("thing I want to save for a while")
        .assert();
    assert_copy.success().stdout("");

    let mut cmd_paste = Command::cargo_bin("cb")?;
    let assert_paste = cmd_paste.arg("memorable-name").assert();
    assert_paste
        .success()
        .stdout("thing I want to save for a while\n");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list.success().stdout("memorable-name\n");

    let mut cmd_copy = Command::cargo_bin("cb")?;
    let assert_copy = cmd_copy
        .arg("secondthing")
        .write_stdin("more things to save for later")
        .assert();
    assert_copy.success().stdout("");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list
        .success()
        .stdout("memorable-name\nsecondthing\n");

    let mut cmd_clear = Command::cargo_bin("cb")?;
    let assert_clear = cmd_clear.arg("clear-all").assert();
    assert_clear.success().stdout("");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list.success().stdout("\n");

    Ok(())
}
