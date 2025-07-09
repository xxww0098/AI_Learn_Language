# Tracing-Subscriber 0.3.19 - Rust 追踪订阅者库使用教程

## 概述

Tracing-Subscriber 是 Tracing 生态系统的核心组件，提供了实现和组合 `tracing` 订阅者的实用工具。它负责收集、处理和输出 tracing 库产生的追踪数据，支持多种输出格式和过滤选项。

**基本信息：**
- 版本：0.3.19
- 许可证：MIT
- 仓库：https://github.com/tokio-rs/tracing
- 官网：https://tokio.rs
- 下载量：205,319,902+

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## 基本使用

### 1. 简单初始化

```rust
use tracing::{info, warn, error};
use tracing_subscriber;

fn main() {
    // 最简单的初始化
    tracing_subscriber::fmt::init();
    
    info!("应用程序启动");
    warn!("这是一个警告");
    error!("这是一个错误");
}
```

### 2. 自定义格式化

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // 自定义订阅者
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    
    tracing::info!("自定义格式化的日志");
    tracing::debug!("调试信息");
}
```

### 3. 环境过滤器

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // 使用环境变量过滤器
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
    
    tracing::trace!("这条不会显示");
    tracing::debug!("这条不会显示");
    tracing::info!("这条会显示");
    tracing::warn!("这条会显示");
    tracing::error!("这条会显示");
}
```

## 详细配置

### 1. 格式化选项

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        // 输出格式
        .with_target(true)              // 显示目标模块
        .with_thread_ids(true)          // 显示线程 ID
        .with_thread_names(true)        // 显示线程名称
        .with_file(true)                // 显示文件名
        .with_line_number(true)         // 显示行号
        .with_level(true)               // 显示级别
        .with_ansi(true)                // 使用 ANSI 颜色
        // Span 格式
        .with_span_events(FmtSpan::FULL) // 显示 span 事件
        // 时间格式
        .with_timer(fmt::time::LocalTime::rfc_3339())
        // 环境过滤器
        .with_env_filter(EnvFilter::new("debug"))
        .init();
    
    tracing::info!("格式化演示");
    
    let span = tracing::info_span!("example_span", key = "value");
    let _guard = span.enter();
    
    tracing::info!("在 span 内的日志");
}
```

### 2. JSON 格式输出

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::new("info"))
        .init();
    
    tracing::info!(user_id = 123, action = "login", "用户登录");
    
    let span = tracing::info_span!("request", request_id = "req-456");
    let _guard = span.enter();
    
    tracing::info!(status = "processing", "处理请求");
    tracing::warn!(warning = "high_latency", latency_ms = 500, "高延迟警告");
}
```

### 3. 多层订阅者

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

fn main() {
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(true);
    
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
    
    tracing::info!("多层订阅者演示");
    tracing::debug!("调试信息");
}
```

## 高级功能

### 1. 文件输出

```rust
use tracing_subscriber::{fmt, EnvFilter};
use std::fs::OpenOptions;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建文件输出
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")?;
    
    // 同时输出到控制台和文件
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true);
    
    let file_layer = fmt::layer()
        .with_writer(file)
        .with_ansi(false)
        .json();
    
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::new("info"))
        .init();
    
    tracing::info!("这条日志会同时输出到控制台和文件");
    tracing::warn!("警告信息");
    
    Ok(())
}
```

### 2. 自定义过滤器

```rust
use tracing_subscriber::{fmt, filter::LevelFilter, EnvFilter, prelude::*};

fn main() {
    // 组合多个过滤器
    let filter = EnvFilter::new("info")
        .add_directive("my_app::database=debug".parse().unwrap())
        .add_directive("my_app::cache=warn".parse().unwrap());
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
    
    tracing::info!("根级别信息");
    
    // 模拟不同模块的日志
    tracing::info!(target: "my_app::database", "数据库调试信息");
    tracing::debug!(target: "my_app::database", "数据库调试信息");
    tracing::info!(target: "my_app::cache", "缓存信息");
    tracing::debug!(target: "my_app::cache", "缓存调试信息（不会显示）");
}
```

### 3. 时间格式化

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::time::{ChronoLocal, FormatTime};

// 自定义时间格式
struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_timer(CustomTimer)
        .with_env_filter(EnvFilter::new("info"))
        .init();
    
    tracing::info!("使用自定义时间格式");
    
    // 使用预定义的时间格式
    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::rfc_3339())
        .with_env_filter(EnvFilter::new("info"))
        .try_init()
        .unwrap_or_else(|_| ());
    
    tracing::info!("使用 RFC 3339 时间格式");
}
```

