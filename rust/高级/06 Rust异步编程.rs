// 06 Rust异步编程 - async/await、Future和异步运行时
// 本章介绍Rust的异步编程：async/await语法、Future trait和异步运行时

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

// 注意：本示例需要添加以下依赖到Cargo.toml：
// [dependencies]
// tokio = { version = "1.0", features = ["full"] }
// futures = "0.3"

fn main() {
    println!("=== Rust异步编程示例 ===");
    println!("注意：本代码需要tokio运行时支持");
    
    // 基本async/await示例
    basic_async_examples();
    
    // Future trait示例
    future_trait_examples();
    
    // 异步并发示例
    async_concurrency_examples();
    
    // 实际应用示例
    practical_examples();
}

// 案例1：基本async/await语法
fn basic_async_examples() {
    println!("\n=== 基本Async/Await示例 ===");
    
    // 简单的异步函数
    async fn say_hello() -> String {
        "Hello, async world!".to_string()
    }
    
    // 带延迟的异步函数
    async fn delayed_greeting(name: &str, delay_ms: u64) -> String {
        // 模拟异步延迟
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        format!("Hello, {}! (after {}ms delay)", name, delay_ms)
    }
    
    // 异步计算
    async fn async_computation(n: u32) -> u32 {
        println!("开始计算 {}", n);
        tokio::time::sleep(Duration::from_millis(100)).await;
        let result = n * n;
        println!("计算完成: {} * {} = {}", n, n, result);
        result
    }
    
    // 运行异步代码的示例函数
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    runtime.block_on(async {
        // 基本异步调用
        let greeting = say_hello().await;
        println!("基本异步: {}", greeting);
        
        // 带延迟的异步调用
        let delayed = delayed_greeting("Alice", 500).await;
        println!("延迟异步: {}", delayed);
        
        // 异步计算
        let result = async_computation(5).await;
        println!("异步计算结果: {}", result);
    });
}

// 案例2：自定义Future
struct CountdownFuture {
    count: usize,
}

impl CountdownFuture {
    fn new(count: usize) -> Self {
        CountdownFuture { count }
    }
}

