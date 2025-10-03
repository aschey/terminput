#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "web_sys_0_3")]
mod mapping;

#[cfg(feature = "web_sys_0_3")]
pub use mapping::*;
