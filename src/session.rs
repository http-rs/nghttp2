use crate::StreamId;
use libnghttp2_sys::nghttp2_session;
use crate::Stream;

/// Session type.
#[derive(Debug)]
pub struct Session {
  inner: nghttp2_session,
}

impl Session {
  /// Create a new instance.
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: unsafe { std::mem::zeroed() },
    }
  }

  /// Receives frames from the remote peer.
  ///
  /// ## Errors
  ///
  /// - NGHTTP2_ERR_EOF The remote peer did shutdown on the connection.
  /// - NGHTTP2_ERR_NOMEM Out of memory.
  /// - NGHTTP2_ERR_CALLBACK_FAILURE The callback function failed.
  /// - NGHTTP2_ERR_BAD_CLIENT_MAGIC Invalid client magic was detected. This
  ///   error only returns when |session| was configured as server and
  ///   nghttp2_option_set_no_recv_client_magic() is not used with nonzero
  ///   value.
  /// - NGHTTP2_ERR_FLOODED Flooding was detected in this HTTP/2 session, and it
  ///   must be closed. This is most likely caused by misbehaviour of peer.
  #[inline]
  pub fn receive(&mut self) -> Result<(), crate::error::Error> {
    let res = unsafe { libnghttp2_sys::nghttp2_session_recv(&mut self.inner) };
    match res {
      0 => Ok(()),
      n => Err(n.into()),
    }
  }

  /// Sends pending frames to the remote peer.
  ///
  /// ## Errors
  ///
  /// - NGHTTP2_ERR_NOMEM Out of memory.
  /// - NGHTTP2_ERR_CALLBACK_FAILURE The callback function failed.
  #[inline]
  pub fn send(&mut self) -> Result<(), crate::error::Error> {
    let res = unsafe { libnghttp2_sys::nghttp2_session_send(&mut self.inner) };
    match res {
      0 => Ok(()),
      n => Err(n.into()),
    }
  }

  /// Tell the `Session` that `size` bytes for `stream_id` were consumed, and
  /// are ready to `WINDOW_UPDATE`. The consumed bytes are counted towards both
  /// connection and stream level WINDOW_UPDATE. This function is intended to be
  /// used without automatic window update
  ///
  /// ## Errors
  ///
  /// :enum:NGHTTP2_ERR_NOMEM Out of memory. :enum:NGHTTP2_ERR_INVALID_ARGUMENT
  /// The |stream_id| is 0. :enum:NGHTTP2_ERR_INVALID_STATE Automatic
  /// WINDOW_UPDATE is not disabled.
  #[inline]
  pub fn consume(
    &mut self,
    stream_id: StreamId,
    size: usize,
  ) -> Result<(), crate::error::Error> {
    let res = unsafe {
      libnghttp2_sys::nghttp2_session_consume(&mut self.inner, stream_id, size)
    };
    match res {
      0 => Ok(()),
      n => Err(n.into()),
    }
  }

  /// Performs post-process of HTTP Upgrade request. This function can be called
  /// from both client and server, but the behavior is very different in each
  /// other.
  // TODO: implement Some branch for stream_user_data pointer
  // TODO: create variants for both client and server
  pub fn upgrade(
    &mut self,
    settings_payload: &[u8],
    is_head_request: bool,
  ) -> Result<Option<&Stream>, crate::error::Error> {
    let head_request = match is_head_request {
      true => 0,
      false => 1,
    };

    let stream_ptr = std::ptr::null_mut();
    let payload_len = settings_payload.len();
    let payload_ptr = settings_payload.as_ptr();

    let res = unsafe {
      libnghttp2_sys::nghttp2_session_upgrade2(
        &mut self.inner,
        payload_ptr,
        payload_len,
        head_request,
        stream_ptr,
      )
    };
    match res {
      0 => Ok(None),
      n => Err(n.into()),
    }
  }

  /// Puts back previously deferred DATA frame in the stream `stream_id` to the
  /// outbound queue.
  ///
  /// ## Errors
  ///
  /// NGHTTP2_ERR_INVALID_ARGUMENT The stream does not exist;
  #[inline]
  pub fn resume_data(
    &mut self,
    stream_id: StreamId,
  ) -> Result<(), crate::error::Error> {
    let res = unsafe {
      libnghttp2_sys::nghttp2_session_resume_data(&mut self.inner, stream_id)
    };
    match res {
      0 => Ok(()),
      n => Err(n.into()),
    }
  }

  /// Returns `true` if the session wants to receive data from the remote peer.
  #[inline]
  pub fn want_read(
    &mut self,
  ) -> bool {
    let res = unsafe {
      libnghttp2_sys::nghttp2_session_want_read(&mut self.inner)
    };
    match res {
      0 => false,
      _ => true,
    }
  }

  /// Returns `true` if the session wants to send data to the remote peer.
  #[inline]
  pub fn want_write(
    &mut self,
  ) -> bool {
    let res = unsafe {
      libnghttp2_sys::nghttp2_session_want_write(&mut self.inner)
    };
    match res {
      0 => false,
      _ => true,
    }
  }
}

impl Drop for Session {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      libnghttp2_sys::nghttp2_session_del(&mut self.inner);
    }
  }
}
