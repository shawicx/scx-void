pub mod windows;
pub mod macos;

#[allow(dead_code)]
pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn get_platform() -> impl SystemOps {
    WindowsPlatform {}
}

#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {}
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {} // Using the same implementation for Linux as macOS for now
}