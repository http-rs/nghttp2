use super::FrameHeader;

/// The HEADERS frame.
#[derive(Debug, Clone)]
pub struct HeadersFrame {
  inner: libnghttp2_sys::nghttp2_data,
}

impl HeadersFrame {
  /// The length of the padding in this frame. This includes PAD_HIGH and
  /// PAD_LOW.
  #[inline]
  pub fn pad_len(&self) -> usize {
    self.inner.padlen
  }

  /// TODO: docs
  pub fn priority_spec(&self) {
    unimplemented!();
  }

  /// TODO: docs
  pub fn pairs(&self) {
    unimplemented!();
  }

  /// TODO: docs
  pub fn category(&self) {
    unimplemented!();
  }
}

impl FrameHeader for HeadersFrame {
  #[inline]
  fn header(&self) -> &libnghttp2_sys::nghttp2_frame_hd {
    &self.inner.hd
  }
}

#[doc(hidden)]
impl From<libnghttp2_sys::nghttp2_data> for HeadersFrame {
  #[inline]
  fn from(inner: libnghttp2_sys::nghttp2_data) -> Self {
    Self { inner }
  }
}

#[doc(hidden)]
impl Into<libnghttp2_sys::nghttp2_data> for HeadersFrame {
  #[inline]
  fn into(self) -> libnghttp2_sys::nghttp2_data {
    self.inner
  }
}
