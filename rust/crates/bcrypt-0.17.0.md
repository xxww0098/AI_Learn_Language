# BCrypt 0.17.0 中文教程

## 简介

BCrypt 是一个专为密码哈希设计的加密库，基于 Blowfish 加密算法。它提供了一种安全的方式来存储密码，具有自适应的成本参数，可以根据硬件能力调整计算复杂度，从而抵御暴力破解和彩虹表攻击。

## 核心特性

- 🔐 自适应密码哈希算法
- 🛡️ 内置盐值（salt）生成
- 🚀 可调节的计算成本
- 📊 时间安全的验证
- 🌐 跨平台支持
- 🔧 简单易用的API

## 基本概念

### 1. 什么是 BCrypt

BCrypt 是一种密码哈希函数，具有以下特点：
- **自适应**：可以通过增加 cost 参数来增加计算时间
- **盐值**：每个密码都有唯一的盐值，防止彩虹表攻击
- **单向性**：无法从哈希值反推出原始密码
- **时间安全**：验证时间与密码长度无关

### 2. Cost 参数

Cost 参数决定了哈希计算的复杂度：
- 较低的 cost（4-8）：适合开发和测试
- 中等的 cost（10-12）：适合大多数应用
- 较高的 cost（13-15）：适合高安全性要求的场景

## 基本用法

### 1. 基本密码哈希和验证

```rust
use bcrypt::{hash, verify, DEFAULT_COST};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "my-secret-password";
    
    // 生成哈希
    let hashed = hash(password, DEFAULT_COST)?;
    println!("哈希值: {}", hashed);
    
    // 验证密码
    let is_valid = verify(password, &hashed)?;
    println!("密码验证: {}", is_valid); // true
    
    // 验证错误密码
    let is_valid = verify("wrong-password", &hashed)?;
    println!("错误密码验证: {}", is_valid); // false
    
    Ok(())
}
```

### 2. 自定义 Cost 参数

```rust
use bcrypt::{hash, verify};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "my-secret-password";
    
    // 使用不同的 cost 值
    let cost_4 = hash(password, 4)?;   // 快速，适合测试
    let cost_10 = hash(password, 10)?; // 标准，适合生产
    let cost_12 = hash(password, 12)?; // 高安全性
    
    println!("Cost 4:  {}", cost_4);
    println!("Cost 10: {}", cost_10);
    println!("Cost 12: {}", cost_12);
    
    // 验证都应该成功
    println!("验证 cost 4:  {}", verify(password, &cost_4)?);
    println!("验证 cost 10: {}", verify(password, &cost_10)?);
    println!("验证 cost 12: {}", verify(password, &cost_12)?);
    
    Ok(())
}
```

### 3. 哈希格式解析

```rust
use bcrypt::hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "test-password";
    let hashed = hash(password, 10)?;
    
    println!("完整哈希: {}", hashed);
    
    // BCrypt 哈希格式: $2b$[cost]$[salt][hash]
    // 例如: $2b$10$N9qo8uLOickgx2ZMRZoMye.f3KVbZVxBSJP6HvqLp0YQmJdQGwO7G
    
    let parts: Vec<&str> = hashed.split('$').collect();
    if parts.len() >= 4 {
        println!("版本: {}", parts[1]);           // 2b
        println!("Cost: {}", parts[2]);           // 10
        println!("盐值和哈希: {}", parts[3]);        // N9qo8uLOickgx2ZMRZoMye.f3KVbZVxBSJP6HvqLp0YQmJdQGwO7G
    }
    
    Ok(())
}
```

## 实际应用示例

### 1. 用户注册和登录系统

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
struct UserService {
    users: HashMap<String, User>,
    next_id: u64,
}

