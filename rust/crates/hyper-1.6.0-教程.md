# Hyper 1.6.0 - Rust HTTP 库完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [HTTP 客户端](#http-客户端)
- [HTTP 服务器](#http-服务器)
- [请求和响应](#请求和响应)
- [异步处理](#异步处理)
- [连接管理](#连接管理)
- [TLS/SSL支持](#tlsssl支持)
- [中间件](#中间件)
- [错误处理](#错误处理)
- [性能优化](#性能优化)
- [与其他库集成](#与其他库集成)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Hyper 是一个现代的、高性能的 HTTP 库，提供了构建 HTTP 客户端和服务器的底层基础设施。它是 Rust 生态系统中许多高级 HTTP 框架的基础，包括 reqwest、warp、axum 等。

### 核心特性
- **高性能**: 零拷贝解析，内存高效
- **异步优先**: 基于 tokio 的异步 I/O
- **HTTP/1.1 和 HTTP/2**: 完整的协议支持
- **类型安全**: 强类型的 HTTP 抽象
- **低级别**: 提供精细的控制能力
- **可扩展**: 灵活的架构设计

### 版本信息
- **当前版本**: 1.6.0
- **发布时间**: 2025-01-28
- **下载次数**: 335,119,989+
- **许可证**: MIT

## 快速开始

### 安装配置

```toml
[dependencies]
hyper = { version = "1.6.0", features = ["full"] }
hyper-util = "0.1"
tokio = { version = "1", features = ["full"] }
http-body-util = "0.1"
```

### 简单的客户端示例

```rust
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::Full;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建客户端
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    // 创建请求
    let req = Request::builder()
        .uri("http://httpbin.org/get")
        .body(Full::new(Bytes::from("")))?;
    
    // 发送请求
    let resp = client.request(req).await?;
    
    println!("Status: {}", resp.status());
    println!("Headers: {:#?}", resp.headers());
    
    // 读取响应体
    let body = resp.into_body();
    let body_bytes = hyper::body::to_bytes(body).await?;
    println!("Body: {}", String::from_utf8_lossy(&body_bytes));
    
    Ok(())
}
```

### 简单的服务器示例

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

## 核心概念

### HTTP 抽象

```rust
use hyper::{Method, Uri, Version, HeaderMap, StatusCode};
use hyper::header::{HeaderName, HeaderValue};

fn understand_http_types() {
    // HTTP 方法
    let get = Method::GET;
    let post = Method::POST;
    let custom = Method::from_bytes(b"CUSTOM").unwrap();
    
    // URI
    let uri: Uri = "https://example.com/path?query=value".parse().unwrap();
    println!("Scheme: {:?}", uri.scheme());
    println!("Host: {:?}", uri.host());
    println!("Path: {}", uri.path());
    println!("Query: {:?}", uri.query());
    
    // HTTP 版本
    let version = Version::HTTP_11;
    println!("Version: {:?}", version);
    
    // 头部
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    headers.insert("x-custom-header", HeaderValue::from_static("custom-value"));
    
    // 状态码
    let status = StatusCode::OK;
    println!("Status: {} {}", status.as_u16(), status.canonical_reason().unwrap_or(""));
}
```

### 请求和响应结构

```rust
use hyper::{Request, Response, Body};
use hyper::body::Bytes;
use http_body_util::Full;

fn request_response_structure() {
    // 创建请求
    let request = Request::builder()
        .method("GET")
        .uri("https://example.com")
        .header("user-agent", "my-app/1.0")
        .body(Full::new(Bytes::from("request body")))
        .unwrap();
    
    // 访问请求组件
    println!("Method: {}", request.method());
    println!("URI: {}", request.uri());
    println!("Headers: {:?}", request.headers());
    
    // 创建响应
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Full::new(Bytes::from("Hello, World!")))
        .unwrap();
    
    // 访问响应组件
    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());
}
```

## HTTP 客户端

### 基本客户端使用

```rust
use hyper::body::Bytes;
use hyper::{Request, Method};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{Full, BodyExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    // GET 请求
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://httpbin.org/get")
        .body(Full::new(Bytes::from("")))?;
    
    let resp = client.request(req).await?;
    println!("GET Response: {}", resp.status());
    
    // POST 请求
    let json_data = r#"{"name": "张三", "age": 30}"#;
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(json_data)))?;
    
    let resp = client.request(req).await?;
    println!("POST Response: {}", resp.status());
    
    // 读取响应体
    let body = resp.into_body();
    let body_bytes = body.collect().await?.to_bytes();
    println!("Response body: {}", String::from_utf8_lossy(&body_bytes));
    
    Ok(())
}
```

### 自定义客户端配置

```rust
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use hyper_util::rt::TokioExecutor;
use http_body_util::Full;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建自定义连接器
    let mut connector = HttpConnector::new();
    connector.set_connect_timeout(Some(Duration::from_secs(10)));
    connector.set_happy_eyeballs_timeout(Some(Duration::from_secs(1)));
    
    // 创建客户端
    let client = Client::builder(TokioExecutor::new())
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build(connector);
    
    // 发送请求
    let req = Request::builder()
        .uri("http://httpbin.org/delay/2")
        .body(Full::new(Bytes::from("")))?;
    
    let resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    
    Ok(())
}
```

### 并发请求

```rust
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::Full;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    // 创建多个请求
    let urls = vec![
        "http://httpbin.org/delay/1",
        "http://httpbin.org/delay/2",
        "http://httpbin.org/delay/3",
    ];
    
    let requests = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let req = Request::builder()
                .uri(url)
                .body(Full::new(Bytes::from("")))
                .unwrap();
            
            let resp = client.request(req).await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(resp.status())
        }
    });
    
    // 并发执行所有请求
    let results = join_all(requests).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(status) => println!("Request {}: {}", i + 1, status),
            Err(e) => println!("Request {} failed: {}", i + 1, e),
        }
    }
    
    Ok(())
}
```

## HTTP 服务器

### 基本服务器实现

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use tokio::net::TcpListener;

async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
        }
        (&Method::GET, "/health") => {
            Ok(Response::new(Full::new(Bytes::from("OK"))))
        }
        (&Method::POST, "/echo") => {
            let body = req.into_body();
            let body_bytes = body.collect().await?.to_bytes();
            Ok(Response::new(Full::new(body_bytes)))
        }
        _ => {
            let mut resp = Response::new(Full::new(Bytes::from("Not Found")));
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("服务器监听地址: {}", addr);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("新连接: {}", addr);
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(handle_request))
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

### 状态共享服务器

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type Counter = Arc<Mutex<HashMap<String, u64>>>;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    counter: Counter,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
        }
        (&Method::GET, "/count") => {
            let path = req.uri().path().to_string();
            let mut counts = counter.lock().unwrap();
            let count = counts.entry(path).or_insert(0);
            *count += 1;
            let response = format!("访问次数: {}", count);
            Ok(Response::new(Full::new(Bytes::from(response))))
        }
        (&Method::GET, "/stats") => {
            let counts = counter.lock().unwrap();
            let stats = serde_json::to_string(&*counts).unwrap();
            Ok(Response::new(Full::new(Bytes::from(stats))))
        }
        _ => {
            let mut resp = Response::new(Full::new(Bytes::from("Not Found")));
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let counter: Counter = Arc::new(Mutex::new(HashMap::new()));
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let counter = counter.clone();
        
        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                handle_request(req, counter.clone())
            });
            
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

### HTTP/2 服务器

```rust
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls};
use std::sync::Arc;

async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Full::new(Bytes::from("Hello, HTTP/2!"))))
        }
        (&Method::GET, "/headers") => {
            let headers_json = serde_json::to_string(req.headers()).unwrap();
            Ok(Response::new(Full::new(Bytes::from(headers_json))))
        }
        _ => {
            let mut resp = Response::new(Full::new(Bytes::from("Not Found")));
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 加载 TLS 证书
    let cert_file = std::fs::read("cert.pem")?;
    let key_file = std::fs::read("key.pem")?;
    
    let cert = rustls::Certificate(cert_file);
    let key = rustls::PrivateKey(key_file);
    
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;
    
    let acceptor = TlsAcceptor::from(Arc::new(config));
    
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("HTTP/2 服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        
        tokio::task::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(tls_stream) => tls_stream,
                Err(err) => {
                    eprintln!("TLS 握手失败: {}", err);
                    return;
                }
            };
            
            if let Err(err) = http2::Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(tls_stream, service_fn(handle_request))
                .await
            {
                eprintln!("处理 HTTP/2 连接时出错: {}", err);
            }
        });
    }
}
```

## 请求和响应

### 处理请求体

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

async fn handle_json_request(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // 检查 Content-Type
    let content_type = req.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if content_type != "application/json" {
        let mut resp = Response::new(Full::new(Bytes::from("Expected JSON")));
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(resp);
    }
    
    // 读取请求体
    let body = req.into_body();
    let body_bytes = body.collect().await?.to_bytes();
    
    // 解析 JSON
    match serde_json::from_slice::<User>(&body_bytes) {
        Ok(user) => {
            println!("接收到用户: {:?}", user);
            
            // 返回响应
            let response_data = serde_json::json!({
                "message": "用户创建成功",
                "user": user
            });
            
            let response = Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(response_data.to_string())))
                .unwrap();
            
            Ok(response)
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "error": "JSON 解析失败",
                "message": e.to_string()
            });
            
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(error_response.to_string())))
                .unwrap();
            
            Ok(response)
        }
    }
}
```

### 流式响应

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::StreamBody;
use tokio::time::{interval, Duration};
use futures::stream::{self, StreamExt};

async fn handle_stream_request(_req: Request<hyper::body::Incoming>) -> Result<Response<StreamBody<impl futures::Stream<Item = Result<hyper::body::Frame<Bytes>, std::io::Error>>>>, hyper::Error> {
    // 创建流式数据
    let stream = stream::unfold(0, |count| async move {
        if count < 10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let data = format!("chunk {}\n", count);
            Some((Ok(hyper::body::Frame::data(Bytes::from(data))), count + 1))
        } else {
            None
        }
    });
    
    let body = StreamBody::new(stream);
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .header("cache-control", "no-cache")
        .body(body)
        .unwrap();
    
    Ok(response)
}
```

