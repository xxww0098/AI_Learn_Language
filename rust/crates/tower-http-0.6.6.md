# Tower-HTTP 0.6.6 中文教程

## 简介

Tower-HTTP 是基于 Tower 生态系统的 HTTP 客户端和服务器中间件和工具库。它提供了一套专门针对 HTTP 协议的中间件组件，可以轻松构建健壮的 HTTP 服务。

## 核心特性

- 🔧 丰富的 HTTP 中间件
- 🚀 异步高性能
- 🛡️ 内置安全功能
- 📊 请求/响应追踪
- 🔄 CORS 支持
- 📝 请求日志
- 🗜️ 响应压缩

## 主要中间件

### 1. CORS (跨域资源共享)

```rust
use tower_http::cors::{CorsLayer, Any};
use http::Method;

// 基本 CORS 配置
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any);

// 更详细的 CORS 配置
let cors = CorsLayer::new()
    .allow_origin("https://example.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE])
    .allow_credentials(true)
    .max_age(Duration::from_secs(3600));

// 应用到服务
let service = ServiceBuilder::new()
    .layer(cors)
    .service(my_service);
```

### 2. 请求追踪和日志

```rust
use tower_http::trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse};
use tracing::{info, span, Level};

// 基本追踪
let trace = TraceLayer::new_for_http();

// 自定义追踪
let trace = TraceLayer::new_for_http()
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO));

// 完全自定义追踪
let trace = TraceLayer::new_for_http()
    .make_span_with(|request: &Request<Body>| {
        span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
        )
    })
    .on_request(|request: &Request<Body>, _span: &Span| {
        info!("started {} {}", request.method(), request.uri());
    })
    .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
        info!(
            "finished with status {} in {:?}",
            response.status(),
            latency
        );
    });

let service = ServiceBuilder::new()
    .layer(trace)
    .service(my_service);
```

### 3. 响应压缩

```rust
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::{SizeAbove, DefaultPredicate};

// 基本压缩
let compression = CompressionLayer::new();

// 自定义压缩设置
let compression = CompressionLayer::new()
    .gzip(true)
    .deflate(true)
    .br(true)
    .compress_when(SizeAbove::new(1024)); // 只压缩大于1KB的响应

let service = ServiceBuilder::new()
    .layer(compression)
    .service(my_service);
```

### 4. 请求体限制

```rust
use tower_http::limit::RequestBodyLimitLayer;

// 限制请求体大小为1MB
let limit = RequestBodyLimitLayer::new(1024 * 1024);

let service = ServiceBuilder::new()
    .layer(limit)
    .service(my_service);
```

### 5. 静态文件服务

```rust
use tower_http::services::{ServeDir, ServeFile};
use std::path::PathBuf;

// 提供静态文件服务
let serve_dir = ServeDir::new("static")
    .append_index_html_on_directories(true)
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_deflate();

// 提供单个文件
let serve_file = ServeFile::new("index.html");

// 与路由结合使用
let app = Router::new()
    .route("/api/*", get(api_handler))
    .nest_service("/static", serve_dir)
    .fallback_service(serve_file);
```

### 6. 请求去重

```rust
use tower_http::decompression::DecompressionLayer;

// 自动解压请求体
let decompression = DecompressionLayer::new();

let service = ServiceBuilder::new()
    .layer(decompression)
    .service(my_service);
```

### 7. 敏感头部处理

```rust
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use http::header::{AUTHORIZATION, COOKIE};

// 标记敏感头部（用于日志脱敏）
let sensitive_headers = SetSensitiveHeadersLayer::new(
    std::iter::once(AUTHORIZATION)
        .chain(std::iter::once(COOKIE))
);

let service = ServiceBuilder::new()
    .layer(sensitive_headers)
    .service(my_service);
```

## 实际应用示例

### 完整的 HTTP 服务器

```rust
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    services::ServeDir,
    sensitive_headers::SetSensitiveHeadersLayer,
};
use tower::{ServiceBuilder, Service};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::time::Duration;
use http::header::{AUTHORIZATION, CONTENT_TYPE};

async fn api_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    
    match path {
        "/api/health" => {
            Ok(Response::new(Body::from("OK")))
        }
        "/api/users" => {
            let users = r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#;
            Ok(Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(users))
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化追踪
    tracing_subscriber::init();
    
    // 构建服务栈
    let service = ServiceBuilder::new()
        // 敏感头部处理
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        // CORS 配置
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .max_age(Duration::from_secs(3600))
        )
        // 请求追踪
        .layer(TraceLayer::new_for_http())
        // 响应压缩
        .layer(CompressionLayer::new())
        // 请求体大小限制
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB
        // 超时设置
        .timeout(Duration::from_secs(30))
        // 实际服务
        .service(tower::service_fn(api_handler));
    
    // 启动服务器
    let addr = ([127, 0, 0, 1], 3000).into();
    println!("Server running on http://{}", addr);
    
    Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await?;
    
    Ok(())
}
```

