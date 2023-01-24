//! Compare the output files.

use crate::prelude::*;

/// Compare output files tool.
#[derive(Debug, Clone)]
pub enum DiffTool {
    /// Use Windows `FC`
    FC(Option<u32>),
    /// Use bash `diff`
    Diff,
    /// Customized diff tool.
    Custom(Vec<String>),
}

/// Enum shows that the result of diff tools.
pub enum DiffToolOk {
    Same,
    Different {
        log_path: PathBuf,
        log_result: io::Result<()>,
    },
}

impl DiffTool {
    /// Return the command program and arguments.
    fn get_command(&self, files: (&PathBuf, &PathBuf)) -> (String, Vec<String>) {
        let (program, mut args) = match self {
            Self::FC(Some(n)) => ("fc".into(), [format!("/LB{}", n)].into()),
            Self::FC(_) => ("fc".into(), Vec::new()),
            Self::Diff => ("diff".into(), Vec::new()),
            Self::Custom(command) => (command[0].to_owned(), command[1..].into()),
        };
        args.push(files.0.to_string_lossy().to_string());
        args.push(files.1.to_string_lossy().to_string());
        (program, args)
    }

    fn get_result(
        output: io::Result<std::process::Output>,
        dump_diff_file: &PathBuf,
        args: Vec<String>,
        program: String,
    ) -> CheckerResult<DiffToolOk> {
        match output {
            Ok(output) if output.status.success() => Ok(DiffToolOk::Same),
            Ok(output) => {
                let log_path = dump_diff_file;
                let log_result = fs::write(&log_path, output.stdout);
                Ok(DiffToolOk::Different {
                    log_path: log_path.to_owned(),
                    log_result,
                })
            }
            Err(err) => Err(Box::new(CheckerError::DiffToolError {
                command: program,
                args,
                err,
            })),
        }
    }

    /// Run the diff tool.
    pub fn run(
        &self,
        files: (&PathBuf, &PathBuf),
        dump_diff_file: &PathBuf,
    ) -> CheckerResult<DiffToolOk> {
        let (program, args) = self.get_command(files);
        let output = Command::new(&program)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output();
        Self::get_result(output, dump_diff_file, args, program)
    }
}

impl TryFrom<Vec<String>> for DiffTool {
    type Error = String;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("`diff_tool` config list cannot be empty".into());
        }
        match value[0].to_ascii_lowercase().as_str() {
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

#[cfg(test)]
mod tests {
    use super::*;
    fn as_vec_str<'a>(v: &'a Vec<String>) -> Vec<&'a str> {
        v.iter().map(|s| s.as_str()).collect()
    }

    #[test]
    fn test_get_command() {
        let (p1, a1) = DiffTool::FC(None).get_command((&"1.txt".into(), &"2.txt".into()));
        assert_eq!(p1.as_str(), "fc");
        assert_eq!(as_vec_str(&a1), vec!["1.txt", "2.txt"]);
        let (p2, a2) = DiffTool::FC(Some(3)).get_command((&"a".into(), &"b".into()));
        assert_eq!(p2.as_str(), "fc");
        assert_eq!(as_vec_str(&a2), vec!["/LB3", "a", "b"]);
        let (p2, a2) = DiffTool::Diff.get_command((&"a".into(), &"b".into()));
        assert_eq!(p2.as_str(), "diff");
        assert_eq!(as_vec_str(&a2), vec!["a", "b"]);
        let (p4, a4) = DiffTool::Custom(vec!["my-diff".into(), "--special".into()])
            .get_command((&"a".into(), &"b".into()));
        assert_eq!(p4.as_str(), "my-diff");
        assert_eq!(as_vec_str(&a4), vec!["--special", "a", "b"]);
    }

    #[test]
    fn test_try_from() {
        assert!(DiffTool::try_from(Vec::from(["abc".into(), "3".into()]))
            .unwrap_err()
            .contains("abc"));
        assert!(DiffTool::try_from(Vec::from(["df".into()]))
            .unwrap_err()
            .contains("df"));
        assert!(DiffTool::try_from(Vec::from(["fc".into(), "-1".into()]))
            .unwrap_err()
            .contains("-1"));
        assert!(DiffTool::try_from(Vec::from(["fc".into(), "abc".into()]))
            .unwrap_err()
            .contains("abc"));
        assert_eq!(
            format!(
                "{:?}",
                DiffTool::try_from(Vec::from(["fc".into(), "1".into()])).unwrap()
            ),
            format!("{:?}", DiffTool::FC(Some(1)))
        );
        assert_eq!(
            format!(
                "{:?}",
                DiffTool::try_from(Vec::from(["fc".into()])).unwrap()
            ),
            format!("{:?}", DiffTool::FC(None))
        );
        assert_eq!(
            format!(
                "{:?}",
                DiffTool::try_from(Vec::from(["diff".into()])).unwrap()
            ),
            format!("{:?}", DiffTool::Diff)
        );
        assert_eq!(
            format!(
                "{:?}",
                DiffTool::try_from(Vec::from(["custom".into(), "my-diff".into()])).unwrap()
            ),
            format!("{:?}", DiffTool::Custom(["my-diff".into()].into()))
        );
        assert_eq!(
            format!(
                "{:?}",
                DiffTool::try_from(Vec::from(["custom".into(), "my-diff".into(), "arg".into()]))
                    .unwrap()
            ),
            format!(
                "{:?}",
                DiffTool::Custom(["my-diff".into(), "arg".into()].into())
            )
        );
    }
}
