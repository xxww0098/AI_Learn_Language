# Log 0.4.27 - Rust 轻量级日志门面完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [日志级别](#日志级别)
- [日志宏](#日志宏)
- [日志实现](#日志实现)
- [高级特性](#高级特性)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)
- [集成指南](#集成指南)

## 概述

Log 是 Rust 生态系统中最重要的日志门面库，提供了一个统一的日志接口，允许应用程序和库与具体的日志实现解耦。

### 核心特性
- **轻量级门面**: 定义标准日志接口，不包含具体实现
- **灵活配置**: 支持不同的日志级别和过滤
- **高性能**: 编译时优化，运行时开销最小
- **可扩展**: 支持自定义日志格式和输出目标
- **生态友好**: 与众多日志实现库兼容

### 版本信息
- **当前版本**: 0.4.27
- **发布时间**: 2025-03-24
- **下载次数**: 488,351,287+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
log = "0.4.27"
# 选择一个日志实现
env_logger = "0.10"  # 简单的环境变量控制日志
```

### 基本示例

```rust
use log::{error, warn, info, debug, trace};

fn main() {
    // 初始化日志实现
    env_logger::init();
    
    // 使用不同级别的日志
    error!("这是错误信息");
    warn!("这是警告信息");
    info!("这是信息");
    debug!("这是调试信息");
    trace!("这是跟踪信息");
    
    // 带格式化的日志
    let user_id = 123;
    let action = "login";
    info!("用户 {} 执行了 {} 操作", user_id, action);
    
    // 条件日志
    if log::log_enabled!(log::Level::Debug) {
        debug!("调试已启用");
    }
}
```

### 环境变量配置

```bash
# 设置日志级别
export RUST_LOG=info

# 设置特定模块的日志级别
export RUST_LOG=my_app=debug,my_lib=warn

# 设置多个模块的日志级别
export RUST_LOG=info,my_app::module=debug
```

## 日志级别

### 级别定义

```rust
use log::Level;

// 五个标准日志级别（从高到低）
let levels = vec![
    Level::Error,   // 错误级别
    Level::Warn,    // 警告级别
    Level::Info,    // 信息级别
    Level::Debug,   // 调试级别
    Level::Trace,   // 跟踪级别
];

// 级别比较
assert!(Level::Error > Level::Warn);
assert!(Level::Info < Level::Debug);

// 级别转换
let level_str = "info";
let level: Level = level_str.parse().unwrap();
println!("解析的级别: {}", level);
```

### 级别过滤

```rust
use log::{Level, LevelFilter};

// LevelFilter 用于配置日志过滤
let filters = vec![
    LevelFilter::Off,     // 关闭所有日志
    LevelFilter::Error,   // 只显示错误
    LevelFilter::Warn,    // 显示警告及以上
    LevelFilter::Info,    // 显示信息及以上
    LevelFilter::Debug,   // 显示调试及以上
    LevelFilter::Trace,   // 显示所有级别
];

// 检查级别是否启用
fn check_level_enabled() {
    if log::log_enabled!(Level::Debug) {
        println!("调试级别已启用");
    }
    
    if log::log_enabled!(target: "my_module", Level::Info) {
        println!("my_module 的信息级别已启用");
    }
}
```

## 日志宏

### 基本宏

```rust
use log::{error, warn, info, debug, trace};

fn basic_logging() {
    // 基本宏使用
    error!("系统错误");
    warn!("警告信息");
    info!("一般信息");
    debug!("调试信息");
    trace!("跟踪信息");
    
    // 格式化输出
    let user = "张三";
    let count = 42;
    info!("用户 {} 有 {} 个消息", user, count);
    
    // 使用命名参数
    info!("用户 {user} 有 {count} 个消息", user = user, count = count);
}
```

### 目标和模块

```rust
use log::{error, warn, info, debug, trace};

fn targeted_logging() {
    // 指定目标
    info!(target: "my_app::auth", "用户登录成功");
    error!(target: "my_app::db", "数据库连接失败");
    
    // 使用模块路径作为目标
    info!(target: module_path!(), "模块信息");
    
    // 在不同模块中
    fn auth_function() {
        info!("认证模块信息");  // 目标自动为 my_app::auth
    }
    
    fn db_function() {
        error!("数据库模块错误");  // 目标自动为 my_app::db
    }
}
```

### 条件日志

```rust
use log::{log_enabled, Level};

fn conditional_logging() {
    // 检查级别是否启用
    if log_enabled!(Level::Debug) {
        let expensive_data = compute_expensive_debug_info();
        debug!("调试信息: {}", expensive_data);
    }
    
    // 检查特定目标的级别
    if log_enabled!(target: "performance", Level::Trace) {
        trace!(target: "performance", "性能跟踪信息");
    }
}

fn compute_expensive_debug_info() -> String {
    // 模拟昂贵的计算
    "昂贵的调试信息".to_string()
}
```

### 结构化日志

```rust
use log::info;

fn structured_logging() {
    // 键值对日志
    info!("用户操作"; "user_id" => 123, "action" => "login", "ip" => "192.168.1.1");
    
    // 混合格式
    info!("用户 {} 登录"; "user_id" => 123, "session_id" => "abc123");
    
    // 使用 serde 序列化
    #[derive(serde::Serialize)]
    struct UserAction {
        user_id: u64,
        action: String,
        timestamp: u64,
    }
    
    let action = UserAction {
        user_id: 123,
        action: "login".to_string(),
        timestamp: 1234567890,
    };
    
    info!("用户操作: {}", serde_json::to_string(&action).unwrap());
}
```

## 日志实现

### env_logger

```rust
use log::{error, warn, info, debug, trace};

fn env_logger_example() {
    // 简单初始化
    env_logger::init();
    
    // 或者使用构建器
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();
    
    // 自定义格式
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "[{}] {} - {}",
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .init();
    
    info!("env_logger 已初始化");
}
```

### simple_logger

```rust
use log::{error, warn, info, debug, trace};

fn simple_logger_example() {
    // 简单初始化
    simple_logger::init().unwrap();
    
    // 带级别初始化
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    
    // 使用构建器
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_path(false)
        .with_threads(false)
        .init()
        .unwrap();
    
    info!("simple_logger 已初始化");
}
```

### fern

```rust
use log::{error, warn, info, debug, trace};

fn fern_example() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
    
    info!("fern 已初始化");
}
```

### slog 桥接

```rust
use log::{error, warn, info, debug, trace};

fn slog_bridge_example() {
    // 创建 slog logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    
    // 设置为全局 logger
    slog_stdlog::init_with_level(logger, log::Level::Info).unwrap();
    
    info!("slog 桥接已初始化");
}
```

## 高级特性

### 自定义日志实现

```rust
use log::{Log, Record, Level, Metadata, SetLoggerError};

struct CustomLogger {
    level: Level,
}

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {} - {}", 
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            );
        }
    }
    
    fn flush(&self) {
        // 刷新日志缓冲区
    }
}

