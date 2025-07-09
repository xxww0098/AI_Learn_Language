# Diesel 2.2.11 - Rust ORM 和查询构建器完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [数据库配置](#数据库配置)
- [迁移系统](#迁移系统)
- [模型定义](#模型定义)
- [查询构建器](#查询构建器)
- [CRUD 操作](#crud-操作)
- [关系映射](#关系映射)
- [事务处理](#事务处理)
- [异步支持](#异步支持)
- [连接池](#连接池)
- [自定义类型](#自定义类型)
- [SQL 函数](#sql-函数)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Diesel 是一个安全、可扩展的 ORM 和查询构建器，支持 PostgreSQL、SQLite 和 MySQL。它提供了类型安全的查询接口，在编译时检查 SQL 查询的正确性。

### 核心特性
- **类型安全**: 编译时 SQL 查询验证
- **零成本抽象**: 高性能的查询执行
- **多数据库支持**: PostgreSQL、MySQL、SQLite
- **强大的查询构建器**: 表达式查询接口
- **迁移系统**: 数据库 schema 版本管理
- **可扩展性**: 自定义类型和函数支持

### 版本信息
- **当前版本**: 2.2.11
- **发布时间**: 2025-06-18
- **下载次数**: 16,290,904+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
diesel = { version = "2.2.11", features = ["postgres", "chrono"] }
diesel_migrations = "2.2.11"
dotenvy = "0.15"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
diesel_migrations = "2.2.11"
```

### 安装 Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### 基本项目结构

```
my_project/
├── Cargo.toml
├── .env
├── diesel.toml
├── migrations/
│   └── 00000000000000_diesel_initial_setup/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── schema.rs
│   └── models.rs
```

### 环境配置

```bash
# .env
DATABASE_URL=postgres://username:password@localhost/database_name
```

```toml
# diesel.toml
[print_schema]
file = "src/schema.rs"
with_docs = false
filter = { only_tables = true }
```

## 数据库配置

### 数据库初始化

```bash
# 创建数据库
diesel setup

# 创建迁移
diesel migration generate create_users
```

### 连接配置

```rust
// src/lib.rs
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
```

### 多数据库支持

```rust
// PostgreSQL
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pg_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// SQLite
use diesel::sqlite::SqliteConnection;

pub fn establish_sqlite_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "test.db".to_string());
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// MySQL
use diesel::mysql::MysqlConnection;

pub fn establish_mysql_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
```

## 迁移系统

### 创建迁移

```sql
-- migrations/2024-01-01-000001_create_users/up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- migrations/2024-01-01-000001_create_users/down.sql
DROP TABLE users;
```

```sql
-- migrations/2024-01-01-000002_create_posts/up.sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    user_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- migrations/2024-01-01-000002_create_posts/down.sql
DROP TABLE posts;
```

### 运行迁移

```bash
# 运行所有未执行的迁移
diesel migration run

# 回滚最后一次迁移
diesel migration revert

# 重新运行迁移
diesel migration redo
```

### 嵌入式迁移

```rust
// src/main.rs
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut PgConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

fn main() {
    let mut connection = establish_connection();
    run_migrations(&mut connection);
    println!("迁移完成！");
}
```

## 模型定义

### 基本模型

```rust
// src/models.rs
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub published: Option<bool>,
    pub user_id: i32,
}
```

### Schema 定义

```rust
// src/schema.rs
// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
```

## 查询构建器

### 基本查询

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users::dsl::*;

fn basic_queries(conn: &mut PgConnection) -> QueryResult<()> {
    // 查询所有用户
    let all_users = users.load::<User>(conn)?;
    println!("所有用户: {:#?}", all_users);
    
    // 查询单个用户
    let user = users.find(1).first::<User>(conn)?;
    println!("用户: {:#?}", user);
    
    // 条件查询
    let filtered_users = users
        .filter(name.eq("张三"))
        .load::<User>(conn)?;
    println!("过滤用户: {:#?}", filtered_users);
    
    // 排序
    let sorted_users = users
        .order(created_at.desc())
        .load::<User>(conn)?;
    println!("排序用户: {:#?}", sorted_users);
    
    // 限制结果
    let limited_users = users
        .limit(10)
        .offset(0)
        .load::<User>(conn)?;
    println!("分页用户: {:#?}", limited_users);
    
    Ok(())
}
```

### 复杂查询

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::{users, posts};

fn complex_queries(conn: &mut PgConnection) -> QueryResult<()> {
    // 连接查询
    let results = users::table
        .inner_join(posts::table)
        .select((users::all_columns, posts::all_columns))
        .load::<(User, Post)>(conn)?;
    
    for (user, post) in results {
        println!("用户: {} - 文章: {}", user.name, post.title);
    }
    
    // 左连接
    let results = users::table
        .left_join(posts::table)
        .select((users::all_columns, posts::all_columns.nullable()))
        .load::<(User, Option<Post>)>(conn)?;
    
    for (user, maybe_post) in results {
        match maybe_post {
            Some(post) => println!("用户: {} - 文章: {}", user.name, post.title),
            None => println!("用户: {} - 无文章", user.name),
        }
    }
    
    // 聚合查询
    let count = users::table.count().get_result::<i64>(conn)?;
    println!("用户总数: {}", count);
    
    // 分组查询
    let post_counts = posts::table
        .group_by(posts::user_id)
        .select((posts::user_id, diesel::dsl::count(posts::id)))
        .load::<(i32, i64)>(conn)?;
    
    for (user_id, count) in post_counts {
        println!("用户 {} 的文章数: {}", user_id, count);
    }
    
    Ok(())
}
```

### 子查询

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::{users, posts};

fn subquery_examples(conn: &mut PgConnection) -> QueryResult<()> {
    // 子查询 - 查找有文章的用户
    let users_with_posts = users::table
        .filter(
            users::id.eq_any(
                posts::table.select(posts::user_id).distinct()
            )
        )
        .load::<User>(conn)?;
    
    println!("有文章的用户: {:#?}", users_with_posts);
    
    // EXISTS 子查询
    let users_with_published_posts = users::table
        .filter(
            diesel::dsl::exists(
                posts::table.filter(
                    posts::user_id.eq(users::id)
                    .and(posts::published.eq(true))
                )
            )
        )
        .load::<User>(conn)?;
    
    println!("有已发布文章的用户: {:#?}", users_with_published_posts);
    
    // 标量子查询
    let users_with_post_count = users::table
        .select((
            users::all_columns,
            posts::table
                .filter(posts::user_id.eq(users::id))
                .count()
                .single_value()
        ))
        .load::<(User, Option<i64>)>(conn)?;
    
    for (user, count) in users_with_post_count {
        println!("用户: {} - 文章数: {:?}", user.name, count.unwrap_or(0));
    }
    
    Ok(())
}
```

## CRUD 操作

### 创建 (Create)

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users;

fn create_operations(conn: &mut PgConnection) -> QueryResult<()> {
    // 创建单个记录
    let new_user = NewUser {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?;
    
    println!("创建用户: {:#?}", user);
    
    // 批量创建
    let new_users = vec![
        NewUser {
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
        },
        NewUser {
            name: "王五".to_string(),
            email: "wangwu@example.com".to_string(),
        },
    ];
    
    let users: Vec<User> = diesel::insert_into(users::table)
        .values(&new_users)
        .get_results(conn)?;
    
    println!("批量创建用户: {:#?}", users);
    
    // 冲突处理 (PostgreSQL)
    let user_result = diesel::insert_into(users::table)
        .values(&new_user)
        .on_conflict(users::email)
        .do_update()
        .set(users::name.eq("张三（更新）"))
        .get_result::<User>(conn);
    
    match user_result {
        Ok(user) => println!("冲突处理后的用户: {:#?}", user),
        Err(e) => println!("冲突处理失败: {}", e),
    }
    
    Ok(())
}
```

### 查询 (Read)

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users::dsl::*;

fn read_operations(conn: &mut PgConnection) -> QueryResult<()> {
    // 查询所有记录
    let all_users = users.load::<User>(conn)?;
    println!("所有用户: {:#?}", all_users);
    
    // 查询单个记录
    let user = users.find(1).first::<User>(conn)?;
    println!("用户 1: {:#?}", user);
    
    // 条件查询
    let filtered_users = users
        .filter(name.like("%张%"))
        .load::<User>(conn)?;
    println!("名字包含'张'的用户: {:#?}", filtered_users);
    
    // 范围查询
    let recent_users = users
        .filter(created_at.gt(chrono::Utc::now() - chrono::Duration::days(30)))
        .load::<User>(conn)?;
    println!("最近 30 天的用户: {:#?}", recent_users);
    
    // 排序和分页
    let paginated_users = users
        .order(created_at.desc())
        .limit(10)
        .offset(0)
        .load::<User>(conn)?;
    println!("分页用户: {:#?}", paginated_users);
    
    // 计数
    let user_count = users.count().get_result::<i64>(conn)?;
    println!("用户总数: {}", user_count);
    
    Ok(())
}
```

### 更新 (Update)

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users::dsl::*;

fn update_operations(conn: &mut PgConnection) -> QueryResult<()> {
    // 更新单个记录
    let updated_user = diesel::update(users.find(1))
        .set(name.eq("张三（更新）"))
        .get_result::<User>(conn)?;
    
    println!("更新用户: {:#?}", updated_user);
    
    // 批量更新
    let updated_count = diesel::update(users.filter(name.like("%测试%")))
        .set(name.eq("测试用户"))
        .execute(conn)?;
    
    println!("批量更新了 {} 个用户", updated_count);
    
    // 使用 AsChangeset
    let update_data = UpdateUser {
        name: Some("新名字".to_string()),
        email: None,
    };
    
    let updated_user = diesel::update(users.find(1))
        .set(&update_data)
        .get_result::<User>(conn)?;
    
    println!("使用 AsChangeset 更新: {:#?}", updated_user);
    
    // 条件更新
    let updated_count = diesel::update(users)
        .filter(created_at.lt(chrono::Utc::now() - chrono::Duration::days(365)))
        .set(name.eq(name.concat(" (旧用户)")))
        .execute(conn)?;
    
    println!("条件更新了 {} 个用户", updated_count);
    
    Ok(())
}
```

### 删除 (Delete)

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users::dsl::*;

fn delete_operations(conn: &mut PgConnection) -> QueryResult<()> {
    // 删除单个记录
    let deleted_user = diesel::delete(users.find(1))
        .get_result::<User>(conn)?;
    
    println!("删除用户: {:#?}", deleted_user);
    
    // 批量删除
    let deleted_count = diesel::delete(users.filter(name.like("%测试%")))
        .execute(conn)?;
    
    println!("批量删除了 {} 个用户", deleted_count);
    
    // 条件删除
    let deleted_count = diesel::delete(users)
        .filter(created_at.lt(chrono::Utc::now() - chrono::Duration::days(365)))
        .execute(conn)?;
    
    println!("条件删除了 {} 个用户", deleted_count);
    
    // 删除所有记录
    let deleted_count = diesel::delete(users).execute(conn)?;
    println!("删除了所有 {} 个用户", deleted_count);
    
    Ok(())
}
```

## 关系映射

### 一对多关系

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::{users, posts};

impl User {
    pub fn posts(&self, conn: &mut PgConnection) -> QueryResult<Vec<Post>> {
        posts::table
            .filter(posts::user_id.eq(self.id))
            .load::<Post>(conn)
    }
    
    pub fn published_posts(&self, conn: &mut PgConnection) -> QueryResult<Vec<Post>> {
        posts::table
            .filter(posts::user_id.eq(self.id))
            .filter(posts::published.eq(true))
            .load::<Post>(conn)
    }
}

impl Post {
    pub fn author(&self, conn: &mut PgConnection) -> QueryResult<User> {
        users::table
            .filter(users::id.eq(self.user_id))
            .first::<User>(conn)
    }
}

fn relationship_examples(conn: &mut PgConnection) -> QueryResult<()> {
    // 查询用户及其文章
    let user = users::table.first::<User>(conn)?;
    let user_posts = user.posts(conn)?;
    
    println!("用户: {}", user.name);
    for post in user_posts {
        println!("  - 文章: {}", post.title);
    }
    
    // 查询文章及其作者
    let post = posts::table.first::<Post>(conn)?;
    let author = post.author(conn)?;
    
    println!("文章: {} - 作者: {}", post.title, author.name);
    
    // 连接查询
    let results = users::table
        .inner_join(posts::table)
        .select((users::all_columns, posts::all_columns))
        .load::<(User, Post)>(conn)?;
    
    for (user, post) in results {
        println!("用户: {} - 文章: {}", user.name, post.title);
    }
    
    Ok(())
}
```

### 多对多关系

```rust
// 添加中间表
diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::user_roles)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn roles(&self, conn: &mut PgConnection) -> QueryResult<Vec<Role>> {
        use crate::schema::{roles, user_roles};
        
        roles::table
            .inner_join(user_roles::table.on(roles::id.eq(user_roles::role_id)))
            .filter(user_roles::user_id.eq(self.id))
            .select(roles::all_columns)
            .load::<Role>(conn)
    }
}

impl Role {
    pub fn users(&self, conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        use crate::schema::{users, user_roles};
        
        users::table
            .inner_join(user_roles::table.on(users::id.eq(user_roles::user_id)))
            .filter(user_roles::role_id.eq(self.id))
            .select(users::all_columns)
            .load::<User>(conn)
    }
}
```

## 事务处理

### 基本事务

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::{users, posts};

fn transaction_examples(conn: &mut PgConnection) -> QueryResult<()> {
    // 基本事务
    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let new_user = NewUser {
            name: "事务用户".to_string(),
            email: "transaction@example.com".to_string(),
        };
        
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)?;
        
        let new_post = NewPost {
            title: "事务文章".to_string(),
            content: "这是一篇在事务中创建的文章".to_string(),
            published: Some(true),
            user_id: user.id,
        };
        
        let post: Post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)?;
        
        Ok((user, post))
    });
    
    match result {
        Ok((user, post)) => {
            println!("事务成功: 用户 {} 创建了文章 {}", user.name, post.title);
        }
        Err(e) => {
            println!("事务失败: {}", e);
        }
    }
    
    Ok(())
}
```

### 嵌套事务

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::{users, posts};

fn nested_transaction_examples(conn: &mut PgConnection) -> QueryResult<()> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let new_user = NewUser {
            name: "外层事务用户".to_string(),
            email: "outer@example.com".to_string(),
        };
        
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)?;
        
        // 嵌套事务
        let nested_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let new_post = NewPost {
                title: "嵌套事务文章".to_string(),
                content: "这是一篇在嵌套事务中创建的文章".to_string(),
                published: Some(true),
                user_id: user.id,
            };
            
            let post: Post = diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result(conn)?;
            
            // 模拟错误
            if post.title.contains("错误") {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            
            Ok(post)
        });
        
        match nested_result {
            Ok(post) => {
                println!("嵌套事务成功: 创建文章 {}", post.title);
                Ok(user)
            }
            Err(e) => {
                println!("嵌套事务失败: {}", e);
                // 可以选择继续或回滚整个事务
                Ok(user)
            }
        }
    });
    
    match result {
        Ok(user) => println!("外层事务成功: 创建用户 {}", user.name),
        Err(e) => println!("外层事务失败: {}", e),
    }
    
    Ok(())
}
```

### 手动事务控制

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users;

fn manual_transaction_control(conn: &mut PgConnection) -> QueryResult<()> {
    // 开始事务
    conn.begin_transaction()?;
    
    let result = (|| {
        let new_user = NewUser {
            name: "手动事务用户".to_string(),
            email: "manual@example.com".to_string(),
        };
        
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)?;
        
        // 设置保存点
        conn.savepoint("user_created")?;
        
        // 尝试更新用户
        let updated_user = diesel::update(users::table.find(user.id))
            .set(users::name.eq("手动事务用户（更新）"))
            .get_result::<User>(conn)?;
        
        // 如果需要可以回滚到保存点
        if updated_user.name.len() > 20 {
            conn.rollback_to_savepoint("user_created")?;
        } else {
            conn.release_savepoint("user_created")?;
        }
        
        Ok(updated_user)
    })();
    
    match result {
        Ok(user) => {
            conn.commit_transaction()?;
            println!("手动事务成功: {}", user.name);
        }
        Err(e) => {
            conn.rollback_transaction()?;
            println!("手动事务失败: {}", e);
        }
    }
    
    Ok(())
}
```

## 异步支持

### 异步配置

```toml
[dependencies]
diesel = { version = "2.2.11", features = ["postgres", "chrono"] }
diesel-async = { version = "0.4", features = ["postgres", "bb8"] }
bb8 = "0.8"
tokio = { version = "1", features = ["full"] }
```

### 异步连接池

```rust
use diesel_async::{AsyncPgConnection, AsyncConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

type AsyncPool = Pool<AsyncPgConnection>;

pub async fn create_async_pool() -> Result<AsyncPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().build(config).await?;
    Ok(pool)
}
```

### 异步 CRUD 操作

```rust
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::models::*;
use crate::schema::users;

async fn async_crud_operations(conn: &mut AsyncPgConnection) -> QueryResult<()> {
    // 异步创建
    let new_user = NewUser {
        name: "异步用户".to_string(),
        email: "async@example.com".to_string(),
    };
    
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .await?;
    
    println!("异步创建用户: {:#?}", user);
    
    // 异步查询
    let all_users = users::table.load::<User>(conn).await?;
    println!("所有用户: {:#?}", all_users);
    
    // 异步更新
    let updated_user = diesel::update(users::table.find(user.id))
        .set(users::name.eq("异步用户（更新）"))
        .get_result::<User>(conn)
        .await?;
    
    println!("异步更新用户: {:#?}", updated_user);
    
    // 异步删除
    let deleted_user = diesel::delete(users::table.find(user.id))
        .get_result::<User>(conn)
        .await?;
    
    println!("异步删除用户: {:#?}", deleted_user);
    
    Ok(())
}
```

### 异步事务

```rust
use diesel_async::{AsyncPgConnection, scoped_futures::ScopedFutureExt};
use crate::models::*;
use crate::schema::{users, posts};

async fn async_transaction_examples(conn: &mut AsyncPgConnection) -> QueryResult<()> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            let new_user = NewUser {
                name: "异步事务用户".to_string(),
                email: "async_transaction@example.com".to_string(),
            };
            
            let user: User = diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)
                .await?;
            
            let new_post = NewPost {
                title: "异步事务文章".to_string(),
                content: "这是一篇在异步事务中创建的文章".to_string(),
                published: Some(true),
                user_id: user.id,
            };
            
            let post: Post = diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result(conn)
                .await?;
            
            Ok((user, post))
        }.scope_boxed()
    }).await;
    
    match result {
        Ok((user, post)) => {
            println!("异步事务成功: 用户 {} 创建了文章 {}", user.name, post.title);
        }
        Err(e) => {
            println!("异步事务失败: {}", e);
        }
    }
    
    Ok(())
}
```

## 连接池

### 同步连接池

```rust
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .connection_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .test_query("SELECT 1")
        .build(manager)
        .expect("Failed to create pool")
}

pub fn get_connection(pool: &Pool) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
    pool.get()
}
```

### 连接池使用示例

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users;

fn pool_usage_examples() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool();
    
    // 获取连接
    let mut conn = get_connection(&pool)?;
    
    // 使用连接
    let new_user = NewUser {
        name: "连接池用户".to_string(),
        email: "pool@example.com".to_string(),
    };
    
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)?;
    
    println!("使用连接池创建用户: {:#?}", user);
    
    // 连接会自动归还到池中
    
    Ok(())
}
```

## 自定义类型

### 自定义数据类型

```rust
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql};
use diesel::sql_types::Text;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

