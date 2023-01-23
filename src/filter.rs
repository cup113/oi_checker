//! Filter output.

use crate::prelude::*;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum OutputFilter {
    StripTrailingWhitespace,
    StripTrailingEmptyLines,
    StripAllWhitespace,
}

impl OutputFilter {
    /// Inner choices
    fn run_strip_trailing_whitespace(content: &String) -> Vec<String> {
        let mut ans = Vec::new();
        for line in content.lines() {
            ans.push(line.trim_end().into());
        }
        ans
    }

    /// Inner choices
    fn run_strip_trailing_empty_lines(content: &String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut buffer_empty_lines = 0u32;
        for line in content.lines() {
            if line.is_empty() {
                buffer_empty_lines += 1;
            } else {
                ans.push(line.into());
                for _ in 0..buffer_empty_lines {
                    ans.push("".into());
                }
                buffer_empty_lines = 0;
            }
        }
        ans
    }

    /// Inner choices
    fn run_strip_all_whitespace<'a>(content: &'a String) -> Vec<String> {
        let mut ans = Vec::new();
        for line in content.lines() {
            ans.push(line.chars().filter(|c| c.is_whitespace()).collect());
        }
        ans
    }

    /// Filter the output file. Read `file` all into memory, filter and write
    /// back to the `file`.
    ///
    /// Error when meeting IO errors
    pub fn run(&self, file: &PathBuf) -> CheckerResult<()> {
        macro_rules! deal_io_err {
            ($result: expr) => {
                $result.map_err(|err| CheckerError::FilterError {
                    filter: *self,
                    err,
                    file: file.to_owned(),
                })?
            };
        }
        let original_content = deal_io_err!(fs::read_to_string(file));
        let filtered_lines = match self {
            Self::StripTrailingWhitespace => Self::run_strip_trailing_whitespace(&original_content),
            Self::StripTrailingEmptyLines => {
                Self::run_strip_trailing_empty_lines(&original_content)
            }
            Self::StripAllWhitespace => Self::run_strip_all_whitespace(&original_content),
        };
        let mut output_file = deal_io_err!(File::create(file));
        for filtered_line in filtered_lines {
            deal_io_err!(output_file.write(filtered_line.as_bytes()));
            deal_io_err!(output_file.write(crate::util::LINE_END.as_bytes()));
        }
        Ok(())
    }
}

impl Display for OutputFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::StripTrailingWhitespace => "strip trailing whitespace",
            Self::StripTrailingEmptyLines => "strip trailing empty lines",
            Self::StripAllWhitespace => "strip all whitespace",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for OutputFilter {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "strip-trailing-whitespace" => Ok(Self::StripTrailingWhitespace),
            "strip-trailing-empty-lines" => Ok(Self::StripTrailingEmptyLines),
            "strip-all-whitespace" => Ok(Self::StripAllWhitespace),
            f => Err(format!(
                "filter {} is not defined in field `default.filters`",
                f
            )),
        }
    }
}