fn init_custom_logger() -> Result<(), SetLoggerError> {
    let logger = CustomLogger { level: Level::Info };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}
```

### 多输出目标

```rust
use log::{Log, Record, Level, Metadata};
use std::fs::OpenOptions;
use std::io::Write;

struct MultiTargetLogger {
    console_level: Level,
    file_level: Level,
    file_path: String,
}

impl Log for MultiTargetLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.console_level || metadata.level() <= self.file_level
    }
    
    fn log(&self, record: &Record) {
        let formatted = format!(
            "[{}] {} - {}\n",
            record.level(),
            record.module_path().unwrap_or("unknown"),
            record.args()
        );
        
        // 输出到控制台
        if record.level() <= self.console_level {
            print!("{}", formatted);
        }
        
        // 输出到文件
        if record.level() <= self.file_level {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.file_path)
            {
                let _ = file.write_all(formatted.as_bytes());
            }
        }
    }
    
    fn flush(&self) {
        use std::io::{stdout, Write};
        let _ = stdout().flush();
    }
}
```

### 过滤器

```rust
use log::{Log, Record, Level, Metadata};

struct FilteredLogger {
    inner: Box<dyn Log>,
    allowed_targets: Vec<String>,
}

impl Log for FilteredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata) && 
        self.allowed_targets.iter().any(|target| metadata.target().starts_with(target))
    }
    
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.inner.log(record);
        }
    }
    
    fn flush(&self) {
        self.inner.flush();
    }
}