### 客户端中间件

```rust
use tower_http::{
    timeout::TimeoutLayer,
    retry::RetryLayer,
    trace::TraceLayer,
    auth::AuthorizeLayer,
};
use tower::{ServiceBuilder, Service};
use hyper::{Body, Request, Response, Client};
use std::time::Duration;

// 自定义重试策略
#[derive(Clone)]
struct HttpRetryPolicy;

impl<T> tower::retry::Policy<Request<Body>, Response<Body>, T> for HttpRetryPolicy {
    type Future = std::future::Ready<tower::retry::Retry>;
    
    fn retry(&self, _: &Request<Body>, result: Result<&Response<Body>, &T>) -> Self::Future {
        let retry = match result {
            Ok(response) => {
                match response.status().as_u16() {
                    // 对 5xx 错误进行重试
                    500..=599 => tower::retry::Retry::Yes,
                    _ => tower::retry::Retry::No,
                }
            }
            // 对网络错误进行重试
            Err(_) => tower::retry::Retry::Yes,
        };
        
        std::future::ready(retry)
    }
}

// 构建 HTTP 客户端
let client = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    .layer(RetryLayer::new(HttpRetryPolicy))
    .service(Client::new());

// 使用客户端
let request = Request::builder()
    .uri("https://api.example.com/users")
    .header("User-Agent", "my-app/1.0")
    .body(Body::empty())
    .unwrap();

let response = client.call(request).await?;
```

### 文件上传服务

```rust
use tower_http::{
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tower::{ServiceBuilder, Service};
use hyper::{Body, Request, Response, Method};
use std::convert::Infallible;
use tokio::fs;

async fn upload_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::POST => {
            // 处理文件上传
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            
            // 保存文件
            let filename = format!("upload_{}.bin", uuid::Uuid::new_v4());
            fs::write(&filename, body).await.unwrap();
            
            Ok(Response::new(Body::from(format!("File saved as {}", filename))))
        }
        _ => {
            Ok(Response::builder()
                .status(405)
                .body(Body::from("Method Not Allowed"))
                .unwrap())
        }
    }
}

// 构建上传服务
let upload_service = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB limit
    .service(tower::service_fn(upload_handler));
```

## 高级用法

### 自定义中间件

```rust
use tower_http::classify::{ClassifyResponse, ClassifiedResponse};
use tower::{Layer, Service};
use http::{Request, Response};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

// 自定义分类器
#[derive(Clone)]
struct CustomClassifier;

impl<B> ClassifyResponse<B> for CustomClassifier {
    type FailureClass = &'static str;
    type ClassifyEos = &'static str;
    
    fn classify_response<E>(
        self,
        res: &Result<Response<B>, E>,
    ) -> ClassifiedResponse<Self::FailureClass, Self::ClassifyEos> {
        match res {
            Ok(response) => {
                if response.status().is_server_error() {
                    ClassifiedResponse::Ready(Err("server_error"))
                } else if response.status().is_client_error() {
                    ClassifiedResponse::Ready(Err("client_error"))
                } else {
                    ClassifiedResponse::Ready(Ok("success"))
                }
            }
            Err(_) => ClassifiedResponse::Ready(Err("error")),
        }
    }
    
    fn classify_error<E>(self, error: &E) -> Self::FailureClass {
        "error"
    }
}

// 使用自定义分类器
let trace = TraceLayer::new_for_http()
    .with_classifier(CustomClassifier);
```

### 条件中间件

```rust
use tower_http::conditional::ConditionalLayer;
use tower::{ServiceBuilder, Service};

// 根据条件应用中间件
let should_compress = std::env::var("ENABLE_COMPRESSION").is_ok();

let service = ServiceBuilder::new()
    .layer(ConditionalLayer::new(
        CompressionLayer::new(),
        should_compress,
    ))
    .service(my_service);
```

### 请求ID中间件

```rust
use tower_http::request_id::{RequestIdLayer, MakeRequestUuid};
use tower::{ServiceBuilder, Service};

// 为每个请求生成唯一ID
let request_id = RequestIdLayer::new(
    http::header::HeaderName::from_static("x-request-id"),
    MakeRequestUuid,
);

let service = ServiceBuilder::new()
    .layer(request_id)
    .service(my_service);
```

### 健康检查

