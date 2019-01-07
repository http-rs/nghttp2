use std::num::NonZeroU8;

/// Determine the exclusivity of a stream.
#[derive(Debug, Clone)]
pub enum StreamExclusivity {
  /// The stream is an exclusive dependency.
  Exclusive,
  /// The stream is a non-exclusive dependency.
  Inclusive(NonZeroU8),
}

impl StreamExclusivity {
  /// Create an instance from a `u8`.
  #[inline]
  pub fn from_u8(n: u8) -> Self {
    match n {
      0 => StreamExclusivity::Exclusive,
      n => {
        let n = unsafe { NonZeroU8::new_unchecked(n) };
        StreamExclusivity::Inclusive(n)
      }
    }
  }
}
