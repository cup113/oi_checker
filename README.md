# OI-Checker README

可灵活配置、基于 Rust 开发的 OI 自动对拍器。

[![wakatime](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5.svg)](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5)
[![GitHub repo size](https://img.shields.io/github/languages/code-size/cup113/oi_checker)](https://github.com/cup113/oi_checker)
[![Latest version](https://img.shields.io/github/v/release/cup113/oi_checker?include_prereleases)](https://github.com/cup113/oi_checker)
[![MIT License](https://img.shields.io/github/license/cup113/oi_checker)](https://github.com/cup113/oi_checker)

## 命令行参数

```
An OI Checker. To get more information, please see README.html

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

## 配置文件

详见`config_default.toml`。

## 使用步骤

1. 安装程序（下载安装包或 `cargo install` 本地编译）。
2. 将程序所在目录添加至 `PATH` 中。
3. 将程序平级的 `config_default.toml` 复制一份到 `config.toml` ，并根据喜好进行修改。
4. 根据命令行参数使用
  - 注: 若需要为一个项目单独设一套配置，可运行命令
    `oi_checker --get-default-config > oi_checker_config.toml` ，然后进行修改。

## 程序运行流程

大致流程详见`state_diagrams/overview.mmd`

TODO
