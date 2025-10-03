#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "termwiz_0_22", feature = "termwiz_0_23"))]
mod mapping;
#[cfg(any(feature = "termwiz_0_22", feature = "termwiz_0_23"))]
pub use mapping::*;
