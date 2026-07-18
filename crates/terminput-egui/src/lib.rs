#![deny(clippy::unwrap_used)]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(
    feature = "egui_0_32",
    feature = "egui_0_33",
    feature = "egui_0_34",
    feature = "egui_0_35"
))]
mod mapping;
#[cfg(any(
    feature = "egui_0_32",
    feature = "egui_0_33",
    feature = "egui_0_34",
    feature = "egui_0_35"
))]
pub use mapping::*;
