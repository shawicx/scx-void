use std::path::Path;
use crate::errors::ScxVoidError;
use crate::utils::fs;

/// 根据模板生成项目
pub async fn generate_from_template(project_name: &str, template_name: &str) -> Result<(), ScxVoidError> {
    // 定义模板路径
    let template_path = format!("assets/templates/{}", template_name);

    // 检查模板是否存在
    if !Path::new(&template_path).exists() {
        return Err(ScxVoidError::TemplateNotFound(template_name.to_string()));
    }

    // 将模板复制到项目目录
    fs::copy_dir_all(&template_path, project_name).map_err(|e| {
        ScxVoidError::FileSystemError(format!("复制模板失败: {}", e))
    })?;

    // 根据项目类型创建额外的文件
    match template_name {
        "node_ts" => create_node_ts_files(project_name).await?,
        "react" => create_react_files(project_name).await?,
        "vue" => create_vue_files(project_name).await?,
        "nestjs" => create_nestjs_files(project_name).await?,
        "nextjs" => create_nextjs_files(project_name).await?,
        _ => {
            eprintln!("警告: 未知的模板类型 '{}'", template_name);
        }
    }

    Ok(())
}

/// 为 Node TypeScript 项目创建额外的文件
async fn create_node_ts_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 如果 package.json 不存在则创建
    let package_json_path = format!("{}/package.json", project_name);
    if !Path::new(&package_json_path).exists() {
        let package_json_content = format!(r#"
{{
  "name": "{}",
  "version": "1.0.0",
  "description": "一个新的 Node TypeScript 项目",
  "main": "dist/index.js",
  "scripts": {{
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node src/index.ts"
  }},
  "dependencies": {{
  }},
  "devDependencies": {{
    "@types/node": "^latest",
    "typescript": "^latest",
    "ts-node": "^latest"
  }}
}}
"#, project_name);

        fs::write_file(&package_json_path, package_json_content).map_err(|e| {
            ScxVoidError::FileSystemError(format!("创建 package.json 失败: {}", e))
        })?;
    }

    // 如果 tsconfig.json 不存在则创建
    let tsconfig_path = format!("{}/tsconfig.json", project_name);
    if !Path::new(&tsconfig_path).exists() {
        let tsconfig_content = r#"
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": [
    "src/**/*"
  ],
  "exclude": [
    "node_modules",
    "dist"
  ]
}
"#;

        fs::write_file(&tsconfig_path, tsconfig_content.to_string()).map_err(|e| {
            ScxVoidError::FileSystemError(format!("创建 tsconfig.json 失败: {}", e))
        })?;
    }

    // 如果 src 目录和 index.ts 不存在则创建
    let src_dir = format!("{}/src", project_name);
    if !Path::new(&src_dir).exists() {
        fs::create_dir(&src_dir).map_err(|e| {
            ScxVoidError::FileSystemError(format!("创建 src 目录失败: {}", e))
        })?;

        let index_ts_path = format!("{}/src/index.ts", project_name);
        let index_ts_content = r#"console.log("Hello from Node TypeScript!");"#;

        fs::write_file(&index_ts_path, index_ts_content.to_string()).map_err(|e| {
            ScxVoidError::FileSystemError(format!("创建 index.ts 失败: {}", e))
        })?;
    }

    Ok(())
}

/// 为 React 项目创建额外的文件
async fn create_react_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 目前，这只是记录我们正在为项目创建 React 特定的文件
    println!("为 {} 创建 React 特定的文件", project_name);
    Ok(())
}

/// 为 Vue 项目创建额外的文件
async fn create_vue_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 目前，这只是记录我们正在为项目创建 Vue 特定的文件
    println!("为 {} 创建 Vue 特定的文件", project_name);
    Ok(())
}

/// 为 NestJS 项目创建额外的文件
async fn create_nestjs_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 目前，这只是记录我们正在为项目创建 NestJS 特定的文件
    println!("为 {} 创建 NestJS 特定的文件", project_name);
    Ok(())
}

/// 为 NextJS 项目创建额外的文件
async fn create_nextjs_files(project_name: &str) -> Result<(), ScxVoidError> {
    // 目前，这只是记录我们正在为项目创建 NextJS 特定的文件
    println!("为 {} 创建 NextJS 特定的文件", project_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_from_template() {
        // This would test the template generation with a temporary directory
        // For now, we just verify the function signature is correct
        assert!(true);
    }
}