```rust
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tower::{ServiceBuilder, Service};
use http::header::AUTHORIZATION;

// 简单的健康检查端点
async fn health_check(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("OK")))
}

// 需要认证的端点
let auth_layer = ValidateRequestHeaderLayer::bearer("secret-token");

let protected_service = ServiceBuilder::new()
    .layer(auth_layer)
    .service(tower::service_fn(protected_handler));

// 路由设置
let app = Router::new()
    .route("/health", get(health_check))
    .route("/protected", get(protected_service));
```

## 性能优化

### 1. 缓存优化

```rust
use tower_http::set_header::SetResponseHeaderLayer;
use http::header::{CACHE_CONTROL, ETAG};

// 设置缓存头
let cache_layer = SetResponseHeaderLayer::overriding(
    CACHE_CONTROL,
    HeaderValue::from_static("public, max-age=3600")
);

let service = ServiceBuilder::new()
    .layer(cache_layer)
    .service(my_service);
```

### 2. 预压缩静态文件

```rust
use tower_http::services::ServeDir;

// 使用预压缩的静态文件
let serve_dir = ServeDir::new("static")
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_deflate();
```

### 3. 连接复用

```rust
use tower_http::add_extension::AddExtensionLayer;
use hyper::client::HttpConnector;

// 使用连接池
let connector = HttpConnector::new();
let client = hyper::Client::builder()
    .pool_idle_timeout(Duration::from_secs(30))
    .build(connector);

let service = ServiceBuilder::new()
    .layer(AddExtensionLayer::new(client))
    .service(my_service);
```

## 监控和可观测性

### 1. 指标收集

```rust
use tower_http::metrics::InFlightRequestsLayer;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// 跟踪并发请求数
let in_flight_requests = Arc::new(AtomicUsize::new(0));

let metrics_layer = InFlightRequestsLayer::new(
    in_flight_requests.clone(),
    |count| {
        println!("Current in-flight requests: {}", count);
    }
);

let service = ServiceBuilder::new()
    .layer(metrics_layer)
    .service(my_service);
```

### 2. 自定义指标

```rust
use tower_http::classify::ClassifyResponse;
use tower::{Layer, Service};
use std::time::Instant;

// 自定义指标收集中间件
#[derive(Clone)]
struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;
    
    fn layer(&self, service: S) -> Self::Service {
        MetricsService { inner: service }
    }
}

struct MetricsService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for MetricsService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, req: Request<B>) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        
        // 记录请求开始
        println!("Request started: {} {}", method, path);
        
        let future = self.inner.call(req);
        
        // 这里可以添加响应时间统计
        // 实际实现需要使用 pin-project 或类似的工具
        
        future
    }
}
```

## 常见问题和解决方案

### 1. CORS 问题

```rust
use tower_http::cors::{CorsLayer, Any};

// 解决 CORS 问题
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_credentials(true);
```

### 2. 大文件上传

```rust
use tower_http::limit::RequestBodyLimitLayer;
use tower::{ServiceBuilder, Service};

// 处理大文件上传
let service = ServiceBuilder::new()
    .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024)) // 100MB
    .timeout(Duration::from_secs(300)) // 5 minutes
    .service(upload_handler);
```

### 3. 静态文件缓存

```rust
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

// 优化静态文件缓存
let serve_dir = ServeDir::new("static")
    .precompressed_gzip()
    .call_fallback_on_method_not_allowed(true);

let service = ServiceBuilder::new()
    .layer(SetResponseHeaderLayer::overriding(
        http::header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000")
    ))
    .service(serve_dir);
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
tower-http = { version = "0.6.6", features = ["full"] }
tower = "0.5"
hyper = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
http = "1.0"
```

## 特性标志

```toml
[dependencies]
tower-http = { version = "0.6.6", features = [
    "cors",           # CORS 支持
    "compression",    # 响应压缩
    "trace",          # 请求追踪
    "timeout",        # 超时处理
    "limit",          # 请求限制
    "fs",             # 文件服务
    "auth",           # 身份验证
    "metrics",        # 指标收集
    "sensitive-headers", # 敏感头部处理
] }
```

## 总结

Tower-HTTP 提供了构建现代 HTTP 服务所需的所有中间件组件。通过组合不同的中间件，你可以快速构建功能丰富、性能优异的 HTTP 服务。

主要优势：
- 🔧 丰富的中间件生态系统
- 🚀 高性能异步处理
- 🛡️ 内置安全功能
- 📊 完整的可观测性支持
- 🔄 灵活的配置选项
- 📝 详细的文档和示例

Tower-HTTP 是构建 Rust HTTP 服务的最佳选择之一。