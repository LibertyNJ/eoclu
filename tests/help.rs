//! Integration tests that exercise help functionality.

use util::assert_output_snapshot;

mod util;

#[cfg(target_family="unix")]
mod unix {
    use super::*;

    #[test]
    fn should_display_concise_help_when_passed_short_flag() {
        let mut cmd = util::create_test_command();
        let assert = cmd.arg("-h").assert().success().code(0);
        assert_output_snapshot!(assert.get_output());
    }

    #[test]
    fn should_display_full_help_when_passed_long_flag() {
        let mut cmd = util::create_test_command();
        let assert = cmd.arg("--help").assert().success().code(0);
        assert_output_snapshot!(assert.get_output());
    }
}
