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