## 实际应用场景

### 1. Web 应用日志配置

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use tracing_subscriber::fmt::format::FmtSpan;
use std::fs::OpenOptions;
use std::io;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 创建日志文件
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("web_app.log")?;
    
    let error_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")?;
    
    // 控制台输出层
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::CLOSE);
    
    // 普通日志文件层
    let file_layer = fmt::layer()
        .with_writer(log_file)
        .with_ansi(false)
        .json()
        .with_span_events(FmtSpan::FULL);
    
    // 错误日志文件层
    let error_layer = fmt::layer()
        .with_writer(error_file)
        .with_ansi(false)
        .json()
        .with_filter(LevelFilter::ERROR);
    
    // 环境过滤器
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,axum=debug"));
    
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(error_layer)
        .with(env_filter)
        .init();
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;
    
    tracing::info!("Web 应用启动");
    
    // 模拟请求处理
    handle_request("GET", "/api/users").await;
    handle_request("POST", "/api/users").await;
    handle_error_request().await;
    
    Ok(())
}

async fn handle_request(method: &str, path: &str) {
    let span = tracing::info_span!(
        "request",
        method = method,
        path = path,
        request_id = uuid::Uuid::new_v4().to_string()
    );
    
    let _guard = span.enter();
    
    tracing::info!("处理请求");
    
    // 模拟处理时间
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    tracing::info!(status = 200, "请求完成");
}

async fn handle_error_request() {
    let span = tracing::error_span!("error_request", request_id = "req-error-123");
    let _guard = span.enter();
    
    tracing::error!("请求处理失败");
    tracing::error!(error = "database_connection_failed", "数据库连接失败");
}
```

### 2. 微服务日志配置

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use serde_json::json;
use std::collections::HashMap;

fn setup_microservice_logging(service_name: &str, version: &str) {
    let format = fmt::format()
        .json()
        .with_current_span(true)
        .with_span_list(true);
    
    let layer = fmt::layer()
        .event_format(format)
        .with_writer(std::io::stdout);
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(layer)
        .with(env_filter)
        .init();
    
    // 记录服务启动信息
    tracing::info!(
        service = service_name,
        version = version,
        timestamp = chrono::Utc::now().to_rfc3339(),
        "服务启动"
    );
}

#[tokio::main]
async fn main() {
    setup_microservice_logging("user-service", "1.0.0");
    
    // 模拟服务间调用
    call_external_service("order-service", "/api/orders").await;
    call_external_service("payment-service", "/api/payments").await;
    
    // 模拟错误处理
    handle_service_error().await;
}

async fn call_external_service(service: &str, endpoint: &str) {
    let trace_id = uuid::Uuid::new_v4().to_string();
    let span = tracing::info_span!(
        "external_call",
        service = service,
        endpoint = endpoint,
        trace_id = trace_id
    );
    
    let _guard = span.enter();
    
    tracing::info!("调用外部服务");
    
    // 模拟网络延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    tracing::info!(
        duration_ms = 200,
        status = "success",
        "外部服务调用完成"
    );
}

async fn handle_service_error() {
    let span = tracing::error_span!(
        "service_error",
        error_type = "external_service_timeout",
        trace_id = uuid::Uuid::new_v4().to_string()
    );
    
    let _guard = span.enter();
    
    tracing::error!(
        service = "payment-service",
        endpoint = "/api/charge",
        timeout_ms = 5000,
        "外部服务调用超时"
    );
}
```

### 3. 性能监控配置

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// 性能监控订阅者
struct PerformanceSubscriber {
    request_count: Arc<AtomicUsize>,
    error_count: Arc<AtomicUsize>,
}

impl PerformanceSubscriber {
    fn new() -> Self {
        Self {
            request_count: Arc::new(AtomicUsize::new(0)),
            error_count: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    fn report_metrics(&self) {
        let requests = self.request_count.load(Ordering::SeqCst);
        let errors = self.error_count.load(Ordering::SeqCst);
        
        tracing::info!(
            requests = requests,
            errors = errors,
            error_rate = if requests > 0 { (errors as f64 / requests as f64) * 100.0 } else { 0.0 },
            "性能指标"
        );
    }
}

fn setup_performance_monitoring() -> Arc<PerformanceSubscriber> {
    let perf_monitor = Arc::new(PerformanceSubscriber::new());
    
    // 格式化层
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_span_events(fmt::format::FmtSpan::CLOSE);
    
    // 环境过滤器
    let env_filter = EnvFilter::new("info");
    
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .init();
    
    perf_monitor
}

#[tokio::main]
async fn main() {
    let perf_monitor = setup_performance_monitoring();
    
    // 启动性能监控任务
    let monitor_clone = perf_monitor.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            monitor_clone.report_metrics();
        }
    });
    
    // 模拟请求处理
    for i in 0..20 {
        let monitor = perf_monitor.clone();
        tokio::spawn(async move {
            process_request(i, monitor).await;
        });
    }
    
    // 等待处理完成
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
}

