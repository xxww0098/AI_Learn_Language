# Futures 0.3.31 - Rust 异步编程基础库完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [Future 和 Stream](#future-和-stream)
- [异步工具](#异步工具)
- [执行器](#执行器)
- [流处理](#流处理)
- [组合器](#组合器)
- [错误处理](#错误处理)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)
- [性能优化](#性能优化)

## 概述

Futures 是 Rust 异步编程的基础库，提供了异步编程所需的核心抽象、工具和组合器。它是 Rust 异步生态系统的基石。

### 核心特性
- **零分配**: 高效的内存使用，避免不必要的分配
- **可组合性**: 强大的组合器支持复杂的异步流程
- **迭代器风格**: 类似标准库迭代器的 API 设计
- **无运行时**: 纯库实现，不依赖特定运行时
- **标准兼容**: 与标准库 Future trait 完全兼容

### 版本信息
- **当前版本**: 0.3.31
- **发布时间**: 2024-10-05
- **下载次数**: 301,608,754+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
futures = "0.3.31"

# 或者选择特定功能
futures = { version = "0.3.31", features = ["executor"] }
```

### 基本示例

```rust
use futures::{future, select, stream::StreamExt};

#[tokio::main]
async fn main() {
    // 创建一个简单的 future
    let future1 = future::ready(42);
    let result = future1.await;
    println!("结果: {}", result);
    
    // 创建一个流
    let stream = futures::stream::iter(vec![1, 2, 3, 4, 5]);
    let doubled: Vec<i32> = stream.map(|x| x * 2).collect().await;
    println!("翻倍后: {:?}", doubled);
    
    // 使用 select 宏
    let future_a = future::ready(1);
    let future_b = future::ready(2);
    
    select! {
        result = future_a => println!("future_a 完成: {}", result),
        result = future_b => println!("future_b 完成: {}", result),
    }
}
```

## 核心概念

### Future Trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 自定义 Future 实现
struct CountdownFuture {
    count: usize,
}

impl Future for CountdownFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready(())
        } else {
            self.count -= 1;
            println!("倒计时: {}", self.count);
            
            // 请求再次轮询
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 使用
async fn use_countdown() {
    let countdown = CountdownFuture { count: 5 };
    countdown.await;
    println!("倒计时完成!");
}
```

### Stream Trait

```rust
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

// 自定义 Stream 实现
struct Counter {
    current: usize,
    max: usize,
}

impl Stream for Counter {
    type Item = usize;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

// 使用
async fn use_counter() {
    let counter = Counter { current: 0, max: 5 };
    let results: Vec<usize> = counter.collect().await;
    println!("计数结果: {:?}", results);
}
```

### Sink Trait

```rust
use futures::sink::{Sink, SinkExt};
use std::pin::Pin;
use std::task::{Context, Poll};

// 自定义 Sink 实现
struct VecSink {
    items: Vec<i32>,
}

impl Sink<i32> for VecSink {
    type Error = ();
    
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    
    fn start_send(mut self: Pin<&mut Self>, item: i32) -> Result<(), Self::Error> {
        self.items.push(item);
        Ok(())
    }
    
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

// 使用
async fn use_sink() {
    let mut sink = VecSink { items: Vec::new() };
    
    sink.send(1).await.unwrap();
    sink.send(2).await.unwrap();
    sink.send(3).await.unwrap();
    
    println!("Sink 中的项目: {:?}", sink.items);
}
```

## Future 和 Stream

### 基本 Future 操作

```rust
use futures::future::{self, FutureExt};

async fn future_operations() {
    // 创建立即完成的 future
    let ready_future = future::ready(42);
    let result = ready_future.await;
    println!("立即结果: {}", result);
    
    // 创建永不完成的 future
    // let pending_future = future::pending::<i32>();
    
    // 映射结果
    let mapped = future::ready(10).map(|x| x * 2);
    println!("映射结果: {}", mapped.await);
    
    // 错误处理
    let error_future = future::err::<i32, &str>("错误");
    let handled = error_future.or_else(|_| future::ok(0));
    println!("错误处理结果: {}", handled.await.unwrap());
    
    // 链式操作
    let chained = future::ready(5)
        .then(|x| future::ready(x * 2))
        .then(|x| future::ready(x + 1));
    println!("链式结果: {}", chained.await);
}
```

### 组合多个 Future

```rust
use futures::future::{self, join, join_all, select_all, try_join};

async fn combine_futures() {
    // join - 等待所有 future 完成
    let future1 = future::ready(1);
    let future2 = future::ready(2);
    let future3 = future::ready(3);
    
    let (a, b, c) = join!(future1, future2, future3);
    println!("Join 结果: {}, {}, {}", a, b, c);
    
    // join_all - 等待 vector 中所有 future 完成
    let futures = vec![
        future::ready(1),
        future::ready(2),
        future::ready(3),
    ];
    let results = join_all(futures).await;
    println!("Join all 结果: {:?}", results);
    
    // try_join - 遇到错误时停止
    let success1 = future::ok::<i32, &str>(1);
    let success2 = future::ok::<i32, &str>(2);
    
    match try_join!(success1, success2) {
        Ok((a, b)) => println!("Try join 成功: {}, {}", a, b),
        Err(e) => println!("Try join 失败: {}", e),
    }
    
    // select_all - 等待第一个完成
    let futures = vec![
        future::ready(1),
        future::ready(2),
        future::ready(3),
    ];
    let (result, _index, _remaining) = select_all(futures).await;
    println!("Select all 结果: {}", result);
}
```

### 条件选择

```rust
use futures::{future, select, pin_mut};

async fn conditional_selection() {
    let future1 = future::ready(1);
    let future2 = future::ready(2);
    
    pin_mut!(future1, future2);
    
    select! {
        result = future1 => println!("Future 1 完成: {}", result),
        result = future2 => println!("Future 2 完成: {}", result),
    }
    
    // 带默认情况
    let mut future3 = future::pending::<i32>();
    pin_mut!(future3);
    
    select! {
        result = future3 => println!("Future 3 完成: {}", result),
        default => println!("没有 future 准备好"),
    }
}
```

## 异步工具

### 异步锁

```rust
use futures::lock::{Mutex, MutexGuard};
use std::sync::Arc;

async fn async_locks() {
    let mutex = Arc::new(Mutex::new(0));
    
    // 创建多个任务
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let mutex = mutex.clone();
        let handle = tokio::spawn(async move {
            let mut guard = mutex.lock().await;
            *guard += 1;
            println!("任务 {} 将值更新为 {}", i, *guard);
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("最终值: {}", *mutex.lock().await);
}
```

### 异步通道

```rust
use futures::channel::{mpsc, oneshot};
use futures::{SinkExt, StreamExt};

async fn async_channels() {
    // 多生产者单消费者通道
    let (mut tx, mut rx) = mpsc::channel::<i32>(10);
    
    // 发送者任务
    let sender = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            println!("发送: {}", i);
        }
    });
    
    // 接收者任务
    let receiver = tokio::spawn(async move {
        while let Some(value) = rx.next().await {
            println!("接收: {}", value);
        }
    });
    
    tokio::join!(sender, receiver);
    
    // 一次性通道
    let (tx, rx) = oneshot::channel::<String>();
    
    let sender = tokio::spawn(async move {
        tx.send("Hello!".to_string()).unwrap();
    });
    
    let receiver = tokio::spawn(async move {
        let message = rx.await.unwrap();
        println!("收到消息: {}", message);
    });
    
    tokio::join!(sender, receiver);
}
```

## 执行器

### 基本执行器

```rust
use futures::executor::{block_on, ThreadPool};
use futures::future;

fn basic_executor() {
    // 阻塞执行器
    let result = block_on(future::ready(42));
    println!("阻塞执行结果: {}", result);
    
    // 线程池执行器
    let pool = ThreadPool::new().unwrap();
    let future = future::ready(100);
    let result = block_on(pool.spawn_with_handle(future).unwrap());
    println!("线程池执行结果: {}", result);
}
```

### 本地执行器

```rust
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use futures::future;

fn local_executor() {
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();
    
    // 产生任务
    spawner.spawn_local(async {
        println!("本地任务1执行");
    }).unwrap();
    
    spawner.spawn_local(async {
        println!("本地任务2执行");
    }).unwrap();
    
    // 运行所有任务
    pool.run();
}
```

## 流处理

### 基本流操作

```rust
use futures::stream::{self, StreamExt};

async fn basic_stream_operations() {
    // 从迭代器创建流
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // 映射
    let doubled: Vec<i32> = stream.map(|x| x * 2).collect().await;
    println!("映射结果: {:?}", doubled);
    
    // 过滤
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let evens: Vec<i32> = stream.filter(|x| future::ready(*x % 2 == 0)).collect().await;
    println!("过滤结果: {:?}", evens);
    
    // 折叠
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let sum = stream.fold(0, |acc, x| future::ready(acc + x)).await;
    println!("折叠结果: {}", sum);
    
    // 取前几个
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let first_three: Vec<i32> = stream.take(3).collect().await;
    println!("取前三个: {:?}", first_three);
}
```

### 高级流操作

```rust
use futures::stream::{self, StreamExt, TryStreamExt};

async fn advanced_stream_operations() {
    // 并发映射
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let results: Vec<i32> = stream
        .map(|x| async move { x * 2 })
        .buffer_unordered(2)  // 最多2个并发
        .collect()
        .await;
    println!("并发映射结果: {:?}", results);
    
    // 错误处理
    let stream = stream::iter(vec![
        Ok(1),
        Err("错误"),
        Ok(3),
    ]);
    
    let results: Result<Vec<i32>, &str> = stream.try_collect().await;
    match results {
        Ok(values) => println!("成功: {:?}", values),
        Err(e) => println!("失败: {}", e),
    }
    
    // 流的链接
    let stream1 = stream::iter(vec![1, 2, 3]);
    let stream2 = stream::iter(vec![4, 5, 6]);
    let chained: Vec<i32> = stream1.chain(stream2).collect().await;
    println!("链接结果: {:?}", chained);
    
    // 流的分块
    let stream = stream::iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chunks: Vec<Vec<i32>> = stream.chunks(3).collect().await;
    println!("分块结果: {:?}", chunks);
}
```

### 自定义流生成器

```rust
use futures::stream::{self, Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

// 斐波那契数列流
struct FibonacciStream {
    current: u64,
    next: u64,
}

impl FibonacciStream {
    fn new() -> Self {
        FibonacciStream { current: 0, next: 1 }
    }
}

impl Stream for FibonacciStream {
    type Item = u64;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        
        if current > 1000 {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(current))
        }
    }
}

async fn fibonacci_example() {
    let fib_stream = FibonacciStream::new();
    let numbers: Vec<u64> = fib_stream.collect().await;
    println!("斐波那契数列: {:?}", numbers);
}
```

## 组合器

### Future 组合器

```rust
use futures::future::{self, FutureExt, TryFutureExt};

async fn future_combinators() {
    // map - 转换成功值
    let future = future::ready(42);
    let mapped = future.map(|x| x * 2);
    println!("映射结果: {}", mapped.await);
    
    // map_err - 转换错误值
    let future = future::err::<i32, &str>("原始错误");
    let mapped_err = future.map_err(|e| format!("处理后的错误: {}", e));
    match mapped_err.await {
        Ok(v) => println!("成功: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    // and_then - 链式成功操作
    let future = future::ok::<i32, &str>(5);
    let chained = future.and_then(|x| future::ok(x * 2));
    println!("链式操作结果: {:?}", chained.await);
    
    // or_else - 链式错误处理
    let future = future::err::<i32, &str>("错误");
    let recovered = future.or_else(|_| future::ok(42));
    println!("错误恢复结果: {:?}", recovered.await);
    
    // inspect - 检查值但不改变
    let future = future::ready(100);
    let inspected = future.inspect(|x| println!("检查值: {}", x));
    println!("最终值: {}", inspected.await);
}
```

### Stream 组合器

```rust
use futures::stream::{self, StreamExt, TryStreamExt};

async fn stream_combinators() {
    // enumerate - 添加索引
    let stream = stream::iter(vec!["a", "b", "c"]);
    let enumerated: Vec<(usize, &str)> = stream.enumerate().collect().await;
    println!("枚举结果: {:?}", enumerated);
    
    // zip - 合并两个流
    let stream1 = stream::iter(vec![1, 2, 3]);
    let stream2 = stream::iter(vec!["a", "b", "c"]);
    let zipped: Vec<(i32, &str)> = stream1.zip(stream2).collect().await;
    println!("合并结果: {:?}", zipped);
    
    // scan - 累积状态
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let scanned: Vec<i32> = stream
        .scan(0, |state, x| {
            *state += x;
            future::ready(Some(*state))
        })
        .collect()
        .await;
    println!("扫描结果: {:?}", scanned);
    
    // skip_while - 跳过满足条件的元素
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let skipped: Vec<i32> = stream
        .skip_while(|x| future::ready(*x < 3))
        .collect()
        .await;
    println!("跳过结果: {:?}", skipped);
    
    // take_while - 取满足条件的元素
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let taken: Vec<i32> = stream
        .take_while(|x| future::ready(*x < 4))
        .collect()
        .await;
    println!("取值结果: {:?}", taken);
}
```

## 错误处理

### 错误传播

```rust
use futures::future::{self, TryFutureExt};
use futures::stream::{self, TryStreamExt};

#[derive(Debug)]
enum MyError {
    Network(String),
    Parse(String),
    Database(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Network(msg) => write!(f, "网络错误: {}", msg),
            MyError::Parse(msg) => write!(f, "解析错误: {}", msg),
            MyError::Database(msg) => write!(f, "数据库错误: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}

async fn error_handling() {
    // Future 错误处理
    let future_result = async {
        let data = fetch_data().await?;
        let parsed = parse_data(data)?;
        save_data(parsed).await?;
        Ok::<_, MyError>(())
    };
    
    match future_result.await {
        Ok(_) => println!("操作成功"),
        Err(e) => println!("操作失败: {}", e),
    }
    
    // Stream 错误处理
    let stream = stream::iter(vec![
        Ok("data1"),
        Err(MyError::Network("连接失败".to_string())),
        Ok("data2"),
    ]);
    
    let results: Result<Vec<&str>, MyError> = stream.try_collect().await;
    match results {
        Ok(data) => println!("收集成功: {:?}", data),
        Err(e) => println!("收集失败: {}", e),
    }
}

async fn fetch_data() -> Result<String, MyError> {
    // 模拟网络请求
    future::ready(Ok("raw data".to_string())).await
}

fn parse_data(data: String) -> Result<String, MyError> {
    // 模拟数据解析
    Ok(format!("parsed: {}", data))
}

async fn save_data(data: String) -> Result<(), MyError> {
    // 模拟数据保存
    future::ready(Ok(())).await
}
```

### 错误恢复

```rust
use futures::future::{self, TryFutureExt};

async fn error_recovery() {
    // 使用 or_else 恢复错误
    let recovered = future::err::<i32, &str>("错误")
        .or_else(|_| future::ok(42));
    
    println!("恢复结果: {:?}", recovered.await);
    
    // 使用 map_err 转换错误
    let converted = future::err::<i32, &str>("原始错误")
        .map_err(|e| format!("转换后的错误: {}", e));
    
    match converted.await {
        Ok(v) => println!("成功: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    // 重试机制
    let mut attempts = 0;
    let max_attempts = 3;
    
    loop {
        attempts += 1;
        
        match risky_operation().await {
            Ok(result) => {
                println!("成功: {}", result);
                break;
            }
            Err(e) if attempts < max_attempts => {
                println!("尝试 {} 失败: {}, 重试中...", attempts, e);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                println!("最终失败: {}", e);
                break;
            }
        }
    }
}

async fn risky_operation() -> Result<String, &'static str> {
    // 模拟可能失败的操作
    if rand::random::<f32>() > 0.7 {
        Ok("操作成功".to_string())
    } else {
        Err("操作失败")
    }
}
```

## 实战案例

### 并发网络请求

```rust
use futures::future::join_all;
use futures::stream::{self, StreamExt};

async fn concurrent_requests() {
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
        "https://httpbin.org/delay/3",
    ];
    
    // 方法1: 使用 join_all
    let futures = urls.iter().map(|url| fetch_url(url));
    let results = join_all(futures).await;
    
    for (i, result) in results.iter().enumerate() {
        println!("请求 {} 结果: {:?}", i, result);
    }
    
    // 方法2: 使用 stream 并发处理
    let results: Vec<String> = stream::iter(urls)
        .map(|url| fetch_url(url))
        .buffer_unordered(3)  // 最多3个并发
        .collect()
        .await;
    
    println!("并发请求结果: {:?}", results);
}

async fn fetch_url(url: &str) -> String {
    // 模拟网络请求
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    format!("来自 {} 的响应", url)
}
```

### 数据处理管道

```rust
use futures::stream::{self, StreamExt, TryStreamExt};

async fn data_processing_pipeline() {
    // 创建数据流
    let data_stream = stream::iter(vec![
        "data1", "data2", "data3", "data4", "data5",
        "invalid", "data6", "data7", "data8", "data9"
    ]);
    
    // 构建处理管道
    let results = data_stream
        .map(|data| validate_data(data))
        .try_filter_map(|data| async move {
            if data.len() > 5 {
                Ok(Some(data))
            } else {
                Ok(None)
            }
        })
        .and_then(|data| transform_data(data))
        .try_collect::<Vec<_>>()
        .await;
    
    match results {
        Ok(processed) => println!("处理结果: {:?}", processed),
        Err(e) => println!("处理错误: {}", e),
    }
}

fn validate_data(data: &str) -> Result<String, &'static str> {
    if data == "invalid" {
        Err("无效数据")
    } else {
        Ok(data.to_string())
    }
}

async fn transform_data(data: String) -> Result<String, &'static str> {
    // 模拟异步转换
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    Ok(format!("transformed_{}", data))
}
```

### 生产者-消费者模式

```rust
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};

async fn producer_consumer() {
    let (mut tx, mut rx) = mpsc::channel::<i32>(10);
    
    // 生产者
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            if let Err(e) = tx.send(i).await {
                println!("发送失败: {}", e);
                break;
            }
            println!("生产: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        
        // 关闭发送端
        tx.close().await.unwrap();
    });
    
    // 消费者
    let consumer = tokio::spawn(async move {
        let mut processed = 0;
        
        while let Some(value) = rx.next().await {
            println!("消费: {}", value);
            processed += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        println!("总共处理: {} 个项目", processed);
    });
    
    // 等待完成
    tokio::join!(producer, consumer);
}
```

## 最佳实践

### 1. 合理使用组合器

```rust
use futures::future::{self, FutureExt};
use futures::stream::{self, StreamExt};

async fn combinator_best_practices() {
    // 好的做法：链式操作
    let result = future::ready(10)
        .map(|x| x * 2)
        .map(|x| x + 5)
        .map(|x| x.to_string())
        .await;
    
    println!("链式结果: {}", result);
    
    // 好的做法：流的流水线处理
    let processed: Vec<String> = stream::iter(vec![1, 2, 3, 4, 5])
        .filter(|x| future::ready(*x % 2 == 0))
        .map(|x| x * 2)
        .map(|x| format!("处理后: {}", x))
        .collect()
        .await;
    
    println!("流处理结果: {:?}", processed);
}
```

### 2. 错误处理策略

```rust
use futures::future::{self, TryFutureExt};

async fn error_handling_strategies() {
    // 策略1: 快速失败
    let quick_fail = async {
        let a = operation_a().await?;
        let b = operation_b(a).await?;
        operation_c(b).await
    };
    
    match quick_fail.await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("失败: {}", e),
    }
    
    // 策略2: 错误累积
    let results = futures::future::join_all(vec![
        operation_a().map_err(|e| format!("A失败: {}", e)),
        operation_b(0).map_err(|e| format!("B失败: {}", e)),
        operation_c(0).map_err(|e| format!("C失败: {}", e)),
    ]).await;
    
    for result in results {
        match result {
            Ok(value) => println!("成功: {}", value),
            Err(e) => println!("错误: {}", e),
        }
    }
}

async fn operation_a() -> Result<i32, &'static str> {
    future::ready(Ok(1)).await
}

async fn operation_b(input: i32) -> Result<i32, &'static str> {
    future::ready(Ok(input + 1)).await
}

async fn operation_c(input: i32) -> Result<i32, &'static str> {
    future::ready(Ok(input * 2)).await
}
```

### 3. 性能优化

```rust
use futures::stream::{self, StreamExt};

async fn performance_optimization() {
    // 使用 buffer_unordered 进行并发处理
    let results: Vec<String> = stream::iter(0..10)
        .map(|i| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            format!("处理 {}", i)
        })
        .buffer_unordered(3)  // 最多3个并发
        .collect()
        .await;
    
    println!("并发处理结果: {:?}", results);
    
    // 使用 chunks 批量处理
    let batched: Vec<Vec<i32>> = stream::iter(0..20)
        .chunks(5)
        .collect()
        .await;
    
    for (i, batch) in batched.iter().enumerate() {
        println!("批次 {}: {:?}", i, batch);
    }
}
```

## 性能优化

### 内存优化

```rust
use futures::stream::{self, StreamExt};

async fn memory_optimization() {
    // 使用迭代器而不是收集全部结果
    let mut count = 0;
    let mut stream = stream::iter(0..1_000_000);
    
    while let Some(value) = stream.next().await {
        if value % 2 == 0 {
            count += 1;
        }
        
        // 只处理，不存储
        if count >= 1000 {
            break;
        }
    }
    
    println!("处理了 {} 个偶数", count);
}
```

### 延迟计算

```rust
use futures::future::{self, FutureExt};

async fn lazy_computation() {
    // 创建延迟计算的 future
    let lazy_future = future::lazy(|_| {
        println!("开始计算");
        expensive_computation()
    });
    
    // 只有在 await 时才会执行
    let result = lazy_future.await;
    println!("计算结果: {}", result);
}

fn expensive_computation() -> i32 {
    // 模拟昂贵的计算
    (0..1000).sum()
}
```

### 缓存和记忆化

```rust
use futures::future::{self, FutureExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type Cache = Arc<Mutex<HashMap<i32, String>>>;

async fn cached_computation(cache: Cache, key: i32) -> String {
    // 检查缓存
    {
        let cache_guard = cache.lock().await;
        if let Some(value) = cache_guard.get(&key) {
            return value.clone();
        }
    }
    
    // 计算新值
    let result = expensive_string_computation(key).await;
    
    // 存入缓存
    {
        let mut cache_guard = cache.lock().await;
        cache_guard.insert(key, result.clone());
    }
    
    result
}

async fn expensive_string_computation(key: i32) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    format!("计算结果: {}", key)
}

async fn caching_example() {
    let cache = Arc::new(Mutex::new(HashMap::new()));
    
    // 第一次调用会计算
    let result1 = cached_computation(cache.clone(), 42).await;
    println!("第一次: {}", result1);
    
    // 第二次调用会使用缓存
    let result2 = cached_computation(cache.clone(), 42).await;
    println!("第二次: {}", result2);
}
```

## 总结

Futures 库是 Rust 异步编程的基础，提供了强大而灵活的工具来构建复杂的异步应用程序。通过本教程，您应该能够：

1. 理解 Future 和 Stream 的核心概念
2. 熟练使用各种组合器构建复杂的异步流程
3. 正确处理异步编程中的错误
4. 优化异步代码的性能和内存使用
5. 构建生产级的异步应用

关键要点：
- 使用组合器而不是手动实现 Future
- 合理处理错误，避免 panic
- 注意内存使用，避免不必要的分配
- 利用并发能力提高性能
- 遵循异步编程的最佳实践

掌握 Futures 库将为您的 Rust 异步编程之旅奠定坚实的基础。