# Serde JSON 1.0.140 中文教程

## 简介

Serde JSON 是 Rust 中最流行的 JSON 序列化和反序列化库。它基于 Serde 框架，提供了高性能、类型安全的 JSON 处理能力。无论是解析 JSON 数据、生成 JSON 输出，还是在 Rust 结构体和 JSON 之间进行转换，Serde JSON 都是首选解决方案。

## 核心特性

- 🚀 高性能序列化和反序列化
- 🔒 类型安全的 JSON 处理
- 🎯 零拷贝解析（支持借用数据）
- 🔧 灵活的自定义序列化
- 📊 支持流式处理
- 🌐 完整的 JSON 规范支持
- 📝 丰富的错误处理

## 基本用法

### 1. 序列化和反序列化

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

fn main() -> Result<(), serde_json::Error> {
    let person = Person {
        name: "张三".to_string(),
        age: 25,
        email: "zhangsan@example.com".to_string(),
        active: true,
    };
    
    // 序列化为 JSON 字符串
    let json_str = serde_json::to_string(&person)?;
    println!("JSON: {}", json_str);
    
    // 格式化的 JSON（美化输出）
    let json_pretty = serde_json::to_string_pretty(&person)?;
    println!("Pretty JSON:\n{}", json_pretty);
    
    // 反序列化
    let person_from_json: Person = serde_json::from_str(&json_str)?;
    println!("从 JSON 解析: {:?}", person_from_json);
    
    Ok(())
}
```

### 2. 处理 JSON 值

```rust
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 JSON 值
    let data = json!({
        "name": "李四",
        "age": 30,
        "hobbies": ["读书", "游泳", "编程"],
        "address": {
            "city": "北京",
            "street": "长安街"
        }
    });
    
    // 访问 JSON 值
    println!("姓名: {}", data["name"]);
    println!("年龄: {}", data["age"]);
    println!("第一个爱好: {}", data["hobbies"][0]);
    println!("城市: {}", data["address"]["city"]);
    
    // 修改 JSON 值
    let mut mutable_data = data.clone();
    mutable_data["age"] = json!(31);
    mutable_data["hobbies"].as_array_mut().unwrap().push(json!("旅行"));
    
    println!("修改后的数据: {}", serde_json::to_string_pretty(&mutable_data)?);
    
    // 类型转换
    let age: u32 = data["age"].as_u64().unwrap() as u32;
    let name: &str = data["name"].as_str().unwrap();
    let hobbies: Vec<&str> = data["hobbies"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    
    println!("年龄: {}, 姓名: {}, 爱好: {:?}", age, name, hobbies);
    
    Ok(())
}
```

### 3. 从不同源读取 JSON

```rust
use serde_json::{from_str, from_reader, from_slice};
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct Config {
    database_url: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从字符串解析
    let json_str = r#"
    {
        "database_url": "postgresql://localhost/mydb",
        "port": 8080,
        "debug": true
    }
    "#;
    
    let config: Config = from_str(json_str)?;
    println!("从字符串解析: {:?}", config);
    
    // 从文件读取
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let config: Config = from_reader(reader)?;
    println!("从文件解析: {:?}", config);
    
    // 从字节数组解析
    let json_bytes = br#"
    {
        "database_url": "postgresql://localhost/mydb",
        "port": 3000,
        "debug": false
    }
    "#;
    
    let config: Config = from_slice(json_bytes)?;
    println!("从字节数组解析: {:?}", config);
    
    Ok(())
}
```

## 高级特性

### 1. 自定义序列化

```rust
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde_json;
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    created_at: DateTime<Utc>,
    password_hash: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut state = serializer.serialize_struct("User", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("created_at", &self.created_at.to_rfc3339())?;
        // 不序列化密码哈希
        state.end()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor, MapAccess};
        use std::fmt;
        
        struct UserVisitor;
        
        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a User object")
            }
            
            fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut created_at = None;
                
                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => id = Some(map.next_value()?),
                        "name" => name = Some(map.next_value()?),
                        "created_at" => {
                            let date_str: String = map.next_value()?;
                            created_at = Some(
                                DateTime::parse_from_rfc3339(&date_str)
                                    .map_err(de::Error::custom)?
                                    .with_timezone(&Utc)
                            );
                        }
                        _ => { map.next_value::<serde_json::Value>()?; }
                    }
                }
                
                Ok(User {
                    id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                    name: name.ok_or_else(|| de::Error::missing_field("name"))?,
                    created_at: created_at.ok_or_else(|| de::Error::missing_field("created_at"))?,
                    password_hash: "".to_string(), // 默认值
                })
            }
        }
        
        deserializer.deserialize_struct("User", &["id", "name", "created_at"], UserVisitor)
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        created_at: Utc::now(),
        password_hash: "secret-hash".to_string(),
    };
    
    let json = serde_json::to_string_pretty(&user)?;
    println!("序列化结果:\n{}", json);
    
    let user_from_json: User = serde_json::from_str(&json)?;
    println!("反序列化结果: {:?}", user_from_json);
    
    Ok(())
}
```

### 2. 字段重命名和跳过

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "status_code")]
    status: u16,
    
    #[serde(rename = "data")]
    content: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    
    #[serde(skip)]
    internal_id: u64,
    
    #[serde(default)]
    retry_count: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let response = ApiResponse {
        status: 200,
        content: "Success".to_string(),
        error_message: None,
        internal_id: 12345,
        retry_count: 0,
    };
    
    let json = serde_json::to_string_pretty(&response)?;
    println!("序列化结果:\n{}", json);
    
    // 注意：internal_id 不会被序列化
    // error_message 为 None 时不会被序列化
    
    let json_input = r#"
    {
        "status_code": 404,
        "data": "Not Found",
        "error_message": "Resource not found"
    }
    "#;
    
    let response: ApiResponse = serde_json::from_str(json_input)?;
    println!("反序列化结果: {:?}", response);
    
    Ok(())
}
```

