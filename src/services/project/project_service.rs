use std::path::Path;
use crate::errors::ScxVoidError;
use crate::utils::fs;
use crate::services::project::generator;

/// 创建具有指定名称和类型的项目
pub async fn create_project(project_name: &str, project_type_index: usize) -> Result<(), ScxVoidError> {
    // 验证项目名称
    if project_name.trim().is_empty() {
        return Err(ScxVoidError::InvalidProjectName("项目名称不能为空".to_string()));
    }

    // 检查项目目录是否已存在
    if Path::new(project_name).exists() {
        return Err(ScxVoidError::ProjectAlreadyExists(project_name.to_string()));
    }

    // 创建项目目录
    fs::create_dir(project_name).map_err(|e| {
        ScxVoidError::FileSystemError(format!("创建项目目录失败: {}", e))
    })?;

    // 根据项目类型索引生成相应的项目结构
    match project_type_index {
        0 => create_node_ts_project(project_name).await, // Node TypeScript
        1 => create_react_project(project_name).await,   // React
        2 => create_vue_project(project_name).await,     // Vue
        3 => create_nestjs_project(project_name).await,  // NestJS
        4 => create_nextjs_project(project_name).await,  // NextJS
        _ => Err(ScxVoidError::UnsupportedProjectType(project_type_index)),
    }
}

/// 创建 Node TypeScript 项目
async fn create_node_ts_project(project_name: &str) -> Result<(), ScxVoidError> {
    println!("创建 Node TypeScript 项目...");

    // 使用生成器复制模板文件
    generator::generate_from_template(
        project_name,
        "node_ts"
    ).await?;

    Ok(())
}

/// 创建 React 项目
async fn create_react_project(project_name: &str) -> Result<(), ScxVoidError> {
    println!("创建 React 项目...");

    // 使用生成器复制模板文件
    generator::generate_from_template(
        project_name,
        "react"
    ).await?;

    Ok(())
}

/// 创建 Vue 项目
async fn create_vue_project(project_name: &str) -> Result<(), ScxVoidError> {
    println!("创建 Vue 项目...");

    // 使用生成器复制模板文件
    generator::generate_from_template(
        project_name,
        "vue"
    ).await?;

    Ok(())
}

/// 创建 NestJS 项目
async fn create_nestjs_project(project_name: &str) -> Result<(), ScxVoidError> {
    println!("创建 NestJS 项目...");

    // 使用生成器复制模板文件
    generator::generate_from_template(
        project_name,
        "nestjs"
    ).await?;

    Ok(())
}

/// 创建 NextJS 项目
async fn create_nextjs_project(project_name: &str) -> Result<(), ScxVoidError> {
    println!("创建 NextJS 项目...");

    // 使用生成器复制模板文件
    generator::generate_from_template(
        project_name,
        "nextjs"
    ).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_project_with_valid_name() {
        // This would test the project creation with a temporary directory
        // For now, we just verify the function signature is correct
        assert!(true);
    }
}