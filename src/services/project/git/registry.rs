use super::types::{GitTemplate, ProjectType};
use std::collections::HashMap;

/// 获取所有预定义模板
pub fn get_all_templates() -> Vec<GitTemplate> {
    vec![
        GitTemplate::predefined(
            "node-ts-cli",
            "Node TypeScript CLI",
            "基于 Node.js + TypeScript 的 CLI 工具项目模板",
            "https://github.com/your-org/node-ts-cli-template.git",
            ProjectType::NodeTsCli,
        ),
        GitTemplate::predefined(
            "vue3-standard",
            "Vue 3 标准",
            "Vue 3 + TypeScript + Vite 标准项目模板",
            "https://github.com/your-org/vue3-template.git",
            ProjectType::Vue3,
        ),
        GitTemplate::predefined(
            "react-modern",
            "React 现代化",
            "React 18 + TypeScript + Vite 现代化项目模板",
            "https://github.com/your-org/react-template.git",
            ProjectType::React,
        ),
        GitTemplate::predefined(
            "nestjs-rest",
            "NestJS REST API",
            "NestJS RESTful API 项目模板",
            "https://github.com/your-org/nestjs-template.git",
            ProjectType::NestJs,
        ),
        GitTemplate::predefined(
            "nextjs-app",
            "NextJS App Router",
            "NextJS 14 + App Router + TypeScript 项目模板",
            "https://github.com/your-org/nextjs-template.git",
            ProjectType::NextJs,
        ),
    ]
}

/// 根据 ID 获取模板
pub fn get_template_by_id(id: &str) -> Option<GitTemplate> {
    let templates = get_all_templates();
    templates.into_iter().find(|t| t.id == id)
}

/// 检查模板 ID 是否存在
pub fn template_exists(id: &str) -> bool {
    get_template_by_id(id).is_some()
}

/// 获取模板 ID 到模板的映射
pub fn get_template_map() -> HashMap<String, GitTemplate> {
    let templates = get_all_templates();
    templates
        .into_iter()
        .map(|t| (t.id.clone(), t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_templates() {
        let templates = get_all_templates();
        assert_eq!(templates.len(), 5);
    }

    #[test]
    fn test_get_template_by_id() {
        let template = get_template_by_id("node-ts-cli");
        assert!(template.is_some());
        assert_eq!(template.unwrap().display_name, "Node TypeScript CLI");
    }

    #[test]
    fn test_get_template_by_id_not_found() {
        let template = get_template_by_id("non-existent");
        assert!(template.is_none());
    }

    #[test]
    fn test_template_exists() {
        assert!(template_exists("node-ts-cli"));
        assert!(template_exists("vue3-standard"));
        assert!(!template_exists("non-existent"));
    }

    #[test]
    fn test_get_template_map() {
        let map = get_template_map();
        assert_eq!(map.len(), 5);
        assert!(map.contains_key("node-ts-cli"));
        assert!(map.contains_key("vue3-standard"));
        assert!(map.contains_key("react-modern"));
        assert!(map.contains_key("nestjs-rest"));
        assert!(map.contains_key("nextjs-app"));
    }
}
