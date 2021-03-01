use crate::errors::*;
use crate::bindings::windows::win32::windows_and_messaging as wm;
use crate::bindings::windows::win32::display_devices::RECT;
use crate::bindings::windows::win32::windows_and_messaging::HWND;
use crate::bindings::windows::FALSE;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

// Find window with class name and caption
// cls/caption can be empty str when ignore
// Return 0 if not found or error
pub fn find(cls: &str, caption: &str) -> isize {
  let cls_wide = OsStr::new(cls).encode_wide().chain(Some(0)).collect::<Vec<_>>();
  let caption_wide = OsStr::new(caption).encode_wide().chain(Some(0)).collect::<Vec<_>>();

  let hwnd = unsafe {
    wm::FindWindowW(
      if cls.len() == 0 { std::ptr::null_mut() } else { cls_wide.as_ptr() }, 
      if caption.len() == 0 { std::ptr::null_mut() } else { caption_wide.as_ptr() }
    )
  };

  hwnd.0
}

// Get window upper-left corner position
pub fn get_pos(hwnd: isize) -> Option<(i32, i32)> {
  let mut rect: RECT = unsafe { std::mem::zeroed() };
  let ret = unsafe { wm::GetWindowRect(HWND(hwnd), &mut rect) };
  if ret == FALSE {
    None
  } else {
    Some((rect.left, rect.top))
  }
}

// Get window upper-left corner position
pub fn get_size(hwnd: isize) -> Option<(i32, i32)> {
  let mut rect: RECT = unsafe { std::mem::zeroed() };
  let ret = unsafe { wm::GetWindowRect(HWND(hwnd), &mut rect) };
  if ret == FALSE {
    None
  } else {
    Some((rect.right - rect.left, rect.bottom - rect.top))
  }
}

#[test]
fn test() {
  // println!("{}", find("", "中文.log - Notepad"));
  println!("{}", find("Notepad", ""));
}
