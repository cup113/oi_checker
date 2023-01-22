//! Parse config (command-line & file)

pub mod cf_parsing;
mod cla_parsing;

use crate::checker_error::BoxedCheckerError;
use crate::compilation::CompilationConfig;
use crate::launch::LaunchConfig;
use crate::CheckerError;
use crate::TryToString;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Get the main configuration
pub fn get_config() -> Result<Config, BoxedCheckerError> {
    let cla_config = cla_parsing::parse_cla();
    let (cf_config, cf_file) = cf_parsing::parse_config_file()?;
    macro_rules! get_default {
        ($name: ident) => {
            cla_config.$name.unwrap_or(cf_config.default.$name)
        };
    }
    let tested_program = get_default!(tested_program);
    let accepted_program = get_default!(accepted_program);
    let data_generator = get_default!(data_generator);
    let test_cases = get_default!(test_cases);
    let test_threads = get_default!(test_threads);
    let ac_timeout = Duration::from_millis(get_default!(ac_timeout).into());
    let program_timeout = Duration::from_millis(get_default!(program_timeout).into());
    let working_directory = get_default!(working_directory);
    let auto_remove_files: AutoRemoveFiles = get_default!(auto_remove_files)
        .as_str()
        .try_into()
        .map_err(|msg| CheckerError::CfgIntegrateError {
            msg,
            file_source: cf_file.to_owned(),
        })?;
    let output_filters: Vec<OutputFilter> = {
        let mut output_filters = Vec::new();
        for filter in get_default!(output_filters) {
            output_filters.push(filter.as_str().try_into().map_err(|msg| {
                CheckerError::CfgIntegrateError {
                    msg,
                    file_source: cf_file.to_owned(),
                }
            })?);
        }
        output_filters
    };
    let diff_tool: DiffTool = get_default!(diff_tool).try_into().map_err(|msg: String| {
        CheckerError::CfgIntegrateError {
            msg,
            file_source: cf_file.to_owned(),
        }
    })?;
    let compilation_rules: ExtensionRules<CompilationConfig> = cf_config
        .compilation
        .into_iter()
        .map(|c| (c.ext.clone(), c.into()))
        .collect::<Vec<_>>()
        .into();
    let launch_rules: ExtensionRules<LaunchConfig> = cf_config
        .launch
        .into_iter()
        .map(|c| (c.ext.clone(), c.into()))
        .collect::<Vec<_>>()
        .into();
    Ok(Config {
        tested_program,
        accepted_program,
        data_generator,
        test_cases,
        test_threads,
        ac_timeout,
        program_timeout,
        working_directory,
        auto_remove_files,
        output_filters,
        diff_tool,
        compilation_rules,
        launch_rules,
    })
}

/// main configuration
#[derive(Debug)]
pub struct Config {
    pub tested_program: PathBuf,
    pub accepted_program: PathBuf,
    pub data_generator: PathBuf,
    pub test_cases: u32,
    pub test_threads: u32,
    pub ac_timeout: Duration,
    pub program_timeout: Duration,
    pub working_directory: PathBuf,
    pub auto_remove_files: AutoRemoveFiles,
    pub output_filters: Vec<OutputFilter>,
    pub diff_tool: DiffTool,
    pub compilation_rules: ExtensionRules<CompilationConfig>,
    pub launch_rules: ExtensionRules<LaunchConfig>,
}

/// Manage rules that is matched by extension names like `launch` and `compilation`
#[derive(Debug, Clone)]
pub struct ExtensionRules<T: Clone> {
    store: Vec<T>,
    mapping: HashMap<String, usize>,
}

impl<T: Clone> ExtensionRules<T> {
    /// Get the rule of the given extension. Return `None` if not found.
    pub fn get_rule(&self, ext: &String) -> Option<&T> {
        self.mapping.get(ext).map(|i| self.store.get(*i).unwrap())
    }
}