impl ToSql<Text, Pg> for UserStatus {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserStatus::Active => out.write_all(b"active")?,
            UserStatus::Inactive => out.write_all(b"inactive")?,
            UserStatus::Suspended => out.write_all(b"suspended")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserStatus {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"active" => Ok(UserStatus::Active),
            b"inactive" => Ok(UserStatus::Inactive),
            b"suspended" => Ok(UserStatus::Suspended),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
```

### JSON 字段支持

```rust
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
pub struct UserSettings {
    pub theme: String,
    pub notifications: bool,
    pub language: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::user_profiles)]
pub struct UserProfile {
    pub id: i32,
    pub user_id: i32,
    pub settings: UserSettings,
    pub metadata: serde_json::Value,
}

// 在 schema 中定义
diesel::table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Int4,
        settings -> Jsonb,
        metadata -> Jsonb,
    }
}
```

### 数组类型支持

```rust
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::sql_types::Array;
use diesel::sql_types::Text;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::articles)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
}

// 在 schema 中定义
diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        tags -> Array<Text>,
        categories -> Array<Text>,
    }
}

fn array_type_examples(conn: &mut PgConnection) -> QueryResult<()> {
    use crate::schema::articles;
    
    // 创建带数组字段的记录
    let new_article = (
        articles::title.eq("Rust 教程"),
        articles::content.eq("这是一篇关于 Rust 的教程"),
        articles::tags.eq(vec!["rust", "programming", "tutorial"]),
        articles::categories.eq(vec!["技术", "编程"]),
    );
    
    let article: Article = diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(conn)?;
    
    println!("创建文章: {:#?}", article);
    
    // 查询包含特定标签的文章
    let articles_with_rust = articles::table
        .filter(articles::tags.contains(vec!["rust"]))
        .load::<Article>(conn)?;
    
    println!("包含 rust 标签的文章: {:#?}", articles_with_rust);
    
    Ok(())
}
```

## SQL 函数

### 内置函数

```rust
use diesel::prelude::*;
use diesel::dsl::*;
use crate::models::*;
use crate::schema::users;

