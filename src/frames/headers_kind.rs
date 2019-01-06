use libnghttp2_sys::nghttp2_headers_category;

/// The category of HEADERS, which indicates the role of the frame.  In
/// HTTP/2 spec, request, response, push response and other arbitrary
/// headers (e.g., trailer fields) are all called just HEADERS.  To
/// give the application the role of incoming HEADERS frame, we define
/// several categories.
pub enum HeadersKind {
  /// The HEADERS frame is opening new stream, which is analogous to
  /// SYN_STREAM in SPDY.
  Request,
  /// The HEADERS frame is the first response headers, which is
  /// analogous to SYN_REPLY in SPDY.
  Response,
  /// The HEADERS frame is the first headers sent against reserved
  /// stream.
  PushResponse,
  /// The HEADERS frame which does not apply for the above categories,
  /// which is analogous to HEADERS in SPDY.  If non-final response
  /// (e.g., status 1xx) is used, final response HEADERS frame will be
  /// categorized here.
  Headers,
}

#[doc(hidden)]
impl From<nghttp2_headers_category> for HeadersKind {
  #[inline]
  fn from(cat: nghttp2_headers_category) -> Self {
    match cat {
      libnghttp2_sys::NGHTTP2_HCAT_REQUEST => HeadersKind::Request,
      libnghttp2_sys::NGHTTP2_HCAT_RESPONSE => HeadersKind::Response,
      libnghttp2_sys::NGHTTP2_HCAT_PUSH_RESPONSE => HeadersKind::PushResponse,
      libnghttp2_sys::NGHTTP2_HCAT_HEADERS => HeadersKind::Headers,
    }
  }
}

#[doc(hidden)]
impl Into<nghttp2_headers_category> for HeadersKind {
  #[inline]
  fn into(self) -> nghttp2_headers_category {
    match cat {
      HeadersKind::Request => libnghttp2_sys::NGHTTP2_HCAT_REQUEST,
      HeadersKind::Response => libnghttp2_sys::NGHTTP2_HCAT_RESPONSE,
      HeadersKind::PushResponse => libnghttp2_sys::NGHTTP2_HCAT_PUSH_RESPONSE,
      HeadersKind::Headers => libnghttp2_sys::NGHTTP2_HCAT_HEADERS,
    }
  }
}