### 文件上传处理

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn handle_file_upload(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // 检查 Content-Type
    let content_type = req.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if !content_type.starts_with("multipart/form-data") {
        let mut resp = Response::new(Full::new(Bytes::from("Expected multipart/form-data")));
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(resp);
    }
    
    // 读取请求体
    let body = req.into_body();
    let body_bytes = body.collect().await?.to_bytes();
    
    // 保存文件
    let filename = format!("uploaded_{}.bin", chrono::Utc::now().timestamp());
    let mut file = File::create(&filename).await
        .map_err(|e| hyper::Error::from(e))?;
    
    file.write_all(&body_bytes).await
        .map_err(|e| hyper::Error::from(e))?;
    
    let response = serde_json::json!({
        "message": "文件上传成功",
        "filename": filename,
        "size": body_bytes.len()
    });
    
    let resp = Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(response.to_string())))
        .unwrap();
    
    Ok(resp)
}
```

## 异步处理

### 异步服务

```rust
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
struct AsyncService {
    counter: Arc<AtomicUsize>,
}

impl AsyncService {
    fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl Service<Request<hyper::body::Incoming>> for AsyncService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    
    fn call(&mut self, req: Request<hyper::body::Incoming>) -> Self::Future {
        let counter = self.counter.clone();
        
        Box::pin(async move {
            // 模拟异步处理
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            
            let response = format!("请求 #{} 处理完成", count);
            Ok(Response::new(Full::new(Bytes::from(response))))
        })
    }
}

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use hyper::server::conn::http1;
    use tokio::net::TcpListener;
    
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("异步服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let service = AsyncService::new();
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

### 任务调度

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

#[derive(Debug)]
struct Task {
    id: u64,
    data: String,
    response_tx: tokio::sync::oneshot::Sender<String>,
}

async fn task_processor(mut rx: mpsc::Receiver<Task>) {
    while let Some(task) = rx.recv().await {
        println!("处理任务: {}", task.id);
        
        // 模拟处理时间
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        let result = format!("任务 {} 处理完成: {}", task.id, task.data);
        
        if let Err(_) = task.response_tx.send(result) {
            println!("发送响应失败，任务 {}", task.id);
        }
    }
}

async fn handle_task_request(
    req: Request<hyper::body::Incoming>,
    task_tx: mpsc::Sender<Task>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    use http_body_util::BodyExt;
    
    let body = req.into_body();
    let body_bytes = body.collect().await?.to_bytes();
    let data = String::from_utf8_lossy(&body_bytes);
    
    let task_id = rand::random::<u64>();
    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
    
    let task = Task {
        id: task_id,
        data: data.to_string(),
        response_tx,
    };
    
    // 发送任务
    if let Err(_) = task_tx.send(task).await {
        let mut resp = Response::new(Full::new(Bytes::from("任务队列已满")));
        *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
        return Ok(resp);
    }
    
    // 等待任务完成
    match timeout(Duration::from_secs(5), response_rx).await {
        Ok(Ok(result)) => {
            Ok(Response::new(Full::new(Bytes::from(result))))
        }
        Ok(Err(_)) => {
            let mut resp = Response::new(Full::new(Bytes::from("任务处理失败")));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(resp)
        }
        Err(_) => {
            let mut resp = Response::new(Full::new(Bytes::from("任务处理超时")));
            *resp.status_mut() = StatusCode::REQUEST_TIMEOUT;
            Ok(resp)
        }
    }
}
```

## 连接管理

### 连接池

```rust
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use hyper_util::rt::TokioExecutor;
use http_body_util::Full;
use std::time::Duration;

struct ConnectionPool {
    client: Client<HttpConnector, Full<Bytes>>,
}

impl ConnectionPool {
    fn new() -> Self {
        let mut connector = HttpConnector::new();
        connector.set_connect_timeout(Some(Duration::from_secs(10)));
        connector.set_happy_eyeballs_timeout(Some(Duration::from_secs(1)));
        
        let client = Client::builder(TokioExecutor::new())
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .http2_only(false)
            .build(connector);
        
        Self { client }
    }
    
    async fn make_request(&self, uri: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let req = Request::builder()
            .uri(uri)
            .body(Full::new(Bytes::from("")))?;
        
        let resp = self.client.request(req).await?;
        let body = resp.into_body();
        
        use http_body_util::BodyExt;
        let body_bytes = body.collect().await?.to_bytes();
        Ok(String::from_utf8_lossy(&body_bytes).to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = ConnectionPool::new();
    
    // 发送多个请求，复用连接
    let urls = vec![
        "http://httpbin.org/get",
        "http://httpbin.org/user-agent",
        "http://httpbin.org/headers",
    ];
    
    for url in urls {
        let response = pool.make_request(url).await?;
        println!("Response length: {}", response.len());
    }
    
    Ok(())
}
```

### Keep-Alive 配置

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::net::TcpListener;
use std::time::Duration;

async fn handle_keepalive(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let response = format!("Path: {}", req.uri().path());
    
    let resp = Response::builder()
        .header("connection", "keep-alive")
        .header("keep-alive", "timeout=5, max=100")
        .body(Full::new(Bytes::from(response)))
        .unwrap();
    
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Keep-Alive 服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            let mut builder = http1::Builder::new();
            builder.keep_alive(true);
            builder.max_buf_size(4096);
            
            if let Err(err) = builder
                .serve_connection(stream, service_fn(handle_keepalive))
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

## TLS/SSL支持

### HTTPS 客户端

```rust
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use hyper_rustls::HttpsConnectorBuilder;
use http_body_util::Full;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建 HTTPS 连接器
    let https_connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();
    
    let client = Client::builder(TokioExecutor::new())
        .build(https_connector);
    
    // 发送 HTTPS 请求
    let req = Request::builder()
        .uri("https://httpbin.org/get")
        .body(Full::new(Bytes::from("")))?;
    
    let resp = client.request(req).await?;
    println!("HTTPS Response: {}", resp.status());
    
    Ok(())
}
```

### HTTPS 服务器

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls};
use std::sync::Arc;

async fn handle_https_request(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from("Hello, HTTPS!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 加载证书和私钥
    let cert_file = std::fs::read("cert.pem")?;
    let key_file = std::fs::read("key.pem")?;
    
    let cert = rustls::Certificate(cert_file);
    let key = rustls::PrivateKey(key_file);
    
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;
    
    let acceptor = TlsAcceptor::from(Arc::new(config));
    
    let addr = "127.0.0.1:3443";
    let listener = TcpListener::bind(addr).await?;
    println!("HTTPS 服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        
        tokio::task::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(tls_stream) => tls_stream,
                Err(err) => {
                    eprintln!("TLS 握手失败: {}", err);
                    return;
                }
            };
            
            if let Err(err) = http1::Builder::new()
                .serve_connection(tls_stream, service_fn(handle_https_request))
                .await
            {
                eprintln!("处理 HTTPS 连接时出错: {}", err);
            }
        });
    }
}
```

## 中间件

### 日志中间件

```rust
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Full;
use std::time::Instant;

async fn logging_middleware<F, Fut>(
    req: Request<hyper::body::Incoming>,
    handler: F,
) -> Result<Response<Full<Bytes>>, hyper::Error>
where
    F: FnOnce(Request<hyper::body::Incoming>) -> Fut,
    Fut: std::future::Future<Output = Result<Response<Full<Bytes>>, hyper::Error>>,
{
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    println!("请求开始: {} {}", method, path);
    
    let response = handler(req).await?;
    
    let duration = start.elapsed();
    println!("请求完成: {} {} - {}ms", method, path, duration.as_millis());
    
    Ok(response)
}

async fn simple_handler(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn handle_with_logging(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    logging_middleware(req, simple_handler).await
}
```

### 认证中间件

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;

async fn auth_middleware<F, Fut>(
    req: Request<hyper::body::Incoming>,
    handler: F,
) -> Result<Response<Full<Bytes>>, hyper::Error>
where
    F: FnOnce(Request<hyper::body::Incoming>) -> Fut,
    Fut: std::future::Future<Output = Result<Response<Full<Bytes>>, hyper::Error>>,
{
    // 检查认证头
    let auth_header = req.headers().get("authorization");
    
    if let Some(auth) = auth_header {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if token == "valid-token" {
                    return handler(req).await;
                }
            }
        }
    }
    
    // 认证失败
    let mut resp = Response::new(Full::new(Bytes::from("Unauthorized")));
    *resp.status_mut() = StatusCode::UNAUTHORIZED;
    Ok(resp)
}

