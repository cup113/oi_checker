//! A module defined possible errors while running OI Checker.
//!
//! Also provide display & exit code

use std::borrow::Cow;

use toml;

use crate::prelude::{io, Display, PathBuf};

/// All error variants in OI Checker
#[derive(Debug)]
pub enum CheckerError {
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
        entries: Vec<(String, String)>,
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
        filter: crate::launch::filter::OutputFilter,
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
pub type CheckerResult<T> = Result<T, BoxedCheckerError>;

impl CheckerError {
    /// Print the error message to `stderr` and exit with the provided code
    pub fn destruct(&self) -> ! {
        use std::process;
        crate::LOGGER.fatal(&self.to_string());
        crate::LOGGER.info(&format!("Help: {}", self.get_help()));
        process::exit(1);
    }

    pub fn get_help<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        use Cow::{Borrowed as B, Owned as O};
        match self {
            Self::CfgFileReadingError { .. } => B("Check file permission."),
            Self::CfgFileParsingError { .. } => B(
                "Check if the file is TOML grammatical and has all the fields \
                and correspond types.",
            ),
            Self::CfgIntegrateError { .. } => B("Check if the value is legal (in options)."),
            Self::CreateWorkDirError { .. } => {
                B("Check directory permission. Avoid using nested path.")
            }
            Self::ArgFormattingTokenError { .. } => B("Correct the grammar of formatting"),
            Self::ArgFormattingKeyError {
                entries: dict_keys, ..
            } => O(format!(
                "Possible key-value pairs are:\n{}",
                dict_keys
                    .iter()
                    .map(|(key, value)| format!("- {}: {}", key, value))
                    .collect::<Vec<_>>()
                    .join("\n")
            )),
            Self::CommandError { .. } => B("Check your program or config."),
            Self::FilterError { .. } => B("This error shouldn't occur, it's a TOC-TOU error."),
            Self::DiffToolError { .. } => B("Please check if the different tool program exists."),
        }
    }
}

impl Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CheckerError::*;
        match self {
            CfgFileReadingError { err, file } => {
                write!(f, "Read config file ({}) failed:\n{}", file.display(), err)
            }
            CfgFileParsingError { err, file } => {
                write!(f, "Parse config file ({}) failed:\n{}", file.display(), err)
            }
            CfgIntegrateError { msg, file_source } => write!(
                f,
                "Error when integrating config (file source: {}): {}",
                file_source.display(),
                msg
            ),
            CreateWorkDirError { err, dir } => write!(
                f,
                "Error when creating working directory ({}): {}",
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
                parsing pattern \"{}\" at pos {}.",
                stage, desc, pattern, pos
            ),
            ArgFormattingKeyError {
                stage,
                pattern,
                key,
                pos,
                ..
            } => write!(
                f,
                "Error when parsing arguments during {}: Key Not Found \
                (key: \"{}\") when parsing pattern \"{}\" at pos {}.",
                stage, key, pattern, pos,
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
                Command: \"{}\" {}",
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
                "Error during filtering file {} (filter: {}): {}",
                file.display(),
                filter,
                err
            ),
            DiffToolError { command, args, err } => write!(
                f,
                "Error during comparing files: {}\n\
                Command: \"{}\" {}\n",
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

impl std::error::Error for CheckerError {}

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
