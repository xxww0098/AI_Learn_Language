# UUID 1.17.0 中文教程

## 简介

UUID（通用唯一识别码）是一个用于生成和解析 UUID 的 Rust 库。UUID 是 128 位的标识符，能够在不依赖中央协调的情况下保证全局唯一性。这个库提供了多种 UUID 版本的生成和操作功能，广泛应用于分布式系统、数据库主键、会话标识等场景。

## 核心概念

### UUID 版本

- **Version 1**: 基于时间戳和 MAC 地址
- **Version 2**: 基于时间戳、MAC 地址和本地域
- **Version 3**: 基于命名空间和名称的 MD5 哈希
- **Version 4**: 随机或伪随机生成
- **Version 5**: 基于命名空间和名称的 SHA-1 哈希
- **Version 6**: 基于时间戳的重排序版本
- **Version 7**: 基于 Unix 时间戳的时间有序版本
- **Version 8**: 自定义格式版本

### UUID 格式

标准 UUID 格式：`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
- 32 个十六进制字符
- 5 个用连字符分隔的组
- 总长度 36 个字符

## 基本用法

### 1. 生成 UUID

```rust
use uuid::Uuid;

fn main() {
    // 生成随机 UUID (Version 4)
    let uuid_v4 = Uuid::new_v4();
    println!("UUID v4: {}", uuid_v4);
    
    // 生成多个 UUID
    for i in 0..5 {
        let uuid = Uuid::new_v4();
        println!("UUID {}: {}", i + 1, uuid);
    }
    
    // 生成 nil UUID (全零)
    let nil_uuid = Uuid::nil();
    println!("Nil UUID: {}", nil_uuid);
    
    // 生成 max UUID (全 1)
    let max_uuid = Uuid::max();
    println!("Max UUID: {}", max_uuid);
}
```

### 2. 解析和验证 UUID

```rust
use uuid::Uuid;

fn main() -> Result<(), uuid::Error> {
    // 从字符串解析 UUID
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = Uuid::parse_str(uuid_str)?;
    println!("解析的 UUID: {}", uuid);
    
    // 验证 UUID 格式
    let valid_uuid = "123e4567-e89b-12d3-a456-426614174000";
    let invalid_uuid = "not-a-uuid";
    
    match Uuid::parse_str(valid_uuid) {
        Ok(uuid) => println!("有效的 UUID: {}", uuid),
        Err(e) => println!("无效的 UUID: {}", e),
    }
    
    match Uuid::parse_str(invalid_uuid) {
        Ok(uuid) => println!("有效的 UUID: {}", uuid),
        Err(e) => println!("无效的 UUID: {}", e),
    }
    
    // 从字节数组解析
    let bytes = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];
    
    let uuid_from_bytes = Uuid::from_bytes(bytes);
    println!("从字节数组创建的 UUID: {}", uuid_from_bytes);
    
    Ok(())
}
```

### 3. UUID 格式转换

```rust
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    
    // 不同格式的字符串输出
    println!("标准格式: {}", uuid);
    println!("简单格式: {}", uuid.simple());
    println!("大写格式: {}", uuid.to_string().to_uppercase());
    println!("大括号格式: {{{}}}", uuid);
    println!("URN 格式: {}", uuid.urn());
    
    // 转换为字节数组
    let bytes = uuid.as_bytes();
    println!("字节数组: {:?}", bytes);
    
    // 转换为 u128
    let as_u128 = uuid.as_u128();
    println!("u128: {}", as_u128);
    
    // 获取各个字段
    let (time_low, time_mid, time_hi_and_version, clock_seq_hi_and_reserved, clock_seq_low, node) = uuid.as_fields();
    println!("时间低位: {:08x}", time_low);
    println!("时间中位: {:04x}", time_mid);
    println!("时间高位和版本: {:04x}", time_hi_and_version);
    println!("时钟序列高位: {:02x}", clock_seq_hi_and_reserved);
    println!("时钟序列低位: {:02x}", clock_seq_low);
    println!("节点: {:012x}", u64::from_be_bytes([0, 0, node[0], node[1], node[2], node[3], node[4], node[5]]));
}
```

## 不同版本的 UUID

### 1. Version 1 - 基于时间戳

```rust
use uuid::Uuid;