fn create_filtered_logger() {
    let base_logger = Box::new(simple_logger::SimpleLogger::new());
    let filtered_logger = FilteredLogger {
        inner: base_logger,
        allowed_targets: vec!["my_app".to_string(), "important_lib".to_string()],
    };
    
    log::set_boxed_logger(Box::new(filtered_logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
```

## 性能优化

### 编译时优化

```rust
// 在 Cargo.toml 中设置
// [features]
// max_level_debug = ["log/max_level_debug"]
// release_max_level_warn = ["log/release_max_level_warn"]

fn performance_logging() {
    // 使用 log_enabled! 避免不必要的字符串格式化
    if log::log_enabled!(log::Level::Debug) {
        let expensive_data = compute_expensive_data();
        debug!("调试数据: {}", expensive_data);
    }
    
    // 使用闭包延迟计算
    debug!("调试数据: {}", || compute_expensive_data());
}

fn compute_expensive_data() -> String {
    // 模拟昂贵的计算
    "昂贵的数据".to_string()
}
```

### 内存优化

```rust
use log::{error, warn, info, debug, trace};

fn memory_efficient_logging() {
    // 避免不必要的字符串分配
    let user_id = 123;
    
    // 好的做法：直接使用引用
    info!("用户 {} 登录", user_id);
    
    // 避免的做法：创建不必要的字符串
    // info!("用户 {} 登录", user_id.to_string());
    
    // 使用 format_args! 避免中间字符串
    info!("{}", format_args!("用户 {} 执行了操作", user_id));
}
```

### 异步日志

```rust
use log::{error, warn, info, debug, trace};
use std::sync::mpsc;
use std::thread;

struct AsyncLogger {
    sender: mpsc::Sender<String>,
}

impl AsyncLogger {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                println!("{}", message);
            }
        });
        
        AsyncLogger { sender }
    }
}

impl log::Log for AsyncLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }
    
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "[{}] {} - {}",
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            );
            
            // 异步发送，不阻塞调用者
            let _ = self.sender.send(message);
        }
    }
    
    fn flush(&self) {
        // 异步 flush 实现
    }
}
```

## 实战案例

### Web 服务器日志

```rust
use log::{error, warn, info, debug, trace};
use std::time::Instant;

struct RequestLogger {
    start_time: Instant,
    method: String,
    path: String,
    user_agent: Option<String>,
}

impl RequestLogger {
    fn new(method: &str, path: &str, user_agent: Option<&str>) -> Self {
        info!("开始处理请求: {} {}", method, path);
        
        Self {
            start_time: Instant::now(),
            method: method.to_string(),
            path: path.to_string(),
            user_agent: user_agent.map(|s| s.to_string()),
        }
    }
    
    fn log_response(&self, status: u16, response_size: usize) {
        let duration = self.start_time.elapsed();
        
        info!(
            "请求完成: {} {} - 状态: {}, 大小: {} bytes, 耗时: {:?}",
            self.method, self.path, status, response_size, duration
        );
        
        // 记录慢请求
        if duration.as_millis() > 1000 {
            warn!(
                "慢请求: {} {} - 耗时: {:?}",
                self.method, self.path, duration
            );
        }
    }
    
    fn log_error(&self, error: &str) {
        error!(
            "请求错误: {} {} - 错误: {}",
            self.method, self.path, error
        );
    }
}

// 使用示例
fn handle_request() {
    let logger = RequestLogger::new("GET", "/api/users", Some("Mozilla/5.0"));
    
    // 模拟请求处理
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    logger.log_response(200, 1024);
}
```

### 数据库连接日志

```rust
use log::{error, warn, info, debug, trace};
use std::time::Instant;

