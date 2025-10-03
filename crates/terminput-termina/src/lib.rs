#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "termina_0_1")]
mod mapping;
#[cfg(feature = "termina_0_1")]
pub use mapping::*;