impl UserService {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }
    
    fn register(&mut self, username: String, email: String, password: String) -> Result<u64, String> {
        // 检查用户名是否已存在
        if self.users.contains_key(&username) {
            return Err("用户名已存在".to_string());
        }
        
        // 验证密码强度
        if password.len() < 8 {
            return Err("密码长度至少8位".to_string());
        }
        
        // 生成密码哈希
        let password_hash = hash(&password, DEFAULT_COST)
            .map_err(|e| format!("密码哈希失败: {}", e))?;
        
        // 创建用户
        let user = User {
            id: self.next_id,
            username: username.clone(),
            email,
            password_hash,
            created_at: chrono::Utc::now(),
        };
        
        self.users.insert(username, user);
        let user_id = self.next_id;
        self.next_id += 1;
        
        Ok(user_id)
    }
    
    fn login(&self, username: &str, password: &str) -> Result<&User, String> {
        let user = self.users.get(username)
            .ok_or_else(|| "用户不存在".to_string())?;
        
        let is_valid = verify(password, &user.password_hash)
            .map_err(|e| format!("密码验证失败: {}", e))?;
        
        if is_valid {
            Ok(user)
        } else {
            Err("密码错误".to_string())
        }
    }
    
    fn change_password(&mut self, username: &str, old_password: &str, new_password: &str) -> Result<(), String> {
        let user = self.users.get(username)
            .ok_or_else(|| "用户不存在".to_string())?;
        
        // 验证旧密码
        let is_valid = verify(old_password, &user.password_hash)
            .map_err(|e| format!("密码验证失败: {}", e))?;
        
        if !is_valid {
            return Err("旧密码错误".to_string());
        }
        
        // 验证新密码强度
        if new_password.len() < 8 {
            return Err("新密码长度至少8位".to_string());
        }
        
        // 生成新密码哈希
        let new_password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| format!("密码哈希失败: {}", e))?;
        
        // 更新用户密码
        if let Some(user) = self.users.get_mut(username) {
            user.password_hash = new_password_hash;
        }
        
        Ok(())
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_service = UserService::new();
    
    // 注册用户
    let user_id = user_service.register(
        "alice".to_string(),
        "alice@example.com".to_string(),
        "secure-password-123".to_string(),
    )?;
    
    println!("用户注册成功，ID: {}", user_id);
    
    // 登录
    let user = user_service.login("alice", "secure-password-123")?;
    println!("登录成功: {:?}", user);
    
    // 更改密码
    user_service.change_password("alice", "secure-password-123", "new-secure-password-456")?;
    println!("密码更改成功");
    
    // 使用新密码登录
    let user = user_service.login("alice", "new-secure-password-456")?;
    println!("使用新密码登录成功: {:?}", user);
    
    Ok(())
}
```

### 2. Web API 认证

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}

type Users = Arc<Mutex<HashMap<String, User>>>;

// 注册端点
async fn register_handler(
    req: RegisterRequest,
    users: Users,
) -> Result<impl Reply, Rejection> {
    let mut users = users.lock().unwrap();
    
    // 检查用户名是否已存在
    if users.contains_key(&req.username) {
        return Ok(warp::reply::json(&ApiResponse {
            success: false,
            message: "用户名已存在".to_string(),
            data: None,
        }));
    }
    
    // 验证密码强度
    if req.password.len() < 8 {
        return Ok(warp::reply::json(&ApiResponse {
            success: false,
            message: "密码长度至少8位".to_string(),
            data: None,
        }));
    }
    
    // 生成密码哈希
    let password_hash = match hash(&req.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Ok(warp::reply::json(&ApiResponse {
                success: false,
                message: "密码哈希失败".to_string(),
                data: None,
            }));
        }
    };
    
    // 创建用户
    let user = User {
        id: users.len() as u64 + 1,
        username: req.username.clone(),
        email: req.email,
        password_hash,
        created_at: chrono::Utc::now(),
    };
    
    users.insert(req.username, user);
    
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "注册成功".to_string(),
        data: None,
    }))
}

// 登录端点
async fn login_handler(
    req: LoginRequest,
    users: Users,
) -> Result<impl Reply, Rejection> {
    let users = users.lock().unwrap();
    
    let user = match users.get(&req.username) {
        Some(user) => user,
        None => {
            return Ok(warp::reply::json(&ApiResponse {
                success: false,
                message: "用户不存在".to_string(),
                data: None,
            }));
        }
    };
    
    let is_valid = match verify(&req.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => {
            return Ok(warp::reply::json(&ApiResponse {
                success: false,
                message: "密码验证失败".to_string(),
                data: None,
            }));
        }
    };
    
    if is_valid {
        Ok(warp::reply::json(&ApiResponse {
            success: true,
            message: "登录成功".to_string(),
            data: Some(serde_json::json!({
                "user_id": user.id,
                "username": user.username,
                "email": user.email
            })),
        }))
    } else {
        Ok(warp::reply::json(&ApiResponse {
            success: false,
            message: "密码错误".to_string(),
            data: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    
    // 注册路由
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users(users.clone()))
        .and_then(register_handler);
    
    // 登录路由
    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users(users.clone()))
        .and_then(login_handler);
    
    let routes = register.or(login);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}
```

