use crate::bindings::windows::win32::keyboard_and_mouse_input as km;
use crate::errors::*;

const BUTTON_LEFT: u8 = 1;
const BUTTON_RIGHT: u8 = 2;
const BUTTON_MIDDLE: u8 = 3;
const BUTTON_X1: u8 = 4;
const BUTTON_X2: u8 = 5;

const INPUT_MOUSE: u32 = 0;
const MOUSEEVENTF_MOVE: u32 = 0x0001;
const MOUSEEVENTF_ABSOLUTE: u32 = 0x8000;
const MOUSEEVENTF_LEFTDOWN: u32 = 0x0002;
const MOUSEEVENTF_LEFTUP: u32 = 0x0004;
const MOUSEEVENTF_RIGHTDOWN: u32 = 0x0008;
const MOUSEEVENTF_RIGHTUP: u32 = 0x0010;
const MOUSEEVENTF_MIDDLEDOWN: u32 = 0x0020;
const MOUSEEVENTF_MIDDLEUP: u32 = 0x0040;
const MOUSEEVENTF_XDOWN: u32 = 0x0080;
const MOUSEEVENTF_XUP: u32 = 0x0100;
const MOUSEEVENTF_WHEEL: u32 = 0x0800;
const MOUSEEVENTF_HWHEEL: u32 = 0x01000;
const XBUTTON1: u32 = 1;
const XBUTTON2: u32 = 2;

pub enum MouseAction {
  Move(i32, i32),
  MoveTo(i32, i32),
  // button
  Click(u8),
  // button, up
  Press(u8, bool),
  Wheel(i32),
  HWheel(i32),
}

#[repr(C)]
#[derive(Debug, Clone)]
struct MouseInput {
  type_: u32,
  mi: km::MOUSEINPUT,
}

pub fn input(action: &MouseAction) -> Result<()> {
  let mut mi: MouseInput = unsafe { std::mem::zeroed() };
  mi.type_ = INPUT_MOUSE;
  match *action {
    MouseAction::Move(x, y) => {
      mi.mi.dx = x;
      mi.mi.dy = y;
      mi.mi.dw_flags = MOUSEEVENTF_MOVE;
    }
    MouseAction::MoveTo(x, y) => {
      mi.mi.dx = x;
      mi.mi.dy = y;
      mi.mi.dw_flags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
    }
    MouseAction::Click(button) => match button {
      BUTTON_LEFT => {
        mi.mi.dw_flags = MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP;
      }
      BUTTON_RIGHT => {
        mi.mi.dw_flags = MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_RIGHTUP;
      }
      BUTTON_MIDDLE => {
        mi.mi.dw_flags = MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MIDDLEUP;
      }
      BUTTON_X1 => {
        mi.mi.dw_flags = MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP;
        mi.mi.mouse_data = XBUTTON1;
      }
      BUTTON_X2 => {
        mi.mi.dw_flags = MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP;
        mi.mi.mouse_data = XBUTTON2;
      }
      _ => bail!("Invalid button"),
    },
    MouseAction::Press(button, up) => match button {
      BUTTON_LEFT => {
        mi.mi.dw_flags = if !up {
          MOUSEEVENTF_LEFTDOWN
        } else {
          MOUSEEVENTF_LEFTUP
        };
      }
      BUTTON_RIGHT => {
        mi.mi.dw_flags = if !up {
          MOUSEEVENTF_RIGHTDOWN
        } else {
          MOUSEEVENTF_RIGHTUP
        };
      }
      BUTTON_MIDDLE => {
        mi.mi.dw_flags = if !up {
          MOUSEEVENTF_MIDDLEDOWN
        } else {
          MOUSEEVENTF_MIDDLEUP
        };
      }
      BUTTON_X1 => {
        mi.mi.dw_flags = if !up {
          MOUSEEVENTF_XDOWN
        } else {
          MOUSEEVENTF_XUP
        };
        mi.mi.mouse_data = XBUTTON1;
      }
      BUTTON_X2 => {
        mi.mi.dw_flags = if !up {
          MOUSEEVENTF_XDOWN
        } else {
          MOUSEEVENTF_XUP
        };
        mi.mi.mouse_data = XBUTTON2;
      }
      _ => bail!("Invalid button"),
    },
    MouseAction::Wheel(amount) => {
      mi.mi.dw_flags = MOUSEEVENTF_WHEEL;
      mi.mi.mouse_data = amount as _;
    }
    MouseAction::HWheel(amount) => {
      mi.mi.dw_flags = MOUSEEVENTF_HWHEEL;
      mi.mi.mouse_data = amount as _;
    }
  }

  let sent = unsafe {
    km::SendInput(
      1u32,
      std::mem::transmute::<*mut MouseInput, *mut km::INPUT>(&mut mi),
      std::mem::size_of::<MouseInput>() as _,
    )
  };
  if sent != 1 as _ {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "SendInput failed."
    ));
  }
  Ok(())
}

#[test]
fn test() {
  // input(&MouseAction::Click(BUTTON_RIGHT)).unwrap();
  // input(&MouseAction::Move(100, 100)).unwrap();
  // input(&MouseAction::MoveTo(0, 0)).unwrap();
  // input(&MouseAction::Press(BUTTON_RIGHT, false)).unwrap();
  // input(&MouseAction::Press(BUTTON_RIGHT, true)).unwrap();
  // input(&MouseAction::Wheel(100)).unwrap();
}
