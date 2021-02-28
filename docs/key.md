# Key Name

Based on [Virtual-Key Codes](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)

Three kind of key string

- single key stroke: Single key outside of bracket. auto detect if SHIFT key is needed.

- press down or up: key name followed by `+` or `-`, `+` means key down, `-` means key up. i.e. `<shift+><a+><a-><shift->`, which is same as `A`, also can be `<shift+a+a-shift->`

- key combination: `<...+...+...>`, i.e. `<ctrl+c>`

In bracket `<>`, multiple keys should be separated by `+` or `-`, i.e. `<aa>` is invalid.

Escaped names:

- `<` : use `<lt>` instead
- `>` : use `<gt>` instead
- `-` : use `<minus>` instead (Use in bracket only)
- `+` : use `<plus>` instead (Use in bracket only)

Other names:

| Name                | Decimal | Hex  | Description             |
| ------------------- | ------- | ---- | ----------------------- |
| backspace           | 8       | 08   | Backspace               |
| bs                  | 8       | 08   | Backspace               |
| tab                 | 9       | 09   | TAB                     |
| clear               | 12      | 0C   | Clear key               |
| enter               | 13      | 0D   | ENTER                   |
| return              | 13      | 0D   | ENTER                   |
| shift               | 16      | 10   | SHIFT key               |
| ctrl                | 17      | 11   | CTRL key                |
| alt                 | 18      | 12   | ALT key                 |
| pause               | 19      | 13   | PAUSE                   |
| caps                | 20      | 14   | CAPS LOCK               |
| kana                | 21      | 15   | IME Kana mode           |
| hangul              | 21      | 15   | IME Hangul mode         |
| ime_on              | 22      | 16   | IME On                  |
| junja               | 23      | 17   | IME Junja mode          |
| final               | 24      | 18   | IME final mode          |
| hanja               | 25      | 19   | IME Hanja mode          |
| kanji               | 25      | 19   | IME Kanji mode          |
| ime_off             | 26      | 1A   | IME Off                 |
| esc                 | 27      | 1B   | ESC                     |
| convert             | 28      | 1C   | IME convert             |
| nonconvert          | 29      | 1D   | IME nonconvert          |
| accept              | 30      | 1E   | IME accept              |
| modechange          | 31      | 1F   | IME mode change request |
| space               | 32      | 20   | SPACE                   |
| pageup              | 33      | 21   | PAGE UP                 |
| pagedown            | 34      | 22   | PAGE DOWN               |
| end                 | 35      | 23   | END                     |
| home                | 36      | 24   | HOME                    |
| left                | 37      | 25   | LEFT ARROW              |
| up                  | 38      | 26   | UP ARROW                |
| right               | 39      | 27   | RIGHT ARROW             |
| down                | 40      | 28   | DOWN ARROW              |
| select              | 41      | 29   | SELECT                  |
| print               | 42      | 2A   | PRINT                   |
| execute             | 43      | 2B   | EXECUTE                 |
| printscreen         | 44      | 2C   | PRINT SCREEN            |
| ps                  | 44      | 2C   | PRINT SCREEN            |
| insert              | 45      | 2D   | INS                     |
| ins                 | 45      | 2D   | INS                     |
| delete              | 46      | 2E   | DEL                     |
| del                 | 46      | 2E   | DEL                     |
| help                | 47      | 2F   | HELP                    |
| lwin                | 91      | 5B   | Left Windows key        |
| rwin                | 92      | 5C   | Right Windows key       |
| apps                | 93      | 5D   | Applications key        |
| sleep               | 95      | 5F   | Computer Sleep key      |
| numpad0             | 96      | 60   | Numeric keypad 0        |
| numpad1             | 97      | 61   | Numeric keypad 1        |
| numpad2             | 98      | 62   | Numeric keypad 2        |
| numpad3             | 99      | 63   | Numeric keypad 3        |
| numpad4             | 100     | 64   | Numeric keypad 4        |
| numpad5             | 101     | 65   | Numeric keypad 5        |
| numpad6             | 102     | 66   | Numeric keypad 6        |
| numpad7             | 103     | 67   | Numeric keypad 7        |
| numpad8             | 104     | 68   | Numeric keypad 8        |
| numpad9             | 105     | 69   | Numeric keypad 9        |
| multiply            | 106     | 6A   | Keypad *                |
| add                 | 107     | 6B   | Keypad +                |
| separator           | 108     | 6C   | Keypad Separator        |
| subtract            | 109     | 6D   | Keypad -                |
| decimal             | 110     | 6E   | Keypad .                |
| divide              | 111     | 6F   | Keypad /                |
| f1                  | 112     | 70   | F1 key                  |
| f2                  | 113     | 71   | F2 key                  |
| f3                  | 114     | 72   | F3 key                  |
| f4                  | 115     | 73   | F4 key                  |
| f5                  | 116     | 74   | F5 key                  |
| f6                  | 117     | 75   | F6 key                  |
| f7                  | 118     | 76   | F7 key                  |
| f8                  | 119     | 77   | F8 key                  |
| f9                  | 120     | 78   | F9 key                  |
| f10                 | 121     | 79   | F10 key                 |
| f11                 | 122     | 7A   | F11 key                 |
| f12                 | 123     | 7B   | F12 key                 |
| f13                 | 124     | 7C   | F13 key                 |
| f14                 | 125     | 7D   | F14 key                 |
| f15                 | 126     | 7E   | F15 key                 |
| f16                 | 127     | 7F   | F16 key                 |
| f17                 | 128     | 80   | F17 key                 |
| f18                 | 129     | 81   | F18 key                 |
| f19                 | 130     | 82   | F19 key                 |
| f20                 | 131     | 83   | F20 key                 |
| f21                 | 132     | 84   | F21 key                 |
| f22                 | 133     | 85   | F22 key                 |
| f23                 | 134     | 86   | F23 key                 |
| f24                 | 135     | 87   | F24 key                 |
| numlock             | 144     | 90   | NUM LOCK key            |
| scroll              | 145     | 91   | SCROLL LOCK key         |
| lshift              | 160     | A0   | Left SHIFT              |
| rshift              | 161     | A1   | Right SHIFT             |
| lctrl               | 162     | A2   | Left CTRL               |
| rctrl               | 163     | A3   | Right CTRL              |
| lalt                | 164     | A4   | Left ALT                |
| ralt                | 165     | A5   | Right ALT               |
| browser_back        | 166     | A6   | Browser Back key        |
| browser_forward     | 167     | A7   | Browser Forward key     |
| browser_refresh     | 168     | A8   | Browser Refresh key     |
| browser_stop        | 169     | A9   | Browser Stop key        |
| browser_search      | 170     | AA   | Browser Search key      |
| browser_favorites   | 171     | AB   | Browser Favorites key   |
| browser_home        | 172     | AC   | Browser Home key        |
| volume_mute         | 173     | AD   | Volume Mute key         |
| volume_down         | 174     | AE   | Volume Down key         |
| volume_up           | 175     | AF   | Volume Up key           |
| media_next_track    | 176     | B0   | Next Track key          |
| media_prev_track    | 177     | B1   | Previous Track key      |
| media_stop          | 178     | B2   | Stop Media key          |
| media_play_pause    | 179     | B3   | Play/Pause Media key    |
| launch_mail         | 180     | B4   | Start Mail key          |
| launch_media_select | 181     | B5   | Select Media key        |
| launch_app1         | 182     | B6   | Start Application 1 key |
| launch_app2         | 183     | B7   | Start Application 2 key |
| attn                | 246     | F6   | Attn key                |
| crsel               | 247     | F7   | CrSel key               |
| exsel               | 248     | F8   | ExSel key               |
| ereof               | 249     | F9   | Erase EOF key           |
| play                | 250     | FA   | Play key                |
| zoom                | 251     | FB   | Zoom key                |
| pa1                 | 253     | FD   | PA1 key                 |
| oem_clear           | 254     | FE   | Clear key               |


Direct use virtual key code

i.e. `<101>` same as `<numpad5>`
