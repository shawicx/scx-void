use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct BunInstaller;

impl Installer for BunInstaller {
    fn name(&self) -> &str {
        "Bun"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("bun", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        #[cfg(target_os = "macos")]
        {
            duct::cmd("bash", &["-c", "curl -fsSL https://bun.sh/install | bash"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        #[cfg(target_os = "windows")]
        {
            duct::cmd("powershell", &["-c", "irm bun.sh/install.ps1 | iex"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }
}
