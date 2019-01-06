use failure::{self, Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::result;

use libnghttp2_sys::nghttp2_error;

/// A specialized [`Result`] type for this crate's operations.
///
/// This is generally used to avoid writing out [Error] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
/// [`Error`]: std.struct.Error.html
pub type Result<T> = result::Result<T, failure::Error>;

/// A list enumerating the categories of errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] struct.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug, Fail)]
pub enum ErrorKind {
  /// Invalid argument passed.
  #[fail(display = "Invalid argument passed.")]
  InvalidArgument,

  /// Out of buffer space.
  #[fail(display = "Out of buffer space.")]
  BufferError,

  /// Out of buffer space.
  #[fail(display = "The specified protocol version is not supported.")]
  UnsupportedVersion,

  /// Indicate that the operation would block.
  #[fail(display = "Indicate that the operation would block.")]
  WouldBlock,

  /// General protocol error
  #[fail(display = "General protocol error")]
  Proto,

  /// The frame is invalid.
  #[fail(display = "The frame is invalid.")]
  InvalidFrame,

  /// The peer performed a shutdown on the connection.
  #[fail(display = "The peer performed a shutdown on the connection.")]
  Eof,

  /// Used as a return value from :func:nghttp2DataSourceReadCallback to indicate that data transfer is postponed. See :func:nghttp2DataSourceReadCallback for details.
  #[fail(
    display = "Used as a return value from :func:nghttp2DataSourceReadCallback to indicate that data transfer is postponed. See :func:nghttp2DataSourceReadCallback for details."
  )]
  Deferred,

  /// Stream ID has reached the maximum value. Therefore no stream ID is available.
  #[fail(
    display = "Stream ID has reached the maximum value. Therefore no stream ID is available."
  )]
  StreamIdNotAvailable,

  /// The stream is already closed; or the stream ID is invalid.
  #[fail(
    display = "The stream is already closed; or the stream ID is invalid."
  )]
  StreamClosed,

  /// RST_STREAM has been added to the outbound queue. The stream is in closing state.
  #[fail(
    display = "RST_STREAM has been added to the outbound queue. The stream is in closing state."
  )]
  StreamClosing,

  /// The transmission is not allowed for this stream (e.g., a frame with END_STREAM flag set has already sent).
  #[fail(
    display = "The transmission is not allowed for this stream (e.g., a frame with END_STREAM flag set has already sent)."
  )]
  StreamShutWr,

  /// The stream ID is invalid.
  #[fail(display = "The stream ID is invalid.")]
  InvalidStreamId,

  /// The state of the stream is not valid (e.g., DATA cannot be sent to the stream if response HEADERS has not been sent).
  #[fail(
    display = "The state of the stream is not valid (e.g., DATA cannot be sent to the stream if response HEADERS has not been sent)."
  )]
  InvalidStreamState,

  /// Another DATA frame has already been deferred.
  #[fail(display = "Another DATA frame has already been deferred.")]
  DeferredDataExist,

  /// Starting new stream is not allowed (e.g., GOAWAY has been sent and/or received).
  #[fail(
    display = "Starting new stream is not allowed (e.g., GOAWAY has been sent and/or received)."
  )]
  StartStreamNotAllowed,

  /// GOAWAY has already been sent.
  #[fail(display = "GOAWAY has already been sent.")]
  GoawayAlreadySent,

  /// The received frame contains the invalid header block (e.g., There are duplicate header names; or the header names are not encoded in US-ASCII character set and not lower cased; or the header name is zero-length string; or the header value contains multiple in-sequence NUL bytes).
  #[fail(
    display = "The received frame contains the invalid header block (e.g., There are duplicate header names; or the header names are not encoded in US-ASCII character set and not lower cased; or the header name is zero-length string; or the header value contains multiple in-sequence NUL bytes)."
  )]
  InvalidHeaderBlock,

  /// Indicates that the context is not suitable to perform the requested operation.
  #[fail(
    display = "Indicates that the context is not suitable to perform the requested operation."
  )]
  InvalidState,

  /// The user callback function failed due to the temporal error.
  #[fail(
    display = "The user callback function failed due to the temporal error."
  )]
  TemporalCallbackFailure,

  /// The length of the frame is invalid, either too large or too small.
  #[fail(
    display = "The length of the frame is invalid, either too large or too small."
  )]
  FrameSizeError,

  /// Header block inflate/deflate error.
  #[fail(display = "Header block inflate/deflate error.")]
  HeaderComp,

  /// Flow control error
  #[fail(display = "Flow control error")]
  FlowControl,

  /// Insufficient buffer size given to function.
  #[fail(display = "Insufficient buffer size given to function.")]
  InsuffBufsize,

  /// Callback was paused by the application
  #[fail(display = "Callback was paused by the application")]
  Pause,

  /// There are too many in-flight SETTING frame and no more transmission of SETTINGS is allowed.
  #[fail(
    display = "There are too many in-flight SETTING frame and no more transmission of SETTINGS is allowed."
  )]
  TooManyInflightSettings,

  /// The server push is disabled.
  #[fail(display = "The server push is disabled.")]
  PushDisabled,

  /// DATA or HEADERS frame for a given stream has been already submitted and has not been fully processed yet. Application should wait for the transmission of the previously submitted frame before submitting another.
  #[fail(
    display = "DATA or HEADERS frame for a given stream has been already submitted and has not been fully processed yet. Application should wait for the transmission of the previously submitted frame before submitting another."
  )]
  DataExist,

  /// The current session is closing due to a connection error or nghttp2SessionTerminateSession() is called.
  #[fail(
    display = "The current session is closing due to a connection error or nghttp2SessionTerminateSession() is called."
  )]
  SessionClosing,

  /// Invalid HTTP header field was received and stream is going to be closed.
  #[fail(
    display = "Invalid HTTP header field was received and stream is going to be closed."
  )]
  HttpHeader,

  /// Violation in HTTP messaging rule.
  #[fail(display = "Violation in HTTP messaging rule.")]
  HttpMessaging,

  /// Stream was refused.
  #[fail(display = "Stream was refused.")]
  RefusedStream,

  /// Unexpected internal error, but recovered.
  #[fail(display = "Unexpected internal error, but recovered.")]
  Internal,

  /// Indicates that a processing was canceled.
  #[fail(display = "Indicates that a processing was canceled.")]
  Cancel,

  /// When a local endpoint expects to receive SETTINGS frame, it receives an other type of frame.
  #[fail(
    display = "When a local endpoint expects to receive SETTINGS frame, it receives an other type of frame."
  )]
  SettingsExpected,

  /// The errors < :enum:FATAL mean that the library is under unexpected condition and processing was terminated (e.g., out of memory). If application receives this error code, it must stop using that :type:nghttp2Session object and only allowed operation for that object is deallocate it using nghttp2SessionDel().
  #[fail(
    display = "The errors < :enum:FATAL mean that the library is under unexpected condition and processing was terminated (e.g., out of memory). If application receives this error code, it must stop using that :type:nghttp2Session object and only allowed operation for that object is deallocate it using nghttp2SessionDel()."
  )]
  Fatal,

  /// Out of memory. This is a fatal error.
  #[fail(display = "Out of memory. This is a fatal error.")]
  Nomem,

  /// The user callback function failed. This is a fatal error.
  #[fail(
    display = "The user callback function failed. This is a fatal error."
  )]
  CallbackFailure,

  /// Invalid client magic (see :macro:NGHTTP2_CLIENT_MAGIC) was received and further processing is not possible.
  #[fail(
    display = "Invalid client magic (see :macro:NGHTTP2_CLIENT_MAGIC) was received and further processing is not possible."
  )]
  BadClientMagic,

  /// Possible flooding by peer was detected in this HTTP/2 session. Flooding is measured by how many PING and SETTINGS frames with ACK flag set are queued for transmission. These frames are response for the peer initiated frames, and peer can cause memory exhaustion on server side.
  #[fail(
    display = "Possible flooding by peer was detected in this HTTP/2 session. Flooding is measured by how many PING and SETTINGS frames with ACK flag set are queued for transmission. These frames are response for the peer initiated frames, and peer can cause memory exhaustion on server side."
  )]
  Flooded,
}

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
#[derive(Debug)]
pub struct Error {
  inner: Context<ErrorKind>,
}

