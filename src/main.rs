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

use crate::checker_error::{BoxedCheckerError, CheckerError, Stage};
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
        let logger = Logger::new("OIChecker".into(), Info);
        let config = config::get_config()?;
        Ok(Self { logger, config })
    }

    /// TODO doc
    fn try_compile(
        &self,
        program: &PathBuf,
        stage: Stage,
    ) -> Result<Option<PathBuf>, BoxedCheckerError> {
        let ext = if let Some(ext) = program.extension() {
            ext
        } else {
            self.logger.info(&format!(
                "No extension name for \"{}\", skip it.",
                program.try_to_string()?
            ));
            return Ok(None);
        };
        let rule = self
            .config
            .compilation_rules
            .get_rule(&ext.try_to_string()?);
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

    /// Main function, run the checker
    fn run(&mut self) -> Result<(), BoxedCheckerError> {
        use crate::launch::{LaunchSuiteEnum, SuiteLauncher};
        use std::fs;
        use threadpool::ThreadPool;
        use console;

        self.logger.info("Parse configuration successfully.");
        if !self.config.working_directory.exists() {
            fs::create_dir(&self.config.working_directory).map_err(|err| {
                CheckerError::CreateWorkDirError {
                    err,
                    dir: self.config.working_directory.to_owned(),
                }
            })?;
        }
        macro_rules! compile {
            ($program: ident, $stage: expr) => {
                if let Some(target) = self.try_compile(&self.config.$program, $stage)? {
                    self.config.$program = target;
                }
            };
        }
        compile!(data_generator, Stage::CompileDG);
        compile!(accepted_program, Stage::CompileAC);
        compile!(tested_program, Stage::CompileTP);
        let pool = ThreadPool::new(self.config.test_threads as usize);
        let suite_launcher: SuiteLauncher = (&*self).into();
        let (tx, rx) = mpsc::channel();
        for index in 1..=self.config.test_cases {
            let suite_launcher = suite_launcher.clone();
            let tx = tx.clone();
            pool.execute(move || {
                suite_launcher.run_suite(index, tx);
            });
        }
        let mut launch_result_count = (0u32, 0u32, 0u32);
        for _ in 1..=self.config.test_cases {
            let launch_result = rx.recv().expect("Receiver should receive");
            let log_content = match launch_result.inner {
                LaunchSuiteEnum::AC(duration) => {
                    launch_result_count.0 += 1;
                    format!("AC ({0:.3} ms)", duration.as_secs_f64() * 1000.0)
                }
                LaunchSuiteEnum::WA(duration, file) => {
                    launch_result_count.1 += 1;
                    format!(
                        "WA ({0:.3} ms) See difference in file {1}",
                        duration.as_secs_f64() * 1000.0,
                        file.display()
                    )
                }
                LaunchSuiteEnum::TLE(duration) => {
                    launch_result_count.1 += 1;
                    format!("TLE ({0:.3} ms", duration.as_secs_f64() * 1000.0)
                }
                LaunchSuiteEnum::UK(hint) => {
                    launch_result_count.2 += 1;
                    format!("UK: {}", hint)
                }
            };
            self.logger.info(&format!(
                "Test #{0:02}: {1}",
                launch_result.index, log_content
            ));
        }
        self.logger.info("Test finished.");
        self.logger.info(&format!(
            "Report: AC {} UK {} WA {} / Total {}",
            console::style(launch_result_count.0).green(),
            console::style(launch_result_count.2).yellow(),
            console::style(launch_result_count.1).red(),
            self.config.test_cases
        ));
        // TODO auto remove files
        Ok(())
    }
}
