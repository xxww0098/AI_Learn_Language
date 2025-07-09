# Once_cell 1.21.3 - Rust 单次赋值单元和延迟值完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [基本用法](#基本用法)
- [高级特性](#高级特性)
- [与lazy_static对比](#与lazy_static对比)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)
- [标准库集成](#标准库集成)

## 概述

Once_cell 是一个提供单次赋值单元和延迟值的 Rust 库。它比 lazy_static 更灵活，API 设计更现代，并且部分功能已被 Rust 标准库采用。

### 核心特性
- **单次赋值**: 确保值只被设置一次
- **延迟初始化**: 值在首次访问时初始化
- **线程安全**: 提供同步和非同步版本
- **零成本抽象**: 高效的内存布局和性能
- **API 灵活**: 比 lazy_static 更丰富的 API

### 版本信息
- **当前版本**: 1.21.3
- **发布时间**: 2025-03-28
- **下载次数**: 490,351,247+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
once_cell = "1.21.3"
```

### 基本示例

```rust
use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;

// 延迟静态变量
static GLOBAL_MAP: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "one".to_string());
    m.insert(2, "two".to_string());
    m
});

// 单次赋值静态变量
static GLOBAL_CONFIG: OnceCell<String> = OnceCell::new();

fn main() {
    // 使用延迟初始化
    println!("Value: {}", GLOBAL_MAP.get(&1).unwrap());
    
    // 设置单次赋值值
    GLOBAL_CONFIG.set("production".to_string()).unwrap();
    println!("Config: {}", GLOBAL_CONFIG.get().unwrap());
    
    // 尝试再次设置会失败
    assert!(GLOBAL_CONFIG.set("debug".to_string()).is_err());
}
```

## 核心概念

### OnceCell vs Lazy

```rust
use once_cell::sync::{OnceCell, Lazy};

// OnceCell: 手动设置值
static CONFIG: OnceCell<String> = OnceCell::new();

// Lazy: 自动初始化
static COMPUTED: Lazy<String> = Lazy::new(|| {
    expensive_computation()
});

fn demonstrate_difference() {
    // OnceCell 需要手动设置
    CONFIG.set("value".to_string()).unwrap();
    println!("Config: {}", CONFIG.get().unwrap());
    
    // Lazy 自动计算
    println!("Computed: {}", &*COMPUTED);
}

fn expensive_computation() -> String {
    "计算结果".to_string()
}
```

### 同步 vs 非同步版本

```rust
use once_cell::{sync, unsync};

// 线程安全版本
static SYNC_VALUE: sync::OnceCell<i32> = sync::OnceCell::new();

fn sync_example() {
    // 可以在多线程中安全使用
    SYNC_VALUE.set(42).unwrap();
    println!("Sync value: {}", SYNC_VALUE.get().unwrap());
}

fn unsync_example() {
    // 非线程安全版本，性能更好
    let unsync_value: unsync::OnceCell<i32> = unsync::OnceCell::new();
    unsync_value.set(42).unwrap();
    println!("Unsync value: {}", unsync_value.get().unwrap());
}
```

## 基本用法

### OnceCell 基本操作

```rust
use once_cell::sync::OnceCell;

fn oncecell_basics() {
    let cell = OnceCell::new();
    
    // 检查是否已设置
    assert!(!cell.is_initialized());
    
    // 设置值
    cell.set(42).unwrap();
    assert!(cell.is_initialized());
    
    // 获取值
    assert_eq!(cell.get(), Some(&42));
    
    // 尝试再次设置会失败
    assert!(cell.set(43).is_err());
    
    // get_or_init 提供延迟初始化
    let cell2 = OnceCell::new();
    let value = cell2.get_or_init(|| 100);
    assert_eq!(value, &100);
}
```

### Lazy 基本操作

```rust
use once_cell::sync::Lazy;

// 全局 Lazy 变量
static GLOBAL_DATA: Lazy<Vec<String>> = Lazy::new(|| {
    vec!["item1".to_string(), "item2".to_string()]
});

fn lazy_basics() {
    // 本地 Lazy 变量
    let local_data = Lazy::new(|| {
        expensive_computation()
    });
    
    // 首次访问时初始化
    println!("Local data: {}", &*local_data);
    
    // 全局数据访问
    println!("Global data: {:?}", &*GLOBAL_DATA);
}

fn expensive_computation() -> String {
    std::thread::sleep(std::time::Duration::from_millis(100));
    "昂贵的计算结果".to_string()
}
```

### 错误处理

```rust
use once_cell::sync::OnceCell;

