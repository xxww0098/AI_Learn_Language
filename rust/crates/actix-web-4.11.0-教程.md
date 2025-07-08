# Actix-web 4.11.0 - Rust 高性能Web框架完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [路由系统](#路由系统)
- [请求处理](#请求处理)
- [响应生成](#响应生成)
- [中间件](#中间件)
- [状态管理](#状态管理)
- [数据库集成](#数据库集成)
- [WebSocket支持](#WebSocket支持)
- [静态文件服务](#静态文件服务)
- [安全性](#安全性)
- [测试](#测试)
- [部署](#部署)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Actix-web 是一个功能强大、实用且极其快速的 Rust Web 框架，基于 Actor 模型构建，提供了出色的性能和灵活性。

### 核心特性
- **高性能**: 基于 Actix Actor 系统，提供卓越的并发性能
- **类型安全**: 完全的类型安全，编译时错误检查
- **异步支持**: 完整的异步/await支持
- **WebSocket**: 内置 WebSocket 支持
- **中间件**: 丰富的中间件生态系统
- **可扩展**: 灵活的架构，易于扩展

### 版本信息
- **当前版本**: 4.11.0
- **发布时间**: 2025-05-12
- **下载次数**: 39,818,610+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
actix-web = "4.11.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 基本示例

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 简单的处理函数
async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello, Actix-web!"))
}

// 带参数的处理函数
async fn get_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = User {
        id: user_id,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    Ok(HttpResponse::Ok().json(user))
}

// POST 处理函数
async fn create_user(user: web::Json<User>) -> Result<HttpResponse> {
    println!("创建用户: {:?}", user);
    Ok(HttpResponse::Created().json(&user.into_inner()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(hello))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 核心概念

### App 和 HttpServer

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result};

async fn handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello from handler!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer 管理多个 App 实例
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handler))
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
            )
    })
    .bind("127.0.0.1:8080")?
    .workers(4) // 设置工作线程数
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
```

### 请求和响应生命周期

```rust
use actix_web::{web, App, HttpRequest, HttpResponse, Result};

async fn analyze_request(req: HttpRequest) -> Result<HttpResponse> {
    let method = req.method();
    let path = req.path();
    let query = req.query_string();
    let headers = req.headers();
    
    // 获取特定头部
    let user_agent = headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    let response = serde_json::json!({
        "method": method.to_string(),
        "path": path,
        "query": query,
        "user_agent": user_agent,
        "headers": headers.iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("invalid")))
            .collect::<std::collections::HashMap<_, _>>()
    });
    
    Ok(HttpResponse::Ok().json(response))
}

fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/analyze", web::get().to(analyze_request));
}
```

## 路由系统

### 基本路由

```rust
use actix_web::{web, App, HttpResponse, Result};

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Index page"))
}

async fn hello_name(name: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(format!("Hello, {}!", name)))
}

async fn user_profile(path: web::Path<(u32, String)>) -> Result<HttpResponse> {
    let (user_id, action) = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("User {} - Action: {}", user_id, action)))
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .route("/hello/{name}", web::get().to(hello_name))
        .route("/users/{id}/{action}", web::get().to(user_profile))
        .route("/users/{id}", web::get().to(get_user))
        .route("/users/{id}", web::put().to(update_user))
        .route("/users/{id}", web::delete().to(delete_user))
        .route("/users", web::post().to(create_user));
}

async fn get_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user_id,
        "action": "get"
    })))
}

async fn update_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user_id,
        "action": "update"
    })))
}

async fn delete_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user_id,
        "action": "delete"
    })))
}

async fn create_user() -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(serde_json::json!({
        "action": "create"
    })))
}
```

### 路由组和作用域

```rust
use actix_web::{web, App, HttpResponse, Result};

// API v1 处理函数
async fn api_v1_users() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API v1 - Users"))
}

async fn api_v1_posts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API v1 - Posts"))
}

