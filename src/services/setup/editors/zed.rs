use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct ZedInstaller;

impl Installer for ZedInstaller {
    fn name(&self) -> &str {
        "Zed"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("zed", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        duct::cmd(
            "bash",
            &["-c", "curl -fsSL https://zed.dev/install.sh | bash"],
        )
        .run()
        .map_err(|e| ScxVoidError::InstallationFailed {
            component: self.name().to_string(),
            reason: e.to_string(),
        })?;
        Ok(())
    }
}
