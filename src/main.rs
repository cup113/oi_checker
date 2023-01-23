//! An OI automatic checker, able to detect AC, WA and TLE

mod checker_error;
mod compilation;
mod config;
mod diff_tool;
mod filter;
mod launch;
mod logging;
mod prelude;
mod util;
mod clean_files;

use launch::LaunchSuiteResult;
use threadpool::ThreadPool;

use crate::config::Config;
use crate::launch::{LaunchSuiteEnum, SuiteLauncher};
use crate::logging::Logger;
use crate::prelude::*;

fn main() {
    let mut oi_checker = OIChecker::new().unwrap_or_else(|err| err.destruct());
    oi_checker.run().unwrap_or_else(|err| err.destruct());
}

struct OIChecker {
    logger: Logger,
    config: Config,
}

impl OIChecker {
    /// Get a new OIChecker. It should be generated once only.
    fn new() -> CheckerResult<Self> {
        use logging::Level::Info;
        let logger = Logger::new("OIChecker".into(), Info);
        let config = config::get_config()?;
        Ok(Self { logger, config })
    }

    /// Main function, run the checker
    fn run(&mut self) -> CheckerResult<()> {
        self.logger.info("Program begins running.");
        self.init_working_directory()?;
        self.compile_all()?;
        let (_pool, rx) = self.launch_suites()?;
        let (launch_result_count, ac_launch_indexes) = self.get_launch_result(rx);
        self.logger.info("Test finished.");
        self.report_total_score(launch_result_count);
        self.clean_generated_files(ac_launch_indexes);
        Ok(())
    }

    /// Create the working directory if it doesn't exist.
    ///
    /// Return `Err(Box<CheckerError::CreateWorkDirError>)` if IOError occurs.
    fn init_working_directory(&self) -> CheckerResult<()> {
        if !self.config.working_directory.exists() {
            fs::create_dir(&self.config.working_directory).map_err(|err| {
                CheckerError::CreateWorkDirError {
                    err,
                    dir: self.config.working_directory.to_owned(),
                }
            })?;
        }
        Ok(())
    }

    /// Compile all related files and replace mapped files.
    ///
    /// Return `Err(_)` when `self.compile_one` failed.
    fn compile_all(&mut self) -> CheckerResult<()> {
        macro_rules! compile_one {
            ($program: ident, $stage: expr) => {
                if let Some(target) = self.compile_one(&self.config.$program, $stage)? {
                    self.config.$program = target;
                }
            };
        }
        compile_one!(data_generator, Stage::CompileDG);
        compile_one!(accepted_program, Stage::CompileAC);
        compile_one!(tested_program, Stage::CompileTP);
        Ok(())
    }

    /// Launch all suites.
    ///
    /// Returned value explanation:
    /// - `Err(_)` => Failed to launch programs.
    /// - `Ok((pool, rx))` => Succeed in launching programs. Return the threadpool
    ///    to extend its lifetime, and the receiver for the next step.
    fn launch_suites(&self) -> CheckerResult<(ThreadPool, mpsc::Receiver<LaunchSuiteResult>)> {
        let pool = ThreadPool::new(self.config.test_threads as usize);
        let suite_launcher: SuiteLauncher = (&*self).into();
        let (tx, rx) = mpsc::channel();
        for i in 0..self.config.test_threads {
            // Warmup
            pool.execute(move || {
                std::thread::sleep(std::time::Duration::from_millis(500 + i as u64 * 10));
            });
        }
        self.logger.info("Warming up...");

        for index in 1..=self.config.test_cases {
            let suite_launcher = suite_launcher.clone();
            let tx = tx.clone();
            pool.execute(move || {
                suite_launcher.run_suite(index, tx);
            });
        }
        Ok((pool, rx))
    }

    /// Get the launch result through receiver.
    ///
    /// `rx` --- The receiver generated in `launch_suites` step.
    ///
    /// Return: `((AC count, UK count, WA/TLE count), Accepted launch indexes)`
    fn get_launch_result(&self, rx: mpsc::Receiver<LaunchSuiteResult>) -> ([u32; 3], Vec<u32>) {
        let mut launch_result_count = [0u32, 0u32, 0u32];
        let mut ac_launch_indexes = Vec::new();
        for _ in 1..=self.config.test_cases {
            let launch_result = rx.recv().expect("Receiver should receive");
            let (result_record_idx, log_content) = match launch_result.inner {
                LaunchSuiteEnum::AC(duration) => {
                    ac_launch_indexes.push(launch_result.index);
                    (
                        0,
                        format!("AC ({0:.3} ms)", duration.as_secs_f64() * 1000.0),
                    )
                }
                LaunchSuiteEnum::WA(duration, file, log_success) => (
                    2,
                    format!(
                        "WA ({0:.3} ms) : See difference in file {1}{2}",
                        duration.as_secs_f64() * 1000.0,
                        file.display(),
                        if log_success { "" } else { "[write failed]" }
                    ),
                ),
                LaunchSuiteEnum::TLE(duration) => (
                    2,
                    format!("TLE ({0:.3} ms)", duration.as_secs_f64() * 1000.0),
                ),
                LaunchSuiteEnum::UK(hint) => (1, format!("UK: {}", hint)),
            };
            launch_result_count[result_record_idx] += 1;
            self.logger.info(&format!(
                "Test #{0:02}: {1}",
                launch_result.index, log_content
            ));
        }
        (launch_result_count, ac_launch_indexes)
    }

    /// Print total score onto the screen with color.
    ///
    /// `launch_result_count` --- the array generated in `get_launch_result` step
    fn report_total_score(&self, launch_result_count: [u32; 3]) {
        self.logger.info(&format!(
            "Report: AC {} UK {} WA {} / Total {}",
            console::style(launch_result_count[0]).green(),
            console::style(launch_result_count[1]).yellow(),
            console::style(launch_result_count[2]).red(),
            self.config.test_cases
        ));
    }

    /// Clean generated files with `self.config.auto_remove_files` setting.
    ///
    /// `ac_launch_indexes` --- the vec generated in `get_launch_result` step
    fn clean_generated_files(&self, ac_launch_indexes: Vec<u32>) {
        match self.config.auto_remove_files.run(
            ac_launch_indexes,
            self.config.test_cases,
            &self.config.working_directory,
        ) {
            Ok(_) => self.logger.info("Remove files successfully."),
            Err(err) => self
                .logger
                .info(&format!("Failed to remove files: {}", err)),
        };
    }

    /// Try to compile a program.
    ///
    /// Returned value Explanation:
    /// - `Err(_)` => Compile error or non-UTF-8 error
    /// - `Ok(None)` => No correspond extension rule, skip compiling.
    /// - `Ok(Some(_))` => The target program after successful compilation.
    fn compile_one(&self, program: &PathBuf, stage: Stage) -> CheckerResult<Option<PathBuf>> {
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
                "No matched compilation config for \"{}\" (extension: {}), skip it",
                program.try_to_string()?,
                ext.try_to_string()?
            ));
            Ok(None)
        }
    }
}
