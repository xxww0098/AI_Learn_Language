# Tokio 1.46.1 - Rust 异步运行时完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [运行时管理](#运行时管理)
- [异步 I/O](#异步-io)
- [并发控制](#并发控制)
- [网络编程](#网络编程)
- [文件操作](#文件操作)
- [定时器和延迟](#定时器和延迟)
- [进程和信号](#进程和信号)
- [性能调优](#性能调优)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)
- [故障排除](#故障排除)

## 概述

Tokio 是 Rust 中最受欢迎的异步运行时库，为构建高性能、高并发的网络应用提供了完整的异步编程框架。

### 核心特性
- **异步运行时**: 提供完整的异步任务调度和执行环境
- **非阻塞 I/O**: 高效的网络和文件 I/O 操作
- **工作窃取调度**: 智能的多线程任务调度算法
- **零成本抽象**: 编译时优化，运行时高效
- **生态丰富**: 与众多异步库良好集成

### 版本信息
- **当前版本**: 1.46.1
- **发布时间**: 2025-07-04
- **下载次数**: 346,895,233+
- **许可证**: MIT

## 快速开始

### 安装配置

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tokio = { version = "1.46.1", features = ["full"] }
```

### 基本示例

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始异步任务");
    
    // 并发执行多个任务
    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        println!("任务1完成");
    };
    
    let task2 = async {
        sleep(Duration::from_millis(200)).await;
        println!("任务2完成");
    };
    
    // 等待所有任务完成
    tokio::join!(task1, task2);
    
    println!("所有任务完成");
}
```

### 功能特性选择

```toml
[dependencies]
# 最小功能集
tokio = { version = "1.46.1", features = ["rt", "macros"] }

# 常用功能
tokio = { version = "1.46.1", features = [
    "rt-multi-thread",  # 多线程运行时
    "macros",          # 宏支持
    "net",             # 网络功能
    "io-util",         # I/O 工具
    "time",            # 时间功能
    "sync",            # 同步原语
    "fs",              # 文件系统
] }

# 完整功能
tokio = { version = "1.46.1", features = ["full"] }
```

## 核心概念

### 异步函数和 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 异步函数
async fn async_function() -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    42
}

// 手动实现 Future
struct MyFuture {
    completed: bool,
}

impl Future for MyFuture {
    type Output = String;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("完成".to_string())
        } else {
            self.completed = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

### 任务和线程

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // 创建异步任务
    let handle = task::spawn(async {
        println!("在任务中执行");
        "任务结果"
    });
    
    // 等待任务完成
    let result = handle.await.unwrap();
    println!("任务结果: {}", result);
    
    // 阻塞任务（适用于CPU密集型工作）
    let handle = task::spawn_blocking(|| {
        // 模拟CPU密集型工作
        std::thread::sleep(Duration::from_millis(100));
        "阻塞任务结果"
    });
    
    let result = handle.await.unwrap();
    println!("阻塞任务结果: {}", result);
}
```

### 错误处理

```rust
use tokio::time::{sleep, Duration};

async fn may_fail() -> Result<String, &'static str> {
    sleep(Duration::from_millis(100)).await;
    Err("模拟错误")
}

#[tokio::main]
async fn main() {
    match may_fail().await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // 使用 ? 操作符
    async fn handle_error() -> Result<(), Box<dyn std::error::Error>> {
        let _result = may_fail().await?;
        Ok(())
    }
    
    if let Err(e) = handle_error().await {
        println!("处理错误: {}", e);
    }
}
```

## 运行时管理

### 自定义运行时

```rust
use tokio::runtime::Runtime;

fn main() {
    // 创建单线程运行时
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("在单线程运行时中执行");
    });
    
    // 创建多线程运行时
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("在多线程运行时中执行");
    });
}
```

### 运行时配置

```rust
use tokio::runtime::Builder;

fn create_custom_runtime() -> Result<Runtime, Box<dyn std::error::Error>> {
    let rt = Builder::new_multi_thread()
        .worker_threads(8)                    // 工作线程数
        .thread_name("my-worker")             // 线程名称
        .thread_stack_size(3 * 1024 * 1024)  // 线程栈大小
        .enable_all()                         // 启用所有功能
        .build()?;
    
    Ok(rt)
}
```

### 运行时句柄

```rust
use tokio::runtime::Handle;

async fn use_runtime_handle() {
    // 获取当前运行时的句柄
    let handle = Handle::current();
    
    // 在另一个线程中使用运行时
    std::thread::spawn(move || {
        handle.block_on(async {
            println!("在另一个线程中使用运行时");
        });
    });
}
```

## 异步 I/O

### 基本 I/O 操作

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件
    let mut file = File::open("example.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("文件内容: {}", contents);
    
    // 写入文件
    let mut file = File::create("output.txt").await?;
    file.write_all(b"Hello, Tokio!").await?;
    file.flush().await?;
    
    Ok(())
}
```

### 缓冲 I/O

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::fs::File;

async fn buffered_io() -> Result<(), Box<dyn std::error::Error>> {
    // 缓冲读取
    let file = File::open("large_file.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        println!("读取行: {}", line);
    }
    
    // 缓冲写入
    let file = File::create("output.txt").await?;
    let mut writer = BufWriter::new(file);
    
    for i in 0..1000 {
        writer.write_all(format!("行 {}\n", i).as_bytes()).await?;
    }
    writer.flush().await?;
    
    Ok(())
}
```

### 流处理

```rust
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

async fn stream_processing() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.txt").await?;
    let reader = BufReader::new(file);
    let lines = LinesStream::new(reader.lines());
    
    // 流式处理
    let mut stream = lines
        .map(|line| line.unwrap().to_uppercase())
        .filter(|line| line.contains("IMPORTANT"));
    
    while let Some(line) = stream.next().await {
        println!("处理行: {}", line);
    }
    
    Ok(())
}
```

## 并发控制

### 信号量

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3)); // 最多3个并发任务
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let semaphore = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("任务 {} 开始执行", i);
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("任务 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 互斥锁

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let mut lock = data.lock().await;
            *lock += 1;
            println!("任务 {} 将数据更新为 {}", i, *lock);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("最终值: {}", *data.lock().await);
}
```

### 读写锁

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // 并发读取
    for i in 0..5 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let read_guard = data.read().await;
            println!("读取器 {} 看到数据: {:?}", i, *read_guard);
        });
        handles.push(handle);
    }
    
    // 写入
    let data_write = data.clone();
    let write_handle = tokio::spawn(async move {
        let mut write_guard = data_write.write().await;
        write_guard.push(6);
        println!("写入器添加了新元素");
    });
    handles.push(write_handle);
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 通道通信

