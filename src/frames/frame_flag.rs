use bitflags::bitflags;

bitflags! {
  /// The flags for HTTP/2 frames.  This enum defines all flags for all frames.
  #[derive(Default)]
  pub struct FrameFlag: u32 {
    /// No flag set.
    const NONE = libnghttp2_sys::NGHTTP2_FLAG_NONE;

    /// The END_STREAM flag.
    const END_STREAM = libnghttp2_sys::NGHTTP2_FLAG_END_STREAM;

    /// The END_HEADERS flag.
    const END_HEADERS = libnghttp2_sys::NGHTTP2_FLAG_END_HEADERS;

    /// The ACK flag
    const ACK = libnghttp2_sys::NGHTTP2_FLAG_ACK;

    /// The PADDED flag.
    const PADDED = libnghttp2_sys::NGHTTP2_FLAG_PADDED;

    /// The PRIORITY flag.
    const PRIORITY = libnghttp2_sys::NGHTTP2_FLAG_PRIORITY;
  }
}

impl Into<u8> for FrameFlag {
  #[inline]
  fn into(self) -> u8 {
    self.bits as u8
  }
}
