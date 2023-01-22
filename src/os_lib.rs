//! A module provides UTF-8 path parsing.
//!
//! Error while detecting non UTF-8 characters.

use crate::{BoxedCheckerError, CheckerError};
use std::{ffi::OsStr, path::PathBuf};
pub trait TryToString {
    /// Try to convert something into string.
    ///
    /// ## Error
    ///
    /// While meeting non-UTF-8 characters, Throw `Box<CheckerError::OsStrUtf8Error>`
    fn try_to_string(&self) -> Result<String, BoxedCheckerError>;
}

impl TryToString for OsStr {
    fn try_to_string(&self) -> Result<String, BoxedCheckerError> {
        match self.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(Box::new(CheckerError::OsStrUtf8Error {
                s: self.to_os_string(),
            })),
        }
    }
}

impl TryToString for PathBuf {
    fn try_to_string(&self) -> Result<String, BoxedCheckerError> {
        self.as_os_str().try_to_string()
    }
}

pub const LINE_END: &'static str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};