async fn protected_handler(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from("Protected resource"))))
}

async fn handle_with_auth(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    auth_middleware(req, protected_handler).await
}
```

## 错误处理

### 自定义错误类型

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("验证失败: {0}")]
    Validation(String),
    #[error("数据库错误: {0}")]
    Database(String),
    #[error("网络错误: {0}")]
    Network(String),
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl From<AppError> for Response<Full<Bytes>> {
    fn from(err: AppError) -> Self {
        let (status, message) = match err {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Network(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            AppError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        let error_response = serde_json::json!({
            "error": true,
            "message": message,
            "status": status.as_u16()
        });
        
        Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(Full::new(Bytes::from(error_response.to_string())))
            .unwrap()
    }
}

async fn handle_with_error_handling(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // 模拟可能出错的操作
    let result = match req.uri().path() {
        "/validation-error" => Err(AppError::Validation("输入参数无效".to_string())),
        "/database-error" => Err(AppError::Database("连接数据库失败".to_string())),
        "/network-error" => Err(AppError::Network("网络超时".to_string())),
        _ => Ok("操作成功".to_string()),
    };
    
    match result {
        Ok(data) => Ok(Response::new(Full::new(Bytes::from(data)))),
        Err(err) => Ok(err.into()),
    }
}
```

### 错误恢复

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct ErrorRecovery {
    error_count: Arc<AtomicUsize>,
    max_errors: usize,
}

