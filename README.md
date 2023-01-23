# OI-Checker README

可灵活配置的 OI 自动对拍器。

项目处于开发阶段，可能会发生破坏性改动。

[![wakatime](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5.svg)](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5)
[![GitHub repo size](https://img.shields.io/github/languages/code-size/cup113/oi_checker)](https://github.com/cup113/oi_checker)
[![Latest version](https://img.shields.io/github/v/release/cup113/oi_checker?include_prereleases)](https://github.com/cup113/oi_checker)
[![MIT License](https://img.shields.io/github/license/cup113/oi_checker)](https://github.com/cup113/oi_checker)

## Usage

```
Usage: oi_checker.exe [OPTIONS]

Options:
  -t, --tested <FILE>                   The program which will be tested.
  -a, --accepted <FILE>                 The program which output correct answers.
  -g, --generator <FILE>                The program which generate data.
  -c, --cases <MILLISECONDS>            Number of test cases. Each starts a test suite.
  -r, --threads <NUMBER>                Concurrent threads numbers.
  -m, --ac-timeout <MILLISECONDS>       If the tested program doesn't finish in this duration (in milliseconds), the result will be TLE.
  -e, --program-timeout <MILLISECONDS>  If any program of a test suite doesn't finish in this duration (in milliseconds), this suite will be terminated and the result will be Unknown.
  -d, --working-dir <MILLISECONDS>      The directory which stores data files and compiled files.
  -u, --auto-remove-files <STRING>      See `config_default.toml` for more information. [possible values: ac, always, never]
  -f, --output-filters <FILTERS>        See `config_default.toml` for more information. Split values with ',' [possible values: strip-trailing-whitespace, strip-trailing-empty-lines, strip-all-whitespace]
  -i, --diff-tool <TOOL>                See `config_default.toml` for more information. Split items with ';'
      --get-default-config              Print the default config.
  -h, --help                            Print help
  -V, --version                         Print version
```

## Config

详见`config_default.toml`。
