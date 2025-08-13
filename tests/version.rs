//! Integration tests that exercise version functionality.

use util::assert_output_snapshot;

mod util;

#[test]
fn should_display_version_when_passed_long_flag() {
    let mut cmd = util::create_test_command();
    let assert = cmd.arg("--version").assert().success().code(0);
    assert_output_snapshot!(assert.get_output());
}

#[test]
fn should_display_version_when_passed_short_flag() {
    let mut cmd = util::create_test_command();
    let assert = cmd.arg("-V").assert().success().code(0);
    assert_output_snapshot!(assert.get_output());
}
