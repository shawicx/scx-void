use crate::errors::ScxVoidError;
use crate::services::project::git::types::ProjectType;
use crate::utils::fs;
use std::path::Path;

/// Ai Code 规则文件管理服务
pub struct AiRuleService;

impl AiRuleService {
    /// 创建新的服务实例
    pub fn new() -> Self {
        Self
    }

    /// 渲染规则内容（纯函数，与文件系统无关）
    /// 由 base 通用段 + 技术栈专属段拼装
    pub fn render(&self, project_type: ProjectType) -> Result<String, ScxVoidError> {
        let base = include_str!("../../../assets/templates/ai_rule/base.md");
        let stack = match project_type {
            ProjectType::Vue3 => include_str!("../../../assets/templates/ai_rule/vue3.md"),
            ProjectType::React => include_str!("../../../assets/templates/ai_rule/react.md"),
            ProjectType::NextJs => include_str!("../../../assets/templates/ai_rule/nextjs.md"),
            ProjectType::NodeTsCli => {
                include_str!("../../../assets/templates/ai_rule/node-cli.md")
            }
            ProjectType::NestJs => include_str!("../../../assets/templates/ai_rule/nestjs.md"),
            ProjectType::Tauri => include_str!("../../../assets/templates/ai_rule/tauri.md"),
            ProjectType::Java => include_str!("../../../assets/templates/ai_rule/java.md"),
        };
        Ok(format!("{}\n\n---\n\n{}", base, stack))
    }

    /// 写入规则文件到指定路径（只负责渲染+写入，不做存在性/备份判断）
    pub async fn manage_rule_file(
        &self,
        project_type: ProjectType,
        target_path: &Path,
    ) -> Result<(), ScxVoidError> {
        let content = self.render(project_type)?;
        let path_str = target_path
            .to_str()
            .ok_or_else(|| ScxVoidError::FileSystemError("目标路径包含非法字符".into()))?;

        fs::write_file(path_str, content)
            .map_err(|e| ScxVoidError::FileSystemError(e.to_string()))?;

        println!("✓ Ai Code 规则文件已生成: {}", target_path.display());
        Ok(())
    }

    /// 验证指定路径的规则文件是否存在
    #[allow(dead_code)]
    pub fn validate_existing_file(&self, path: &Path) -> Result<bool, ScxVoidError> {
        Ok(path.exists())
    }

    /// 备份指定路径的规则文件（追加 .backup 后缀）
    pub async fn backup_existing_file(&self, path: &Path) -> Result<(), ScxVoidError> {
        if !path.exists() {
            return Ok(());
        }

        let src = path
            .to_str()
            .ok_or_else(|| ScxVoidError::FileSystemError("源路径包含非法字符".into()))?;
        let backup_path = format!("{}.backup", src);

        fs::copy_file(src, &backup_path)
            .map_err(|e| ScxVoidError::FileSystemError(e.to_string()))?;

        println!("✓ 现有规则文件已备份: {}", backup_path);
        Ok(())
    }
}

impl Default for AiRuleService {
    fn default() -> Self {
        Self::new()
    }
}

/// CLI 入口：默认写入当前目录的 AGENTS.md
/// 职责：存在性检查 + 备份 + 调用底层写入
pub async fn manage_ai_rule_file(
    project_type: ProjectType,
    force: bool,
) -> Result<(), ScxVoidError> {
    let service = AiRuleService::new();
    let path = Path::new("AGENTS.md");

    // 不强制覆盖时，先检查文件是否已存在
    if !force && path.exists() {
        return Err(ScxVoidError::AiRuleFileExists(path.to_path_buf()));
    }

    // 强制覆盖且文件已存在时，先备份
    if force && path.exists() {
        service.backup_existing_file(path).await?;
    }

    service.manage_rule_file(project_type, path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_combines_base_and_stack() {
        let service = AiRuleService::new();
        let content = service.render(ProjectType::Vue3).unwrap();

        // 必须包含 base 段的核心标题
        assert!(content.contains("核心原则"));
        assert!(content.contains("依赖管理"));
        assert!(content.contains("禁止事项"));

        // 必须包含 Vue3 段的标题
        assert!(content.contains("Vue 3 项目规则"));
        assert!(content.contains("Vue Router"));
        assert!(content.contains("Pinia"));
    }

    #[test]
    fn test_render_each_stack() {
        let service = AiRuleService::new();
        let stacks = [
            ProjectType::Vue3,
            ProjectType::React,
            ProjectType::NextJs,
            ProjectType::NodeTsCli,
            ProjectType::NestJs,
            ProjectType::Tauri,
            ProjectType::Java,
        ];

        for stack in stacks {
            let content = service.render(stack);
            assert!(content.is_ok(), "render({:?}) 失败", stack);
            let content = content.unwrap();
            assert!(!content.is_empty());
            // base 段必须出现在所有技术栈输出中
            assert!(content.contains("核心原则"));
        }
    }

    #[test]
    fn test_render_separator() {
        let service = AiRuleService::new();
        let content = service.render(ProjectType::React).unwrap();
        // base 与 stack 之间必须用分隔符拼接
        assert!(content.contains("\n\n---\n\n"));
    }

    #[test]
    fn test_render_stack_specific_content() {
        let service = AiRuleService::new();

        // Tauri 必须提到 security 字段
        let tauri = service.render(ProjectType::Tauri).unwrap();
        assert!(tauri.contains("security") || tauri.contains("capabilities"));

        // Java 必须提到 Maven 或 Gradle
        let java = service.render(ProjectType::Java).unwrap();
        assert!(java.contains("Maven") || java.contains("Gradle"));

        // node-cli 必须提到 ESM
        let cli = service.render(ProjectType::NodeTsCli).unwrap();
        assert!(cli.contains("ESM"));
    }
}
