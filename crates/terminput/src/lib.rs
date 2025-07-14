#![no_std]
#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod encoder;
mod key;
mod mouse;
#[cfg(feature = "std")]
mod parser;

use alloc::string::String;
use core::error::Error;
use core::fmt;

#[cfg(feature = "std")]
pub use encoder::*;
pub use key::*;
pub use mouse::*;

/// The supplied event could not be converted into the requested type.
#[derive(Debug)]
pub struct UnsupportedEvent(pub String);

impl fmt::Display for UnsupportedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported event: {}", self.0)
    }
}

impl Error for UnsupportedEvent {}

/// An application event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Event {
    /// The application gained focus.
    FocusGained,
    /// The application lost focus.
    FocusLost,
    /// A keyboard input event.
    Key(KeyEvent),
    /// A mouse input event.
    Mouse(MouseEvent),
    /// A string that was pasted into the application.
    Paste(String),
    /// An resize event with new dimensions after resize.
    Resize {
        /// New number of rows.
        rows: u32,
        /// New number of columns.
        cols: u32,
    },
}

/// Whether to include [`KeyEventKind::Repeat`] when checking for key down events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repeats {
    /// Include key repeat events.
    Include,
    /// Exclude key repeat events.
    Exclude,
}

impl Event {
    /// Returns the contained event if this is a [`KeyEvent`].
    pub fn as_key(&self) -> Option<KeyEvent> {
        if let Self::Key(key_event) = self {
            Some(*key_event)
        } else {
            None
        }
    }

    /// Returns whether the event is a [`KeyEvent`].
    pub fn is_key(&self) -> bool {
        self.as_key().is_some()
    }

    /// Returns the contained event if it is a key press event.
    ///
    /// If [`Repeats::Include`] is passed, both [`KeyEventKind::Press`] and [`KeyEventKind::Repeat`]
    /// are considered press events.
    ///
    /// If [`Repeats::Exclude`] is passed, only [`KeyEventKind::Press`] is considered.
    pub fn as_key_press(&self, repeats: Repeats) -> Option<KeyEvent> {
        if repeats == Repeats::Include {
            if let Self::Key(
                key_event @ KeyEvent {
                    kind: KeyEventKind::Press | KeyEventKind::Repeat,
                    ..
                },
            ) = self
            {
                Some(*key_event)
            } else {
                None
            }
        } else if let Self::Key(
            key_event @ KeyEvent {
                kind: KeyEventKind::Press,
                ..
            },
        ) = self
        {
            Some(*key_event)
        } else {
            None
        }
    }

    /// Returns whether the contained event is a key press event.
    ///
    /// If [`Repeats::Include`] is passed, both [`KeyEventKind::Press`] and [`KeyEventKind::Repeat`]
    /// are considered press events.
    ///
    /// If [`Repeats::Exclude`] is passed, only [`KeyEventKind::Press`] is considered.
    pub fn is_key_press(&self, repeats: Repeats) -> bool {
        self.as_key_press(repeats).is_some()
    }

    /// Returns the contained event if it is a [`KeyEventKind::Repeat`] key event.
    pub fn as_key_repeat(&self) -> Option<KeyEvent> {
        if let Self::Key(
            key_event @ KeyEvent {
                kind: KeyEventKind::Repeat,
                ..
            },
        ) = self
        {
            Some(*key_event)
        } else {
            None
        }
    }

    /// Returns whether the contained event is a [`KeyEventKind::Repeat`] key event.
    pub fn is_key_repeat(&self) -> bool {
        self.as_key_repeat().is_some()
    }

    /// Returns the contained event if it is a [`KeyEventKind::Release`] key event.
    pub fn as_key_release(&self) -> Option<KeyEvent> {
        if let Self::Key(
            key_event @ KeyEvent {
                kind: KeyEventKind::Release,
                ..
            },
        ) = self
        {
            Some(*key_event)
        } else {
            None
        }
    }

    /// Returns whether the contained event is a [`KeyEventKind::Release`] key event.
    pub fn is_key_release(&self) -> bool {
        self.as_key_release().is_some()
    }

    /// Returns the contained event if it is a [`MouseEvent`].
    pub fn as_mouse(&self) -> Option<MouseEvent> {
        if let Self::Mouse(mouse_event) = self {
            Some(*mouse_event)
        } else {
            None
        }
    }

