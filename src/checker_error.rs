//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::{fmt::Display, io, path::PathBuf, process};
use toml;

#[derive(Debug)]
pub enum Stage {
    Compile,
    LaunchAccepted,
    LaunchTested,
}

#[derive(Debug)]
pub enum CheckerError {
    CfgFileNotFoundError {
        tried_files: [PathBuf; 3]
    },
    CfgFileReadingError {
        err: io::Error,
        file: PathBuf,
    },
    CfgFileParsingError {
        err: toml::de::Error,
        file: PathBuf,
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
    pub fn get_exit_code(&self) -> i32 {
        todo!() // TODO
    }

    pub fn destruct(&self) -> ! {
        eprintln!("{}", self);
        process::exit(self.get_exit_code());
    }
}
