//! Parse config (command-line & file)

pub mod cf_parsing;
mod cla_parsing;

use crate::prelude::*;

use crate::clean_files::AutoRemoveFiles;
use crate::compilation::CompilationConfig;
use crate::diff_tool::DiffTool;
use crate::filter::OutputFilter;
use crate::launch::LaunchConfig;
use dyn_formatting::{self, DynamicFormatError};

const CONFIG_FILE_DEFAULT: &'static str = include_str!("../config_default.toml");

// Get the main configuration.
pub fn get_config() -> CheckerResult<Config> {
    let cla_config = cla_parsing::parse_cla();
    let (cf_config, cf_file) = cf_parsing::parse_config_file()?;

    macro_rules! get_default {
        ($name: ident) => {
            cla_config.$name.unwrap_or(cf_config.default.$name)
        };
    }

    macro_rules! error {
        ($msg: expr) => {
            Box::new(CheckerError::CfgIntegrateError {
                msg: $msg,
                file_source: cf_file.to_owned(),
            })
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
        .map_err(|msg| error!(msg))?;
    let output_filters: Vec<OutputFilter> = {
        let mut output_filters = Vec::new();
        for filter in get_default!(output_filters) {
            output_filters.push(filter.as_str().try_into().map_err(|msg| error!(msg))?);
        }
        output_filters
    };
    let diff_tool: DiffTool = get_default!(diff_tool)
        .try_into()
        .map_err(|msg: String| error!(msg))?;
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
    if working_directory == PathBuf::from(".") {
        return Err(error!(
            "Working directory shouldn't be current directory because it may be deleted.".into()
        ));
    }
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
pub struct ExtensionRules<T> {
    store: Vec<T>,
    mapping: HashMap<String, usize>,
}

impl<T> ExtensionRules<T> {
    /// Get the rule of the given extension. Return `None` if not found.
    pub fn get_rule(&self, ext: &String) -> Option<&T> {
        self.mapping.get(ext).map(|i| self.store.get(*i).unwrap())
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

/// Simple, dynamic, Python-styled string formatting (Only support `String`,
/// `{key}` patterns ).
///
/// Escape like `{{` or `}}`.
///
/// # Errors
///
/// 1. Raise `CheckerError::ArgFormattingKeyError` while the key in the brackets
///    is not found.
/// 2. Raise `CheckerError::ArgFormattingTokenError` while there is any
///    unmatched bracket (`{` or `}`)
pub fn dynamic_format(
    pattern: &str,
    dictionary: &HashMap<&str, &str>,
    stage: Stage,
) -> CheckerResult<String> {
    use CheckerError::*;
    use DynamicFormatError::*;
    dyn_formatting::dynamic_format(pattern, dictionary).map_err(|e| {
        if let KeyError {
            pattern,
            key,
            dict_keys,
            pos,
        } = e
        {
            Box::new(ArgFormattingKeyError {
                stage,
                pattern,
                key,
                dict_keys,
                pos,
            })
        } else if let TokenError { pattern, desc, pos } = e {
            Box::new(ArgFormattingTokenError {
                stage,
                pattern,
                desc,
                pos,
            })
        } else {
            unreachable!();
        }
    })
}