fn main() {
    // 需要启用 "v1" 特性
    #[cfg(feature = "v1")]
    {
        use uuid::timestamp::{Timestamp, NoContext};
        
        // 生成基于时间戳的 UUID
        let context = NoContext;
        let timestamp = Timestamp::now(context);
        let uuid_v1 = Uuid::new_v1(timestamp, &[1, 2, 3, 4, 5, 6]).unwrap();
        println!("UUID v1: {}", uuid_v1);
        
        // 检查版本
        println!("UUID 版本: {}", uuid_v1.get_version_num());
        
        // 提取时间戳
        if let Some(timestamp) = uuid_v1.get_timestamp() {
            println!("时间戳: {:?}", timestamp);
        }
    }
}
```

### 2. Version 3 和 5 - 基于命名空间

```rust
use uuid::Uuid;

fn main() {
    // Version 3 (MD5)
    let namespace = Uuid::NAMESPACE_DNS;
    let name = "example.com";
    let uuid_v3 = Uuid::new_v3(&namespace, name.as_bytes());
    println!("UUID v3: {}", uuid_v3);
    
    // Version 5 (SHA-1)
    let uuid_v5 = Uuid::new_v5(&namespace, name.as_bytes());
    println!("UUID v5: {}", uuid_v5);
    
    // 相同的命名空间和名称会产生相同的 UUID
    let uuid_v3_duplicate = Uuid::new_v3(&namespace, name.as_bytes());
    let uuid_v5_duplicate = Uuid::new_v5(&namespace, name.as_bytes());
    
    println!("V3 相同: {}", uuid_v3 == uuid_v3_duplicate);
    println!("V5 相同: {}", uuid_v5 == uuid_v5_duplicate);
    
    // 不同的命名空间
    let url_uuid_v5 = Uuid::new_v5(&Uuid::NAMESPACE_URL, "https://example.com".as_bytes());
    println!("URL 命名空间 UUID v5: {}", url_uuid_v5);
    
    // 自定义命名空间
    let custom_namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
    let custom_uuid_v5 = Uuid::new_v5(&custom_namespace, "custom-name".as_bytes());
    println!("自定义命名空间 UUID v5: {}", custom_uuid_v5);
}
```

### 3. Version 4 - 随机生成

```rust
use uuid::Uuid;

fn main() {
    // 生成随机 UUID
    let uuid_v4 = Uuid::new_v4();
    println!("UUID v4: {}", uuid_v4);
    
    // 批量生成
    let uuids: Vec<Uuid> = (0..10).map(|_| Uuid::new_v4()).collect();
    for (i, uuid) in uuids.iter().enumerate() {
        println!("UUID {}: {}", i + 1, uuid);
    }
    
    // 检查重复（在实际应用中几乎不可能）
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    println!("UUID 相同: {}", uuid1 == uuid2);
}
```

### 4. Version 6 和 7 - 基于时间戳（有序）

```rust
use uuid::Uuid;

