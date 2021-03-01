use crate::hotkey;
use crate::keyboard;
use crate::mouse;
use crate::window;
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn kb_input(keys: *const c_char) -> c_int {
  let s = unsafe { std::ffi::CStr::from_ptr(keys).to_str().unwrap() };
  match keyboard::input(s) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn wait_keys_up(keys: *const c_char) {
  let s = unsafe { std::ffi::CStr::from_ptr(keys).to_str().unwrap() };
  keyboard::wait_keys_up(s);
}

#[no_mangle]
pub extern "C" fn r#move(x: c_int, y: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::Move(x, y)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn move_to(x: c_int, y: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::MoveTo(x, y)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn click(button: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::Click(button as _)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn press(button: c_int, up: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::Press(button as _, up != 0)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn wheel(amount: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::Wheel(amount)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

#[no_mangle]
pub extern "C" fn hwheel(amount: c_int) -> c_int {
  match mouse::input(&mouse::MouseAction::HWheel(amount)) {
    Ok(()) => 0,
    Err(e) => {
      println!("{}", e.description());
      1
    }
  }
}

// if success return hotkey id, otherwise return 0
#[no_mangle]
pub extern "C" fn hotkey_register(hotkey: *const c_char) -> c_int {
  let s = unsafe { std::ffi::CStr::from_ptr(hotkey).to_str().unwrap() };
  match hotkey::register(s) {
    Ok(hotkey_id) => hotkey_id,
    Err(e) => {
      println!("{}", e.description());
      0
    }
  }
}

#[no_mangle]
pub extern "C" fn hotkey_unregister(hotkey_id: c_int) {
  hotkey::unregister(hotkey_id);
}

// if success return hotkey id, otherwise return 0
#[no_mangle]
pub extern "C" fn hotkey_wait() -> c_int {
  match hotkey::wait() {
    Ok(hotkey_id) => hotkey_id,
    Err(e) => {
      println!("{}", e.description());
      0
    }
  }
}

// Sleep milliseconds
#[no_mangle]
pub extern "C" fn sleep(ms: i32) {
  std::thread::sleep(std::time::Duration::from_millis(ms as _));
}

#[no_mangle]
pub extern "C" fn window_find(cls: *const c_char, caption: *const c_char) -> isize {
  let cls_slice = unsafe { std::ffi::CStr::from_ptr(cls).to_str().unwrap() };
  let caption_slice = unsafe { std::ffi::CStr::from_ptr(caption).to_str().unwrap() };
  window::find(cls_slice, caption_slice)
}

#[no_mangle]
pub extern "C" fn window_pos(hwnd: isize, x: *mut i32, y: *mut i32) -> c_int {
  match window::get_pos(hwnd) {
    Some((retx, rety)) => unsafe {
      if x != std::ptr::null_mut() {
        *x = retx;
      }
      if y != std::ptr::null_mut() {
        *y = rety;
      }
      return 0;
    }
    None => {
      return 1;
    }
  }
}

#[no_mangle]
pub extern "C" fn window_size(hwnd: isize, x: *mut i32, y: *mut i32) -> c_int {
  match window::get_size(hwnd) {
    Some((retx, rety)) => unsafe {
      if x != std::ptr::null_mut() {
        *x = retx;
      }
      if y != std::ptr::null_mut() {
        *y = rety;
      }
      return 0;
    }
    None => {
      return 1;
    }
  }
}
