//! Clean the files after launching.

use crate::prelude::*;

/// Clean files setting.
#[derive(Debug)]
pub enum AutoRemoveFiles {
    AC,
    Always,
    Never,
}

impl AutoRemoveFiles {
    /// Clean the files.
    pub fn run(
        &self,
        ac_launch_indexes: Vec<u32>,
        test_cases: u32,
        work_dir: &PathBuf,
        created_work_dir: bool,
    ) -> Result<(), io::Error> {
        let remove_suite = |i: u32| -> io::Result<()> {
            fs::remove_file(work_dir.join(format!("data{}.in", i)))?;
            fs::remove_file(work_dir.join(format!("ac{}.out", i)))?;
            fs::remove_file(work_dir.join(format!("tested{}.out", i)))?;
            Ok(())
        };

        let remove_all = || -> io::Result<()> {
            // If working directory exists originally, it shouldn't be removed.
            if created_work_dir {
                fs::remove_dir_all(work_dir)?;
                crate::LOGGER.info("Remove working directory.");
            }
            Ok(())
        };

        match self {
            Self::Never => Ok(()),
            Self::Always => {
                for i in 1..=test_cases {
                    remove_suite(i)?;
                }
                crate::LOGGER.info(&format!("Remove all {} generated files.", test_cases));
                remove_all()?;
                Ok(())
            }
            Self::AC => {
                for i in ac_launch_indexes.iter() {
                    remove_suite(*i)?;
                }
                crate::LOGGER.info(&format!(
                    "Remove {} generated files.",
                    ac_launch_indexes.len()
                ));
                if ac_launch_indexes.len() == test_cases as usize {
                    remove_all()?;
                }
                Ok(())
            }
        }
    }
}

impl TryFrom<&str> for AutoRemoveFiles {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ac" => Ok(Self::AC),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            s => Err(format!(
                "`{}` is not allowed in field `auto_remove_files`",
                s
            )),
        }
    }
}