fn main() {
    // 需要启用 "v6" 和 "v7" 特性
    #[cfg(feature = "v6")]
    {
        use uuid::timestamp::{Timestamp, NoContext};
        
        // Version 6 - 重排序的基于时间戳的 UUID
        let context = NoContext;
        let timestamp = Timestamp::now(context);
        let uuid_v6 = Uuid::new_v6(timestamp, &[1, 2, 3, 4, 5, 6]).unwrap();
        println!("UUID v6: {}", uuid_v6);
    }
    
    #[cfg(feature = "v7")]
    {
        use uuid::timestamp::{Timestamp, NoContext};
        
        // Version 7 - 基于 Unix 时间戳的有序 UUID
        let context = NoContext;
        let timestamp = Timestamp::now(context);
        let uuid_v7 = Uuid::new_v7(timestamp).unwrap();
        println!("UUID v7: {}", uuid_v7);
        
        // 生成多个 v7 UUID 显示时间排序
        let mut v7_uuids = Vec::new();
        for _ in 0..5 {
            std::thread::sleep(std::time::Duration::from_millis(1));
            let timestamp = Timestamp::now(context);
            v7_uuids.push(Uuid::new_v7(timestamp).unwrap());
        }
        
        println!("时间有序的 UUID v7:");
        for uuid in v7_uuids {
            println!("{}", uuid);
        }
    }
}
```

## 实际应用示例

### 1. 数据库主键生成器

```rust
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
struct Order {
    id: Uuid,
    user_id: Uuid,
    total: f64,
    status: OrderStatus,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

struct Database {
    users: HashMap<Uuid, User>,
    orders: HashMap<Uuid, Order>,
}

impl Database {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            orders: HashMap::new(),
        }
    }
    
    fn create_user(&mut self, name: String, email: String) -> Uuid {
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            name,
            email,
            created_at: chrono::Utc::now(),
        };
        
        self.users.insert(user_id, user);
        user_id
    }
    
    fn create_order(&mut self, user_id: Uuid, total: f64) -> Result<Uuid, String> {
        if !self.users.contains_key(&user_id) {
            return Err("用户不存在".to_string());
        }
        
        let order_id = Uuid::new_v4();
        let order = Order {
            id: order_id,
            user_id,
            total,
            status: OrderStatus::Pending,
            created_at: chrono::Utc::now(),
        };
        
        self.orders.insert(order_id, order);
        Ok(order_id)
    }
    
    fn get_user(&self, user_id: &Uuid) -> Option<&User> {
        self.users.get(user_id)
    }
    
    fn get_order(&self, order_id: &Uuid) -> Option<&Order> {
        self.orders.get(order_id)
    }
    
    fn get_user_orders(&self, user_id: &Uuid) -> Vec<&Order> {
        self.orders
            .values()
            .filter(|order| order.user_id == *user_id)
            .collect()
    }
    
    fn update_order_status(&mut self, order_id: &Uuid, status: OrderStatus) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.status = status;
            Ok(())
        } else {
            Err("订单不存在".to_string())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::new();
    
    // 创建用户
    let user_id = db.create_user("张三".to_string(), "zhangsan@example.com".to_string());
    println!("创建用户 ID: {}", user_id);
    
    // 创建订单
    let order_id = db.create_order(user_id, 99.99)?;
    println!("创建订单 ID: {}", order_id);
    
    // 查询用户
    if let Some(user) = db.get_user(&user_id) {
        println!("用户信息: {:?}", user);
    }
    
    // 查询订单
    if let Some(order) = db.get_order(&order_id) {
        println!("订单信息: {:?}", order);
    }
    
    // 更新订单状态
    db.update_order_status(&order_id, OrderStatus::Processing)?;
    println!("订单状态已更新");
    
    // 获取用户的所有订单
    let user_orders = db.get_user_orders(&user_id);
    println!("用户的订单: {:?}", user_orders);
    
    Ok(())
}
```

### 2. 会话管理系统

```rust
use uuid::Uuid;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
struct Session {
    id: Uuid,
    user_id: Uuid,
    created_at: SystemTime,
    last_accessed: SystemTime,
    expires_at: SystemTime,
    data: HashMap<String, String>,
}

impl Session {
    fn new(user_id: Uuid, ttl: Duration) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: now,
            last_accessed: now,
            expires_at: now + ttl,
            data: HashMap::new(),
        }
    }
    
    fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
    
    fn refresh(&mut self, ttl: Duration) {
        let now = SystemTime::now();
        self.last_accessed = now;
        self.expires_at = now + ttl;
    }
    
    fn set_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    fn get_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

struct SessionManager {
    sessions: HashMap<Uuid, Session>,
    default_ttl: Duration,
}

impl SessionManager {
    fn new(default_ttl: Duration) -> Self {
        Self {
            sessions: HashMap::new(),
            default_ttl,
        }
    }
    
    fn create_session(&mut self, user_id: Uuid) -> Uuid {
        let session = Session::new(user_id, self.default_ttl);
        let session_id = session.id;
        
        self.sessions.insert(session_id, session);
        session_id
    }
    