```rust
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    // 多生产者单消费者通道
    let (tx, mut rx) = mpsc::channel::<i32>(32);
    
    // 生产者
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    
    // 消费者
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("接收到: {}", value);
        }
    });
    
    tokio::join!(producer, consumer);
    
    // 一次性通道
    let (tx, rx) = oneshot::channel::<String>();
    
    let sender = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        tx.send("Hello from oneshot!".to_string()).unwrap();
    });
    
    let receiver = tokio::spawn(async move {
        let message = rx.await.unwrap();
        println!("收到一次性消息: {}", message);
    });
    
    tokio::join!(sender, receiver);
}
```

## 网络编程

### TCP 服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("新连接来自: {}", addr);
        
        // 为每个连接创建一个任务
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => break, // 连接关闭
            Ok(n) => {
                let data = &buffer[..n];
                println!("收到数据: {:?}", String::from_utf8_lossy(data));
                
                // 回显数据
                if let Err(e) = socket.write_all(data).await {
                    eprintln!("写入失败: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("读取失败: {}", e);
                break;
            }
        }
    }
}
```

### TCP 客户端

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("已连接到服务器");
    
    // 发送数据
    stream.write_all(b"Hello, Server!").await?;
    
    // 读取响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("服务器响应: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}
```

### UDP 套接字