impl ErrorRecovery {
    fn new(max_errors: usize) -> Self {
        Self {
            error_count: Arc::new(AtomicUsize::new(0)),
            max_errors,
        }
    }
    
    async fn handle_with_recovery<F, Fut>(
        &self,
        req: Request<hyper::body::Incoming>,
        handler: F,
    ) -> Result<Response<Full<Bytes>>, hyper::Error>
    where
        F: FnOnce(Request<hyper::body::Incoming>) -> Fut,
        Fut: std::future::Future<Output = Result<Response<Full<Bytes>>, hyper::Error>>,
    {
        let current_errors = self.error_count.load(Ordering::SeqCst);
        
        if current_errors >= self.max_errors {
            let mut resp = Response::new(Full::new(Bytes::from("服务暂时不可用")));
            *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
            return Ok(resp);
        }
        
        match handler(req).await {
            Ok(response) => {
                // 成功时重置错误计数
                self.error_count.store(0, Ordering::SeqCst);
                Ok(response)
            }
            Err(err) => {
                // 增加错误计数
                self.error_count.fetch_add(1, Ordering::SeqCst);
                
                // 返回错误响应
                let mut resp = Response::new(Full::new(Bytes::from("处理请求时发生错误")));
                *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                Ok(resp)
            }
        }
    }
}