fn builtin_functions(conn: &mut PgConnection) -> QueryResult<()> {
    // 字符串函数
    let users_with_uppercase = users::table
        .select((users::id, upper(users::name)))
        .load::<(i32, String)>(conn)?;
    
    println!("大写用户名: {:#?}", users_with_uppercase);
    
    // 数学函数
    let user_count = users::table.count().get_result::<i64>(conn)?;
    println!("用户数量: {}", user_count);
    
    // 日期函数
    let recent_users = users::table
        .filter(users::created_at.gt(now - 30.days()))
        .load::<User>(conn)?;
    
    println!("最近 30 天的用户: {:#?}", recent_users);
    
    // 聚合函数
    let stats = users::table
        .select((
            count(users::id),
            min(users::created_at),
            max(users::created_at),
        ))
        .first::<(i64, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>)>(conn)?;
    
    println!("用户统计: {:#?}", stats);
    
    Ok(())
}
```

### 自定义函数

```rust
use diesel::prelude::*;
use diesel::sql_types::*;

// 定义自定义 SQL 函数
diesel::sql_function!(fn lower(x: Text) -> Text);
diesel::sql_function!(fn char_length(x: Text) -> Integer);
diesel::sql_function!(fn coalesce(x: Nullable<Text>, y: Text) -> Text);

