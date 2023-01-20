//! Compile program source files.

use crate::checker_error::{CheckerError, Stage};
use crate::config::cf_parsing;
use crate::dyn_formatting;
use crate::path_lib::TryToString;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
    pub fn run(
        &self,
        work_folder: &PathBuf,
        file: &PathBuf,
        stage: Stage,
    ) -> Result<(), CheckerError> {
        let filename_no_extension = {
            if let Some(stem) = file.file_stem() {
                stem
            } else {
                file.as_os_str()
            }
        };
        // to give the &str longer lifetime
        let s_filename_no_extension = filename_no_extension.try_to_string()?;
        let s_work_folder = work_folder.try_to_string()?;
        let s_filename = file.file_name().unwrap_or_default().try_to_string()?;
        let s_file = file.try_to_string()?;
        let target_dict: HashMap<&str, &str> = HashMap::from([
            ("work_folder", s_work_folder.as_str()),
            ("filename_no_extension", s_filename_no_extension.as_str()),
            ("filename", s_filename.as_str()),
        ]);
        let target = dyn_formatting::dynamic_format(&self.target, &target_dict, stage)?;
        let args_dict: HashMap<&str, &str> = HashMap::from([
            ("optimize_flag", self.optimize_flag.as_str()),
            ("file", s_file.as_str()),
            ("target", target.as_str()),
        ]);
        let mut args: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in self.args.iter() {
            args.push(dyn_formatting::dynamic_format(
                arg,
                &args_dict,
                Stage::CompileDG,
            )?);
        }
        let output = Command::new(&self.command)
            .stderr(Stdio::inherit())
            .args(args.clone())
            .output()
            .map_err(|e| CheckerError::CompileError {
                command: self.command.to_owned(),
                args: args.to_owned(),
                file: file.to_owned(),
                msg: e.to_string(),
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(CheckerError::CompileError {
                command: self.command.to_owned(),
                args: args.to_owned(),
                file: file.to_owned(),
                msg: format!("Compile failed: Compiler exited with {}", output.status),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
