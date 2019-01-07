#![forbid(future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! ```

mod error;
mod frames;
mod name_value_flags;
mod settings;
mod stream;

/// The Settings identifier.
pub type SettingsId = libnghttp2_sys::nghttp2_settings_id;

pub use crate::error::*;
pub use crate::frames::*;
pub use crate::name_value_flags::*;
pub use crate::settings::*;
pub use crate::stream::*;
