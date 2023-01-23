//! Command-line argument parser.

use crate::prelude::*;
use clap::{
    builder::{
        IntoResettable, NonEmptyStringValueParser, PathBufValueParser, RangedU64ValueParser,
        ValueParser,
    },
    parser::ValuesRef,
    Arg, ArgAction, Command,
};

/// Parse command-line arguments
pub fn parse_cla() -> ClaConfig {
    let app = Command::new(env!("CARGO_PKG_NAME"))
        .about("An OI Checker. To get more information, please see README.html")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(make_arg(
            "tested-program",
            "tested",
            't',
            "FILE",
            PathBufValueParser::new(),
            "The program which will be tested.",
        ))
        .arg(make_arg(
            "accepted-program",
            "accepted",
            'a',
            "FILE",
            PathBufValueParser::new(),
            "The program which output correct answers.",
        ))
        .arg(make_arg(
            "data-generator",
            "generator",
            'g',
            "FILE",
            PathBufValueParser::new(),
            "The program which generate data.",
        ))
        .arg(make_arg(
            "test-cases",
            "cases",
            'c',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..),
            "Number of test cases. Each starts a test suite.",
        ))
        .arg(make_arg(
            "test-threads",
            "threads",
            'r',
            "NUMBER",
            RangedU64ValueParser::<u32>::new().range(1..=255),
            "Concurrent threads numbers.",
        ))
        .arg(make_arg(
            "ac-timeout",
            "ac-timeout",
            'm',
            "MILLISECONDS",
            RangedU64ValueParser::<u64>::new().range(1..),
            "If the tested program doesn't finish in this duration \
            (in milliseconds), the result will be TLE.",
        ))
        .arg(make_arg(
            "program-timeout",
            "program-timeout",
            'e',
            "MILLISECONDS",
            RangedU64ValueParser::<u64>::new().range(1..),
            "If any program of a test suite doesn't finish in this duration \
            (in milliseconds), this suite will be terminated \
            and the result will be Unknown.",
        ))
        .arg(make_arg(
            "working-directory",
            "working-dir",
            'd',
            "MILLISECONDS",
            PathBufValueParser::new(),
            "The directory which stores data files and compiled files.",
        ))
        .arg(make_arg(
            "auto-remove-files",
            "auto-remove-files",
            'u',
            "STRING",
            ["ac", "always", "never"],
            "See `config_default.toml` for more information.",
        ))
        .arg(
            Arg::new("output-filters")
                .long("output-filters")
                .short('f')
                .value_name("FILTERS")
                .value_delimiter(',')
                .value_parser([
                    "strip-trailing-whitespace",
                    "strip-trailing-empty-lines",
                    "strip-all-whitespace",
                ])
                .help(
                    "See `config_default.toml` for more information. \
                    Split values with ','",
                ),
        )
        .arg(
            Arg::new("diff-tool")
                .long("diff-tool")
                .short('i')
                .value_name("TOOL")
                .value_delimiter(';')
                .value_parser(NonEmptyStringValueParser::new())
                .help(
                    "See `config_default.toml` for more information. \
                    Split items with ';'",
                ),
        )
        .arg(
            Arg::new("get-default-config")
                .long("get-default-config")
                .action(ArgAction::SetTrue)
                .help("Print the default config."),
        );
    let matches = app.get_matches();
    if matches.get_flag("get-default-config") {
        println!("{}", crate::config::CONFIG_FILE_DEFAULT);
        std::process::exit(0);
    }

    /// Get an usual config item
    macro_rules! get_one {
        ($id: expr, $tp: ty) => {{
            let result: Option<$tp> = matches.get_one($id).map(|s: &$tp| s.to_owned());
            result
        }};
    }

    let get_many_string = |id: &str| -> Option<Vec<String>> {
        matches.get_many(id).map(|values_ref: ValuesRef<String>| {
            values_ref.map(|value_ref| value_ref.to_owned()).collect()
        })
    };

    let tested_program = get_one!("tested-program", PathBuf);
    let accepted_program = get_one!("accepted-program", PathBuf);
    let data_generator = get_one!("data-generator", PathBuf);
    let test_cases = get_one!("test-cases", u32);
    let test_threads = get_one!("test-threads", u32);
    let ac_timeout = get_one!("ac-timeout", u64);
    let program_timeout = get_one!("program-timeout", u64);
    let working_directory = get_one!("working-directory", PathBuf);
    let auto_remove_files = get_one!("auto-remove-files", String);
    let output_filters = get_many_string("output-filters");
    let diff_tool = get_many_string("diff-tool");
    ClaConfig {
        tested_program,
        accepted_program,
        data_generator,
        test_cases,
        test_threads,
        ac_timeout,
        program_timeout,
        working_directory,
        auto_remove_files,
        output_filters,
        diff_tool,
    }
}

/// Command line arguments configuration.
#[derive(Debug)]
pub struct ClaConfig {
    pub tested_program: Option<PathBuf>,
    pub accepted_program: Option<PathBuf>,
    pub data_generator: Option<PathBuf>,
    pub test_cases: Option<u32>,
    pub test_threads: Option<u32>,
    pub ac_timeout: Option<u64>,
    pub program_timeout: Option<u64>,
    pub working_directory: Option<PathBuf>,
    pub auto_remove_files: Option<String>,
    pub output_filters: Option<Vec<String>>,
    pub diff_tool: Option<Vec<String>>,
}

/// Make an argument for most case to reuse the code.
fn make_arg(
    id: &'static str,
    long: &'static str,
    short: char,
    value_name: &'static str,
    value_parser: impl IntoResettable<ValueParser>,
    help: &'static str,
) -> Arg {
    Arg::new(id)
        .action(ArgAction::Set)
        .long(long)
        .short(short)
        .value_name(value_name)
        .value_parser(value_parser)
        .help(help)
}
