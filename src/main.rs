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
    oi_checker.run().unwrap_or_else(|e| e.destruct());
}

struct OIChecker {
    logger: Logger,
}

impl OIChecker {
    fn new() -> Self {
        let logger = Logger::new(String::from("ROOT"), logging::Level::Info);
        Self { logger }
    }

    /// Main function, run the checker
    fn run(&mut self) -> Result<(), CheckerError> {
        // TODO
        let config = config::get_config()?;
        self.logger.info("配置读取成功！");
        macro_rules! try_compile {
            ($program: ident) => {
                if let Some(ext) = config.$program.extension() {
                    let rule = config.compilation_rules.get_rule(ext.try_to_string()?);
                    if let Some(rule) = rule {
                        rule.run(config.working_directory, config.$program)?;
                    }
                }
            };
        }
        try_compile!(accepted_program);
        todo!();
    }
}
