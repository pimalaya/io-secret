//! Module gathering I/O-free, composable and iterable state machines.
//!
//! Coroutines emit [`crate::Io`] requests that need to be processed by
//! [`crate::handlers`] in order to continue their progression.

mod read;

#[doc(inline)]
pub use self::read::Read;
