# Dotenvy 0.15.7 - Rust 环境变量管理库使用教程

## 概述

Dotenvy 是一个维护良好的 dotenv crate 分支，用于从 `.env` 文件中加载环境变量。它是 Rust 生态系统中处理环境变量的标准选择，特别适合配置管理和开发环境设置。

**基本信息：**
- 版本：0.15.7
- 许可证：MIT
- 仓库：https://github.com/allan2/dotenvy
- 下载量：46,921,313+
- 创建时间：2022-02-28

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dotenvy = "0.15"
```

## 基本使用

### 1. 创建 .env 文件

在项目根目录创建 `.env` 文件：

```env
# 数据库配置
DATABASE_URL=postgresql://user:password@localhost/mydb
DATABASE_MAX_CONNECTIONS=10

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
API_KEY=your-secret-api-key

# 应用配置
APP_NAME=MyApp
APP_VERSION=1.0.0
DEBUG=true
LOG_LEVEL=info

# 可选配置
OPTIONAL_FEATURE_ENABLED=false
CACHE_TTL=3600
```

### 2. 加载环境变量

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    // 加载 .env 文件
    dotenv().ok();
    
    // 读取环境变量
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 必须设置");
    
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT 必须是有效的端口号");
    
    println!("数据库 URL: {}", database_url);
    println!("服务器端口: {}", server_port);
}
```

### 3. 错误处理

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    match dotenv() {
        Ok(path) => println!("成功加载 .env 文件: {:?}", path),
        Err(e) => println!("警告: 无法加载 .env 文件: {}", e),
    }
    
    // 使用 Result 处理环境变量
    let config = load_config();
    match config {
        Ok(cfg) => println!("配置加载成功: {:?}", cfg),
        Err(e) => eprintln!("配置加载失败: {}", e),
    }
}

#[derive(Debug)]
struct Config {
    database_url: String,
    server_port: u16,
    api_key: String,
    debug: bool,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(Config {
        database_url: env::var("DATABASE_URL")?,
        server_port: env::var("SERVER_PORT")?.parse()?,
        api_key: env::var("API_KEY")?,
        debug: env::var("DEBUG")?.parse()?,
    })
}
```

## 高级功能

### 1. 指定 .env 文件路径

```rust
use dotenvy::from_path;
use std::path::Path;

fn main() {
    // 从指定路径加载
    let env_path = Path::new("config/.env");
    match from_path(env_path) {
        Ok(_) => println!("成功加载配置文件"),
        Err(e) => println!("加载配置文件失败: {}", e),
    }
    
    // 从多个可能的路径加载
    let possible_paths = vec![
        "config/.env",
        ".env.local",
        ".env",
    ];
    
    for path in possible_paths {
        if let Ok(_) = from_path(path) {
            println!("成功加载配置文件: {}", path);
            break;
        }
    }
}
```

### 2. 使用 `from_read` 从任意 Reader 加载

```rust
use dotenvy::from_read;
use std::io::Cursor;

fn main() {
    let env_content = r#"
        APP_NAME=MyApp
        VERSION=1.0.0
        DEBUG=true
    "#;
    
    let cursor = Cursor::new(env_content);
    match from_read(cursor) {
        Ok(_) => println!("成功从字符串加载环境变量"),
        Err(e) => println!("加载失败: {}", e),
    }
}
```

### 3. 环境变量解析工具

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    // 字符串解析
    let app_name = get_env_string("APP_NAME", "DefaultApp");
    
    // 数字解析
    let port = get_env_number("SERVER_PORT", 8080);
    
    // 布尔值解析
    let debug = get_env_bool("DEBUG", false);
    
    // 数组解析
    let allowed_hosts = get_env_array("ALLOWED_HOSTS", vec!["localhost".to_string()]);
    
    println!("应用名称: {}", app_name);
    println!("端口: {}", port);
    println!("调试模式: {}", debug);
    println!("允许的主机: {:?}", allowed_hosts);
}

fn get_env_string(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_env_number<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Copy,
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn get_env_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .and_then(|s| match s.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}

fn get_env_array(key: &str, default: Vec<String>) -> Vec<String> {
    env::var(key)
        .ok()
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or(default)
}
```

## 配置管理模式

### 1. 结构化配置

```rust
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub features: FeatureConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FeatureConfig {
    pub cache_enabled: bool,
    pub metrics_enabled: bool,
    pub debug_mode: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        Ok(AppConfig {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")?.parse()?,
                timeout: env::var("DATABASE_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
                workers: env::var("SERVER_WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()?,
            },
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
                file: env::var("LOG_FILE").ok(),
            },
            features: FeatureConfig {
                cache_enabled: env::var("CACHE_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                metrics_enabled: env::var("METRICS_ENABLED")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()?,
                debug_mode: env::var("DEBUG")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()?,
            },
        })
    }
}

// 全局配置单例
use std::sync::OnceLock;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        AppConfig::load().expect("加载配置失败")
    })
}
```

