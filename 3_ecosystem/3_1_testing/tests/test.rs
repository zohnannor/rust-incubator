use std::process::{Child, Command, Output, Stdio};
use std::{fmt::Write as _, io::Write as _};

use rand::Rng;

fn contains(output: &[u8], expected: &str) -> bool {
    String::from_utf8_lossy(output).contains(expected)
}

fn spawn(arg: &[&str]) -> Child {
    Command::new("cargo")
        .args(["r", "-p", "step_3_1", "--"])
        .args(arg)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run binary")
}

fn output(args: &[&str]) -> Output {
    Command::new("cargo")
        .args(["r", "-p", "step_3_1", "--"])
        .args(args)
        .output()
        .expect("failed to get output")
}

fn input(child: &mut Child, inputs: &[&str]) {
    for input in inputs {
        dbg!(input);
        child
            .stdin
            .as_ref()
            .expect("failed to get stdin")
            .write_all(input.as_bytes())
            .expect("failed to write input to stdin");
        child
            .stdin
            .as_ref()
            .expect("failed to get stdin")
            .write_all(b"\n")
            .expect("failed to write newline");
    }
}

fn stdout(child: Child) -> Output {
    child
        .wait_with_output()
        .expect("failed to get child's output after writing")
}

#[test]
fn works() {
    spawn(&[]).kill().expect("failed to kill process");
}

#[test]
fn no_secret_number_is_specified() {
    let output = output(&[]);

    assert!(!output.status.success(), "exit status was not unsuccessful");
    assert!(
        contains(&output.stdout, "Guess the number!"),
        "The output was: {:?}",
        output
    );
    assert!(
        contains(&output.stderr, "No secret number is specified"),
        "The output was: {:?}",
        output
    );
}

#[test]
fn secret_number_is_not_a_number() {
    let output = output(&["a"]);

    assert!(!output.status.success(), "exit status was not unsuccessful");
    assert!(
        contains(&output.stdout, "Guess the number!"),
        "The output was: {:?}",
        output
    );
    assert!(
        contains(
            &output.stderr,
            "Secret number is not a number: ParseIntError"
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn takes_secret_number_argument() {
    let child = spawn(&["42"]);
    child.stdin.as_ref().unwrap().write_all(b"42").unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn invalid_input() {
    let output = output(&["\u{1234}\x42\n"]);

    assert!(!output.status.success(), "exit status was not unsuccessful");
    assert!(
        contains(&output.stdout, "Guess the number!"),
        "The output was: {:?}",
        output
    );
    assert!(
        contains(
            &output.stderr,
            "Secret number is not a number: ParseIntError"
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn negative_number() {
    let output = output(&["-42"]);

    assert!(!output.status.success(), "exit status was not unsuccessful");
    assert!(
        contains(&output.stdout, "Guess the number!"),
        "The output was: {:?}",
        output
    );
    assert!(
        contains(
            &output.stderr,
            "Secret number is not a number: ParseIntError"
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn u64_max() {
    let output = output(&[&u64::MAX.to_string()]);

    assert!(!output.status.success(), "exit status was not unsuccessful");
    assert!(
        contains(&output.stdout, "Guess the number!"),
        "The output was: {:?}",
        output
    );
    assert!(
        contains(
            &output.stderr,
            "Secret number is not a number: ParseIntError"
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn guessed() {
    let mut child = spawn(&["42"]);
    input(&mut child, &["42"]);
    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
You guessed: 42
You win!"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn two_arguments() {
    let mut child = spawn(&["42", "42"]);
    input(&mut child, &["42"]);
    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
You guessed: 42
You win!"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn two_small() {
    let mut child = spawn(&["42"]);
    input(&mut child, &["0", "42"]);
    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
You guessed: 0
Too small!
Please input your guess.
You guessed: 42
You win!"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn two_big() {
    let mut child = spawn(&["42"]);
    input(&mut child, &["1000", "42"]);
    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
You guessed: 1000
Too big!
Please input your guess.
You guessed: 42
You win!"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn u32_max() {
    let n = u32::MAX.to_string();
    let mut child = spawn(&[&n]);
    input(&mut child, &[&n]);
    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(
            &output.stdout,
            r#"Guess the number!
Please input your guess.
You guessed: 4294967295
You win!"#
        ),
        "The output was: {:?}",
        output
    );
}

#[test]
fn random_inputs() {
    let mut expected_output = String::from("Guess the number!\nPlease input your guess.\n");
    let secret_number = rand::thread_rng().gen_range(0..=u32::MAX);
    let mut child = spawn(&[&secret_number.to_string()]);

    for _ in 0..10 {
        let guess = rand::thread_rng().gen_range(0..=u32::MAX);
        input(&mut child, &[&guess.to_string()]);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => write!(
                expected_output,
                "You guessed: {}\nToo big!\nPlease input your guess.\n",
                guess
            )
            .unwrap(),
            std::cmp::Ordering::Less => write!(
                expected_output,
                "You guessed: {}\nToo small!\nPlease input your guess.\n",
                guess
            )
            .unwrap(),
            std::cmp::Ordering::Equal => continue,
        };
    }

    input(&mut child, &[&secret_number.to_string()]);
    write!(
        expected_output,
        "You guessed: {}\nYou win!\n",
        secret_number
    )
    .unwrap();

    let output = stdout(child);

    assert!(output.status.success(), "exit status was not successful");
    assert!(
        contains(&output.stdout, &expected_output),
        "The output was: {:?}\n\nExpected output is: {:?}",
        output,
        expected_output
    );
}
