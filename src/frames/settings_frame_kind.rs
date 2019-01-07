use libnghttp2_sys::nghttp2_settings_id;
use std::hint::unreachable_unchecked;

/// Possible values of the `SETTINGS` frame.
#[derive(Debug, Clone)]
pub enum SettingsFrameKind {
  /// Table size.
  HeaderTableSize,
  /// Enable push.
  EnablePush,
  /// Max concurrent streams.
  MaxConcurrentStreams,
  /// Initial window size.
  InitialWindowSize,
  /// Max frame size.
  MaxFrameSize,
  /// Max header list size.
  MaxHeaderListSize,
}

impl SettingsFrameKind {
  #[inline]
  fn from_sys(setting: nghttp2_settings_id) -> Self {
    match setting {
      libnghttp2_sys::NGHTTP2_SETTINGS_HEADER_TABLE_SIZE => {
        SettingsFrameKind::HeaderTableSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_ENABLE_PUSH => SettingsFrameKind::EnablePush,
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS => {
        SettingsFrameKind::MaxConcurrentStreams
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE => {
        SettingsFrameKind::InitialWindowSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_FRAME_SIZE => {
        SettingsFrameKind::MaxFrameSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE => {
        SettingsFrameKind::MaxHeaderListSize
      }
      _ => unsafe { unreachable_unchecked() },
    }
  }

  #[inline]
  fn into_sys(self) -> nghttp2_settings_id {
    match self {
      SettingsFrameKind::HeaderTableSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_HEADER_TABLE_SIZE
      }
      SettingsFrameKind::EnablePush => libnghttp2_sys::NGHTTP2_SETTINGS_ENABLE_PUSH,
      SettingsFrameKind::MaxConcurrentStreams => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS
      }
      SettingsFrameKind::InitialWindowSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE
      }
      SettingsFrameKind::MaxFrameSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_FRAME_SIZE
      }
      SettingsFrameKind::MaxHeaderListSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE
      }
    }
  }
}
