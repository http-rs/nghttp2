use libnghttp2_sys::nghttp2_frame_type;
use std::hint::unreachable_unchecked;

/// The frame types in HTTP/2 specification.
#[derive(Debug, Clone)]
pub enum FrameKind {
  /// The DATA frame.
  Data,
  /// The HEADERS frame.
  Headers,
  /// The PRIORITY frame.
  Priority,
  /// The RST_STREAM frame.
  RstStream,
  /// The SETTINGS frame.
  Settings,
  /// The PUSH_PROMISE frame.
  PushPromise,
  /// The PING frame.
  Ping,
  /// The GOAWAY frame.
  Goaway,
  /// The WINDOW_UPDATE frame.
  WindowUpdate,
  /// The CONTINUATION frame.
  ///
  /// This frame type won't be passed to any
  /// callbacks because the library processes this frame type and its
  /// preceding HEADERS/PUSH_PROMISE as a single frame.
  Continuation,
  /// The ALTSVC frame, which is defined in [`RFC 7383`].
  ///
  /// [`RFC 7383`]: https://tools.ietf.org/html/rfc7838#section-4.
  Altsvc,
  /// The ORIGIN frame, which is defined by [`RFC 8336`].
  ///
  /// [`RFC 8336`]: https://tools.ietf.org/html/rfc8336.
  Origin,
}

#[doc(hidden)]
impl From<nghttp2_frame_type> for FrameKind {
  #[inline]
  fn from(frame_type: nghttp2_frame_type) -> Self {
    match frame_type {
      libnghttp2_sys::NGHTTP2_DATA => FrameKind::Data,
      libnghttp2_sys::NGHTTP2_HEADERS => FrameKind::Headers,
      libnghttp2_sys::NGHTTP2_PRIORITY => FrameKind::Priority,
      libnghttp2_sys::NGHTTP2_RST_STREAM => FrameKind::RstStream,
      libnghttp2_sys::NGHTTP2_SETTINGS => FrameKind::Settings,
      libnghttp2_sys::NGHTTP2_PUSH_PROMISE => FrameKind::PushPromise,
      libnghttp2_sys::NGHTTP2_PING => FrameKind::Ping,
      libnghttp2_sys::NGHTTP2_GOAWAY => FrameKind::Goaway,
      libnghttp2_sys::NGHTTP2_WINDOW_UPDATE => FrameKind::WindowUpdate,
      libnghttp2_sys::NGHTTP2_CONTINUATION => FrameKind::Continuation,
      libnghttp2_sys::NGHTTP2_ALTSVC => FrameKind::Altsvc,
      libnghttp2_sys::NGHTTP2_ORIGIN => FrameKind::Origin,
      _ => unsafe { unreachable_unchecked() },
    }
  }
}

#[doc(hidden)]
impl Into<nghttp2_frame_type> for FrameKind {
  #[inline]
  fn into(self) -> nghttp2_frame_type {
    match self {
      FrameKind::Data => libnghttp2_sys::NGHTTP2_DATA,
      FrameKind::Headers => libnghttp2_sys::NGHTTP2_HEADERS,
      FrameKind::Priority => libnghttp2_sys::NGHTTP2_PRIORITY,
      FrameKind::RstStream => libnghttp2_sys::NGHTTP2_RST_STREAM,
      FrameKind::Settings => libnghttp2_sys::NGHTTP2_SETTINGS,
      FrameKind::PushPromise => libnghttp2_sys::NGHTTP2_PUSH_PROMISE,
      FrameKind::Ping => libnghttp2_sys::NGHTTP2_PING,
      FrameKind::Goaway => libnghttp2_sys::NGHTTP2_GOAWAY,
      FrameKind::WindowUpdate => libnghttp2_sys::NGHTTP2_WINDOW_UPDATE,
      FrameKind::Continuation => libnghttp2_sys::NGHTTP2_CONTINUATION,
      FrameKind::Altsvc => libnghttp2_sys::NGHTTP2_ALTSVC,
      FrameKind::Origin => libnghttp2_sys::NGHTTP2_ORIGIN,
    }
  }
}