impl Error {
  /// Access the [`ErrorKind`] member.
  ///
  /// [`ErrorKind`]: enum.ErrorKind.html
  pub fn kind(&self) -> &ErrorKind {
    &*self.inner.get_context()
  }

  fn from_sys(error_code: nghttp2_error) -> Self {
    let kind = match error_code {
      libnghttp2_sys::NGHTTP2_ERR_INVALID_ARGUMENT => {
        ErrorKind::InvalidArgument
      }
      libnghttp2_sys::NGHTTP2_ERR_BAD_CLIENT_MAGIC => ErrorKind::BadClientMagic,
      libnghttp2_sys::NGHTTP2_ERR_BUFFER_ERROR => ErrorKind::BufferError,
      libnghttp2_sys::NGHTTP2_ERR_CALLBACK_FAILURE => {
        ErrorKind::CallbackFailure
      }
      libnghttp2_sys::NGHTTP2_ERR_CANCEL => ErrorKind::Cancel,
      libnghttp2_sys::NGHTTP2_ERR_DATA_EXIST => ErrorKind::DataExist,
      libnghttp2_sys::NGHTTP2_ERR_DEFERRED => ErrorKind::Deferred,
      libnghttp2_sys::NGHTTP2_ERR_DEFERRED_DATA_EXIST => {
        ErrorKind::DeferredDataExist
      }
      libnghttp2_sys::NGHTTP2_ERR_EOF => ErrorKind::Eof,
      libnghttp2_sys::NGHTTP2_ERR_FATAL => ErrorKind::Fatal,
      libnghttp2_sys::NGHTTP2_ERR_FLOODED => ErrorKind::Flooded,
      libnghttp2_sys::NGHTTP2_ERR_FLOW_CONTROL => ErrorKind::FlowControl,
      libnghttp2_sys::NGHTTP2_ERR_FRAME_SIZE_ERROR => ErrorKind::FrameSizeError,
      libnghttp2_sys::NGHTTP2_ERR_GOAWAY_ALREADY_SENT => {
        ErrorKind::GoawayAlreadySent
      }
      libnghttp2_sys::NGHTTP2_ERR_HEADER_COMP => ErrorKind::HeaderComp,
      libnghttp2_sys::NGHTTP2_ERR_HTTP_HEADER => ErrorKind::HttpHeader,
      libnghttp2_sys::NGHTTP2_ERR_HTTP_MESSAGING => ErrorKind::HttpMessaging,
      libnghttp2_sys::NGHTTP2_ERR_INSUFF_BUFSIZE => ErrorKind::InsuffBufsize,
      libnghttp2_sys::NGHTTP2_ERR_INTERNAL => ErrorKind::Internal,
      libnghttp2_sys::NGHTTP2_ERR_INVALID_ARGUMENT => {
        ErrorKind::InvalidArgument
      }
      libnghttp2_sys::NGHTTP2_ERR_INVALID_FRAME => ErrorKind::InvalidFrame,
      libnghttp2_sys::NGHTTP2_ERR_INVALID_HEADER_BLOCK => {
        ErrorKind::InvalidHeaderBlock
      }
      libnghttp2_sys::NGHTTP2_ERR_INVALID_STATE => ErrorKind::InvalidState,
      libnghttp2_sys::NGHTTP2_ERR_INVALID_STREAM_ID => {
        ErrorKind::InvalidStreamId
      }
      libnghttp2_sys::NGHTTP2_ERR_INVALID_STREAM_STATE => {
        ErrorKind::InvalidStreamState
      }
      libnghttp2_sys::NGHTTP2_ERR_NOMEM => ErrorKind::Nomem,
      libnghttp2_sys::NGHTTP2_ERR_PAUSE => ErrorKind::Pause,
      libnghttp2_sys::NGHTTP2_ERR_PROTO => ErrorKind::Proto,
      libnghttp2_sys::NGHTTP2_ERR_PUSH_DISABLED => ErrorKind::PushDisabled,
      libnghttp2_sys::NGHTTP2_ERR_REFUSED_STREAM => ErrorKind::RefusedStream,
      libnghttp2_sys::NGHTTP2_ERR_SESSION_CLOSING => ErrorKind::SessionClosing,
      libnghttp2_sys::NGHTTP2_ERR_SETTINGS_EXPECTED => {
        ErrorKind::SettingsExpected
      }
      libnghttp2_sys::NGHTTP2_ERR_START_STREAM_NOT_ALLOWED => {
        ErrorKind::StartStreamNotAllowed
      }
      libnghttp2_sys::NGHTTP2_ERR_STREAM_CLOSED => ErrorKind::StreamClosed,
      libnghttp2_sys::NGHTTP2_ERR_STREAM_CLOSING => ErrorKind::StreamClosing,
      libnghttp2_sys::NGHTTP2_ERR_STREAM_ID_NOT_AVAILABLE => {
        ErrorKind::StreamIdNotAvailable
      }
      libnghttp2_sys::NGHTTP2_ERR_STREAM_SHUT_WR => ErrorKind::StreamShutWr,
      libnghttp2_sys::NGHTTP2_ERR_TEMPORAL_CALLBACK_FAILURE => {
        ErrorKind::TemporalCallbackFailure
      }
      libnghttp2_sys::NGHTTP2_ERR_TOO_MANY_INFLIGHT_SETTINGS => {
        ErrorKind::TooManyInflightSettings
      }
      libnghttp2_sys::NGHTTP2_ERR_UNSUPPORTED_VERSION => {
        ErrorKind::UnsupportedVersion
      }
      libnghttp2_sys::NGHTTP2_ERR_WOULDBLOCK => ErrorKind::WouldBlock,
      _ => unimplemented!(),
    };
    let inner = Context::new(kind);
    Error { inner }
  }
}

impl Fail for Error {
  fn cause(&self) -> Option<&dyn Fail> {
    self.inner.cause()
  }

  fn backtrace(&self) -> Option<&Backtrace> {
    self.inner.backtrace()
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.inner, f)
  }
}

impl From<ErrorKind> for Error {
  fn from(kind: ErrorKind) -> Error {
    let inner = Context::new(kind);
    Error { inner }
  }
}

impl From<Context<ErrorKind>> for Error {
  fn from(inner: Context<ErrorKind>) -> Error {
    Error { inner }
  }
}
