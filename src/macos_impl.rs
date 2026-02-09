use std::path::Path;
use std::process::Command;

/// 打开文件或文件夹
pub fn open(path: &Path) -> napi::Result<()> {
  let status = Command::new("open")
    .arg(path)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute open command: {}", e)))?;

  if status.success() {
    Ok(())
  } else {
    Err(napi::Error::from_reason(format!(
      "Failed to open path: {}",
      path.display()
    )))
  }
}

/// 在 Finder 中显示并选中文件
pub fn reveal(path: &Path) -> napi::Result<()> {
  let status = Command::new("open")
    .arg("-R")
    .arg(path)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute open command: {}", e)))?;

  if status.success() {
    Ok(())
  } else {
    Err(napi::Error::from_reason(format!(
      "Failed to reveal path: {}",
      path.display()
    )))
  }
}

/// 使用指定程序打开文件
pub fn open_with(path: &Path, app: &str) -> napi::Result<()> {
  let status = Command::new("open")
    .arg("-a")
    .arg(app)
    .arg(path)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute open command: {}", e)))?;

  if status.success() {
    Ok(())
  } else {
    Err(napi::Error::from_reason(format!(
      "Failed to open {} with {}",
      path.display(),
      app
    )))
  }
}