// API v2 处理函数
async fn api_v2_users() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API v2 - Users"))
}

async fn api_v2_posts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API v2 - Posts"))
}

// 管理后台处理函数
async fn admin_dashboard() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Admin Dashboard"))
}

async fn admin_users() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Admin Users"))
}

fn configure_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/users", web::get().to(api_v1_users))
            .route("/posts", web::get().to(api_v1_posts))
    );
}

fn configure_api_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v2")
            .route("/users", web::get().to(api_v2_users))
            .route("/posts", web::get().to(api_v2_posts))
    );
}

fn configure_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/dashboard", web::get().to(admin_dashboard))
            .route("/users", web::get().to(admin_users))
    );
}

fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >
> {
    App::new()
        .service(
            web::scope("/api")
                .configure(configure_api_v1)
                .configure(configure_api_v2)
        )
        .configure(configure_admin)
}
```

### 路由守卫

```rust
use actix_web::{web, App, HttpRequest, HttpResponse, Result, guard};

async fn api_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("API endpoint"))
}

async fn json_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("JSON endpoint"))
}

async fn admin_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Admin endpoint"))
}

fn configure_guards(cfg: &mut web::ServiceConfig) {
    cfg
        // 只接受 GET 请求
        .route("/api", web::route().guard(guard::Get()).to(api_handler))
        // 只接受 Content-Type 为 application/json 的请求
        .route("/json", web::route()
            .guard(guard::Header("content-type", "application/json"))
            .to(json_handler))
        // 只接受来自特定主机的请求
        .route("/admin", web::route()
            .guard(guard::Host("admin.example.com"))
            .to(admin_handler))
        // 自定义守卫
        .route("/custom", web::route()
            .guard(guard::fn_guard(|req| {
                req.headers().contains_key("x-api-key")
            }))
            .to(api_handler));
}
```

## 请求处理

### 提取器 (Extractors)

```rust
use actix_web::{web, App, HttpRequest, HttpResponse, Result, FromRequest};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserQuery {
    name: Option<String>,
    age: Option<u32>,
    active: Option<bool>,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

// 路径参数提取
async fn get_user_by_id(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = User {
        id: user_id,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 30,
    };
    Ok(HttpResponse::Ok().json(user))
}

// 查询参数提取
async fn search_users(query: web::Query<UserQuery>) -> Result<HttpResponse> {
    let name = query.name.as_deref().unwrap_or("所有用户");
    let age = query.age.unwrap_or(0);
    let active = query.active.unwrap_or(true);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "search": {
            "name": name,
            "age": age,
            "active": active
        },
        "results": ["用户1", "用户2", "用户3"]
    })))
}

// JSON 请求体提取
async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<HttpResponse> {
    let user = User {
        id: 1,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
        age: user_data.age,
    };
    
    println!("创建用户: {:?}", user);
    Ok(HttpResponse::Created().json(user))
}

// 表单数据提取
#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(form: web::Form<LoginForm>) -> Result<HttpResponse> {
    if form.username == "admin" && form.password == "password" {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "登录成功"
        })))
    } else {
        Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "error",
            "message": "用户名或密码错误"
        })))
    }
}

// 头部提取
async fn get_user_agent(req: HttpRequest) -> Result<HttpResponse> {
    let user_agent = req.headers().get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_agent": user_agent
    })))
}

// 组合多个提取器
async fn complex_handler(
    path: web::Path<(u32, String)>,
    query: web::Query<UserQuery>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (user_id, action) = path.into_inner();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": user_id,
        "action": action,
        "query": {
            "name": query.name,
            "age": query.age,
            "active": query.active
        },
        "method": req.method().to_string()
    })))
}
```

### 自定义提取器

```rust
use actix_web::{web, App, HttpRequest, HttpResponse, Result, FromRequest, Error};
use futures::future::{Ready, ready};
use std::task::{Context, Poll};

