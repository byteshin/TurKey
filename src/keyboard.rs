use crate::bindings::windows::win32::keyboard_and_mouse_input as km;
use crate::errors::*;

const VK_SHIFT: u8 = 0x10;
const VK_CONTROL: u8 = 0x11;
const VK_MENU: u8 = 0x12;

const VK_LSHIFT: u8 = 0xA0;
const VK_RSHIFT: u8 = 0xA1;
const VK_LCONTROL: u8 = 0xA2;
const VK_RCONTROL: u8 = 0xA3;
const VK_LMENU: u8 = 0xA4;
const VK_RMENU: u8 = 0xA5;
const VK_LWIN: u8 = 0x5B;
const VK_RWIN: u8 = 0x5C;

const INPUT_KEYBOARD: u32 = 1;
const KEYEVENTF_KEYUP: u32 = 2;

static KEYNAME_CODE_MAP: &[(&str, u8)] = &[
  ("backspace", 0x08),
  ("bs", 0x08),
  ("tab", 0x09),
  ("clear", 0x0C),
  ("return", 0x0D),
  ("enter", 0x0D),
  ("shift", VK_SHIFT),
  ("ctrl", VK_CONTROL),
  ("alt", VK_MENU),
  ("pause", 0x13),
  ("caps", 0x14),
  ("kana", 0x15),
  ("hangul", 0x15),
  ("ime_on", 0x16),
  ("junja", 0x17),
  ("final", 0x18),
  ("hanja", 0x19),
  ("kanji", 0x19),
  ("ime_off", 0x1A),
  ("esc", 0x1B),
  ("convert", 0x1C),
  ("nonconvert", 0x1D),
  ("accept", 0x1E),
  ("modechange", 0x1F),
  ("space", 0x20),
  ("pageup", 0x21),
  ("pagedown", 0x22),
  ("end", 0x23),
  ("home", 0x24),
  ("left", 0x25),
  ("up", 0x26),
  ("right", 0x27),
  ("down", 0x28),
  ("select", 0x29),
  ("print", 0x2A),
  ("execute", 0x2B),
  ("ps", 0x2C),
  ("printscreen", 0x2C),
  ("insert", 0x2D),
  ("ins", 0x2D),
  ("delete", 0x2E),
  ("del", 0x2E),
  ("help", 0x2F),
  ("win", 0x5B),
  ("lwin", VK_LWIN),
  ("rwin", VK_RWIN),
  ("apps", 0x5D),
  ("sleep", 0x5F),
  ("numpad0", 0x60),
  ("numpad1", 0x61),
  ("numpad2", 0x62),
  ("numpad3", 0x63),
  ("numpad4", 0x64),
  ("numpad5", 0x65),
  ("numpad6", 0x66),
  ("numpad7", 0x67),
  ("numpad8", 0x68),
  ("numpad9", 0x69),
  ("multiply", 0x6A),  // keypad *
  ("add", 0x6B),       // keypad +
  ("separator", 0x6C), // ?
  ("subtract", 0x6D),  // keypad -
  ("decimal", 0x6E),   // keypad .
  ("divide", 0x6F),    // keypad /
  ("f1", 0x70),
  ("f2", 0x71),
  ("f3", 0x72),
  ("f4", 0x73),
  ("f5", 0x74),
  ("f6", 0x75),
  ("f7", 0x76),
  ("f8", 0x77),
  ("f9", 0x78),
  ("f10", 0x79),
  ("f11", 0x7A),
  ("f12", 0x7B),
  ("f13", 0x7C),
  ("f14", 0x7D),
  ("f15", 0x7E),
  ("f16", 0x7F),
  ("f17", 0x80),
  ("f18", 0x81),
  ("f19", 0x82),
  ("f20", 0x83),
  ("f21", 0x84),
  ("f22", 0x85),
  ("f23", 0x86),
  ("f24", 0x87),
  ("numlock", 0x90),
  ("scroll", 0x91),
  ("lshift", VK_LSHIFT),
  ("rshift", VK_RSHIFT),
  ("lctrl", VK_LCONTROL),
  ("rctrl", VK_RCONTROL),
  ("lalt", VK_LMENU),
  ("ralt", VK_RMENU),
  ("browser_back", 0xA6),
  ("browser_forward", 0xA7),
  ("browser_refresh", 0xA8),
  ("browser_stop", 0xA9),
  ("browser_search", 0xAA),
  ("browser_favorites", 0xAB),
  ("browser_home", 0xAC),
  ("volume_mute", 0xAD),
  ("volume_down", 0xAE),
  ("volume_up", 0xAF),
  ("media_next_track", 0xB0),
  ("media_prev_track", 0xB1),
  ("media_stop", 0xB2),
  ("media_play_pause", 0xB3),
  ("launch_mail", 0xB4),
  ("launch_media_select", 0xB5),
  ("launch_app1", 0xB6),
  ("launch_app2", 0xB7),
  ("attn", 0xF6),
  ("crsel", 0xF7),
  ("exsel", 0xF8),
  ("ereof", 0xF9),
  ("play", 0xFA),
  ("zoom", 0xFB),
  ("pa1", 0xFD),
  ("oem_clear", 0xFE),
];

