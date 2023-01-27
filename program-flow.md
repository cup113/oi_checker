# 程序运行流程

> 注：步骤后的❌表示尚未实现，⏳表示开发中或未经验证，✅表示开发完成且已验证，🆗表示已通过系统性测试。
>
> 为简化图表，箭头默认为该步骤执行成功后的行为；如无特别标注错误处理，均默认传给上一级程序进行处理。
>
> 步骤末尾的 `*` 表示详见下方备注。

## 总览 Overview

```mermaid
stateDiagram-v2
  GetConfig : 获取配置 * ✅
  InitWD : 初始化工作目录 * ✅
  Compile : 编译 3 个程序 * ✅
  Launch : 运行多组程序 * ✅
  GetRes : 获取运行结果 * ✅
  Report : 报告结果 * ✅
  Clean : 删除生成文件 * ✅
  FatalHandle : 关键错误处理 * ✅

  Success : 正常运行(状态码 0)
  Failed : 错误(状态码 1)
  FailedArgs : 命令行参数错误(状态码 2)

  [*] --> GetConfig
  GetConfig --> Success : 仅查看信息
  GetConfig --> InitWD
  InitWD --> Compile
  Compile --> Launch
  Launch --> GetRes
  GetRes --> Report
  Report --> Clean
  Clean --> Success

  GetConfig --> Failed : 配置文件读取错误
  GetConfig --> FailedArgs : 命令行参数错误
  InitWD --> FatalHandle : 创建目录失败
  Compile --> FatalHandle : 编译失败
  FatalHandle --> Failed

  Success --> [*]
  Failed --> [*]
  FailedArgs --> [*]
```

备注：

- 获取配置：详见[获取配置 Get Configuration](#获取配置-get-configuration)
- 初始化工作目录：检测目录是否存在，如不存在则创建。
- 编译 3 个程序：即尝试编译数据生成器、正确程序、待测程序。单次编译详见[编译程序 Compile Program](#编译程序-compile-program)
- 运行多组程序：根据运行配置中`test_cases`组程序组，单组运行详见[TODO]
- 获取运行结果：详见[TODO]
- 报告结果：将 `AC` `UK` `TLE` `WA` 为结果的样例数及总样例数彩色输出到终端中。
- 删除生成文件：详见[TODO]
- 关键错误处理：详见[TODO]

## 获取配置 Get Configuration

```mermaid
stateDiagram-v2
  Argument : 获取命令行配置 ✅
  File : 获取文件配置 * ✅
  Integrate : 整合配置 * ✅

  Success : 输出配置
  Info : 仅查看信息
  FailedArgs : 命令行参数错误
  Failed : 关键错误

  [*] --> Argument
  Argument --> File
  File --> Integrate
  Integrate --> Success

  Argument --> Info : 仅查看信息
  Argument --> FailedArgs : 命令行参数错误
  File --> Failed : 配置文件错误
  Integrate --> Failed : 整合配置错误

  Success --> [*]
  Info --> [*]
  FailedArgs --> [*]
  Failed --> [*]
```

备注：

- 获取文件配置：详见[获取文件配置 Get File Config](#获取文件配置-get-file-config)
- 整合配置：即以配置文件内容为基础，将传递的命令行参数覆盖，并对配置加以校验，校验失败则抛出错误。

## 获取文件配置 Get File Config

```mermaid
stateDiagram-v2
  Try1 : 尝试读取当前目录下的 oi_checker_config.toml ✅
  Try2 : 尝试读取程序目录下的 config.toml ✅
  Try3 : 尝试读取程序目录下的 config_default.toml 🆗
  Fallback : 生成 config_default.toml 到程序目录下 🆗
  ReadFile : 读取配置文件 🆗
  Deserialize : 反序列化（解析）文件 ✅

  Success : 返回文件配置
  Failed : 关键错误

  [*] --> Try1
  Try1 --> ReadFile
  Try2 --> ReadFile
  Try3 --> ReadFile
  Fallback --> ReadFile
  ReadFile --> Deserialize
  Deserialize --> Success

  Try1 --> Try2 : 文件不存在
  Try2 --> Try3 : 文件不存在
  Try3 --> Fallback : 文件不存在
  ReadFile --> Failed : 读取错误
  Deserialize --> Failed : 解析错误

  Success --> [*]
  Failed --> [*]
```

## 编译程序 Compile Program

```mermaid
stateDiagram-v2
  FmtArgs : 格式化参数 ✅
  Command : 运行编译命令 ✅
  GetStatus : 获取编译器状态码 ✅

  Success : 编译成功
  Failed : 关键错误

  [*] --> FmtArgs
  FmtArgs --> Command
  Command --> GetStatus
  GetStatus --> Success

  FmtArgs --> Failed : 格式化错误
  Command --> Failed : 编译错误
  GetStatus --> Failed : 状态码不为0
```
