# SeaORM 2.0.0-rc.1 - Rust 异步动态 ORM 完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [数据库配置](#数据库配置)
- [实体定义](#实体定义)
- [迁移系统](#迁移系统)
- [查询构建器](#查询构建器)
- [CRUD 操作](#crud-操作)
- [关系映射](#关系映射)
- [事务处理](#事务处理)
- [连接池](#连接池)
- [JSON 支持](#json-支持)
- [分页和排序](#分页和排序)
- [原始SQL查询](#原始sql查询)
- [模拟和测试](#模拟和测试)
- [CLI 工具](#cli-工具)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

SeaORM 是一个现代的、异步的、动态的 ORM 框架，专为 Rust 设计。它提供了类型安全的数据库操作，支持 MySQL、PostgreSQL 和 SQLite，并且具有出色的异步性能。

### 核心特性
- **异步优先**: 完全异步的 API 设计
- **动态查询**: 运行时构建查询
- **类型安全**: 编译时类型检查
- **多数据库支持**: MySQL、PostgreSQL、SQLite
- **关系映射**: 强大的关系支持
- **迁移系统**: 自动化数据库迁移
- **Mock 支持**: 内置测试支持

### 版本信息
- **当前版本**: 2.0.0-rc.1
- **发布时间**: 2025-07-05
- **下载次数**: 7,601,885+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
sea-orm = { version = "2.0.0-rc.1", features = [
    "sqlx-postgres",
    "sqlx-mysql", 
    "sqlx-sqlite",
    "runtime-tokio-rustls",
    "macros",
    "with-chrono",
    "with-json",
    "with-uuid",
] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### 基本示例

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    
    // 创建用户
    let new_user = ActiveModel {
        name: Set("张三".to_string()),
        email: Set("zhangsan@example.com".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    
    let user = new_user.insert(&db).await?;
    println!("创建用户: {:?}", user);
    
    // 查询用户
    let users = Entity::find().all(&db).await?;
    println!("所有用户: {:?}", users);
    
    Ok(())
}
```

## 数据库配置

### 连接配置

```rust
use sea_orm::*;

// PostgreSQL 连接
async fn connect_postgres() -> Result<DatabaseConnection, DbErr> {
    let database_url = "postgresql://user:password@localhost:5432/database";
    Database::connect(database_url).await
}

// MySQL 连接
async fn connect_mysql() -> Result<DatabaseConnection, DbErr> {
    let database_url = "mysql://user:password@localhost:3306/database";
    Database::connect(database_url).await
}

// SQLite 连接
async fn connect_sqlite() -> Result<DatabaseConnection, DbErr> {
    let database_url = "sqlite:./database.db";
    Database::connect(database_url).await
}

// 内存数据库
async fn connect_memory() -> Result<DatabaseConnection, DbErr> {
    Database::connect("sqlite::memory:").await
}
```

### 高级连接配置

```rust
use sea_orm::*;
use std::time::Duration;

async fn connect_with_options() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new("postgresql://user:password@localhost:5432/database");
    
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    
    Database::connect(opt).await
}
```

## 实体定义

### 基本实体

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub name: String,
    
    #[sea_orm(column_type = "Text", nullable)]
    pub bio: Option<String>,
    
    #[sea_orm(default_value = "true")]
    pub is_active: bool,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Posts,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
    
    fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;
        this.updated_at = Set(chrono::Utc::now());
        Ok(this)
    }
}
```

### 复杂实体

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    pub title: String,
    
    #[sea_orm(column_type = "Text")]
    pub content: String,
    
    #[sea_orm(column_type = "Text", nullable)]
    pub excerpt: Option<String>,
    
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<serde_json::Value>,
    
    pub user_id: i32,
    pub category_id: Option<i32>,
    
    #[sea_orm(default_value = "false")]
    pub published: bool,
    
    #[sea_orm(default_value = "0")]
    pub view_count: i32,
    
    #[sea_orm(column_type = "Double", nullable)]
    pub rating: Option<f64>,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
    
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

### 自定义字段类型

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DeriveIntoActiveValue)]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DeriveIntoActiveValue)]
pub struct UserProfile {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub social_links: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "advanced_posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    pub title: String,
    
    #[sea_orm(column_type = "Text")]
    pub content: String,
    
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: PostStatus,
    
    #[sea_orm(column_type = "Json")]
    pub profile: UserProfile,
    
    #[sea_orm(column_type = "Array(Text)")]
    pub tags: Vec<String>,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

## 迁移系统

### 创建迁移

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Bio).text())
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        
        // 创建索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Bio,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
```

### 复杂迁移

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 posts 表
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Content).text().not_null())
                    .col(ColumnDef::new(Post::Excerpt).text())
                    .col(ColumnDef::new(Post::Metadata).json())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .col(ColumnDef::new(Post::CategoryId).integer())
                    .col(
                        ColumnDef::new(Post::Published)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Post::ViewCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Post::Rating).double())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_user_id")
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        
        // 创建复合索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_post_user_published")
                    .table(Post::Table)
                    .col(Post::UserId)
                    .col(Post::Published)
                    .to_owned(),
            )
            .await?;
        
        // 创建全文索引（PostgreSQL）
        if matches!(manager.get_database_backend(), sea_orm::DatabaseBackend::Postgres) {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_post_fulltext")
                        .table(Post::Table)
                        .col(Post::Title)
                        .col(Post::Content)
                        .index_type(IndexType::Gin)
                        .to_owned(),
                )
                .await?;
        }
        
        Ok(())
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Content,
    Excerpt,
    Metadata,
    UserId,
    CategoryId,
    Published,
    ViewCount,
    Rating,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
```

### 运行迁移

```rust
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    
    // 创建迁移器
    let migrator = Migrator::new();
    
    // 运行所有迁移
    migrator.up(&db, None).await?;
    
    // 回滚最后一个迁移
    migrator.down(&db, Some(1)).await?;
    
    // 获取迁移状态
    let status = migrator.status(&db).await?;
    println!("迁移状态: {:?}", status);
    
    Ok(())
}
```

## 查询构建器

### 基本查询

```rust
use sea_orm::*;

async fn basic_queries(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 查询所有记录
    let users = user::Entity::find().all(db).await?;
    println!("所有用户: {:?}", users);
    
    // 查询单个记录
    let user = user::Entity::find_by_id(1).one(db).await?;
    println!("用户: {:?}", user);
    
    // 条件查询
    let active_users = user::Entity::find()
        .filter(user::Column::IsActive.eq(true))
        .all(db)
        .await?;
    println!("活跃用户: {:?}", active_users);
    
    // 复杂条件
    let filtered_users = user::Entity::find()
        .filter(
            user::Column::Name.contains("张")
                .and(user::Column::IsActive.eq(true))
                .or(user::Column::Email.ends_with("@admin.com"))
        )
        .all(db)
        .await?;
    println!("过滤用户: {:?}", filtered_users);
    
    // 排序
    let sorted_users = user::Entity::find()
        .order_by_asc(user::Column::Name)
        .order_by_desc(user::Column::CreatedAt)
        .all(db)
        .await?;
    println!("排序用户: {:?}", sorted_users);
    
    // 限制和偏移
    let paginated_users = user::Entity::find()
        .limit(10)
        .offset(0)
        .all(db)
        .await?;
    println!("分页用户: {:?}", paginated_users);
    
    Ok(())
}
```

### 聚合查询

```rust
use sea_orm::*;

async fn aggregation_queries(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 计数
    let user_count = user::Entity::find().count(db).await?;
    println!("用户总数: {}", user_count);
    
    // 条件计数
    let active_count = user::Entity::find()
        .filter(user::Column::IsActive.eq(true))
        .count(db)
        .await?;
    println!("活跃用户数: {}", active_count);
    
    // 聚合函数
    let post_stats = post::Entity::find()
        .select_only()
        .column_as(post::Column::Id.count(), "post_count")
        .column_as(post::Column::ViewCount.sum(), "total_views")
        .column_as(post::Column::ViewCount.avg(), "avg_views")
        .column_as(post::Column::ViewCount.min(), "min_views")
        .column_as(post::Column::ViewCount.max(), "max_views")
        .into_tuple::<(i64, Option<i64>, Option<f64>, Option<i32>, Option<i32>)>()
        .one(db)
        .await?;
    
    if let Some((count, total, avg, min, max)) = post_stats {
        println!("文章统计: 总数={}, 总浏览={:?}, 平均浏览={:?}, 最小浏览={:?}, 最大浏览={:?}", 
                 count, total, avg, min, max);
    }
    
    // 分组查询
    let user_post_counts = post::Entity::find()
        .select_only()
        .column(post::Column::UserId)
        .column_as(post::Column::Id.count(), "post_count")
        .group_by(post::Column::UserId)
        .into_tuple::<(i32, i64)>()
        .all(db)
        .await?;
    
    for (user_id, count) in user_post_counts {
        println!("用户 {} 的文章数: {}", user_id, count);
    }
    
    Ok(())
}
```

### 连接查询

```rust
use sea_orm::*;

async fn join_queries(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 内连接
    let users_with_posts = user::Entity::find()
        .find_also_related(post::Entity)
        .all(db)
        .await?;
    
    for (user, post) in users_with_posts {
        if let Some(post) = post {
            println!("用户 {} 的文章: {}", user.name, post.title);
        }
    }
    
    // 左连接
    let users_with_optional_posts = user::Entity::find()
        .find_with_related(post::Entity)
        .all(db)
        .await?;
    
    for (user, posts) in users_with_optional_posts {
        println!("用户 {} 有 {} 篇文章", user.name, posts.len());
    }
    
    // 自定义连接
    let custom_join = user::Entity::find()
        .join(JoinType::InnerJoin, user::Relation::Posts.def())
        .filter(post::Column::Published.eq(true))
        .select_only()
        .column(user::Column::Name)
        .column(post::Column::Title)
        .into_tuple::<(String, String)>()
        .all(db)
        .await?;
    
    for (user_name, post_title) in custom_join {
        println!("用户 {} 的已发布文章: {}", user_name, post_title);
    }
    
    Ok(())
}
```

## CRUD 操作

### 创建 (Create)

```rust
use sea_orm::*;

async fn create_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 创建单个记录
    let new_user = user::ActiveModel {
        name: Set("张三".to_string()),
        email: Set("zhangsan@example.com".to_string()),
        bio: Set(Some("这是张三的个人简介".to_string())),
        is_active: Set(true),
        ..Default::default()
    };
    
    let user = new_user.insert(db).await?;
    println!("创建用户: {:?}", user);
    
    // 批量创建
    let users = vec![
        user::ActiveModel {
            name: Set("李四".to_string()),
            email: Set("lisi@example.com".to_string()),
            ..Default::default()
        },
        user::ActiveModel {
            name: Set("王五".to_string()),
            email: Set("wangwu@example.com".to_string()),
            ..Default::default()
        },
    ];
    
    let insert_result = user::Entity::insert_many(users).exec(db).await?;
    println!("批量创建用户: {:?}", insert_result);
    
    // 插入或更新
    let upsert_user = user::ActiveModel {
        id: Set(1),
        name: Set("张三（更新）".to_string()),
        email: Set("zhangsan@example.com".to_string()),
        ..Default::default()
    };
    
    let upsert_result = user::Entity::insert(upsert_user)
        .on_conflict(
            OnConflict::column(user::Column::Email)
                .update_column(user::Column::Name)
                .to_owned(),
        )
        .exec(db)
        .await?;
    
    println!("插入或更新结果: {:?}", upsert_result);
    
    Ok(())
}
```

### 查询 (Read)

```rust
use sea_orm::*;

async fn read_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 查询所有记录
    let all_users = user::Entity::find().all(db).await?;
    println!("所有用户: {:?}", all_users);
    
    // 按ID查询
    let user_by_id = user::Entity::find_by_id(1).one(db).await?;
    println!("用户 1: {:?}", user_by_id);
    
    // 条件查询
    let users_by_name = user::Entity::find()
        .filter(user::Column::Name.contains("张"))
        .all(db)
        .await?;
    println!("名字包含'张'的用户: {:?}", users_by_name);
    
    // 复杂条件
    let filtered_users = user::Entity::find()
        .filter(
            user::Column::IsActive.eq(true)
                .and(user::Column::CreatedAt.gt(chrono::Utc::now() - chrono::Duration::days(30)))
        )
        .order_by_desc(user::Column::CreatedAt)
        .limit(10)
        .all(db)
        .await?;
    println!("最近30天的活跃用户: {:?}", filtered_users);
    
    // 选择特定字段
    let user_names = user::Entity::find()
        .select_only()
        .column(user::Column::Name)
        .into_tuple::<String>()
        .all(db)
        .await?;
    println!("用户名列表: {:?}", user_names);
    
    // 分页查询
    let paginated = user::Entity::find()
        .paginate(db, 10);
    
    let page_1 = paginated.fetch_page(0).await?;
    println!("第一页用户: {:?}", page_1);
    
    let total_pages = paginated.num_pages().await?;
    println!("总页数: {}", total_pages);
    
    Ok(())
}
```

### 更新 (Update)

```rust
use sea_orm::*;

async fn update_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 更新单个记录
    let user = user::Entity::find_by_id(1).one(db).await?;
    
    if let Some(user) = user {
        let mut active_user: user::ActiveModel = user.into();
        active_user.name = Set("张三（更新）".to_string());
        active_user.bio = Set(Some("更新后的简介".to_string()));
        
        let updated_user = active_user.update(db).await?;
        println!("更新用户: {:?}", updated_user);
    }
    
    // 批量更新
    let update_result = user::Entity::update_many()
        .col_expr(user::Column::IsActive, Expr::value(false))
        .filter(user::Column::CreatedAt.lt(chrono::Utc::now() - chrono::Duration::days(365)))
        .exec(db)
        .await?;
    
    println!("批量更新了 {} 个用户", update_result.rows_affected);
    
    // 条件更新
    let conditional_update = user::Entity::update_many()
        .col_expr(
            user::Column::Name,
            Expr::col(user::Column::Name).concat(" (VIP)")
        )
        .filter(user::Column::Email.ends_with("@vip.com"))
        .exec(db)
        .await?;
    
    println!("条件更新了 {} 个VIP用户", conditional_update.rows_affected);
    
    Ok(())
}
```

### 删除 (Delete)

```rust
use sea_orm::*;

async fn delete_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 删除单个记录
    let user = user::Entity::find_by_id(1).one(db).await?;
    
    if let Some(user) = user {
        let delete_result = user.delete(db).await?;
        println!("删除用户: {:?}", delete_result);
    }
    
    // 按ID删除
    let delete_result = user::Entity::delete_by_id(2).exec(db).await?;
    println!("删除用户 2: {:?}", delete_result);
    
    // 批量删除
    let bulk_delete = user::Entity::delete_many()
        .filter(user::Column::IsActive.eq(false))
        .exec(db)
        .await?;
    
    println!("批量删除了 {} 个非活跃用户", bulk_delete.rows_affected);
    
    // 条件删除
    let conditional_delete = user::Entity::delete_many()
        .filter(user::Column::CreatedAt.lt(chrono::Utc::now() - chrono::Duration::days(365)))
        .exec(db)
        .await?;
    
    println!("条件删除了 {} 个旧用户", conditional_delete.rows_affected);
    
    Ok(())
}
```

## 关系映射

### 一对多关系

```rust
use sea_orm::*;

// 在用户实体中定义关系
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Posts,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

// 在文章实体中定义关系
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

async fn one_to_many_examples(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 查询用户及其文章
    let users_with_posts = user::Entity::find()
        .find_with_related(post::Entity)
        .all(db)
        .await?;
    
    for (user, posts) in users_with_posts {
        println!("用户 {} 有 {} 篇文章:", user.name, posts.len());
        for post in posts {
            println!("  - {}", post.title);
        }
    }
    
    // 查询文章及其作者
    let posts_with_authors = post::Entity::find()
        .find_also_related(user::Entity)
        .all(db)
        .await?;
    
    for (post, author) in posts_with_authors {
        if let Some(author) = author {
            println!("文章 '{}' 作者: {}", post.title, author.name);
        }
    }
    
    Ok(())
}
```

### 多对多关系

```rust
use sea_orm::*;

// 中间表实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub post_id: i32,
    #[sea_orm(primary_key)]
    pub tag_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// 在文章实体中添加多对多关系
impl Related<super::tag::Entity> for super::post::Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Tag.def()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Post.def().rev())
    }
}

// 在标签实体中添加多对多关系
impl Related<super::post::Entity> for super::tag::Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Post.def()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Tag.def().rev())
    }
}

async fn many_to_many_examples(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 查询文章及其标签
    let posts_with_tags = post::Entity::find()
        .find_with_related(tag::Entity)
        .all(db)
        .await?;
    
    for (post, tags) in posts_with_tags {
        println!("文章 '{}' 的标签:", post.title);
        for tag in tags {
            println!("  - {}", tag.name);
        }
    }
    
    // 查询标签及其文章
    let tags_with_posts = tag::Entity::find()
        .find_with_related(post::Entity)
        .all(db)
        .await?;
    
    for (tag, posts) in tags_with_posts {
        println!("标签 '{}' 的文章:", tag.name);
        for post in posts {
            println!("  - {}", post.title);
        }
    }
    
    // 创建多对多关系
    let post_tag = post_tag::ActiveModel {
        post_id: Set(1),
        tag_id: Set(1),
        created_at: Set(chrono::Utc::now()),
    };
    
    let result = post_tag.insert(db).await?;
    println!("创建文章-标签关系: {:?}", result);
    
    Ok(())
}
```

### 自引用关系

```rust
use sea_orm::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id"
    )]
    Parent,
    #[sea_orm(has_many = "Entity")]
    Children,
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

async fn self_referencing_examples(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 查询评论及其子评论
    let comments_with_children = comment::Entity::find()
        .find_with_related(comment::Entity)
        .all(db)
        .await?;
    
    for (comment, children) in comments_with_children {
        println!("评论: {}", comment.content);
        for child in children {
            println!("  回复: {}", child.content);
        }
    }
    
    // 查询顶级评论（没有父评论）
    let top_level_comments = comment::Entity::find()
        .filter(comment::Column::ParentId.is_null())
        .all(db)
        .await?;
    
    println!("顶级评论: {:?}", top_level_comments);
    
    Ok(())
}
```

## 事务处理

### 基本事务

```rust
use sea_orm::*;

async fn basic_transaction(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    
    // 在事务中执行操作
    let user = user::ActiveModel {
        name: Set("事务用户".to_string()),
        email: Set("transaction@example.com".to_string()),
        ..Default::default()
    };
    
    let user = user.insert(&txn).await?;
    
    let post = post::ActiveModel {
        title: Set("事务文章".to_string()),
        content: Set("这是一篇在事务中创建的文章".to_string()),
        user_id: Set(user.id),
        ..Default::default()
    };
    
    let post = post.insert(&txn).await?;
    
    // 提交事务
    txn.commit().await?;
    
    println!("事务成功: 用户 {} 创建了文章 {}", user.name, post.title);
    
    Ok(())
}
```

### 事务回滚

```rust
use sea_orm::*;

async fn transaction_rollback(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    
    let result = async {
        let user = user::ActiveModel {
            name: Set("回滚用户".to_string()),
            email: Set("rollback@example.com".to_string()),
            ..Default::default()
        };
        
        let user = user.insert(&txn).await?;
        
        // 模拟错误条件
        if user.name.contains("回滚") {
            return Err(DbErr::Custom("模拟错误，需要回滚".to_string()));
        }
        
        let post = post::ActiveModel {
            title: Set("不会创建的文章".to_string()),
            content: Set("这篇文章不会被创建".to_string()),
            user_id: Set(user.id),
            ..Default::default()
        };
        
        let post = post.insert(&txn).await?;
        
        Ok((user, post))
    }.await;
    
    match result {
        Ok((user, post)) => {
            txn.commit().await?;
            println!("事务成功: 用户 {} 创建了文章 {}", user.name, post.title);
        }
        Err(e) => {
            txn.rollback().await?;
            println!("事务回滚: {}", e);
        }
    }
    
    Ok(())
}
```

### 事务闭包

```rust
use sea_orm::*;

async fn transaction_closure(db: &DatabaseConnection) -> Result<(), DbErr> {
    let result = db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            let user = user::ActiveModel {
                name: Set("闭包用户".to_string()),
                email: Set("closure@example.com".to_string()),
                ..Default::default()
            };
            
            let user = user.insert(txn).await?;
            
            let post = post::ActiveModel {
                title: Set("闭包文章".to_string()),
                content: Set("这是一篇在事务闭包中创建的文章".to_string()),
                user_id: Set(user.id),
                ..Default::default()
            };
            
            let post = post.insert(txn).await?;
            
            // 可以在这里添加更多操作
            
            Ok((user, post))
        })
    }).await?;
    
    println!("事务闭包成功: 用户 {} 创建了文章 {}", result.0.name, result.1.title);
    
    Ok(())
}
```

## 连接池

### 连接池配置

```rust
use sea_orm::*;
use std::time::Duration;

async fn create_connection_pool() -> Result<DatabaseConnection, DbErr> {
    let database_url = "postgresql://user:password@localhost:5432/database";
    
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");
    
    Database::connect(opt).await
}
```

### 连接池管理

```rust
use sea_orm::*;
use std::sync::Arc;

pub struct DatabaseManager {
    db: Arc<DatabaseConnection>,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let db = Database::connect(database_url).await?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    pub fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.db.clone()
    }
    
    pub async fn check_connection(&self) -> Result<(), DbErr> {
        self.db.ping().await?;
        Ok(())
    }
    
    pub async fn close(&self) -> Result<(), DbErr> {
        self.db.close().await?;
        Ok(())
    }
}

async fn connection_pool_example() -> Result<(), DbErr> {
    let manager = DatabaseManager::new("sqlite::memory:").await?;
    
    // 检查连接
    manager.check_connection().await?;
    
    // 使用连接
    let db = manager.get_connection();
    let users = user::Entity::find().all(&*db).await?;
    println!("查询到 {} 个用户", users.len());
    
    // 关闭连接
    manager.close().await?;
    
    Ok(())
}
```

## JSON 支持

### JSON 字段操作

```rust
use sea_orm::*;
use serde_json::json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "articles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content: String,
    #[sea_orm(column_type = "Json")]
    pub metadata: serde_json::Value,
    #[sea_orm(column_type = "Json", nullable)]
    pub settings: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn json_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 创建带 JSON 字段的记录
    let article = article::ActiveModel {
        title: Set("JSON 示例文章".to_string()),
        content: Set("这是一篇包含 JSON 字段的文章".to_string()),
        metadata: Set(json!({
            "tags": ["rust", "database", "json"],
            "category": "技术",
            "author": {
                "name": "张三",
                "email": "zhangsan@example.com"
            },
            "stats": {
                "views": 100,
                "likes": 50
            }
        })),
        settings: Set(Some(json!({
            "comments_enabled": true,
            "featured": false,
            "priority": 1
        }))),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    
    let article = article.insert(db).await?;
    println!("创建文章: {:?}", article);
    
    // 查询 JSON 字段
    let articles = article::Entity::find()
        .filter(article::Column::Metadata.contains("rust"))
        .all(db)
        .await?;
    
    for article in articles {
        println!("文章: {}", article.title);
        println!("元数据: {}", article.metadata);
        
        // 解析 JSON 数据
        if let Some(tags) = article.metadata.get("tags") {
            println!("标签: {:?}", tags);
        }
        
        if let Some(author) = article.metadata.get("author") {
            println!("作者: {}", author.get("name").unwrap_or(&json!("unknown")));
        }
    }
    
    Ok(())
}
```

### JSON 查询操作

```rust
use sea_orm::*;
use serde_json::json;

async fn json_query_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // JSON 路径查询
    let articles_by_author = article::Entity::find()
        .filter(
            Expr::cust_with_values(
                "metadata ->> 'author' ->> 'name' = ?",
                vec!["张三"]
            )
        )
        .all(db)
        .await?;
    
    println!("张三的文章: {:?}", articles_by_author);
    
    // JSON 数组查询
    let articles_with_rust_tag = article::Entity::find()
        .filter(
            Expr::cust_with_values(
                "metadata -> 'tags' ? ?",
                vec!["rust"]
            )
        )
        .all(db)
        .await?;
    
    println!("包含 rust 标签的文章: {:?}", articles_with_rust_tag);
    
    // 更新 JSON 字段
    let update_result = article::Entity::update_many()
        .col_expr(
            article::Column::Metadata,
            Expr::cust_with_values(
                "jsonb_set(metadata, '{stats,views}', ?)",
                vec![json!(200)]
            )
        )
        .filter(article::Column::Id.eq(1))
        .exec(db)
        .await?;
    
    println!("更新 JSON 字段: {:?}", update_result);
    
    Ok(())
}
```

## 分页和排序

### 基本分页

```rust
use sea_orm::*;

async fn basic_pagination(db: &DatabaseConnection) -> Result<(), DbErr> {
    let page_size = 10;
    let page_num = 1;
    
    // 使用 limit 和 offset
    let users = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .limit(page_size)
        .offset((page_num - 1) * page_size)
        .all(db)
        .await?;
    
    println!("第 {} 页用户: {:?}", page_num, users);
    
    // 获取总数
    let total = user::Entity::find().count(db).await?;
    let total_pages = (total + page_size - 1) / page_size;
    
    println!("总用户数: {}, 总页数: {}", total, total_pages);
    
    Ok(())
}
```

### 分页器

```rust
use sea_orm::*;

async fn paginator_example(db: &DatabaseConnection) -> Result<(), DbErr> {
    let posts_per_page = 5;
    
    // 创建分页器
    let paginator = post::Entity::find()
        .filter(post::Column::Published.eq(true))
        .order_by_desc(post::Column::CreatedAt)
        .paginate(db, posts_per_page);
    
    // 获取总页数
    let total_pages = paginator.num_pages().await?;
    println!("总页数: {}", total_pages);
    
    // 获取第一页
    let first_page = paginator.fetch_page(0).await?;
    println!("第一页文章: {:?}", first_page);
    
    // 获取第二页
    let second_page = paginator.fetch_page(1).await?;
    println!("第二页文章: {:?}", second_page);
    
    // 遍历所有页
    for page_num in 0..total_pages {
        let page = paginator.fetch_page(page_num).await?;
        println!("第 {} 页有 {} 篇文章", page_num + 1, page.len());
    }
    
    Ok(())
}
```

### 游标分页

```rust
use sea_orm::*;

async fn cursor_pagination(db: &DatabaseConnection) -> Result<(), DbErr> {
    let page_size = 10;
    let mut last_id = 0;
    
    loop {
        let posts = post::Entity::find()
            .filter(post::Column::Id.gt(last_id))
            .order_by_asc(post::Column::Id)
            .limit(page_size)
            .all(db)
            .await?;
        
        if posts.is_empty() {
            break;
        }
        
        println!("获取 {} 篇文章", posts.len());
        for post in &posts {
            println!("  - ID: {}, 标题: {}", post.id, post.title);
        }
        
        last_id = posts.last().unwrap().id;
    }
    
    Ok(())
}
```

### 复杂排序

```rust
use sea_orm::*;

async fn complex_sorting(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 多字段排序
    let posts = post::Entity::find()
        .order_by_desc(post::Column::Published)
        .order_by_desc(post::Column::CreatedAt)
        .order_by_asc(post::Column::Title)
        .all(db)
        .await?;
    
    println!("多字段排序文章: {:?}", posts);
    
    // 条件排序
    let posts_with_conditional_sort = post::Entity::find()
        .order_by_desc(
            Expr::case(
                Expr::col(post::Column::Published).eq(true),
                1
            )
            .finally(0)
        )
        .order_by_desc(post::Column::ViewCount)
        .all(db)
        .await?;
    
    println!("条件排序文章: {:?}", posts_with_conditional_sort);
    
    // 随机排序
    let random_posts = post::Entity::find()
        .order_by_desc(Expr::cust("RANDOM()"))
        .limit(5)
        .all(db)
        .await?;
    
    println!("随机文章: {:?}", random_posts);
    
    Ok(())
}
```

## 原始SQL查询

### 原始查询

```rust
use sea_orm::*;

async fn raw_queries(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 原始 SQL 查询
    let users: Vec<user::Model> = user::Entity::find()
        .from_raw_sql(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT * FROM users WHERE name LIKE '%张%'".to_string(),
        ))
        .all(db)
        .await?;
    
    println!("原始查询用户: {:?}", users);
    
    // 带参数的原始查询
    let active_users: Vec<user::Model> = user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT * FROM users WHERE is_active = $1 AND created_at > $2",
            vec![true.into(), chrono::Utc::now().date_naive().into()],
        ))
        .all(db)
        .await?;
    
    println!("活跃用户: {:?}", active_users);
    
    // 原始查询返回自定义结构
    #[derive(Debug, FromQueryResult)]
    struct UserStats {
        user_id: i32,
        user_name: String,
        post_count: i64,
        avg_views: Option<f64>,
    }
    
    let stats: Vec<UserStats> = UserStats::find_by_statement(Statement::from_string(
        DatabaseBackend::Postgres,
        r#"
        SELECT 
            u.id as user_id,
            u.name as user_name,
            COUNT(p.id) as post_count,
            AVG(p.view_count) as avg_views
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        GROUP BY u.id, u.name
        ORDER BY post_count DESC
        "#.to_string(),
    ))
    .all(db)
    .await?;
    
    println!("用户统计: {:?}", stats);
    
    Ok(())
}
```

### 执行原始SQL

```rust
use sea_orm::*;

async fn execute_raw_sql(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 执行原始 SQL
    let result = db.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        "UPDATE users SET updated_at = NOW() WHERE is_active = true".to_string(),
    )).await?;
    
    println!("更新了 {} 行", result.rows_affected());
    
    // 带参数的原始 SQL
    let result = db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "DELETE FROM posts WHERE created_at < $1",
        vec![chrono::Utc::now().date_naive().into()],
    )).await?;
    
    println!("删除了 {} 行", result.rows_affected());
    
    // 查询单个值
    let count: Option<i64> = db.query_one(Statement::from_string(
        DatabaseBackend::Postgres,
        "SELECT COUNT(*) FROM users".to_string(),
    ))
    .await?
    .map(|row| row.try_get("", "count"))
    .transpose()?;
    
    println!("用户总数: {:?}", count);
    
    Ok(())
}
```

## 模拟和测试

### 模拟数据库

```rust
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![user::Model {
                id: 1,
                name: "张三".to_string(),
                email: "zhangsan@example.com".to_string(),
                bio: None,
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }],
            vec![user::Model {
                id: 2,
                name: "李四".to_string(),
                email: "lisi@example.com".to_string(),
                bio: None,
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }],
        ])
        .append_exec_results(vec![
            MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 2,
                rows_affected: 1,
            },
        ])
        .into_connection();
    
    // 使用模拟数据库
    let users = user::Entity::find().all(&db).await?;
    println!("模拟查询用户: {:?}", users);
    
    let new_user = user::ActiveModel {
        name: Set("王五".to_string()),
        email: Set("wangwu@example.com".to_string()),
        ..Default::default()
    };
    
    let user = new_user.insert(&db).await?;
    println!("模拟创建用户: {:?}", user);
    
    Ok(())
}
```

### 测试示例

```rust
use sea_orm::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_user() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user::Model {
                id: 1,
                name: "测试用户".to_string(),
                email: "test@example.com".to_string(),
                bio: None,
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        
        let new_user = user::ActiveModel {
            name: Set("测试用户".to_string()),
            email: Set("test@example.com".to_string()),
            ..Default::default()
        };
        
        let user = new_user.insert(&db).await.unwrap();
        
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_find_users() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![
                user::Model {
                    id: 1,
                    name: "用户1".to_string(),
                    email: "user1@example.com".to_string(),
                    bio: None,
                    is_active: true,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                user::Model {
                    id: 2,
                    name: "用户2".to_string(),
                    email: "user2@example.com".to_string(),
                    bio: None,
                    is_active: true,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
            ]])
            .into_connection();
        
        let users = user::Entity::find().all(&db).await.unwrap();
        
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "用户1");
        assert_eq!(users[1].name, "用户2");
    }
}
```

## CLI 工具

### 实体生成

```bash
# 从数据库生成实体
sea-orm-cli generate entity \
    --database-url "postgresql://user:password@localhost:5432/database" \
    --output-dir src/entities

# 生成实体到特定文件
sea-orm-cli generate entity \
    --database-url "postgresql://user:password@localhost:5432/database" \
    --output-dir src/entities \
    --with-serde both \
    --serde-skip-deserializing-primary-key

# 生成模拟数据
sea-orm-cli generate entity \
    --database-url "postgresql://user:password@localhost:5432/database" \
    --output-dir src/entities \
    --with-copy-enums
```

### 迁移生成

```bash
# 创建新迁移
sea-orm-cli migrate init

# 生成迁移文件
sea-orm-cli migrate generate create_users_table

# 应用迁移
sea-orm-cli migrate up

# 回滚迁移
sea-orm-cli migrate down

# 重置数据库
sea-orm-cli migrate reset

# 检查迁移状态
sea-orm-cli migrate status
```

## 实战案例

### 博客系统

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

// 博客服务
pub struct BlogService {
    db: DatabaseConnection,
}

impl BlogService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    
    pub async fn create_post_with_tags(
        &self,
        title: String,
        content: String,
        user_id: i32,
        tag_names: Vec<String>,
    ) -> Result<post::Model, DbErr> {
        let txn = self.db.begin().await?;
        
        // 创建文章
        let post = post::ActiveModel {
            title: Set(title),
            content: Set(content),
            user_id: Set(user_id),
            published: Set(false),
            ..Default::default()
        };
        
        let post = post.insert(&txn).await?;
        
        // 处理标签
        for tag_name in tag_names {
            // 查找或创建标签
            let tag = match tag::Entity::find()
                .filter(tag::Column::Name.eq(&tag_name))
                .one(&txn)
                .await?
            {
                Some(tag) => tag,
                None => {
                    let new_tag = tag::ActiveModel {
                        name: Set(tag_name),
                        ..Default::default()
                    };
                    new_tag.insert(&txn).await?
                }
            };
            
            // 创建文章-标签关联
            let post_tag = post_tag::ActiveModel {
                post_id: Set(post.id),
                tag_id: Set(tag.id),
                ..Default::default()
            };
            
            post_tag.insert(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(post)
    }
    
    pub async fn get_published_posts_with_pagination(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<PostWithAuthor>, u64), DbErr> {
        let paginator = post::Entity::find()
            .filter(post::Column::Published.eq(true))
            .order_by_desc(post::Column::CreatedAt)
            .paginate(&self.db, page_size);
        
        let total_pages = paginator.num_pages().await?;
        let posts = paginator.fetch_page(page).await?;
        
        let mut posts_with_authors = Vec::new();
        
        for post in posts {
            let author = user::Entity::find_by_id(post.user_id)
                .one(&self.db)
                .await?
                .ok_or_else(|| DbErr::Custom("用户不存在".to_string()))?;
            
            posts_with_authors.push(PostWithAuthor { post, author });
        }
        
        Ok((posts_with_authors, total_pages))
    }
    
    pub async fn search_posts(
        &self,
        query: &str,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<post::Model>, DbErr> {
        let posts = post::Entity::find()
            .filter(
                post::Column::Title.contains(query)
                    .or(post::Column::Content.contains(query))
            )
            .filter(post::Column::Published.eq(true))
            .order_by_desc(post::Column::CreatedAt)
            .limit(page_size)
            .offset(page * page_size)
            .all(&self.db)
            .await?;
        
        Ok(posts)
    }
    
    pub async fn get_user_stats(&self, user_id: i32) -> Result<UserStats, DbErr> {
        let user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| DbErr::Custom("用户不存在".to_string()))?;
        
        let post_count = post::Entity::find()
            .filter(post::Column::UserId.eq(user_id))
            .count(&self.db)
            .await?;
        
        let published_count = post::Entity::find()
            .filter(post::Column::UserId.eq(user_id))
            .filter(post::Column::Published.eq(true))
            .count(&self.db)
            .await?;
        
        let total_views = post::Entity::find()
            .filter(post::Column::UserId.eq(user_id))
            .select_only()
            .column_as(post::Column::ViewCount.sum(), "total_views")
            .into_tuple::<Option<i64>>()
            .one(&self.db)
            .await?
            .flatten()
            .unwrap_or(0);
        
        Ok(UserStats {
            user,
            post_count,
            published_count,
            total_views,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct PostWithAuthor {
    pub post: post::Model,
    pub author: user::Model,
}

#[derive(Debug, Serialize)]
pub struct UserStats {
    pub user: user::Model,
    pub post_count: u64,
    pub published_count: u64,
    pub total_views: i64,
}
```

### 电商系统

```rust
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub stock_quantity: i32,
    pub category_id: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct EcommerceService {
    db: DatabaseConnection,
}

impl EcommerceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    
    pub async fn create_order(
        &self,
        user_id: i32,
        items: Vec<OrderItem>,
    ) -> Result<order::Model, DbErr> {
        let txn = self.db.begin().await?;
        
        let mut total_amount = rust_decimal::Decimal::from(0);
        
        // 验证商品库存并计算总价
        for item in &items {
            let product = product::Entity::find_by_id(item.product_id)
                .one(&txn)
                .await?
                .ok_or_else(|| DbErr::Custom("商品不存在".to_string()))?;
            
            if product.stock_quantity < item.quantity {
                return Err(DbErr::Custom("库存不足".to_string()));
            }
            
            total_amount += product.price * rust_decimal::Decimal::from(item.quantity);
        }
        
        // 创建订单
        let order = order::ActiveModel {
            user_id: Set(user_id),
            total_amount: Set(total_amount),
            status: Set("pending".to_string()),
            ..Default::default()
        };
        
        let order = order.insert(&txn).await?;
        
        // 创建订单项并更新库存
        for item in items {
            let order_item = order_item::ActiveModel {
                order_id: Set(order.id),
                product_id: Set(item.product_id),
                quantity: Set(item.quantity),
                price: Set(item.price),
                ..Default::default()
            };
            
            order_item.insert(&txn).await?;
            
            // 更新库存
            product::Entity::update_many()
                .col_expr(
                    product::Column::StockQuantity,
                    Expr::col(product::Column::StockQuantity).sub(item.quantity)
                )
                .filter(product::Column::Id.eq(item.product_id))
                .exec(&txn)
                .await?;
        }
        
        txn.commit().await?;
        Ok(order)
    }
    
    pub async fn get_products_by_category(
        &self,
        category_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<product::Model>, DbErr> {
        let products = product::Entity::find()
            .filter(product::Column::CategoryId.eq(category_id))
            .filter(product::Column::IsActive.eq(true))
            .order_by_asc(product::Column::Name)
            .limit(page_size)
            .offset(page * page_size)
            .all(&self.db)
            .await?;
        
        Ok(products)
    }
    
    pub async fn search_products(
        &self,
        query: &str,
        min_price: Option<rust_decimal::Decimal>,
        max_price: Option<rust_decimal::Decimal>,
    ) -> Result<Vec<product::Model>, DbErr> {
        let mut find = product::Entity::find()
            .filter(product::Column::Name.contains(query))
            .filter(product::Column::IsActive.eq(true));
        
        if let Some(min_price) = min_price {
            find = find.filter(product::Column::Price.gte(min_price));
        }
        
        if let Some(max_price) = max_price {
            find = find.filter(product::Column::Price.lte(max_price));
        }
        
        let products = find
            .order_by_asc(product::Column::Price)
            .all(&self.db)
            .await?;
        
        Ok(products)
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderItem {
    pub product_id: i32,
    pub quantity: i32,
    pub price: rust_decimal::Decimal,
}
```

## 最佳实践

### 1. 项目结构

```
src/
├── entities/              # 实体定义
│   ├── mod.rs
│   ├── user.rs
│   ├── post.rs
│   └── prelude.rs
├── services/              # 业务逻辑
│   ├── mod.rs
│   ├── user_service.rs
│   └── post_service.rs
├── migration/             # 迁移文件
│   ├── mod.rs
│   └── m20240101_000001_create_users.rs
├── config/                # 配置管理
│   ├── mod.rs
│   └── database.rs
└── main.rs
```

### 2. 错误处理

```rust
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("数据库错误: {0}")]
    Database(#[from] DbErr),
    #[error("验证错误: {0}")]
    Validation(String),
    #[error("业务逻辑错误: {0}")]
    Business(String),
    #[error("未找到资源: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, ServiceError>;
```

### 3. 连接管理

```rust
use sea_orm::*;
use std::env;

pub async fn create_database_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        .acquire_timeout(std::time::Duration::from_secs(8))
        .idle_timeout(std::time::Duration::from_secs(8))
        .max_lifetime(std::time::Duration::from_secs(8))
        .sqlx_logging(true);
    
    Database::connect(opt).await
}
```

### 4. 测试配置

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        
        // 运行迁移
        Migrator::up(&db, None).await.unwrap();
        
        db
    }
    
    #[tokio::test]
    async fn test_user_service() {
        let db = setup_test_db().await;
        let service = UserService::new(db);
        
        // 测试逻辑
    }
}
```

## 总结

SeaORM 是一个功能强大的异步 ORM 框架，提供了现代化的数据库操作体验。通过本教程，您应该能够：

1. 理解 SeaORM 的核心概念和设计理念
2. 设置数据库连接和迁移系统
3. 定义实体和执行 CRUD 操作
4. 构建复杂查询和处理关系
5. 实现事务处理和连接池管理
6. 使用 JSON 支持和分页功能
7. 编写测试和使用 CLI 工具

关键要点：
- 异步优先的设计
- 类型安全的查询构建
- 强大的关系映射
- 灵活的迁移系统
- 完善的测试支持

SeaORM 的设计理念是提供一个现代化、高性能的 ORM 解决方案，它在保持类型安全的同时提供了灵活的动态查询能力。