impl<T: Clone> From<Vec<(Vec<String>, T)>> for ExtensionRules<T> {
    fn from(value: Vec<(Vec<String>, T)>) -> Self {
        let mut store = Vec::with_capacity(value.len());
        let mut mapping = HashMap::new();
        for (i, v) in value.into_iter().enumerate() {
            store.push(v.1);
            for s in v.0 {
                mapping.insert(s, i);
            }
        }
        Self { store, mapping }
    }
}

#[derive(Debug)]
pub enum AutoRemoveFiles {
    AC,
    Always,
    Never,
}

impl TryFrom<&str> for AutoRemoveFiles {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ac" => Ok(Self::AC),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            s => Err(format!(
                "`{}` is not allowed in field `auto_remove_files`",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFilter {
    StripTrailingWhitespace,
    StripTrailingEmptyLines,
    StripAllWhitespace,
}

impl OutputFilter {
    fn run_strip_trailing_whitespace(content: &String) -> Vec<String> {
        let mut ans = Vec::new();
        for line in content.lines() {
            ans.push(line.trim_end().into());
        }
        ans
    }

    fn run_strip_trailing_empty_lines(content: &String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut buffer_empty_lines = 0u32;
        for line in content.lines() {
            if line.is_empty() {
                buffer_empty_lines += 1;
            } else {
                ans.push(line.into());
                for _ in 0..buffer_empty_lines {
                    ans.push("".into());
                }
                buffer_empty_lines = 0;
            }
        }
        ans
    }

    fn run_strip_all_whitespace<'a>(content: &'a String) -> Vec<String> {
        let mut ans = Vec::new();
        for line in content.lines() {
            ans.push(line.chars().filter(|c| c.is_whitespace()).collect());
        }
        ans
    }

    /// TODO doc
    pub fn run(&self, file: &PathBuf) -> Result<(), BoxedCheckerError> {
        use std::fs::{self, File};
        use std::io::Write;
        macro_rules! deal_io_err {
            ($result: expr) => {
                $result.map_err(|err| CheckerError::FilterError {
                    filter: *self,
                    err,
                    file: file.to_owned(),
                })?
            };
        }
        let original_content = deal_io_err!(fs::read_to_string(file));
        let filtered_lines = match self {
            Self::StripTrailingWhitespace => Self::run_strip_trailing_whitespace(&original_content),
            Self::StripTrailingEmptyLines => {
                Self::run_strip_trailing_empty_lines(&original_content)
            }
            Self::StripAllWhitespace => Self::run_strip_all_whitespace(&original_content),
        };
        let mut output_file = deal_io_err!(File::create(file));
        for filtered_line in filtered_lines {
            deal_io_err!(output_file.write(filtered_line.as_bytes()));
            deal_io_err!(output_file.write(crate::os_lib::LINE_END.as_bytes()));
        }
        Ok(())
    }
}

impl std::fmt::Display for OutputFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::StripTrailingWhitespace => "strip trailing whitespace",
            Self::StripTrailingEmptyLines => "strip trailing empty lines",
            Self::StripAllWhitespace => "strip all whitespace",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for OutputFilter {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "strip-trailing-whitespace" => Ok(Self::StripTrailingWhitespace),
            "strip-trailing-empty-lines" => Ok(Self::StripTrailingEmptyLines),
            "strip-all-whitespace" => Ok(Self::StripAllWhitespace),
            f => Err(format!(
                "filter {} is not defined in field `default.filters`",
                f
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DiffTool {
    FC(Option<u32>),
    Diff,
    Custom(Vec<String>),
}

pub enum DiffToolOk {
    Success,
    Different { log_path: PathBuf, log_success: bool },
}

impl DiffTool {
    /// TODO doc
    pub fn run(
        &self,
        files: (&PathBuf, &PathBuf),
        dump_diff_file: &PathBuf,
    ) -> Result<DiffToolOk, BoxedCheckerError> {
        use std::fs;
        use std::process::{Command, Stdio};
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
