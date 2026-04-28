use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct WebstormInstaller;

impl Installer for WebstormInstaller {
    fn name(&self) -> &str {
        "WebStorm"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("webstorm", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
            .or_else(|| {
                #[cfg(target_os = "macos")]
                {
                    let path = std::path::Path::new("/Applications/WebStorm.app");
                    if path.exists() {
                        Some("installed".to_string())
                    } else {
                        None
                    }
                }
                #[cfg(target_os = "windows")]
                {
                    None
                }
            })
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        #[cfg(target_os = "macos")]
        {
            duct::cmd("brew", &["install", "--cask", "webstorm"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        #[cfg(target_os = "windows")]
        {
            duct::cmd("winget", &["install", "JetBrains.WebStorm"])
                .run()
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }
}
