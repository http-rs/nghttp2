use libnghttp2_sys::nghttp2_stream;

mod stream_exclusivity;
mod stream_priority;
mod stream_state;

pub use crate::stream::stream_exclusivity::*;
pub use crate::stream::stream_priority::*;
pub use crate::stream::stream_state::*;

/// The Stream identifier.
pub type StreamId = i32;

/// The Stream struct.
#[derive(Debug)]
pub struct Stream {
  inner: nghttp2_stream,
}

impl Stream {
  /// Create a new instance.
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: unsafe { std::mem::zeroed() },
    }
  }
}
