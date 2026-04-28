use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct CursorInstaller;

impl Installer for CursorInstaller {
    fn name(&self) -> &str {
        "Cursor"
    }

    fn is_installed(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            duct::cmd("cursor", &["--version"])
                .read()
                .ok()
                .map(|v| v.trim().to_string())
                .or_else(|| {
                    let path = std::path::Path::new("/Applications/Cursor.app");
                    if path.exists() {
                        Some("installed".to_string())
                    } else {
                        None
                    }
                })
        }
        #[cfg(target_os = "windows")]
        {
            duct::cmd("cursor", &["--version"])
                .read()
                .ok()
                .map(|v| v.trim().to_string())
        }
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        #[cfg(target_os = "macos")]
        {
            println!("  请从 https://cursor.com 下载安装 Cursor");
            Ok(())
        }

        #[cfg(target_os = "windows")]
        {
            duct::cmd("winget", &["install", "Cursor.Cursor"])
                .run()
                .map(|_| ())
                .map_err(|e| ScxVoidError::InstallationFailed {
                    component: self.name().to_string(),
                    reason: e.to_string(),
                })
        }
    }
}