```rust
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("UDP 服务器监听 127.0.0.1:8080");
    
    let mut buf = [0; 1024];
    
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("收到来自 {} 的数据: {}", addr, String::from_utf8_lossy(&buf[..len]));
        
        // 回显数据
        socket.send_to(&buf[..len], &addr).await?;
    }
}
```

### HTTP 客户端示例

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn simple_http_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("httpbin.org:80").await?;
    
    let request = "GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes()).await?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    
    println!("HTTP 响应:\n{}", response);
    Ok(())
}
```

## 文件操作

### 异步文件操作

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件
    let content = fs::read_to_string("example.txt").await?;
    println!("文件内容: {}", content);
    
    // 写入文件
    fs::write("output.txt", "Hello, Tokio!").await?;
    
    // 追加内容
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .await?;
    
    file.write_all(b"新的日志条目\n").await?;
    
    // 创建目录
    fs::create_dir_all("nested/directories").await?;
    
    // 列出目录内容
    let mut entries = fs::read_dir(".").await?;
    while let Some(entry) = entries.next_entry().await? {
        println!("目录项: {:?}", entry.path());
    }
    
    Ok(())
}
```

### 文件监视

```rust
use tokio::fs;
use std::path::Path;

async fn watch_file_changes(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_modified = fs::metadata(path).await?.modified()?;
    
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        if let Ok(metadata) = fs::metadata(path).await {
            let modified = metadata.modified()?;
            if modified > last_modified {
                println!("文件 {:?} 已被修改", path);
                last_modified = modified;
            }
        }
    }
}
```

## 定时器和延迟

### 基本定时器

```rust
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    println!("开始时间: {:?}", Instant::now());
    
    // 简单延迟
    sleep(Duration::from_millis(100)).await;
    println!("100ms 后");
    
    // 精确定时
    let start = Instant::now();
    sleep(Duration::from_millis(500)).await;
    println!("实际延迟: {:?}", start.elapsed());
}
```

### 间隔定时器

```rust
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut interval = interval(Duration::from_millis(100));
    
    for i in 0..10 {
        interval.tick().await;
        println!("定时器触发 {}", i);
    }
}
```

### 超时控制

```rust
use tokio::time::{timeout, Duration};

async fn long_running_task() -> Result<String, &'static str> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok("任务完成".to_string())
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(1), long_running_task()).await {
        Ok(result) => println!("任务结果: {:?}", result),
        Err(_) => println!("任务超时"),
    }
}
```

### 高级定时器

```rust
use tokio::time::{sleep_until, Instant, Duration};

#[tokio::main]
async fn main() {
    // 在特定时间点唤醒
    let target_time = Instant::now() + Duration::from_secs(5);
    sleep_until(target_time).await;
    println!("在目标时间唤醒");
    
    // 周期性任务
    let mut next_run = Instant::now();
    
    for i in 0..5 {
        next_run += Duration::from_secs(1);
        sleep_until(next_run).await;
        println!("周期性任务 {}", i);
    }
}
```

## 进程和信号

### 进程管理

```rust
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 执行简单命令
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .await?;
    
    println!("退出状态: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // 交互式进程
    let mut child = Command::new("cat")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin.write_all(b"Hello, process!\n").await?;
        drop(stdin); // 关闭stdin
    }
    
    let output = child.wait_with_output().await?;
    println!("进程输出: {}", String::from_utf8_lossy(&output.stdout));
    
    Ok(())
}
```

