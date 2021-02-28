use crate::bindings::windows::win32::data_exchange::GlobalAddAtomA;
use crate::bindings::windows::win32::keyboard_and_mouse_input as km;
use crate::bindings::windows::win32::windows_and_messaging::GetMessageA;
use crate::bindings::windows::win32::windows_and_messaging::HWND;
use crate::bindings::windows::win32::windows_and_messaging::MSG;
use crate::bindings::windows::FALSE;
use crate::errors::*;
use crate::keyboard;

const MOD_NOREPEAT: u32 = 0x4000;
const WM_HOTKEY: u32 = 786;

// Register hotkey and return hotkey id
pub fn register(hotkey: &str) -> Result<i32> {
  let atom = unsafe { GlobalAddAtomA(hotkey.as_ptr() as _) };
  if atom == 0 {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "GlobalAddAtom failed."
    ));
  }

  let mut modifiers: u32 = 0;
  let mut vk: u32 = 0;

  let keys = keyboard::parse(hotkey)?;
  let mut main_key_found = false;
  let mut key_up_found = false;
  for key in keys {
    if key.up && !key_up_found {
      key_up_found = true;
    }
    if key_up_found && !key.up {
      bail!("Invalid hotkey")
    }
    if !key.up && !keyboard::is_modifier(key.code) {
      println!("input:{:?}, main_key_found:{}", key, main_key_found);
      if main_key_found {
        bail!("Invalid hotkey")
      }
      main_key_found = true;
      modifiers = key.state.into();
      vk = key.code.into();
    }
  }

  modifiers |= MOD_NOREPEAT;

  let ret = unsafe { km::RegisterHotKey(HWND(0), atom as _, modifiers, vk) };
  if ret == FALSE {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "RegisterHotKey failed."
    ));
  }

  Ok(atom.into())
}

pub fn unregister(id: i32) {
  unsafe { km::UnregisterHotKey(HWND(0), id) };
}

pub fn wait() -> Result<i32> {
  let mut msg: MSG = unsafe { std::mem::zeroed() };
  while unsafe { GetMessageA(&mut msg, HWND(0), 0, 0) } != FALSE {
    if msg.message == WM_HOTKEY {
      return Ok(msg.w_param.0 as _);
    }
  }
  bail!("Unknown")
}

#[test]
fn test() {
  // println!("Start test...");
  // let id = register("<ctrl+alt+f1>").unwrap();
  // println!("Registered id:{}", id);
  // let ret = wait().unwrap();
  // println!("Waited id:{}", id);
  // unregister(id);
  // println!("Finish");
}
