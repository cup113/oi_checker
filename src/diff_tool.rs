//! Compare the output files.

use crate::prelude::*;

/// Compare output files tool.
#[derive(Debug, Clone)]
pub enum DiffTool {
    FC(Option<u32>),
    Diff,
    Custom(Vec<String>),
}

/// Enum shows that the result of diff tools.
pub enum DiffToolOk {
    Success,
    Different {
        log_path: PathBuf,
        log_success: bool,
    },
}

impl DiffTool {
    /// Run the diff tool.
    pub fn run(
        &self,
        files: (&PathBuf, &PathBuf),
        dump_diff_file: &PathBuf,
    ) -> CheckerResult<DiffToolOk> {
        let (program, mut args): (String, Vec<String>) = match self {
            Self::FC(Some(n)) => ("fc".into(), [format!("/LB{}", n)].into()),
            Self::FC(_) => ("fc".into(), Vec::new()),
            Self::Diff => ("diff".into(), Vec::new()),
            Self::Custom(command) => (command[0].to_owned(), command[1..].into()),
        };
        args.push(files.0.try_to_string()?);
        args.push(files.1.try_to_string()?);
        let output = Command::new(&program)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output();
        match output {
            Ok(output) if output.status.success() => Ok(DiffToolOk::Success),
            Ok(output) => {
                let log_path = dump_diff_file;
                let log_success = fs::write(&log_path, output.stdout).is_ok();
                Ok(DiffToolOk::Different {
                    log_path: log_path.to_owned(),
                    log_success,
                })
            }
            Err(err) => Err(Box::new(CheckerError::DiffToolError {
                command: program,
                args: args,
                err,
            })),
        }
    }
}

impl TryFrom<Vec<String>> for DiffTool {
    type Error = String;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("`diff_tool` config list cannot be empty".into());
        }
        match value[0].as_str() {
            "fc" => match value.get(1) {
                None => Ok(DiffTool::FC(None)),
                Some(s) => Ok(DiffTool::FC(Some(s.parse::<u32>().map_err(|_| {
                    format!("Expected a positive number in field `fc[1]`, found {}", s)
                })?))),
            },
            "diff" => Ok(DiffTool::Diff),
            "custom" => Ok(DiffTool::Custom(value[1..].to_vec())),
            r => return Err(format!("Rule {} is not defined.", r)),
        }
    }
}