    /// Returns whether the contained event is a [`MouseEvent`].
    pub fn is_mouse(&self) -> bool {
        self.as_mouse().is_some()
    }

    /// Returns the contained event if it is a [`MouseEventKind::Down`] mouse event.
    pub fn as_mouse_down(&self) -> Option<(MouseEvent, MouseButton)> {
        if let Self::Mouse(
            mouse_event @ MouseEvent {
                kind: MouseEventKind::Down(button),
                ..
            },
        ) = self
        {
            Some((*mouse_event, *button))
        } else {
            None
        }
    }

    /// Returns whether the event is a [`MouseEventKind::Down`] mouse event.
    pub fn is_mouse_down(&self) -> bool {
        self.as_mouse_down().is_some()
    }

    /// Returns the contained event if it is a [`MouseEventKind::Up`] mouse event.
    pub fn as_mouse_up(&self) -> Option<(MouseEvent, MouseButton)> {
        if let Self::Mouse(
            mouse_event @ MouseEvent {
                kind: MouseEventKind::Up(button),
                ..
            },
        ) = self
        {
            Some((*mouse_event, *button))
        } else {
            None
        }
    }

    /// Returns whether the event is a [`MouseEventKind::Up`] mouse event.
    pub fn is_mouse_up(&self) -> bool {
        self.as_mouse_up().is_some()
    }

    /// Returns the contained event if it is a [`MouseEventKind::Drag`] mouse event.
    pub fn as_mouse_drag(&self) -> Option<(MouseEvent, MouseButton)> {
        if let Self::Mouse(
            mouse_event @ MouseEvent {
                kind: MouseEventKind::Drag(button),
                ..
            },
        ) = self
        {
            Some((*mouse_event, *button))
        } else {
            None
        }
    }

    /// Returns whether the event is a [`MouseEventKind::Drag`] mouse event.
    pub fn is_mouse_drag(&self) -> bool {
        self.as_mouse_drag().is_some()
    }

    /// Returns the contained event if it is a [`MouseEventKind::Moved`] mouse event.
    pub fn as_mouse_move(&self) -> Option<MouseEvent> {
        if let Self::Mouse(
            mouse_event @ MouseEvent {
                kind: MouseEventKind::Moved,
                ..
            },
        ) = self
        {
            Some(*mouse_event)
        } else {
            None
        }
    }

    /// Returns whether the event is a [`MouseEventKind::Moved`] mouse event.
    pub fn is_mouse_move(&self) -> bool {
        self.as_mouse_move().is_some()
    }

    /// Returns the contained event if it is a [`MouseEventKind::Scroll`] mouse event.
    pub fn as_mouse_scroll(&self) -> Option<(MouseEvent, ScrollDirection)> {
        if let Self::Mouse(
            mouse_event @ MouseEvent {
                kind: MouseEventKind::Scroll(direction),
                ..
            },
        ) = self
        {
            Some((*mouse_event, *direction))
        } else {
            None
        }
    }

    /// Returns whether the event is a [`MouseEventKind::Scroll`] mouse event.
    pub fn is_mouse_scroll(&self) -> bool {
        self.as_mouse_scroll().is_some()
    }

    /// Returns whether the event is [`Event::FocusGained`].
    pub fn is_focus_gained(&self) -> bool {
        *self == Self::FocusGained
    }

    /// Returns whether the event is [`Event::FocusLost`].
    pub fn is_focus_lost(&self) -> bool {
        *self == Self::FocusLost
    }

    /// Returns the pasted text if the event is [`Event::Paste`].
    pub fn as_paste(&self) -> Option<&str> {
        if let Self::Paste(text) = self {
            Some(text)
        } else {
            None
        }
    }

    /// Returns whether the event is [`Event::Paste`].
    pub fn is_paste(&self) -> bool {
        self.as_paste().is_some()
    }

    /// Returns the `(rows, cols)` from the contained event if it is a [`Event::Resize`] event.
    pub fn as_resize(&self) -> Option<(u32, u32)> {
        if let Self::Resize { rows, cols } = self {
            Some((*rows, *cols))
        } else {
            None
        }
    }

    /// Returns whether the event is [`Event::Resize`].
    pub fn is_resize(&self) -> bool {
        self.as_resize().is_some()
    }
}