// 更复杂的函数
diesel::sql_function! {
    #[sql_name = "string_agg"]
    fn string_agg(expr: Text, delimiter: Text) -> Nullable<Text>;
}

fn custom_functions(conn: &mut PgConnection) -> QueryResult<()> {
    use crate::schema::users;
    
    // 使用自定义函数
    let users_with_length = users::table
        .select((users::name, char_length(users::name)))
        .load::<(String, i32)>(conn)?;
    
    println!("用户名长度: {:#?}", users_with_length);
    
    // 使用 COALESCE
    let users_with_default = users::table
        .select((users::id, coalesce(users::email.nullable(), "no-email@example.com")))
        .load::<(i32, String)>(conn)?;
    
    println!("用户邮箱（带默认值）: {:#?}", users_with_default);
    
    Ok(())
}
```

## 性能优化

### 查询优化

```rust
use diesel::prelude::*;
use diesel::debug_query;
use crate::models::*;
use crate::schema::{users, posts};

fn query_optimization(conn: &mut PgConnection) -> QueryResult<()> {
    // 使用索引字段进行查询
    let indexed_query = users::table
        .filter(users::email.eq("user@example.com"))
        .first::<User>(conn)?;
    
    // 限制返回字段
    let limited_fields = users::table
        .select((users::id, users::name))
        .load::<(i32, String)>(conn)?;
    
    // 使用 LIMIT 限制结果数量
    let limited_results = users::table
        .limit(100)
        .load::<User>(conn)?;
    
    // 批量加载而不是 N+1 查询
    let users_with_posts = users::table
        .inner_join(posts::table)
        .select((users::all_columns, posts::all_columns))
        .load::<(User, Post)>(conn)?;
    
    // 使用 EXISTS 而不是 IN 子查询
    let users_with_posts_exists = users::table
        .filter(diesel::dsl::exists(
            posts::table.filter(posts::user_id.eq(users::id))
        ))
        .load::<User>(conn)?;
    
    // 打印查询 SQL（调试用）
    let query = users::table
        .filter(users::name.like("%test%"))
        .order(users::created_at.desc());
    
    println!("SQL 查询: {}", debug_query::<diesel::pg::Pg, _>(&query));
    
    Ok(())
}
```

### 批量操作

```rust
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users;