struct DatabaseLogger;

impl DatabaseLogger {
    fn log_connection_attempt(&self, host: &str, database: &str) {
        info!("尝试连接数据库: {}@{}", database, host);
    }
    
    fn log_connection_success(&self, host: &str, database: &str, duration: std::time::Duration) {
        info!("数据库连接成功: {}@{} (耗时: {:?})", database, host, duration);
    }
    
    fn log_connection_failure(&self, host: &str, database: &str, error: &str) {
        error!("数据库连接失败: {}@{} - 错误: {}", database, host, error);
    }
    
    fn log_query(&self, query: &str, params: &[&str]) {
        debug!("执行查询: {} - 参数: {:?}", query, params);
    }
    
    fn log_slow_query(&self, query: &str, duration: std::time::Duration) {
        warn!("慢查询: {} (耗时: {:?})", query, duration);
    }
}

// 使用示例
fn database_operations() {
    let logger = DatabaseLogger;
    
    let start = Instant::now();
    logger.log_connection_attempt("localhost", "myapp");
    
    // 模拟连接
    std::thread::sleep(std::time::Duration::from_millis(50));
    logger.log_connection_success("localhost", "myapp", start.elapsed());
    
    // 模拟查询
    logger.log_query("SELECT * FROM users WHERE id = ?", &["123"]);
    
    // 模拟慢查询
    let query_start = Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    logger.log_slow_query("SELECT * FROM large_table", query_start.elapsed());
}
```

### 应用程序生命周期日志

```rust
use log::{error, warn, info, debug, trace};

struct ApplicationLogger;

impl ApplicationLogger {
    fn log_startup(&self, version: &str) {
        info!("应用程序启动 - 版本: {}", version);
    }
    
    fn log_config_loaded(&self, config_path: &str) {
        info!("配置文件加载成功: {}", config_path);
    }
    
    fn log_service_started(&self, service_name: &str, port: u16) {
        info!("服务启动: {} on port {}", service_name, port);
    }
    
    fn log_health_check(&self, service: &str, status: &str) {
        match status {
            "healthy" => debug!("健康检查: {} - {}", service, status),
            "unhealthy" => error!("健康检查: {} - {}", service, status),
            _ => warn!("健康检查: {} - {}", service, status),
        }
    }
    
    fn log_shutdown(&self, reason: &str) {
        info!("应用程序关闭 - 原因: {}", reason);
    }
}

// 使用示例
fn application_lifecycle() {
    let logger = ApplicationLogger;
    
    logger.log_startup("1.0.0");
    logger.log_config_loaded("/etc/myapp/config.toml");
    logger.log_service_started("HTTP Server", 8080);
    
    // 模拟健康检查
    logger.log_health_check("database", "healthy");
    logger.log_health_check("cache", "unhealthy");
    
    logger.log_shutdown("user request");
}
```

## 最佳实践

### 1. 选择合适的日志级别

```rust
use log::{error, warn, info, debug, trace};

fn proper_log_levels() {
    // ERROR - 严重错误，需要立即处理
    error!("数据库连接失败，无法启动应用");
    
    // WARN - 警告，可能导致问题
    warn!("磁盘空间不足，剩余: {}MB", 100);
    
    // INFO - 一般信息，程序正常运行
    info!("用户 {} 登录成功", "john");
    
    // DEBUG - 调试信息，开发时使用
    debug!("处理请求: method={}, path={}", "GET", "/api/users");
    
    // TRACE - 详细跟踪，非常详细的执行路径
    trace!("进入函数: process_request");
}
```

### 2. 使用结构化日志

```rust
use log::info;

fn structured_logging_best_practice() {
    // 使用键值对
    info!("用户操作"; 
        "user_id" => 123,
        "action" => "login",
        "ip" => "192.168.1.1",
        "timestamp" => chrono::Utc::now().timestamp()
    );
    
    // 使用 JSON 格式
    let event = serde_json::json!({
        "event": "user_login",
        "user_id": 123,
        "ip": "192.168.1.1",
        "timestamp": chrono::Utc::now().timestamp()
    });
    
    info!("{}", event);
}
```

### 3. 避免敏感信息泄露

```rust
use log::{info, debug};