    fn get_session(&mut self, session_id: &Uuid) -> Option<&mut Session> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if session.is_expired() {
                self.sessions.remove(session_id);
                None
            } else {
                session.refresh(self.default_ttl);
                Some(session)
            }
        } else {
            None
        }
    }
    
    fn destroy_session(&mut self, session_id: &Uuid) -> bool {
        self.sessions.remove(session_id).is_some()
    }
    
    fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        self.sessions.retain(|_, session| now <= session.expires_at);
    }
    
    fn get_user_sessions(&self, user_id: &Uuid) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|session| session.user_id == *user_id && !session.is_expired())
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session_manager = SessionManager::new(Duration::from_secs(3600)); // 1小时
    
    // 创建用户会话
    let user_id = Uuid::new_v4();
    let session_id = session_manager.create_session(user_id);
    println!("创建会话 ID: {}", session_id);
    
    // 获取会话并设置数据
    if let Some(session) = session_manager.get_session(&session_id) {
        session.set_data("username".to_string(), "张三".to_string());
        session.set_data("role".to_string(), "admin".to_string());
        println!("会话数据已设置");
    }
    
    // 获取会话数据
    if let Some(session) = session_manager.get_session(&session_id) {
        if let Some(username) = session.get_data("username") {
            println!("用户名: {}", username);
        }
        if let Some(role) = session.get_data("role") {
            println!("角色: {}", role);
        }
    }
    
    // 模拟多个会话
    let mut session_ids = Vec::new();
    for i in 0..5 {
        let session_id = session_manager.create_session(user_id);
        session_ids.push(session_id);
        println!("创建会话 {}: {}", i + 1, session_id);
    }
    
    // 获取用户的所有会话
    let user_sessions = session_manager.get_user_sessions(&user_id);
    println!("用户的会话数量: {}", user_sessions.len());
    
    // 清理过期会话
    session_manager.cleanup_expired();
    
    // 销毁会话
    if session_manager.destroy_session(&session_id) {
        println!("会话已销毁");
    }
    
    Ok(())
}
```

### 3. 分布式系统中的请求追踪

```rust
use uuid::Uuid;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
struct TraceContext {
    trace_id: Uuid,
    span_id: Uuid,
    parent_span_id: Option<Uuid>,
    baggage: HashMap<String, String>,
}

impl TraceContext {
    fn new() -> Self {
        Self {
            trace_id: Uuid::new_v4(),
            span_id: Uuid::new_v4(),
            parent_span_id: None,
            baggage: HashMap::new(),
        }
    }
    
    fn child_span(&self) -> Self {
        Self {
            trace_id: self.trace_id,
            span_id: Uuid::new_v4(),
            parent_span_id: Some(self.span_id),
            baggage: self.baggage.clone(),
        }
    }
    
    fn set_baggage(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }
    
    fn get_baggage(&self, key: &str) -> Option<&String> {
        self.baggage.get(key)
    }
}

#[derive(Debug, Clone)]
struct Span {
    span_id: Uuid,
    trace_id: Uuid,
    parent_span_id: Option<Uuid>,
    operation_name: String,
    start_time: SystemTime,
    end_time: Option<SystemTime>,
    tags: HashMap<String, String>,
    logs: Vec<LogEntry>,
}

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: SystemTime,
    level: LogLevel,
    message: String,
    fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl Span {
    fn new(context: &TraceContext, operation_name: String) -> Self {
        Self {
            span_id: context.span_id,
            trace_id: context.trace_id,
            parent_span_id: context.parent_span_id,
            operation_name,
            start_time: SystemTime::now(),
            end_time: None,
            tags: HashMap::new(),
            logs: Vec::new(),
        }
    }
    
    fn set_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }
    
    fn log(&mut self, level: LogLevel, message: String) {
        let log_entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            message,
            fields: HashMap::new(),
        };
        self.logs.push(log_entry);
    }
    
    fn finish(&mut self) {
        self.end_time = Some(SystemTime::now());
    }
    
    fn duration(&self) -> Option<Duration> {
        self.end_time.map(|end| end.duration_since(self.start_time).unwrap_or(Duration::ZERO))
    }
}