fn batch_operations(conn: &mut PgConnection) -> QueryResult<()> {
    // 批量插入
    let new_users: Vec<NewUser> = (1..=1000)
        .map(|i| NewUser {
            name: format!("用户{}", i),
            email: format!("user{}@example.com", i),
        })
        .collect();
    
    let inserted_users = diesel::insert_into(users::table)
        .values(&new_users)
        .get_results::<User>(conn)?;
    
    println!("批量插入了 {} 个用户", inserted_users.len());
    
    // 批量更新
    let updated_count = diesel::update(users::table)
        .filter(users::name.like("用户%"))
        .set(users::name.eq(users::name.concat(" (批量更新)")))
        .execute(conn)?;
    
    println!("批量更新了 {} 个用户", updated_count);
    
    // 批量删除
    let deleted_count = diesel::delete(users::table)
        .filter(users::name.like("%批量更新%"))
        .execute(conn)?;
    
    println!("批量删除了 {} 个用户", deleted_count);
    
    Ok(())
}
```

### 连接优化

```rust
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use std::time::Duration;

pub fn create_optimized_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(20)                                    // 最大连接数
        .min_idle(Some(5))                              // 最小空闲连接数
        .connection_timeout(Duration::from_secs(30))     // 连接超时
        .idle_timeout(Some(Duration::from_secs(600)))    // 空闲超时
        .max_lifetime(Some(Duration::from_secs(3600)))   // 连接最大生存时间
        .test_query("SELECT 1")                         // 测试查询
        .build(manager)
        .expect("Failed to create pool")
}
```

## 实战案例

### 博客系统

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 扩展模型
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::post_tags)]
pub struct PostTag {
    pub post_id: i32,
    pub tag_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::comments)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// 博客服务
pub struct BlogService {
    pool: crate::Pool,
}

impl BlogService {
    pub fn new(pool: crate::Pool) -> Self {
        Self { pool }
    }
    
    pub fn create_post_with_tags(
        &self,
        new_post: NewPost,
        tag_ids: Vec<i32>,
    ) -> Result<Post, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 创建文章
            let post: Post = diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result(conn)?;
            
            // 关联标签
            if !tag_ids.is_empty() {
                let post_tags: Vec<_> = tag_ids.into_iter()
                    .map(|tag_id| {
                        (
                            post_tags::post_id.eq(post.id),
                            post_tags::tag_id.eq(tag_id),
                        )
                    })
                    .collect();
                
                diesel::insert_into(post_tags::table)
                    .values(&post_tags)
                    .execute(conn)?;
            }
            
            Ok(post)
        }).map_err(|e| e.into())
    }
    
    pub fn get_post_with_details(&self, post_id: i32) -> Result<PostWithDetails, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        // 获取文章基本信息
        let post: Post = posts::table.find(post_id).first(&mut conn)?;
        
        // 获取作者信息
        let author: User = users::table.find(post.user_id).first(&mut conn)?;
        
        // 获取标签
        let tags: Vec<Tag> = tags::table
            .inner_join(post_tags::table.on(tags::id.eq(post_tags::tag_id)))
            .filter(post_tags::post_id.eq(post_id))
            .select(tags::all_columns)
            .load(&mut conn)?;
        
        // 获取评论
        let comments: Vec<Comment> = comments::table
            .filter(comments::post_id.eq(post_id))
            .order(comments::created_at.asc())
            .load(&mut conn)?;
        
        Ok(PostWithDetails {
            post,
            author,
            tags,
            comments,
        })
    }
    
    pub fn search_posts(&self, query: &str, page: i64, per_page: i64) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        let posts = posts::table
            .filter(
                posts::title.ilike(format!("%{}%", query))
                .or(posts::content.ilike(format!("%{}%", query)))
            )
            .filter(posts::published.eq(true))
            .order(posts::created_at.desc())
            .limit(per_page)
            .offset((page - 1) * per_page)
            .load(&mut conn)?;
        
        Ok(posts)
    }
    
    pub fn get_popular_posts(&self, limit: i64) -> Result<Vec<PostWithStats>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        let posts_with_stats = posts::table
            .left_join(comments::table)
            .group_by(posts::id)
            .select((
                posts::all_columns,
                diesel::dsl::count(comments::id.nullable()),
            ))
            .order(diesel::dsl::count(comments::id.nullable()).desc())
            .limit(limit)
            .load::<(Post, i64)>(&mut conn)?;
        
        let result = posts_with_stats.into_iter()
            .map(|(post, comment_count)| PostWithStats {
                post,
                comment_count,
            })
            .collect();
        
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithDetails {
    pub post: Post,
    pub author: User,
    pub tags: Vec<Tag>,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithStats {
    pub post: Post,
    pub comment_count: i64,
}
```

