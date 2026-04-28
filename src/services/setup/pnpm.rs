use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct PnpmInstaller;

impl Installer for PnpmInstaller {
    fn name(&self) -> &str {
        "pnpm"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("pnpm", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        duct::cmd("npm", &["install", "-g", "pnpm"])
            .run()
            .map_err(|e| ScxVoidError::InstallationFailed {
                component: self.name().to_string(),
                reason: e.to_string(),
            })
            .map(|_| ())
    }
}
