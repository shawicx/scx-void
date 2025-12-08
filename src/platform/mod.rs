pub mod windows;
pub mod macos;

pub use windows::*;
pub use macos::*;

pub trait SystemOps {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression;
}

#[cfg(target_os = "windows")]
pub fn get_platform() -> impl SystemOps {
    WindowsPlatform {}
}

#[cfg(target_os = "macos")]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {}
}

#[cfg(target_os = "linux")]
pub fn get_platform() -> impl SystemOps {
    MacosPlatform {} // Using the same implementation for Linux as macOS for now
}