### 用户权限系统

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::permissions)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub resource: String,
    pub action: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::role_permissions)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub granted: bool,
}

pub struct PermissionService {
    pool: crate::Pool,
}

impl PermissionService {
    pub fn new(pool: crate::Pool) -> Self {
        Self { pool }
    }
    
    pub fn user_has_permission(
        &self,
        user_id: i32,
        resource: &str,
        action: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        let has_permission = users::table
            .inner_join(user_roles::table.on(users::id.eq(user_roles::user_id)))
            .inner_join(roles::table.on(user_roles::role_id.eq(roles::id)))
            .inner_join(role_permissions::table.on(roles::id.eq(role_permissions::role_id)))
            .inner_join(permissions::table.on(role_permissions::permission_id.eq(permissions::id)))
            .filter(users::id.eq(user_id))
            .filter(permissions::resource.eq(resource))
            .filter(permissions::action.eq(action))
            .filter(role_permissions::granted.eq(true))
            .count()
            .get_result::<i64>(&mut conn)?;
        
        Ok(has_permission > 0)
    }
    
    pub fn get_user_permissions(&self, user_id: i32) -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        let permissions = permissions::table
            .inner_join(role_permissions::table.on(permissions::id.eq(role_permissions::permission_id)))
            .inner_join(roles::table.on(role_permissions::role_id.eq(roles::id)))
            .inner_join(user_roles::table.on(roles::id.eq(user_roles::role_id)))
            .filter(user_roles::user_id.eq(user_id))
            .filter(role_permissions::granted.eq(true))
            .select(permissions::all_columns)
            .distinct()
            .load(&mut conn)?;
        
