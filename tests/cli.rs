use assert_cmd::Command;

#[test]
fn one_test_so_it_is_single_threaded() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_clear = Command::cargo_bin("cb")?;
    let assert_clear = cmd_clear.arg("clear-all").assert();
    assert_clear.success().stdout("");

    let mut cmd_paste = Command::cargo_bin("cb")?;
    let assert_paste = cmd_paste.arg("unregistered-name").arg("--verbose").assert();
    assert_paste.success().stderr("register not found\n");

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
    assert_list.success().stdout("memorable-name");

    let mut cmd_copy = Command::cargo_bin("cb")?;
    let assert_copy = cmd_copy
        .arg("secondthing")
        .arg("--verbose")
        .write_stdin("more things to save for later")
        .assert();
    assert_copy
        .success()
        .stdout("more things to save for later\ncopied successfully to register: secondthing\n");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list.success().stdout("memorable-name\nsecondthing");

    let mut cmd_dump = Command::cargo_bin("cb")?;
    let assert_dump = cmd_dump.arg("dump").assert();
    assert_dump
        .success()
        .stdout("thing I want to save for a while\nmore things to save for later\n");

    let mut cmd_clear = Command::cargo_bin("cb")?;
    let assert_clear = cmd_clear.arg("memorable-name").arg("clear").assert();
    assert_clear.success().stdout("");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list.success().stdout("secondthing");

    let mut cmd_clear = Command::cargo_bin("cb")?;
    let assert_clear = cmd_clear.arg("clear-all").assert();
    assert_clear.success().stdout("");

    let mut cmd_list = Command::cargo_bin("cb")?;
    let assert_list = cmd_list.arg("list").assert();
    assert_list.success().stdout("");

    Ok(())
}