struct Tracer {
    spans: HashMap<Uuid, Span>,
}

impl Tracer {
    fn new() -> Self {
        Self {
            spans: HashMap::new(),
        }
    }
    
    fn start_span(&mut self, context: &TraceContext, operation_name: String) -> Uuid {
        let span = Span::new(context, operation_name);
        let span_id = span.span_id;
        
        self.spans.insert(span_id, span);
        span_id
    }
    
    fn get_span(&mut self, span_id: &Uuid) -> Option<&mut Span> {
        self.spans.get_mut(span_id)
    }
    
    fn finish_span(&mut self, span_id: &Uuid) {
        if let Some(span) = self.spans.get_mut(span_id) {
            span.finish();
        }
    }
    
    fn get_trace(&self, trace_id: &Uuid) -> Vec<&Span> {
        self.spans
            .values()
            .filter(|span| span.trace_id == *trace_id)
            .collect()
    }
}

// 模拟分布式系统中的服务调用
fn simulate_distributed_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracer = Tracer::new();
    
    // 创建根追踪上下文
    let mut root_context = TraceContext::new();
    root_context.set_baggage("user_id".to_string(), "12345".to_string());
    root_context.set_baggage("request_id".to_string(), "req-001".to_string());
    
    println!("开始分布式请求追踪");
    println!("Trace ID: {}", root_context.trace_id);
    
    // 开始根 span
    let root_span_id = tracer.start_span(&root_context, "http_request".to_string());
    
    if let Some(span) = tracer.get_span(&root_span_id) {
        span.set_tag("http.method".to_string(), "GET".to_string());
        span.set_tag("http.url".to_string(), "/api/users/profile".to_string());
        span.log(LogLevel::Info, "开始处理 HTTP 请求".to_string());
    }
    
    // 模拟数据库查询
    let db_context = root_context.child_span();
    let db_span_id = tracer.start_span(&db_context, "database_query".to_string());
    
    if let Some(span) = tracer.get_span(&db_span_id) {
        span.set_tag("db.type".to_string(), "postgresql".to_string());
        span.set_tag("db.statement".to_string(), "SELECT * FROM users WHERE id = $1".to_string());
        span.log(LogLevel::Info, "执行数据库查询".to_string());
    }
    
    // 模拟查询耗时
    std::thread::sleep(Duration::from_millis(50));
    
    tracer.finish_span(&db_span_id);
    
    // 模拟外部 API 调用
    let api_context = root_context.child_span();
    let api_span_id = tracer.start_span(&api_context, "external_api_call".to_string());
    
    if let Some(span) = tracer.get_span(&api_span_id) {
        span.set_tag("http.method".to_string(), "GET".to_string());
        span.set_tag("http.url".to_string(), "https://api.example.com/user/preferences".to_string());
        span.log(LogLevel::Info, "调用外部 API".to_string());
    }
    
    // 模拟 API 调用耗时
    std::thread::sleep(Duration::from_millis(100));
    
    tracer.finish_span(&api_span_id);
    
    // 完成根 span
    if let Some(span) = tracer.get_span(&root_span_id) {
        span.log(LogLevel::Info, "HTTP 请求处理完成".to_string());
    }
    
    tracer.finish_span(&root_span_id);
    
    // 输出追踪结果
    let trace_spans = tracer.get_trace(&root_context.trace_id);
    println!("\n追踪结果 (Trace ID: {}):", root_context.trace_id);
    
    for span in trace_spans {
        println!("  Span: {} ({})", span.operation_name, span.span_id);
        println!("    父 Span: {:?}", span.parent_span_id);
        println!("    持续时间: {:?}", span.duration());
        println!("    标签: {:?}", span.tags);
        println!("    日志条目: {}", span.logs.len());
        println!();
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simulate_distributed_request()?;
    Ok(())
}
```

### 4. 文件系统中的唯一文件名生成

```rust
use uuid::Uuid;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

