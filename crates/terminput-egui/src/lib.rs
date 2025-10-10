#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "egui_0_32", feature = "egui_0_33"))]
mod mapping;
#[cfg(any(feature = "egui_0_32", feature = "egui_0_33"))]
pub use mapping::*;
