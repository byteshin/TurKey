use crate::bindings::windows::win32::display_devices::RECT;
use crate::bindings::windows::win32::keyboard_and_mouse_input as km;
use crate::bindings::windows::win32::menus_and_resources as mr;
use crate::bindings::windows::win32::system_services as ss;
use crate::bindings::windows::win32::windows_and_messaging as wm;
use crate::bindings::windows::FALSE;
use crate::errors::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

// Find window with class name and caption
// cls/caption can be empty str when ignore
// Return 0 if not found or error
pub fn find(cls: &str, caption: &str) -> isize {
  let cls_wide = OsStr::new(cls)
    .encode_wide()
    .chain(Some(0))
    .collect::<Vec<_>>();
  let caption_wide = OsStr::new(caption)
    .encode_wide()
    .chain(Some(0))
    .collect::<Vec<_>>();

  let hwnd = unsafe {
    wm::FindWindowW(
      if cls.len() == 0 {
        std::ptr::null_mut()
      } else {
        cls_wide.as_ptr()
      },
      if caption.len() == 0 {
        std::ptr::null_mut()
      } else {
        caption_wide.as_ptr()
      },
    )
  };

  hwnd.0
}

// Get window upper-left corner position
pub fn get_pos(hwnd: isize) -> Option<(i32, i32)> {
  let mut rect: RECT = unsafe { std::mem::zeroed() };
  let ret = unsafe { wm::GetWindowRect(wm::HWND(hwnd), &mut rect) };
  if ret == FALSE {
    None
  } else {
    Some((rect.left, rect.top))
  }
}

// Get window upper-left corner position
pub fn get_size(hwnd: isize) -> Option<(i32, i32)> {
  let mut rect: RECT = unsafe { std::mem::zeroed() };
  let ret = unsafe { wm::GetWindowRect(wm::HWND(hwnd), &mut rect) };
  if ret == FALSE {
    None
  } else {
    Some((rect.right - rect.left, rect.bottom - rect.top))
  }
}

#[repr(C)]
struct KeyboardRawInput {
  header: km::RAWINPUTHEADER,
  keyboard: km::RAWKEYBOARD,
}

extern "system" fn WindProc(
  h_wnd: wm::HWND,
  msg: u32,
  w_param: wm::WPARAM,
  l_param: wm::LPARAM,
) -> ss::LRESULT {
  if msg == ss::WM_INPUT as _ {
    println!(
      "GOT message {} w_param {} l_param {}",
      msg, w_param.0, l_param.0
    );
    let mut kri: KeyboardRawInput = unsafe { std::mem::zeroed() };
    let mut kri_size: u32 = std::mem::size_of::<KeyboardRawInput>() as _;
    let ret = unsafe {
      km::GetRawInputData(
        l_param.0,
        0x10000003, /* RID_INPUT */
        std::mem::transmute(&mut kri),
        &mut kri_size,
        std::mem::size_of::<km::RAWINPUTHEADER>() as _,
      )
    };
    if ret as i32 == -1 {
      println!(
        "GetRawInputData failed. {:?}",
        std::io::Error::last_os_error()
      );
    }
    println!(
      "Keyboard VK:{}, Flags:{}",
      kri.keyboard.vkey, kri.keyboard.flags
    );
  }
  unsafe { wm::DefWindowProcA(h_wnd, msg, w_param, l_param) }
}

pub fn log_keys() -> Result<()> {
  let mut class_name = String::from("Test");
  let mut wcx: wm::WNDCLASSEXA = unsafe { std::mem::zeroed() };
  wcx.cb_size = std::mem::size_of::<wm::WNDCLASSEXA>() as _;
  wcx.lpfn_wnd_proc = Some(WindProc);
  wcx.h_instance = unsafe { ss::HINSTANCE(ss::GetModuleHandleA(std::ptr::null_mut())) };
  wcx.lpsz_class_name = class_name.as_mut_ptr() as _;

  let ret = unsafe { wm::RegisterClassExA(&wcx) };
  if ret == 0u16 {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "RegisterClassEx failed."
    ));
  }

  let hwnd = unsafe {
    wm::CreateWindowExA(
      0,
      wcx.lpsz_class_name,
      std::ptr::null_mut(),
      0,
      0,
      0,
      0,
      0,
      wm::HWND(ss::HWND_MESSAGE as _),
      mr::HMENU(0),
      wcx.h_instance,
      std::ptr::null_mut(),
    )
  };

  if hwnd.0 == 0 {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "CreateWindowEx failed."
    ));
  }

  let mut rids: [km::RAWINPUTDEVICE; 2] = unsafe { std::mem::zeroed() };
  rids[0].us_usage_page = 1;
  rids[0].us_usage = 6;
  rids[0].dw_flags = 0x100;
  rids[0].hwnd_target = hwnd;

  let ret = unsafe {
    km::RegisterRawInputDevices(
      rids.as_mut_ptr() as _,
      1,
      std::mem::size_of::<km::RAWINPUTDEVICE>() as _,
    )
  };
  if ret == FALSE {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "RegisterRawInputDevices failed."
    ));
  }

  println!("{}", ret.0);

  let mut msg: wm::MSG = unsafe { std::mem::zeroed() };

  while unsafe { wm::GetMessageA(&mut msg, wm::HWND(0), 0, 0) } != FALSE {
    unsafe {
      wm::TranslateMessage(&mut msg);
      wm::DispatchMessageA(&mut msg);
    }
  }

  Ok(())
}

#[test]
fn test() {
  // println!("{}", find("", "中文.log - Notepad"));
  println!("{}", find("Notepad", ""));
  let ret = log_keys();
}