### 信号处理

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 监听 Ctrl+C
    let ctrl_c = signal::ctrl_c();
    
    tokio::select! {
        _ = ctrl_c => {
            println!("收到 Ctrl+C，正在退出...");
        }
        _ = async {
            for i in 0..100 {
                println!("工作中... {}", i);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        } => {
            println!("工作完成");
        }
    }
    
    Ok(())
}
```

## 性能调优

### 性能监控

```rust
use tokio::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct PerformanceMonitor {
    task_count: Arc<AtomicU64>,
    start_time: Instant,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            task_count: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }
    
    async fn monitor_task<F, R>(&self, task: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        let start = Instant::now();
        self.task_count.fetch_add(1, Ordering::Relaxed);
        
        let result = task.await;
        
        let duration = start.elapsed();
        println!("任务耗时: {:?}", duration);
        
        result
    }
    
    fn print_stats(&self) {
        let total_time = self.start_time.elapsed();
        let total_tasks = self.task_count.load(Ordering::Relaxed);
        
        println!("总任务数: {}", total_tasks);
        println!("总运行时间: {:?}", total_time);
        println!("任务吞吐量: {:.2} tasks/sec", 
                 total_tasks as f64 / total_time.as_secs_f64());
    }
}
```

### 内存优化

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

// 限制并发任务数量以控制内存使用
async fn memory_efficient_processing(items: Vec<String>) {
    let semaphore = Arc::new(Semaphore::new(100)); // 最多100个并发任务
    let mut handles = Vec::new();
    
    for item in items {
        let semaphore = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            // 处理item
            process_item(item).await;
        });
        handles.push(handle);
    }
    
    // 分批等待任务完成
    for chunk in handles.chunks(1000) {
        for handle in chunk {
            handle.await.unwrap();
        }
    }
}

async fn process_item(item: String) {
    // 处理逻辑
    tokio::time::sleep(Duration::from_millis(10)).await;
}
```

### 任务调度优化

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // CPU密集型任务使用专用线程池
    let cpu_intensive_result = task::spawn_blocking(|| {
        // 模拟CPU密集型计算
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    }).await.unwrap();
    
    // I/O密集型任务使用异步
    let io_result = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "I/O 完成"
    }.await;
    
    println!("CPU结果: {}, I/O结果: {}", cpu_intensive_result, io_result);
}
```

## 实战案例

### 聊天服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _) = broadcast::channel(32);
    
    println!("聊天服务器启动在 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let rx = tx.subscribe();
        
        tokio::spawn(async move {
            handle_client(socket, addr, tx, rx).await;
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    
    // 发送欢迎消息
    let welcome = format!("欢迎 {} 加入聊天室!\n", addr);
    let _ = writer.write_all(welcome.as_bytes()).await;
    
    loop {
        tokio::select! {
            // 读取客户端消息
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // 连接关闭
                    Ok(_) => {
                        let message = line.trim().to_string();
                        if !message.is_empty() {
                            let _ = tx.send((message, addr));
                        }
                        line.clear();
                    }
                    Err(_) => break,
                }
            }
            // 广播消息给客户端
            result = rx.recv() => {
                if let Ok((message, sender_addr)) = result {
                    if sender_addr != addr {
                        let formatted = format!("{}: {}\n", sender_addr, message);
                        let _ = writer.write_all(formatted.as_bytes()).await;
                    }
                }
            }
        }
    }
    
    println!("客户端 {} 断开连接", addr);
}
```

### 文件上传服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("文件上传服务器启动在 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("新的上传连接: {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_upload(socket).await {
                eprintln!("上传处理错误: {}", e);
            }
        });
    }
}

