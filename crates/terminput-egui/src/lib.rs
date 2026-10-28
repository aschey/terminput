#![deny(clippy::unwrap_used)]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "egui_0_35", feature = "egui_0_36", feature = "egui_0_37",))]
mod mapping;
#[cfg(any(feature = "egui_0_35", feature = "egui_0_36", feature = "egui_0_37",))]
pub use mapping::*;
