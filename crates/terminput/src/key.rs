use core::hash::{Hash, Hasher};

use bitflags::bitflags;

/// A key input event.
#[derive(Debug, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEvent {
    /// The key code.
    pub code: KeyCode,
    /// Key modifiers.
    pub modifiers: KeyModifiers,
    /// Type of key event - press, release, or repeat.
    pub kind: KeyEventKind,
    /// Other keyboard properties.
    pub state: KeyEventState,
}

impl KeyEvent {
    /// Creates a new [`KeyEvent`] with the default state and no modifiers.
    pub const fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    /// Sets the [`KeyModifiers`].
    pub const fn modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Sets the [`KeyEventKind`].
    pub const fn kind(mut self, kind: KeyEventKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the [`KeyEventState`].
    pub const fn state(mut self, state: KeyEventState) -> Self {
        self.state = state;
        self
    }

    /// Normalizes the event so the shift modifier is applied appropriately.
    pub fn normalize_case(mut self) -> Self {
        let c = match self.code {
            KeyCode::Char(c) => c,
            _ => return self,
        };

        if c.is_ascii_uppercase() {
            self.modifiers.insert(KeyModifiers::SHIFT);
        } else if self.modifiers.contains(KeyModifiers::SHIFT) {
            self.code = KeyCode::Char(c.to_ascii_uppercase());
        }
        self
    }
}

impl PartialEq for KeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            code: lhs_code,
            modifiers: lhs_modifiers,
            kind: lhs_kind,
            state: lhs_state,
        } = self.normalize_case();
        let Self {
            code: rhs_code,
            modifiers: rhs_modifiers,
            kind: rhs_kind,
            state: rhs_state,
        } = other.normalize_case();
        (lhs_code == rhs_code)
            && (lhs_modifiers == rhs_modifiers)
            && (lhs_kind == rhs_kind)
            && (lhs_state == rhs_state)
    }
}

impl Eq for KeyEvent {}

impl Hash for KeyEvent {
    fn hash<H: Hasher>(&self, hash_state: &mut H) {
        let Self {
            code,
            modifiers,
            kind,
            state,
        } = self.normalize_case();
        code.hash(hash_state);
        modifiers.hash(hash_state);
        kind.hash(hash_state);
        state.hash(hash_state);
    }
}

/// Represents whether the modifier came from the left or right side of the keyboard, where
/// applicable.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModifierDirection {
    /// Modifier came from the left side of the keyboard.
    Left,
    /// Modifier came from the right side of the keyboard.
    Right,
    /// Direction is unknown or not applicable.
    Unknown,
}

/// Represents a key.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Escape key.
    Esc,
    /// Caps Lock key.
    CapsLock,
    /// Scroll Lock key.
    ScrollLock,
    /// Num Lock key.
    NumLock,
    /// Print Screen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,
    /// The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
    KeypadBegin,
    /// A media key.
    Media(MediaKeyCode),
    /// A modifier key.
    Modifier(ModifierKeyCode, ModifierDirection),
}

bitflags! {
    /// Represents key modifiers (shift, control, alt, etc.).
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    pub struct KeyModifiers: u8 {
        /// No modifiers.
        const NONE = 0;
        /// Key was pressed with shift.
        const SHIFT = 1;
        /// Key was pressed with alt.
        const ALT = 1<<1;
        /// Key was pressed with control.
        const CTRL = 1<<2;
        /// Key was pressed with super.
        const SUPER = 1<<3;
        /// Key was pressed with hyper.
        const HYPER = 1<<4;
        /// Key was pressed with meta.
        const META = 1<<5;
    }
}

/// Shift modifier.
pub const SHIFT: KeyModifiers = KeyModifiers::SHIFT;
/// Alt modifier.
pub const ALT: KeyModifiers = KeyModifiers::ALT;
/// Ctrl modifier.
pub const CTRL: KeyModifiers = KeyModifiers::CTRL;
/// Super modifier.
pub const SUPER: KeyModifiers = KeyModifiers::SUPER;
/// Hyper modifier.
pub const HYPER: KeyModifiers = KeyModifiers::HYPER;
/// Meta modifier.
pub const META: KeyModifiers = KeyModifiers::META;

/// Type of key event. Repeat and release events may not be emitted if the input source is not
/// configured to do so.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyEventKind {
    /// Key press.
    Press,
    /// Key repeat.
    Repeat,
    /// Key release.
    Release,
}

