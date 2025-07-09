# Axum 0.8.4 - Rust Web框架详细使用教程

## 📋 基本信息

- **名称**: axum
- **版本**: 0.8.4
- **发布日期**: 2025年4月30日
- **描述**: 专注于人机工程学和模块化的Web框架
- **仓库**: https://github.com/tokio-rs/axum
- **许可证**: MIT
- **下载量**: 139,574,242+ 次
- **维护者**: Tokio团队

## 🚀 框架特性

### 核心优势
- **100%安全**: 使用 `#![forbid(unsafe_code)]` 确保完全用安全的Rust实现
- **异步优先**: 基于Tokio运行时构建，原生支持异步操作
- **类型安全**: 强类型系统，编译时错误检查
- **模块化设计**: 高度可组合的架构
- **生态系统集成**: 完全兼容Tower和tower-http生态系统
- **高性能**: 基于Hyper构建，性能接近原生Hyper

### 主要功能模块

1. **路由系统 (Router)**
   - 无宏API的路由定义
   - 嵌套路由支持
   - 动态路径参数

2. **提取器 (Extractors)**
   - 声明式请求解析
   - 内置多种提取器类型
   - 自定义提取器支持

3. **响应生成 (Responses)**
   - 最小化样板代码
   - 多种响应类型支持
   - 自定义响应实现

4. **中间件 (Middleware)**
   - Tower生态系统兼容
   - 丰富的内置中间件
   - 自定义中间件开发

## 📦 安装与配置

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
axum = "0.8.4"
tokio = { version = "1.0", features = ["full"] }
```

### 可选功能 (Features)

```toml
[dependencies]
axum = { version = "0.8.4", features = [
    "json",        # JSON序列化支持
    "form",        # 表单数据解析
    "multipart",   # 文件上传支持
    "ws",          # WebSocket支持
    "macros",      # 宏支持
    "tracing",     # 日志追踪
    "http2",       # HTTP/2支持
] }
```

## 🛠️ 基础使用

### 1. Hello World示例

```rust
use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Hello, World!</h1>") }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 2. 路由定义

```rust
use axum::{
    extract::Path,
    routing::{get, post, put, delete},
    Router,
};

let app = Router::new()
    .route("/", get(root))
    .route("/users", get(get_users).post(create_user))
    .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
    .route("/hello/:name", get(hello));

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}
```

### 3. 请求提取器

```rust
use axum::{
    extract::{Path, Query, Json},
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct Params {
    page: Option<u32>,
    per_page: Option<u32>,
}

// 路径参数提取
async fn get_user(Path(user_id): Path<u64>) -> String {
    format!("User ID: {}", user_id)
}

// 查询参数提取
async fn get_users(Query(params): Query<Params>) -> String {
    format!("Page: {}, Per page: {}", 
        params.page.unwrap_or(1), 
        params.per_page.unwrap_or(10)
    )
}

// JSON请求体提取
async fn create_user(Json(payload): Json<CreateUser>) -> ResponseJson<User> {
    let user = User {
        id: 1337,
        name: payload.name,
        email: payload.email,
    };
    ResponseJson(user)
}
```

### 4. 状态管理

```rust
use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppState {
    counter: Arc<RwLock<i32>>,
}

async fn get_counter(State(state): State<AppState>) -> Json<i32> {
    let counter = state.counter.read().await;
    Json(*counter)
}

async fn increment_counter(State(state): State<AppState>) -> Json<i32> {
    let mut counter = state.counter.write().await;
    *counter += 1;
    Json(*counter)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(RwLock::new(0)),
    };

    let app = Router::new()
        .route("/counter", get(get_counter))
        .route("/increment", post(increment_counter))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 🔧 中间件使用

### 1. 内置中间件

```rust
use axum::{
    middleware,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
};
use std::time::Duration;

let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(CompressionLayer::new())
            .layer(CorsLayer::permissive())
    );
```

### 2. 自定义中间件

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            // 验证token逻辑
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(middleware::from_fn(auth_middleware));
```

## 📁 文件处理

### 1. 静态文件服务

```rust
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

let app = Router::new()
    .route("/api/hello", get(|| async { "Hello API!" }))
    .nest_service("/static", ServeDir::new("assets"));
```

### 2. 文件上传

```rust
use axum::{
    extract::Multipart,
    response::Html,
    routing::{get, post},
    Router,
};

async fn upload_form() -> Html<&'static str> {
    Html(r#"
        <form action="/upload" method="post" enctype="multipart/form-data">
            <input type="file" name="file">
            <input type="submit" value="Upload">
        </form>
    "#)
}

async fn upload_file(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        
        println!("Field: {}, Filename: {}, Size: {}", name, filename, data.len());
        
        // 保存文件逻辑
        tokio::fs::write(&filename, data).await.unwrap();
    }
    "File uploaded successfully".to_string()
}

let app = Router::new()
    .route("/", get(upload_form))
    .route("/upload", post(upload_file));
```

