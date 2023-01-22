//! A library provides UTF-8 path parsing and more utilities.
//!
//! Return error while detecting non UTF-8 characters.

use crate::CheckerError;
use std::{ffi::OsStr, path::PathBuf};
pub trait TryToString {
    fn try_to_string(&self) -> Result<String, CheckerError>;
}

impl TryToString for OsStr {
    fn try_to_string(&self) -> Result<String, CheckerError> {
        match self.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(CheckerError::OsStrUtf8Error {
                s: self.to_os_string(),
            }),
        }
    }
}

impl TryToString for PathBuf {
    fn try_to_string(&self) -> Result<String, CheckerError> {
        self.as_os_str().try_to_string()
    }
}
