# SQLx 0.8.6 - Rust SQL 工具包完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [数据库连接](#数据库连接)
- [编译时检查](#编译时检查)
- [查询操作](#查询操作)
- [事务处理](#事务处理)
- [连接池](#连接池)
- [类型映射](#类型映射)
- [迁移系统](#迁移系统)
- [异步流处理](#异步流处理)
- [错误处理](#错误处理)
- [测试支持](#测试支持)
- [宏系统](#宏系统)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

SQLx 是一个现代的、异步的、纯 Rust SQL 工具包，支持编译时检查的查询而无需 DSL。它支持 PostgreSQL、MySQL 和 SQLite，提供了类型安全的数据库操作。

### 核心特性
- **编译时检查**: 在编译时验证 SQL 查询
- **异步优先**: 完全异步的 API 设计
- **类型安全**: 强类型的结果映射
- **多数据库支持**: PostgreSQL、MySQL、SQLite
- **无 DSL**: 直接使用 SQL 语句
- **连接池**: 内置连接池管理
- **迁移系统**: 自动化数据库迁移

### 版本信息
- **当前版本**: 0.8.6
- **发布时间**: 2025-05-19
- **下载次数**: 41,842,809+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
sqlx = { version = "0.8.6", features = [
    "runtime-tokio-rustls",
    "postgres",
    "mysql",
    "sqlite",
    "chrono",
    "uuid",
    "json",
    "macros",
    "migrate",
] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### 基本示例

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::{Row, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 创建连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/database")
        .await?;
    
    // 创建表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    // 插入数据
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind("张三")
    .bind("zhangsan@example.com")
    .fetch_one(&pool)
    .await?;
    
    println!("创建用户: {:?}", user);
    
    // 查询数据
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;
    
    println!("所有用户: {:?}", users);
    
    Ok(())
}
```

## 数据库连接

### PostgreSQL 连接

```rust
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{ConnectOptions, Pool, Postgres};
use std::time::Duration;

// 使用连接字符串
async fn connect_with_url() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/database")
        .await?;
    
    Ok(pool)
}

// 使用连接选项
async fn connect_with_options() -> Result<Pool<Postgres>, sqlx::Error> {
    let options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("user")
        .password("password")
        .database("database")
        .application_name("my_app")
        .log_statements(log::LevelFilter::Debug)
        .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1));
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(3600))
        .connect_with(options)
        .await?;
    
    Ok(pool)
}
```

### MySQL 连接

```rust
use sqlx::mysql::{MySqlPoolOptions, MySqlConnectOptions};
use sqlx::{Pool, MySql};

async fn connect_mysql() -> Result<Pool<MySql>, sqlx::Error> {
    let options = MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username("user")
        .password("password")
        .database("database")
        .charset("utf8mb4")
        .collation("utf8mb4_unicode_ci");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    Ok(pool)
}
```

### SQLite 连接

```rust
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{Pool, Sqlite};

async fn connect_sqlite() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename("database.db")
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    Ok(pool)
}

// 内存数据库
async fn connect_memory() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await?;
    
    Ok(pool)
}
```

## 编译时检查

### 查询宏

```rust
use sqlx::{query, query_as, query_scalar};
use sqlx::postgres::PgPool;

#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

async fn compile_time_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 编译时检查的查询
    let user = query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        1
    )
    .fetch_one(pool)
    .await?;
    
    println!("用户: {:?}", user);
    
    // 标量查询
    let count = query_scalar!(
        "SELECT COUNT(*) FROM users WHERE name LIKE $1",
        "%张%"
    )
    .fetch_one(pool)
    .await?;
    
    println!("用户数量: {:?}", count);
    
    // 插入操作
    let id = query_scalar!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        "李四",
        "lisi@example.com"
    )
    .fetch_one(pool)
    .await?;
    
    println!("新用户ID: {:?}", id);
    
    Ok(())
}
```

### 文件查询

```rust
use sqlx::{query_file, query_file_as, query_file_scalar};

// 创建 sql/get_user.sql 文件
// SELECT id, name, email FROM users WHERE id = $1

async fn file_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 从文件读取查询
    let user = query_file_as!(User, "sql/get_user.sql", 1)
        .fetch_one(pool)
        .await?;
    
    println!("用户: {:?}", user);
    
    // 从文件读取标量查询
    let count = query_file_scalar!("sql/count_users.sql")
        .fetch_one(pool)
        .await?;
    
    println!("用户数量: {:?}", count);
    
    Ok(())
}
```

### 类型安全的查询

```rust
use sqlx::types::{Json, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    avatar_url: Option<String>,
    bio: Option<String>,
    preferences: serde_json::Value,
}

#[derive(Debug, sqlx::FromRow)]
struct ExtendedUser {
    id: Uuid,
    name: String,
    email: String,
    profile: Json<UserProfile>,
    tags: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

async fn type_safe_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 插入复杂类型
    let profile = UserProfile {
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        bio: Some("这是用户简介".to_string()),
        preferences: serde_json::json!({
            "theme": "dark",
            "language": "zh-CN"
        }),
    };
    
    let user_id = Uuid::new_v4();
    let tags = vec!["rust".to_string(), "programming".to_string()];
    
    let user = query_as!(
        ExtendedUser,
        r#"
        INSERT INTO extended_users (id, name, email, profile, tags, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, profile as "profile: Json<UserProfile>", tags, created_at
        "#,
        user_id,
        "王五",
        "wangwu@example.com",
        Json(&profile),
        &tags,
        chrono::Utc::now()
    )
    .fetch_one(pool)
    .await?;
    
    println!("创建扩展用户: {:?}", user);
    
    // 查询复杂类型
    let users = query_as!(
        ExtendedUser,
        r#"
        SELECT id, name, email, profile as "profile: Json<UserProfile>", tags, created_at
        FROM extended_users
        WHERE $1 = ANY(tags)
        "#,
        "rust"
    )
    .fetch_all(pool)
    .await?;
    
    println!("Rust 用户: {:?}", users);
    
    Ok(())
}
```

## 查询操作

### 基本查询

```rust
use sqlx::{query, query_as, Row};
use sqlx::postgres::PgPool;

async fn basic_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 执行查询
    let result = query("CREATE TABLE IF NOT EXISTS posts (id SERIAL PRIMARY KEY, title TEXT, content TEXT)")
        .execute(pool)
        .await?;
    
    println!("创建表结果: {:?}", result);
    
    // 插入数据
    let result = query("INSERT INTO posts (title, content) VALUES ($1, $2)")
        .bind("第一篇文章")
        .bind("这是第一篇文章的内容")
        .execute(pool)
        .await?;
    
    println!("插入结果: {:?}", result);
    
    // 查询单行
    let row = query("SELECT id, title, content FROM posts WHERE id = $1")
        .bind(1)
        .fetch_one(pool)
        .await?;
    
    let id: i32 = row.get("id");
    let title: String = row.get("title");
    let content: String = row.get("content");
    
    println!("文章: {} - {} - {}", id, title, content);
    
    // 查询多行
    let rows = query("SELECT id, title FROM posts")
        .fetch_all(pool)
        .await?;
    
    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        println!("文章: {} - {}", id, title);
    }
    
    Ok(())
}
```

### 结构化查询

```rust
use sqlx::{query_as, FromRow};