fn error_handling() {
    let cell = OnceCell::new();
    
    // get_or_try_init 用于可能失败的初始化
    let result = cell.get_or_try_init(|| -> Result<String, &'static str> {
        if rand::random() {
            Ok("success".to_string())
        } else {
            Err("initialization failed")
        }
    });
    
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // 如果初始化失败，可以重试
    if cell.get().is_none() {
        let _ = cell.get_or_init(|| "default".to_string());
    }
}
```

## 高级特性

### 自定义初始化逻辑

```rust
use once_cell::sync::{OnceCell, Lazy};
use std::collections::HashMap;

struct ConfigManager {
    settings: OnceCell<HashMap<String, String>>,
}

impl ConfigManager {
    fn new() -> Self {
        ConfigManager {
            settings: OnceCell::new(),
        }
    }
    
    fn load_config(&self) -> &HashMap<String, String> {
        self.settings.get_or_init(|| {
            let mut config = HashMap::new();
            // 模拟从文件或环境变量加载配置
            config.insert("database_url".to_string(), "localhost:5432".to_string());
            config.insert("api_key".to_string(), "secret_key".to_string());
            config
        })
    }
    
    fn get_setting(&self, key: &str) -> Option<&String> {
        self.load_config().get(key)
    }
}

fn custom_initialization() {
    let manager = ConfigManager::new();
    
    if let Some(db_url) = manager.get_setting("database_url") {
        println!("Database URL: {}", db_url);
    }
}
```

### 条件初始化

```rust
use once_cell::sync::OnceCell;

static FEATURE_FLAGS: OnceCell<HashMap<String, bool>> = OnceCell::new();

fn conditional_initialization() {
    let flags = FEATURE_FLAGS.get_or_init(|| {
        let mut flags = HashMap::new();
        
        // 根据环境变量设置特性标志
        flags.insert("new_ui".to_string(), std::env::var("NEW_UI").is_ok());
        flags.insert("debug_mode".to_string(), cfg!(debug_assertions));
        flags.insert("experimental".to_string(), false);
        
        flags
    });
    
    if *flags.get("new_ui").unwrap_or(&false) {
        println!("使用新UI");
    }
    
    if *flags.get("debug_mode").unwrap_or(&false) {
        println!("调试模式已启用");
    }
}
```

### 资源管理

```rust
use once_cell::sync::OnceCell;
use std::sync::Arc;

struct DatabaseConnection {
    url: String,
}

impl DatabaseConnection {
    fn new(url: &str) -> Self {
        println!("创建数据库连接: {}", url);
        DatabaseConnection {
            url: url.to_string(),
        }
    }
    
    fn query(&self, sql: &str) -> String {
        format!("执行查询 '{}' on {}", sql, self.url)
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("关闭数据库连接: {}", self.url);
    }
}

static DB_CONNECTION: OnceCell<Arc<DatabaseConnection>> = OnceCell::new();

fn resource_management() {
    let conn = DB_CONNECTION.get_or_init(|| {
        Arc::new(DatabaseConnection::new("postgresql://localhost:5432/mydb"))
    });
    
    let result = conn.query("SELECT * FROM users");
    println!("查询结果: {}", result);
}
```

### 原子操作和竞争检测

```rust
use once_cell::race::OnceBox;
use std::thread;

fn race_detection() {
    let once_box = OnceBox::new();
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let once_box = &once_box;
            thread::spawn(move || {
                // 多个线程尝试设置值，只有一个会成功
                let value = Box::new(format!("Value from thread {}", i));
                match once_box.set(value) {
                    Ok(()) => println!("Thread {} 成功设置值", i),
                    Err(value) => println!("Thread {} 设置失败，值被丢弃: {}", i, value),
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    if let Some(value) = once_box.get() {
        println!("最终值: {}", value);
    }
}
```

## 与lazy_static对比

### 语法对比

```rust
// lazy_static 语法
use lazy_static::lazy_static;

lazy_static! {
    static ref LAZY_STATIC_MAP: HashMap<i32, String> = {
        let mut m = HashMap::new();
        m.insert(1, "one".to_string());
        m
    };
}

// once_cell 语法
use once_cell::sync::Lazy;

static ONCE_CELL_MAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "one".to_string());
    m
});
```

### 功能对比

```rust
use once_cell::sync::{Lazy, OnceCell};

// once_cell 提供更多灵活性
static FLEXIBLE_CONFIG: OnceCell<String> = OnceCell::new();

