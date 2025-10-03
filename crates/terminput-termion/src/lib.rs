#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(all(not(windows), feature = "termion_4"))]
mod mapping;
#[cfg(all(not(windows), feature = "termion_4"))]
pub use mapping::*;