impl Future for CountdownFuture {
    type Output = String;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready("倒计时完成!".to_string())
        } else {
            println!("倒计时: {}", self.count);
            self.count -= 1;
            
            // 唤醒当前任务以便再次轮询
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 定时器Future
struct TimerFuture {
    duration: Duration,
    started: Option<std::time::Instant>,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        TimerFuture {
            duration,
            started: None,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let started = self.started.get_or_insert_with(std::time::Instant::now);
        
        if started.elapsed() >= self.duration {
            Poll::Ready(())
        } else {
            // 在实际实现中，这里应该注册一个真正的定时器
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn future_trait_examples() {
    println!("\n=== Future Trait示例 ===");
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    runtime.block_on(async {
        // 自定义倒计时Future
        let countdown = CountdownFuture::new(3);
        let result = countdown.await;
        println!("倒计时结果: {}", result);
        
        // 自定义定时器Future
        println!("开始定时器...");
        let timer = TimerFuture::new(Duration::from_secs(1));
        timer.await;
        println!("定时器完成!");
    });
}

// 案例3：异步并发
async fn fetch_data(id: u32, delay_ms: u64) -> String {
    println!("开始获取数据 {}", id);
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    println!("完成获取数据 {}", id);
    format!("数据{}", id)
}

async fn process_data(data: String) -> String {
    println!("开始处理: {}", data);
    tokio::time::sleep(Duration::from_millis(200)).await;
    let processed = format!("已处理的{}", data);
    println!("完成处理: {}", processed);
    processed
}

fn async_concurrency_examples() {
    println!("\n=== 异步并发示例 ===");
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    runtime.block_on(async {
        // 顺序执行
        println!("--- 顺序执行 ---");
        let start = std::time::Instant::now();
        
        let data1 = fetch_data(1, 500).await;
        let data2 = fetch_data(2, 300).await;
        let data3 = fetch_data(3, 400).await;
        
        println!("顺序执行耗时: {:?}", start.elapsed());
        println!("结果: {:?}", vec![data1, data2, data3]);
        
        // 并发执行
        println!("\n--- 并发执行 ---");
        let start = std::time::Instant::now();
        
        let (data1, data2, data3) = tokio::join!(
            fetch_data(4, 500),
            fetch_data(5, 300),
            fetch_data(6, 400)
        );
        
        println!("并发执行耗时: {:?}", start.elapsed());
        println!("结果: {:?}", vec![data1, data2, data3]);
        
        // 使用tokio::spawn并发执行
        println!("\n--- 使用spawn并发执行 ---");
        let start = std::time::Instant::now();
        
        let task1 = tokio::spawn(fetch_data(7, 500));
        let task2 = tokio::spawn(fetch_data(8, 300));
        let task3 = tokio::spawn(fetch_data(9, 400));
        
        let results = tokio::try_join!(task1, task2, task3).unwrap();
        
        println!("spawn并发执行耗时: {:?}", start.elapsed());
        println!("结果: {:?}", results);
        
        // 流水线处理
        println!("\n--- 流水线处理 ---");
        let data = fetch_data(10, 300).await;
        let processed = process_data(data).await;
        println!("流水线结果: {}", processed);
    });
}

// 异步HTTP客户端模拟
struct AsyncHttpClient;

impl AsyncHttpClient {
    async fn get(&self, url: &str) -> Result<String, String> {
        println!("发送GET请求到: {}", url);
        
        // 模拟网络延迟
        let delay = match url {
            "https://api.example.com/fast" => 100,
            "https://api.example.com/slow" => 1000,
            "https://api.example.com/error" => return Err("网络错误".to_string()),
            _ => 500,
        };
        
        tokio::time::sleep(Duration::from_millis(delay)).await;
        Ok(format!("来自 {} 的响应数据", url))
    }
    
    async fn post(&self, url: &str, data: &str) -> Result<String, String> {
        println!("发送POST请求到: {} with data: {}", url, data);
        tokio::time::sleep(Duration::from_millis(300)).await;
        Ok(format!("POST响应: 已处理数据 '{}'", data))
    }
}

// 异步数据库模拟
struct AsyncDatabase {
    data: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, String>>>,
}

impl AsyncDatabase {
    fn new() -> Self {
        AsyncDatabase {
            data: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
    
    async fn insert(&self, key: String, value: String) -> Result<(), String> {
        println!("插入数据: {} = {}", key, value);
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let mut data = self.data.lock().await;
        data.insert(key, value);
        Ok(())
    }
    
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        println!("查询数据: {}", key);
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        let data = self.data.lock().await;
        Ok(data.get(key).cloned())
    }
    
    async fn delete(&self, key: &str) -> Result<bool, String> {
        println!("删除数据: {}", key);
        tokio::time::sleep(Duration::from_millis(75)).await;
        
        let mut data = self.data.lock().await;
        Ok(data.remove(key).is_some())
    }
}

// Web服务器模拟
struct AsyncWebServer {
    client: AsyncHttpClient,
    database: AsyncDatabase,
}

impl AsyncWebServer {
    fn new() -> Self {
        AsyncWebServer {
            client: AsyncHttpClient,
            database: AsyncDatabase::new(),
        }
    }
    
    async fn handle_request(&self, path: &str) -> String {
        match path {
            "/api/data" => self.handle_data_request().await,
            "/api/external" => self.handle_external_request().await,
            "/health" => "OK".to_string(),
            _ => "404 Not Found".to_string(),
        }
    }
    
    async fn handle_data_request(&self) -> String {
        // 并发执行数据库操作
        let insert_task = self.database.insert("user1".to_string(), "Alice".to_string());
        let insert_task2 = self.database.insert("user2".to_string(), "Bob".to_string());
        
        tokio::try_join!(insert_task, insert_task2).unwrap();
        
        // 查询数据
        let user1 = self.database.get("user1").await.unwrap();
        let user2 = self.database.get("user2").await.unwrap();
        
        format!("用户数据: {:?}, {:?}", user1, user2)
    }
    
    async fn handle_external_request(&self) -> String {
        // 并发调用多个外部API
        let (fast_result, slow_result) = tokio::join!(
            self.client.get("https://api.example.com/fast"),
            self.client.get("https://api.example.com/slow")
        );
        
        match (fast_result, slow_result) {
            (Ok(fast), Ok(slow)) => format!("快速: {}, 慢速: {}", fast, slow),
            (Ok(fast), Err(_)) => format!("仅快速成功: {}", fast),
            (Err(_), Ok(slow)) => format!("仅慢速成功: {}", slow),
            (Err(_), Err(_)) => "所有请求都失败".to_string(),
        }
    }
}

// 异步任务调度器
struct AsyncTaskScheduler {
    tasks: std::sync::Arc<tokio::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl AsyncTaskScheduler {
    fn new() -> Self {
        AsyncTaskScheduler {
            tasks: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
    
    async fn schedule<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(future);
        let mut tasks = self.tasks.lock().await;
        tasks.push(handle);
    }
    
    async fn wait_all(&self) {
        let mut tasks = self.tasks.lock().await;
        
        while let Some(task) = tasks.pop() {
            if let Err(e) = task.await {
                println!("任务执行失败: {:?}", e);
            }
        }
    }
}

fn practical_examples() {
    println!("\n=== 实际应用示例 ===");
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    runtime.block_on(async {
        // HTTP客户端示例
        println!("--- HTTP客户端示例 ---");
        let client = AsyncHttpClient;
        
        let get_result = client.get("https://api.example.com/fast").await;
        println!("GET结果: {:?}", get_result);
        
        let post_result = client.post("https://api.example.com/data", "test data").await;
        println!("POST结果: {:?}", post_result);
        
        // 数据库示例
        println!("\n--- 数据库示例 ---");
        let db = AsyncDatabase::new();
        
        db.insert("key1".to_string(), "value1".to_string()).await.unwrap();
        db.insert("key2".to_string(), "value2".to_string()).await.unwrap();
        
        if let Ok(Some(value)) = db.get("key1").await {
            println!("查询到的值: {}", value);
        }
        
        let deleted = db.delete("key1").await.unwrap();
        println!("删除结果: {}", deleted);
        
        // Web服务器示例
        println!("\n--- Web服务器示例 ---");
        let server = AsyncWebServer::new();
        
        let response1 = server.handle_request("/api/data").await;
        println!("API响应: {}", response1);
        
        let response2 = server.handle_request("/api/external").await;
        println!("外部API响应: {}", response2);
        
        // 任务调度器示例
        println!("\n--- 任务调度器示例 ---");
        let scheduler = AsyncTaskScheduler::new();
        
        // 调度多个异步任务
        for i in 1..=3 {
            scheduler.schedule(async move {
                println!("任务 {} 开始", i);
                tokio::time::sleep(Duration::from_millis(i * 100)).await;
                println!("任务 {} 完成", i);
            }).await;
        }
        
        println!("等待所有任务完成...");
        scheduler.wait_all().await;
        println!("所有任务已完成");
    });
}

// 异步流处理
async fn stream_processing_example() {
    println!("\n=== 异步流处理示例 ===");
    
    use tokio_stream::{StreamExt, wrappers::IntervalStream};
    
    // 创建一个定时流
    let interval = tokio::time::interval(Duration::from_millis(500));
    let mut stream = IntervalStream::new(interval).take(5);
    
    let mut counter = 0;
    while let Some(_) = stream.next().await {
        counter += 1;
        println!("流事件 {}", counter);
    }
    
    println!("流处理完成");
}

// 错误处理示例
async fn error_handling_example() {
    println!("\n=== 异步错误处理示例 ===");
    
    async fn might_fail(should_fail: bool) -> Result<String, String> {
        if should_fail {
            Err("操作失败".to_string())
        } else {
            Ok("操作成功".to_string())
        }
    }
    
    // 使用?操作符
    async fn chain_operations() -> Result<String, String> {
        let result1 = might_fail(false).await?;
        let result2 = might_fail(false).await?;
        Ok(format!("{} -> {}", result1, result2))
    }
    
    // 错误处理
    match chain_operations().await {
        Ok(result) => println!("成功: {}", result),
        Err(error) => println!("失败: {}", error),
    }
    
    // 使用try_join!处理多个可能失败的操作
    let (result1, result2) = tokio::try_join!(
        might_fail(false),
        might_fail(true)
    );
    
    match (result1, result2) {
        (Ok(r1), Ok(r2)) => println!("都成功: {}, {}", r1, r2),
        _ => println!("至少一个失败"),
    }
}

// 性能监控
struct PerformanceMonitor;

impl PerformanceMonitor {
    async fn monitor_async_operation<F, T>(operation: F) -> T
    where
        F: Future<Output = T>,
    {
        let start = std::time::Instant::now();
        let result = operation.await;
        let duration = start.elapsed();
        println!("异步操作耗时: {:?}", duration);
        result
    }
}

async fn performance_monitoring_example() {
    println!("\n=== 性能监控示例 ===");
    
    // 监控异步操作
    let result = PerformanceMonitor::monitor_async_operation(async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        "操作完成"
    }).await;
    
    println!("监控结果: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_basic_async() {
        async fn add_async(a: i32, b: i32) -> i32 {
            a + b
        }
        
        let result = add_async(2, 3).await;
        assert_eq!(result, 5);
    }
    
    #[tokio::test]
    async fn test_concurrent_execution() {
        let start = std::time::Instant::now();
        
        let (r1, r2, r3) = tokio::join!(
            async { tokio::time::sleep(Duration::from_millis(100)).await; 1 },
            async { tokio::time::sleep(Duration::from_millis(100)).await; 2 },
            async { tokio::time::sleep(Duration::from_millis(100)).await; 3 }
        );
        
        let duration = start.elapsed();
        
        assert_eq!((r1, r2, r3), (1, 2, 3));
        // 并发执行应该比顺序执行快
        assert!(duration < Duration::from_millis(250));
    }
    
    #[tokio::test]
    async fn test_async_database() {
        let db = AsyncDatabase::new();
        
        db.insert("test_key".to_string(), "test_value".to_string()).await.unwrap();
        
        let value = db.get("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));
        
        let deleted = db.delete("test_key").await.unwrap();
        assert!(deleted);
        
        let value = db.get("test_key").await.unwrap();
        assert_eq!(value, None);
    }
    
    #[tokio::test]
    async fn test_custom_future() {
        let countdown = CountdownFuture::new(0);
        let result = countdown.await;
        assert_eq!(result, "倒计时完成!");
    }
    
    #[test]
    fn test_sync_wrapper() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        let result = runtime.block_on(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        });
        
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_examples() {
        basic_async_examples();
        future_trait_examples();
        async_concurrency_examples();
        practical_examples();
    }
}

// 异步编程要点总结：
// 1. async函数返回Future，需要await来执行
// 2. tokio::join!并发执行多个Future
// 3. tokio::spawn创建独立的异步任务
// 4. async/await提供同步代码的编写体验
// 5. Future trait是异步编程的核心抽象
// 6. 异步运行时管理Future的执行
// 7. 异步代码具有传染性，需要全链路支持
// 8. 合理使用并发可以显著提高I/O密集型应用的性能