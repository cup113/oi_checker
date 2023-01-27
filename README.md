# OI-Checker README

可灵活配置、基于 Rust 开发的 OI 自动对拍器。

[![wakatime](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5.svg)](https://wakatime.com/badge/user/b039f61c-2701-482d-9f84-542f07630e52/project/d4ca9e8d-4006-440d-92c7-b95b26fda0e5)
[![GitHub repo size](https://img.shields.io/github/languages/code-size/cup113/oi_checker)](https://github.com/cup113/oi_checker)
[![Latest version](https://img.shields.io/github/v/release/cup113/oi_checker?include_prereleases)](https://github.com/cup113/oi_checker)
[![MIT License](https://img.shields.io/github/license/cup113/oi_checker)](https://github.com/cup113/oi_checker)

## 程序优势

- 体积小。安装包展开不超过 10MB 。
- 支持路径名为包括中文在内的所有 `Unicode` 有效字符。
- 自动化测试。这意味着您可以让它运行成百上千的测试以尽可能覆盖边缘情况。
- 多线程。加速运行测评。
- 友好的错误提示。程序对于考虑到的常见错误给予了非常详细的提示。

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

## 核心逻辑：程序验证

### 名词解释

- `Data Generator` (DG) 数据生成器
- `ACcepted program` (AC) 正确程序
- `Tested Program` (TP) 待测程序
- `Output Filter` (Filter) 输出过滤程序
- `Diff Tool` 文本差异比较程序
- `AC` Accepted 输出正确
- `UK` Unknown 结果未知
- `TLE` Time Limit Exceeded 程序超时
- `WA` Wrong Answer 程序输出错误

注：不考虑 `CE` (Compile Error) 是因为无法被编译会使主程序提前退出。

### 程序要求

- 数据生成器：能生成数据范围内的数据。不接受除命令行参数以外的输入，输出到 `stdout` 中，可选根据测试样例编号不同提供不同难度的数据，尽量能覆盖边缘情况。
- 正确程序：别人写的程序，或自己编写的更直接、暴力（或使用了第三方语言/库）的程序，从 `stdin` 中读取数据，输出结果到 `stdout` 中。若该程序超时，则可能会引发 `UK` 。
- 待测程序：待评测的程序。从 `stdin` 中读取数据，输出结果到 `stdout` 中。
- 输出过滤程序：暂时只支持内置 `3` 种过滤程序。
- 文本差异比较程序：接受最后两个文件名为比较的两个文件，并在两文件相同（或符合要求）时返回状态码 `0` ，不同时返回其他任意状态码（即与 Windows `FC`, bash `diff` 的行为保持一致）。

### Trick

- 如果你想只基于提供的样例输入输出进行测评，可以将数据生成器编写为按照序号从数据文件中读入，再输出到 `stdout` 中；将正确程序编写为按序号从预期输出结果中读入（忽略数据内容），再输出到 `stdout` 中。但这样无法测试更多的样例。
- 如果一个问题构造数据时可以较容易地得到它的答案，那么可以在数据生成器中将预期结果输出到暂存文件中（注：文件名记得带上测试序号，否则会被覆盖），然后正确程序编写为从对应文件中读取并输出至 `stdout` 中，之后删除暂存文件。
