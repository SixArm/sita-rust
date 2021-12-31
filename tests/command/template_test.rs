use assertables::*;
use lazy_static::*;
use std::path::PathBuf;
use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

lazy_static! {
    pub static ref DIR: PathBuf = TESTS_DIR.join("command").join("template");
}

#[test]
fn test_command_x_template() {
    // Given
    let template: PathBuf = DIR.join("custom-template.html");
    let input: PathBuf = DIR.join("example.md");
    let actual: PathBuf = DIR.join("example.html");
    let expect: PathBuf = DIR.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(template.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual.exists() path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
