# ThisError 2.0.12 中文教程

## 简介

ThisError 是一个用于派生 `Error` 特征的过程宏库。它简化了自定义错误类型的创建，提供了声明式的错误定义方式。与 `anyhow` 不同，`thiserror` 专注于错误类型的定义和实现，特别适合于库代码。

## 核心特性

- 🎯 自动派生 `Error` 特征
- 📝 声明式的错误定义
- 🔗 错误链和源错误支持
- 🚀 零运行时成本
- 📊 灵活的错误消息格式化
- 🔧 与标准库完全兼容

## 基本用法

### 1. 简单错误定义

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO 错误")]
    Io,
    
    #[error("网络错误: {message}")]
    Network { message: String },
    
    #[error("解析错误: {0}")]
    Parse(String),
    
    #[error("无效的输入值: {value}")]
    InvalidInput { value: i32 },
}

fn main() {
    let errors = vec![
        MyError::Io,
        MyError::Network { message: "连接超时".to_string() },
        MyError::Parse("无效的 JSON 格式".to_string()),
        MyError::InvalidInput { value: -1 },
    ];
    
    for error in errors {
        println!("错误: {}", error);
    }
}
```

### 2. 带源错误的错误类型

```rust
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("读取文件失败")]
    ReadError(#[from] io::Error),
    
    #[error("解析 JSON 失败")]
    JsonError(#[from] serde_json::Error),
    
    #[error("数据验证失败: {message}")]
    ValidationError {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

fn read_and_parse_file(path: &str) -> Result<serde_json::Value, DataError> {
    let content = std::fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    Ok(json)
}

fn main() {
    match read_and_parse_file("nonexistent.json") {
        Ok(data) => println!("数据: {}", data),
        Err(e) => {
            println!("错误: {}", e);
            
            // 遍历错误链
            let mut source = e.source();
            while let Some(err) = source {
                println!("  原因: {}", err);
                source = err.source();
            }
        }
    }
}
```

### 3. 复杂错误结构

```rust
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("连接错误: {host}:{port}")]
    Connection { host: String, port: u16 },
    
    #[error("查询执行失败")]
    QueryExecution(#[from] QueryError),
    
    #[error("事务失败: {reason}")]
    Transaction { reason: String },
    
    #[error("权限不足: 需要 {required} 权限")]
    Permission { required: String },
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("SQL 语法错误: {sql}")]
    Syntax { sql: String },
    
    #[error("表不存在: {table}")]
    TableNotFound { table: String },
    
    #[error("列不存在: {column} 在表 {table} 中")]
    ColumnNotFound { table: String, column: String },
    
    #[error("约束违反: {constraint}")]
    ConstraintViolation { constraint: String },
}

fn execute_query(sql: &str) -> Result<Vec<String>, DatabaseError> {
    // 模拟查询执行
    if sql.contains("SELCT") {
        return Err(QueryError::Syntax { sql: sql.to_string() }.into());
    }
    
    if sql.contains("nonexistent_table") {
        return Err(QueryError::TableNotFound { table: "nonexistent_table".to_string() }.into());
    }
    
    Ok(vec!["结果1".to_string(), "结果2".to_string()])
}

fn main() {
    let test_queries = vec![
        "SELECT * FROM users",
        "SELCT * FROM users",  // 语法错误
        "SELECT * FROM nonexistent_table",
    ];
    
    for query in test_queries {
        match execute_query(query) {
            Ok(results) => println!("查询成功: {:?}", results),
            Err(e) => println!("查询失败: {}", e),
        }
    }
}
```

## 高级特性

### 1. 透明错误传播

```rust
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("配置文件错误")]
    File(#[from] io::Error),
    
    #[error("JSON 解析错误")]
    Json(#[from] serde_json::Error),
    
    #[error("环境变量错误")]
    Env(#[from] std::env::VarError),
    
    #[error("无效的配置值: {key} = {value}")]
    InvalidValue { key: String, value: String },
}

#[derive(serde::Deserialize, Debug)]
struct Config {
    database_url: String,
    port: u16,
    debug: bool,
}

fn load_config() -> Result<Config, ConfigError> {
    // 尝试从文件加载
    let config_content = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_content)?;
    
    // 验证配置
    if config.port == 0 {
        return Err(ConfigError::InvalidValue {
            key: "port".to_string(),
            value: config.port.to_string(),
        });
    }
    
    Ok(config)
}

fn main() {
    match load_config() {
        Ok(config) => println!("配置加载成功: {:?}", config),
        Err(e) => println!("配置加载失败: {}", e),
    }
}
```

### 2. 条件错误字段

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP 错误: {code}")]
    Http {
        code: u16,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    #[error("序列化错误")]
    Serialization(#[from] serde_json::Error),
    
    #[error("验证失败: {field}")]
    Validation {
        field: String,
        #[source]
        source: Option<ValidationError>,
    },
}

#[derive(Error, Debug)]
#[error("字段 '{field}' 的值无效: {reason}")]
pub struct ValidationError {
    field: String,
    reason: String,
}

fn validate_user_input(data: &str) -> Result<serde_json::Value, ApiError> {
    let json: serde_json::Value = serde_json::from_str(data)?;
    
    // 验证必需字段
    if !json.get("name").and_then(|v| v.as_str()).map_or(false, |s| !s.is_empty()) {
        return Err(ApiError::Validation {
            field: "name".to_string(),
            source: Some(ValidationError {
                field: "name".to_string(),
                reason: "名称不能为空".to_string(),
            }),
        });
    }
    
    Ok(json)
}

fn main() {
    let test_data = vec![
        r#"{"name": "Alice", "age": 30}"#,
        r#"{"name": "", "age": 30}"#,
        r#"{"age": 30}"#,
        r#"invalid json"#,
    ];
    
    for data in test_data {
        match validate_user_input(data) {
            Ok(json) => println!("验证成功: {}", json),
            Err(e) => {
                println!("验证失败: {}", e);
                if let Some(source) = e.source() {
                    println!("  详情: {}", source);
                }
            }
        }
    }
}
```

## 实际应用示例

### 1. 文件处理库

```rust
use thiserror::Error;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Error, Debug)]
pub enum FileProcessorError {
    #[error("文件 IO 错误")]
    Io(#[from] io::Error),
    
    #[error("不支持的文件格式: {extension}")]
    UnsupportedFormat { extension: String },
    
    #[error("文件大小超过限制: {size} bytes (最大 {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },
    
    #[error("文件为空")]
    EmptyFile,
    
    #[error("编码错误: {encoding}")]
    EncodingError { encoding: String },
    
    #[error("处理错误: {message}")]
    ProcessingError { message: String },
}

pub struct FileProcessor {
    max_file_size: u64,
    supported_extensions: Vec<String>,
}

impl FileProcessor {
    pub fn new(max_file_size: u64) -> Self {
        Self {
            max_file_size,
            supported_extensions: vec!["txt".to_string(), "md".to_string(), "json".to_string()],
        }
    }
    
    pub fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<String, FileProcessorError> {
        let path = path.as_ref();
        
        // 检查文件扩展名
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        if !self.supported_extensions.contains(&extension.to_string()) {
            return Err(FileProcessorError::UnsupportedFormat {
                extension: extension.to_string(),
            });
        }
        
        // 检查文件大小
        let metadata = fs::metadata(path)?;
        if metadata.len() > self.max_file_size {
            return Err(FileProcessorError::FileTooLarge {
                size: metadata.len(),
                max: self.max_file_size,
            });
        }
        
        // 读取文件内容
        let content = fs::read_to_string(path)?;
        
        // 检查是否为空文件
        if content.is_empty() {
            return Err(FileProcessorError::EmptyFile);
        }
        
        // 根据文件类型处理
        match extension {
            "txt" | "md" => self.process_text(&content),
            "json" => self.process_json(&content),
            _ => Err(FileProcessorError::UnsupportedFormat {
                extension: extension.to_string(),
            }),
        }
    }
    
    fn process_text(&self, content: &str) -> Result<String, FileProcessorError> {
        // 检查编码
        if !content.is_ascii() {
            return Err(FileProcessorError::EncodingError {
                encoding: "非 ASCII 字符".to_string(),
            });
        }
        
        // 处理文本
        let lines: Vec<&str> = content.lines().collect();
        let word_count = content.split_whitespace().count();
        
        let result = format!(
            "文本处理结果:\n行数: {}\n字数: {}\n字符数: {}\n",
            lines.len(),
            word_count,
            content.len()
        );
        
        Ok(result)
    }
    
    fn process_json(&self, content: &str) -> Result<String, FileProcessorError> {
        // 尝试解析 JSON
        let json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| FileProcessorError::ProcessingError {
                message: format!("JSON 解析失败: {}", e),
            })?;
        
        let result = format!(
            "JSON 处理结果:\n类型: {}\n键数量: {}\n",
            match &json {
                serde_json::Value::Object(_) => "对象",
                serde_json::Value::Array(_) => "数组",
                serde_json::Value::String(_) => "字符串",
                serde_json::Value::Number(_) => "数字",
                serde_json::Value::Bool(_) => "布尔值",
                serde_json::Value::Null => "null",
            },
            if let serde_json::Value::Object(obj) = &json {
                obj.len()
            } else {
                0
            }
        );
        
        Ok(result)
    }
}

fn main() {
    let processor = FileProcessor::new(1024 * 1024); // 1MB 限制
    
    let test_files = vec![
        "test.txt",
        "test.json",
        "test.exe",
        "nonexistent.txt",
    ];
    
    for file in test_files {
        match processor.process_file(file) {
            Ok(result) => println!("文件 {} 处理成功:\n{}", file, result),
            Err(e) => {
                println!("文件 {} 处理失败: {}", file, e);
                
                // 根据错误类型提供建议
                match e {
                    FileProcessorError::UnsupportedFormat { extension } => {
                        println!("  建议: 仅支持 txt、md、json 格式，当前格式: {}", extension);
                    }
                    FileProcessorError::FileTooLarge { size, max } => {
                        println!("  建议: 文件大小 {} 超过限制 {}，请减小文件大小", size, max);
                    }
                    FileProcessorError::EmptyFile => {
                        println!("  建议: 请确保文件包含内容");
                    }
                    _ => {}
                }
            }
        }
        println!();
    }
}
```

### 2. 网络客户端库

```rust
use thiserror::Error;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("连接超时: {timeout:?}")]
    Timeout { timeout: Duration },
    
    #[error("DNS 解析失败: {hostname}")]
    DnsResolution { hostname: String },
    
    #[error("TLS 握手失败: {reason}")]
    TlsHandshake { reason: String },
    
    #[error("HTTP 状态错误: {code} {message}")]
    HttpStatus { code: u16, message: String },
    
    #[error("序列化错误")]
    Serialization(#[from] serde_json::Error),
    
    #[error("网络 IO 错误")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("网络错误")]
    Network(#[from] NetworkError),
    
    #[error("认证失败: {reason}")]
    Authentication { reason: String },
    
    #[error("配置错误: {message}")]
    Configuration { message: String },
    
    #[error("请求限制: {limit} 请求/秒")]
    RateLimit { limit: u32 },
}

pub struct HttpClient {
    base_url: String,
    timeout: Duration,
}

impl HttpClient {
    pub fn new(base_url: String, timeout: Duration) -> Result<Self, ClientError> {
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            return Err(ClientError::Configuration {
                message: "URL 必须以 http:// 或 https:// 开头".to_string(),
            });
        }
        
        Ok(Self { base_url, timeout })
    }
    
    pub fn get(&self, endpoint: &str) -> Result<String, ClientError> {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'));
        
        // 模拟网络请求
        self.mock_request("GET", &url)
    }
    
    pub fn post(&self, endpoint: &str, data: &serde_json::Value) -> Result<String, ClientError> {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'));
        
        // 序列化数据
        let _json_data = serde_json::to_string(data)?;
        
        // 模拟网络请求
        self.mock_request("POST", &url)
    }
    
    fn mock_request(&self, method: &str, url: &str) -> Result<String, ClientError> {
        // 模拟不同的错误情况
        if url.contains("timeout") {
            return Err(NetworkError::Timeout { timeout: self.timeout }.into());
        }
        
        if url.contains("dns-error") {
            return Err(NetworkError::DnsResolution { hostname: "invalid.domain".to_string() }.into());
        }
        
        if url.contains("tls-error") {
            return Err(NetworkError::TlsHandshake { reason: "证书验证失败".to_string() }.into());
        }
        
        if url.contains("404") {
            return Err(NetworkError::HttpStatus { code: 404, message: "Not Found".to_string() }.into());
        }
        
        if url.contains("401") {
            return Err(ClientError::Authentication { reason: "Token 已过期".to_string() });
        }
        
        if url.contains("rate-limit") {
            return Err(ClientError::RateLimit { limit: 100 });
        }
        
        // 模拟成功响应
        Ok(format!(r#"{{"method": "{}", "url": "{}", "status": "success"}}"#, method, url))
    }
}

fn main() {
    let client = match HttpClient::new("https://api.example.com".to_string(), Duration::from_secs(30)) {
        Ok(client) => client,
        Err(e) => {
            println!("客户端创建失败: {}", e);
            return;
        }
    };
    
    let test_endpoints = vec![
        "/users",
        "/timeout",
        "/dns-error",
        "/tls-error",
        "/404",
        "/401",
        "/rate-limit",
    ];
    
    for endpoint in test_endpoints {
        match client.get(endpoint) {
            Ok(response) => println!("GET {} 成功: {}", endpoint, response),
            Err(e) => {
                println!("GET {} 失败: {}", endpoint, e);
                
                // 根据错误类型提供不同的处理建议
                match &e {
                    ClientError::Network(NetworkError::Timeout { timeout }) => {
                        println!("  建议: 增加超时时间，当前: {:?}", timeout);
                    }
                    ClientError::Network(NetworkError::HttpStatus { code, .. }) => {
                        println!("  建议: 检查 HTTP 状态码 {} 对应的处理逻辑", code);
                    }
                    ClientError::Authentication { .. } => {
                        println!("  建议: 检查认证凭据是否有效");
                    }
                    ClientError::RateLimit { limit } => {
                        println!("  建议: 实现请求限制，当前限制: {} 请求/秒", limit);
                    }
                    _ => {}
                }
            }
        }
        println!();
    }
}
```

## 最佳实践

### 1. 与 Anyhow 结合使用

```rust
use thiserror::Error;
use anyhow::Result;

// 库定义的错误类型
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("配置错误: {message}")]
    Configuration { message: String },
    
    #[error("操作失败: {operation}")]
    Operation { operation: String },
}

// 库函数使用 thiserror 定义的错误
pub fn library_function(config: &str) -> Result<String, LibraryError> {
    if config.is_empty() {
        return Err(LibraryError::Configuration {
            message: "配置不能为空".to_string(),
        });
    }
    
    Ok("操作成功".to_string())
}

// 应用程序使用 anyhow 处理错误
fn main() -> Result<()> {
    let result = library_function("")?;
    println!("结果: {}", result);
    
    Ok(())
}
```

### 2. 错误转换和包装

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("连接错误")]
    Connection,
    
    #[error("查询错误: {sql}")]
    Query { sql: String },
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("数据库错误")]
    Database(#[from] DatabaseError),
    
    #[error("业务逻辑错误: {message}")]
    Business { message: String },
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("服务错误")]
    Service(#[from] ServiceError),
    
    #[error("请求格式错误: {details}")]
    BadRequest { details: String },
}

