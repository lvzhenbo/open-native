use std::path::Path;
use std::process::Command;

/// 打开文件或文件夹
pub fn open(path: &Path) -> napi::Result<()> {
  // 尝试使用 xdg-open
  let status = Command::new("xdg-open")
    .arg(path)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute xdg-open: {}", e)))?;

  if status.success() {
    Ok(())
  } else {
    Err(napi::Error::from_reason(format!(
      "Failed to open path: {}",
      path.display()
    )))
  }
}

/// 在文件管理器中显示并选中文件
pub fn reveal(path: &Path) -> napi::Result<()> {
  // 首先尝试使用 dbus 调用 org.freedesktop.FileManager1
  let path_str = path.to_string_lossy();
  let file_uri = if path_str.starts_with("file://") {
    path_str.to_string()
  } else {
    format!(
      "file://{}",
      path.canonicalize().unwrap_or(path.to_path_buf()).display()
    )
  };

  let dbus_result = Command::new("dbus-send")
    .args([
      "--session",
      "--dest=org.freedesktop.FileManager1",
      "--type=method_call",
      "/org/freedesktop/FileManager1",
      "org.freedesktop.FileManager1.ShowItems",
      &format!("array:string:{}", file_uri),
      "string:",
    ])
    .status();

  if let Ok(status) = dbus_result {
    if status.success() {
      return Ok(());
    }
  }

  // 如果 dbus 失败，尝试使用不同的文件管理器
  // Nautilus (GNOME)
  if Command::new("nautilus")
    .arg("--select")
    .arg(path)
    .status()
    .map(|s| s.success())
    .unwrap_or(false)
  {
    return Ok(());
  }

  // Dolphin (KDE)
  if Command::new("dolphin")
    .arg("--select")
    .arg(path)
    .status()
    .map(|s| s.success())
    .unwrap_or(false)
  {
    return Ok(());
  }

  // Nemo (Cinnamon)
  if Command::new("nemo")
    .arg(path)
    .status()
    .map(|s| s.success())
    .unwrap_or(false)
  {
    return Ok(());
  }

  // 最后回退到 xdg-open 打开父文件夹
  let parent = path.parent().unwrap_or(path);
  let status = Command::new("xdg-open")
    .arg(parent)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute xdg-open: {}", e)))?;

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
  let status = Command::new(app)
    .arg(path)
    .status()
    .map_err(|e| napi::Error::from_reason(format!("Failed to execute {}: {}", app, e)))?;

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
