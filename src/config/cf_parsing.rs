use crate::checker_error::CheckerError;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};
use toml;

pub fn parse_config_file() -> Result<Config, CheckerError> {
    let program_dir = env::current_exe()
        .expect("Can't get env::current_exe")
        .parent()
        .unwrap()
        .to_path_buf();
    let current_dir = env::current_dir().expect("Can't get env::current_dir");
    let alter_files = [
        current_dir.join("oi_checker_config.toml"),
        program_dir.join("config.toml"),
        program_dir.join("config_default.toml"),
    ];
    let config_file = {
        let mut c: Option<PathBuf> = None;
        for alter_file in alter_files.iter() {
            if alter_file.is_file() {
                c = Some(alter_file.to_owned());
                break;
            }
        }
        match c {
            Some(c) => c,
            None => {
                return Err(CheckerError::CfgFileNotFoundError {
                    tried_files: alter_files,
                })
            }
        }
    };
    let config = fs::read_to_string(config_file.as_path()).map_err(|err| {
        CheckerError::CfgFileReadingError {
            err,
            file: config_file.clone(),
        }
    })?;
    let config: Config =
        toml::from_str(&config).map_err(|err| CheckerError::CfgFileParsingError {
            err,
            file: config_file,
        })?;
    Ok(config)
}

#[derive(Deserialize)]
pub struct Config {
    default: DefaultConfig,
    compilation: Vec<CompilationConfig>,
}

#[derive(Deserialize)]
pub struct DefaultConfig {
    tested_program: PathBuf,
    accepted_program: PathBuf,
    data_generator: PathBuf,
    test_cases: u32,
    test_threads: u32,
    ac_timeout: u32,
    program_timeout: u32,
    working_directory: PathBuf,
    auto_remove_files: String,
    output_filters: Vec<String>,
    diff_tool: Vec<String>,
}

#[derive(Deserialize)]
pub struct CompilationConfig {
    ext: Vec<String>,
    target: String,
    optimize_flag: String,
    command: String,
    args: Vec<String>,
}

#[derive(Deserialize)]
pub struct LaunchConfig {
    ext: Vec<String>,
    command: String,
    args: Vec<String>,
}
