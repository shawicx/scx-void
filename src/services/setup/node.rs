use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct NodeInstaller;

impl Installer for NodeInstaller {
    fn name(&self) -> &str {
        "Node.js (LTS)"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("node", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        duct::cmd("fnm", &["install", "--lts"]).run().map_err(|e| {
            ScxVoidError::InstallationFailed {
                component: self.name().to_string(),
                reason: e.to_string(),
            }
        })?;

        duct::cmd("fnm", &["default", "lts-latest"])
            .run()
            .map_err(|e| ScxVoidError::InstallationFailed {
                component: self.name().to_string(),
                reason: format!("设置默认版本失败: {}", e),
            })?;

        Ok(())
    }
}
