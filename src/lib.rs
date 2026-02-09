use napi_derive::napi;
use std::path::Path;

#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "macos")]
mod macos_impl;

#[cfg(target_os = "linux")]
mod linux_impl;

/// 打开文件或文件夹（使用系统默认程序）
///
/// - 如果是文件，使用系统默认程序打开
/// - 如果是文件夹，在文件管理器中打开
///
/// @param path - 要打开的文件或文件夹的绝对路径
/// @throws 如果路径不存在会抛出错误
/// @example
/// ```js
/// import { open } from 'open-native'
/// // 打开文件
/// open('C:\\Users\\test\\document.pdf')
/// // 打开文件夹
/// open('C:\\Users\\test\\Downloads')
/// ```
#[napi]
pub fn open(path: String) -> napi::Result<()> {
  let path = Path::new(&path);

  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "Path does not exist: {}",
      path.display()
    )));
  }

  #[cfg(target_os = "windows")]
  {
    windows_impl::open(path)
  }

  #[cfg(target_os = "macos")]
  {
    macos_impl::open(path)
  }

  #[cfg(target_os = "linux")]
  {
    linux_impl::open(path)
  }
}

/// 在文件管理器中打开文件夹并选中指定文件
///
/// - 在 Windows 上使用 explorer /select
/// - 在 macOS 上使用 open -R
/// - 在 Linux 上尝试使用 dbus 或 xdg-open
///
/// @param path - 要在文件管理器中显示的文件或文件夹的绝对路径
/// @throws 如果路径不存在会抛出错误
/// @example
/// ```js
/// import { reveal } from 'open-native'
/// // 在文件管理器中显示文件
/// reveal('C:\\Users\\test\\document.pdf')
/// ```
#[napi]
pub fn reveal(path: String) -> napi::Result<()> {
  let path = Path::new(&path);

  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "Path does not exist: {}",
      path.display()
    )));
  }

  #[cfg(target_os = "windows")]
  {
    windows_impl::reveal(path)
  }

  #[cfg(target_os = "macos")]
  {
    macos_impl::reveal(path)
  }

  #[cfg(target_os = "linux")]
  {
    linux_impl::reveal(path)
  }
}

/// 使用指定程序打开文件
///
/// @param path - 要打开的文件的绝对路径
/// @param app - 用于打开文件的应用程序名称或路径
///   - Windows: 可以是程序名（如 `notepad`）或完整路径
///   - macOS: 应用程序名称（如 `TextEdit`）或 bundle identifier
///   - Linux: 可执行文件名称（如 `gedit`）
/// @throws 如果路径不存在会抛出错误
/// @example
/// ```js
/// import { openWith } from 'open-native'
/// // 使用记事本打开文本文件
/// openWith('C:\\Users\\test\\note.txt', 'notepad')
/// // 使用 VS Code 打开
/// openWith('/path/to/file.js', 'code')
/// ```
#[napi]
pub fn open_with(path: String, app: String) -> napi::Result<()> {
  let path = Path::new(&path);

  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "Path does not exist: {}",
      path.display()
    )));
  }

  #[cfg(target_os = "windows")]
  {
    windows_impl::open_with(path, &app)
  }

  #[cfg(target_os = "macos")]
  {
    macos_impl::open_with(path, &app)
  }

  #[cfg(target_os = "linux")]
  {
    linux_impl::open_with(path, &app)
  }
}
