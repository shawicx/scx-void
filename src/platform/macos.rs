#[allow(dead_code)]
use crate::platform::SystemOps;

pub struct MacosPlatform;

impl SystemOps for MacosPlatform {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression {
        // Convert seconds to minutes for macOS shutdown command
        let minutes = (seconds as f64 / 60.0).ceil() as u64;
        duct::cmd("sudo", &["shutdown", "-h", &format!("+{}", minutes)])
    }
}

#[allow(dead_code)]
impl MacosPlatform {
    pub fn new() -> Self {
        MacosPlatform {}
    }
}
