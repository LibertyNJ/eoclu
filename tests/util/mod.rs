use assert_cmd::Command;

pub fn create_test_command() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