// 自定义提取器：API Key
struct ApiKey(String);

impl FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let api_key = req.headers().get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        match api_key {
            Some(key) if key == "secret-key" => ready(Ok(ApiKey(key))),
            _ => ready(Err(actix_web::error::ErrorUnauthorized("Invalid API key"))),
        }
    }
}

// 使用自定义提取器
async fn protected_endpoint(api_key: ApiKey) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "访问受保护的端点",
        "api_key": api_key.0
    })))
}

// 用户信息提取器
#[derive(serde::Serialize)]
struct UserInfo {
    id: u32,
    name: String,
    role: String,
}

impl FromRequest for UserInfo {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // 从 JWT token 或 session 中提取用户信息
        let token = req.headers().get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        match token {
            Some("valid-token") => ready(Ok(UserInfo {
                id: 1,
                name: "张三".to_string(),
                role: "admin".to_string(),
            })),
            _ => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
        }
    }
}

async fn user_profile(user: UserInfo) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(user))
}
```

## 响应生成

### 响应类型

```rust
use actix_web::{web, HttpResponse, Result, http::StatusCode};
use serde_json::json;

// JSON 响应
async fn json_response() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello, World!",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "data": {
            "items": [1, 2, 3, 4, 5]
        }
    })))
}

// 文本响应
async fn text_response() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("这是一个文本响应"))
}

// HTML 响应
async fn html_response() -> Result<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Actix-web Demo</title>
    </head>
    <body>
        <h1>欢迎使用 Actix-web!</h1>
        <p>这是一个 HTML 响应示例。</p>
    </body>
    </html>
    "#;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

// 自定义状态码
async fn custom_status() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::CREATED)
        .json(json!({
            "status": "created",
            "message": "资源创建成功"
        })))
}

// 错误响应
async fn error_response() -> Result<HttpResponse> {
    Ok(HttpResponse::BadRequest().json(json!({
        "error": "bad_request",
        "message": "请求参数无效",
        "details": {
            "field": "email",
            "reason": "格式不正确"
        }
    })))
}

// 重定向响应
async fn redirect_response() -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
        .append_header(("Location", "/new-location"))
        .finish())
}

// 设置响应头
async fn headers_response() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("X-Custom-Header", "custom-value"))
        .append_header(("Cache-Control", "no-cache"))
        .json(json!({
            "message": "带有自定义头部的响应"
        })))
}

// 流式响应
use actix_web::web::Bytes;
use futures::stream::iter;

async fn streaming_response() -> Result<HttpResponse> {
    let data = vec![
        "chunk1\n",
        "chunk2\n", 
        "chunk3\n",
    ];
    
    let stream = iter(data.into_iter().map(|chunk| {
        Ok::<_, actix_web::Error>(Bytes::from(chunk))
    }));
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(stream))
}
```

### 文件响应

```rust
use actix_web::{web, HttpResponse, Result, http::header};
use actix_files::NamedFile;

// 文件下载
async fn download_file() -> Result<HttpResponse> {
    let file = NamedFile::open("./static/document.pdf")?;
    Ok(HttpResponse::Ok()
        .append_header(header::ContentDisposition::attachment("document.pdf"))
        .streaming(file))
}

// 内联文件显示
async fn view_file() -> Result<HttpResponse> {
    let file = NamedFile::open("./static/image.jpg")?;
    Ok(HttpResponse::Ok()
        .append_header(header::ContentDisposition::inline())
        .streaming(file))
}

