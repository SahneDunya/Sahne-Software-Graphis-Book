#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEvent {
    Moved { x: f32, y: f32 },
    ButtonPressed { button: MouseButton, x: f32, y: f32 },
    ButtonReleased { button: MouseButton, x: f32, y: f32 },
    WheelScrolled { delta_x: f32, delta_y: f32 },
    Entered { x: f32, y: f32 },
    Left { x: f32, y: f32 },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardKey {
    // Alfanümerik tuşlar
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,

    // Fonksiyon tuşları
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,

    // Düzenleyici tuşlar
    Space, Enter, Escape, Tab, Backspace, Delete, Insert,
    Up, Down, Left, Right,
    PageUp, PageDown, Home, End,

    // Değiştirici tuşlar
    ShiftLeft, ShiftRight,
    ControlLeft, ControlRight,
    AltLeft, AltRight,
    SuperLeft, SuperRight,
    CapsLock, NumLock, ScrollLock,

    // Sayısal tuş takımı
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadDecimal, NumpadEnter, NumpadEquals, NumpadPlus, NumpadMinus, NumpadMultiply, NumpadDivide,

    // Semboller ve diğerleri
    Grave, Minus, Equals, BracketLeft, BracketRight, Backslash,
    Semicolon, Apostrophe, Comma, Period, Slash,

    // Diğer
    PrintScreen, Pause,

    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardEvent {
    KeyPressed { key: KeyboardKey },
    KeyReleased { key: KeyboardKey },
    TextInput { character: char },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowEvent {
    Resized { width: u32, height: u32 },
    Moved { x: i32, y: i32 },
    Closed,
    Focused,
    Unfocused,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Touch {
    pub id: u64, // Her bir dokunuşu ayırt etmek için benzersiz bir ID
    pub position: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TouchEvent {
    Started(Vec<Touch>),
    Moved(Vec<Touch>),
    Ended(Vec<Touch>),
    Cancelled(Vec<Touch>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Window(WindowEvent),
    Touch(TouchEvent), // Dokunmatik olayları ekledik
    // İlerleyen aşamalarda diğer olay türlerini ekleyebilirsiniz.
}