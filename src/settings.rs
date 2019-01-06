use libnghttp2_sys::nghttp2_settings_id;
use std::hint::unreachable_unchecked;

/// Possible values of the `SETTINGS` frame.
#[derive(Debug, Clone)]
pub enum SettingsKind {
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

impl SettingsKind {
  #[inline]
  fn from_sys(setting: nghttp2_settings_id) -> Self {
    match setting {
      libnghttp2_sys::NGHTTP2_SETTINGS_HEADER_TABLE_SIZE => {
        SettingsKind::HeaderTableSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_ENABLE_PUSH => SettingsKind::EnablePush,
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS => {
        SettingsKind::MaxConcurrentStreams
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE => {
        SettingsKind::InitialWindowSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_FRAME_SIZE => {
        SettingsKind::MaxFrameSize
      }
      libnghttp2_sys::NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE => {
        SettingsKind::MaxHeaderListSize
      }
      _ => unsafe { unreachable_unchecked() },
    }
  }

  #[inline]
  fn into_sys(self) -> nghttp2_settings_id {
    match self {
      SettingsKind::HeaderTableSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_HEADER_TABLE_SIZE
      }
      SettingsKind::EnablePush => libnghttp2_sys::NGHTTP2_SETTINGS_ENABLE_PUSH,
      SettingsKind::MaxConcurrentStreams => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_CONCURRENT_STREAMS
      }
      SettingsKind::InitialWindowSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_INITIAL_WINDOW_SIZE
      }
      SettingsKind::MaxFrameSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_FRAME_SIZE
      }
      SettingsKind::MaxHeaderListSize => {
        libnghttp2_sys::NGHTTP2_SETTINGS_MAX_HEADER_LIST_SIZE
      }
    }
  }
}
