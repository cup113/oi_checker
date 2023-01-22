//! #TODO doc

mod checker_error;
mod compilation;
mod config;
mod dyn_formatting;
mod launch;
mod logging;
mod path_lib;

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use crate::checker_error::{CheckerError, Stage};
use crate::launch::LaunchOk;
use crate::logging::Logger;
use crate::path_lib::TryToString;

fn main() {
    let mut oi_checker = OIChecker::new().unwrap_or_else(|err| err.destruct());
    oi_checker.run().unwrap_or_else(|err| err.destruct());
}

struct OIChecker {
    logger: Logger,
    config: config::Config,
}

impl OIChecker {
    /// Get a new OIChecker. It should be generated once only.
    fn new() -> Result<Self, CheckerError> {
        use logging::Level::Info;
        let logger = Logger::new(String::from("OIChecker"), Info);
        let config = config::get_config()?;
        Ok(Self { logger, config })
    }

    /// TODO doc
    fn try_compile(
        &self,
        program: &PathBuf,
        stage: Stage,
    ) -> Result<Option<PathBuf>, CheckerError> {
        let ext = if let Some(ext) = program.extension() {
            ext
        } else {
            self.logger.info(&format!(
                "No extension name for \"{}\", skip it.",
                program.try_to_string()?
            ));
            return Ok(None);
        };
        let rule = self.config.compilation_rules.get_rule(ext.try_to_string()?);
        if let Some(rule) = rule {
            let target = rule.run(&self.config.working_directory, &program, stage)?;
            self.logger.info(&format!(
                "Compile {} successfully.",
                program.try_to_string()?
            ));
            Ok(Some(target.into()))
        } else {
            self.logger.info(&format!(
                "No matched compilation config for \"{}\" (extension = {}), skip it",
                program.try_to_string()?,
                ext.try_to_string()?
            ));
            Ok(None)
        }
    }

    /// TODO doc
    fn launch_one(
        &self,
        program: &PathBuf,
        extra_args: Vec<String>,
        input_file: &Option<PathBuf>,
        output_file: &PathBuf,
        stage: Stage,
    ) -> Result<LaunchOk, CheckerError> {
        let default_launch_rule = launch::LaunchConfig::default();
        let launch_rule = if let Some(ext) = program.extension() {
            self.config
                .launch_rules
                .get_rule(ext.try_to_string()?)
                .unwrap_or(&default_launch_rule)
        } else {
            &default_launch_rule
        };
        launch_rule.run(
            program,
            stage,
            extra_args,
            self.config.program_timeout,
            input_file,
            output_file,
        )
    }

    /// TODO doc
    fn launch_suite(&self, index: u32) -> LaunchSuiteResult {
        let data_file = PathBuf::from(format!("data{}.in", index));
        let ac_out_file = PathBuf::from(format!("ac{}.out", index));
        let tp_out_file = PathBuf::from(format!("tested{}.out", index));

        let dg_result = self.launch_one(
            &self.config.data_generator,
            Vec::from([index.to_string(), self.config.test_cases.to_string()]),
            &None,
            &data_file,
            Stage::LaunchDG,
        );
        let dg_handle = dg_result
            .map(|o| match o {
                LaunchOk::Success(_) => None,
                LaunchOk::Timeout(_) => Some("Timeout".into()),
            })
            .unwrap_or_else(|err| Some(format!("Inner Error: {}", err)));
        if let Some(hint) = dg_handle {
            return LaunchSuiteResult {
                index,
                inner: LaunchSuiteEnum::UK(format!("Launch data generator failed: {}", hint)),
            };
        }

        let ac_result = self.launch_one(
            &self.config.accepted_program,
            Vec::from([index.to_string(), self.config.test_cases.to_string()]),
            &Some(data_file.clone()),
            &ac_out_file,
            Stage::LaunchAC,
        );
        let ac_handle = ac_result
            .map(|o| match o {
                LaunchOk::Success(_) => None,
                LaunchOk::Timeout(_) => Some("Timeout".into()),
            })
            .unwrap_or_else(|err| Some(format!("Inner Error: {}", err)));
        if let Some(hint) = ac_handle {
            return LaunchSuiteResult {
                index,
                inner: LaunchSuiteEnum::UK(format!("Launch data generator failed: {}", hint)),
            };
        }

        let tp_result = self.launch_one(
            &self.config.tested_program,
            Vec::new(),
            &Some(data_file.clone()),
            &tp_out_file,
            Stage::LaunchTP,
        );
        let tp_handle = tp_result
            .map(|o| match o {
                LaunchOk::Success(_) => None,
                LaunchOk::Timeout(_) => Some("Timeout".into()),
            })
            .unwrap_or_else(|err| Some(format!("Inner Error: {}", err)));
        if let Some(hint) = tp_handle {
            return LaunchSuiteResult {
                index,
                inner: LaunchSuiteEnum::UK(format!("Launch tested program failed: {}", hint)),
            };
        }
        todo!()
        // TODO compare
    }

    /// Main function, run the checker
    fn run(&mut self) -> Result<(), CheckerError> {
        use std::fs;
        self.logger.info("Parse configuration successfully.");
        if !self.config.working_directory.exists() {
            fs::create_dir(&self.config.working_directory).map_err(|err| {
                CheckerError::CreateWorkDirError {
                    err,
                    dir: self.config.working_directory.to_owned(),
                }
            })?;
        }
        macro_rules! try_compile {
            ($program: ident, $stage: expr) => {
                if let Some(target) = self.try_compile(&self.config.$program, $stage)? {
                    self.config.$program = target;
                }
            };
        }
        try_compile!(data_generator, Stage::CompileDG);
        try_compile!(accepted_program, Stage::CompileAC);
        try_compile!(tested_program, Stage::CompileTP);
        todo!(); // TODO
    }
}

struct LaunchSuiteResult {
    index: u32,
    inner: LaunchSuiteEnum,
}

enum LaunchSuiteEnum {
    AC(Duration),
    WA(Duration, PathBuf),
    TLE(Duration),
    UK(String),
}
