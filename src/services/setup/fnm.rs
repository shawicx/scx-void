use crate::errors::ScxVoidError;
use crate::services::setup::installer::{Installer, ShellConfig};

pub struct FnmInstaller;

impl Installer for FnmInstaller {
    fn name(&self) -> &str {
        "fnm (Fast Node Manager)"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("fnm", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        #[cfg(target_os = "macos")]
        {
            duct::cmd(
                "bash",
                &["-c", "curl -fsSL https://fnm.vercel.app/install | bash"],
            )
            .run()
            .map_err(|e| ScxVoidError::InstallationFailed {
                component: self.name().to_string(),
                reason: e.to_string(),
            })?;

            ShellConfig::append_if_absent("eval \"$(fnm env --use-on-cd)\"")?;
        }

        #[cfg(target_os = "windows")]
        {
            duct::cmd("winget", &["install", "Schniz.fnm"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }
}