### 2. 环境特定配置

```rust
use dotenvy::from_path;
use std::path::Path;

pub fn load_env_for_environment() {
    let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    
    // 按优先级加载配置文件
    let config_files = vec![
        format!(".env.{}.local", env),
        format!(".env.{}", env),
        ".env.local".to_string(),
        ".env".to_string(),
    ];
    
    for config_file in config_files {
        if Path::new(&config_file).exists() {
            match from_path(&config_file) {
                Ok(_) => println!("成功加载配置文件: {}", config_file),
                Err(e) => println!("加载配置文件失败 {}: {}", config_file, e),
            }
        }
    }
}
```

## 与 Web 框架集成

### 1. 与 Actix-web 集成

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
struct AppState {
    config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = AppConfig::load().expect("加载配置失败");
    let app_state = AppState { config: config.clone() };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/config", web::get().to(get_config_info))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now()
    })))
}

async fn get_config_info(data: web::Data<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "server": {
            "host": data.config.server.host,
            "port": data.config.server.port,
            "workers": data.config.server.workers
        },
        "features": {
            "cache_enabled": data.config.features.cache_enabled,
            "metrics_enabled": data.config.features.metrics_enabled,
            "debug_mode": data.config.features.debug_mode
        }
    })))
}
```

### 2. 与 Tokio 集成

```rust
use dotenvy::dotenv;
use tokio::time::{sleep, Duration};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let config = AppConfig::load()?;
    
    // 异步服务启动
    let server_task = tokio::spawn(async move {
        start_server(config.server).await;
    });
    
    // 后台任务
    let background_task = tokio::spawn(async move {
        background_worker(config.features).await;
    });
    
    // 等待所有任务完成
    tokio::try_join!(server_task, background_task)?;
    
    Ok(())
}

async fn start_server(config: ServerConfig) {
    println!("服务器启动在 {}:{}", config.host, config.port);
    // 服务器逻辑
}

async fn background_worker(features: FeatureConfig) {
    if features.cache_enabled {
        println!("缓存服务已启用");
    }
    
    if features.metrics_enabled {
        println!("指标收集已启用");
    }
    
    loop {
        // 后台任务逻辑
        sleep(Duration::from_secs(60)).await;
    }
}
```

## 安全最佳实践

### 1. 敏感信息处理

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    // 加载敏感配置
    let secrets = load_secrets();
    
    // 使用后立即清理
    match secrets {
        Ok(secret_config) => {
            // 使用配置
            process_with_secrets(&secret_config);
            // 配置会在作用域结束时自动清理
        }
        Err(e) => {
            eprintln!("加载敏感配置失败: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct SecretConfig {
    api_key: String,
    jwt_secret: String,
    database_password: String,
}

impl Drop for SecretConfig {
    fn drop(&mut self) {
        // 清理敏感信息
        self.api_key.clear();
        self.jwt_secret.clear();
        self.database_password.clear();
    }
}

fn load_secrets() -> Result<SecretConfig, Box<dyn std::error::Error>> {
    Ok(SecretConfig {
        api_key: env::var("API_KEY")?,
        jwt_secret: env::var("JWT_SECRET")?,
        database_password: env::var("DATABASE_PASSWORD")?,
    })
}

fn process_with_secrets(secrets: &SecretConfig) {
    // 使用敏感配置进行处理
    println!("使用 API 密钥进行认证...");
}
```

### 2. 配置验证

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    match validate_config() {
        Ok(_) => println!("配置验证通过"),
        Err(e) => {
            eprintln!("配置验证失败: {}", e);
            std::process::exit(1);
        }
    }
}