fn flexibility_demo() {
    // 可以在运行时决定初始化逻辑
    let config_source = if std::env::var("USE_FILE").is_ok() {
        "file_config"
    } else {
        "env_config"
    };
    
    FLEXIBLE_CONFIG.set(config_source.to_string()).unwrap();
    
    // lazy_static 无法做到这种运行时决策
}
```

### 性能对比

```rust
use once_cell::sync::Lazy;
use std::time::Instant;

static PERFORMANCE_DATA: Lazy<Vec<u64>> = Lazy::new(|| {
    (0..1_000_000).collect()
});

fn performance_comparison() {
    let start = Instant::now();
    
    // 首次访问
    let len = PERFORMANCE_DATA.len();
    println!("首次访问耗时: {:?}, 长度: {}", start.elapsed(), len);
    
    let start = Instant::now();
    
    // 后续访问
    let len = PERFORMANCE_DATA.len();
    println!("后续访问耗时: {:?}, 长度: {}", start.elapsed(), len);
}
```

## 性能优化

### 内存布局优化

```rust
use once_cell::sync::Lazy;

// 大型数据结构使用 Box 减少栈使用
static LARGE_DATA: Lazy<Box<[u8; 1_000_000]>> = Lazy::new(|| {
    Box::new([0u8; 1_000_000])
});

// 小型数据直接存储
static SMALL_DATA: Lazy<[i32; 10]> = Lazy::new(|| {
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
});

fn memory_optimization() {
    println!("Large data first byte: {}", LARGE_DATA[0]);
    println!("Small data sum: {}", SMALL_DATA.iter().sum::<i32>());
}
```

### 缓存计算结果

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

static PRIME_CACHE: Lazy<HashMap<u32, bool>> = Lazy::new(|| {
    let mut cache = HashMap::new();
    
    // 预计算一些素数
    for n in 2..1000 {
        cache.insert(n, is_prime(n));
    }
    
    cache
});

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn cached_computation() {
    // 使用预计算的缓存
    for n in [17, 25, 97, 100] {
        if let Some(&is_prime) = PRIME_CACHE.get(&n) {
            println!("{} is prime: {}", n, is_prime);
        }
    }
}
```

### 延迟加载策略

```rust
use once_cell::sync::OnceCell;

struct HeavyResource {
    data: Vec<String>,
}

impl HeavyResource {
    fn new() -> Self {
        println!("正在加载重型资源...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        HeavyResource {
            data: (0..1000).map(|i| format!("item_{}", i)).collect(),
        }
    }
    
    fn get_data(&self) -> &[String] {
        &self.data
    }
}

static HEAVY_RESOURCE: OnceCell<HeavyResource> = OnceCell::new();

fn lazy_loading_strategy() {
    println!("程序启动");
    
    // 只有在需要时才加载重型资源
    if should_use_heavy_resource() {
        let resource = HEAVY_RESOURCE.get_or_init(|| HeavyResource::new());
        println!("资源项目数: {}", resource.get_data().len());
    }
}

fn should_use_heavy_resource() -> bool {
    // 某些条件决定是否需要资源
    true
}
```

## 实战案例

### 配置管理系统

```rust
use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct AppConfig {
    database_url: String,
    api_key: String,
    log_level: String,
    features: HashMap<String, bool>,
}

impl AppConfig {
    fn load() -> Self {
        AppConfig {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:memory:".to_string()),
            api_key: std::env::var("API_KEY")
                .unwrap_or_else(|_| "dev_key".to_string()),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            features: Self::load_features(),
        }
    }
    
    fn load_features() -> HashMap<String, bool> {
        let mut features = HashMap::new();
        features.insert("feature_a".to_string(), true);
        features.insert("feature_b".to_string(), false);
        features.insert("experimental".to_string(), 
                       std::env::var("EXPERIMENTAL").is_ok());
        features
    }
}

static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::load());

fn config_management_system() {
    println!("数据库 URL: {}", CONFIG.database_url);
    println!("日志级别: {}", CONFIG.log_level);
    
    if *CONFIG.features.get("experimental").unwrap_or(&false) {
        println!("实验性功能已启用");
    }
}
```

### 单例模式实现

