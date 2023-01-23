//! Main module: Launch the programs.

use crate::config::{cf_parsing, dynamic_format};
use crate::diff_tool;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub command: Option<String>,
    pub args: Vec<String>,
}

impl From<cf_parsing::LaunchConfig> for LaunchConfig {
    fn from(value: cf_parsing::LaunchConfig) -> Self {
        Self {
            command: Some(value.command.to_owned()),
            args: value.args.to_owned(),
        }
    }
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            command: None,
            args: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum LaunchOk {
    Success(Duration),
    Timeout(Duration),
}

impl LaunchConfig {
    /// Get the arguments
    fn get_args(&self, file: &PathBuf, stage: Stage) -> CheckerResult<Vec<String>> {
        // to give the &str longer lifetime
        let s_file = file.try_to_string()?;
        let args_dict: HashMap<&str, &str> = [("file", s_file.as_str())].into();
        let mut args: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in self.args.iter() {
            args.push(dynamic_format(arg, &args_dict, stage)?);
        }
        Ok(args)
    }

    /// TODO doc
    fn run_inner(
        mut command: Command,
        input_file: &Option<PathBuf>,
        output_file: &PathBuf,
        tx: mpsc::Sender<Result<(), io::Error>>,
    ) {
        use std::io::Write;
        let mut child = match command.spawn() {
            Ok(output) => output,
            Err(e) => {
                tx.send(Ok(())).ignore();
                tx.send(Err(e)).ignore();
                return;
            }
        };
        if let Some(input_file) = input_file {
            let input_buf = match fs::read(input_file) {
                Ok(input_buf) => input_buf,
                Err(e) => {
                    tx.send(Ok(())).ignore();
                    tx.send(Err(e)).ignore();
                    return;
                }
            };
            let input_buf = input_buf.as_slice();
            let mut child_stdin = child.stdin.take().expect("Stdin not piped");
            child_stdin.write(input_buf).ignore();
        }
        tx.send(Ok(())).ignore();
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                tx.send(Err(e)).ignore();
                return;
            }
        };
        tx.send(Ok(())).ignore();
        fs::write(output_file, output.stdout).ignore();
    }

    /// TODO doc
    pub fn run(
        &self,
        file: &PathBuf,
        stage: Stage,
        extra_args: Vec<String>,
        timeout: Duration,
        input_file: &Option<PathBuf>,
        output_file: &PathBuf,
    ) -> CheckerResult<LaunchOk> {
        let args = {
            let mut args = self.get_args(file, stage)?;
            args.extend(extra_args);
            args
        };
        let program = self.command.clone().unwrap_or(file.try_to_string()?);
        let (tx, rx) = mpsc::channel();
        let command: Command = {
            let mut command = Command::new(program.to_owned());
            command.args(&args).stdout(Stdio::piped());
            if let Some(_) = input_file {
                command.stdin(Stdio::piped());
            }
            command
        };
        let _input_file = input_file.to_owned();
        let _output_file = output_file.to_owned();
        let handle = thread::spawn(move || {
            Self::run_inner(command, &_input_file, &_output_file, tx);
        });
        rx.recv().ignore();
        let start = Instant::now();
        let received = rx.recv_timeout(timeout);
        let duration = start.elapsed();
        handle.join().ignore();
        if let Err(_) = received {
            Ok(LaunchOk::Timeout(duration))
        } else if let Ok(Err(err)) = received {
            Err(Box::new(CheckerError::CommandError {
                stage,
                command: program.to_owned(),
                args,
                file: program.into(),
                msg: format!("Error when launching: {}", err),
            }))
        } else {
            Ok(LaunchOk::Success(duration))
        }
    }
}

#[derive(Clone)]
pub struct SuiteLauncher {
    rules: crate::config::ExtensionRules<LaunchConfig>,
    test_cases: u32,
    program_timeout: Duration,
    accepted_timeout: Duration,
    working_directory: PathBuf,
    data_generator: PathBuf,
    accepted_program: PathBuf,
    tested_program: PathBuf,
    output_filters: Vec<crate::filter::OutputFilter>,
    diff_tool: crate::diff_tool::DiffTool,
}