struct FileManager {
    base_path: PathBuf,
}

impl FileManager {
    fn new<P: AsRef<Path>>(base_path: P) -> std::io::Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        
        // 创建基础目录
        fs::create_dir_all(&base_path)?;
        
        Ok(Self { base_path })
    }
    
    fn generate_unique_filename(&self, extension: &str) -> String {
        let uuid = Uuid::new_v4();
        format!("{}.{}", uuid, extension)
    }
    
    fn save_file(&self, data: &[u8], extension: &str) -> std::io::Result<(Uuid, PathBuf)> {
        let uuid = Uuid::new_v4();
        let filename = format!("{}.{}", uuid, extension);
        let file_path = self.base_path.join(&filename);
        
        fs::write(&file_path, data)?;
        
        Ok((uuid, file_path))
    }
    
    fn save_text_file(&self, content: &str, extension: &str) -> std::io::Result<(Uuid, PathBuf)> {
        self.save_file(content.as_bytes(), extension)
    }
    
    fn get_file_path(&self, uuid: &Uuid, extension: &str) -> PathBuf {
        let filename = format!("{}.{}", uuid, extension);
        self.base_path.join(filename)
    }
    
    fn read_file(&self, uuid: &Uuid, extension: &str) -> std::io::Result<Vec<u8>> {
        let file_path = self.get_file_path(uuid, extension);
        fs::read(file_path)
    }
    
    fn read_text_file(&self, uuid: &Uuid, extension: &str) -> std::io::Result<String> {
        let file_path = self.get_file_path(uuid, extension);
        fs::read_to_string(file_path)
    }
    
    fn delete_file(&self, uuid: &Uuid, extension: &str) -> std::io::Result<()> {
        let file_path = self.get_file_path(uuid, extension);
        fs::remove_file(file_path)
    }
    
    fn file_exists(&self, uuid: &Uuid, extension: &str) -> bool {
        let file_path = self.get_file_path(uuid, extension);
        file_path.exists()
    }
    
    fn list_files(&self) -> std::io::Result<Vec<(Uuid, String)>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(dot_pos) = filename.rfind('.') {
                        let uuid_str = &filename[..dot_pos];
                        let extension = &filename[dot_pos + 1..];
                        
                        if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                            files.push((uuid, extension.to_string()));
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_manager = FileManager::new("./temp_files")?;
    
    // 保存文本文件
    let content = "这是一个测试文件的内容。\n包含多行文本。";
    let (text_uuid, text_path) = file_manager.save_text_file(content, "txt")?;
    println!("保存文本文件: {} -> {:?}", text_uuid, text_path);
    
    // 保存二进制文件
    let binary_data = b"Binary data content";
    let (binary_uuid, binary_path) = file_manager.save_file(binary_data, "bin")?;
    println!("保存二进制文件: {} -> {:?}", binary_uuid, binary_path);
    
    // 保存 JSON 文件
    let json_content = r#"{"name": "test", "value": 42}"#;
    let (json_uuid, json_path) = file_manager.save_text_file(json_content, "json")?;
    println!("保存 JSON 文件: {} -> {:?}", json_uuid, json_path);
    
    // 读取文件
    let read_content = file_manager.read_text_file(&text_uuid, "txt")?;
    println!("读取的文本内容: {}", read_content);
    
    let read_binary = file_manager.read_file(&binary_uuid, "bin")?;
    println!("读取的二进制内容: {:?}", read_binary);
    
    // 检查文件是否存在
    println!("文本文件存在: {}", file_manager.file_exists(&text_uuid, "txt"));
    println!("二进制文件存在: {}", file_manager.file_exists(&binary_uuid, "bin"));
    
    // 列出所有文件
    let files = file_manager.list_files()?;
    println!("所有文件:");
    for (uuid, extension) in files {
        println!("  {}.{}", uuid, extension);
    }
    
    // 删除文件
    file_manager.delete_file(&text_uuid, "txt")?;
    println!("删除文本文件: {}", text_uuid);
    
    // 再次检查文件是否存在
    println!("文本文件存在: {}", file_manager.file_exists(&text_uuid, "txt"));
    
    // 清理剩余文件
    file_manager.delete_file(&binary_uuid, "bin")?;
    file_manager.delete_file(&json_uuid, "json")?;
    
    Ok(())
}
```

## 序列化和反序列化

### 1. 与 Serde 集成

```rust
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    user: User,
    session_id: Uuid,
    expires_at: chrono::DateTime<chrono::Utc>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: Uuid::new_v4(),
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        created_at: chrono::Utc::now(),
    };
    
    let response = ApiResponse {
        user,
        session_id: Uuid::new_v4(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
    };
    
    // 序列化为 JSON
    let json = serde_json::to_string_pretty(&response)?;
    println!("JSON 输出:\n{}", json);
    
    // 从 JSON 反序列化
    let parsed: ApiResponse = serde_json::from_str(&json)?;
    println!("解析结果: {:?}", parsed);
    
    // 序列化为字节
    let bytes = bincode::serialize(&response)?;
    println!("二进制序列化大小: {} bytes", bytes.len());
    
    // 从字节反序列化
    let deserialized: ApiResponse = bincode::deserialize(&bytes)?;
    println!("二进制反序列化结果: {:?}", deserialized);
    
    Ok(())
}
```

### 2. 数据库存储

```rust
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result, params};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: Uuid,
    name: String,
    price: f64,
    description: String,
    category_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: Uuid,
    name: String,
    description: String,
}

