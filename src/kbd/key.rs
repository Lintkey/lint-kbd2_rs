#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KbdKey {
    Normal(QwertyKey),
    State(StateKey),
}

impl KbdKey {
    // pub const NONE: Self = KbdKey::Normal(QwertyKey::None);
}

/// 非Modifier Key，可直接转换为USB keycode
#[allow(unused)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QwertyKey {
    /// Reserved, no-key.
    // None = 0x00,
    /// Keyboard roll over error, too many keys are pressed simultaneously, not a physical key.
    /// NKRO: n-key rollover.
    ErrorRollover = 0x01,
    /// Keyboard post fail error, not a physical key.
    PostFail = 0x02,
    /// An undefined error, not a physical key.
    ErrorUndefined = 0x03,
    /// `a` and `A`
    A = 0x04,
    /// `b` and `B`
    B = 0x05,
    /// `c` and `C`
    C = 0x06,
    /// `d` and `D`
    D = 0x07,
    /// `e` and `E`
    E = 0x08,
    /// `f` and `F`
    F = 0x09,
    /// `g` and `G`
    G = 0x0A,
    /// `h` and `H`
    H = 0x0B,
    /// `i` and `I`
    I = 0x0C,
    /// `j` and `J`
    J = 0x0D,
    /// `k` and `K`
    K = 0x0E,
    /// `l` and `L`
    L = 0x0F,
    /// `m` and `M`
    M = 0x10,
    /// `n` and `N`
    N = 0x11,
    /// `o` and `O`
    O = 0x12,
    /// `p` and `P`
    P = 0x13,
    /// `q` and `Q`
    Q = 0x14,
    /// `r` and `R`
    R = 0x15,
    /// `s` and `S`
    S = 0x16,
    /// `t` and `T`
    T = 0x17,
    /// `u` and `U`
    U = 0x18,
    /// `v` and `V`
    V = 0x19,
    /// `w` and `W`
    W = 0x1A,
    /// `x` and `X`
    X = 0x1B,
    /// `y` and `Y`
    Y = 0x1C,
    /// `z` and `Z`
    Z = 0x1D,
    /// `1` and `!`
    Kc1 = 0x1E,
    /// `2` and `@`
    Kc2 = 0x1F,
    /// `3` and `#`
    Kc3 = 0x20,
    /// `4` and `$`
    Kc4 = 0x21,
    /// `5` and `%`
    Kc5 = 0x22,
    /// `6` and `^`
    Kc6 = 0x23,
    /// `7` and `&`
    Kc7 = 0x24,
    /// `8` and `*`
    Kc8 = 0x25,
    /// `9` and `(`
    Kc9 = 0x26,
    /// `0` and `)`
    Kc0 = 0x27,
    /// `Enter`
    Enter = 0x28,
    /// `Esc`
    Escape = 0x29,
    /// `Backspace`
    Backspace = 0x2A,
    /// `Tab`
    Tab = 0x2B,
    /// `Space`
    Space = 0x2C,
    /// `-` and `_`
    Minus = 0x2D,
    /// `=` and `+`
    Equal = 0x2E,
    /// `[` and `{`
    LeftBracket = 0x2F,
    /// `]` and `}`
    RightBracket = 0x30,
    /// `\` and `|`
    Backslash = 0x31,
    /// Non-US `#` and `~`
    NonusHash = 0x32,
    /// `;` and `:`
    Semicolon = 0x33,
    /// `'` and `"`
    Quote = 0x34,
    /// `~` and `\``
    Grave = 0x35,
    /// `,` and `<`
    Comma = 0x36,
    /// `.` and `>`
    Dot = 0x37,
    /// `/` and `?`
    Slash = 0x38,
    /// `CapsLock`
    CapsLock = 0x39,
    /// `F1`
    F1 = 0x3A,
    /// `F2`
    F2 = 0x3B,
    /// `F3`
    F3 = 0x3C,
    /// `F4`
    F4 = 0x3D,
    /// `F5`
    F5 = 0x3E,
    /// `F6`
    F6 = 0x3F,
    /// `F7`
    F7 = 0x40,
    /// `F8`
    F8 = 0x41,
    /// `F9`
    F9 = 0x42,
    /// `F10`
    F10 = 0x43,
    /// `F11`
    F11 = 0x44,
    /// `F12`
    F12 = 0x45,
    /// Print Screen
    PrintScreen = 0x46,
    /// Scroll Lock
    ScrollLock = 0x47,
    /// Pause
    Pause = 0x48,
    /// Insert
    Insert = 0x49,
    /// Home
    Home = 0x4A,
    /// Page Up
    PageUp = 0x4B,
    /// Delete
    Delete = 0x4C,
    /// End
    End = 0x4D,
    /// Page Down
    PageDown = 0x4E,
    /// Right arrow
    Right = 0x4F,
    /// Left arrow
    Left = 0x50,
    /// Down arrow
    Down = 0x51,
    /// Up arrow
    Up = 0x52,
    /// Nums Lock
    NumLock = 0x53,
    /// `/` on keypad
    KpSlash = 0x54,
    /// `*` on keypad
    KpAsterisk = 0x55,
    /// `-` on keypad
    KpMinus = 0x56,
    /// `+` on keypad
    KpPlus = 0x57,
    /// `Enter` on keypad
    KpEnter = 0x58,
    /// `1` on keypad
    Kp1 = 0x59,
    /// `2` on keypad
    Kp2 = 0x5A,
    /// `3` on keypad
    Kp3 = 0x5B,
    /// `4` on keypad
    Kp4 = 0x5C,
    /// `5` on keypad
    Kp5 = 0x5D,
    /// `6` on keypad
    Kp6 = 0x5E,
    /// `7` on keypad
    Kp7 = 0x5F,
    /// `8` on keypad
    Kp8 = 0x60,
    /// `9` on keypad
    Kp9 = 0x61,
    /// `0` on keypad
    Kp0 = 0x62,
    /// `.` on keypad
    KpDot = 0x63,
    /// Non-US `\` or `|`
    NonusBackslash = 0x64,
    /// `Application`
    Application = 0x65,
    /// `Power`
    KbPower = 0x66,
    /// `=` on keypad
    KpEqual = 0x67,
    /// `F13`
    F13 = 0x68,
    /// `F14`
    F14 = 0x69,
    /// `F15`
    F15 = 0x6A,
    /// `F16`
    F16 = 0x6B,
    /// `F17`
    F17 = 0x6C,
    /// `F18`
    F18 = 0x6D,
    /// `F19`
    F19 = 0x6E,
    /// `F20`
    F20 = 0x6F,
    /// `F21`
    F21 = 0x70,
    /// `F22`
    F22 = 0x71,
    /// `F23`
    F23 = 0x72,
    /// `F24`
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    /// Mute
    KbMute = 0x7F,
    /// Volume Up
    KbVolumeUp = 0x80,
    /// Volume Down
    KbVolumeDown = 0x81,
    /// Locking Caps Lock
    LockingCapsLock = 0x82,
    /// Locking Num Lock
    LockingNumLock = 0x83,
    /// Locking scroll lock
    LockingScrollLock = 0x84,
    KpComma = 0x85,
    KpEqualAs400 = 0x86,
    International1 = 0x87,
    International2 = 0x88,
    International3 = 0x89,
    International4 = 0x8A,
    International5 = 0x8B,
    International6 = 0x8C,
    International7 = 0x8D,
    International8 = 0x8E,
    International9 = 0x8F,
    Language1 = 0x90,
    Language2 = 0x91,
    Language3 = 0x92,
    Language4 = 0x93,
    Language5 = 0x94,
    Language6 = 0x95,
    Language7 = 0x96,
    Language8 = 0x97,
    Language9 = 0x98,
    AlternateErase = 0x99,
    SystemRequest = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    Crsel = 0xA3,
    Exsel = 0xA4,
    SystemPower = 0xA5,
    SystemSleep = 0xA6,
    SystemWake = 0xA7,
    AudioMute = 0xA8,
    AudioVolUp = 0xA9,
    AudioVolDown = 0xAA,
    MediaNextTrack = 0xAB,
    MediaPrevTrack = 0xAC,
    MediaStop = 0xAD,
    MediaPlayPause = 0xAE,
    MediaSelect = 0xAF,
    MediaEject = 0xB0,
    Mail = 0xB1,
    Calculator = 0xB2,
    MyComputer = 0xB3,
    WwwSearch = 0xB4,
    WwwHome = 0xB5,
    WwwBack = 0xB6,
    WwwForward = 0xB7,
    WwwStop = 0xB8,
    WwwRefresh = 0xB9,
    WwwFavorites = 0xBA,
    MediaFastForward = 0xBB,
    MediaRewind = 0xBC,
    /// Brightness Up
    BrightnessUp = 0xBD,
    /// Brightness Down
    BrightnessDown = 0xBE,
    ControlPanel = 0xBF,
    Assistant = 0xC0,
    MissionControl = 0xC1,
    Launchpad = 0xC2,
    /// Mouse Up
    MouseUp = 0xCD,
    /// Mouse Down
    MouseDown = 0xCE,
    /// Mouse Left
    MouseLeft = 0xCF,
    /// Mouse Right
    MouseRight = 0xD0,
    /// Mouse Button 1(Left)
    MouseBtn1 = 0xD1,
    /// Mouse Button 2(Right)
    MouseBtn2 = 0xD2,
    /// Mouse Button 3(Middle)
    MouseBtn3 = 0xD3,
    /// Mouse Button 4(Back)
    MouseBtn4 = 0xD4,
    /// Mouse Button 5(Forward)
    MouseBtn5 = 0xD5,
    MouseBtn6 = 0xD6,
    MouseBtn7 = 0xD7,
    MouseBtn8 = 0xD8,
    MouseWheelUp = 0xD9,
    MouseWheelDown = 0xDA,
    MouseWheelLeft = 0xDB,
    MouseWheelRight = 0xDC,
    MouseAccel0 = 0xDD,
    MouseAccel1 = 0xDE,
    MouseAccel2 = 0xDF,
}

impl Into<KbdKey> for QwertyKey {
    fn into(self) -> KbdKey {
        KbdKey::Normal(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateKey {
    /// \[USB\] Modifier Key，Win、Shift等状态键
    Modifier(ModifierKey),
    /// 层操作键，
    Layer(LayerKey),
}

impl Into<KbdKey> for StateKey {
    fn into(self) -> KbdKey {
        KbdKey::State(self)
    }
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModifierKey {
    /// Left Control
    LCtrl = 0xE0,
    /// Left Shift
    LShift = 0xE1,
    /// Left Alt
    LAlt = 0xE2,
    /// Left GUI
    LGui = 0xE3,
    /// Right Control
    RCtrl = 0xE4,
    /// Right Shift
    RShift = 0xE5,
    /// Right Alt
    RAlt = 0xE6,
    /// Right GUI
    RGui = 0xE7,
}

impl Into<StateKey> for ModifierKey {
    fn into(self) -> StateKey {
        StateKey::Modifier(self)
    }
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayerKey {
    LayerOn(u8),
    LayerSwitch(u8),
}

impl Into<StateKey> for LayerKey {
    fn into(self) -> StateKey {
        StateKey::Layer(self)
    }
}