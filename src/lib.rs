#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! ```

mod error;
mod frame;

pub use crate::frame::FrameKind;
pub use crate::error::{Error, ErrorKind, Result};
