use crate::StreamId;
use libnghttp2_sys::nghttp2_priority_spec;
use std::num::NonZeroU8;

/// Determine the exclusivity of a stream.
#[derive(Debug, Clone)]
pub enum StreamExclusivity {
  /// The stream is an exclusive dependency.
  Exclusive,
  /// The stream is a non-exclusive dependency.
  Inclusive(NonZeroU8),
}

impl StreamExclusivity {
  /// Create an instance from a `u8`.
  #[inline]
  pub fn from_u8(n: u8) -> Self {
    match n {
      0 => StreamExclusivity::Exclusive,
      n => {
        let n = unsafe { NonZeroU8::new_unchecked(n) };
        StreamExclusivity::Inclusive(n)
      }
    }
  }
}

/// The structure to specify stream dependency.
#[derive(Debug, Clone)]
pub struct StreamPriority {
  inner: nghttp2_priority_spec,
}

impl StreamPriority {
  /// Get the Stream ID.
  #[inline]
  pub fn stream_id(&self) -> StreamId {
    self.inner.stream_id
  }

  /// The weight of this dependency.
  #[inline]
  pub fn weight(&self) -> i32 {
    self.inner.weight
  }

  /// The weight of this dependency.
  #[inline]
  pub fn exclusivity(&self) -> StreamExclusivity {
    StreamExclusivity::from_u8(self.inner.exclusive)
  }
}
