use crate::checker_error::{CheckerError, Stage};
use crate::config::cf_parsing;
use crate::TryToString;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;
use std::{fs, io};

#[derive(Debug)]
pub struct LaunchConfig {
    pub command: Option<String>,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum LaunchResult {
    Success(Duration),
    Timeout(Duration),
    IOError(io::Error),
    CheckerError(CheckerError),
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

impl LaunchConfig {
    /// TODO doc
    fn get_args(&self, file: &PathBuf, stage: Stage) -> Result<Vec<String>, CheckerError> {
        use crate::dyn_formatting;
        use std::collections::HashMap;
        // to give the &str longer lifetime
        let s_file = file.try_to_string()?;
        let args_dict: HashMap<&str, &str> = HashMap::from([("file", s_file.as_str())]);
        let mut args: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in self.args.iter() {
            args.push(dyn_formatting::dynamic_format(arg, &args_dict, stage)?);
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
                let _ = tx.send(Ok(()));
                let _ = tx.send(Err(e));
                return;
            }
        };
        if let Some(input_file) = input_file {
            let input_buf = match fs::read(input_file) {
                Ok(input_buf) => input_buf,
                Err(e) => {
                    let _ = tx.send(Ok(()));
                    let _ = tx.send(Err(e));
                    return;
                }
            };
            let input_buf = input_buf.as_slice();
            let mut child_stdin = child.stdin.take().expect("Stdin not piped");
            let _ = child_stdin.write(input_buf);
        }
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                let _ = tx.send(Err(e));
                return;
            }
        };
        let _ = fs::write(output_file, output.stdout);
        let _ = tx.send(Ok(()));
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
    ) -> LaunchResult {
        use std::process::Stdio;
        use std::thread;
        use std::time::Instant;
        let args = {
            let mut args = match self.get_args(file, stage) {
                Ok(args) => args,
                Err(err) => return LaunchResult::CheckerError(err),
            };
            args.extend(extra_args);
            args
        };
        let program = self.command.clone().unwrap_or(match file.try_to_string() {
            Ok(file) => file,
            Err(err) => return LaunchResult::CheckerError(err),
        });
        println!("{}", program);
        let (tx, rx) = mpsc::channel();
        let command: Command = {
            let mut command = Command::new(program);
            command.args(args).stdout(Stdio::piped());
            if let Some(_) = input_file {
                command.stdin(Stdio::piped());
            }
            command
        };
        let _input_file = input_file.to_owned();
        let _output_file = output_file.to_owned();
        thread::spawn(move || {
            Self::run_inner(command, &_input_file, &_output_file, tx);
        });
        let _ = rx.recv();
        let start = Instant::now();
        let received = rx.recv_timeout(timeout);
        if let Err(_) = received {
            LaunchResult::Timeout(start.elapsed())
        } else if let Ok(Err(err)) = received {
            LaunchResult::IOError(err)
        } else {
            LaunchResult::Success(start.elapsed())
        }
    }
}