### 3. 枚举序列化

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Event {
    #[serde(rename = "user_login")]
    UserLogin { user_id: u64, timestamp: String },
    
    #[serde(rename = "user_logout")]
    UserLogout { user_id: u64, session_duration: u32 },
    
    #[serde(rename = "file_upload")]
    FileUpload { 
        user_id: u64, 
        filename: String,
        file_size: u64 
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
}

fn main() -> Result<(), serde_json::Error> {
    // 标记枚举
    let events = vec![
        Event::UserLogin {
            user_id: 123,
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        },
        Event::FileUpload {
            user_id: 123,
            filename: "document.pdf".to_string(),
            file_size: 1024000,
        },
    ];
    
    for event in &events {
        let json = serde_json::to_string_pretty(event)?;
        println!("事件 JSON:\n{}\n", json);
    }
    
    // 无标记枚举
    let values = vec![
        Value::String("Hello".to_string()),
        Value::Number(42.0),
        Value::Boolean(true),
        Value::Array(vec![
            Value::String("item1".to_string()),
            Value::Number(123.0),
        ]),
    ];
    
    for value in &values {
        let json = serde_json::to_string(value)?;
        println!("值 JSON: {}", json);
    }
    
    Ok(())
}
```

## 实际应用示例

### 1. Web API 客户端

```rust
use serde::{Deserialize, Serialize};
use serde_json;
use reqwest;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    error: String,
    message: String,
}

struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
    
    async fn get_user(&self, user_id: u32) -> Result<User, Box<dyn Error>> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let user: User = response.json().await?;
            Ok(user)
        } else {
            let error: ApiError = response.json().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("API Error: {}", error.message)
            )))
        }
    }
    
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, Box<dyn Error>> {
        let url = format!("{}/users", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let user: User = response.json().await?;
            Ok(user)
        } else {
            let error: ApiError = response.json().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("API Error: {}", error.message)
            )))
        }
    }
    
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let url = format!("{}/users", self.base_url);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let users: Vec<User> = response.json().await?;
            Ok(users)
        } else {
            let error: ApiError = response.json().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("API Error: {}", error.message)
            )))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ApiClient::new("https://jsonplaceholder.typicode.com".to_string());
    
    // 获取用户
    let user = client.get_user(1).await?;
    println!("用户: {:?}", user);
    
    // 创建用户
    let new_user_request = CreateUserRequest {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        username: "zhangsan".to_string(),
    };
    
    let created_user = client.create_user(new_user_request).await?;
    println!("创建的用户: {:?}", created_user);
    
    // 列出用户
    let users = client.list_users().await?;
    println!("用户列表: {:?}", users);
    
    Ok(())
}
```

### 2. 配置文件管理

```rust
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    max_connections: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: u32,
    max_request_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LogConfig {
    level: String,
    file: Option<String>,
    max_size: u64,
    rotation: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppConfig {
    debug: bool,
    environment: String,
    database: DatabaseConfig,
    server: ServerConfig,
    logging: LogConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            debug: false,
            environment: "production".to_string(),
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "password".to_string(),
                database: "myapp".to_string(),
                max_connections: 10,
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
                max_request_size: 1024 * 1024, // 1MB
            },
            logging: LogConfig {
                level: "info".to_string(),
                file: None,
                max_size: 10 * 1024 * 1024, // 10MB
                rotation: true,
            },
        }
    }
}

