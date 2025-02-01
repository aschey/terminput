use crate::KeyModifiers;

/// A mouse event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseEvent {
    /// The kind of mouse event that was caused.
    pub kind: MouseEventKind,
    /// The column that the event occurred on.
    pub column: u16,
    /// The row that the event occurred on.
    pub row: u16,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
}

/// Mouse scroll direction.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollDirection {
    /// Scrolled mouse wheel upwards (away from the user).
    Up,
    /// Scrolled mouse wheel downwards (towards the user).
    Down,
    /// Scrolled mouse wheel left (usually on a laptop touchpad).
    Left,
    /// Scrolled mouse wheel right (usually on a laptop touchpad).
    Right,
}

impl ScrollDirection {
    /// Convenience method for applying a change in the vertical or horizontal direction in response
    /// to a scroll event.
    pub fn delta(&self) -> ScrollDelta {
        match self {
            Self::Up => ScrollDelta { x: 0, y: 1 },
            Self::Down => ScrollDelta { x: 0, y: -1 },
            Self::Left => ScrollDelta { x: 1, y: 0 },
            Self::Right => ScrollDelta { x: -1, y: 0 },
        }
    }
}

/// Represents the change that should be applied to the content in response to a scroll event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollDelta {
    /// Change in the x (horizontal) direction.
    pub x: i32,
    /// Change in the y (vertical) direction.
    pub y: i32,
}

/// The type of mouse event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseEventKind {
    /// Pressed mouse button. Contains the button that was pressed.
    Down(MouseButton),
    /// Released mouse button. Contains the button that was released.
    Up(MouseButton),
    /// Moved the mouse cursor while pressing the contained mouse button.
    Drag(MouseButton),
    /// Moved the mouse cursor while not pressing a mouse button.
    Moved,
    /// Scrolled mouse wheel downwards (towards the user).
    Scroll(ScrollDirection),
}

/// The mouse button used for this event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Mouse button could not be determined.
    Unknown,
}
