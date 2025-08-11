use assert_cmd::Command;

pub fn create_test_command() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

macro_rules! assert_output_snapshot {
    ($output:expr) => {{
        let snapshot = format!(
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&$output.stdout),
            String::from_utf8_lossy(&$output.stderr)
        );
        insta::assert_snapshot!(snapshot);
    }};
}

pub(crate) use assert_output_snapshot;
