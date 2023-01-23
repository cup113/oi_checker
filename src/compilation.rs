//! Compile program source files.

use crate::config::{cf_parsing, dynamic_format};
use crate::prelude::*;

#[derive(Debug, Clone)]
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
    /// Get arguments of the compilation.
    /// Return a tuple which means `(target_path, arguments)`
    ///
    /// # Error
    ///
    /// 1. `CheckerError::OsStrUtf8Error` when meet non-UTF-8 characters
    /// 2. `CheckerError::CompileError` when compilation failed due to
    ///    command not found or compiler exited with non-zero code.
    fn get_args(
        &self,
        work_folder: &PathBuf,
        file: &PathBuf,
        stage: Stage,
    ) -> CheckerResult<(String, Vec<String>)> {
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
        let target_dict: HashMap<&str, &str> = [
            ("work_folder", s_work_folder.as_str()),
            ("filename_no_extension", s_filename_no_extension.as_str()),
            ("filename", s_filename.as_str()),
        ]
        .into();
        let target = dynamic_format(&self.target, &target_dict, stage)?;
        let args_dict: HashMap<&str, &str> = [
            ("optimize_flag", self.optimize_flag.as_str()),
            ("file", s_file.as_str()),
            ("target", target.as_str()),
        ]
        .into();
        let mut args: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in self.args.iter() {
            args.push(dynamic_format(arg, &args_dict, stage)?);
        }
        Ok((target, args))
    }

    /// Compile the program with the config.
    pub fn run(
        &self,
        work_folder: &PathBuf,
        file: &PathBuf,
        stage: Stage,
    ) -> CheckerResult<String> {
        let (target, args) = self.get_args(work_folder, file, stage)?;
        let error = |msg: String| {
            Box::new(CheckerError::CommandError {
                stage,
                command: self.command.to_owned(),
                args: args.to_owned(),
                file: file.to_owned(),
                msg,
            })
        };
        let output = Command::new(&self.command)
            .stderr(Stdio::inherit())
            .args(args.clone())
            .output()
            .map_err(|e| error(format!("IOError: {}", e.to_string())))?;
        if output.status.success() {
            Ok(target)
        } else {
            Err(error(format!("Compiler exited with {}", output.status)))
        }
    }
}