static ALIAS_NAME_MAP: &[(&str, char)] = &[("lt", '<'), ("gt", '>'), ("minus", '-'), ("plus", '+')];

// Parse one char to (keycode, shift_pressed)
pub fn parse_char(ch: char) -> Result<(u8, bool)> {
  if ch.len_utf8() != 1 {
    bail!("Invalid char");
  }
  let ret = unsafe { km::VkKeyScanA(ch as i8) };
  if ret == -1 {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "VkScanA failed."
    ));
  }
  Ok(((ret & 0xFF) as u8, (ret & 0x0100) != 0))
}

// Parse key name to (keycode, shift_pressed)
pub fn parse_name(name: &str) -> Result<(u8, bool)> {
  for &(n, c) in KEYNAME_CODE_MAP.iter() {
    if n == name {
      return Ok((c, false));
    }
  }

  for &(n, c) in ALIAS_NAME_MAP.iter() {
    if n == name {
      return parse_char(c);
    }
  }

  if name.len() == 1 {
    return parse_char(name.chars().next().unwrap());
  }

  match name.parse::<u8>() {
    Ok(c) => return Ok((c, false)),
    _ => (),
  }

  Err(format!("Unknown key name {}", name).into())
}

pub fn is_shift(c: u8) -> bool {
  c == VK_SHIFT || c == VK_LSHIFT || c == VK_RSHIFT
}

pub fn is_alt(c: u8) -> bool {
  c == VK_MENU || c == VK_LMENU || c == VK_RMENU
}

pub fn is_ctrl(c: u8) -> bool {
  c == VK_CONTROL || c == VK_LCONTROL || c == VK_RCONTROL
}

pub fn is_win(c: u8) -> bool {
  c == VK_LWIN || c == VK_RWIN
}

pub fn is_modifier(c: u8) -> bool {
  is_shift(c) || is_alt(c) || is_ctrl(c) || is_win(c)
}

pub const STATE_ALT_PRESSED: u8 = 0x01;
pub const STATE_CTRL_PRESSED: u8 = 0x02;
pub const STATE_SHIFT_PRESSED: u8 = 0x04;
pub const STATE_WIN_PRESSED: u8 = 0x08;

#[derive(Debug)]
pub struct KeyAction {
  pub code: u8,
  pub up: bool,
  pub state: u8,
}

fn append_key_press(keys: &mut Vec<KeyAction>, code: u8, up: bool, state: &mut u8) {
  keys.push(KeyAction {
    code,
    up,
    state: *state,
  });
  if is_shift(code) {
    *state = if up {
      *state & !STATE_SHIFT_PRESSED
    } else {
      *state | STATE_SHIFT_PRESSED
    }
  } else if is_ctrl(code) {
    *state = if up {
      *state & !STATE_CTRL_PRESSED
    } else {
      *state | STATE_CTRL_PRESSED
    }
  } else if is_alt(code) {
    *state = if up {
      *state & !STATE_ALT_PRESSED
    } else {
      *state | STATE_ALT_PRESSED
    }
  } else if is_win(code) {
    *state = if up {
      *state & !STATE_WIN_PRESSED
    } else {
      *state | STATE_WIN_PRESSED
    }
  }
}

fn append_key_down_up(keys: &mut Vec<KeyAction>, code: u8, shifted: bool, state: &mut u8) {
  if shifted {
    append_key_press(keys, VK_SHIFT as u8, false, state);
  }
  append_key_press(keys, code, false, state);
  append_key_press(keys, code, true, state);
  if shifted {
    append_key_press(keys, VK_SHIFT as u8, true, state);
  }
}

