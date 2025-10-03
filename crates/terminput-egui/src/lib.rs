#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "egui_0_32")]
mod mapping;
#[cfg(feature = "egui_0_32")]
pub use mapping::*;