async fn unreliable_handler(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // 模拟不稳定的处理
    if rand::random::<f64>() < 0.3 {
        return Err(hyper::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "随机错误",
        )));
    }
    
    Ok(Response::new(Full::new(Bytes::from("处理成功"))))
}
```

## 实战案例

### RESTful API 服务器

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

type UserStore = Arc<Mutex<HashMap<u32, User>>>;

async fn handle_users_api(
    req: Request<hyper::body::Incoming>,
    store: UserStore,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/users") => {
            let users = store.lock().unwrap();
            let user_list: Vec<User> = users.values().cloned().collect();
            let response = serde_json::to_string(&user_list).unwrap();
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(response)))
                .unwrap())
        }
        (&Method::POST, "/users") => {
            let body = req.into_body();
            let body_bytes = body.collect().await?.to_bytes();
            
            match serde_json::from_slice::<User>(&body_bytes) {
                Ok(mut user) => {
                    let mut users = store.lock().unwrap();
                    let id = users.len() as u32 + 1;
                    user.id = id;
                    users.insert(id, user.clone());
                    
                    let response = serde_json::to_string(&user).unwrap();
                    
                    Ok(Response::builder()
                        .status(StatusCode::CREATED)
                        .header("content-type", "application/json")
                        .body(Full::new(Bytes::from(response)))
                        .unwrap())
                }
                Err(_) => {
                    let mut resp = Response::new(Full::new(Bytes::from("Invalid JSON")));
                    *resp.status_mut() = StatusCode::BAD_REQUEST;
                    Ok(resp)
                }
            }
        }
        (&Method::GET, path) if path.starts_with("/users/") => {
            if let Ok(id) = path.trim_start_matches("/users/").parse::<u32>() {
                let users = store.lock().unwrap();
                match users.get(&id) {
                    Some(user) => {
                        let response = serde_json::to_string(user).unwrap();
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("content-type", "application/json")
                            .body(Full::new(Bytes::from(response)))
                            .unwrap())
                    }
                    None => {
                        let mut resp = Response::new(Full::new(Bytes::from("User not found")));
                        *resp.status_mut() = StatusCode::NOT_FOUND;
                        Ok(resp)
                    }
                }
            } else {
                let mut resp = Response::new(Full::new(Bytes::from("Invalid user ID")));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                Ok(resp)
            }
        }
        _ => {
            let mut resp = Response::new(Full::new(Bytes::from("Not Found")));
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_store: UserStore = Arc::new(Mutex::new(HashMap::new()));
    
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("RESTful API 服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let store = user_store.clone();
        
        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                handle_users_api(req, store.clone())
            });
            
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

### 代理服务器

```rust
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode, Uri};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{Full, BodyExt};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;

