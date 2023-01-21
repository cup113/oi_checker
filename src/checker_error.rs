//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::{
    ffi::OsString,
    fmt::{Display, Write},
    io,
    path::PathBuf,
    process,
};
use toml;

/// Which stage the error occurs
#[derive(Debug, Clone, Copy)]
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
    CompileError {
        command: String,
        args: Vec<String>,
        file: PathBuf,
        msg: String,
    },
    LaunchError {
        command: String,
        args: Vec<String>,
        file: PathBuf,
        msg: String,
    },
    CleanFilesError {
        err: io::Error,
        file: PathBuf,
    },
}

impl Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        use CheckerError::*;
        match self {
            OsStrUtf8Error { s } => {
                write!(f, "Invalid non-UTF-8 string: {}.", s.to_string_lossy())
            }
            CfgFileNotFoundError { tried_files } => {
                write!(
                    f,
                    "Cannot found config file. We have tried {}, {} and {}.",
                    tried_files[0].display(),
                    tried_files[1].display(),
                    tried_files[2].display(),
                )
            }
            CfgFileReadingError { err, file } => todo!(),
            CfgFileParsingError { err, file } => {
                write!(f, "Parse config file ({}) failed:\n{}", file.display(), err)
            }
            CfgIntegrateError { msg, file_source } => todo!(),
            CreateWorkDirError { err, dir } => todo!(),
            ArgFormattingTokenError {
                stage,
                pattern,
                desc,
                pos,
            } => todo!(),
            ArgFormattingKeyError {
                stage,
                pattern,
                key,
                pos,
            } => todo!(),
            CompileError {
                command,
                args,
                file,
                msg,
            } => todo!(),
            LaunchError {
                command,
                args,
                file,
                msg,
            } => todo!(),
            CleanFilesError { err, file } => todo!(),
        }
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
            ArgFormattingTokenError { .. } => 22,
            ArgFormattingKeyError { .. } => 23,
            CompileError { .. } => 24,
            LaunchError { .. } => unreachable!(),
            CleanFilesError { .. } => 25,
        }
    }

    /// Print the error message to `stderr` and exit with the provided code
    pub fn destruct(&self) -> ! {
        eprintln!("{}", self);
        process::exit(self.get_exit_code());
    }
}