        Ok(permissions)
    }
    
    pub fn assign_role_to_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get()?;
        
        diesel::insert_into(user_roles::table)
            .values((
                user_roles::user_id.eq(user_id),
                user_roles::role_id.eq(role_id),
            ))
            .on_conflict((user_roles::user_id, user_roles::role_id))
            .do_nothing()
            .execute(&mut conn)?;
        
        Ok(())
    }
}
```

## 最佳实践

### 1. 项目结构

```
src/
├── main.rs
├── lib.rs
├── schema.rs          # 数据库 schema
├── models/            # 数据模型
│   ├── mod.rs
│   ├── user.rs
│   ├── post.rs
│   └── comment.rs
├── services/          # 业务逻辑
│   ├── mod.rs
│   ├── user_service.rs
│   └── post_service.rs
├── repositories/      # 数据访问层
│   ├── mod.rs
│   ├── user_repo.rs
│   └── post_repo.rs
└── utils/
    ├── mod.rs
    └── database.rs
```

### 2. 错误处理

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("连接错误: {0}")]
    Connection(#[from] diesel::ConnectionError),
    #[error("查询错误: {0}")]
    Query(#[from] diesel::result::Error),
    #[error("连接池错误: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("记录不存在: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
```

### 3. 连接管理

```rust
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use std::env;

pub fn create_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool")
}
```

### 4. 测试配置

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use diesel::connection::SimpleConnection;
    
    fn setup_test_db() -> PgConnection {
        let mut conn = establish_connection();
        
        // 开始事务
        conn.begin_test_transaction().unwrap();
        
        conn
    }
    
    #[test]
    fn test_create_user() {
        let mut conn = setup_test_db();
        
        let new_user = NewUser {
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
        };
        
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .unwrap();
        
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
        
        // 事务会自动回滚
    }
}
```

## 总结

Diesel 是一个功能强大的 ORM 和查询构建器，提供了类型安全的数据库操作。通过本教程，您应该能够：

1. 理解 Diesel 的核心概念和设计原理
2. 设置数据库连接和迁移系统
3. 定义模型和执行 CRUD 操作
4. 构建复杂查询和处理关系
5. 实现事务处理和异步支持
6. 优化性能和管理连接池

关键要点：
- 类型安全的查询构建
- 编译时 SQL 验证
- 强大的迁移系统
- 高性能的零成本抽象
- 良好的错误处理

Diesel 的设计哲学是在保证安全性的同时提供高性能，它是 Rust 生态系统中最成熟的 ORM 解决方案之一。