// 动态生成文件
async fn generate_csv() -> Result<HttpResponse> {
    let csv_data = "ID,Name,Email\n1,张三,zhangsan@example.com\n2,李四,lisi@example.com";
    
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "text/csv"))
        .append_header(header::ContentDisposition::attachment("users.csv"))
        .body(csv_data))
}
```

## 中间件

### 内置中间件

```rust
use actix_web::{
    web, App, HttpServer, HttpResponse, Result,
    middleware::{Logger, DefaultHeaders, Compress, NormalizePath}
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // 日志中间件
            .wrap(Logger::default())
            // 压缩中间件
            .wrap(Compress::default())
            // 路径规范化中间件
            .wrap(NormalizePath::trim())
            // 默认头部中间件
            .wrap(DefaultHeaders::new()
                .add(("X-Version", "1.0"))
                .add(("X-Frame-Options", "DENY")))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello with middleware!"))
}
```

### 自定义中间件

```rust
use actix_web::{
    web, App, HttpServer, HttpResponse, Result, Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
};
use futures::future::{Ready, ready};
use std::task::{Context, Poll};

// 请求计时中间件
pub struct Timing;

impl<S, B> Transform<S, ServiceRequest> for Timing
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimingMiddleware { service }))
    }
}

pub struct TimingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let elapsed = start.elapsed();
            
            // 添加响应头
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-response-time"),
                HeaderValue::from_str(&format!("{}ms", elapsed.as_millis())).unwrap(),
            );
            
            println!("请求处理时间: {:?}", elapsed);
            Ok(res)
        })
    }
}

// 认证中间件
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 检查认证头
        let auth_header = req.headers().get("authorization");
        
        if let Some(auth) = auth_header {
            if let Ok(auth_str) = auth.to_str() {
                if auth_str.starts_with("Bearer ") {
                    // 验证通过，继续处理请求
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        fut.await
                    });
                }
            }
        }
        
        // 认证失败
        Box::pin(async move {
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "unauthorized",
                    "message": "需要有效的认证token"
                }))
                .into_body();
            
            Ok(req.into_response(response))
        })
    }
}

// 使用自定义中间件
fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >
> {
    App::new()
        .wrap(Timing)
        .service(
            web::scope("/api")
                .wrap(Auth)
                .route("/protected", web::get().to(protected_handler))
        )
        .route("/public", web::get().to(public_handler))
}

async fn protected_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("这是受保护的端点"))
}

