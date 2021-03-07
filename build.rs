fn main() {
  windows::build!(
    windows::win32::keyboard_and_mouse_input::*,
    windows::win32::data_exchange::GlobalAddAtomA,
    windows::win32::windows_and_messaging::*,
    windows::win32::display_devices::RECT,
    windows::win32::system_services::GetModuleHandleA,
    windows::win32::system_services::HWND_MESSAGE,
    windows::win32::system_services::WM_INPUT,
    windows::win32::menus_and_resources::HMENU,
  );
}
