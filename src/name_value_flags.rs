use bitflags::bitflags;

bitflags! {
  /// The flags for header field name/value pair.
  #[derive(Default)]
  pub struct NameValueFlags: u32 {
    /// No flag set.
    const NONE = libnghttp2_sys::NGHTTP2_NV_FLAG_NONE;

    /// Indicates that this name/value pair must not be indexed ("Literal
    /// Header Field never Indexed" representation must be used in HPACK
    /// encoding).  Other implementation calls this bit as "sensitive".
    const NO_INDEX = libnghttp2_sys::NGHTTP2_NV_FLAG_NO_INDEX;

    /// This flag is set solely by application.  If this flag is set, the
    /// library does not make a copy of header field name.  This could
    /// improve performance.
    const NO_COPY_NAME = libnghttp2_sys::NGHTTP2_NV_FLAG_NO_COPY_NAME;

    /// This flag is set solely by application.  If this flag is set, the
    /// library does not make a copy of header field value.  This could
    /// improve performance.
    const NO_COPY_VALUE = libnghttp2_sys::NGHTTP2_NV_FLAG_NO_COPY_VALUE;
  }
}
