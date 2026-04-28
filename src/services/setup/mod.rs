pub mod bun;
pub mod chsrc;
pub mod editors;
pub mod fnm;
pub mod installer;
pub mod node;
pub mod pnpm;

use crate::errors::ScxVoidError;
use installer::Installer;

pub struct SetupService {
    installers: Vec<Box<dyn Installer>>,
}

impl SetupService {
    pub fn frontend() -> Self {
        let mut installers: Vec<Box<dyn Installer>> = vec![
            Box::new(fnm::FnmInstaller),
            Box::new(node::NodeInstaller),
            Box::new(pnpm::PnpmInstaller),
            Box::new(bun::BunInstaller),
            Box::new(chsrc::ChsrcInstaller),
            Box::new(editors::cursor::CursorInstaller),
            Box::new(editors::vscode::VscodeInstaller),
            Box::new(editors::webstorm::WebstormInstaller),
        ];
        #[cfg(target_os = "macos")]
        installers.push(Box::new(editors::zed::ZedInstaller));
        Self { installers }
    }

    pub fn installer_names(&self) -> Vec<(String, Option<String>)> {
        self.installers
            .iter()
            .map(|i| (i.name().to_string(), i.is_installed()))
            .collect()
    }

    pub fn install_selected(&self, indices: &[usize]) -> Vec<Result<(), ScxVoidError>> {
        let mut results = Vec::new();
        for &idx in indices {
            let installer = &self.installers[idx];
            if installer.is_installed().is_some() {
                println!("{} 已安装，跳过", installer.name());
                results.push(Ok(()));
                continue;
            }
            println!("正在安装 {}...", installer.name());
            let result = installer.install();
            match &result {
                Ok(()) => println!("{} 安装完成 ✓", installer.name()),
                Err(e) => {
                    eprintln!("{}", e);
                    if indices.last() != Some(&idx) {
                        println!("安装出错，是否继续安装剩余组件？");
                    }
                }
            }
            results.push(result);
        }
        results
    }
}
