use crate::StreamId;

/// Access the Header fields on a Frame
pub trait Header {
  /// Get the flags from the Frame Header.
  ///
  /// TODO(yw): figure out which flags
  /// TODO(yw): create a flags type
  fn flags(&self) -> u8;

  /// Access the stream identifier (aka, stream ID).
  fn stream_id(&self) -> StreamId;
}
