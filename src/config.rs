//! Parse config (command-line & file)

pub mod cf_parsing;
mod cla_parsing;

use crate::compilation::CompilationConfig;
use crate::CheckerError;
use std::{collections::HashMap, path::PathBuf, time::Duration};

// Get the main configuration
pub fn get_config() -> Result<Config, CheckerError> {
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
    let auto_remove_files = AutoRemoveFiles::try_from(get_default!(auto_remove_files).as_str())
        .map_err(|msg| CheckerError::CfgIntegrateError {
            msg,
            file_source: cf_file.to_owned(),
        })?;
    let output_filters: Vec<OutputFilter> = {
        let mut output_filters = Vec::new();
        for filter in get_default!(output_filters) {
            output_filters.push(OutputFilter::try_from(filter.as_str()).map_err(|msg| {
                CheckerError::CfgIntegrateError {
                    msg,
                    file_source: cf_file.to_owned(),
                }
            })?);
        }
        output_filters
    };
    let diff_tool = DiffTool::try_from(get_default!(diff_tool)).map_err(|msg: String| {
        CheckerError::CfgIntegrateError {
            msg,
            file_source: cf_file.to_owned(),
        }
    })?;
    let compilation_rules = ExtensionRules::from(
        cf_config
            .compilation
            .into_iter()
            .map(|c| (c.ext.clone(), CompilationConfig::from(c)))
            .collect::<Vec<_>>(),
    );
    let launch_rules = ExtensionRules::from(
        cf_config
            .launch
            .into_iter()
            .map(|c| (c.ext.clone(), LaunchConfig::from(c)))
            .collect::<Vec<_>>(),
    );
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
#[derive(Debug)]
pub struct ExtensionRules<T> {
    store: Vec<T>,
    mapping: HashMap<String, usize>,
}

impl<T> ExtensionRules<T> {
    /// Get the rule of the given extension. Return `None` if not found.
    pub fn get_rule(&self, key: String) -> Option<&T> {
        self.mapping.get(&key).map(|i| self.store.get(*i).unwrap())
    }
}

impl<T> From<Vec<(Vec<String>, T)>> for ExtensionRules<T> {
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

#[derive(Debug)]
pub enum OutputFilter {
    StripTrailingWhitespace,
    StripTrailingEmptyLines,
    StripAllWhitespace,
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

#[derive(Debug)]
pub enum DiffTool {
    FC(Option<u32>),
    Diff,
    Custom(Vec<String>),
}

impl TryFrom<Vec<String>> for DiffTool {
    type Error = String;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(String::from("`diff_tool` config list cannot be empty"));
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

#[derive(Debug)]
pub struct LaunchConfig {
    pub command: String,
    pub args: Vec<String>,
}

impl From<cf_parsing::LaunchConfig> for LaunchConfig {
    fn from(value: cf_parsing::LaunchConfig) -> Self {
        Self {
            command: value.command,
            args: value.args,
        }
    }
}