async fn proxy_handler(
    mut req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    // 修改请求 URI
    let uri = req.uri().clone();
    let target_uri = format!("http://httpbin.org{}", uri.path_and_query().unwrap());
    
    *req.uri_mut() = target_uri.parse().unwrap();
    
    // 移除 Host 头部
    req.headers_mut().remove("host");
    
    // 转发请求
    match client.request(req).await {
        Ok(resp) => {
            let (parts, body) = resp.into_parts();
            let body_bytes = body.collect().await?.to_bytes();
            
            let mut proxy_resp = Response::from_parts(parts, Full::new(body_bytes));
            
            // 添加代理头部
            proxy_resp.headers_mut().insert("x-proxied-by", "hyper-proxy".parse().unwrap());
            
            Ok(proxy_resp)
        }
        Err(e) => {
            eprintln!("代理请求失败: {}", e);
            let mut resp = Response::new(Full::new(Bytes::from("代理请求失败")));
            *resp.status_mut() = StatusCode::BAD_GATEWAY;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("代理服务器监听地址: {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(proxy_handler))
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

## 最佳实践

### 1. 性能优化

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::net::TcpListener;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    
    // 设置 TCP 选项
    listener.set_ttl(64)?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        
        // 设置 TCP 选项
        stream.set_nodelay(true)?;
        
        tokio::task::spawn(async move {
            let mut builder = http1::Builder::new();
            
            // 优化 HTTP/1.1 设置
            builder.keep_alive(true);
            builder.http1_writev(true);
            builder.http1_title_case_headers(true);
            builder.max_buf_size(8192);
            
            if let Err(err) = builder
                .serve_connection(stream, service_fn(optimized_handler))
                .await
            {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}

async fn optimized_handler(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // 使用预分配的响应
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .header("content-length", "13")
        .body(Full::new(Bytes::from("Hello, World!")))
        .unwrap();
    
    Ok(response)
}
```

### 2. 错误处理

```rust
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use http_body_util::Full;
use std::fmt;

#[derive(Debug)]
enum AppError {
    BadRequest(String),
    Unauthorized,
    NotFound,
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::NotFound => write!(f, "Not Found"),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl From<AppError> for Response<Full<Bytes>> {
    fn from(err: AppError) -> Self {
        let (status, message) = match err {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        Response::builder()
            .status(status)
            .header("content-type", "text/plain")
            .body(Full::new(Bytes::from(message)))
            .unwrap()
    }
}
```

### 3. 资源管理

```rust
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Full;
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use std::sync::Arc;

struct Server {
    semaphore: Arc<Semaphore>,
}

impl Server {
    fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_connections)),
        }
    }
    
    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _permit = self.semaphore.acquire().await?;
        
        http1::Builder::new()
            .serve_connection(stream, service_fn(handle_request))
            .await?;
        
        Ok(())
    }
}

async fn handle_request(
    _req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = Server::new(1000); // 最大1000个并发连接
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        
        tokio::task::spawn(async move {
            if let Err(err) = server.handle_connection(stream).await {
                eprintln!("处理连接时出错: {}", err);
            }
        });
    }
}
```

## 总结

Hyper 是一个功能强大的底层 HTTP 库，提供了构建高性能 HTTP 客户端和服务器的基础设施。通过本教程，您应该能够：

1. 理解 Hyper 的核心概念和架构
2. 构建 HTTP 客户端和服务器
3. 处理异步请求和响应
4. 实现连接管理和性能优化
5. 集成 TLS/SSL 支持
6. 编写中间件和错误处理

关键要点：
- Hyper 提供了底层的 HTTP 抽象
- 注重性能和内存效率
- 完全异步的设计
- 类型安全的 API
- 可扩展的架构

Hyper 的设计理念是提供一个安全、快速、正确的 HTTP 实现，它是许多高级框架的基础，理解 Hyper 有助于更好地使用这些框架。