/// Media keys.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MediaKeyCode {
    /// Play media key.
    Play,
    /// Pause media key.
    Pause,
    /// Play/Pause media key.
    PlayPause,
    /// Reverse media key.
    Reverse,
    /// Stop media key.
    Stop,
    /// Fast-forward media key.
    FastForward,
    /// Rewind media key.
    Rewind,
    /// Next-track media key.
    TrackNext,
    /// Previous-track media key.
    TrackPrevious,
    /// Record media key.
    Record,
    /// Lower-volume media key.
    LowerVolume,
    /// Raise-volume media key.
    RaiseVolume,
    /// Mute media key.
    MuteVolume,
}

/// A modifier key event.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModifierKeyCode {
    /// Left Shift key.
    Shift,
    /// Left Control key.
    Control,
    /// Left Alt key.
    Alt,
    /// Left Super key.
    Super,
    /// Left Hyper key.
    Hyper,
    /// Left Meta key.
    Meta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

bitflags! {
    /// Represents extra state about the key event.
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    pub struct KeyEventState: u8 {
        /// No extra state applicable.
        const NONE = 0;
        /// The key event came from the keypad.
        const KEYPAD = 1;
        /// Caps Lock was enabled for this key event.
        const CAPS_LOCK = 1<<1;
        /// Num Lock was enabled for this key event.
        const NUM_LOCK = 1<<2;

    }
}

/// Keypad event state.
pub const KEYPAD: KeyEventState = KeyEventState::KEYPAD;
/// Caps lock event state.
pub const CAPS_LOCK: KeyEventState = KeyEventState::CAPS_LOCK;
/// Num lock event state.
pub const NUM_LOCK: KeyEventState = KeyEventState::NUM_LOCK;

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }
}

/// Pattern matching for key events.
///
/// There are three forms:
/// - `key!(keyCode)`
/// - `key!(modifiers, keyCode)`
/// - `key!(eventState, modifiers, keyCode)`
///
/// If `modifiers` is omitted, only events with no modifiers are matched.
///
/// If `eventState` is omitted, any event state will be matched.
///
/// # Example
///
/// ```
/// use terminput::KeyCode::*;
/// use terminput::{ALT, CTRL, Event, KeyModifiers, Repeats, key, modifiers};
///
/// fn handle_event(event: Event) {
///     if let Some(key_event) = event.as_key_press(Repeats::Include) {
///         const CTRL_ALT: KeyModifiers = modifiers!(CTRL, ALT);
///
///         match key_event {
///             key!(Char('a')) => {
///                 println!("'a' pressed");
///             }
///             key!(CTRL | ALT, Char('d')) => {
///                 println!("'ctrl+d' or 'alt+d' pressed");
///             }
///             key!(CTRL_ALT, Char('c')) => {
///                 println!("'ctrl+alt+c' pressed");
///             }
///             key!(CAPS_LOCK, KeyModifiers::NONE, Char('d')) => {
///                 println!("'d' pressed with caps lock");
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! key {
    ($state:pat, $mods:pat, $code:pat) => {
        $crate::KeyEvent {
            code: $code,
            modifiers: $mods,
            state: $state,
            ..
        }
    };
    ($mods:pat, $code:pat) => {
        $crate::KeyEvent {
            code: $code,
            modifiers: $mods,
            ..
        }
    };
    ($code:pat) => {
        $crate::KeyEvent {
            code: $code,
            modifiers: $crate::KeyModifiers::NONE,
            ..
        }
    };
}

/// Macro for combining key modifiers in a way that's valid in `const` contexts. Useful for match
/// expressions.
///
/// # Example
///
/// ```
/// use terminput::{ALT, CTRL, KeyModifiers, modifiers};
///
/// const mods: KeyModifiers = modifiers!(CTRL, ALT);
/// assert_eq!(mods, CTRL | ALT);
/// ```
#[macro_export]
macro_rules! modifiers {
    ($($key_mod:tt),+) => {
        $crate::KeyModifiers::from_bits_retain($($key_mod.bits())|+)
    };
}

/// Macro for combining key states together in a way that's valid in `const` contexts. Useful for
/// match expressions.
///
/// # Example
///
/// ```
/// use terminput::{CAPS_LOCK, KeyEventState, NUM_LOCK, states};
///
/// const key_state: KeyEventState = states!(CAPS_LOCK, NUM_LOCK);
/// assert_eq!(key_state, CAPS_LOCK | NUM_LOCK);
/// ```
#[macro_export]
macro_rules! states {
    ($($state:tt),+) => {
        $crate::KeyEventState::from_bits_retain($($state.bits())|+)
    };
}
