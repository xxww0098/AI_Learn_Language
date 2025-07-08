# Reqwest 0.12.22 - Rust HTTP 客户端库完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [基本用法](#基本用法)
- [请求方法](#请求方法)
- [请求配置](#请求配置)
- [响应处理](#响应处理)
- [JSON 支持](#json-支持)
- [表单数据](#表单数据)
- [文件上传](#文件上传)
- [认证](#认证)
- [代理设置](#代理设置)
- [超时和重试](#超时和重试)
- [Cookie管理](#cookie管理)
- [TLS/SSL配置](#tlsssl配置)
- [并发请求](#并发请求)
- [中间件](#中间件)
- [错误处理](#错误处理)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Reqwest 是一个现代的、功能丰富的 Rust HTTP 客户端库，提供了简单易用的 API 来发送 HTTP 请求。它基于 hyper 库构建，提供了出色的性能和可靠性。

### 核心特性
- **异步支持**: 基于 tokio 的异步 HTTP 客户端
- **简单易用**: 类似 Python requests 的简洁 API
- **功能丰富**: 支持 JSON、表单、文件上传、认证等
- **高性能**: 基于 hyper 的高效 HTTP 实现
- **可定制**: 灵活的配置选项和中间件支持
- **跨平台**: 支持多种操作系统和架构

### 版本信息
- **当前版本**: 0.12.22
- **发布时间**: 2025-07-01
- **下载次数**: 236,309,913+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
reqwest = { version = "0.12.22", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 基本示例

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 简单的 GET 请求
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);
    
    // POST JSON 请求
    let user = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    
    println!("Response: {}", response.text().await?);
    
    Ok(())
}
```

## 基本用法

### 创建客户端

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用默认配置
    let client = Client::new();
    
    // 使用自定义配置
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("my-app/1.0")
        .build()?;
    
    // 发送请求
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;
    
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    
    Ok(())
}
```

### 便捷方法

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("GET Response: {}", response.text().await?);
    
    // POST 请求
    let response = reqwest::post("https://httpbin.org/post")
        .body("Hello, World!")
        .send()
        .await?;
    println!("POST Response: {}", response.text().await?);
    
    Ok(())
}
```

## 请求方法

### HTTP 方法

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // GET 请求
    let response = client.get("https://httpbin.org/get").send().await?;
    println!("GET: {}", response.status());
    
    // POST 请求
    let response = client.post("https://httpbin.org/post")
        .body("data")
        .send()
        .await?;
    println!("POST: {}", response.status());
    
    // PUT 请求
    let response = client.put("https://httpbin.org/put")
        .body("updated data")
        .send()
        .await?;
    println!("PUT: {}", response.status());
    
    // DELETE 请求
    let response = client.delete("https://httpbin.org/delete").send().await?;
    println!("DELETE: {}", response.status());
    
    // PATCH 请求
    let response = client.patch("https://httpbin.org/patch")
        .body("patch data")
        .send()
        .await?;
    println!("PATCH: {}", response.status());
    
    // HEAD 请求
    let response = client.head("https://httpbin.org/get").send().await?;
    println!("HEAD: {}", response.status());
    
    Ok(())
}
```

### 自定义请求方法

```rust
use reqwest::{Client, Method};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 使用自定义方法
    let response = client
        .request(Method::OPTIONS, "https://httpbin.org/get")
        .send()
        .await?;
    
    println!("OPTIONS: {}", response.status());
    
    // 使用字符串方法
    let response = client
        .request("TRACE".parse::<Method>().unwrap(), "https://httpbin.org/get")
        .send()
        .await?;
    
    println!("TRACE: {}", response.status());
    
    Ok(())
}
```

## 请求配置

### 设置头部

```rust
use reqwest::{Client, header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 设置单个头部
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "my-app/1.0")
        .header("X-Custom-Header", "custom-value")
        .send()
        .await?;
    
    println!("Headers Response: {}", response.text().await?);
    
    // 设置多个头部
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", "Bearer token123".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    
    let response = client
        .post("https://httpbin.org/post")
        .headers(headers)
        .body("test data")
        .send()
        .await?;
    
    println!("Multiple Headers Response: {}", response.text().await?);
    
    Ok(())
}
```

### 查询参数

```rust
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct SearchParams {
    q: String,
    page: u32,
    limit: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 使用字符串参数
    let response = client
        .get("https://httpbin.org/get")
        .query(&[("name", "张三"), ("age", "30")])
        .send()
        .await?;
    
    println!("String Query Response: {}", response.text().await?);
    
    // 使用结构体参数
    let params = SearchParams {
        q: "rust".to_string(),
        page: 1,
        limit: 10,
    };
    
    let response = client
        .get("https://httpbin.org/get")
        .query(&params)
        .send()
        .await?;
    
    println!("Struct Query Response: {}", response.text().await?);
    
    // 组合查询参数
    let response = client
        .get("https://httpbin.org/get")
        .query(&[("base", "param")])
        .query(&[("additional", "param")])
        .send()
        .await?;
    
    println!("Combined Query Response: {}", response.text().await?);
    
    Ok(())
}
```

### 请求体

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 字符串请求体
    let response = client
        .post("https://httpbin.org/post")
        .body("Hello, World!")
        .send()
        .await?;
    
    println!("String Body Response: {}", response.text().await?);
    
    // 字节数组请求体
    let data = b"binary data";
    let response = client
        .post("https://httpbin.org/post")
        .body(data.to_vec())
        .send()
        .await?;
    
    println!("Bytes Body Response: {}", response.text().await?);
    
    // 流式请求体
    use tokio::fs::File;
    use tokio_util::codec::{BytesCodec, FramedRead};
    
    let file = File::open("test.txt").await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    
    let response = client
        .post("https://httpbin.org/post")
        .body(reqwest::Body::wrap_stream(stream))
        .send()
        .await?;
    
    println!("Stream Body Response: {}", response.text().await?);
    
    Ok(())
}
```

## 响应处理

### 响应状态和头部

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://httpbin.org/get").send().await?;
    
    // 状态码
    println!("Status: {}", response.status());
    println!("Status Code: {}", response.status().as_u16());
    println!("Is Success: {}", response.status().is_success());
    println!("Is Client Error: {}", response.status().is_client_error());
    println!("Is Server Error: {}", response.status().is_server_error());
    
    // 响应头
    println!("Headers: {:#?}", response.headers());
    
    if let Some(content_type) = response.headers().get("content-type") {
        println!("Content-Type: {:?}", content_type);
    }
    
    // URL 信息
    println!("URL: {}", response.url());
    
    // 响应体
    let body = response.text().await?;
    println!("Body: {}", body);
    
    Ok(())
}
```

### 不同格式的响应

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    origin: String,
    headers: std::collections::HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 文本响应
    let response = client.get("https://httpbin.org/get").send().await?;
    let text = response.text().await?;
    println!("Text: {}", text);
    
    // JSON 响应
    let response = client.get("https://httpbin.org/json").send().await?;
    let json: serde_json::Value = response.json().await?;
    println!("JSON: {:#?}", json);
    
    // 结构化 JSON 响应
    let response = client.get("https://httpbin.org/get").send().await?;
    let api_response: ApiResponse = response.json().await?;
    println!("Structured JSON: {:#?}", api_response);
    
    // 字节响应
    let response = client.get("https://httpbin.org/bytes/1024").send().await?;
    let bytes = response.bytes().await?;
    println!("Bytes length: {}", bytes.len());
    
    Ok(())
}
```

### 流式响应

```rust
use reqwest::Client;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://httpbin.org/stream/10").send().await?;
    
    // 流式读取响应
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                println!("Received chunk: {} bytes", bytes.len());
                // 处理数据块
            }
            Err(e) => {
                eprintln!("Error reading chunk: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

## JSON 支持

### 发送 JSON 数据

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 创建用户
    let new_user = CreateUserRequest {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 30,
    };
    
    let response = client
        .post("https://httpbin.org/post")
        .json(&new_user)
        .send()
        .await?;
    
    println!("Create User Response: {}", response.text().await?);
    
    // 更新用户
    let update_data = serde_json::json!({
        "name": "张三（已更新）",
        "email": "zhangsan.updated@example.com"
    });
    
    let response = client
        .put("https://httpbin.org/put")
        .json(&update_data)
        .send()
        .await?;
    
    println!("Update User Response: {}", response.text().await?);
    
    Ok(())
}
```

### 接收 JSON 响应

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiUser {
    id: u32,
    name: String,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    data: T,
    status: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 获取 JSON 响应
    let response = client.get("https://jsonplaceholder.typicode.com/users/1").send().await?;
    
    // 检查响应状态
    if response.status().is_success() {
        let user: ApiUser = response.json().await?;
        println!("User: {:#?}", user);
    } else {
        println!("Request failed: {}", response.status());
    }
    
    // 处理可能的错误响应
    let response = client.get("https://jsonplaceholder.typicode.com/users/999").send().await?;
    
    match response.status().as_u16() {
        200 => {
            let user: ApiUser = response.json().await?;
            println!("User found: {:#?}", user);
        }
        404 => {
            println!("User not found");
        }
        _ => {
            println!("Unexpected status: {}", response.status());
        }
    }
    
    Ok(())
}
```

## 表单数据

### 发送表单数据

```rust
use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 使用 HashMap 发送表单数据
    let mut form_data = HashMap::new();
    form_data.insert("username", "zhangsan");
    form_data.insert("password", "password123");
    form_data.insert("email", "zhangsan@example.com");
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&form_data)
        .send()
        .await?;
    
    println!("Form Response: {}", response.text().await?);
    
    // 使用数组发送表单数据
    let form_data = [
        ("name", "张三"),
        ("age", "30"),
        ("city", "北京"),
    ];
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&form_data)
        .send()
        .await?;
    
    println!("Array Form Response: {}", response.text().await?);
    
    Ok(())
}
```

### 多部分表单数据

```rust
use reqwest::{Client, multipart};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 创建多部分表单
    let form = multipart::Form::new()
        .text("username", "zhangsan")
        .text("email", "zhangsan@example.com")
        .text("description", "这是一个测试用户");
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("Multipart Form Response: {}", response.text().await?);
    
    Ok(())
}
```

## 文件上传

### 上传文件

```rust
use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 读取文件内容
    let mut file = File::open("test.txt").await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;
    
    // 创建多部分表单并上传文件
    let form = multipart::Form::new()
        .text("description", "文件上传测试")
        .part("file", 
            multipart::Part::bytes(contents)
                .file_name("test.txt")
                .mime_str("text/plain")?
        );
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("File Upload Response: {}", response.text().await?);
    
    Ok(())
}
```

### 上传多个文件

```rust
use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 读取多个文件
    let mut file1 = File::open("file1.txt").await?;
    let mut contents1 = Vec::new();
    file1.read_to_end(&mut contents1).await?;
    
    let mut file2 = File::open("file2.txt").await?;
    let mut contents2 = Vec::new();
    file2.read_to_end(&mut contents2).await?;
    
    // 创建多部分表单上传多个文件
    let form = multipart::Form::new()
        .text("title", "多文件上传")
        .part("file1", 
            multipart::Part::bytes(contents1)
                .file_name("file1.txt")
                .mime_str("text/plain")?
        )
        .part("file2", 
            multipart::Part::bytes(contents2)
                .file_name("file2.txt")
                .mime_str("text/plain")?
        );
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("Multiple Files Upload Response: {}", response.text().await?);
    
    Ok(())
}
```

### 流式文件上传

```rust
use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 打开文件并创建流
    let file = File::open("large_file.txt").await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    
    // 创建流式上传
    let form = multipart::Form::new()
        .text("description", "大文件流式上传")
        .part("file", 
            multipart::Part::stream(reqwest::Body::wrap_stream(stream))
                .file_name("large_file.txt")
                .mime_str("text/plain")?
        );
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("Stream Upload Response: {}", response.text().await?);
    
    Ok(())
}
```

## 认证

### 基本认证

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 基本认证
    let response = client
        .get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", Some("pass"))
        .send()
        .await?;
    
    println!("Basic Auth Response: {}", response.text().await?);
    
    // 无密码的基本认证
    let response = client
        .get("https://httpbin.org/basic-auth/user/")
        .basic_auth("user", None::<&str>)
        .send()
        .await?;
    
    println!("Basic Auth (no password) Response: {}", response.text().await?);
    
    Ok(())
}
```

### Bearer Token 认证

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Bearer Token 认证
    let token = "your_jwt_token_here";
    let response = client
        .get("https://httpbin.org/bearer")
        .bearer_auth(token)
        .send()
        .await?;
    
    println!("Bearer Auth Response: {}", response.text().await?);
    
    // 手动设置 Authorization 头
    let response = client
        .get("https://httpbin.org/bearer")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    
    println!("Manual Bearer Auth Response: {}", response.text().await?);
    
    Ok(())
}
```

### 自定义认证

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // API Key 认证
    let api_key = "your_api_key_here";
    let response = client
        .get("https://httpbin.org/get")
        .header("X-API-Key", api_key)
        .send()
        .await?;
    
    println!("API Key Auth Response: {}", response.text().await?);
    
    // 自定义认证头
    let response = client
        .get("https://httpbin.org/get")
        .header("X-Auth-Token", "custom_token")
        .header("X-Client-ID", "client_123")
        .send()
        .await?;
    
    println!("Custom Auth Response: {}", response.text().await?);
    
    Ok(())
}
```

## 代理设置

### 配置代理

```rust
use reqwest::{Client, Proxy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // HTTP 代理
    let proxy = Proxy::http("http://proxy.example.com:8080")?;
    let client = Client::builder()
        .proxy(proxy)
        .build()?;
    
    let response = client.get("https://httpbin.org/ip").send().await?;
    println!("HTTP Proxy Response: {}", response.text().await?);
    
    // HTTPS 代理
    let proxy = Proxy::https("https://proxy.example.com:8080")?;
    let client = Client::builder()
        .proxy(proxy)
        .build()?;
    
    let response = client.get("https://httpbin.org/ip").send().await?;
    println!("HTTPS Proxy Response: {}", response.text().await?);
    
    // 所有协议代理
    let proxy = Proxy::all("http://proxy.example.com:8080")?;
    let client = Client::builder()
        .proxy(proxy)
        .build()?;
    
    let response = client.get("https://httpbin.org/ip").send().await?;
    println!("All Protocol Proxy Response: {}", response.text().await?);
    
    Ok(())
}
```

### 带认证的代理

```rust
use reqwest::{Client, Proxy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 带认证的代理
    let proxy = Proxy::http("http://proxy.example.com:8080")?
        .basic_auth("proxy_user", "proxy_pass");
    
    let client = Client::builder()
        .proxy(proxy)
        .build()?;
    
    let response = client.get("https://httpbin.org/ip").send().await?;
    println!("Authenticated Proxy Response: {}", response.text().await?);
    
    Ok(())
}
```

## 超时和重试

### 设置超时

```rust
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 全局超时
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    
    let response = client.get("https://httpbin.org/delay/5").send().await?;
    println!("Global Timeout Response: {}", response.text().await?);
    
    // 连接超时
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()?;
    
    let response = client.get("https://httpbin.org/get").send().await?;
    println!("Connect Timeout Response: {}", response.text().await?);
    
    Ok(())
}
```

### 重试机制

```rust
use reqwest::Client;
use std::time::Duration;

async fn retry_request(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let mut retries = 0;
    
    loop {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response);
                } else if retries < max_retries {
                    retries += 1;
                    println!("请求失败，重试第 {} 次", retries);
                    tokio::time::sleep(Duration::from_secs(2_u64.pow(retries))).await;
                } else {
                    return Ok(response);
                }
            }
            Err(e) => {
                if retries < max_retries {
                    retries += 1;
                    println!("网络错误，重试第 {} 次: {}", retries, e);
                    tokio::time::sleep(Duration::from_secs(2_u64.pow(retries))).await;
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 使用重试机制
    let response = retry_request(&client, "https://httpbin.org/status/500", 3).await?;
    println!("Retry Response: {}", response.status());
    
    Ok(())
}
```

## Cookie管理

### 自动Cookie管理

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启用 Cookie 存储
    let client = Client::builder()
        .cookie_store(true)
        .build()?;
    
    // 第一个请求设置 Cookie
    let response = client
        .get("https://httpbin.org/cookies/set/session_id/abc123")
        .send()
        .await?;
    
    println!("Set Cookie Response: {}", response.text().await?);
    
    // 第二个请求自动发送 Cookie
    let response = client
        .get("https://httpbin.org/cookies")
        .send()
        .await?;
    
    println!("Get Cookie Response: {}", response.text().await?);
    
    Ok(())
}
```

### 手动Cookie管理

```rust
use reqwest::{Client, header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 手动设置 Cookie
    let response = client
        .get("https://httpbin.org/cookies")
        .header(header::COOKIE, "session_id=abc123; user_id=456")
        .send()
        .await?;
    
    println!("Manual Cookie Response: {}", response.text().await?);
    
    // 从响应中获取 Cookie
    let response = client
        .get("https://httpbin.org/cookies/set/new_session/xyz789")
        .send()
        .await?;
    
    if let Some(set_cookie) = response.headers().get(header::SET_COOKIE) {
        println!("Set-Cookie Header: {:?}", set_cookie);
    }
    
    Ok(())
}
```

## TLS/SSL配置

### TLS配置

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 禁用证书验证（仅用于开发）
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get("https://self-signed.badssl.com/").send().await?;
    println!("Invalid Cert Response: {}", response.status());
    
    // 添加自定义根证书
    let cert = std::fs::read("custom_ca.pem")?;
    let cert = reqwest::Certificate::from_pem(&cert)?;
    
    let client = Client::builder()
        .add_root_certificate(cert)
        .build()?;
    
    let response = client.get("https://example.com").send().await?;
    println!("Custom CA Response: {}", response.status());
    
    Ok(())
}
```

### 客户端证书

```rust
use reqwest::{Client, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载客户端证书
    let cert = std::fs::read("client_cert.p12")?;
    let identity = Identity::from_pkcs12_der(&cert, "certificate_password")?;
    
    let client = Client::builder()
        .identity(identity)
        .build()?;
    
    let response = client.get("https://client-cert.example.com").send().await?;
    println!("Client Cert Response: {}", response.status());
    
    Ok(())
}
```

## 并发请求

### 并发发送请求

```rust
use reqwest::Client;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 并发发送多个请求
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
        "https://httpbin.org/delay/3",
    ];
    
    let mut handles = Vec::new();
    
    for url in urls {
        let client = client.clone();
        let handle = task::spawn(async move {
            let response = client.get(url).send().await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(response.text().await?)
        });
        handles.push(handle);
    }
    
    // 等待所有请求完成
    for handle in handles {
        match handle.await? {
            Ok(response) => println!("Response: {}", response.len()),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### 使用 join! 宏

```rust
use reqwest::Client;
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 使用 join! 宏并发执行
    let (resp1, resp2, resp3) = join!(
        client.get("https://httpbin.org/delay/1").send(),
        client.get("https://httpbin.org/delay/2").send(),
        client.get("https://httpbin.org/delay/3").send()
    );
    
    println!("Response 1: {}", resp1?.status());
    println!("Response 2: {}", resp2?.status());
    println!("Response 3: {}", resp3?.status());
    
    Ok(())
}
```

## 中间件

### 请求拦截器

```rust
use reqwest::{Client, Request, Response};
use reqwest::header;

// 自定义中间件
async fn add_auth_header(mut req: Request) -> Result<Request, Box<dyn std::error::Error>> {
    req.headers_mut().insert(
        header::AUTHORIZATION,
        "Bearer your_token_here".parse().unwrap(),
    );
    Ok(req)
}

async fn log_request(req: &Request) {
    println!("Request: {} {}", req.method(), req.url());
}

async fn log_response(resp: &Response) {
    println!("Response: {} {}", resp.status(), resp.url());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 创建请求
    let mut request = client.get("https://httpbin.org/get").build()?;
    
    // 应用中间件
    request = add_auth_header(request).await?;
    
    // 记录请求
    log_request(&request).await;
    
    // 发送请求
    let response = client.execute(request).await?;
    
    // 记录响应
    log_response(&response).await;
    
    println!("Body: {}", response.text().await?);
    
    Ok(())
}
```

### 响应拦截器

```rust
use reqwest::{Client, Response};
use serde_json::Value;

async fn parse_api_response(response: Response) -> Result<Value, Box<dyn std::error::Error>> {
    let status = response.status();
    let text = response.text().await?;
    
    if status.is_success() {
        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    } else {
        Err(format!("API Error: {} - {}", status, text).into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let response = client.get("https://httpbin.org/json").send().await?;
    
    // 使用响应拦截器
    let json = parse_api_response(response).await?;
    println!("Parsed JSON: {:#?}", json);
    
    Ok(())
}
```

## 错误处理

### 错误类型

```rust
use reqwest::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 处理不同类型的错误
    match client.get("https://invalid-url").send().await {
        Ok(response) => {
            println!("Success: {}", response.status());
        }
        Err(e) => {
            if e.is_timeout() {
                println!("请求超时");
            } else if e.is_connect() {
                println!("连接错误");
            } else if e.is_request() {
                println!("请求错误");
            } else {
                println!("其他错误: {}", e);
            }
        }
    }
    
    Ok(())
}
```

### 自定义错误处理

```rust
use reqwest::{Client, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    #[error("HTTP错误: {status}")]
    Http { status: StatusCode },
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API错误: {message}")]
    Api { message: String },
}

async fn safe_request(client: &Client, url: &str) -> Result<serde_json::Value, ApiError> {
    let response = client.get(url).send().await?;
    
    match response.status() {
        StatusCode::OK => {
            let json = response.json().await?;
            Ok(json)
        }
        status => Err(ApiError::Http { status }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    match safe_request(&client, "https://httpbin.org/json").await {
        Ok(json) => println!("Success: {:#?}", json),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
```

## 实战案例

### REST API 客户端

```rust
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
            
        Self {
            client,
            base_url,
            api_key,
        }
    }
    
    async fn get_user(&self, id: u32) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        
        match response.status() {
            StatusCode::OK => {
                let api_response: ApiResponse<User> = response.json().await?;
                api_response.data
                    .ok_or_else(|| "用户数据为空".into())
            }
            StatusCode::NOT_FOUND => Err("用户不存在".into()),
            _ => Err(format!("请求失败: {}", response.status()).into()),
        }
    }
    
    async fn create_user(&self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(user)
            .send()
            .await?;
        
        match response.status() {
            StatusCode::CREATED => {
                let api_response: ApiResponse<User> = response.json().await?;
                api_response.data
                    .ok_or_else(|| "创建用户失败".into())
            }
            _ => Err(format!("创建用户失败: {}", response.status()).into()),
        }
    }
    
    async fn update_user(&self, id: u32, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(user)
            .send()
            .await?;
        
        match response.status() {
            StatusCode::OK => {
                let api_response: ApiResponse<User> = response.json().await?;
                api_response.data
                    .ok_or_else(|| "更新用户失败".into())
            }
            StatusCode::NOT_FOUND => Err("用户不存在".into()),
            _ => Err(format!("更新用户失败: {}", response.status()).into()),
        }
    }
    
    async fn delete_user(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err("用户不存在".into()),
            _ => Err(format!("删除用户失败: {}", response.status()).into()),
        }
    }
    
    async fn list_users(&self, page: u32, limit: u32) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let url = format!("{}/users", self.base_url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("page", page), ("limit", limit)])
            .send()
            .await?;
        
        match response.status() {
            StatusCode::OK => {
                let api_response: ApiResponse<Vec<User>> = response.json().await?;
                api_response.data
                    .ok_or_else(|| "获取用户列表失败".into())
            }
            _ => Err(format!("获取用户列表失败: {}", response.status()).into()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ApiClient::new(
        "https://api.example.com".to_string(),
        "your_api_key_here".to_string(),
    );
    
    // 创建用户
    let new_user = User {
        id: None,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        phone: Some("13800138000".to_string()),
    };
    
    let created_user = api_client.create_user(&new_user).await?;
    println!("创建用户成功: {:#?}", created_user);
    
    // 获取用户
    let user_id = created_user.id.unwrap();
    let user = api_client.get_user(user_id).await?;
    println!("获取用户: {:#?}", user);
    
    // 更新用户
    let mut updated_user = user.clone();
    updated_user.name = "张三（已更新）".to_string();
    let updated_user = api_client.update_user(user_id, &updated_user).await?;
    println!("更新用户: {:#?}", updated_user);
    
    // 获取用户列表
    let users = api_client.list_users(1, 10).await?;
    println!("用户列表: {:#?}", users);
    
    // 删除用户
    api_client.delete_user(user_id).await?;
    println!("删除用户成功");
    
    Ok(())
}
```

### 文件下载器

```rust
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures::StreamExt;

struct FileDownloader {
    client: Client,
}

impl FileDownloader {
    fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap();
            
        Self { client }
    }
    
    async fn download_file(&self, url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("下载失败: {}", response.status()).into());
        }
        
        let total_size = response.content_length().unwrap_or(0);
        println!("文件大小: {} bytes", total_size);
        
        let mut file = File::create(file_path).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            
            if total_size > 0 {
                let progress = (downloaded as f64 / total_size as f64) * 100.0;
                println!("下载进度: {:.2}%", progress);
            }
        }
        
        println!("下载完成: {}", file_path);
        Ok(())
    }
    
    async fn download_multiple_files(&self, urls: Vec<(&str, &str)>) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles = Vec::new();
        
        for (url, file_path) in urls {
            let client = self.client.clone();
            let url = url.to_string();
            let file_path = file_path.to_string();
            
            let handle = tokio::spawn(async move {
                let downloader = FileDownloader { client };
                downloader.download_file(&url, &file_path).await
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await??;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = FileDownloader::new();
    
    // 下载单个文件
    downloader.download_file(
        "https://httpbin.org/bytes/1024",
        "downloaded_file.bin"
    ).await?;
    
    // 下载多个文件
    let files = vec![
        ("https://httpbin.org/bytes/512", "file1.bin"),
        ("https://httpbin.org/bytes/1024", "file2.bin"),
        ("https://httpbin.org/bytes/2048", "file3.bin"),
    ];
    
    downloader.download_multiple_files(files).await?;
    
    Ok(())
}
```

## 最佳实践

### 1. 客户端复用

```rust
// 好的做法：复用客户端
use reqwest::Client;
use std::sync::Arc;

struct ApiService {
    client: Arc<Client>,
}

impl ApiService {
    fn new() -> Self {
        let client = Arc::new(Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap());
            
        Self { client }
    }
    
    async fn make_request(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }
}

// 避免的做法：每次请求都创建新客户端
async fn bad_practice() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new(); // 每次都创建新客户端
    let response = client.get("https://example.com").send().await?;
    Ok(response.text().await?)
}
```

### 2. 错误处理

```rust
use reqwest::{Client, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),
    #[error("HTTP错误: {status}")]
    Http { status: StatusCode },
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),
}

async fn robust_request(client: &Client, url: &str) -> Result<serde_json::Value, ApiError> {
    let response = client.get(url).send().await?;
    
    match response.status() {
        StatusCode::OK => {
            let json = response.json().await?;
            Ok(json)
        }
        status => Err(ApiError::Http { status }),
    }
}
```

### 3. 超时和重试

```rust
use reqwest::Client;
use std::time::Duration;

fn create_robust_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .unwrap()
}

async fn retry_request<F, T>(
    operation: F,
    max_retries: u32,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, Box<dyn std::error::Error>>> + Send>>,
{
    let mut retries = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }
                retries += 1;
                let delay = Duration::from_secs(2_u64.pow(retries - 1));
                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

### 4. 性能优化

```rust
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;

struct OptimizedClient {
    client: Arc<Client>,
    semaphore: Arc<Semaphore>,
}

impl OptimizedClient {
    fn new(max_concurrent_requests: usize) -> Self {
        let client = Arc::new(Client::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .unwrap());
            
        let semaphore = Arc::new(Semaphore::new(max_concurrent_requests));
        
        Self { client, semaphore }
    }
    
    async fn request(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let _permit = self.semaphore.acquire().await?;
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }
}
```

## 总结

Reqwest 是一个功能强大且易于使用的 HTTP 客户端库，提供了构建现代 HTTP 客户端所需的所有功能。通过本教程，您应该能够：

1. 理解 Reqwest 的核心概念和设计原理
2. 掌握各种请求方法和响应处理
3. 实现认证、代理、超时等高级功能
4. 处理 JSON、表单和文件上传
5. 编写健壮的错误处理和重试逻辑

关键要点：
- 复用客户端实例以提高性能
- 合理设置超时和重试机制
- 使用结构化的错误处理
- 充分利用异步特性进行并发请求
- 注意资源管理和连接池配置

Reqwest 的简洁 API 和强大功能使其成为 Rust 生态系统中 HTTP 客户端的首选库。