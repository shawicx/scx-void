use std::fs::{self, File};
use std::io::Write;

/// 在指定路径创建目录
pub fn create_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

/// 将内容写入文件
pub fn write_file(path: &str, content: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 将一个目录的所有内容复制到另一个目录
pub fn copy_dir_all(src: &str, dst: &str) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir_all(
                entry.path().to_str().unwrap(),
                &format!("{}/{}", dst, entry.file_name().to_str().unwrap()),
            )?;
        } else {
            fs::copy(
                entry.path(),
                format!("{}/{}", dst, entry.file_name().to_str().unwrap()),
            )?;
        }
    }

    Ok(())
}

/// 复制单个文件
pub fn copy_file(src: &str, dst: &str) -> std::io::Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::fs;
    // use std::path::Path;

    // Temporarily disabling tests that require tempfile
    // #[test]
    // fn test_create_dir() {
    //     let temp_dir = TempDir::new().unwrap();
    //     let test_path = format!("{}/test_dir", temp_dir.path().to_string_lossy());

    //     assert!(create_dir(&test_path).is_ok());
    //     assert!(Path::new(&test_path).exists());
    // }
}