#[derive(Debug, FromRow)]
struct Post {
    id: i32,
    title: String,
    content: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn structured_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 查询到结构体
    let posts = query_as::<_, Post>("SELECT id, title, content, created_at FROM posts")
        .fetch_all(pool)
        .await?;
    
    println!("所有文章: {:?}", posts);
    
    // 条件查询
    let post = query_as::<_, Post>(
        "SELECT id, title, content, created_at FROM posts WHERE title LIKE $1"
    )
    .bind("%第一%")
    .fetch_optional(pool)
    .await?;
    
    match post {
        Some(post) => println!("找到文章: {:?}", post),
        None => println!("没有找到文章"),
    }
    
    // 分页查询
    let posts = query_as::<_, Post>(
        "SELECT id, title, content, created_at FROM posts ORDER BY id LIMIT $1 OFFSET $2"
    )
    .bind(10)
    .bind(0)
    .fetch_all(pool)
    .await?;
    
    println!("第一页文章: {:?}", posts);
    
    Ok(())
}
```

### 流式查询

```rust
use sqlx::{query_as, Row};
use futures::stream::StreamExt;

async fn streaming_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 流式处理大量数据
    let mut stream = query_as::<_, Post>("SELECT id, title, content, created_at FROM posts")
        .fetch(pool);
    
    while let Some(post) = stream.next().await {
        match post {
            Ok(post) => {
                println!("处理文章: {}", post.title);
                // 处理单个文章
            }
            Err(e) => {
                eprintln!("查询错误: {}", e);
                break;
            }
        }
    }
    
    // 流式处理原始行
    let mut stream = query("SELECT id, title FROM posts").fetch(pool);
    
    while let Some(row) = stream.next().await {
        match row {
            Ok(row) => {
                let id: i32 = row.get("id");
                let title: String = row.get("title");
                println!("处理: {} - {}", id, title);
            }
            Err(e) => {
                eprintln!("查询错误: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

## 事务处理

### 基本事务

```rust
use sqlx::{query, query_as, Acquire};

async fn basic_transaction(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    // 在事务中执行操作
    let post = query_as::<_, Post>(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at"
    )
    .bind("事务文章")
    .bind("这是在事务中创建的文章")
    .fetch_one(&mut *tx)
    .await?;
    
    println!("创建文章: {:?}", post);
    
    // 更新操作
    let result = query("UPDATE posts SET title = $1 WHERE id = $2")
        .bind("事务文章（更新）")
        .bind(post.id)
        .execute(&mut *tx)
        .await?;
    
    println!("更新结果: {:?}", result);
    
    // 提交事务
    tx.commit().await?;
    
    println!("事务提交成功");
    
    Ok(())
}
```

### 事务回滚

```rust
use sqlx::{query, query_as};

async fn transaction_rollback(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    let result = async {
        // 创建文章
        let post = query_as::<_, Post>(
            "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at"
        )
        .bind("回滚文章")
        .bind("这篇文章会被回滚")
        .fetch_one(&mut *tx)
        .await?;
        
        println!("创建文章: {:?}", post);
        
        // 模拟错误条件
        if post.title.contains("回滚") {
            return Err(sqlx::Error::RowNotFound);
        }
        
        Ok(post)
    }.await;
    
    match result {
        Ok(post) => {
            tx.commit().await?;
            println!("事务提交: {:?}", post);
        }
        Err(e) => {
            tx.rollback().await?;
            println!("事务回滚: {}", e);
        }
    }
    
    Ok(())
}
```

### 保存点

```rust
use sqlx::{query, query_as, Acquire};

async fn savepoint_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    // 创建第一篇文章
    let post1 = query_as::<_, Post>(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at"
    )
    .bind("文章1")
    .bind("第一篇文章")
    .fetch_one(&mut *tx)
    .await?;
    
    println!("创建文章1: {:?}", post1);
    
    // 创建保存点
    let savepoint = tx.begin().await?;
    
    let inner_result = async {
        // 创建第二篇文章
        let post2 = query_as::<_, Post>(
            "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at"
        )
        .bind("文章2")
        .bind("第二篇文章")
        .fetch_one(&mut *savepoint)
        .await?;
        
        println!("创建文章2: {:?}", post2);
        
        // 模拟错误
        if post2.title.contains("2") {
            return Err(sqlx::Error::RowNotFound);
        }
        
        Ok(post2)
    }.await;
    
    match inner_result {
        Ok(_) => {
            savepoint.commit().await?;
            println!("保存点提交");
        }
        Err(e) => {
            savepoint.rollback().await?;
            println!("保存点回滚: {}", e);
        }
    }
    
    // 提交外层事务
    tx.commit().await?;
    
    println!("外层事务提交");
    
    Ok(())
}
```

## 连接池

### 连接池配置

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub struct DatabasePool {
    pool: Pool<Postgres>,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(3600))
            .test_before_acquire(true)
            .connect(database_url)
            .await?;
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    
    pub async fn close(&self) {
        self.pool.close().await;
    }
    
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
```

### 连接池监控

```rust
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub struct PoolMonitor {
    pool: Pool<Postgres>,
}

impl PoolMonitor {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn monitor_pool(&self) {
        loop {
            let size = self.pool.size();
            let idle = self.pool.num_idle();
            let active = size - idle;
            
            println!("连接池状态: 总连接数={}, 活跃连接数={}, 空闲连接数={}", 
                     size, active, idle);
            
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }
    
    pub async fn stress_test(&self, concurrent_requests: usize) -> Result<(), sqlx::Error> {
        let mut handles = Vec::new();
        
        for i in 0..concurrent_requests {
            let pool = self.pool.clone();
            let handle = tokio::spawn(async move {
                let result = query("SELECT $1 as id")
                    .bind(i as i32)
                    .fetch_one(&pool)
                    .await;
                
                match result {
                    Ok(row) => {
                        let id: i32 = row.get("id");
                        println!("请求 {} 完成: {}", i, id);
                    }
                    Err(e) => {
                        eprintln!("请求 {} 失败: {}", i, e);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
        
        Ok(())
    }
}
```

## 类型映射

### 自定义类型

```rust
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Type};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

impl Type<Postgres> for UserStatus {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("user_status")
    }
}

impl Encode<'_, Postgres> for UserStatus {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        let status_str = match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
        };
        
        <String as Encode<Postgres>>::encode(status_str.to_string(), buf)
    }
}

impl Decode<'_, Postgres> for UserStatus {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let status_str = <String as Decode<Postgres>>::decode(value)?;
        
        match status_str.as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            _ => Err(format!("Unknown status: {}", status_str).into()),
        }
    }
}

#[derive(Debug, FromRow)]
struct UserWithStatus {
    id: i32,
    name: String,
    email: String,
    status: UserStatus,
}

async fn custom_type_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 创建枚举类型
    query("CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended')")
        .execute(pool)
        .await?;
    
    // 创建表
    query(
        r#"
        CREATE TABLE users_with_status (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            status user_status DEFAULT 'active'
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // 插入数据
    let user = query_as::<_, UserWithStatus>(
        "INSERT INTO users_with_status (name, email, status) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind("张三")
    .bind("zhangsan@example.com")
    .bind(UserStatus::Active)
    .fetch_one(pool)
    .await?;
    
    println!("创建用户: {:?}", user);
    
    // 查询数据
    let users = query_as::<_, UserWithStatus>(
        "SELECT * FROM users_with_status WHERE status = $1"
    )
    .bind(UserStatus::Active)
    .fetch_all(pool)
    .await?;
    
    println!("活跃用户: {:?}", users);
    
    Ok(())
}
```

### JSON 类型

```rust
use sqlx::types::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserPreferences {
    theme: String,
    language: String,
    notifications: bool,
    features: Vec<String>,
}

#[derive(Debug, FromRow)]
struct UserWithPreferences {
    id: i32,
    name: String,
    email: String,
    preferences: Json<UserPreferences>,
}

async fn json_type_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 创建表
    query(
        r#"
        CREATE TABLE users_with_preferences (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            preferences JSONB NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // 插入数据
    let preferences = UserPreferences {
        theme: "dark".to_string(),
        language: "zh-CN".to_string(),
        notifications: true,
        features: vec!["beta".to_string(), "experimental".to_string()],
    };
    
    let user = query_as::<_, UserWithPreferences>(
        "INSERT INTO users_with_preferences (name, email, preferences) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind("李四")
    .bind("lisi@example.com")
    .bind(Json(&preferences))
    .fetch_one(pool)
    .await?;
    
    println!("创建用户: {:?}", user);
    
    // JSON 查询
    let users = query_as::<_, UserWithPreferences>(
        "SELECT * FROM users_with_preferences WHERE preferences ->> 'theme' = $1"
    )
    .bind("dark")
    .fetch_all(pool)
    .await?;
    
    println!("深色主题用户: {:?}", users);
    
    Ok(())
}
```

## 迁移系统

### 迁移文件

```rust
// migrations/001_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

// migrations/002_create_posts.sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_published ON posts(published);
```

### 迁移代码

```rust
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

async fn run_migrations() -> Result<(), sqlx::Error> {
    let database_url = "sqlite:test.db";
    
    // 创建数据库（如果不存在）
    if !Sqlite::database_exists(database_url).await? {
        Sqlite::create_database(database_url).await?;
        println!("数据库创建成功");
    }
    
    // 连接数据库
    let pool = SqlitePool::connect(database_url).await?;
    
    // 运行迁移
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    println!("迁移完成");
    
    Ok(())
}
```

### 嵌入式迁移

```rust
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!(); // 默认从 ./migrations 目录

async fn embedded_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 运行嵌入式迁移
    MIGRATOR.run(pool).await?;
    
    println!("嵌入式迁移完成");
    
    Ok(())
}

// 自定义迁移目录
static CUSTOM_MIGRATOR: Migrator = sqlx::migrate!("./custom_migrations");

async fn custom_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    CUSTOM_MIGRATOR.run(pool).await?;
    
    println!("自定义迁移完成");
    
    Ok(())
}
```

## 异步流处理

### 基本流操作

```rust
use sqlx::{query, query_as, Row};
use futures::stream::StreamExt;

async fn stream_processing(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 创建大量测试数据
    for i in 0..1000 {
        query("INSERT INTO posts (title, content) VALUES ($1, $2)")
            .bind(format!("文章 {}", i))
            .bind(format!("这是第 {} 篇文章的内容", i))
            .execute(pool)
            .await?;
    }
    
    // 流式处理
    let mut stream = query_as::<_, Post>("SELECT id, title, content, created_at FROM posts")
        .fetch(pool);
    
    let mut count = 0;
    while let Some(post) = stream.next().await {
        match post {
            Ok(post) => {
                count += 1;
                if count % 100 == 0 {
                    println!("已处理 {} 篇文章", count);
                }
                
                // 处理单篇文章
                process_post(&post).await;
            }
            Err(e) => {
                eprintln!("处理文章时出错: {}", e);
                break;
            }
        }
    }
    
    println!("总共处理了 {} 篇文章", count);
    
    Ok(())
}

async fn process_post(post: &Post) {
    // 模拟文章处理
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    
    // 这里可以进行实际的文章处理逻辑
    println!("处理文章: {}", post.title);
}
```

### 并发流处理

```rust
use sqlx::{query_as, Row};
use futures::stream::{StreamExt, TryStreamExt};
use std::sync::Arc;

async fn concurrent_stream_processing(pool: &PgPool) -> Result<(), sqlx::Error> {
    let pool = Arc::new(pool.clone());
    
    // 并发处理流
    let results = query_as::<_, Post>("SELECT id, title, content, created_at FROM posts")
        .fetch(pool.as_ref())
        .map(|result| {
            let pool = pool.clone();
            async move {
                match result {
                    Ok(post) => {
                        // 并发处理每个文章
                        let enhanced_post = enhance_post(&pool, post).await?;
                        Ok(enhanced_post)
                    }
                    Err(e) => Err(e),
                }
            }
        })
        .buffer_unordered(10) // 最多并发 10 个任务
        .try_collect::<Vec<_>>()
        .await?;
    
    println!("并发处理完成，共处理 {} 篇文章", results.len());
    
    Ok(())
}

async fn enhance_post(pool: &PgPool, mut post: Post) -> Result<Post, sqlx::Error> {
    // 模拟文章增强处理
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    // 更新文章
    query("UPDATE posts SET content = $1 WHERE id = $2")
        .bind(format!("{} (已增强)", post.content))
        .bind(post.id)
        .execute(pool)
        .await?;
    
    post.content = format!("{} (已增强)", post.content);
    
    Ok(post)
}
```

## 错误处理

### 错误类型

```rust
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("连接错误: {0}")]
    Connection(#[from] SqlxError),
    #[error("数据验证错误: {0}")]
    Validation(String),
    #[error("业务逻辑错误: {0}")]
    Business(String),
    #[error("资源不存在: {0}")]
    NotFound(String),
    #[error("权限不足: {0}")]
    Forbidden(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

impl From<SqlxError> for DatabaseError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => DatabaseError::NotFound("记录不存在".to_string()),
            SqlxError::Database(db_err) => {
                if db_err.is_unique_violation() {
                    DatabaseError::Validation("唯一约束违反".to_string())
                } else if db_err.is_foreign_key_violation() {
                    DatabaseError::Validation("外键约束违反".to_string())
                } else {
                    DatabaseError::Connection(SqlxError::Database(db_err))
                }
            }
            _ => DatabaseError::Connection(err),
        }
    }
}
```

### 错误处理示例

```rust
use sqlx::{query_as, query};

async fn error_handling_example(pool: &PgPool) -> Result<()> {
    // 处理唯一约束冲突
    let result = query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind("张三")
    .bind("zhangsan@example.com")
    .fetch_one(pool)
    .await;
    
    match result {
        Ok(user) => println!("创建用户成功: {:?}", user),
        Err(SqlxError::Database(db_err)) if db_err.is_unique_violation() => {
            println!("用户已存在，尝试更新...");
            
            let user = query_as::<_, User>(
                "UPDATE users SET name = $1 WHERE email = $2 RETURNING *"
            )
            .bind("张三")
            .bind("zhangsan@example.com")
            .fetch_one(pool)
            .await?;
            
            println!("更新用户成功: {:?}", user);
        }
        Err(e) => return Err(e.into()),
    }
    
    // 处理行不存在
    let user = query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(999)
        .fetch_optional(pool)
        .await?;
    
    match user {
        Some(user) => println!("找到用户: {:?}", user),
        None => println!("用户不存在"),
    }
    
    Ok(())
}
```

## 测试支持

### 测试配置

```rust
use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub async fn setup_test_db() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/test_db".to_string());
    
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    // 运行迁移
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    pool
}

pub async fn cleanup_test_db(pool: &PgPool) {
    // 清理测试数据
    sqlx::query("TRUNCATE users, posts CASCADE")
        .execute(pool)
        .await
        .unwrap();
}
```

### 测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    
    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_test_db().await;
        
        // 测试创建用户
        let user = query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
        )
        .bind("测试用户")
        .bind("test@example.com")
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_transaction_rollback() {
        let pool = setup_test_db().await;
        
        let result = async {
            let mut tx = pool.begin().await?;
            
            // 创建用户
            let user = query_as::<_, User>(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
            )
            .bind("事务用户")
            .bind("transaction@example.com")
            .fetch_one(&mut *tx)
            .await?;
            
            // 模拟错误
            if user.name.contains("事务") {
                return Err(sqlx::Error::RowNotFound);
            }
            
            tx.commit().await?;
            Ok(user)
        }.await;
        
        // 验证事务回滚
        assert!(result.is_err());
        
        let count = query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(count, 0);
        
        cleanup_test_db(&pool).await;
    }
}
```

## 宏系统

### 查询宏详解

```rust
use sqlx::{query, query_as, query_scalar, query_unchecked};

// 基本查询宏
async fn query_macros_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    // query! - 编译时检查的查询
    let result = query!("SELECT id, name, email FROM users WHERE id = $1", 1)
        .fetch_optional(pool)
        .await?;
    
    if let Some(row) = result {
        println!("用户: {} - {}", row.id, row.name);
    }
    
    // query_as! - 编译时检查的结构体映射
    let user = query_as!(
        User,
        "SELECT id, name, email FROM users WHERE email = $1",
        "test@example.com"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some(user) = user {
        println!("用户: {:?}", user);
    }
    
    // query_scalar! - 标量查询
    let count = query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    
    println!("用户数量: {:?}", count);
    
    // query_unchecked! - 不检查的查询（动态SQL）
    let table_name = "users";
    let rows = query_unchecked!(&format!("SELECT * FROM {}", table_name))
        .fetch_all(pool)
        .await?;
    
    println!("查询到 {} 行", rows.len());
    
    Ok(())
}
```

### 自定义宏

```rust
// 定义查询宏
macro_rules! find_by_id {
    ($pool:expr, $table:literal, $id:expr) => {
        query(&format!("SELECT * FROM {} WHERE id = $1", $table))
            .bind($id)
            .fetch_optional($pool)
    };
}

macro_rules! count_table {
    ($pool:expr, $table:literal) => {
        query_scalar(&format!("SELECT COUNT(*) FROM {}", $table))
            .fetch_one($pool)
    };
}

async fn custom_macro_example(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 使用自定义宏
    let user_row = find_by_id!(pool, "users", 1).await?;
    
    if let Some(row) = user_row {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("用户: {} - {}", id, name);
    }
    
    // 计数宏
    let count: i64 = count_table!(pool, "users").await?;
    println!("用户数量: {}", count);
    
    Ok(())
}
```

## 实战案例

### 用户管理系统

```rust
use sqlx::{query, query_as, query_scalar, PgPool};
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, sqlx::Error> {
        // 检查用户名是否已存在
        let existing_user = query_scalar!(
            "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2",
            request.username,
            request.email
        )
        .fetch_one(&self.pool)
        .await?;
        
        if existing_user.unwrap_or(0) > 0 {
            return Err(sqlx::Error::RowNotFound); // 自定义错误处理
        }
        
        // 哈希密码
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(request.password.as_bytes(), &salt)
            .map_err(|_| sqlx::Error::RowNotFound)?
            .to_string();
        
        // 创建用户
        let user = query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, password_hash, is_active, created_at, updated_at
            "#,
            request.username,
            request.email,
            password_hash,
            true,
            chrono::Utc::now(),
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn authenticate_user(&self, request: LoginRequest) -> Result<Option<User>, sqlx::Error> {
        let user = query_as!(
            User,
            "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE username = $1 AND is_active = true",
            request.username
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(user) = user {
            // 验证密码
            let parsed_hash = PasswordHash::new(&user.password_hash)
                .map_err(|_| sqlx::Error::RowNotFound)?;
            
            if Argon2::default()
                .verify_password(request.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                return Ok(Some(user));
            }
        }
        
        Ok(None)
    }
    
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = query_as!(
            User,
            "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn update_user_email(&self, id: i32, new_email: String) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "UPDATE users SET email = $1, updated_at = $2 WHERE id = $3 RETURNING id, username, email, password_hash, is_active, created_at, updated_at",
            new_email,
            chrono::Utc::now(),
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn deactivate_user(&self, id: i32) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE users SET is_active = false, updated_at = $1 WHERE id = $2",
            chrono::Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_users_paginated(&self, page: i64, page_size: i64) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        
        let users = query_as!(
            User,
            "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(users)
    }
}
```

### 订单处理系统

```rust
use sqlx::{query, query_as, query_scalar, PgPool, Acquire};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Order {
    id: Uuid,
    user_id: i32,
    total_amount: rust_decimal::Decimal,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct OrderItem {
    id: Uuid,
    order_id: Uuid,
    product_id: i32,
    quantity: i32,
    unit_price: rust_decimal::Decimal,
    total_price: rust_decimal::Decimal,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Product {
    id: i32,
    name: String,
    price: rust_decimal::Decimal,
    stock_quantity: i32,
}

#[derive(Debug, Deserialize)]
struct CreateOrderRequest {
    user_id: i32,
    items: Vec<OrderItemRequest>,
}

#[derive(Debug, Deserialize)]
struct OrderItemRequest {
    product_id: i32,
    quantity: i32,
}

pub struct OrderService {
    pool: PgPool,
}

impl OrderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        // 验证产品库存并计算总价
        let mut total_amount = rust_decimal::Decimal::new(0, 0);
        let mut order_items = Vec::new();
        
        for item in &request.items {
            let product = query_as!(
                Product,
                "SELECT id, name, price, stock_quantity FROM products WHERE id = $1 FOR UPDATE",
                item.product_id
            )
            .fetch_one(&mut *tx)
            .await?;
            
            if product.stock_quantity < item.quantity {
                return Err(sqlx::Error::RowNotFound); // 库存不足
            }
            
            let unit_price = product.price;
            let total_price = unit_price * rust_decimal::Decimal::new(item.quantity as i64, 0);
            total_amount += total_price;
            
            order_items.push((item.product_id, item.quantity, unit_price, total_price));
        }
        
        // 创建订单
        let order_id = Uuid::new_v4();
        let order = query_as!(
            Order,
            r#"
            INSERT INTO orders (id, user_id, total_amount, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, total_amount, status, created_at, updated_at
            "#,
            order_id,
            request.user_id,
            total_amount,
            "pending",
            chrono::Utc::now(),
            chrono::Utc::now()
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // 创建订单项目并更新库存
        for (product_id, quantity, unit_price, total_price) in order_items {
            // 创建订单项
            query!(
                r#"
                INSERT INTO order_items (id, order_id, product_id, quantity, unit_price, total_price)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                Uuid::new_v4(),
                order_id,
                product_id,
                quantity,
                unit_price,
                total_price
            )
            .execute(&mut *tx)
            .await?;
            
            // 更新库存
            query!(
                "UPDATE products SET stock_quantity = stock_quantity - $1 WHERE id = $2",
                quantity,
                product_id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        
        Ok(order)
    }
    
    pub async fn get_order_with_items(&self, order_id: Uuid) -> Result<Option<(Order, Vec<OrderItem>)>, sqlx::Error> {
        let order = query_as!(
            Order,
            "SELECT id, user_id, total_amount, status, created_at, updated_at FROM orders WHERE id = $1",
            order_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(order) = order {
            let items = query_as!(
                OrderItem,
                "SELECT id, order_id, product_id, quantity, unit_price, total_price FROM order_items WHERE order_id = $1",
                order_id
            )
            .fetch_all(&self.pool)
            .await?;
            
            Ok(Some((order, items)))
        } else {
            Ok(None)
        }
    }
    
    pub async fn update_order_status(&self, order_id: Uuid, new_status: String) -> Result<Order, sqlx::Error> {
        let order = query_as!(
            Order,
            "UPDATE orders SET status = $1, updated_at = $2 WHERE id = $3 RETURNING id, user_id, total_amount, status, created_at, updated_at",
            new_status,
            chrono::Utc::now(),
            order_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(order)
    }
    
    pub async fn get_user_orders(&self, user_id: i32, page: i64, page_size: i64) -> Result<Vec<Order>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        
        let orders = query_as!(
            Order,
            "SELECT id, user_id, total_amount, status, created_at, updated_at FROM orders WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            user_id,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(orders)
    }
}
```

## 最佳实践

### 1. 项目结构

```
src/
├── main.rs
├── lib.rs
├── config/
│   ├── mod.rs
│   └── database.rs
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── order.rs
├── services/
│   ├── mod.rs
│   ├── user_service.rs
│   └── order_service.rs
├── utils/
│   ├── mod.rs
│   └── error.rs
└── migrations/
    ├── 001_create_users.sql
    └── 002_create_orders.sql
```

### 2. 错误处理

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    #[error("验证错误: {0}")]
    Validation(String),
    #[error("业务错误: {0}")]
    Business(String),
    #[error("未找到: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### 3. 配置管理

```rust
use sqlx::PgPool;
use std::env;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a number"),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .expect("DB_MIN_CONNECTIONS must be a number"),
        }
    }
    
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect(&self.database_url)
            .await
    }
}
```

### 4. 测试配置

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    
    async fn setup_test_pool() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/test_db".to_string());
        
        let pool = PgPool::connect(&database_url).await.unwrap();
        
        // 运行迁移
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        pool
    }
    
    #[tokio::test]
    async fn test_database_operations() {
        let pool = setup_test_pool().await;
        
        // 测试逻辑
        
        // 清理
        sqlx::query("TRUNCATE TABLE users CASCADE")
            .execute(&pool)
            .await
            .unwrap();
    }
}
```

## 总结

SQLx 是一个功能强大的 Rust SQL 工具包，提供了编译时检查的类型安全数据库操作。通过本教程，您应该能够：

1. 理解 SQLx 的核心概念和特性
2. 配置和管理数据库连接
3. 使用编译时检查的查询
4. 实现 CRUD 操作和事务处理
5. 处理复杂的类型映射
6. 使用迁移系统管理数据库结构
7. 实现异步流处理和错误处理

关键要点：
- 编译时 SQL 查询验证
- 异步优先的设计
- 类型安全的结果映射
- 灵活的连接池管理
- 强大的宏系统

SQLx 的设计理念是在保持 SQL 原生性的同时提供类型安全，它是 Rust 生态系统中最受欢迎的数据库工具之一。