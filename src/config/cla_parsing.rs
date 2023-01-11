use clap::{
    builder::{
        IntoResettable, NonEmptyStringValueParser, PathBufValueParser, RangedU64ValueParser,
        ValueParser,
    },
    parser::ValuesRef,
    Arg, ArgAction, Command,
};
use std::path::PathBuf;

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
            "test-cases",
            "cases",
            'c',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..),
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
            RangedU64ValueParser::<u32>::new().range(1..),
        ))
        .arg(make_arg(
            "program-timeout",
            "program-timeout",
            'e',
            "MILLISECONDS",
            RangedU64ValueParser::<u32>::new().range(1..),
        ))
        .arg(make_arg(
            "working-directory",
            "working-dir",
            'd',
            "MILLISECONDS",
            PathBufValueParser::new(),
        ))
        .arg(make_arg(
            "auto-remove-files",
            "auto-remove-files",
            'u',
            "AC|Always|Never",
            ["AC", "Always", "Never"],
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
                ]),
        )
        .arg(
            Arg::new("diff-tool")
                .long("diff-tool")
                .short('i')
                .value_name("TOOL")
                .value_delimiter(';')
                .value_parser(NonEmptyStringValueParser::new()),
        );
    let matches = app.get_matches();
    macro_rules! get_one {
        ($id: expr, $tp: ty) => {{
            let result: Option<$tp> = matches.get_one($id).map(|s: &$tp| s.to_owned());
            result
        }};
    }
    let tested_program = get_one!("tested-program", PathBuf);
    let accepted_program = get_one!("accepted-program", PathBuf);
    let data_generator = get_one!("data-generator", PathBuf);
    let test_cases = get_one!("test-cases", u32);
    let test_threads = get_one!("test-threads", u32);
    let ac_timeout = get_one!("ac-timeout", u32);
    let program_timeout = get_one!("program-timeout", u32);
    let working_directory = get_one!("working-directory", PathBuf);
    let auto_remove_files = get_one!("auto-remove-files", String);
    let output_filters = matches
        .get_many("output-filters")
        .map(|f: ValuesRef<String>| {
            f.to_owned()
                .map(|s: &String| s.to_owned())
                .collect::<Vec<_>>()
        });
    let diff_tool = matches.get_many("diff-tool").map(|f: ValuesRef<String>| {
        f.to_owned()
            .map(|s: &String| s.to_owned())
            .collect::<Vec<_>>()
    });
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

#[derive(Debug)]
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
    output_filters: Option<Vec<String>>,
    diff_tool: Option<Vec<String>>,
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

#[cfg(test)]
mod tests {}
