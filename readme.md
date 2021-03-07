# Turkey

## What is Turkey?

A keyboard and mouse automation dll for Windows.

## How to use?

It's better use with python.

- Download release dll
- Use with following python3 code.

```
import ctypes
class Turkey:
    def __init__(self, path):
        self.dll_path = path
        self.dll = ctypes.cdll.LoadLibrary(path)
    
    def kb_input(self, s : str) -> int:
        """
        # Input key arrays
        tk.kb_input("abcd1234!@#$")

        # Input combination keys
        # Show desktop shortcut Windows + d
        tk.kb_input("<win+d>")

        # Input key pressed down '+' or up '-'
        # Input "ABC" when caps-lock is off
        tk.kb_input("<shift+>abc<shift->")

        # Input key pressed using '+' or '-'
        # Input "ABC" when caps-lock is off
        tk.kb_input("<shift+>abc<shift->")

        # Some keys must escaped
        # Input "<>"
        tk.kb_input("<lt><gt>")

        More details of key name please see [key.md](dos/key.md).
        """
        return self.dll.kb_input(s.encode())
    
    def move(self, x : int, y : int) -> int:
        """Move mouse relatively"""
        return self.dll.move(x, y)
    
    def move_to(self, x : int, y : int) -> int:
        """Move mouse to a given position"""
        return self.dll.move_to(x, y)
    
    button_left = 1
    button_right = 2
    button_middle = 3
    button_x1 = 4
    button_x2 = 5
    def click(self, button : int) -> int:
        """Click(press down and up) mouse button"""
        return self.dll.click(button)

    def press(self, button : int, up : bool) -> int:
        """Press down or up mouse button"""
        return self.dll.press(button, 1 if up else 0)
    
    def wheel(self, amount : int) -> int:
        """mouse wheel"""
        return self.dll.wheel(amount)
    
    def hwheel(self, amount : int) -> int:
        """mouse horizontal wheel"""
        return self.dll.wheel(amount)
        return self.dll.hwheel(amount)
    
    def hotkey_register(self, hotkey : str) -> int:
        """
        hotkey_id = tk.hotkey_register("<ctrl+alt+q>")
        if hotkey_id <= 0:
            raise "Error"
        while True:
            waited = tk.hotkey_wait()
            if waited == hotkey_id:
                tk.wait_keys_up()
                # Do something...
                pass
        tk.hotkey_unregister(hotkey_id)
        """
        return self.dll.hotkey_register(hotkey.encode())
    
    def hotkey_unregister(self, hotkey_id : int) -> int:
        return self.dll.hotkey_unregister(hotkey_id)
    
    def hotkey_wait(self) -> int:
        return self.dll.hotkey_wait()
    
    def wait_keys_up(self, s :str) -> int:
        return self.dll.wait_keys_up(s.encode())

    def window_find(self, cls: str, caption: str) -> int:
        return self.dll.window_find(cls.encode(), caption.encode())
    
    def window_pos(self, hwnd: int) -> (int, int):
        x = ctypes.c_int32(0)
        y = ctypes.c_int32(0)
        ret = self.dll.window_pos(ctypes.c_size_t(hwnd), ctypes.byref(x), ctypes.byref(y))
        if ret == 0:
            return (x, y)
        else:
            return None

    def window_size(self, hwnd: int) -> (int, int):
        x = ctypes.c_int32(0)
        y = ctypes.c_int32(0)
        ret = self.dll.window_size(ctypes.c_size_t(hwnd), ctypes.byref(x), ctypes.byref(y))
        if ret == 0:
            return (x, y)
        else:
            return None

tk = Turkey("path/to/turkey.dll")
```

## How to build?

- Install rust
- Clone or download repo
- Run `cargo build`