async fn public_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("这是公开的端点"))
}
```

## 状态管理

### 应用状态

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use std::sync::Mutex;
use std::collections::HashMap;

// 应用状态结构
struct AppState {
    counter: Mutex<usize>,
    users: Mutex<HashMap<u32, String>>,
}

// 计数器处理函数
async fn increment_counter(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "counter": *counter
    })))
}

async fn get_counter(data: web::Data<AppState>) -> Result<HttpResponse> {
    let counter = data.counter.lock().unwrap();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "counter": *counter
    })))
}

// 用户管理
async fn add_user(
    data: web::Data<AppState>,
    user_info: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let mut users = data.users.lock().unwrap();
    let user_id = users.len() as u32 + 1;
    let user_name = user_info.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    users.insert(user_id, user_name.clone());
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": user_id,
        "name": user_name,
        "message": "用户创建成功"
    })))
}

async fn get_users(data: web::Data<AppState>) -> Result<HttpResponse> {
    let users = data.users.lock().unwrap();
    let user_list: Vec<_> = users.iter()
        .map(|(id, name)| serde_json::json!({
            "id": id,
            "name": name
        }))
        .collect();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "users": user_list,
        "total": users.len()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/counter", web::get().to(get_counter))
            .route("/counter/increment", web::post().to(increment_counter))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 数据库连接池

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, Error};
use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// 获取所有用户
async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let result = sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await;
    
    match result {
        Ok(rows) => {
            let users: Vec<User> = rows.into_iter()
                .map(|row| User {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                })
                .collect();
            
            Ok(HttpResponse::Ok().json(users))
        }
        Err(e) => {
            println!("数据库错误: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库查询失败"
            })))
        }
    }
}

// 创建用户
async fn create_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user_data.name,
        user_data.email
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(e) => {
            println!("创建用户失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "创建用户失败"
            })))
        }
    }
}

// 获取特定用户
async fn get_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    
    let result = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;
    
    match result {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "用户不存在"
        }))),
        Err(e) => {
            println!("查询用户失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "查询用户失败"
            })))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建数据库连接池
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/database".to_string());
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 数据库集成

### SQLx 集成

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, Error};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author_id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    author_id: i32,
}

#[derive(Deserialize)]
struct UpdatePost {
    title: Option<String>,
    content: Option<String>,
}

// 获取所有文章
async fn get_posts(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, author_id, created_at FROM posts ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        println!("数据库错误: {}", e);
        actix_web::error::ErrorInternalServerError("数据库查询失败")
    })?;
    
    Ok(HttpResponse::Ok().json(posts))
}

// 创建文章
async fn create_post(
    pool: web::Data<PgPool>,
    post_data: web::Json<CreatePost>,
) -> Result<HttpResponse, Error> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, author_id) VALUES ($1, $2, $3) RETURNING id, title, content, author_id, created_at"
    )
    .bind(&post_data.title)
    .bind(&post_data.content)
    .bind(&post_data.author_id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        println!("创建文章失败: {}", e);
        actix_web::error::ErrorInternalServerError("创建文章失败")
    })?;
    
    Ok(HttpResponse::Created().json(post))
}

// 获取特定文章
async fn get_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let post_id = path.into_inner();
    
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, author_id, created_at FROM posts WHERE id = $1"
    )
    .bind(post_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        println!("查询文章失败: {}", e);
        actix_web::error::ErrorInternalServerError("查询文章失败")
    })?;
    
    match post {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "文章不存在"
        }))),
    }
}

// 更新文章
async fn update_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    update_data: web::Json<UpdatePost>,
) -> Result<HttpResponse, Error> {
    let post_id = path.into_inner();
    
    // 构建动态更新查询
    let mut query = "UPDATE posts SET ".to_string();
    let mut params = Vec::new();
    let mut param_count = 0;
    
    if let Some(title) = &update_data.title {
        param_count += 1;
        query.push_str(&format!("title = ${}, ", param_count));
        params.push(title.as_str());
    }
    
    if let Some(content) = &update_data.content {
        param_count += 1;
        query.push_str(&format!("content = ${}, ", param_count));
        params.push(content.as_str());
    }
    
    if param_count == 0 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "没有提供更新字段"
        })));
    }
    
    // 移除最后的逗号和空格
    query.truncate(query.len() - 2);
    param_count += 1;
    query.push_str(&format!(" WHERE id = ${} RETURNING id, title, content, author_id, created_at", param_count));
    
    let mut sqlx_query = sqlx::query_as::<_, Post>(&query);
    
    // 绑定参数
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }
    sqlx_query = sqlx_query.bind(post_id);
    
    let post = sqlx_query
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| {
            println!("更新文章失败: {}", e);
            actix_web::error::ErrorInternalServerError("更新文章失败")
        })?;
    
    match post {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "文章不存在"
        }))),
    }
}

// 删除文章
async fn delete_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let post_id = path.into_inner();
    
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(post_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            println!("删除文章失败: {}", e);
            actix_web::error::ErrorInternalServerError("删除文章失败")
        })?;
    
    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "文章不存在"
        })))
    }
}

// 配置路由
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/posts")
            .route("", web::get().to(get_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post))
    );
}
```

## WebSocket支持

### 基本WebSocket服务