### 3. 批量密码处理

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use rayon::prelude::*;
use std::time::Instant;

struct PasswordBatch {
    passwords: Vec<String>,
}

impl PasswordBatch {
    fn new(passwords: Vec<String>) -> Self {
        Self { passwords }
    }
    
    // 单线程哈希
    fn hash_sequential(&self, cost: u32) -> Result<Vec<String>, bcrypt::BcryptError> {
        let start = Instant::now();
        let hashes: Result<Vec<_>, _> = self.passwords
            .iter()
            .map(|password| hash(password, cost))
            .collect();
        
        let duration = start.elapsed();
        println!("单线程哈希 {} 个密码耗时: {:?}", self.passwords.len(), duration);
        
        hashes
    }
    
    // 多线程哈希
    fn hash_parallel(&self, cost: u32) -> Result<Vec<String>, bcrypt::BcryptError> {
        let start = Instant::now();
        let hashes: Result<Vec<_>, _> = self.passwords
            .par_iter()
            .map(|password| hash(password, cost))
            .collect();
        
        let duration = start.elapsed();
        println!("多线程哈希 {} 个密码耗时: {:?}", self.passwords.len(), duration);
        
        hashes
    }
    
    // 批量验证
    fn verify_batch(&self, hashes: &[String]) -> Result<Vec<bool>, bcrypt::BcryptError> {
        let start = Instant::now();
        let results: Result<Vec<_>, _> = self.passwords
            .par_iter()
            .zip(hashes.par_iter())
            .map(|(password, hash)| verify(password, hash))
            .collect();
        
        let duration = start.elapsed();
        println!("批量验证 {} 个密码耗时: {:?}", self.passwords.len(), duration);
        
        results
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passwords = vec![
        "password1".to_string(),
        "password2".to_string(),
        "password3".to_string(),
        "password4".to_string(),
        "password5".to_string(),
    ];
    
    let batch = PasswordBatch::new(passwords);
    
    // 性能对比
    let hashes_seq = batch.hash_sequential(10)?;
    let hashes_par = batch.hash_parallel(10)?;
    
    // 验证结果
    let results = batch.verify_batch(&hashes_seq)?;
    println!("验证结果: {:?}", results);
    
    Ok(())
}
```

## 安全最佳实践

### 1. Cost 参数选择

```rust
use bcrypt::{hash, DEFAULT_COST};
use std::time::Instant;

fn benchmark_cost(password: &str, cost: u32) -> Result<std::time::Duration, bcrypt::BcryptError> {
    let start = Instant::now();
    let _hash = hash(password, cost)?;
    Ok(start.elapsed())
}

fn find_optimal_cost() -> Result<u32, bcrypt::BcryptError> {
    let password = "test-password";
    let target_duration = std::time::Duration::from_millis(100); // 目标100ms
    
    for cost in 4..=15 {
        let duration = benchmark_cost(password, cost)?;
        println!("Cost {}: {:?}", cost, duration);
        
        if duration >= target_duration {
            return Ok(cost);
        }
    }
    
    Ok(DEFAULT_COST)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let optimal_cost = find_optimal_cost()?;
    println!("推荐的 cost 参数: {}", optimal_cost);
    
    Ok(())
}
```

### 2. 密码强度验证

```rust
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;

struct PasswordValidator {
    min_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_numbers: bool,
    require_special: bool,
}

impl PasswordValidator {
    fn new() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
        }
    }
    
    fn validate(&self, password: &str) -> Result<(), String> {
        // 长度检查
        if password.len() < self.min_length {
            return Err(format!("密码长度至少需要 {} 位", self.min_length));
        }
        
        // 大写字母检查
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err("密码必须包含至少一个大写字母".to_string());
        }
        
        // 小写字母检查
        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err("密码必须包含至少一个小写字母".to_string());
        }
        
        // 数字检查
        if self.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err("密码必须包含至少一个数字".to_string());
        }
        
        // 特殊字符检查
        if self.require_special {
            let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
            if !password.chars().any(|c| special_chars.contains(c)) {
                return Err("密码必须包含至少一个特殊字符".to_string());
            }
        }
        
        // 常见密码检查
        let common_passwords = vec![
            "password", "123456", "qwerty", "abc123", "password123"
        ];
        
        if common_passwords.contains(&password.to_lowercase().as_str()) {
            return Err("不能使用常见密码".to_string());
        }
        
        Ok(())
    }
    
    fn hash_if_valid(&self, password: &str) -> Result<String, String> {
        self.validate(password)?;
        hash(password, DEFAULT_COST)
            .map_err(|e| format!("密码哈希失败: {}", e))
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = PasswordValidator::new();
    
    let passwords = vec![
        "password",           // 太简单
        "Password1",          // 缺少特殊字符
        "Password1!",         // 符合要求
        "VeryStr0ngP@ssw0rd!", // 符合要求
    ];
    
    for password in passwords {
        match validator.hash_if_valid(password) {
            Ok(hash) => println!("密码 '{}' 哈希: {}", password, hash),
            Err(err) => println!("密码 '{}' 验证失败: {}", password, err),
        }
    }
    
    Ok(())
}
```

### 3. 安全的密码重置

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
struct PasswordResetToken {
    token: String,
    user_id: u64,
    expires_at: SystemTime,
}

struct PasswordResetService {
    tokens: HashMap<String, PasswordResetToken>,
    users: HashMap<u64, User>,
}

impl PasswordResetService {
    fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            users: HashMap::new(),
        }
    }
    
    fn generate_reset_token(&mut self, user_id: u64) -> Result<String, String> {
        if !self.users.contains_key(&user_id) {
            return Err("用户不存在".to_string());
        }
        
        let token = Uuid::new_v4().to_string();
        let expires_at = SystemTime::now() + Duration::from_secs(3600); // 1小时后过期
        
        let reset_token = PasswordResetToken {
            token: token.clone(),
            user_id,
            expires_at,
        };
        
        self.tokens.insert(token.clone(), reset_token);
        
        Ok(token)
    }
    
    fn reset_password(&mut self, token: &str, new_password: &str) -> Result<(), String> {
        let reset_token = self.tokens.get(token)
            .ok_or_else(|| "无效的重置令牌".to_string())?;
        
        // 检查令牌是否过期
        if SystemTime::now() > reset_token.expires_at {
            self.tokens.remove(token);
            return Err("重置令牌已过期".to_string());
        }
        
        // 验证新密码
        if new_password.len() < 8 {
            return Err("新密码长度至少8位".to_string());
        }
        
        // 生成新密码哈希
        let password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| format!("密码哈希失败: {}", e))?;
        
        // 更新用户密码
        if let Some(user) = self.users.get_mut(&reset_token.user_id) {
            user.password_hash = password_hash;
        }
        
        // 删除使用过的令牌
        self.tokens.remove(token);
        
        Ok(())
    }
    
    fn cleanup_expired_tokens(&mut self) {
        let now = SystemTime::now();
        self.tokens.retain(|_, token| now <= token.expires_at);
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reset_service = PasswordResetService::new();
    
    // 添加测试用户
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: hash("old-password", DEFAULT_COST)?,
        created_at: chrono::Utc::now(),
    };
    
    reset_service.users.insert(1, user);
    
    // 生成重置令牌
    let token = reset_service.generate_reset_token(1)?;
    println!("重置令牌: {}", token);
    
    // 重置密码
    reset_service.reset_password(&token, "new-secure-password")?;
    println!("密码重置成功");
    
    // 清理过期令牌
    reset_service.cleanup_expired_tokens();
    
    Ok(())
}
```