impl SuiteLauncher {
    pub fn run_suite(&self, index: u32, tx: mpsc::Sender<LaunchSuiteResult>) {
        tx.send(LaunchSuiteResult {
            index,
            inner: self.run_suite_inner(index),
        })
        .expect("Sender should send successfully");
    }
    /// TODO doc
    fn run_one(
        &self,
        program: &PathBuf,
        extra_args: Vec<String>,
        input_file: &Option<PathBuf>,
        output_file: &PathBuf,
        stage: Stage,
    ) -> CheckerResult<LaunchOk> {
        let default_launch_rule = LaunchConfig::default();
        let launch_rule = if let Some(ext) = program.extension() {
            self.rules
                .get_rule(&ext.try_to_string()?)
                .unwrap_or(&default_launch_rule)
        } else {
            &default_launch_rule
        };
        launch_rule.run(
            program,
            stage,
            extra_args,
            self.program_timeout,
            input_file,
            output_file,
        )
    }

    /// TODO doc
    fn run_suite_inner(&self, index: u32) -> LaunchSuiteEnum {
        let work_dir = &self.working_directory;
        let data_file = work_dir.join(format!("data{}.in", index));
        let ac_out_file = work_dir.join(format!("ac{}.out", index));
        let tp_out_file = work_dir.join(format!("tested{}.out", index));

        let dg_result = self.run_one(
            &self.data_generator,
            [index.to_string(), self.test_cases.to_string()].into(),
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
            return LaunchSuiteEnum::UK(format!("Launch data generator failed: {}", hint));
        }

        let tp_result = self.run_one(
            &self.tested_program,
            Vec::new(),
            &Some(data_file.clone()),
            &tp_out_file,
            Stage::LaunchTP,
        );
        let tp_duration = match tp_result {
            Ok(LaunchOk::Success(duration) | LaunchOk::Timeout(duration)) => duration,
            Err(err) => {
                return LaunchSuiteEnum::UK(format!("Launch tested program failed: {}", err))
            }
        };
        if tp_duration > self.program_timeout {
            return LaunchSuiteEnum::TLE(tp_duration);
        }

        let ac_result = self.run_one(
            &self.accepted_program,
            [index.to_string(), self.test_cases.to_string()].into(),
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
            return LaunchSuiteEnum::UK(format!("Launch data generator failed: {}", hint));
        }

        for output_filter in self.output_filters.iter() {
            if let Err(err) = output_filter.run(&ac_out_file) {
                return LaunchSuiteEnum::UK(format!("Filter accepted output file failed: {}", err));
            };
            if let Err(err) = output_filter.run(&tp_out_file) {
                return LaunchSuiteEnum::UK(format!("Filter tested output file failed: {}", err));
            };
        }

        let diff_result = self.diff_tool.run(
            (&tp_out_file, &ac_out_file),
            &work_dir.join(format!("wa{}.log", index)),
        );
        match diff_result {
            Ok(diff_ok) => match diff_ok {
                diff_tool::DiffToolOk::Different {
                    log_path,
                    log_success,
                } => return LaunchSuiteEnum::WA(tp_duration, log_path, log_success),
                diff_tool::DiffToolOk::Success => (),
            },
            Err(err) => return LaunchSuiteEnum::UK(format!("Different tool failed: {}", err)),
        }
        if tp_duration <= self.accepted_timeout {
            LaunchSuiteEnum::AC(tp_duration)
        } else {
            LaunchSuiteEnum::TLE(tp_duration)
        }
    }
}

/// The result of launching a suite.
pub struct LaunchSuiteResult {
    pub index: u32,
    pub inner: LaunchSuiteEnum,
}

/// The inner enum of `LaunchSuiteEnum`.
pub enum LaunchSuiteEnum {
    AC(Duration),
    WA(Duration, PathBuf, bool),
    TLE(Duration),
    UK(String),
}

impl From<&crate::OIChecker> for SuiteLauncher {
    fn from(value: &crate::OIChecker) -> Self {
        let c = &value.config;
        Self {
            rules: c.launch_rules.to_owned(),
            test_cases: c.test_cases,
            program_timeout: c.program_timeout,
            accepted_timeout: c.ac_timeout,
            working_directory: c.working_directory.to_owned(),
            data_generator: c.data_generator.to_owned(),
            accepted_program: c.accepted_program.to_owned(),
            tested_program: c.tested_program.to_owned(),
            output_filters: c.output_filters.to_owned(),
            diff_tool: c.diff_tool.to_owned(),
        }
    }
}