```rust
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result, Error};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler, Handler, Message};
use std::time::{Duration, Instant};

// WebSocket 消息类型
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

// WebSocket 会话
struct WebSocketSession {
    id: usize,
    hb: Instant,
}

impl WebSocketSession {
    fn new() -> Self {
        WebSocketSession {
            id: 0,
            hb: Instant::now(),
        }
    }
    
    // 心跳检测
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("WebSocket 心跳超时，断开连接");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        println!("WebSocket 连接建立");
    }
    
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket 连接关闭");
    }
}

// 处理 WebSocket 消息
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text = text.trim();
                println!("收到消息: {}", text);
                
                // 处理不同类型的消息
                match text {
                    "ping" => ctx.text("pong"),
                    "time" => ctx.text(chrono::Utc::now().to_rfc3339()),
                    "close" => ctx.close(Some(ws::CloseCode::Normal.into())),
                    _ => {
                        // 回显消息
                        ctx.text(format!("Echo: {}", text));
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("收到二进制消息: {} bytes", bin.len());
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket 关闭: {:?}", reason);
                ctx.close(reason);
            }
            _ => {}
        }
    }
}

// WebSocket 握手处理
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("WebSocket 握手请求");
    ws::start(WebSocketSession::new(), &req, stream)
}

// 聊天室示例
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Clients = Arc<Mutex<HashMap<usize, actix::Addr<ChatSession>>>>;

struct ChatSession {
    id: usize,
    clients: Clients,
    hb: Instant,
}

impl ChatSession {
    fn new(clients: Clients) -> Self {
        ChatSession {
            id: 0,
            clients,
            hb: Instant::now(),
        }
    }
    
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("客户端 {} 心跳超时", act.id);
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        
        // 生成唯一ID
        self.id = rand::random::<usize>();
        
        // 添加到客户端列表
        self.clients.lock().unwrap().insert(self.id, ctx.address());
        
        println!("客户端 {} 加入聊天室", self.id);
        
        // 广播加入消息
        let message = serde_json::json!({
            "type": "join",
            "user_id": self.id,
            "message": format!("用户 {} 加入了聊天室", self.id)
        });
        
        self.broadcast_message(message.to_string());
    }
    
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // 从客户端列表移除
        self.clients.lock().unwrap().remove(&self.id);
        
        println!("客户端 {} 离开聊天室", self.id);
        
        // 广播离开消息
        let message = serde_json::json!({
            "type": "leave",
            "user_id": self.id,
            "message": format!("用户 {} 离开了聊天室", self.id)
        });
        
        self.broadcast_message(message.to_string());
    }
}

impl ChatSession {
    fn broadcast_message(&self, message: String) {
        let clients = self.clients.lock().unwrap();
        for (id, addr) in clients.iter() {
            if *id != self.id {
                addr.do_send(WsMessage(message.clone()));
            }
        }
    }
}

impl Handler<WsMessage> for ChatSession {
    type Result = ();
    
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let text = text.trim();
                
                // 解析消息
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                    if let Some(msg_type) = json.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "message" => {
                                if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
                                    let broadcast_msg = serde_json::json!({
                                        "type": "message",
                                        "user_id": self.id,
                                        "content": content,
                                        "timestamp": chrono::Utc::now().to_rfc3339()
                                    });
                                    
                                    self.broadcast_message(broadcast_msg.to_string());
                                }
                            }
                            _ => {
                                ctx.text(serde_json::json!({
                                    "type": "error",
                                    "message": "未知的消息类型"
                                }).to_string());
                            }
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {
                println!("收到二进制消息，暂不支持");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => {}
        }
    }
}

// 聊天室 WebSocket 处理
async fn chat_websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, Error> {
    ws::start(ChatSession::new(clients.get_ref().clone()), &req, stream)
}

// 配置 WebSocket 路由
fn configure_websocket(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/ws", web::get().to(websocket_handler))
        .route("/chat", web::get().to(chat_websocket_handler));
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(clients.clone()))
            .configure(configure_websocket)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 静态文件服务

### 静态文件配置

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware};
use actix_files::{Files, NamedFile};

// 自定义静态文件处理
async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

async fn custom_404() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(include_str!("../static/404.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // 服务静态文件
            .service(Files::new("/static", "static").show_files_listing())
            // 自定义路由
            .route("/favicon.ico", web::get().to(favicon))
            // 图片文件服务
            .service(
                Files::new("/images", "assets/images")
                    .use_etag(true)
                    .use_last_modified(true)
            )
            // SPA 应用支持
            .service(
                Files::new("/app", "dist")
                    .index_file("index.html")
            )
            // 默认处理（404）
            .default_service(web::get().to(custom_404))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 安全性

### CORS 配置

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, http::header};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://yourdomain.com")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .route("/api/data", web::get().to(get_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_data() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "CORS enabled data"
    })))
}
```