async fn process_request(id: usize, monitor: Arc<PerformanceSubscriber>) {
    let start_time = Instant::now();
    
    let span = tracing::info_span!(
        "request",
        request_id = id,
        processing_time = tracing::field::Empty
    );
    
    let _guard = span.enter();
    
    monitor.request_count.fetch_add(1, Ordering::SeqCst);
    
    tracing::info!("开始处理请求");
    
    // 模拟处理时间
    let processing_time = rand::random::<u64>() % 1000;
    tokio::time::sleep(tokio::time::Duration::from_millis(processing_time)).await;
    
    // 模拟随机错误
    if rand::random::<f64>() < 0.1 {
        monitor.error_count.fetch_add(1, Ordering::SeqCst);
        tracing::error!("请求处理失败");
    } else {
        tracing::info!("请求处理成功");
    }
    
    let duration = start_time.elapsed();
    span.record("processing_time", &duration.as_millis());
    
    tracing::info!(
        duration_ms = duration.as_millis(),
        "请求处理完成"
    );
}
```

## 测试环境配置

### 1. 测试日志配置

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_test::traced_test;

fn setup_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(EnvFilter::new("debug"))
        .try_init();
}

#[traced_test]
fn test_with_tracing() {
    setup_test_logging();
    
    tracing::info!("测试开始");
    
    let result = calculate_sum(5, 3);
    tracing::info!(result = result, "计算完成");
    
    assert_eq!(result, 8);
    
    tracing::info!("测试通过");
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    tracing::debug!(a = a, b = b, "计算两数之和");
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_setup() {
        setup_test_logging();
        test_with_tracing();
    }
}
```

### 2. 基准测试配置

```rust
use tracing_subscriber::{fmt, EnvFilter};
use std::time::Instant;

fn setup_benchmark_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(false)
        .init();
}

fn benchmark_function<F>(name: &str, iterations: usize, mut f: F)
where
    F: FnMut() -> (),
{
    let span = tracing::info_span!("benchmark", name = name, iterations = iterations);
    let _guard = span.enter();
    
    tracing::info!("开始基准测试");
    
    let start = Instant::now();
    
    for i in 0..iterations {
        if i % 1000 == 0 {
            tracing::debug!(iteration = i, "基准测试进度");
        }
        f();
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;
    
    tracing::info!(
        total_time_ms = duration.as_millis(),
        avg_time_ns = avg_time,
        iterations = iterations,
        "基准测试完成"
    );
}

fn main() {
    setup_benchmark_logging();
    
    // 基准测试示例
    benchmark_function("string_creation", 10000, || {
        let _s = String::from("Hello, world!");
    });
    
    benchmark_function("vector_operations", 5000, || {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
    });
}
```

## 错误处理和调试

### 1. 订阅者错误处理

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::fs::OpenOptions;

fn setup_logging_with_fallback() {
    // 尝试设置文件日志
    let file_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log");
    
    let (file_layer, console_layer) = match file_result {
        Ok(file) => {
            tracing::info!("日志文件创建成功");
            (
                Some(fmt::layer().with_writer(file).json()),
                fmt::layer().with_writer(std::io::stdout)
            )
        }
        Err(e) => {
            eprintln!("无法创建日志文件: {}", e);
            (None, fmt::layer().with_writer(std::io::stdout))
        }
    };
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|e| {
            eprintln!("环境过滤器创建失败: {}", e);
            EnvFilter::new("info")
        });
    
    let mut registry = tracing_subscriber::registry()
        .with(console_layer)
        .with(env_filter);
    
    if let Some(file_layer) = file_layer {
        registry = registry.with(file_layer);
    }
    
    registry.init();
}