```rust
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

struct Logger {
    level: String,
    output: Mutex<Vec<String>>,
}

impl Logger {
    fn new(level: &str) -> Self {
        Logger {
            level: level.to_string(),
            output: Mutex::new(Vec::new()),
        }
    }
    
    fn log(&self, message: &str) {
        let mut output = self.output.lock().unwrap();
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        output.push(format!("[{}] {}: {}", timestamp, self.level, message));
    }
    
    fn get_logs(&self) -> Vec<String> {
        self.output.lock().unwrap().clone()
    }
}

static LOGGER: OnceCell<Arc<Logger>> = OnceCell::new();

fn get_logger() -> &'static Arc<Logger> {
    LOGGER.get_or_init(|| {
        Arc::new(Logger::new("INFO"))
    })
}

fn singleton_pattern() {
    let logger = get_logger();
    logger.log("应用程序启动");
    logger.log("处理用户请求");
    
    // 在应用程序的任何地方都可以获得同一个 logger 实例
    let same_logger = get_logger();
    same_logger.log("处理完成");
    
    println!("日志条目数: {}", logger.get_logs().len());
}
```

### 缓存系统

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct CacheEntry<V> {
    value: V,
    created_at: std::time::Instant,
    ttl: std::time::Duration,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: std::time::Duration) -> Self {
        CacheEntry {
            value,
            created_at: std::time::Instant::now(),
            ttl,
        }
    }
    
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new() -> Self {
        Cache {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    fn get(&self, key: &K) -> Option<V> {
        let cache = self.data.read().unwrap();
        if let Some(entry) = cache.get(key) {
            if !entry.is_expired() {
                return Some(entry.value.clone());
            }
        }
        None
    }
    
    fn set(&self, key: K, value: V, ttl: std::time::Duration) {
        let mut cache = self.data.write().unwrap();
        cache.insert(key, CacheEntry::new(value, ttl));
    }
    
    fn cleanup_expired(&self) {
        let mut cache = self.data.write().unwrap();
        cache.retain(|_, entry| !entry.is_expired());
    }
}

static STRING_CACHE: Lazy<Cache<String, String>> = Lazy::new(|| Cache::new());

fn cache_system() {
    // 设置缓存项
    STRING_CACHE.set(
        "user_123".to_string(), 
        "John Doe".to_string(), 
        std::time::Duration::from_secs(60)
    );
    
    // 获取缓存项
    if let Some(user_name) = STRING_CACHE.get(&"user_123".to_string()) {
        println!("用户名: {}", user_name);
    }
    
    // 清理过期项
    STRING_CACHE.cleanup_expired();
}
```

### 连接池实现

```rust
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

struct Connection {
    id: u32,
    created_at: std::time::Instant,
}

impl Connection {
    fn new(id: u32) -> Self {
        Connection {
            id,
            created_at: std::time::Instant::now(),
        }
    }
    
    fn execute(&self, query: &str) -> String {
        format!("Connection {} executed: {}", self.id, query)
    }
    
    fn is_healthy(&self) -> bool {
        // 检查连接是否健康
        self.created_at.elapsed() < std::time::Duration::from_secs(3600)
    }
}

struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<Connection>>>,
    max_size: u32,
    current_id: Arc<Mutex<u32>>,
}

impl ConnectionPool {
    fn new(max_size: u32) -> Self {
        ConnectionPool {
            connections: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
            current_id: Arc::new(Mutex::new(0)),
        }
    }
    
    fn get_connection(&self) -> Option<Connection> {
        let mut connections = self.connections.lock().unwrap();
        
        // 尝试从池中获取健康的连接
        while let Some(conn) = connections.pop_front() {
            if conn.is_healthy() {
                return Some(conn);
            }
        }
        
        // 如果没有可用连接，创建新的
        if connections.len() < self.max_size as usize {
            let mut id = self.current_id.lock().unwrap();
            *id += 1;
            Some(Connection::new(*id))
        } else {
            None
        }
    }
    
    fn return_connection(&self, connection: Connection) {
        if connection.is_healthy() {
            let mut connections = self.connections.lock().unwrap();
            connections.push_back(connection);
        }
    }
}

static DB_POOL: Lazy<ConnectionPool> = Lazy::new(|| ConnectionPool::new(10));

fn connection_pool() {
    if let Some(conn) = DB_POOL.get_connection() {
        let result = conn.execute("SELECT * FROM users");
        println!("查询结果: {}", result);
        
        // 归还连接
        DB_POOL.return_connection(conn);
    } else {
        println!("无法获取数据库连接");
    }
}
```

## 最佳实践

### 1. 选择合适的类型

```rust
use once_cell::sync::{OnceCell, Lazy};

// 使用 Lazy 当初始化逻辑固定时
static COMPUTED_VALUE: Lazy<String> = Lazy::new(|| {
    expensive_computation()
});

// 使用 OnceCell 当需要运行时决定初始化时
static RUNTIME_CONFIG: OnceCell<String> = OnceCell::new();

