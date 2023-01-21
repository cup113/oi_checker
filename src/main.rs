//! #TODO doc

mod checker_error;
mod compilation;
mod config;
mod dyn_formatting;
mod launch;
mod logging;
mod path_lib;

use std::path::PathBuf;

use crate::path_lib::TryToString;
use checker_error::{CheckerError, Stage};
use logging::Logger;

fn main() {
    let mut oi_checker = OIChecker::new().unwrap_or_else(|err| err.destruct());
    oi_checker.run().unwrap_or_else(|err| err.destruct());
}

struct OIChecker {
    logger: Logger,
    config: config::Config,
}

impl OIChecker {
    fn new() -> Result<Self, CheckerError> {
        use logging::Level::Info;
        let logger = Logger::new(String::from("OIChecker"), Info);
        let config = config::get_config()?;
        Ok(Self { logger, config })
    }

    fn try_compile(&self, program_ptr: *mut PathBuf, stage: Stage) -> Result<(), CheckerError> {
        let program = unsafe { (*program_ptr).to_owned() };
        let ext = if let Some(ext) = program.extension() {
            ext
        } else {
            self.logger.info(&format!(
                "No extension name for \"{}\", skip it.",
                program.try_to_string()?
            ));
            return Ok(());
        };
        let rule = self.config.compilation_rules.get_rule(ext.try_to_string()?);
        if let Some(rule) = rule {
            let target = rule.run(&self.config.working_directory, &program, stage)?;
            self.logger.info(&format!(
                "Compile {} successfully.",
                program.try_to_string()?
            ));
            unsafe {
                (*program_ptr) = PathBuf::from(target);
            }
        } else {
            self.logger.info(&format!(
                "No matched compilation config for \"{}\" (extension = {}), skip it",
                program.try_to_string()?,
                ext.try_to_string()?
            ));
        }
        Ok(())
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
        let data_generator_ptr = &mut self.config.data_generator as *mut PathBuf;
        let accepted_program_ptr = &mut self.config.accepted_program as *mut PathBuf;
        let tested_program_ptr = &mut self.config.tested_program as *mut PathBuf;
        self.try_compile(data_generator_ptr, Stage::CompileDG)?;
        self.try_compile(accepted_program_ptr, Stage::CompileAC)?;
        self.try_compile(tested_program_ptr, Stage::CompileTP)?;
        todo!(); // TODO
    }
}
