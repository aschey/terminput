#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "egui")]
mod egui;
pub mod encoder;
mod key;
mod mouse;
pub mod parser;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;

use core::fmt;
use std::error::Error;

pub use key::*;
pub use mouse::*;

#[derive(Debug)]
pub struct UnsupportedEvent(String);

impl fmt::Display for UnsupportedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported event: {}", self.0)
    }
}

impl Error for UnsupportedEvent {}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum Event {
    /// The terminal gained focus
    FocusGained,
    /// The terminal lost focus
    FocusLost,
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A single mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// A string that was pasted into the terminal. Only emitted if bracketed paste has been
    /// enabled.
    Paste(String),
    /// An resize event with new dimensions after resize (columns, rows).
    /// **Note** that resize events can occur in batches.
    Resize(u16, u16),
}
