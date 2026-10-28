#![deny(clippy::unwrap_used)]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "termina_0_4")]
mod mapping;
#[cfg(feature = "termina_0_4")]
pub use mapping::*;
