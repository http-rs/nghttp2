use bitflags::bitflags;

bitflags! {
  /// The flags for header inflation.
  #[derive(Default)]
  pub struct FrameHeaderInflateFlags: u32 {
    /// No flag set.
    const NONE = libnghttp2_sys::NGHTTP2_HD_INFLATE_NONE;

    /// Indicates all headers were inflated.
    const FINAL = libnghttp2_sys::NGHTTP2_HD_INFLATE_FINAL;

    /// Indicates a header was emitted.
    const EMIT = libnghttp2_sys::NGHTTP2_HD_INFLATE_EMIT;
  }
}
