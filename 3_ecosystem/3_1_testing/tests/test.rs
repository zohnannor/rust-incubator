use std::{ffi::OsStr, process::Command};

fn contains(output: &[u8], expected: &str) -> bool {
    String::from_utf8_lossy(output).contains(expected)
}

fn spawn<S: AsRef<OsStr>>(arg: S) -> std::process::Child {
    Command::new("cargo")
        .args(["r", "-p", "step_3_1", "--"])
        .arg(arg)
        .spawn()
        .expect("failed to run binary")
}

fn output<I, S>(args: I) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("cargo")
        .args(["r", "-p", "step_3_1", "--"])
        .args(args)
        .output()
        .expect("failed to get output")
}

#[test]
fn works() {
    spawn("").kill().expect("failed to kill process");
}

#[test]
fn no_secret_number_is_specified() {
    let output = output(&[] as &[&str]);

    assert!(!output.status.success());
    let pat = &"No secret number is specified";
    let v = &output.stderr;
    assert!(contains(v, pat), "{:?}", output);
}

#[test]
fn secret_number_is_not_a_number() {
    let output = Command::new("cargo")
        .args(["r", "-p", "step_3_1", "--", "a"])
        .output()
        .expect("failed to kill process");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("Secret number is not a number: ParseIntError"));
}
