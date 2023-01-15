//! #TODO

use crate::checker_error::{CheckerError, Stage};
use crate::config::cf_parsing;
use crate::dyn_formatting;
use crate::path_lib::TryToString;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct CompilationConfig {
    pub target: String,
    pub optimize_flag: String,
    pub command: String,
    pub args: Vec<String>,
}

impl From<cf_parsing::CompilationConfig> for CompilationConfig {
    fn from(value: cf_parsing::CompilationConfig) -> Self {
        Self {
            target: value.target,
            optimize_flag: value.optimize_flag,
            command: value.command,
            args: value.args,
        }
    }
}

impl CompilationConfig {
    fn run(&self, work_folder: PathBuf, file: PathBuf) -> Result<(), CheckerError> {
        let filename_no_extension = {
            if let Some(prefix) = file.extension() { // FIXME not ext
                prefix
            } else {
                file.as_os_str()
            }
        };
        // to give the &str longer lifetime
        let s_filename_no_extension = filename_no_extension.try_to_string()?;
        let s_work_folder = work_folder.try_to_string()?;
        let s_filename = file.try_to_string()?;
        let dict: HashMap<&str, &str> = HashMap::from([
            ("work_folder", s_work_folder.as_str()),
            ("filename_no_extension", s_filename_no_extension.as_str()),
            ("filename", s_filename.as_str()),
        ]);
        let mut args: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in self.args.iter() {
            args.push(dyn_formatting::dynamic_format(arg, &dict, Stage::Compile)?);
        }
        Command::new(&self.command).args(args).output();
        Ok(())
    }
}