async fn handle_upload(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件名长度
    let mut name_len_bytes = [0u8; 4];
    socket.read_exact(&mut name_len_bytes).await?;
    let name_len = u32::from_be_bytes(name_len_bytes) as usize;
    
    // 读取文件名
    let mut name_bytes = vec![0u8; name_len];
    socket.read_exact(&mut name_bytes).await?;
    let filename = String::from_utf8(name_bytes)?;
    
    // 读取文件大小
    let mut size_bytes = [0u8; 8];
    socket.read_exact(&mut size_bytes).await?;
    let file_size = u64::from_be_bytes(size_bytes);
    
    println!("开始接收文件: {}, 大小: {} bytes", filename, file_size);
    
    // 创建文件
    let mut file = File::create(&filename).await?;
    
    // 分块读取和写入
    let mut remaining = file_size;
    let mut buffer = vec![0u8; 8192];
    
    while remaining > 0 {
        let to_read = std::cmp::min(buffer.len(), remaining as usize);
        let bytes_read = socket.read(&mut buffer[..to_read]).await?;
        
        if bytes_read == 0 {
            break;
        }
        
        file.write_all(&buffer[..bytes_read]).await?;
        remaining -= bytes_read as u64;
    }
    
    file.flush().await?;
    
    // 发送确认
    socket.write_all(b"OK").await?;
    
    println!("文件 {} 上传完成", filename);
    Ok(())
}
```

### 代理服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("代理服务器启动在 127.0.0.1:8080");
    
    loop {
        let (client_socket, addr) = listener.accept().await?;
        println!("新的代理连接: {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_proxy(client_socket).await {
                eprintln!("代理处理错误: {}", e);
            }
        });
    }
}

async fn handle_proxy(mut client_socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // 连接到目标服务器
    let mut target_socket = TcpStream::connect("httpbin.org:80").await?;
    
    // 双向数据转发
    let (mut client_reader, mut client_writer) = client_socket.split();
    let (mut target_reader, mut target_writer) = target_socket.split();
    
    let client_to_target = async {
        let mut buffer = [0u8; 4096];
        loop {
            match client_reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if target_writer.write_all(&buffer[..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    };
    
    let target_to_client = async {
        let mut buffer = [0u8; 4096];
        loop {
            match target_reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if client_writer.write_all(&buffer[..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    };
    
    tokio::join!(client_to_target, target_to_client);
    
    Ok(())
}
```

## 最佳实践

### 1. 合理的任务分解

```rust
// 好的做法：将大任务分解为小任务
async fn process_large_dataset(data: Vec<String>) {
    let chunk_size = 1000;
    
    for chunk in data.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec();
        tokio::spawn(async move {
            process_chunk(chunk_vec).await;
        });
    }
}

async fn process_chunk(chunk: Vec<String>) {
    for item in chunk {
        // 处理单个项目
        process_item(item).await;
    }
}
```

### 2. 适当的错误处理

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum AppError {
    NetworkError(String),
    ParseError(String),
    TimeoutError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            AppError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            AppError::TimeoutError => write!(f, "超时错误"),
        }
    }
}

impl Error for AppError {}

async fn robust_operation() -> Result<String, AppError> {
    // 网络请求
    let response = tokio::time::timeout(
        Duration::from_secs(5),
        make_request()
    ).await
    .map_err(|_| AppError::TimeoutError)?
    .map_err(|e| AppError::NetworkError(e.to_string()))?;
    
    // 解析响应
    parse_response(response)
        .map_err(|e| AppError::ParseError(e.to_string()))
}

async fn make_request() -> Result<String, Box<dyn Error>> {
    // 模拟网络请求
    Ok("response data".to_string())
}

fn parse_response(data: String) -> Result<String, Box<dyn Error>> {
    // 模拟解析
    Ok(data)
}
```

### 3. 资源管理

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

struct ResourceManager {
    semaphore: Arc<Semaphore>,
}

impl ResourceManager {
    fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
    
    async fn execute<F, R>(&self, task: F) -> Result<R, Box<dyn std::error::Error>>
    where
        F: std::future::Future<Output = Result<R, Box<dyn std::error::Error>>>,
    {
        let _permit = self.semaphore.acquire().await?;
        task.await
    }
}

// 使用资源管理器
#[tokio::main]
async fn main() {
    let manager = ResourceManager::new(10);
    
    let mut handles = Vec::new();
    
    for i in 0..100 {
        let manager = &manager;
        let handle = tokio::spawn(async move {
            manager.execute(async move {
                // 执行受限制的任务
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok::<_, Box<dyn std::error::Error>>(i)
            }).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        match handle.await {
            Ok(Ok(result)) => println!("任务完成: {}", result),
            Ok(Err(e)) => println!("任务失败: {}", e),
            Err(e) => println!("任务panic: {}", e),
        }
    }
}
```

### 4. 性能监控