fn database_operation() -> Result<String, DatabaseError> {
    Err(DatabaseError::Query { sql: "SELECT * FROM users".to_string() })
}

fn service_operation() -> Result<String, ServiceError> {
    database_operation()?;
    Ok("服务操作成功".to_string())
}

fn api_handler() -> Result<String, ApiError> {
    service_operation()?;
    Ok("API 处理成功".to_string())
}

fn main() {
    match api_handler() {
        Ok(result) => println!("成功: {}", result),
        Err(e) => {
            println!("API 错误: {}", e);
            
            // 遍历错误链
            let mut source = e.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("  级别 {}: {}", level, err);
                source = err.source();
                level += 1;
            }
        }
    }
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
thiserror = "2.0.12"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 总结

ThisError 是 Rust 中定义自定义错误类型的最佳工具。它提供了声明式的错误定义方式，自动实现 `Error` 特征，特别适合库开发。

主要特性：
- 🎯 声明式错误定义，减少样板代码
- 📝 灵活的错误消息格式化
- 🔗 完整的错误链和源错误支持
- 🚀 零运行时成本的宏展开
- 📊 优秀的调试体验
- 🔧 与标准库和生态系统完美集成

使用场景：
- **库开发**: 定义库特定的错误类型
- **错误分层**: 创建清晰的错误层次结构
- **错误转换**: 在不同层次间转换错误
- **类型安全**: 提供类型安全的错误处理

ThisError 与 Anyhow 配合使用，构成了 Rust 错误处理的最佳实践。