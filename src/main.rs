//! #TODO doc

mod checker_error;
mod compilation;
mod config;
mod dyn_formatting;
mod launch;
mod logger;

use checker_error::CheckerError;

fn main() {
    let mut oi_checker = OIChecker::new();
    oi_checker.run().unwrap_or_else(|e| e.destruct());
}

struct OIChecker {}

impl OIChecker {
    fn new() -> Self {
        Self {}
    }

    /// Main function, run the checker
    fn run(&mut self) -> Result<(), CheckerError> {
        // TODO
        let logger = logger::Logger::new(String::from("ROOT"), logger::Level::Info);
        let config = config::get_config()?;
        logger.info("配置读取成功！");
        todo!();
    }
}
