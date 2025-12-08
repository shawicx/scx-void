#[cfg(test)]
mod system_commands_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_system_shutdown_command_exists() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("scx-void")?;
        let assert = cmd.arg("--help").assert();
        assert.success().stdout(predicates::str::contains("system"));

        Ok(())
    }

    #[test]
    fn test_system_shutdown_help() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("scx-void")?;
        let assert = cmd.arg("system").arg("shutdown").arg("--help").assert();
        assert.success()
            .stdout(predicates::str::contains("--timer"))
            .stdout(predicates::str::contains("关机计时器"));

        Ok(())
    }
}