## 🔌 WebSocket支持

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                if socket.send(msg).await.is_err() {
                    break;
                }
            }
        }
    }
}

let app = Router::new()
    .route("/ws", get(websocket_handler));
```

## 🎯 错误处理

### 1. 自定义错误类型

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
enum AppError {
    NotFound,
    InternalServerError,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn fallible_handler() -> Result<String, AppError> {
    Err(AppError::BadRequest("Something went wrong".to_string()))
}
```

### 2. 全局错误处理

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

async fn handle_error(error: Box<dyn std::error::Error + Send + Sync>) -> Response {
    let body = Json(json!({
        "error": error.to_string(),
    }));

    (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
}
```

## 🧪 测试

### 1. 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello_world() {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

### 2. 集成测试

```rust
use axum_test::TestServer;

#[tokio::test]
async fn test_api() {
    let app = create_app();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/users").await;
    assert_eq!(response.status_code(), 200);
}
```

## 🔧 配置与部署

### 1. 生产环境配置

```rust
use axum::{
    extract::ConnectInfo,
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {}", addr)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
```

### 2. 优雅关闭

```rust
use axum::{routing::get, Router};
use tokio::signal;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
```

## 🎨 最佳实践

### 1. 项目结构

```
src/
├── main.rs          # 应用入口
├── handlers/        # 处理器模块
│   ├── mod.rs
│   ├── auth.rs
│   └── users.rs
├── middleware/      # 中间件
│   ├── mod.rs
│   └── auth.rs
├── models/          # 数据模型
│   ├── mod.rs
│   └── user.rs
├── routes/          # 路由定义
│   ├── mod.rs
│   └── api.rs
└── utils/           # 工具函数
    ├── mod.rs
    └── database.rs
```

### 2. 模块化设计

```rust
// src/handlers/users.rs
use axum::{extract::Path, Json};
use crate::models::User;

pub async fn get_user(Path(id): Path<u64>) -> Json<User> {
    // 用户获取逻辑
    Json(User::default())
}

pub async fn create_user(Json(user): Json<User>) -> Json<User> {
    // 用户创建逻辑
    Json(user)
}

// src/routes/api.rs
use axum::{routing::get, Router};
use crate::handlers::users;

pub fn create_routes() -> Router {
    Router::new()
        .route("/users/:id", get(users::get_user))
        .route("/users", post(users::create_user))
}
```

### 3. 性能优化

```rust
use axum::{
    middleware,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use std::time::Duration;

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB限制
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(CompressionLayer::new())
    );
```

## 📚 进阶主题

### 1. 自定义提取器

```rust
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};

struct ApiKey(String);

#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let api_key = parts
            .headers
            .get("X-API-Key")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| {
                (StatusCode::UNAUTHORIZED, "Missing API key").into_response()
            })?;

        Ok(ApiKey(api_key.to_string()))
    }
}
```

### 2. 数据库集成

```rust
use axum::{
    extract::State,
    routing::get,
    Router,
};
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    
    Json(users)
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    let state = AppState { db: pool };
    
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(state);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 🔍 调试与监控

### 1. 日志配置

```rust
use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/", get(hello))
        .layer(TraceLayer::new_for_http());

    info!("Server starting on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn hello() -> &'static str {
    info!("Hello endpoint called");
    "Hello, World!"
}
```

### 2. 健康检查

```rust
use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    let health_status = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
    });

    (StatusCode::OK, Json(health_status))
}

let app = Router::new()
    .route("/health", get(health_check));
```

## 📈 性能基准测试

根据2025年的基准测试结果，Axum在性能方面表现优异：

- **吞吐量**: 仅次于Actix-web，处理大量并发请求能力强
- **内存使用**: 12-20MB的内存占用，资源效率高
- **延迟**: 低延迟响应，适合高性能应用
- **资源效率**: 在中等负载下比Actix-web更高效

## 🎯 选择Axum的时机

### 推荐使用场景：
- 新项目开发
- 需要强类型安全的应用
- 与Tokio生态系统深度集成
- 注重代码可读性和维护性
- 需要丰富中间件支持

### 与其他框架对比：
- **vs Actix-web**: Axum提供更好的类型安全和更简单的API
- **vs Rocket**: Axum异步支持更好，性能更优
- **vs Warp**: Axum API更直观，学习曲线更平缓

## 🚀 总结

Axum 0.8.4 是一个现代化的Rust Web框架，它结合了性能、安全性和开发体验。其基于Tokio和Tower的设计使其在2025年成为Rust Web开发的首选框架之一。

无论是构建RESTful API、WebSocket应用还是全栈Web应用，Axum都能提供强大的功能和优秀的性能。通过本教程的学习，你应该能够掌握Axum的核心概念并开始构建自己的Web应用。

---

*本教程基于Axum 0.8.4版本编写，更新日期：2025年7月*