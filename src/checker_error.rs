//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::{fmt::Display, io, path::PathBuf, process, ffi::OsString};
use toml;

/// Which stage the error occurs
#[derive(Debug)]
pub enum Stage {
    Compile,
    LaunchAccepted,
    LaunchTested,
}

/// All error variants in OI Checker
#[derive(Debug)]
pub enum CheckerError {
    OsStrUtf8Error {
        s: OsString
    },
    CfgFileNotFoundError {
        tried_files: [PathBuf; 3],
    },
    CfgFileReadingError {
        err: io::Error,
        file: PathBuf,
    },
    CfgFileParsingError {
        err: toml::de::Error,
        file: PathBuf,
    },
    CfgIntegrateError {
        msg: String,
        file_source: PathBuf,
    },
    CompileError {
        file: PathBuf,
        code: i32,
    },
    ArgFormattingTokenError {
        stage: Stage,
        pattern: String,
        desc: String,
        pos: usize,
    },
    ArgFormattingKeyError {
        stage: Stage,
        pattern: String,
        key: String,
        pos: usize,
    },
    CleanFilesError {
        err: io::Error,
        file: PathBuf,
    },
}

impl Display for CheckerError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!() // TODO
    }
}

impl CheckerError {
    /// Get the exit code of each specific error type
    pub fn get_exit_code(&self) -> i32 {
        todo!() // TODO
    }

    /// Print the error message to `stderr` and exit with the provided code
    pub fn destruct(&self) -> ! {
        eprintln!("{:?}", self); // TODO
        process::exit(self.get_exit_code());
    }
}
