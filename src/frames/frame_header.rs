use crate::StreamId;
use libnghttp2_sys::nghttp2_frame_hd;

/// Access the Header fields on a Frame.
pub trait FrameHeader {
  /// Access the header directly.
  fn header(&self) -> &nghttp2_frame_hd;

  /// Get the flags from the Frame Header.
  ///
  /// TODO(yw): figure out which flags
  /// TODO(yw): create a flags type
  #[inline]
  fn flags(&self) -> u8 {
    self.header().flags
  }

  /// Access the stream identifier (aka, stream ID).
  #[inline]
  fn stream_id(&self) -> StreamId {
    self.header().stream_id
  }
}