## 性能优化

### 1. 缓存优化

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};

struct HashCache {
    cache: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    ttl: Duration,
}

impl HashCache {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }
    
    fn get_or_hash(&self, password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
        let cache_key = format!("{}:{}", password, cost);
        
        // 检查缓存
        {
            let cache = self.cache.lock().unwrap();
            if let Some((hash, timestamp)) = cache.get(&cache_key) {
                if timestamp.elapsed().unwrap() < self.ttl {
                    return Ok(hash.clone());
                }
            }
        }
        
        // 生成新哈希
        let hash = hash(password, cost)?;
        
        // 更新缓存
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(cache_key, (hash.clone(), SystemTime::now()));
        }
        
        Ok(hash)
    }
    
    fn cleanup_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
        let now = SystemTime::now();
        cache.retain(|_, (_, timestamp)| now.duration_since(*timestamp).unwrap() < self.ttl);
    }
}
```

### 2. 异步处理

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use tokio::task;

async fn hash_async(password: String, cost: u32) -> Result<String, bcrypt::BcryptError> {
    task::spawn_blocking(move || {
        hash(&password, cost)
    }).await.unwrap()
}

async fn verify_async(password: String, hash: String) -> Result<bool, bcrypt::BcryptError> {
    task::spawn_blocking(move || {
        verify(&password, &hash)
    }).await.unwrap()
}

// 使用示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "test-password".to_string();
    
    // 异步哈希
    let hash = hash_async(password.clone(), DEFAULT_COST).await?;
    println!("异步哈希: {}", hash);
    
    // 异步验证
    let is_valid = verify_async(password, hash).await?;
    println!("异步验证: {}", is_valid);
    
    Ok(())
}
```

