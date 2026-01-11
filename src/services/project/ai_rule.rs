use crate::errors::ScxVoidError;
use crate::utils::fs;
use std::path::Path;

/// Ai Code 规则文件管理服务
pub struct AiRuleService;

impl AiRuleService {
    /// 创建新的服务实例
    pub fn new() -> Self {
        Self
    }

    /// 管理 Ai Code 规则文件
    /// template_type: 模板类型 ("basic" 或 "advanced")
    /// force: 是否强制覆盖现有文件
    pub async fn manage_rule_file(
        &self,
        template_type: &str,
        force: bool,
    ) -> Result<(), ScxVoidError> {
        let rule_file_path = "AGENTS.md";

        // 先验证模板类型
        let content = self.get_template_content(template_type)?;

        // 检查文件是否已存在
        if Path::new(rule_file_path).exists() && !force {
            return Err(ScxVoidError::AiRuleFileExists(rule_file_path.into()));
        }

        // 写入文件
        fs::write_file(rule_file_path, content.clone())
            .map_err(|e| ScxVoidError::FileSystemError(e.to_string()))?;

        println!("✓ Ai Code 规则文件已生成: {}", rule_file_path);

        Ok(())
    }

    /// 根据模板类型获取内容
    fn get_template_content(&self, template_type: &str) -> Result<String, ScxVoidError> {
        match template_type {
            "advanced" => Ok(self.get_advanced_template()),
            "basic" => Ok(self.get_basic_template()),
            _ => Err(ScxVoidError::InvalidTemplateType(template_type.to_string())),
        }
    }

    /// 获取高级模板内容
    fn get_advanced_template(&self) -> String {
        include_str!("../../../assets/templates/ai_rule/advanced.md").to_string()
    }

    /// 获取基础模板内容
    fn get_basic_template(&self) -> String {
        include_str!("../../../assets/templates/ai_rule/basic.md").to_string()
    }

    /// 验证现有规则文件状态
    pub fn validate_existing_file(&self) -> Result<bool, ScxVoidError> {
        let rule_file_path = "AGENTS.md";

        if !Path::new(rule_file_path).exists() {
            return Ok(false);
        }

        // 可以添加更多验证逻辑，比如检查文件格式等
        Ok(true)
    }

    /// 备份现有规则文件
    pub async fn backup_existing_file(&self) -> Result<(), ScxVoidError> {
        let rule_file_path = "AGENTS.md";
        let backup_path = "AGENTS.md.backup";

        if Path::new(rule_file_path).exists() {
            fs::copy_file(rule_file_path, backup_path)
                .map_err(|e| ScxVoidError::FileSystemError(e.to_string()))?;
            println!("✓ 现有规则文件已备份: {}", backup_path);
        }

        Ok(())
    }
}

impl Default for AiRuleService {
    fn default() -> Self {
        Self::new()
    }
}

/// 公共接口函数，供 CLI 层调用
pub async fn manage_ai_rule_file(template_type: &str, force: bool) -> Result<(), ScxVoidError> {
    let service = AiRuleService::new();

    // 如果不强制覆盖，先检查现有文件
    if !force && service.validate_existing_file()? {
        return Err(ScxVoidError::AiRuleFileExists("AGENTS.md".into()));
    }

    // 如果使用 force 并且文件已存在，先创建备份
    if force && Path::new("AGENTS.md").exists() {
        service.backup_existing_file().await?;
    }

    service.manage_rule_file(template_type, force).await
}
