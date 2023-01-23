//! A module provides utilities.

use crate::prelude::{CheckerResult, CheckerError};
use std::{ffi::OsStr, path::PathBuf};

pub trait IgnoreResult {
    /// Ignore the result.
    fn ignore(&self) {
        let _ = self;
    }
}

impl<T, E> IgnoreResult for Result<T, E> {}

pub trait TryToString {
    /// Try to convert something into string.
    ///
    /// While meeting non-UTF-8 characters, Throw `Box<CheckerError::OsStrUtf8Error>`
    fn try_to_string(&self) -> CheckerResult<String>;
}

impl TryToString for OsStr {
    fn try_to_string(&self) -> CheckerResult<String> {
        match self.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(Box::new(CheckerError::OsStrUtf8Error {
                s: self.to_os_string(),
            })),
        }
    }
}

impl TryToString for PathBuf {
    fn try_to_string(&self) -> CheckerResult<String> {
        self.as_os_str().try_to_string()
    }
}

/// Line end str (Cross-platform behavior)
pub const LINE_END: &'static str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};