fn validate_config() -> Result<(), Box<dyn std::error::Error>> {
    // 验证必需的环境变量
    let required_vars = vec![
        "DATABASE_URL",
        "API_KEY",
        "JWT_SECRET",
    ];
    
    for var in required_vars {
        if env::var(var).is_err() {
            return Err(format!("必需的环境变量 {} 未设置", var).into());
        }
    }
    
    // 验证数据库 URL 格式
    let db_url = env::var("DATABASE_URL")?;
    if !db_url.starts_with("postgresql://") && !db_url.starts_with("mysql://") {
        return Err("DATABASE_URL 格式不正确".into());
    }
    
    // 验证端口号
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse()
        .map_err(|_| "SERVER_PORT 必须是有效的端口号")?;
    
    if port < 1024 && port != 80 && port != 443 {
        return Err("端口号必须大于 1024 或为 80/443".into());
    }
    
    // 验证日志级别
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let valid_levels = vec!["error", "warn", "info", "debug", "trace"];
    if !valid_levels.contains(&log_level.as_str()) {
        return Err(format!("无效的日志级别: {}. 有效值: {:?}", log_level, valid_levels).into());
    }
    
    Ok(())
}
```

## 开发工具和调试

### 1. 配置检查工具

```rust
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    println!("=== 环境变量配置检查 ===");
    
    // 检查所有环境变量
    check_all_env_vars();
    
    // 检查特定配置
    check_database_config();
    check_server_config();
    check_logging_config();
}

fn check_all_env_vars() {
    println!("\n--- 所有环境变量 ---");
    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (key, value) in vars {
        if key.contains("SECRET") || key.contains("PASSWORD") || key.contains("KEY") {
            println!("{} = [已隐藏]", key);
        } else {
            println!("{} = {}", key, value);
        }
    }
}

fn check_database_config() {
    println!("\n--- 数据库配置 ---");
    match env::var("DATABASE_URL") {
        Ok(url) => {
            println!("✓ DATABASE_URL 已设置");
            if url.starts_with("postgresql://") {
                println!("  - 数据库类型: PostgreSQL");
            } else if url.starts_with("mysql://") {
                println!("  - 数据库类型: MySQL");
            } else {
                println!("  - 数据库类型: 未知");
            }
        }
        Err(_) => println!("✗ DATABASE_URL 未设置"),
    }
    
    let max_conn = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "未设置".to_string());
    println!("  - 最大连接数: {}", max_conn);
}

fn check_server_config() {
    println!("\n--- 服务器配置 ---");
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    
    println!("  - 主机: {}", host);
    println!("  - 端口: {}", port);
    
    match port.parse::<u16>() {
        Ok(p) => println!("  - 端口验证: ✓ 有效 ({})", p),
        Err(_) => println!("  - 端口验证: ✗ 无效"),
    }
}

fn check_logging_config() {
    println!("\n--- 日志配置 ---");
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let file = env::var("LOG_FILE").unwrap_or_else(|_| "未设置".to_string());
    
    println!("  - 日志级别: {}", level);
    println!("  - 日志文件: {}", file);
    
    let valid_levels = vec!["error", "warn", "info", "debug", "trace"];
    if valid_levels.contains(&level.as_str()) {
        println!("  - 级别验证: ✓ 有效");
    } else {
        println!("  - 级别验证: ✗ 无效");
    }
}
```

### 2. 配置文件生成器

```rust
use std::fs::File;
use std::io::Write;

fn main() {
    generate_env_template().expect("生成配置模板失败");
    println!("已生成 .env.template 文件");
}

fn generate_env_template() -> std::io::Result<()> {
    let template = r#"# 应用配置
APP_NAME=MyApp
APP_VERSION=1.0.0
DEBUG=false

# 数据库配置
DATABASE_URL=postgresql://user:password@localhost/dbname
DATABASE_MAX_CONNECTIONS=10
DATABASE_TIMEOUT=30

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
SERVER_WORKERS=4

# 日志配置
LOG_LEVEL=info
# LOG_FILE=/var/log/myapp.log

# 安全配置
API_KEY=your-api-key-here
JWT_SECRET=your-jwt-secret-here

# 功能开关
CACHE_ENABLED=true
METRICS_ENABLED=false

# 第三方服务
REDIS_URL=redis://localhost:6379
EMAIL_SERVICE_URL=smtp://localhost:587
"#;
    
    let mut file = File::create(".env.template")?;
    file.write_all(template.as_bytes())?;
    
    Ok(())
}
```

## 总结

Dotenvy 是一个简单而强大的环境变量管理库，提供了：

1. **简单易用**：一行代码即可加载 .env 文件
2. **灵活性**：支持多种加载方式和文件格式
3. **安全性**：适合处理敏感配置信息
4. **可维护性**：良好的错误处理和调试支持
5. **生态集成**：与各种 Web 框架和工具无缝集成

**最佳实践建议：**
- 将 `.env` 文件添加到 `.gitignore`
- 为不同环境创建不同的配置文件
- 使用结构化配置管理
- 对敏感信息进行适当的安全处理
- 在生产环境中验证配置的完整性

Dotenvy 是 Rust 项目中管理环境变量的首选库，特别适合需要在不同环境间切换配置的应用程序。