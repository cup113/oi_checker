//! Config file parsing module.

use crate::prelude::*;
use serde::Deserialize;
use std::env;
use toml;

fn get_config_filename() -> PathBuf {
    let program_dir = env::current_exe()
        .expect("Can't get env::current_exe")
        .parent()
        .expect("Program can't BE root directory")
        .to_owned();
    let alter_files: [PathBuf; 3] = [
        "oi_checker_config.toml".into(),
        program_dir.join("config.toml"),
        program_dir.join("config_default.toml"),
    ];
    let config_file = {
        let mut config_file: Option<&PathBuf> = None;
        for alter_file in alter_files.iter() {
            if alter_file.is_file() {
                config_file = Some(alter_file);
                break;
            }
        }
        config_file.unwrap_or({
            fs::write(&alter_files[2], crate::config::CONFIG_FILE_DEFAULT).ignore();
            &alter_files[2]
        })
    };
    config_file.to_owned()
}

/// Parse the config file.
pub fn parse_config_file() -> CheckerResult<(Config, PathBuf)> {
    let config_file = get_config_filename();
    let config = fs::read_to_string(config_file.as_path()).map_err(|err| {
        CheckerError::CfgFileReadingError {
            err,
            file: config_file.to_owned(),
        }
    })?;
    let config: Config =
        toml::from_str(&config).map_err(|err| CheckerError::CfgFileParsingError {
            err,
            file: config_file.to_owned(),
        })?;
    Ok((config, config_file))
}

/// Main configuration of config file
#[derive(Deserialize, Debug)]
pub struct Config {
    pub default: DefaultConfig,
    pub compilation: Vec<CompilationConfig>,
    pub launch: Vec<LaunchConfig>,
}

/// `default` field in toml file
#[derive(Deserialize, Debug)]
pub struct DefaultConfig {
    pub tested_program: PathBuf,
    pub accepted_program: PathBuf,
    pub data_generator: PathBuf,
    pub test_cases: u32,
    pub test_threads: u32,
    pub ac_timeout: u64,
    pub program_timeout: u64,
    pub working_directory: PathBuf,
    pub auto_remove_files: String,
    pub output_filters: Vec<String>,
    pub diff_tool: Vec<String>,
}

/// `compile` field in toml file
#[derive(Deserialize, Debug)]
pub struct CompilationConfig {
    pub ext: Vec<String>,
    pub target: String,
    pub optimize_flag: String,
    pub command: String,
    pub args: Vec<String>,
}

/// `launch` field in toml file
#[derive(Deserialize, Debug)]
pub struct LaunchConfig {
    pub ext: Vec<String>,
    pub command: String,
    pub args: Vec<String>,
}
