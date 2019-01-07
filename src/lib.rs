#![forbid(future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! ```

mod frames;
mod error;
mod settings;
mod name_value_flags;
mod stream;

/// The Stream identifier.
pub type StreamId = i32;

/// The Settings identifier.
pub type SettingsId = libnghttp2_sys::nghttp2_settings_id;

pub use crate::name_value_flags::*;
pub use crate::frames::*;
pub use crate::stream::*;
pub use crate::error::*;
pub use crate::settings::*;
