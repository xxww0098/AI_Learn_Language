# Clap 4.5.40 - Rust 命令行参数解析完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [基本用法](#基本用法)
- [派生宏API](#派生宏API)
- [构建器API](#构建器API)
- [高级特性](#高级特性)
- [子命令系统](#子命令系统)
- [配置和环境变量](#配置和环境变量)
- [错误处理](#错误处理)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Clap 是 Rust 中最流行的命令行参数解析库，提供了简单易用、高效且功能完整的 CLI 应用程序构建能力。

### 核心特性
- **多种API风格**: 支持派生宏和构建器两种API风格
- **功能丰富**: 支持位置参数、选项、标志、子命令等
- **自动生成**: 自动生成帮助信息和补全脚本
- **类型安全**: 编译时类型检查和验证
- **高度可配置**: 灵活的自定义和扩展能力

### 版本信息
- **当前版本**: 4.5.40
- **发布时间**: 2025-06-09
- **下载次数**: 454,984,340+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
clap = { version = "4.5.40", features = ["derive"] }
```

### 基本示例

```rust
use clap::{Arg, Command, ArgMatches};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .about("一个示例应用程序")
        .author("作者名 <email@example.com>")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("设置输入文件")
                .required(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("启用详细输出")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let verbose = matches.get_flag("verbose");

    println!("输入文件: {}", input_file);
    if verbose {
        println!("详细模式已启用");
    }
}
```

### 使用派生宏

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "一个示例应用程序", long_about = None)]
struct Cli {
    /// 输入文件路径
    #[arg(short, long, value_name = "FILE")]
    input: String,

    /// 启用详细输出
    #[arg(short, long)]
    verbose: bool,

    /// 输出格式
    #[arg(short, long, default_value = "json")]
    format: String,
}

fn main() {
    let cli = Cli::parse();

    println!("输入文件: {}", cli.input);
    println!("输出格式: {}", cli.format);
    
    if cli.verbose {
        println!("详细模式已启用");
    }
}
```

## 核心概念

### 参数类型

```rust
use clap::{Arg, Command, ArgAction};

fn argument_types() {
    let app = Command::new("demo")
        // 位置参数
        .arg(
            Arg::new("file")
                .help("要处理的文件")
                .required(true)
                .index(1)
        )
        // 选项参数
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出文件路径")
        )
        // 标志参数
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("详细输出")
                .action(ArgAction::SetTrue)
        )
        // 多值参数
        .arg(
            Arg::new("exclude")
                .short('e')
                .long("exclude")
                .help("排除的文件模式")
                .action(ArgAction::Append)
        );

    let matches = app.get_matches();
    
    // 获取不同类型的参数值
    if let Some(file) = matches.get_one::<String>("file") {
        println!("处理文件: {}", file);
    }
    
    if let Some(output) = matches.get_one::<String>("output") {
        println!("输出到: {}", output);
    }
    
    if matches.get_flag("verbose") {
        println!("详细模式已启用");
    }
    
    let excludes: Vec<_> = matches
        .get_many::<String>("exclude")
        .unwrap_or_default()
        .collect();
    if !excludes.is_empty() {
        println!("排除模式: {:?}", excludes);
    }
}
```

### 值解析和验证

```rust
use clap::{Arg, Command, value_parser};

fn value_parsing() {
    let app = Command::new("calculator")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .value_parser(value_parser!(i32))
                .help("一个整数")
        )
        .arg(
            Arg::new("rate")
                .short('r')
                .long("rate")
                .value_parser(value_parser!(f64))
                .help("一个浮点数")
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(clap::value_parser!(u16).range(1..=65535))
                .help("端口号 (1-65535)")
        );

    let matches = app.get_matches();
    
    if let Some(number) = matches.get_one::<i32>("number") {
        println!("数字: {}", number);
    }
    
    if let Some(rate) = matches.get_one::<f64>("rate") {
        println!("比率: {}", rate);
    }
    
    if let Some(port) = matches.get_one::<u16>("port") {
        println!("端口: {}", port);
    }
}
```

## 基本用法

### 简单CLI应用

```rust
use clap::{Arg, Command};

fn simple_cli() {
    let matches = Command::new("file-processor")
        .version("1.0.0")
        .about("文件处理工具")
        .arg(
            Arg::new("input")
                .help("输入文件路径")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出文件路径")
                .default_value("output.txt")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("输出格式")
                .value_parser(["json", "xml", "csv"])
                .default_value("json")
        )
        .arg(
            Arg::new("compress")
                .short('c')
                .long("compress")
                .help("压缩输出")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let compress = matches.get_flag("compress");

    println!("处理文件: {} -> {}", input, output);
    println!("格式: {}", format);
    if compress {
        println!("启用压缩");
    }
}
```

### 复杂参数配置

```rust
use clap::{Arg, Command, ArgGroup, ArgAction};

fn complex_arguments() {
    let app = Command::new("advanced-tool")
        .version("2.0.0")
        .about("高级工具示例")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("配置文件路径")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("详细输出级别")
                .action(ArgAction::Count)
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("静默模式")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("N")
                .help("线程数量")
                .value_parser(clap::value_parser!(usize).range(1..=32))
                .default_value("4")
        )
        .arg(
            Arg::new("input-format")
                .long("input-format")
                .value_name("FORMAT")
                .help("输入格式")
                .value_parser(["auto", "json", "yaml", "toml"])
                .default_value("auto")
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("仅显示将要执行的操作")
                .action(ArgAction::SetTrue)
        )
        // 参数组：verbose 和 quiet 互斥
        .group(
            ArgGroup::new("verbosity")
                .args(["verbose", "quiet"])
                .required(false)
        );

    let matches = app.get_matches();
    
    let verbose_level = matches.get_count("verbose");
    let quiet = matches.get_flag("quiet");
    let threads = matches.get_one::<usize>("threads").unwrap();
    let input_format = matches.get_one::<String>("input-format").unwrap();
    let dry_run = matches.get_flag("dry-run");

    if quiet {
        println!("静默模式");
    } else {
        match verbose_level {
            0 => println!("正常输出"),
            1 => println!("详细输出"),
            2 => println!("更详细输出"),
            _ => println!("调试输出"),
        }
    }

    println!("使用 {} 个线程", threads);
    println!("输入格式: {}", input_format);
    
    if dry_run {
        println!("模拟运行模式");
    }
}
```

## 派生宏API

### 基本结构体

```rust
use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(version = "1.0.0")]
#[command(about = "演示派生宏API", long_about = None)]
struct Cli {
    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// 详细程度
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// 工作目录
    #[arg(short = 'C', long, value_name = "DIR")]
    directory: Option<String>,

    /// 输出格式
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// 子命令
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Xml,
}

#[derive(Subcommand)]
enum Commands {
    /// 处理文件
    Process {
        /// 输入文件
        input: String,
        /// 输出文件
        #[arg(short, long)]
        output: Option<String>,
        /// 处理选项
        #[command(flatten)]
        opts: ProcessOptions,
    },
    /// 验证配置
    Validate {
        /// 配置文件
        config: String,
        /// 严格模式
        #[arg(long)]
        strict: bool,
    },
}

#[derive(Args)]
struct ProcessOptions {
    /// 并行处理
    #[arg(short, long)]
    parallel: bool,

    /// 最大文件大小 (MB)
    #[arg(long, default_value = "100")]
    max_size: u64,

    /// 排除模式
    #[arg(short, long)]
    exclude: Vec<String>,
}

fn derive_api_demo() {
    let cli = Cli::parse();

    if let Some(config) = cli.config {
        println!("使用配置文件: {}", config);
    }

    match cli.verbose {
        0 => println!("正常输出"),
        1 => println!("详细输出"),
        2 => println!("调试输出"),
        _ => println!("跟踪输出"),
    }

    println!("输出格式: {:?}", cli.format);

    match cli.command {
        Some(Commands::Process { input, output, opts }) => {
            println!("处理文件: {}", input);
            if let Some(output) = output {
                println!("输出到: {}", output);
            }
            
            if opts.parallel {
                println!("使用并行处理");
            }
            
            println!("最大文件大小: {} MB", opts.max_size);
            
            if !opts.exclude.is_empty() {
                println!("排除模式: {:?}", opts.exclude);
            }
        }
        Some(Commands::Validate { config, strict }) => {
            println!("验证配置: {}", config);
            if strict {
                println!("使用严格模式");
            }
        }
        None => {
            println!("没有指定子命令");
        }
    }
}
```

### 自定义验证

```rust
use clap::{Parser, ArgAction};
use std::path::PathBuf;

#[derive(Parser)]
struct AppArgs {
    /// 输入文件 (必须存在)
    #[arg(short, long, value_parser = validate_input_file)]
    input: PathBuf,

    /// 输出目录 (必须是目录)
    #[arg(short, long, value_parser = validate_output_dir)]
    output: PathBuf,

    /// 端口号 (1024-65535)
    #[arg(short, long, value_parser = validate_port)]
    port: u16,

    /// 线程数 (1-CPU核心数)
    #[arg(short, long, value_parser = validate_threads)]
    threads: usize,
}

fn validate_input_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        Err(format!("输入文件不存在或不是文件: {}", s))
    }
}

fn validate_output_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else if !path.exists() {
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("无法创建输出目录: {}", e))?;
        Ok(path)
    } else {
        Err(format!("输出路径不是目录: {}", s))
    }
}

fn validate_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse()
        .map_err(|_| format!("无效的端口号: {}", s))?;
    
    if port < 1024 {
        Err("端口号必须大于等于 1024".to_string())
    } else {
        Ok(port)
    }
}

fn validate_threads(s: &str) -> Result<usize, String> {
    let threads: usize = s.parse()
        .map_err(|_| format!("无效的线程数: {}", s))?;
    
    let max_threads = num_cpus::get();
    if threads == 0 {
        Err("线程数必须大于 0".to_string())
    } else if threads > max_threads {
        Err(format!("线程数不能超过 CPU 核心数: {}", max_threads))
    } else {
        Ok(threads)
    }
}

fn custom_validation_demo() {
    let args = AppArgs::parse();
    
    println!("输入文件: {:?}", args.input);
    println!("输出目录: {:?}", args.output);
    println!("端口: {}", args.port);
    println!("线程数: {}", args.threads);
}
```

## 构建器API

### 动态命令构建

```rust
use clap::{Arg, Command, ArgMatches};

fn dynamic_command_building() {
    let mut app = Command::new("dynamic-app")
        .version("1.0.0")
        .about("动态构建的CLI应用");

    // 根据条件添加参数
    if cfg!(debug_assertions) {
        app = app.arg(
            Arg::new("debug")
                .long("debug")
                .help("启用调试模式")
                .action(clap::ArgAction::SetTrue)
        );
    }

    // 动态添加环境相关的参数
    let env_vars = ["HOME", "PATH", "USER"];
    for var in &env_vars {
        if std::env::var(var).is_ok() {
            app = app.arg(
                Arg::new(var.to_lowercase())
                    .long(&format!("use-{}", var.to_lowercase()))
                    .help(&format!("使用环境变量 {}", var))
                    .action(clap::ArgAction::SetTrue)
            );
        }
    }

    // 基于配置文件动态添加选项
    if let Ok(config) = std::fs::read_to_string("config.json") {
        // 假设配置文件包含可用的选项
        app = app.arg(
            Arg::new("use-config")
                .long("use-config")
                .help("使用配置文件中的设置")
                .action(clap::ArgAction::SetTrue)
        );
    }

    let matches = app.get_matches();
    
    if matches.get_flag("debug") {
        println!("调试模式已启用");
    }
}
```

### 条件参数和组

```rust
use clap::{Arg, Command, ArgGroup, ArgAction};

fn conditional_arguments() {
    let app = Command::new("conditional-app")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("操作模式")
                .value_parser(["server", "client", "standalone"])
                .required(true)
        )
        // 服务器模式特有的参数
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("服务器端口")
                .value_parser(clap::value_parser!(u16))
                .required_if_eq("mode", "server")
        )
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("ADDRESS")
                .help("绑定地址")
                .default_value("0.0.0.0")
                .required_if_eq("mode", "server")
        )
        // 客户端模式特有的参数
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .value_name("HOST")
                .help("服务器地址")
                .required_if_eq("mode", "client")
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("连接超时")
                .value_parser(clap::value_parser!(u64))
                .default_value("30")
                .required_if_eq("mode", "client")
        )
        // 通用参数
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("详细输出")
                .action(ArgAction::Count)
        )
        // 互斥的日志选项
        .arg(
            Arg::new("log-file")
                .long("log-file")
                .value_name("FILE")
                .help("日志文件")
        )
        .arg(
            Arg::new("log-syslog")
                .long("log-syslog")
                .help("使用系统日志")
                .action(ArgAction::SetTrue)
        )
        .group(
            ArgGroup::new("logging")
                .args(["log-file", "log-syslog"])
                .required(false)
        );

    let matches = app.get_matches();
    
    let mode = matches.get_one::<String>("mode").unwrap();
    println!("运行模式: {}", mode);

    match mode.as_str() {
        "server" => {
            let port = matches.get_one::<u16>("port").unwrap();
            let bind = matches.get_one::<String>("bind").unwrap();
            println!("服务器模式: {}:{}", bind, port);
        }
        "client" => {
            let host = matches.get_one::<String>("host").unwrap();
            let timeout = matches.get_one::<u64>("timeout").unwrap();
            println!("客户端模式: 连接到 {}, 超时 {}s", host, timeout);
        }
        "standalone" => {
            println!("独立模式");
        }
        _ => unreachable!(),
    }

    let verbose = matches.get_count("verbose");
    if verbose > 0 {
        println!("详细级别: {}", verbose);
    }
}
```

## 高级特性

### 自定义帮助和版本

```rust
use clap::{Arg, Command, ArgAction};

fn custom_help_and_version() {
    let app = Command::new("advanced-help")
        .version("1.2.3")
        .about("一个具有自定义帮助的应用程序")
        .long_about(
            "这是一个更详细的应用程序描述。\n\
             它可以包含多行文本和格式化信息。\n\n\
             示例用法:\n\
             \t advanced-help --input file.txt --output result.txt\n\
             \t advanced-help process --parallel --threads 4"
        )
        .after_help(
            "更多信息:\n\
             \t 访问 https://example.com/docs 获取完整文档\n\
             \t 报告问题: https://github.com/user/repo/issues"
        )
        .before_help("在帮助信息之前显示的文本")
        .override_usage("advanced-help [OPTIONS] <COMMAND>")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("输入文件路径")
                .long_help(
                    "指定要处理的输入文件。\n\
                     支持的格式: JSON, YAML, TOML\n\
                     文件必须存在且可读。"
                )
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出文件路径")
                .long_help("指定处理结果的输出路径。如果文件存在将被覆盖。")
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("强制覆盖现有文件")
                .action(ArgAction::SetTrue)
        );

    // 自定义版本信息
    let app = app.mut_arg("version", |arg| {
        arg.help("显示版本信息并退出")
    });

    // 自定义帮助信息
    let app = app.mut_arg("help", |arg| {
        arg.help("显示帮助信息并退出")
    });

    let matches = app.get_matches();
    
    // 处理参数...
}
```

### 补全脚本生成

```rust
use clap::{Command, CommandFactory};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(clap::Parser)]
#[command(name = "myapp")]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,
    
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(clap::Subcommand)]
enum SubCommands {
    /// 生成shell补全脚本
    Completions {
        /// 目标shell
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn generate_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn completion_demo() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(SubCommands::Completions { shell }) => {
            let mut cmd = Cli::command();
            eprintln!("正在为 {:?} 生成补全脚本...", shell);
            generate_completions(shell, &mut cmd);
        }
        None => {
            // 正常的应用程序逻辑
            println!("运行正常逻辑");
        }
    }
}

// 使用示例:
// cargo run -- completions bash > completions.bash
// source completions.bash
```

### 配置文件集成

```rust
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Serialize, Deserialize)]
#[command(name = "configurable-app")]
struct Config {
    /// 配置文件路径
    #[arg(short, long)]
    #[serde(skip)]
    config: Option<PathBuf>,

    /// 服务器地址
    #[arg(short, long, default_value = "localhost")]
    host: String,

    /// 端口号
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// 数据库URL
    #[arg(long)]
    database_url: Option<String>,

    /// 工作线程数
    #[arg(short, long, default_value = "4")]
    workers: usize,

    /// 启用调试模式
    #[arg(long)]
    debug: bool,

    /// 日志级别
    #[arg(long, default_value = "info")]
    log_level: String,
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Config::parse();
        
        // 如果指定了配置文件，从文件加载配置
        if let Some(config_path) = &config.config {
            let file_content = std::fs::read_to_string(config_path)?;
            let file_config: Config = match config_path.extension().and_then(|s| s.to_str()) {
                Some("json") => serde_json::from_str(&file_content)?,
                Some("yaml") | Some("yml") => serde_yaml::from_str(&file_content)?,
                Some("toml") => toml::from_str(&file_content)?,
                _ => return Err("不支持的配置文件格式".into()),
            };
            
            // 合并配置：命令行参数优先于配置文件
            config = merge_config(file_config, config);
        }
        
        Ok(config)
    }
}

fn merge_config(file_config: Config, cli_config: Config) -> Config {
    Config {
        config: cli_config.config,
        host: if cli_config.host != "localhost" { cli_config.host } else { file_config.host },
        port: if cli_config.port != 8080 { cli_config.port } else { file_config.port },
        database_url: cli_config.database_url.or(file_config.database_url),
        workers: if cli_config.workers != 4 { cli_config.workers } else { file_config.workers },
        debug: cli_config.debug || file_config.debug,
        log_level: if cli_config.log_level != "info" { cli_config.log_level } else { file_config.log_level },
    }
}

fn config_integration_demo() {
    match Config::load() {
        Ok(config) => {
            println!("配置加载成功:");
            println!("  服务器: {}:{}", config.host, config.port);
            println!("  工作线程: {}", config.workers);
            println!("  调试模式: {}", config.debug);
            println!("  日志级别: {}", config.log_level);
            
            if let Some(db_url) = config.database_url {
                println!("  数据库: {}", db_url);
            }
        }
        Err(e) => {
            eprintln!("配置加载失败: {}", e);
            std::process::exit(1);
        }
    }
}
```

## 子命令系统

### 基本子命令

```rust
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "git-like")]
#[command(about = "类似Git的版本控制工具")]
struct Cli {
    /// 全局的详细输出选项
    #[arg(short, long, global = true)]
    verbose: bool,

    /// 工作目录
    #[arg(short = 'C', long, global = true)]
    directory: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化仓库
    Init {
        /// 仓库路径
        path: Option<String>,
        /// 裸仓库
        #[arg(long)]
        bare: bool,
    },
    /// 添加文件到暂存区
    Add {
        /// 要添加的文件
        files: Vec<String>,
        /// 添加所有文件
        #[arg(short, long)]
        all: bool,
    },
    /// 提交变更
    Commit {
        /// 提交信息
        #[arg(short, long)]
        message: String,
        /// 修改最后一次提交
        #[arg(long)]
        amend: bool,
        /// 签名提交
        #[arg(short = 'S', long)]
        gpg_sign: bool,
    },
    /// 推送到远程仓库
    Push {
        /// 远程仓库名称
        remote: Option<String>,
        /// 分支名称
        branch: Option<String>,
        /// 强制推送
        #[arg(short, long)]
        force: bool,
        /// 推送标签
        #[arg(long)]
        tags: bool,
    },
    /// 拉取远程变更
    Pull {
        /// 远程仓库名称
        remote: Option<String>,
        /// 分支名称
        branch: Option<String>,
        /// 变基而不是合并
        #[arg(long)]
        rebase: bool,
    },
    /// 分支管理
    Branch {
        #[command(subcommand)]
        action: BranchCommands,
    },
}

#[derive(Subcommand)]
enum BranchCommands {
    /// 列出分支
    List {
        /// 包括远程分支
        #[arg(short, long)]
        all: bool,
    },
    /// 创建新分支
    Create {
        /// 分支名称
        name: String,
        /// 基于的提交
        from: Option<String>,
    },
    /// 删除分支
    Delete {
        /// 分支名称
        name: String,
        /// 强制删除
        #[arg(short, long)]
        force: bool,
    },
}

fn subcommand_demo() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("详细模式已启用");
    }

    if let Some(dir) = cli.directory {
        println!("工作目录: {}", dir);
    }

    match cli.command {
        Commands::Init { path, bare } => {
            let repo_path = path.unwrap_or_else(|| ".".to_string());
            println!("在 {} 初始化仓库", repo_path);
            if bare {
                println!("创建裸仓库");
            }
        }
        Commands::Add { files, all } => {
            if all {
                println!("添加所有文件到暂存区");
            } else {
                println!("添加文件到暂存区: {:?}", files);
            }
        }
        Commands::Commit { message, amend, gpg_sign } => {
            println!("提交变更: {}", message);
            if amend {
                println!("修改上次提交");
            }
            if gpg_sign {
                println!("GPG签名提交");
            }
        }
        Commands::Push { remote, branch, force, tags } => {
            let remote = remote.unwrap_or_else(|| "origin".to_string());
            let branch = branch.unwrap_or_else(|| "main".to_string());
            println!("推送到 {}/{}", remote, branch);
            if force {
                println!("强制推送");
            }
            if tags {
                println!("推送标签");
            }
        }
        Commands::Pull { remote, branch, rebase } => {
            let remote = remote.unwrap_or_else(|| "origin".to_string());
            let branch = branch.unwrap_or_else(|| "main".to_string());
            println!("从 {}/{} 拉取", remote, branch);
            if rebase {
                println!("使用变基模式");
            }
        }
        Commands::Branch { action } => {
            match action {
                BranchCommands::List { all } => {
                    println!("列出分支");
                    if all {
                        println!("包括远程分支");
                    }
                }
                BranchCommands::Create { name, from } => {
                    println!("创建分支: {}", name);
                    if let Some(from_commit) = from {
                        println!("基于: {}", from_commit);
                    }
                }
                BranchCommands::Delete { name, force } => {
                    println!("删除分支: {}", name);
                    if force {
                        println!("强制删除");
                    }
                }
            }
        }
    }
}
```

### 插件式子命令

```rust
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "extensible-tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<MainCommands>,
}

#[derive(Subcommand)]
enum MainCommands {
    /// 内置的核心命令
    Core {
        #[command(subcommand)]
        action: CoreCommands,
    },
    /// 扩展命令 (通过外部程序实现)
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Subcommand)]
enum CoreCommands {
    /// 显示状态
    Status,
    /// 配置工具
    Config {
        /// 配置键
        key: String,
        /// 配置值
        value: Option<String>,
    },
}

fn plugin_system_demo() {
    let cli = Cli::parse();

    match cli.command {
        Some(MainCommands::Core { action }) => {
            match action {
                CoreCommands::Status => {
                    println!("显示工具状态");
                }
                CoreCommands::Config { key, value } => {
                    if let Some(value) = value {
                        println!("设置配置 {} = {}", key, value);
                    } else {
                        println!("获取配置 {}", key);
                    }
                }
            }
        }
        Some(MainCommands::External(args)) => {
            if let Some(subcommand) = args.first() {
                // 尝试执行外部子命令
                let program_name = format!("extensible-tool-{}", subcommand);
                let mut cmd = Command::new(&program_name);
                
                if args.len() > 1 {
                    cmd.args(&args[1..]);
                }
                
                match cmd.status() {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("外部命令执行失败");
                            std::process::exit(status.code().unwrap_or(1));
                        }
                    }
                    Err(_) => {
                        eprintln!("未知的子命令: {}", subcommand);
                        eprintln!("请确保 {} 在 PATH 中", program_name);
                        std::process::exit(1);
                    }
                }
            }
        }
        None => {
            println!("请指定一个子命令");
        }
    }
}
```

## 配置和环境变量

### 环境变量集成

```rust
use clap::Parser;
use std::env;

#[derive(Parser)]
#[command(name = "env-aware")]
struct Config {
    /// 服务器主机名
    #[arg(long, env = "SERVER_HOST", default_value = "localhost")]
    host: String,

    /// 服务器端口
    #[arg(long, env = "SERVER_PORT", default_value = "8080")]
    port: u16,

    /// 数据库URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,

    /// API密钥
    #[arg(long, env = "API_KEY")]
    api_key: Option<String>,

    /// 工作目录
    #[arg(long, env = "WORK_DIR")]
    work_dir: Option<String>,

    /// 日志级别
    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    log_level: String,

    /// 调试模式 (支持多种环境变量格式)
    #[arg(long, env = "DEBUG")]
    debug: bool,

    /// 最大连接数
    #[arg(long, env = "MAX_CONNECTIONS", default_value = "100")]
    max_connections: usize,
}

fn environment_integration() {
    // 可以通过环境变量或命令行参数配置
    let config = Config::parse();

    println!("配置信息:");
    println!("  主机: {}", config.host);
    println!("  端口: {}", config.port);
    
    if let Some(db_url) = config.database_url {
        println!("  数据库: {}", db_url);
    } else {
        println!("  数据库: 未配置");
    }
    
    if let Some(api_key) = config.api_key {
        println!("  API密钥: {}***", &api_key[..3.min(api_key.len())]);
    } else {
        println!("  API密钥: 未配置");
    }
    
    println!("  日志级别: {}", config.log_level);
    println!("  调试模式: {}", config.debug);
    println!("  最大连接数: {}", config.max_connections);
}

// 使用示例:
// export SERVER_HOST=production.example.com
// export SERVER_PORT=443
// export DEBUG=true
// cargo run
```

### 配置优先级

```rust
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser, Serialize, Deserialize, Clone)]
struct AppConfig {
    /// 配置文件路径
    #[arg(short, long)]
    #[serde(skip)]
    config_file: Option<String>,

    /// 服务器地址
    #[arg(long, env = "SERVER_HOST")]
    host: Option<String>,

    /// 端口号
    #[arg(long, env = "SERVER_PORT")]
    port: Option<u16>,

    /// 数据库URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,

    /// 工作线程数
    #[arg(long, env = "WORKERS")]
    workers: Option<usize>,

    /// 调试模式
    #[arg(long, env = "DEBUG")]
    debug: Option<bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            config_file: None,
            host: Some("localhost".to_string()),
            port: Some(8080),
            database_url: None,
            workers: Some(4),
            debug: Some(false),
        }
    }
}

impl AppConfig {
    fn resolve() -> Self {
        // 1. 默认配置
        let mut config = AppConfig::default();
        
        // 2. 从配置文件加载 (如果指定)
        let cli_args = AppConfig::parse();
        if let Some(config_path) = &cli_args.config_file {
            if let Ok(file_content) = std::fs::read_to_string(config_path) {
                if let Ok(file_config) = serde_json::from_str::<AppConfig>(&file_content) {
                    config = merge_configs(config, file_config);
                }
            }
        }
        
        // 3. 环境变量会在解析时自动应用
        
        // 4. 命令行参数具有最高优先级
        config = merge_configs(config, cli_args);
        
        config
    }
}

fn merge_configs(base: AppConfig, override_config: AppConfig) -> AppConfig {
    AppConfig {
        config_file: override_config.config_file.or(base.config_file),
        host: override_config.host.or(base.host),
        port: override_config.port.or(base.port),
        database_url: override_config.database_url.or(base.database_url),
        workers: override_config.workers.or(base.workers),
        debug: override_config.debug.or(base.debug),
    }
}

fn configuration_priority_demo() {
    let config = AppConfig::resolve();
    
    println!("最终配置 (按优先级: 命令行 > 环境变量 > 配置文件 > 默认值):");
    println!("  主机: {}", config.host.unwrap_or_default());
    println!("  端口: {}", config.port.unwrap_or_default());
    
    if let Some(db_url) = config.database_url {
        println!("  数据库: {}", db_url);
    }
    
    println!("  工作线程: {}", config.workers.unwrap_or_default());
    println!("  调试模式: {}", config.debug.unwrap_or_default());
}
```

## 错误处理

### 自定义错误信息

```rust
use clap::{Parser, error::ErrorKind};

#[derive(Parser)]
#[command(name = "error-demo")]
struct Args {
    /// 输入文件
    #[arg(short, long)]
    input: String,

    /// 输出文件
    #[arg(short, long)]
    output: String,

    /// 线程数 (1-16)
    #[arg(short, long, default_value = "4")]
    threads: usize,
}

fn validate_args(args: &Args) -> Result<(), String> {
    // 验证输入文件存在
    if !std::path::Path::new(&args.input).exists() {
        return Err(format!("输入文件不存在: {}", args.input));
    }

    // 验证输出文件的父目录存在
    if let Some(parent) = std::path::Path::new(&args.output).parent() {
        if !parent.exists() {
            return Err(format!("输出目录不存在: {}", parent.display()));
        }
    }

    // 验证线程数范围
    if args.threads == 0 || args.threads > 16 {
        return Err("线程数必须在 1-16 之间".to_string());
    }

    Ok(())
}

fn custom_error_handling() {
    let args = Args::parse();
    
    if let Err(error) = validate_args(&args) {
        eprintln!("错误: {}", error);
        std::process::exit(1);
    }
    
    println!("验证通过，开始处理...");
    println!("输入: {}", args.input);
    println!("输出: {}", args.output);
    println!("线程: {}", args.threads);
}
```

### 优雅的错误处理

```rust
use clap::Parser;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("配置错误: {message}")]
    Config { message: String },
    
    #[error("验证失败: {field} - {reason}")]
    Validation { field: String, reason: String },
    
    #[error("网络错误: {0}")]
    Network(String),
}

#[derive(Parser)]
struct SafeArgs {
    /// 配置文件
    #[arg(short, long)]
    config: Option<String>,

    /// 输入文件
    #[arg(short, long)]
    input: String,

    /// 输出文件
    #[arg(short, long)]
    output: String,
}

impl SafeArgs {
    fn validate(&self) -> Result<(), AppError> {
        // 验证输入文件
        if !std::path::Path::new(&self.input).exists() {
            return Err(AppError::Validation {
                field: "input".to_string(),
                reason: format!("文件不存在: {}", self.input),
            });
        }

        // 验证配置文件
        if let Some(config_path) = &self.config {
            if !std::path::Path::new(config_path).exists() {
                return Err(AppError::Config {
                    message: format!("配置文件不存在: {}", config_path),
                });
            }
        }

        // 验证输出目录
        if let Some(parent) = std::path::Path::new(&self.output).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(())
    }
}

fn graceful_error_handling() -> Result<(), AppError> {
    let args = SafeArgs::parse();
    
    // 验证参数
    args.validate()?;
    
    // 如果有配置文件，加载它
    if let Some(config_path) = &args.config {
        let _config_content = std::fs::read_to_string(config_path)?;
        println!("加载配置文件: {}", config_path);
    }
    
    println!("处理文件: {} -> {}", args.input, args.output);
    
    // 模拟可能失败的操作
    if args.input.contains("fail") {
        return Err(AppError::Network("模拟网络错误".to_string()));
    }
    
    Ok(())
}

fn error_handling_demo() {
    if let Err(error) = graceful_error_handling() {
        eprintln!("应用程序错误: {}", error);
        
        // 根据错误类型提供不同的退出码
        let exit_code = match error {
            AppError::Io(_) => 2,
            AppError::Config { .. } => 3,
            AppError::Validation { .. } => 4,
            AppError::Network(_) => 5,
        };
        
        std::process::exit(exit_code);
    }
}
```

## 实战案例

### 文件处理工具

```rust
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "file-tool")]
#[command(about = "文件处理工具集")]
#[command(version = "1.0.0")]
struct FileTool {
    /// 全局详细输出
    #[arg(short, long, global = true)]
    verbose: bool,

    /// 配置文件
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: FileCommands,
}

#[derive(Subcommand)]
enum FileCommands {
    /// 复制文件或目录
    Copy {
        /// 源路径
        source: PathBuf,
        /// 目标路径
        destination: PathBuf,
        /// 递归复制目录
        #[arg(short, long)]
        recursive: bool,
        /// 覆盖现有文件
        #[arg(short, long)]
        force: bool,
        /// 保持权限和时间戳
        #[arg(short, long)]
        preserve: bool,
    },
    /// 移动文件或目录
    Move {
        /// 源路径
        source: PathBuf,
        /// 目标路径
        destination: PathBuf,
        /// 覆盖现有文件
        #[arg(short, long)]
        force: bool,
    },
    /// 删除文件或目录
    Remove {
        /// 要删除的路径
        paths: Vec<PathBuf>,
        /// 递归删除目录
        #[arg(short, long)]
        recursive: bool,
        /// 强制删除，忽略权限
        #[arg(short, long)]
        force: bool,
        /// 交互模式，删除前确认
        #[arg(short, long)]
        interactive: bool,
    },
    /// 列出目录内容
    List {
        /// 要列出的路径
        path: Option<PathBuf>,
        /// 显示详细信息
        #[arg(short, long)]
        long: bool,
        /// 显示隐藏文件
        #[arg(short = 'a', long)]
        all: bool,
        /// 递归列出子目录
        #[arg(short, long)]
        recursive: bool,
        /// 排序方式
        #[arg(short, long, value_enum, default_value = "name")]
        sort: SortBy,
    },
    /// 查找文件
    Find {
        /// 搜索路径
        #[arg(default_value = ".")]
        path: PathBuf,
        /// 文件名模式
        #[arg(short, long)]
        name: Option<String>,
        /// 文件类型
        #[arg(short, long, value_enum)]
        file_type: Option<FileType>,
        /// 文件大小过滤
        #[arg(short, long)]
        size: Option<String>,
        /// 最大搜索深度
        #[arg(long)]
        max_depth: Option<usize>,
    },
    /// 压缩文件或目录
    Archive {
        /// 归档文件名
        archive: PathBuf,
        /// 要压缩的文件/目录
        sources: Vec<PathBuf>,
        /// 压缩格式
        #[arg(short, long, value_enum, default_value = "zip")]
        format: ArchiveFormat,
        /// 压缩级别 (1-9)
        #[arg(short, long, default_value = "6")]
        level: u8,
    },
    /// 解压文件
    Extract {
        /// 归档文件
        archive: PathBuf,
        /// 解压目标目录
        #[arg(short, long)]
        destination: Option<PathBuf>,
        /// 覆盖现有文件
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(ValueEnum, Clone)]
enum SortBy {
    Name,
    Size,
    Date,
    Type,
}

#[derive(ValueEnum, Clone)]
enum FileType {
    File,
    Directory,
    Symlink,
}

#[derive(ValueEnum, Clone)]
enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    TarBz2,
}

fn file_tool_demo() {
    let tool = FileTool::parse();

    if tool.verbose {
        println!("详细模式已启用");
    }

    if let Some(config) = tool.config {
        println!("使用配置文件: {}", config.display());
    }

    match tool.command {
        FileCommands::Copy { source, destination, recursive, force, preserve } => {
            println!("复制 {} 到 {}", source.display(), destination.display());
            if recursive {
                println!("递归复制目录");
            }
            if force {
                println!("强制覆盖现有文件");
            }
            if preserve {
                println!("保持权限和时间戳");
            }
        }
        FileCommands::Move { source, destination, force } => {
            println!("移动 {} 到 {}", source.display(), destination.display());
            if force {
                println!("强制覆盖现有文件");
            }
        }
        FileCommands::Remove { paths, recursive, force, interactive } => {
            println!("删除文件: {:?}", paths);
            if recursive {
                println!("递归删除目录");
            }
            if force {
                println!("强制删除");
            }
            if interactive {
                println!("交互模式");
            }
        }
        FileCommands::List { path, long, all, recursive, sort } => {
            let list_path = path.unwrap_or_else(|| PathBuf::from("."));
            println!("列出目录: {}", list_path.display());
            if long {
                println!("详细信息");
            }
            if all {
                println!("包括隐藏文件");
            }
            if recursive {
                println!("递归列出");
            }
            println!("排序方式: {:?}", sort);
        }
        FileCommands::Find { path, name, file_type, size, max_depth } => {
            println!("在 {} 中查找文件", path.display());
            if let Some(name_pattern) = name {
                println!("文件名模式: {}", name_pattern);
            }
            if let Some(ftype) = file_type {
                println!("文件类型: {:?}", ftype);
            }
            if let Some(size_filter) = size {
                println!("大小过滤: {}", size_filter);
            }
            if let Some(depth) = max_depth {
                println!("最大深度: {}", depth);
            }
        }
        FileCommands::Archive { archive, sources, format, level } => {
            println!("创建归档: {}", archive.display());
            println!("源文件: {:?}", sources);
            println!("格式: {:?}", format);
            println!("压缩级别: {}", level);
        }
        FileCommands::Extract { archive, destination, force } => {
            println!("解压归档: {}", archive.display());
            if let Some(dest) = destination {
                println!("解压到: {}", dest.display());
            }
            if force {
                println!("强制覆盖");
            }
        }
    }
}
```

### Web服务器CLI

```rust
use clap::{Parser, Subcommand, ValueEnum};
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "webserver")]
#[command(about = "简单的Web服务器")]
#[command(version = "2.0.0")]
struct WebServer {
    /// 全局配置文件
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// 全局日志级别
    #[arg(long, global = true, value_enum, default_value = "info")]
    log_level: LogLevel,

    #[command(subcommand)]
    command: ServerCommands,
}

#[derive(Subcommand)]
enum ServerCommands {
    /// 启动服务器
    Start {
        /// 绑定地址
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: IpAddr,

        /// 监听端口
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// 静态文件目录
        #[arg(short, long, default_value = ".")]
        static_dir: PathBuf,

        /// 工作线程数
        #[arg(short, long)]
        workers: Option<usize>,

        /// 启用HTTPS
        #[arg(long)]
        tls: bool,

        /// TLS证书文件
        #[arg(long, requires = "tls")]
        cert: Option<PathBuf>,

        /// TLS私钥文件
        #[arg(long, requires = "tls")]
        key: Option<PathBuf>,

        /// 启用热重载
        #[arg(long)]
        reload: bool,

        /// 启用压缩
        #[arg(long)]
        compress: bool,

        /// 请求超时时间 (秒)
        #[arg(long, default_value = "30")]
        timeout: u64,

        /// 最大请求大小 (MB)
        #[arg(long, default_value = "10")]
        max_request_size: u64,
    },
    /// 停止服务器
    Stop {
        /// PID文件路径
        #[arg(short, long, default_value = "webserver.pid")]
        pid_file: PathBuf,

        /// 强制停止
        #[arg(short, long)]
        force: bool,
    },
    /// 重启服务器
    Restart {
        /// PID文件路径
        #[arg(short, long, default_value = "webserver.pid")]
        pid_file: PathBuf,

        /// 重启时的等待时间 (秒)
        #[arg(long, default_value = "5")]
        wait: u64,
    },
    /// 检查服务器状态
    Status {
        /// PID文件路径
        #[arg(short, long, default_value = "webserver.pid")]
        pid_file: PathBuf,

        /// 输出格式
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },
    /// 生成配置文件模板
    Config {
        /// 输出配置文件路径
        #[arg(short, long, default_value = "webserver.toml")]
        output: PathBuf,

        /// 配置类型
        #[arg(short, long, value_enum, default_value = "development")]
        template: ConfigTemplate,

        /// 覆盖现有文件
        #[arg(long)]
        force: bool,
    },
    /// 验证配置文件
    Validate {
        /// 配置文件路径
        config: PathBuf,

        /// 详细输出验证结果
        #[arg(short, long)]
        verbose: bool,
    },
    /// 显示服务器日志
    Logs {
        /// 日志文件路径
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// 显示最后N行
        #[arg(short, long, default_value = "100")]
        tail: usize,

        /// 跟踪日志文件
        #[arg(short = 'f', long)]
        follow: bool,

        /// 过滤日志级别
        #[arg(long, value_enum)]
        level: Option<LogLevel>,
    },
}

#[derive(ValueEnum, Clone)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Human,
    Json,
    Yaml,
}

#[derive(ValueEnum, Clone)]
enum ConfigTemplate {
    Development,
    Production,
    Testing,
}

fn webserver_demo() {
    let server = WebServer::parse();

    println!("日志级别: {:?}", server.log_level);

    if let Some(config) = server.config {
        println!("使用配置文件: {}", config.display());
    }

    match server.command {
        ServerCommands::Start {
            host,
            port,
            static_dir,
            workers,
            tls,
            cert,
            key,
            reload,
            compress,
            timeout,
            max_request_size,
        } => {
            println!("启动Web服务器");
            println!("监听地址: {}:{}", host, port);
            println!("静态文件目录: {}", static_dir.display());

            if let Some(worker_count) = workers {
                println!("工作线程数: {}", worker_count);
            } else {
                println!("工作线程数: 自动检测");
            }

            if tls {
                println!("启用HTTPS");
                if let (Some(cert_file), Some(key_file)) = (cert, key) {
                    println!("证书文件: {}", cert_file.display());
                    println!("私钥文件: {}", key_file.display());
                }
            }

            if reload {
                println!("启用热重载");
            }

            if compress {
                println!("启用压缩");
            }

            println!("请求超时: {}秒", timeout);
            println!("最大请求大小: {}MB", max_request_size);
        }
        ServerCommands::Stop { pid_file, force } => {
            println!("停止服务器");
            println!("PID文件: {}", pid_file.display());
            if force {
                println!("强制停止");
            }
        }
        ServerCommands::Restart { pid_file, wait } => {
            println!("重启服务器");
            println!("PID文件: {}", pid_file.display());
            println!("等待时间: {}秒", wait);
        }
        ServerCommands::Status { pid_file, format } => {
            println!("检查服务器状态");
            println!("PID文件: {}", pid_file.display());
            println!("输出格式: {:?}", format);
        }
        ServerCommands::Config { output, template, force } => {
            println!("生成配置文件模板");
            println!("输出文件: {}", output.display());
            println!("模板类型: {:?}", template);
            if force {
                println!("覆盖现有文件");
            }
        }
        ServerCommands::Validate { config, verbose } => {
            println!("验证配置文件: {}", config.display());
            if verbose {
                println!("详细输出验证结果");
            }
        }
        ServerCommands::Logs { file, tail, follow, level } => {
            if let Some(log_file) = file {
                println!("日志文件: {}", log_file.display());
            } else {
                println!("使用默认日志文件");
            }
            
            println!("显示最后 {} 行", tail);
            
            if follow {
                println!("跟踪日志文件");
            }
            
            if let Some(filter_level) = level {
                println!("过滤级别: {:?}", filter_level);
            }
        }
    }
}
```

## 最佳实践

### 1. API选择指南

```rust
use clap::{Parser, Command, Arg};

// 简单应用使用派生宏
#[derive(Parser)]
struct SimpleApp {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    verbose: bool,
}

// 复杂应用使用构建器API
fn complex_app() -> Command {
    Command::new("complex")
        .version("1.0")
        .about("复杂应用示例")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("输入文件")
                .required(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("详细输出")
                .action(clap::ArgAction::SetTrue)
        )
}

fn api_selection_guide() {
    // 推荐：简单CLI使用派生宏
    let simple = SimpleApp::parse();
    println!("简单应用 - 输入: {}, 详细: {}", simple.input, simple.verbose);
    
    // 推荐：复杂或动态CLI使用构建器
    let complex = complex_app().get_matches();
    println!("复杂应用 - 输入: {}", complex.get_one::<String>("input").unwrap());
}
```

### 2. 错误信息优化

```rust
use clap::{Parser, ArgAction, error::ErrorKind};

#[derive(Parser)]
#[command(name = "optimized")]
#[command(about = "优化的错误信息示例")]
#[command(long_about = "这个应用展示了如何提供清晰的错误信息和帮助文本")]
struct OptimizedApp {
    /// 输入文件路径 (必须存在且可读)
    #[arg(short, long, help = "输入文件路径")]
    #[arg(long_help = "指定要处理的输入文件。\n文件必须存在且当前用户有读取权限。\n支持的格式: .txt, .csv, .json")]
    input: String,

    /// 输出目录 (将自动创建如果不存在)
    #[arg(short, long, help = "输出目录")]
    #[arg(long_help = "指定输出文件的目录。\n如果目录不存在，将自动创建。\n确保有写入权限。")]
    output: String,

    /// 处理模式: fast(快速) 或 careful(仔细)
    #[arg(short, long, default_value = "fast")]
    #[arg(help = "处理模式")]
    #[arg(long_help = "选择处理模式:\n  fast    - 快速处理，可能跳过一些验证\n  careful - 仔细处理，包含完整验证")]
    mode: String,

    /// 详细输出级别 (可重复使用增加详细度)
    #[arg(short, long, action = ArgAction::Count)]
    #[arg(help = "增加输出详细度")]
    #[arg(long_help = "控制输出详细程度:\n  -v    基本调试信息\n  -vv   详细调试信息\n  -vvv  跟踪级别信息")]
    verbose: u8,
}

fn error_optimization_demo() {
    let app = OptimizedApp::parse();
    
    // 自定义验证逻辑
    if !std::path::Path::new(&app.input).exists() {
        eprintln!("错误: 输入文件不存在: {}", app.input);
        eprintln!("提示: 请检查文件路径是否正确");
        std::process::exit(1);
    }
    
    if !["fast", "careful"].contains(&app.mode.as_str()) {
        eprintln!("错误: 无效的处理模式: {}", app.mode);
        eprintln!("提示: 支持的模式有 'fast' 和 'careful'");
        std::process::exit(1);
    }
    
    println!("配置验证通过");
}
```

### 3. 性能优化

```rust
use clap::{Parser, Command, value_parser};

#[derive(Parser)]
struct PerformantApp {
    /// 使用value_parser进行类型转换
    #[arg(short, long, value_parser = value_parser!(u32).range(1..=100))]
    threads: u32,

    /// 预定义的选择值
    #[arg(short, long, value_parser = ["json", "yaml", "toml"])]
    format: String,

    /// 文件列表
    files: Vec<String>,
}

fn performance_optimization() {
    // 1. 使用静态字符串避免分配
    const ABOUT: &str = "高性能CLI应用示例";
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    let app = PerformantApp::parse();
    
    // 2. 避免不必要的字符串分配
    match app.format.as_str() {
        "json" => println!("使用JSON格式"),
        "yaml" => println!("使用YAML格式"),
        "toml" => println!("使用TOML格式"),
        _ => unreachable!(), // value_parser确保不会到达这里
    }
    
    // 3. 高效处理文件列表
    if !app.files.is_empty() {
        println!("处理 {} 个文件", app.files.len());
        for (i, file) in app.files.iter().enumerate() {
            println!("  {}: {}", i + 1, file);
        }
    }
}
```

### 4. 测试策略

```rust
use clap::{Parser, Command};

#[derive(Parser)]
#[command(name = "testable-app")]
struct TestableApp {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    verbose: bool,
}

impl TestableApp {
    // 提供用于测试的构造函数
    #[cfg(test)]
    fn new_for_test(input: String, verbose: bool) -> Self {
        TestableApp { input, verbose }
    }
    
    // 将CLI逻辑分离到可测试的函数中
    fn run(&self) -> Result<String, String> {
        if self.input.is_empty() {
            return Err("输入不能为空".to_string());
        }
        
        let result = format!("处理文件: {}", self.input);
        
        if self.verbose {
            Ok(format!("{} (详细模式)", result))
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        let app = TestableApp::new_for_test("test.txt".to_string(), false);
        let result = app.run().unwrap();
        assert_eq!(result, "处理文件: test.txt");
    }
    
    #[test]
    fn test_verbose_mode() {
        let app = TestableApp::new_for_test("test.txt".to_string(), true);
        let result = app.run().unwrap();
        assert_eq!(result, "处理文件: test.txt (详细模式)");
    }
    
    #[test]
    fn test_empty_input() {
        let app = TestableApp::new_for_test("".to_string(), false);
        let result = app.run();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "输入不能为空");
    }
    
    #[test]
    fn test_command_parsing() {
        // 测试命令行解析
        let cmd = TestableApp::command();
        let matches = cmd.try_get_matches_from(vec!["test", "-i", "input.txt", "-v"]);
        assert!(matches.is_ok());
    }
}

fn testing_demo() {
    let app = TestableApp::parse();
    match app.run() {
        Ok(result) => println!("{}", result),
        Err(error) => {
            eprintln!("错误: {}", error);
            std::process::exit(1);
        }
    }
}
```

## 总结

Clap 是 Rust 中最强大和灵活的命令行参数解析库。通过本教程，您应该能够：

1. 理解 clap 的核心概念和两种API风格
2. 构建从简单到复杂的CLI应用程序
3. 实现子命令系统和插件架构
4. 集成配置文件和环境变量
5. 优化性能和提供良好的用户体验

关键要点：
- 派生宏API适合简单和中等复杂度的应用
- 构建器API提供最大的灵活性和控制
- 合理的错误处理和帮助信息改善用户体验
- 类型安全的参数解析避免运行时错误
- 测试友好的设计确保代码质量

Clap 的强大功能使其成为构建专业级CLI工具的首选库，掌握它将大大提升您的 Rust 命令行应用开发能力。