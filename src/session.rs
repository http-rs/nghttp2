use crate::StreamId;
use libnghttp2_sys::nghttp2_session;

/// Session type.
#[derive(Debug)]
pub struct Session {
  inner: nghttp2_session,
}

impl Session {
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
  pub unsafe fn upgrade(
    &mut self,
    settings_payload: *const u8,
    settings_payloadlen: usize,
    head_request: std::os::raw::c_int,
    stream_user_data: *mut std::ffi::c_void,
  ) -> Result<(), crate::error::Error> {
    // `nghttp2_session_upgrade` has been deprecated. Only use upgrade2.
    let res = unsafe {
      libnghttp2_sys::nghttp2_session_upgrade2(
        &mut self.inner,
        settings_payload,
        settings_payloadlen,
        head_request,
        stream_user_data,
      )
    };
    match res {
      0 => Ok(()),
      n => Err(n.into()),
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
