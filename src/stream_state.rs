use libnghttp2_sys::nghttp2_stream_proto_state;
use std::hint::unreachable_unchecked;

/// State of stream as described in RFC 7540.
#[derive(Debug, Clone)]
pub enum StreamState {
  /// idle state.
  Idle,
  /// open state.
  Open,
  /// reserved (local) state.
  ReservedLocal,
  /// reserved (remote) state.
  ReservedRemote,
  /// half closed (local) state.
  HalfClosedLocal,
  /// half closed (remote) state.
  HalfClosedRemote,
  /// closed state.
  Closed,
}

#[doc(hidden)]
impl From<nghttp2_stream_proto_state> for StreamState {
  #[inline]
  fn from(from: nghttp2_stream_proto_state) -> Self {
    match from {
      libnghttp2_sys::NGHTTP2_STREAM_STATE_IDLE => StreamState::Idle,
      libnghttp2_sys::NGHTTP2_STREAM_STATE_OPEN => StreamState::Open,
      libnghttp2_sys::NGHTTP2_STREAM_STATE_RESERVED_LOCAL => {
        StreamState::ReservedLocal
      }
      libnghttp2_sys::NGHTTP2_STREAM_STATE_RESERVED_REMOTE => {
        StreamState::ReservedRemote
      }
      libnghttp2_sys::NGHTTP2_STREAM_STATE_HALF_CLOSED_LOCAL => {
        StreamState::HalfClosedLocal
      }
      libnghttp2_sys::NGHTTP2_STREAM_STATE_HALF_CLOSED_REMOTE => {
        StreamState::HalfClosedRemote
      }
      libnghttp2_sys::NGHTTP2_STREAM_STATE_CLOSED => StreamState::Closed,
      _ => unsafe { unreachable_unchecked() },
    }
  }
}

#[doc(hidden)]
impl Into<nghttp2_stream_proto_state> for StreamState {
  #[inline]
  fn into(self) -> nghttp2_stream_proto_state {
    match self {
      StreamState::Idle => libnghttp2_sys::NGHTTP2_STREAM_STATE_IDLE,
      StreamState::Open => libnghttp2_sys::NGHTTP2_STREAM_STATE_OPEN,
      StreamState::ReservedLocal => {
        libnghttp2_sys::NGHTTP2_STREAM_STATE_RESERVED_LOCAL
      }
      StreamState::ReservedRemote => {
        libnghttp2_sys::NGHTTP2_STREAM_STATE_RESERVED_REMOTE
      }
      StreamState::HalfClosedLocal => {
        libnghttp2_sys::NGHTTP2_STREAM_STATE_HALF_CLOSED_LOCAL
      }
      StreamState::HalfClosedRemote => {
        libnghttp2_sys::NGHTTP2_STREAM_STATE_HALF_CLOSED_REMOTE
      }
      StreamState::Closed => libnghttp2_sys::NGHTTP2_STREAM_STATE_CLOSED,
    }
  }
}
