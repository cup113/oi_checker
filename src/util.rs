//! A module provides utilities.

pub trait IgnoreResult {
    /// Ignore the result to avoid compiler warnings.
    fn ignore(&self) {
        let _ = self;
    }
}

impl<T, E> IgnoreResult for Result<T, E> {}

/// Line end str (Cross-platform behavior)
pub const LINE_END: &'static str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};