### 安全头部

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::DefaultHeaders};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new()
                .add(("X-Frame-Options", "DENY"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                .add(("Content-Security-Policy", "default-src 'self'"))
            )
            .route("/", web::get().to(secure_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn secure_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Secure response"))
}
```

## 测试

### 单元测试

```rust
use actix_web::{test, web, App, HttpResponse, Result};
use serde_json::json;

async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello, World!"
    })))
}

async fn echo(data: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(data.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new().route("/hello", web::get().to(hello))
        ).await;
        
        let req = test::TestRequest::get()
            .uri("/hello")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        let result: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(result["message"], "Hello, World!");
    }
    
    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(
            App::new().route("/echo", web::post().to(echo))
        ).await;
        
        let payload = json!({
            "name": "test",
            "value": 42
        });
        
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&payload)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        let result: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(result, payload);
    }
}
```

### 集成测试

```rust
use actix_web::{test, web, App, HttpResponse, Result, middleware};
use serde_json::json;

// 测试应用工厂
fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >
> {
    App::new()
        .wrap(middleware::Logger::default())
        .service(
            web::scope("/api")
                .route("/users", web::get().to(get_users))
                .route("/users", web::post().to(create_user))
        )
}

async fn get_users() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    })))
}

async fn create_user(user: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(json!({
        "id": 3,
        "name": user.get("name").unwrap_or(&json!("Unknown")),
        "message": "User created"
    })))
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_get_users() {
        let app = test::init_service(create_app()).await;
        
        let req = test::TestRequest::get()
            .uri("/api/users")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        let result: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(result["users"].as_array().unwrap().len(), 2);
    }
    
    #[actix_web::test]
    async fn test_create_user() {
        let app = test::init_service(create_app()).await;
        
        let user_data = json!({
            "name": "Charlie",
            "email": "charlie@example.com"
        });
        
        let req = test::TestRequest::post()
            .uri("/api/users")
            .set_json(&user_data)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        
        let result: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(result["id"], 3);
        assert_eq!(result["name"], "Charlie");
    }
}
```

## 实战案例

### RESTful API 服务

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result, Error, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    created_at: String,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

// 应用状态
struct AppState {
    tasks: Mutex<HashMap<u32, Task>>,
    next_id: Mutex<u32>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            tasks: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }
}

// 获取所有任务
async fn get_tasks(data: web::Data<AppState>) -> Result<HttpResponse> {
    let tasks = data.tasks.lock().unwrap();
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "tasks": task_list,
        "total": task_list.len()
    })))
}

// 获取特定任务
async fn get_task(
    data: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse> {
    let task_id = path.into_inner();
    let tasks = data.tasks.lock().unwrap();
    
    match tasks.get(&task_id) {
        Some(task) => Ok(HttpResponse::Ok().json(task)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Task not found",
            "task_id": task_id
        }))),
    }
}

// 创建任务
async fn create_task(
    data: web::Data<AppState>,
    task_data: web::Json<CreateTask>,
) -> Result<HttpResponse> {
    let mut tasks = data.tasks.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();
    
    let task = Task {
        id: *next_id,
        title: task_data.title.clone(),
        description: task_data.description.clone(),
        completed: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    tasks.insert(*next_id, task.clone());
    *next_id += 1;
    
    Ok(HttpResponse::Created().json(task))
}

// 更新任务
async fn update_task(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    update_data: web::Json<UpdateTask>,
) -> Result<HttpResponse> {
    let task_id = path.into_inner();
    let mut tasks = data.tasks.lock().unwrap();
    
    match tasks.get_mut(&task_id) {
        Some(task) => {
            if let Some(title) = &update_data.title {
                task.title = title.clone();
            }
            if let Some(description) = &update_data.description {
                task.description = description.clone();
            }
            if let Some(completed) = update_data.completed {
                task.completed = completed;
            }
            
            Ok(HttpResponse::Ok().json(task.clone()))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Task not found",
            "task_id": task_id
        }))),
    }
}

// 删除任务
async fn delete_task(
    data: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse> {
    let task_id = path.into_inner();
    let mut tasks = data.tasks.lock().unwrap();
    
    match tasks.remove(&task_id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Task not found",
            "task_id": task_id
        }))),
    }
}

// 健康检查
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let app_state = web::Data::new(AppState::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new()
                .add(("X-API-Version", "1.0"))
            )
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    .service(
                        web::scope("/tasks")
                            .route("", web::get().to(get_tasks))
                            .route("", web::post().to(create_task))
                            .route("/{id}", web::get().to(get_task))
                            .route("/{id}", web::put().to(update_task))
                            .route("/{id}", web::delete().to(delete_task))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 最佳实践

### 1. 项目结构

```
src/
├── main.rs              # 应用入口
├── handlers/            # 请求处理函数
│   ├── mod.rs
│   ├── auth.rs
│   └── users.rs
├── models/              # 数据模型
│   ├── mod.rs
│   └── user.rs
├── middleware/          # 自定义中间件
│   ├── mod.rs
│   └── auth.rs
├── services/            # 业务逻辑
│   ├── mod.rs
│   └── user_service.rs
├── config/              # 配置管理
│   ├── mod.rs
│   └── database.rs
└── utils/               # 工具函数
    ├── mod.rs
    └── validation.rs