struct ProductDatabase {
    conn: Connection,
}

impl ProductDatabase {
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // 创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                description TEXT,
                category_id TEXT NOT NULL,
                FOREIGN KEY (category_id) REFERENCES categories (id)
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    fn create_category(&self, name: String, description: String) -> Result<Uuid> {
        let category_id = Uuid::new_v4();
        
        self.conn.execute(
            "INSERT INTO categories (id, name, description) VALUES (?1, ?2, ?3)",
            params![category_id.to_string(), name, description],
        )?;
        
        Ok(category_id)
    }
    
    fn create_product(&self, name: String, price: f64, description: String, category_id: Uuid) -> Result<Uuid> {
        let product_id = Uuid::new_v4();
        
        self.conn.execute(
            "INSERT INTO products (id, name, price, description, category_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![product_id.to_string(), name, price, description, category_id.to_string()],
        )?;
        
        Ok(product_id)
    }
    
    fn get_category(&self, category_id: &Uuid) -> Result<Option<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description FROM categories WHERE id = ?1"
        )?;
        
        let category = stmt.query_row(params![category_id.to_string()], |row| {
            Ok(Category {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                name: row.get(1)?,
                description: row.get(2)?,
            })
        }).optional()?;
        
        Ok(category)
    }
    
    fn get_product(&self, product_id: &Uuid) -> Result<Option<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, price, description, category_id FROM products WHERE id = ?1"
        )?;
        
        let product = stmt.query_row(params![product_id.to_string()], |row| {
            Ok(Product {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                name: row.get(1)?,
                price: row.get(2)?,
                description: row.get(3)?,
                category_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap(),
            })
        }).optional()?;
        
        Ok(product)
    }
    
    fn get_products_by_category(&self, category_id: &Uuid) -> Result<Vec<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, price, description, category_id FROM products WHERE category_id = ?1"
        )?;
        
        let products = stmt.query_map(params![category_id.to_string()], |row| {
            Ok(Product {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                name: row.get(1)?,
                price: row.get(2)?,
                description: row.get(3)?,
                category_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap(),
            })
        })?;
        
        let mut result = Vec::new();
        for product in products {
            result.push(product?);
        }
        
        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = ProductDatabase::new("products.db")?;
    
    // 创建分类
    let electronics_id = db.create_category("电子产品".to_string(), "各种电子设备".to_string())?;
    let books_id = db.create_category("图书".to_string(), "各种书籍".to_string())?;
    
    println!("创建分类: 电子产品 ({})", electronics_id);
    println!("创建分类: 图书 ({})", books_id);
    
    // 创建产品
    let laptop_id = db.create_product(
        "笔记本电脑".to_string(),
        5999.99,
        "高性能笔记本电脑".to_string(),
        electronics_id,
    )?;
    
    let phone_id = db.create_product(
        "智能手机".to_string(),
        2999.99,
        "最新款智能手机".to_string(),
        electronics_id,
    )?;
    
    let book_id = db.create_product(
        "Rust 编程语言".to_string(),
        89.99,
        "学习 Rust 编程的最佳书籍".to_string(),
        books_id,
    )?;
    
    println!("创建产品: 笔记本电脑 ({})", laptop_id);
    println!("创建产品: 智能手机 ({})", phone_id);
    println!("创建产品: Rust 编程语言 ({})", book_id);
    
    // 查询产品
    if let Some(laptop) = db.get_product(&laptop_id)? {
        println!("查询到笔记本电脑: {:?}", laptop);
    }
    
    // 查询分类下的所有产品
    let electronics_products = db.get_products_by_category(&electronics_id)?;
    println!("电子产品分类下的产品:");
    for product in electronics_products {
        println!("  - {}: ¥{}", product.name, product.price);
    }
    
    Ok(())
}
```

## 性能优化

### 1. 批量生成 UUID

```rust
use uuid::Uuid;
use std::time::Instant;

