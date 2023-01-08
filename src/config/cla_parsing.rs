use clap::Command;
use std::{path::PathBuf, time::Duration};

pub enum AutoRemoveFilesMode {
    AcceptedOnly,
    Always,
    Never,
}

pub struct CommandLineArgs {
    test_threads: Option<u32>,
    ac_timeout: Option<Duration>,
    program_timeout: Option<Duration>,
    test_cases: Option<u32>,
    working_directory: Option<PathBuf>,
    tested_program: Option<PathBuf>,
    accepted_program: Option<PathBuf>,
    data_generator: Option<PathBuf>,
    auto_remove_files: Option<AutoRemoveFilesMode>,
    output_filters: Vec<String>,
    diff_tool: Vec<String>,
}
