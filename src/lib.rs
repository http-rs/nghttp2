#![forbid(future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! ```

mod error;
pub mod frames;
mod settings;

/// The Stream identifier.
pub type StreamId = i32;

pub use crate::error::{Error, ErrorKind, Result};
pub use crate::settings::SettingsKind;