pub fn parse(keys_str: &str) -> Result<Vec<KeyAction>> {
  let mut keys: Vec<KeyAction> = Vec::new();
  let mut in_bracket = false;
  let mut name = String::new();
  let mut state: u8 = 0;
  let mut segs: Vec<(u8, bool)> = Vec::new();

  for c in keys_str.chars() {
    if !in_bracket {
      if c == '<' {
        segs.clear();
        in_bracket = true;
      } else {
        let (code, shifted) = parse_char(c)?;
        append_key_down_up(&mut keys, code, shifted, &mut state);
      }
    } else {
      if c == '>' {
        if segs.is_empty() && name.is_empty() {
          bail!(format!("Empty angle brackets"))
        }
        if name.is_empty() {
          // End with '-' or '+', e.g. <ctrl+a+>
          for seg in segs.iter() {
            append_key_press(&mut keys, seg.0, seg.1, &mut state);
          }
        } else if segs.is_empty() {
          // One key, e.g. <pagedown>
          let (code, shifted) = parse_name(&name)?;
          name.clear();
          append_key_down_up(&mut keys, code, shifted, &mut state);
        } else {
          // Combination, e.g. <ctrl+c>
          let (code, _) = parse_name(&name)?;
          name.clear();
          segs.push((code, false));
          // Press down every keys
          for seg in segs.iter() {
            if seg.1 {
              bail!("Combination keys in angle brackets can only contains '+'")
            }
            append_key_press(&mut keys, seg.0, false, &mut state);
          }
          // Press up keys in reversed order
          for seg in segs.iter().rev() {
            append_key_press(&mut keys, seg.0, true, &mut state);
          }
        }
        in_bracket = false;
      } else if c == '+' || c == '-' {
        let (code, _) = parse_name(&name)?;
        name.clear();
        segs.push((code, c == '-'));
      } else {
        name.push(c);
      }
    }
  }

  if in_bracket {
    bail!("Unclosed angle brackets");
  }

  Ok(keys)
}

#[repr(C)]
#[derive(Debug, Clone)]
struct KeyboardInput {
  type_: u32,
  ki: km::KEYBDINPUT,
  reserved: u32,
}

pub fn input_parsed(keys: &Vec<KeyAction>) -> Result<()> {
  let zeroed: KeyboardInput = unsafe { std::mem::zeroed() };
  let mut inputs: Vec<KeyboardInput> = vec![zeroed; keys.len()];
  for i in 0..inputs.len() {
    let key = &keys[i];
    let mut input = &mut inputs[i];
    input.type_ = INPUT_KEYBOARD;
    input.ki.w_vk = key.code.into();
    input.ki.dw_flags = if key.up { KEYEVENTF_KEYUP } else { 0 };
  }

  let sent = unsafe {
    km::SendInput(
      inputs.len() as _,
      inputs.as_mut_ptr() as _,
      std::mem::size_of::<KeyboardInput>() as _,
    )
  };
  if sent != inputs.len() as _ {
    bail!(Error::with_chain(
      std::io::Error::last_os_error(),
      "SendInput failed."
    ));
  }
  Ok(())
}

pub fn input(keys_str: &str) -> Result<()> {
  let keys = parse(keys_str)?;
  input_parsed(&keys)
}

pub fn wait_keys_up(keys_str: &str) {
  let mut keys: Vec<KeyAction> = Vec::new();
  if keys_str.is_empty() {
    keys.push(KeyAction {
      code: VK_SHIFT as u8,
      up: false,
      state: 0,
    });
    keys.push(KeyAction {
      code: VK_MENU as u8,
      up: false,
      state: 0,
    });
    keys.push(KeyAction {
      code: VK_CONTROL as u8,
      up: false,
      state: 0,
    });
    keys.push(KeyAction {
      code: VK_LWIN as u8,
      up: false,
      state: 0,
    });
    keys.push(KeyAction {
      code: VK_RWIN as u8,
      up: false,
      state: 0,
    });
  } else {
    match parse(keys_str) {
      Ok(ret) => {
        keys = ret;
      }
      Err(_) => return (),
    }
  }

  let mut keys_up = false;
  while !keys_up {
    keys_up = true;
    for input in keys.iter() {
      let mut code: u8 = input.code;
      if is_shift(code) {
        code = VK_SHIFT as u8;
      } else if is_alt(code) {
        code = VK_MENU as u8;
      } else if is_ctrl(code) {
        code = VK_CONTROL as u8;
      }
      let ret = unsafe { km::GetAsyncKeyState(code.into()) };
      if (ret as u16 & 0x8000) != 0 {
        keys_up = false;
        break;
      }
    }
    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}

#[test]
fn test() {
  // println!(
  //   "Size of KEYBDINPUT {}",
  //   std::mem::size_of::<km::KEYBDINPUT>()
  // );
  // println!(
  //   "Size of KeyboardInput {}",
  //   std::mem::size_of::<KeyboardInput>()
  // );
  println!("Testing start");
  // let result = input("<win+d>");
  // let result = parse("<win+d>");
  // let result = parse_char('d');
  // println!("result: {:?}", result);
}
