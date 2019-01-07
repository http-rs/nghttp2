use super::StreamId;
use libnghttp2_sys::nghttp2_priority_spec;
use super::StreamExclusivity;

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