## 错误处理

### 1. 自定义错误类型

```rust
use thiserror::Error;
use bcrypt::BcryptError;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("BCrypt error: {0}")]
    BcryptError(#[from] BcryptError),
    
    #[error("Invalid password format")]
    InvalidPassword,
    
    #[error("Password too weak")]
    WeakPassword,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Authentication failed")]
    AuthFailed,
}

fn secure_login(username: &str, password: &str, stored_hash: &str) -> Result<bool, AuthError> {
    if password.len() < 8 {
        return Err(AuthError::WeakPassword);
    }
    
    let is_valid = verify(password, stored_hash)?;
    
    if is_valid {
        Ok(true)
    } else {
        Err(AuthError::AuthFailed)
    }
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
bcrypt = "0.17.0"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4"] }
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
rayon = "1.7"
```

## 总结

BCrypt 是一个强大而安全的密码哈希库，提供了自适应的加密强度和简单的API。通过合理的配置和最佳实践，可以构建安全的密码管理系统。

主要特性：
- 🔐 自适应密码哈希算法
- 🛡️ 抗暴力破解和彩虹表攻击
- 🚀 高性能和可扩展性
- 📊 时间安全的验证
- 🔧 简单易用的API
- 🌐 跨平台支持

BCrypt 是 Rust 生态系统中密码安全的首选解决方案。