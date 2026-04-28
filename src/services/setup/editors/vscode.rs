use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct VscodeInstaller;

impl Installer for VscodeInstaller {
    fn name(&self) -> &str {
        "Visual Studio Code"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("code", &["--version"])
            .read()
            .ok()
            .map(|v| v.lines().next().unwrap_or("").trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        #[cfg(target_os = "macos")]
        {
            duct::cmd("brew", &["install", "--cask", "visual-studio-code"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        #[cfg(target_os = "windows")]
        {
            duct::cmd("winget", &["install", "Microsoft.VisualStudioCode"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }
}
