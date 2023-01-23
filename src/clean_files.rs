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
    ) -> Result<(), std::io::Error> {
        match self {
            Self::Never => Ok(()),
            Self::Always => {
                for i in 1..=test_cases {
                    fs::remove_file(work_dir.join(format!("data{}.in", i)))?;
                    fs::remove_file(work_dir.join(format!("ac{}.out", i)))?;
                    fs::remove_file(work_dir.join(format!("tested{}.out", i)))?;
                }
                if ac_launch_indexes.len() == test_cases as usize {
                    fs::remove_dir_all(work_dir)?;
                }
                Ok(())
            }
            Self::AC => {
                for i in ac_launch_indexes.iter() {
                    fs::remove_file(work_dir.join(format!("data{}.in", i)))?;
                    fs::remove_file(work_dir.join(format!("ac{}.out", i)))?;
                    fs::remove_file(work_dir.join(format!("tested{}.out", i)))?;
                }
                if ac_launch_indexes.len() == test_cases as usize {
                    fs::remove_dir_all(work_dir)?;
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
