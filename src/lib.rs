#![forbid(future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! ```

pub mod frames;

mod error;
mod settings;
mod stream_state;
mod name_value_flags;

/// The Stream identifier.
pub type StreamId = i32;

/// The Settings identifier.
pub type SettingsId = libnghttp2_sys::nghttp2_settings_id;

pub use crate::name_value_flags::NameValueFlags;
pub use crate::stream_state::StreamState;
pub use crate::error::{Error, ErrorKind, Result};
pub use crate::settings::SettingsKind;
