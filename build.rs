fn main() {
  windows::build!(
    windows::win32::keyboard_and_mouse_input::*,
    windows::win32::data_exchange::GlobalAddAtomA,
    windows::win32::windows_and_messaging::MSG,
    windows::win32::windows_and_messaging::GetMessageA,
  );
}
