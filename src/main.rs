//! #TODO doc

mod checker_error;
mod compilation;
mod config;
mod dyn_formatting;
mod launch;
mod logging;
mod path_lib;

use checker_error::CheckerError;
use logging::Logger;

use crate::path_lib::TryToString;

fn main() {
    let mut oi_checker = OIChecker::new();
    if let Err(e) = oi_checker.run() {
        e.destruct();
    }
}

struct OIChecker {
    logger: Logger,
}

impl OIChecker {
    fn new() -> Self {
        use logging::Level::Info;
        let logger = Logger::new(String::from("OIChecker"), Info);
        Self { logger }
    }

    /// Main function, run the checker
    fn run(&mut self) -> Result<(), CheckerError> {
        use crate::checker_error::Stage;
        use std::fs;
        let config = config::get_config()?;
        self.logger.info("Parse configuration successfully.");
        if !config.working_directory.exists() {
            if let Err(err) = fs::create_dir(&config.working_directory) {
                return Err(CheckerError::CreateWorkDirError {
                    err,
                    dir: config.working_directory.to_owned(),
                });
            }
        }
        macro_rules! try_compile {
            ($program: ident, $stage: expr) => {
                if let Some(ext) = config.$program.extension() {
                    let rule = config.compilation_rules.get_rule(ext.try_to_string()?);
                    if let Some(rule) = rule {
                        rule.run(&config.working_directory, &config.$program, $stage)?;
                        self.logger.info(&format!(
                            "Compile {} successfully.",
                            config.$program.try_to_string()?
                        ));
                    } else {
                        self.logger.info(&format!(
                            "No matched compilation config for \"{}\", skip it.",
                            config.$program.try_to_string()?
                        ));
                    }
                }
            };
        }
        try_compile!(data_generator, Stage::CompileDG);
        try_compile!(accepted_program, Stage::CompileAC);
        try_compile!(tested_program, Stage::CompileTP);
        todo!(); // TODO
    }
}
