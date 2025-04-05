#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(not(windows))]
mod termion;
#[cfg(not(windows))]
pub use termion::*;
