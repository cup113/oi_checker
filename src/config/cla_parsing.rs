use clap::{
    builder::{IntoResettable, PathBufValueParser, RangedU64ValueParser, ValueParser},
    Arg, ArgAction, Command,
};
use std::{path::PathBuf, time::Duration};

pub struct ClaConfig {
    tested_program: Option<PathBuf>,
    accepted_program: Option<PathBuf>,
    data_generator: Option<PathBuf>,
    test_cases: Option<u32>,
    test_threads: Option<u32>,
    ac_timeout: Option<u32>,
    program_timeout: Option<u32>,
    working_directory: Option<PathBuf>,
    auto_remove_files: Option<String>,
    output_filters: Option<String>,
    diff_tool: Option<String>,
}

fn make_arg(
    id: &'static str,
    long: &'static str,
    short: char,
    value_name: &'static str,
    value_parser: impl IntoResettable<ValueParser>,
) -> Arg {
    Arg::new(id)
        .action(ArgAction::Set)
        .long(long)
        .short(short)
        .value_name(value_name)
        .value_parser(value_parser)
}

pub fn parse_cla() {
    let app = Command::new(env!("CARGO_PKG_NAME"))
        .about("An OI Checker. To see more information, please see README.html")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(make_arg(
            "tested-program",
            "tested",
            't',
            "FILE",
            PathBufValueParser::new(),
        ))
        .arg(make_arg(
            "accepted-program",
            "accepted",
            'a',
            "FILE",
            PathBufValueParser::new(),
        ))
        .arg(make_arg(
            "data-generator",
            "generator",
            'g',
            "FILE",
            PathBufValueParser::new(),
        ))
        .arg(make_arg(
            "test-threads",
            "threads",
            'r',
            "NUMBER",
            RangedU64ValueParser::<u32>::new().range(1..=125),
        ))
        .arg(make_arg(
            "ac-timeout",
            "ac-timeout",
            'm',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..)
        ))
        .arg(make_arg(
            "program-timeout",
            "program-timeout",
            'e',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..)
        ))
        .arg(make_arg(
            "test-cases",
            "test-cases",
            'e',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..)
        ));
    app.get_matches();
}
