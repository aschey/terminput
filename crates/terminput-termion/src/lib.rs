#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(not(windows))]
mod termion;
#[cfg(not(windows))]
pub use termion::*;