impl AppConfig {
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    fn merge_with_env(&mut self) {
        use std::env;
        
        if let Ok(debug) = env::var("DEBUG") {
            self.debug = debug.parse().unwrap_or(false);
        }
        
        if let Ok(env) = env::var("ENVIRONMENT") {
            self.environment = env;
        }
        
        if let Ok(db_host) = env::var("DB_HOST") {
            self.database.host = db_host;
        }
        
        if let Ok(db_port) = env::var("DB_PORT") {
            self.database.port = db_port.parse().unwrap_or(5432);
        }
        
        if let Ok(server_port) = env::var("SERVER_PORT") {
            self.server.port = server_port.parse().unwrap_or(8080);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建默认配置
    let default_config = AppConfig::default();
    
    // 保存默认配置到文件
    default_config.save_to_file("config.json")?;
    println!("默认配置已保存到 config.json");
    
    // 从文件加载配置
    let mut config = AppConfig::load_from_file("config.json")?;
    println!("从文件加载的配置: {:?}", config);
    
    // 合并环境变量
    config.merge_with_env();
    println!("合并环境变量后的配置: {:?}", config);
    
    // 修改配置
    config.debug = true;
    config.server.workers = 8;
    config.logging.level = "debug".to_string();
    
    // 保存修改后的配置
    config.save_to_file("config.json")?;
    println!("修改后的配置已保存");
    
    Ok(())
}
```

### 3. 数据处理和转换

```rust
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct RawData {
    timestamp: String,
    user_id: u64,
    action: String,
    details: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessedData {
    timestamp: chrono::DateTime<chrono::Utc>,
    user_id: u64,
    action_type: ActionType,
    metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ActionType {
    Login,
    Logout,
    Purchase,
    View,
    Unknown,
}

impl From<&str> for ActionType {
    fn from(s: &str) -> Self {
        match s {
            "login" => ActionType::Login,
            "logout" => ActionType::Logout,
            "purchase" => ActionType::Purchase,
            "view" => ActionType::View,
            _ => ActionType::Unknown,
        }
    }
}

struct DataProcessor;

impl DataProcessor {
    fn process_raw_data(&self, raw_data: &RawData) -> Result<ProcessedData, Box<dyn std::error::Error>> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&raw_data.timestamp)?
            .with_timezone(&chrono::Utc);
        
        let action_type = ActionType::from(raw_data.action.as_str());
        
        let mut metadata = HashMap::new();
        
        // 从 details 中提取元数据
        if let Value::Object(details) = &raw_data.details {
            for (key, value) in details {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                metadata.insert(key.clone(), value_str);
            }
        }
        
        Ok(ProcessedData {
            timestamp,
            user_id: raw_data.user_id,
            action_type,
            metadata,
        })
    }
    
    fn batch_process(&self, raw_data_list: &[RawData]) -> Vec<ProcessedData> {
        raw_data_list
            .iter()
            .filter_map(|raw_data| self.process_raw_data(raw_data).ok())
            .collect()
    }
    
    fn aggregate_by_action(&self, processed_data: &[ProcessedData]) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        
        for data in processed_data {
            let action_name = format!("{:?}", data.action_type);
            *counts.entry(action_name).or_insert(0) += 1;
        }
        
        counts
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_json = r#"
    [
        {
            "timestamp": "2023-01-01T12:00:00Z",
            "user_id": 123,
            "action": "login",
            "details": {
                "ip": "192.168.1.1",
                "user_agent": "Mozilla/5.0"
            }
        },
        {
            "timestamp": "2023-01-01T12:05:00Z",
            "user_id": 123,
            "action": "view",
            "details": {
                "page": "/dashboard",
                "duration": 30
            }
        },
        {
            "timestamp": "2023-01-01T12:10:00Z",
            "user_id": 456,
            "action": "purchase",
            "details": {
                "product_id": "prod_123",
                "amount": 99.99,
                "currency": "USD"
            }
        }
    ]
    "#;
    
    // 解析原始数据
    let raw_data: Vec<RawData> = serde_json::from_str(raw_json)?;
    println!("原始数据: {:#?}", raw_data);
    
    // 处理数据
    let processor = DataProcessor;
    let processed_data = processor.batch_process(&raw_data);
    println!("处理后的数据: {:#?}", processed_data);
    
    // 聚合统计
    let action_counts = processor.aggregate_by_action(&processed_data);
    println!("操作统计: {:?}", action_counts);
    
    // 输出为 JSON
    let output_json = serde_json::to_string_pretty(&processed_data)?;
    println!("输出 JSON:\n{}", output_json);
    
    Ok(())
}
```

## 性能优化

### 1. 零拷贝反序列化

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct BorrowedData<'a> {
    #[serde(borrow)]
    name: &'a str,
    age: u32,
    #[serde(borrow)]
    email: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct OwnedData {
    name: String,
    age: u32,
    email: String,
}

fn main() -> Result<(), serde_json::Error> {
    let json_str = r#"
    {
        "name": "Alice",
        "age": 25,
        "email": "alice@example.com"
    }
    "#;
    
    // 零拷贝反序列化（借用 JSON 字符串中的数据）
    let borrowed: BorrowedData = serde_json::from_str(json_str)?;
    println!("借用数据: {:?}", borrowed);
    
    // 拥有数据的反序列化
    let owned: OwnedData = serde_json::from_str(json_str)?;
    println!("拥有数据: {:?}", owned);
    
    Ok(())
}
```

### 2. 流式处理

```rust
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, StreamDeserializer};
use std::io::{self, BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn process_large_json_file() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = r#"
    {"timestamp": "2023-01-01T12:00:00Z", "level": "INFO", "message": "Application started"}
    {"timestamp": "2023-01-01T12:01:00Z", "level": "DEBUG", "message": "Processing request"}
    {"timestamp": "2023-01-01T12:02:00Z", "level": "ERROR", "message": "Database connection failed"}
    {"timestamp": "2023-01-01T12:03:00Z", "level": "INFO", "message": "Retrying database connection"}
    "#;
    
    let reader = BufReader::new(json_data.as_bytes());
    
    // 逐行处理 JSON 数据
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let entry: LogEntry = serde_json::from_str(&line)?;
            
            // 处理每个日志条目
            match entry.level.as_str() {
                "ERROR" => println!("🚨 错误: {}", entry.message),
                "WARN" => println!("⚠️ 警告: {}", entry.message),
                "INFO" => println!("ℹ️ 信息: {}", entry.message),
                "DEBUG" => println!("🔍 调试: {}", entry.message),
                _ => println!("📝 日志: {}", entry.message),
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_large_json_file()?;
    Ok(())
}
```

### 3. 自定义序列化器

```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;

struct FastStruct {
    id: u64,
    name: String,
    active: bool,
}

impl Serialize for FastStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 手动序列化，避免反射开销
        let mut state = serializer.serialize_struct("FastStruct", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("active", &self.active)?;
        state.end()
    }
}

fn main() -> Result<(), serde_json::Error> {
    let data = FastStruct {
        id: 1,
        name: "Test".to_string(),
        active: true,
    };
    
    let json = serde_json::to_string(&data)?;
    println!("Fast serialization: {}", json);
    
    Ok(())
}
```

## 错误处理

### 1. 详细的错误信息

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn handle_json_errors() {
    let invalid_json = r#"
    {
        "name": "Alice",
        "age": "not_a_number",
        "email": "alice@example.com"
    }
    "#;
    
    match serde_json::from_str::<Person>(invalid_json) {
        Ok(person) => println!("解析成功: {:?}", person),
        Err(e) => {
            println!("解析失败:");
            println!("错误: {}", e);
            println!("行号: {}", e.line());
            println!("列号: {}", e.column());
            
            // 获取详细错误信息
            match e.classify() {
                serde_json::error::Category::Io => println!("IO 错误"),
                serde_json::error::Category::Syntax => println!("语法错误"),
                serde_json::error::Category::Data => println!("数据错误"),
                serde_json::error::Category::Eof => println!("意外结束"),
            }
        }
    }
}

fn main() {
    handle_json_errors();
}
```

### 2. 容错的反序列化

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct FlexiblePerson {
    name: String,
    
    #[serde(default)]
    age: u32,
    
    #[serde(default)]
    email: String,
    
    #[serde(default)]
    active: bool,
}

impl Default for FlexiblePerson {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            age: 0,
            email: "unknown@example.com".to_string(),
            active: false,
        }
    }
}

fn main() -> Result<(), serde_json::Error> {
    // 不完整的 JSON 数据
    let incomplete_json = r#"
    {
        "name": "Bob"
    }
    "#;
    
    let person: FlexiblePerson = serde_json::from_str(incomplete_json)?;
    println!("解析结果: {:?}", person);
    
    // 部分字段错误的 JSON
    let partial_json = r#"
    {
        "name": "Charlie",
        "age": 30,
        "email": "charlie@example.com"
    }
    "#;
    
    let person: FlexiblePerson = serde_json::from_str(partial_json)?;
    println!("解析结果: {:?}", person);
    
    Ok(())
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.140"
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
```

## 总结

Serde JSON 是 Rust 生态系统中功能最强大、性能最优秀的 JSON 处理库。它提供了类型安全、高性能的序列化和反序列化功能，支持丰富的自定义选项和优化特性。

主要特性：
- 🚀 高性能序列化/反序列化
- 🔒 类型安全的 JSON 处理
- 🎯 零拷贝解析支持
- 🔧 丰富的自定义选项
- 📊 流式处理支持
- 🌐 完整的 JSON 规范支持
- 📝 详细的错误处理

无论是构建 Web API、处理配置文件，还是数据转换，Serde JSON 都是 Rust 开发的首选工具。