# OI-Checker README

可灵活配置的 OI 自动对拍器。

项目处于开发阶段，可能会发生破坏性改动。

[![wakatime](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5.svg)](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5)
[![GitHub repo size](https://img.shields.io/github/languages/code-size/cup113/oi_checker)](https://github.com/cup113/oi_checker)
[![Latest version](https://img.shields.io/github/v/release/cup113/oi_checker?include_prereleases)](https://github.com/cup113/oi_checker)
[![MIT License](https://img.shields.io/github/license/cup113/oi_checker)](https://github.com/cup113/oi_checker)

## Usage

```
An OI Checker. To get more information, please see README.html

Usage: oi_checker.exe [OPTIONS]

Options:
  -t, --tested <FILE>

  -a, --accepted <FILE>

  -g, --generator <FILE>

  -c, --cases <MILLISECONDS>

  -r, --threads <NUMBER>

  -m, --ac-timeout <MILLISECONDS>

  -e, --program-timeout <MILLISECONDS>

  -d, --working-dir <MILLISECONDS>

  -u, --auto-remove-files <ac|always|never>
          [possible values: ac, always, never]
  -f, --output-filters <FILTERS>
          [possible values: strip-trailing-whitespace, strip-trailing-empty-lines, strip-all-whitespace]
  -i, --diff-tool <TOOL>

      --get-default-config

  -h, --help
          Print help
  -V, --version
          Print version
```

## Config

详见`config_default.toml`。