fn type_selection() {
    // 运行时设置配置
    let config = if std::env::var("PRODUCTION").is_ok() {
        "prod_config"
    } else {
        "dev_config"
    };
    
    RUNTIME_CONFIG.set(config.to_string()).unwrap();
}

fn expensive_computation() -> String {
    "计算结果".to_string()
}
```

### 2. 错误处理

```rust
use once_cell::sync::OnceCell;

static FALLIBLE_RESOURCE: OnceCell<Result<String, &'static str>> = OnceCell::new();

fn error_handling_best_practice() {
    let resource = FALLIBLE_RESOURCE.get_or_init(|| {
        if std::env::var("FAIL").is_ok() {
            Err("初始化失败")
        } else {
            Ok("初始化成功".to_string())
        }
    });
    
    match resource {
        Ok(value) => println!("资源: {}", value),
        Err(e) => println!("错误: {}", e),
    }
}
```

### 3. 内存管理

```rust
use once_cell::sync::Lazy;

// 对于大型数据，使用 Box 避免栈溢出
static LARGE_ARRAY: Lazy<Box<[u8; 1_000_000]>> = Lazy::new(|| {
    Box::new([0u8; 1_000_000])
});

// 对于共享数据，使用 Arc
static SHARED_DATA: Lazy<Arc<Vec<String>>> = Lazy::new(|| {
    Arc::new(vec!["shared".to_string(), "data".to_string()])
});

fn memory_management() {
    // 访问大型数组
    println!("Large array length: {}", LARGE_ARRAY.len());
    
    // 共享数据
    let data_ref = SHARED_DATA.clone();
    println!("Shared data: {:?}", data_ref);
}
```

### 4. 测试友好设计

```rust
use once_cell::sync::OnceCell;

static TEST_CONFIG: OnceCell<String> = OnceCell::new();

fn get_config() -> &'static String {
    TEST_CONFIG.get_or_init(|| {
        std::env::var("CONFIG").unwrap_or_else(|_| "default".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_with_custom_config() {
        // 在测试中设置特定配置
        TEST_CONFIG.set("test_config".to_string()).unwrap();
        assert_eq!(get_config(), "test_config");
    }
}

fn testing_friendly_design() {
    println!("配置: {}", get_config());
}
```

## 标准库集成

### std::sync::OnceLock (Rust 1.70+)

```rust
// 使用标准库的 OnceLock (如果可用)
#[cfg(feature = "std")]
use std::sync::OnceLock;

#[cfg(feature = "std")]
static STD_ONCE_LOCK: OnceLock<String> = OnceLock::new();

// 使用 once_cell 作为 fallback
#[cfg(not(feature = "std"))]
use once_cell::sync::OnceCell as OnceLock;

#[cfg(not(feature = "std"))]
static STD_ONCE_LOCK: OnceLock<String> = OnceLock::new();

fn standard_library_integration() {
    STD_ONCE_LOCK.set("标准库值".to_string()).unwrap();
    println!("值: {}", STD_ONCE_LOCK.get().unwrap());
}
```

### 迁移到标准库

```rust
// 当前使用 once_cell
use once_cell::sync::{Lazy, OnceCell};

static CURRENT_LAZY: Lazy<String> = Lazy::new(|| "value".to_string());
static CURRENT_ONCE: OnceCell<String> = OnceCell::new();

// 未来迁移到标准库 (当可用时)
// use std::sync::{LazyLock, OnceLock};
// static FUTURE_LAZY: LazyLock<String> = LazyLock::new(|| "value".to_string());
// static FUTURE_ONCE: OnceLock<String> = OnceLock::new();

fn migration_preparation() {
    println!("Current lazy: {}", &*CURRENT_LAZY);
    CURRENT_ONCE.set("current".to_string()).unwrap();
    println!("Current once: {}", CURRENT_ONCE.get().unwrap());
}
```

## 总结

Once_cell 是一个功能强大且灵活的库，提供了比 lazy_static 更现代的 API 设计。通过本教程，您应该能够：

1. 理解 OnceCell 和 Lazy 的区别和用途
2. 正确选择同步和非同步版本
3. 实现高效的延迟初始化和单例模式
4. 处理错误和资源管理
5. 为将来迁移到标准库做准备

关键要点：
- OnceCell 用于手动设置的单次赋值
- Lazy 用于自动初始化的延迟值
- 注意线程安全需求选择合适版本
- 合理处理初始化错误
- 考虑内存使用和性能影响

Once_cell 的设计理念已经影响了 Rust 标准库的发展，掌握它不仅有助于当前项目，也为未来使用标准库的类似功能做好准备。