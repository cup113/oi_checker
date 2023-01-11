mod cf_parsing;
mod cla_parsing;

use crate::CheckerError;
use std::{path::PathBuf, time::Duration, collections::HashMap};

pub enum AutoRemoveFiles {
    AC,
    Always,
    Never,
}

pub enum OutputFilter {
    StripTrailingWhitespace,
    StripTrailingEmptyLines,
    StripAllWhitespace,
}

pub enum DiffTool {
    FC(Option<u32>),
    Diff,
    Custom(Vec<String>)
}

pub struct CompilationConfig {
    target: String,
    optimize_flag: String,
    command: String,
    args: Vec<String>,
}

pub struct LaunchConfig {
    command: String,
    args: Vec<String>,
}

pub struct Config {
    tested_program: PathBuf,
    accepted_program: PathBuf,
    data_generator: PathBuf,
    test_cases: u32,
    test_threads: u32,
    ac_timeout: Duration,
    program_timeout: Duration,
    working_directory: PathBuf,
    auto_remove_files: AutoRemoveFiles,
    output_filters: Vec<OutputFilter>,
    diff_tool: DiffTool,
    compilation_rules: HashMap<String, CompilationConfig>,
    launch_rules: HashMap<String, LaunchConfig>
}

// TODO doc
pub fn get_config() -> Result<Config, CheckerError> {
    let mut cla_config = cla_parsing::parse_cla();
    let mut cf_config = cf_parsing::parse_config_file()?;
    println!("{:?}", cla_config);
    println!("{:?}", cf_config);
    // TODO integrate to return Config
    todo!();
}