fn secure_logging() {
    let password = "secret123";
    let email = "user@example.com";
    
    // 好的做法：不记录敏感信息
    info!("用户登录: {}", mask_email(&email));
    
    // 避免的做法：记录敏感信息
    // debug!("密码: {}", password);  // 危险！
    
    // 在调试时可以记录部分信息
    debug!("密码长度: {}", password.len());
}

fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (name, domain) = email.split_at(at_pos);
        let masked_name = if name.len() > 2 {
            format!("{}***", &name[..2])
        } else {
            "***".to_string()
        };
        format!("{}@{}", masked_name, domain)
    } else {
        "***".to_string()
    }
}
```

### 4. 性能考虑

```rust
use log::{debug, log_enabled, Level};

fn performance_conscious_logging() {
    // 检查日志级别再计算
    if log_enabled!(Level::Debug) {
        let expensive_data = compute_expensive_debug_data();
        debug!("调试数据: {}", expensive_data);
    }
    
    // 使用闭包延迟计算
    debug!("调试数据: {}", || compute_expensive_debug_data());
    
    // 避免不必要的字符串格式化
    let user_id = 123;
    debug!("用户ID: {}", user_id);  // 好
    // debug!("用户ID: {}", user_id.to_string());  // 避免
}

fn compute_expensive_debug_data() -> String {
    // 模拟昂贵的计算
    "昂贵的调试数据".to_string()
}
```

### 5. 错误处理和日志

```rust
use log::{error, warn, info};

fn error_handling_with_logging() {
    match risky_operation() {
        Ok(result) => {
            info!("操作成功: {}", result);
        }
        Err(e) => {
            error!("操作失败: {}", e);
            // 可能需要额外的错误处理
        }
    }
}

fn risky_operation() -> Result<String, &'static str> {
    // 模拟可能失败的操作
    Ok("success".to_string())
}
```

## 集成指南

### 与 Tokio 集成

```rust
use log::{info, error};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("异步应用启动");
    
    let handle = tokio::spawn(async {
        info!("异步任务开始");
        sleep(Duration::from_millis(100)).await;
        info!("异步任务完成");
    });
    
    if let Err(e) = handle.await {
        error!("任务失败: {}", e);
    }
}
```

### 与 tracing 对比

```rust
// 使用 log
use log::{info, error};

fn traditional_logging() {
    info!("处理用户请求");
    process_request();
    info!("请求处理完成");
}

// 使用 tracing (更现代的方式)
use tracing::{info, error, instrument};

#[instrument]
fn modern_logging() {
    info!("处理用户请求");
    process_request();
    info!("请求处理完成");
}

fn process_request() {
    // 处理逻辑
}
```

### 库作者指南

```rust
// 在库中使用 log
use log::{debug, error};

pub fn library_function(input: &str) -> Result<String, &'static str> {
    debug!("库函数调用: input={}", input);
    
    if input.is_empty() {
        error!("输入为空");
        return Err("输入不能为空");
    }
    
    let result = process_input(input);
    debug!("库函数结果: {}", result);
    
    Ok(result)
}

fn process_input(input: &str) -> String {
    format!("处理后: {}", input)
}
```

## 总结

Log 库是 Rust 日志生态系统的基础，它提供了简洁而强大的日志接口。通过本教程，您应该能够：

1. 理解日志门面的概念和优势
2. 正确使用各种日志级别和宏
3. 选择合适的日志实现
4. 优化日志性能
5. 遵循日志记录的最佳实践

关键要点：
- 选择合适的日志级别，避免日志噪音
- 使用结构化日志提高可读性
- 注意性能影响，避免不必要的计算
- 保护敏感信息，确保安全
- 与其他库和工具良好集成

Log 库的设计哲学是简单而高效，它为整个 Rust 生态系统提供了统一的日志接口，是构建可维护应用程序的重要工具。