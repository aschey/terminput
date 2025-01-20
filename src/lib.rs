//#![deny(missing_docs)]
#![forbid(clippy::unwrap_used)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::missing_fields_in_debug)]
#![warn(clippy::use_self)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "egui")]
mod egui;
mod encoder;
mod key;
mod mouse;
mod parser;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;

use core::fmt;
use std::error::Error;

pub use encoder::*;
pub use key::*;
pub use mouse::*;
pub use parser::*;

/// The supplied event could not be converted into the requested type.
#[derive(Debug)]
pub struct UnsupportedEvent(String);

impl fmt::Display for UnsupportedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported event: {}", self.0)
    }
}

impl Error for UnsupportedEvent {}

/// An application event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
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
