#[allow(dead_code)]
use crate::platform::SystemOps;

#[allow(dead_code)]
pub struct WindowsPlatform;

impl SystemOps for WindowsPlatform {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression {
        duct::cmd("shutdown", &["/s", "/t", &seconds.to_string()])
    }
}

#[allow(dead_code)]
impl WindowsPlatform {
    pub fn new() -> Self {
        WindowsPlatform {}
    }
}