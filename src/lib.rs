#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod coroutines;
mod io;
mod secret;
#[cfg(feature = "serde")]
pub mod serde;

#[doc(inline)]
pub use self::{io::Io, secret::Secret};
