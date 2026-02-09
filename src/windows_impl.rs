use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::Shell::{
  Common::ITEMIDLIST, ILFree, SHOpenFolderAndSelectItems, SHParseDisplayName, ShellExecuteW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::core::{PCWSTR, w};

fn to_wide_string(s: &OsStr) -> Vec<u16> {
  s.encode_wide().chain(std::iter::once(0)).collect()
}

/// 打开文件或文件夹
pub fn open(path: &Path) -> napi::Result<()> {
  let path_wide = to_wide_string(path.as_os_str());

  unsafe {
    let result = ShellExecuteW(
      None,
      w!("open"),
      PCWSTR::from_raw(path_wide.as_ptr()),
      PCWSTR::null(),
      PCWSTR::null(),
      SW_SHOW,
    );

    // ShellExecuteW 返回值大于 32 表示成功
    if result.0 as isize > 32 {
      Ok(())
    } else {
      Err(napi::Error::from_reason(format!(
        "Failed to open path: {}",
        path.display()
      )))
    }
  }
}

/// 在资源管理器中显示并选中文件
pub fn reveal(path: &Path) -> napi::Result<()> {
  unsafe {
    // 初始化 COM
    let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

    let path_wide = to_wide_string(path.as_os_str());
    let mut pidl: *mut ITEMIDLIST = null_mut();

    let hr = SHParseDisplayName(
      PCWSTR::from_raw(path_wide.as_ptr()),
      None,
      &mut pidl,
      0,
      None,
    );

    if hr.is_err() {
      CoUninitialize();
      return Err(napi::Error::from_reason(format!(
        "Failed to parse path: {}",
        path.display()
      )));
    }

    let result = SHOpenFolderAndSelectItems(pidl as *const ITEMIDLIST, None, 0);

    ILFree(Some(pidl as *const ITEMIDLIST));
    CoUninitialize();

    if result.is_ok() {
      Ok(())
    } else {
      Err(napi::Error::from_reason(format!(
        "Failed to reveal path: {}",
        path.display()
      )))
    }
  }
}

/// 使用指定程序打开文件
pub fn open_with(path: &Path, app: &str) -> napi::Result<()> {
  let path_wide = to_wide_string(path.as_os_str());
  let app_wide = to_wide_string(OsStr::new(app));

  unsafe {
    let result = ShellExecuteW(
      None,
      w!("open"),
      PCWSTR::from_raw(app_wide.as_ptr()),
      PCWSTR::from_raw(path_wide.as_ptr()),
      PCWSTR::null(),
      SW_SHOW,
    );

    if result.0 as isize > 32 {
      Ok(())
    } else {
      Err(napi::Error::from_reason(format!(
        "Failed to open {} with {}",
        path.display(),
        app
      )))
    }
  }
}
