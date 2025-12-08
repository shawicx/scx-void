use crate::platform::SystemOps;

pub struct WindowsPlatform;

impl SystemOps for WindowsPlatform {
    fn shutdown_in(&self, seconds: u64) -> duct::Expression {
        duct::cmd("shutdown", &["/s", "/t", &seconds.to_string()])
    }
}

impl WindowsPlatform {
    pub fn new() -> Self {
        WindowsPlatform {}
    }
}