fn main() {
    setup_logging_with_fallback();
    
    tracing::info!("应用程序启动");
    tracing::warn!("这是一个警告");
    tracing::error!("这是一个错误");
}
```

### 2. 调试配置

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

fn setup_debug_logging() {
    // 创建调试专用的格式化器
    let debug_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(fmt::format::FmtSpan::FULL)
        .with_ansi(true);
    
    // 调试过滤器
    let debug_filter = EnvFilter::new("trace")
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("tokio=info".parse().unwrap());
    
    tracing_subscriber::registry()
        .with(debug_layer)
        .with(debug_filter)
        .init();
}

#[tracing::instrument]
fn debug_function(input: &str) -> String {
    tracing::trace!("函数开始执行");
    tracing::debug!(input = input, "输入参数");
    
    let result = format!("处理结果: {}", input);
    tracing::debug!(result = %result, "处理完成");
    
    tracing::trace!("函数执行结束");
    result
}

fn main() {
    setup_debug_logging();
    
    tracing::info!("调试模式启动");
    
    let result = debug_function("test input");
    tracing::info!(result = %result, "函数调用完成");
}
```

## 最佳实践

### 1. 生产环境配置

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::fs::OpenOptions;
use std::io;

fn setup_production_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 创建日志文件
    let app_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/myapp/app.log")?;
    
    let error_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/myapp/error.log")?;
    
    // 应用日志层 - JSON 格式，便于日志聚合
    let app_layer = fmt::layer()
        .with_writer(app_log)
        .json()
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_current_span(true);
    
    // 错误日志层
    let error_layer = fmt::layer()
        .with_writer(error_log)
        .json()
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::ERROR);
    
    // 控制台输出（仅关键信息）
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .compact()
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::WARN);
    
    // 生产环境过滤器
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,hyper=warn,tokio=warn"));
    
    tracing_subscriber::registry()
        .with(app_layer)
        .with(error_layer)
        .with(console_layer)
        .with(env_filter)
        .init();
    
    Ok(())
}

fn main() {
    setup_production_logging().expect("日志初始化失败");
    
    tracing::info!("生产环境应用启动");
    tracing::warn!("这是一个警告");
    tracing::error!("这是一个错误");
}
```

### 2. 配置管理

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct LogConfig {
    level: String,
    format: String,
    output: Vec<String>,
    file_path: Option<String>,
    rotation: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "human".to_string(),
            output: vec!["console".to_string()],
            file_path: None,
            rotation: None,
        }
    }
}

fn load_log_config() -> LogConfig {
    match fs::read_to_string("log_config.toml") {
        Ok(content) => {
            toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("配置文件解析失败: {}", e);
                LogConfig::default()
            })
        }
        Err(_) => {
            eprintln!("无法读取配置文件，使用默认配置");
            LogConfig::default()
        }
    }
}

fn setup_logging_from_config(config: &LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));
    
    let mut registry = tracing_subscriber::registry().with(env_filter);
    
    for output in &config.output {
        match output.as_str() {
            "console" => {
                let console_layer = match config.format.as_str() {
                    "json" => fmt::layer().json(),
                    "compact" => fmt::layer().compact(),
                    _ => fmt::layer(),
                };
                registry = registry.with(console_layer);
            }
            "file" => {
                if let Some(file_path) = &config.file_path {
                    let file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)?;
                    
                    let file_layer = fmt::layer()
                        .with_writer(file)
                        .json()
                        .with_ansi(false);
                    
                    registry = registry.with(file_layer);
                }
            }
            _ => eprintln!("未知的输出类型: {}", output),
        }
    }
    
    registry.init();
    Ok(())
}

fn main() {
    let config = load_log_config();
    
    setup_logging_from_config(&config)
        .expect("日志配置失败");
    
    tracing::info!("使用配置文件启动日志");
    tracing::debug!("调试信息");
    tracing::warn!("警告信息");
    tracing::error!("错误信息");
}
```

## 总结

Tracing-Subscriber 是一个功能强大的追踪数据处理库，提供了：

1. **灵活的配置选项**：支持多种输出格式和过滤规则
2. **模块化设计**：可以组合不同的层来创建复杂的日志处理管道
3. **高性能**：异步友好且性能开销小
4. **生产就绪**：适合各种部署环境的配置选项
5. **丰富的生态系统**：与各种工具和框架良好集成

**使用建议：**
- 根据环境（开发/测试/生产）选择合适的配置
- 使用结构化日志（JSON）便于后续分析
- 合理设置过滤规则避免日志过载
- 考虑日志轮转和存储管理
- 在生产环境中监控日志性能影响

Tracing-Subscriber 是构建可观测性系统的重要基础设施，配合 Tracing 库可以为 Rust 应用提供完整的追踪和监控解决方案。