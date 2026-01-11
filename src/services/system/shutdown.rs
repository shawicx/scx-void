// Define the shutdown functionality directly here, without depending on external platform modules
// The platform-specific logic will be handled in the CLI layer

use duct;

pub struct ShutdownService;

impl ShutdownService {
    #[cfg(target_os = "windows")]
    pub fn shutdown_in(seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = duct::cmd("shutdown", &["/s", "/t", &seconds.to_string()]);
        cmd.run()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub fn shutdown_in(seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Convert seconds to minutes for macOS shutdown command
        let minutes = (seconds as f64 / 60.0).ceil() as u64;
        let cmd = duct::cmd("sudo", &["shutdown", "-h", &format!("+{}", minutes)]);
        cmd.run()?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn shutdown_in(seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = duct::cmd("shutdown", &["-h", &format!("+{}", seconds)]);
        cmd.run()?;
        Ok(())
    }
}