fn benchmark_uuid_generation() {
    let count = 1_000_000;
    
    // 单个生成
    let start = Instant::now();
    for _ in 0..count {
        let _uuid = Uuid::new_v4();
    }
    let duration = start.elapsed();
    println!("单个生成 {} 个 UUID 耗时: {:?}", count, duration);
    
    // 批量生成
    let start = Instant::now();
    let uuids: Vec<Uuid> = (0..count).map(|_| Uuid::new_v4()).collect();
    let duration = start.elapsed();
    println!("批量生成 {} 个 UUID 耗时: {:?}", count, duration);
    
    // 并行生成
    let start = Instant::now();
    let uuids: Vec<Uuid> = (0..count).into_par_iter().map(|_| Uuid::new_v4()).collect();
    let duration = start.elapsed();
    println!("并行生成 {} 个 UUID 耗时: {:?}", count, duration);
}

fn main() {
    benchmark_uuid_generation();
}
```

### 2. 内存优化

```rust
use uuid::Uuid;
use std::collections::HashMap;

// 使用 UUID 作为键的优化
struct OptimizedMap {
    // 使用 u128 作为键更高效
    inner: HashMap<u128, String>,
}

impl OptimizedMap {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    
    fn insert(&mut self, uuid: Uuid, value: String) {
        self.inner.insert(uuid.as_u128(), value);
    }
    
    fn get(&self, uuid: &Uuid) -> Option<&String> {
        self.inner.get(&uuid.as_u128())
    }
    
    fn remove(&mut self, uuid: &Uuid) -> Option<String> {
        self.inner.remove(&uuid.as_u128())
    }
}

fn main() {
    let mut map = OptimizedMap::new();
    
    // 插入数据
    for i in 0..10 {
        let uuid = Uuid::new_v4();
        map.insert(uuid, format!("value_{}", i));
    }
    
    println!("HashMap 创建完成");
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
uuid = { version = "1.17.0", features = ["v4", "serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
bincode = "1.3"
rusqlite = { version = "0.29", features = ["bundled"] }
rayon = "1.7"

# 可选特性
# uuid = { version = "1.17.0", features = ["v1", "v3", "v4", "v5", "v6", "v7", "v8", "serde"] }
```

## 总结

UUID 库是 Rust 生态系统中生成和处理唯一标识符的标准解决方案。它支持多种 UUID 版本，提供了高性能的生成和解析功能，广泛应用于分布式系统、数据库、文件系统等场景。

主要特性：
- 🔢 支持多种 UUID 版本 (v1-v8)
- 🚀 高性能的生成和解析
- 🔒 线程安全和内存安全
- 📊 与 Serde 完美集成
- 🌐 跨平台支持
- 🔧 丰富的格式转换选项

无论是构建分布式系统、数据库应用，还是文件管理系统，UUID 都是确保唯一性的最佳选择。