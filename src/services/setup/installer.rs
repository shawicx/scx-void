use crate::errors::ScxVoidError;
use std::io::{self, BufRead, Write};

pub trait Installer {
    fn name(&self) -> &str;
    fn is_installed(&self) -> Option<String>;
    fn install(&self) -> Result<(), ScxVoidError>;
}

#[allow(dead_code)]
pub struct ShellConfig;

#[allow(dead_code)]
impl ShellConfig {
    fn default_rc_path() -> Option<std::path::PathBuf> {
        let home = dirs::home_dir()?;
        #[cfg(target_os = "macos")]
        {
            let shell = std::env::var("SHELL").unwrap_or_default();
            if shell.contains("bash") {
                Some(home.join(".bashrc"))
            } else {
                Some(home.join(".zshrc"))
            }
        }
        #[cfg(target_os = "windows")]
        {
            let profile = dirs::document_dir()?;
            Some(profile.join("PowerShell/Microsoft.PowerShell_profile.ps1"))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = home;
            None
        }
    }

    pub fn append_if_absent(line: &str) -> Result<(), ScxVoidError> {
        let path = Self::default_rc_path().ok_or_else(|| ScxVoidError::ShellConfigError {
            path: "unknown".to_string(),
            reason: "无法确定 Shell 配置文件路径".to_string(),
        })?;

        if path.exists() {
            let file = std::fs::File::open(&path).map_err(|e| ScxVoidError::ShellConfigError {
                path: path.display().to_string(),
                reason: e.to_string(),
            })?;
            let reader = io::BufReader::new(file);
            if reader
                .lines()
                .any(|l| l.map(|l| l.contains(line)).unwrap_or(false))
            {
                return Ok(());
            }
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| ScxVoidError::ShellConfigError {
                path: path.display().to_string(),
                reason: e.to_string(),
            })?;

        writeln!(file, "\n{}", line).map_err(|e| ScxVoidError::ShellConfigError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })
    }
}
