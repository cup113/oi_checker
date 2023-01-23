//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::ffi::OsString;
use std::fmt::Display;
use std::io;
use std::path::PathBuf;
use toml;

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
        dict_keys: Vec<String>,
        pos: usize,
    },
    CommandError {
        stage: Stage,
        command: String,
        args: Vec<String>,
        file: PathBuf,
        msg: String,
    },
    FilterError {
        filter: crate::config::OutputFilter,
        err: io::Error,
        file: PathBuf,
    },
    DiffToolError {
        command: String,
        args: Vec<String>,
        err: io::Error,
    },
}

pub type BoxedCheckerError = Box<CheckerError>;

impl Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CheckerError::*;
        match self {
            OsStrUtf8Error { s } => write!(
                f,
                "Invalid non-UTF-8 string: {}.\n\
                Help: Make sure path names consist of legal UTF-8 characters.",
                s.to_string_lossy()
            ),
            CfgFileNotFoundError { tried_files } => write!(
                f,
                "Cannot found config file. We have tried {}, {} and {}.\n\
                Help: Make sure at least one config file exist.",
                tried_files[0].display(),
                tried_files[1].display(),
                tried_files[2].display(),
            ),
            CfgFileReadingError { err, file } => write!(
                f,
                "Read config file ({}) failed:\n{}\n\
                Help: Check file permission.",
                file.display(),
                err
            ),
            CfgFileParsingError { err, file } => write!(
                f,
                "Parse config file ({}) failed:\n{}\n\
                Help: Check if the file is TOML grammatical and has all of the \
                fields and correspond types.",
                file.display(),
                err
            ),
            CfgIntegrateError { msg, file_source } => write!(
                f,
                "Error when integrating config (file source: {}): {}\n\
                Help: Check if the value is legal (in options).",
                file_source.display(),
                msg
            ),
            CreateWorkDirError { err, dir } => write!(
                f,
                "Error when creating working directory ({}): {}\n\
                Help: Check directory permission. Avoid using nested path.",
                dir.display(),
                err
            ),
            ArgFormattingTokenError {
                stage,
                pattern,
                desc,
                pos,
            } => write!(
                f,
                "Error when parsing arguments during {}: Token Error ({}) when \
                parsing pattern \"{}\" at pos {}.\n\
                Help: Correct the grammar of formatting",
                stage, desc, pattern, pos
            ),
            ArgFormattingKeyError {
                stage,
                pattern,
                key,
                dict_keys,
                pos,
            } => write!(
                f,
                "Error when parsing arguments during {}: Key Not Found \
                (key: \"{}\") when parsing pattern \"{}\" at pos {}.\n\
                Help: Possible keys are: {}",
                stage,
                key,
                pattern,
                pos,
                dict_keys.join(","),
            ),
            CommandError {
                stage,
                command,
                args,
                file,
                msg,
            } => write!(
                f,
                "Error during {} (file: {}): {}.\n\
                Command: \"{}\" {}\n\
                Help: Check your program or config.",
                stage,
                file.display(),
                msg,
                command,
                args.iter()
                    .map(|arg| format!("\"{}\"", arg))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            FilterError { filter, err, file } => write!(
                f,
                "Error during filtering file {} (filter: {}): {}\n\
                Help: This error shouldn't occur.",
                file.display(),
                filter,
                err
            ),
            DiffToolError { command, args, err } => write!(
                f,
                "Error during comparing files: {}\n\
                Command: \"{}\" {}\n\
                Help: Please check if the different tool program exists.",
                err,
                command,
                args.iter()
                    .map(|arg| format!("\"{}\"", arg))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
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
            CommandError { .. } => 24,
            FilterError { .. } => 25,
            DiffToolError { .. } => 26,
        }
    }

    /// Print the error message to `stderr` and exit with the provided code
    pub fn destruct(&self) -> ! {
        use std::process;
        eprintln!("{}", self);
        process::exit(self.get_exit_code());
    }
}

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

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Stage::*;
        let s = match *self {
            CompileDG => "compiling data generator",
            CompileAC => "compiling accepted program",
            CompileTP => "compiling tested program",
            LaunchDG => "launching data generator",
            LaunchAC => "launching accepted program",
            LaunchTP => "launching tested program",
        };
        write!(f, "{}", s)
    }
}