```

### 2. 错误处理

```rust
use actix_web::{ResponseError, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized,
    NotFound(String),
    InternalError(String),
    DatabaseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "bad_request",
                "message": msg
            })),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "unauthorized",
                "message": "Authentication required"
            })),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "not_found",
                "message": msg
            })),
            ApiError::InternalError(msg) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal_error",
                "message": msg
            })),
            ApiError::DatabaseError(msg) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "database_error",
                "message": msg
            })),
        }
    }
}

// 使用自定义错误
async fn example_handler() -> Result<HttpResponse, ApiError> {
    // 业务逻辑
    if some_condition {
        return Err(ApiError::BadRequest("Invalid input".to_string()));
    }
    
    Ok(HttpResponse::Ok().json("Success"))
}
```

### 3. 配置管理

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        
        settings
            .merge(config::Environment::with_prefix("APP"))?
            .merge(config::File::with_name("config/default").required(false))?;
            
        settings.try_into()
    }
}
```

### 4. 性能优化

```rust
use actix_web::{web, App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 启用压缩
            .wrap(middleware::Compress::default())
            // 启用日志
            .wrap(middleware::Logger::default())
            // 设置默认头部
            .wrap(middleware::DefaultHeaders::new()
                .add(("Server", "Actix-Web"))
            )
    })
    .bind("127.0.0.1:8080")?
    .workers(4) // 设置工作线程数
    .run()
    .await
}
```

## 总结

Actix-web 是一个功能强大的 Rust Web 框架，提供了构建高性能 Web 应用所需的所有工具。通过本教程，您应该能够：

1. 理解 Actix-web 的核心概念和架构
2. 构建完整的 RESTful API 服务
3. 实现 WebSocket 实时通信
4. 集成数据库和中间件
5. 编写测试和优化性能

关键要点：
- 使用类型安全的提取器处理请求
- 合理设计中间件和错误处理
- 利用异步特性提高性能
- 遵循 RESTful 设计原则
- 注重安全性和测试

Actix-web 的高性能和灵活性使其成为构建现代 Web 应用的优秀选择。