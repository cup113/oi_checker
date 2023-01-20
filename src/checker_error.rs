//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::{ffi::OsString, fmt::Display, io, path::PathBuf, process};
use toml;

/// Which stage the error occurs
#[derive(Debug)]
pub enum Stage {
    CompileDG,
    CompileAC,
    CompileTP,
    LaunchDG,
    LaunchAC,
    LaunchTP,
}

/// All error variants in OI Checker
#[derive(Debug)]
pub enum CheckerError {
    OsStrUtf8Error {
        s: OsString,
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
    CreateWorkDirError {
        err: io::Error,
        dir: PathBuf,
    },
    CompileError {
        command: String,
        args: Vec<String>,
        file: PathBuf,
        msg: String,
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
        use CheckerError::*;
        match *self {
            OsStrUtf8Error { .. } => 16,
            CfgFileNotFoundError { .. } => 17,
            CfgFileReadingError { .. } => 18,
            CfgFileParsingError { .. } => 19,
            CfgIntegrateError { .. } => 20,
            CreateWorkDirError { .. } => 21,
            CompileError { .. } => 22,
            ArgFormattingTokenError { .. } => 23,
            ArgFormattingKeyError { .. } => 24,
            CleanFilesError { .. } => 25,
        }
    }

    /// Print the error message to `stderr` and exit with the provided code
    pub fn destruct(&self) -> ! {
        eprintln!("{:?}", self); // TODO
        process::exit(self.get_exit_code());
    }
}