```rust
use tokio::time::{Instant, Duration};
use std::sync::atomic::{AtomicU64, Ordering};

struct Metrics {
    request_count: AtomicU64,
    total_duration: AtomicU64,
}

impl Metrics {
    fn new() -> Self {
        Self {
            request_count: AtomicU64::new(0),
            total_duration: AtomicU64::new(0),
        }
    }
    
    async fn measure<F, R>(&self, operation: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        let start = Instant::now();
        let result = operation.await;
        let duration = start.elapsed();
        
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.total_duration.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        
        result
    }
    
    fn print_stats(&self) {
        let count = self.request_count.load(Ordering::Relaxed);
        let total_ms = self.total_duration.load(Ordering::Relaxed);
        
        if count > 0 {
            println!("请求数: {}", count);
            println!("平均响应时间: {}ms", total_ms / count);
        }
    }
}
```

## 故障排除

### 常见问题和解决方案

#### 1. 任务泄漏

```rust
// 问题：忘记等待任务完成
async fn bad_example() {
    tokio::spawn(async {
        // 这个任务可能不会完成
        tokio::time::sleep(Duration::from_secs(10)).await;
    });
    // 函数结束，但任务仍在运行
}

// 解决方案：使用 JoinHandle
async fn good_example() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(10)).await;
    });
    
    // 等待任务完成或设置超时
    match tokio::time::timeout(Duration::from_secs(5), handle).await {
        Ok(Ok(_)) => println!("任务完成"),
        Ok(Err(e)) => println!("任务失败: {}", e),
        Err(_) => println!("任务超时"),
    }
}
```

#### 2. 死锁问题

```rust
// 问题：锁顺序不一致导致死锁
async fn potential_deadlock() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let m1 = mutex1.clone();
    let m2 = mutex2.clone();
    
    let task1 = tokio::spawn(async move {
        let _lock1 = m1.lock().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let _lock2 = m2.lock().await;
    });
    
    let task2 = tokio::spawn(async move {
        let _lock2 = mutex2.lock().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let _lock1 = mutex1.lock().await;
    });
    
    tokio::join!(task1, task2);
}

// 解决方案：使用一致的锁顺序
async fn avoid_deadlock() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let m1 = mutex1.clone();
    let m2 = mutex2.clone();
    
    let task1 = tokio::spawn(async move {
        let _lock1 = m1.lock().await;
        let _lock2 = m2.lock().await;
        // 使用锁
    });
    
    let task2 = tokio::spawn(async move {
        let _lock1 = mutex1.lock().await;
        let _lock2 = mutex2.lock().await;
        // 使用锁
    });
    
    tokio::join!(task1, task2);
}
```

#### 3. 内存泄漏

```rust
// 问题：循环引用导致内存泄漏
struct Node {
    value: i32,
    next: Option<Arc<Mutex<Node>>>,
}

// 解决方案：使用弱引用
use std::sync::Weak;

struct SafeNode {
    value: i32,
    next: Option<Weak<Mutex<SafeNode>>>,
}
```

#### 4. 调试技巧

```rust
// 启用调试输出
use tracing::{info, debug, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    debug!("应用启动");
    
    let result = risky_operation().await;
    
    match result {
        Ok(value) => info!("操作成功: {}", value),
        Err(e) => error!("操作失败: {}", e),
    }
}

async fn risky_operation() -> Result<i32, &'static str> {
    debug!("开始执行危险操作");
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(42)
}
```

## 总结

Tokio 是 Rust 异步编程的核心库，它提供了完整的异步运行时环境。通过本教程，您应该能够：

1. 理解 Tokio 的核心概念和异步编程模型
2. 熟练使用 Tokio 的各种功能模块
3. 构建高性能的网络应用和服务
4. 避免常见的异步编程陷阱
5. 优化应用的性能和资源使用

关键要点：
- 合理选择功能特性以减少编译时间和二进制大小
- 正确处理异步任务的生命周期
- 使用适当的同步原语避免竞态条件
- 监控和调试异步应用的性能
- 遵循最佳实践确保代码质量

Tokio 的强大功能使其成为构建现代 Rust 应用的理想选择，掌握它将大大提升您的异步编程能力。