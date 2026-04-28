use crate::errors::ScxVoidError;
use crate::services::setup::installer::Installer;

pub struct ChsrcInstaller;

impl Installer for ChsrcInstaller {
    fn name(&self) -> &str {
        "chsrc (镜像源切换)"
    }

    fn is_installed(&self) -> Option<String> {
        duct::cmd("chsrc", &["--version"])
            .read()
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn install(&self) -> Result<(), ScxVoidError> {
        let binary_path = download_binary()?;
        make_executable(&binary_path)?;

        duct::cmd("chsrc", &["set", "node"])
            .run()
            .map(|_| ())
            .map_err(|e| ScxVoidError::InstallationFailed {
                component: self.name().to_string(),
                reason: format!("镜像源切换失败: {}", e),
            })?;

        Ok(())
    }
}

fn download_url() -> String {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let arch = "aarch64-macos";
    #[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
    let arch = "x86_64-macos";
    #[cfg(target_os = "windows")]
    let arch = "x86_64-windows";

    #[cfg(target_os = "windows")]
    let ext = ".exe";
    #[cfg(not(target_os = "windows"))]
    let ext = "";

    format!(
        "https://github.com/RubyMetric/chsrc/releases/latest/download/chsrc-{}{}",
        arch, ext
    )
}

fn download_binary() -> Result<std::path::PathBuf, ScxVoidError> {
    let url = download_url();
    let dest = install_path()?;

    let response =
        reqwest::blocking::get(&url).map_err(|e| ScxVoidError::NetworkError(e.to_string()))?;
    if !response.status().is_success() {
        return Err(ScxVoidError::InstallationFailed {
            component: "chsrc".to_string(),
            reason: format!("下载失败: HTTP {}", response.status()),
        });
    }

    let bytes = response
        .bytes()
        .map_err(|e| ScxVoidError::NetworkError(e.to_string()))?;
    std::fs::write(&dest, &bytes).map_err(|e| ScxVoidError::FileSystemError(e.to_string()))?;

    Ok(dest)
}

fn install_path() -> Result<std::path::PathBuf, ScxVoidError> {
    #[cfg(target_os = "macos")]
    {
        Ok(std::path::PathBuf::from("/usr/local/bin/chsrc"))
    }
    #[cfg(target_os = "windows")]
    {
        let home = dirs::home_dir()
            .ok_or_else(|| ScxVoidError::FileSystemError("无法获取 home 目录".to_string()))?;
        Ok(home.join("AppData/Local/bin/chsrc.exe"))
    }
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) -> Result<(), ScxVoidError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| ScxVoidError::FileSystemError(e.to_string()))
}

#[cfg(windows)]
fn make_executable(_path: &std::path::Path) -> Result<(), ScxVoidError> {
    Ok(())
}
