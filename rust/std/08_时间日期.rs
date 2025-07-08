// 08_时间日期.rs
// Rust标准库时间和日期处理详解

/*
Rust标准库的时间处理主要通过std::time模块提供：

核心类型：
- Instant：单调递增的时间点，用于测量时间间隔
- SystemTime：系统时间，可以转换为Unix时间戳
- Duration：时间间隔，表示两个时间点之间的差值

主要特点：
- 单调性：Instant不受系统时间调整影响
- 精度：纳秒级精度，但实际精度依赖于平台
- 线程安全：所有时间类型都是线程安全的
- 跨平台：在不同操作系统上提供一致的API

常用操作：
- 时间测量：性能基准测试
- 超时控制：网络请求、线程等待
- 时间戳：日志记录、数据存储
- 定时器：周期性任务执行
- 时间格式化：虽然标准库功能有限，但可以转换为Unix时间戳

注意事项：
- Instant适用于测量时间间隔
- SystemTime适用于获取当前时间
- 复杂的日期时间操作建议使用chrono库
- 时区处理需要外部库支持
*/

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    println!("=== Rust标准库时间和日期处理 ===");
    
    // 1. Duration 时间间隔
    println!("\n1. Duration 时间间隔：");
    duration_examples();
    
    // 2. Instant 瞬时时间
    println!("\n2. Instant 瞬时时间：");
    instant_examples();
    
    // 3. SystemTime 系统时间
    println!("\n3. SystemTime 系统时间：");
    system_time_examples();
    
    // 4. 时间测量和基准测试
    println!("\n4. 时间测量和基准测试：");
    time_measurement();
    
    // 5. 超时控制
    println!("\n5. 超时控制：");
    timeout_control();
    
    // 6. 定时器和调度
    println!("\n6. 定时器和调度：");
    timer_and_scheduling();
    
    // 7. 时间格式化和解析
    println!("\n7. 时间格式化和解析：");
    time_formatting();
    
    // 8. 性能分析工具
    println!("\n8. 性能分析工具：");
    performance_analysis();
    
    // 9. 实际应用示例
    println!("\n9. 实际应用示例：");
    practical_examples();
    
    // 10. 最佳实践
    println!("\n10. 最佳实践：");
    best_practices();
    
    println!("\n=== 时间日期学习完成 ===");
}

// Duration 时间间隔示例
fn duration_examples() {
    // 创建Duration
    let d1 = Duration::from_secs(5);
    let d2 = Duration::from_millis(1500);
    let d3 = Duration::from_micros(1_000_000);
    let d4 = Duration::from_nanos(1_000_000_000);
    
    println!("不同单位的Duration:");
    println!("  5秒: {:?}", d1);
    println!("  1500毫秒: {:?}", d2);
    println!("  1000000微秒: {:?}", d3);
    println!("  1000000000纳秒: {:?}", d4);
    
    // Duration运算
    let d5 = d1 + d2;
    let d6 = d1 - Duration::from_millis(500);
    let d7 = d2 * 3;
    let d8 = d1 / 2;
    
    println!("\nDuration运算:");
    println!("  5秒 + 1.5秒 = {:?}", d5);
    println!("  5秒 - 0.5秒 = {:?}", d6);
    println!("  1.5秒 × 3 = {:?}", d7);
    println!("  5秒 ÷ 2 = {:?}", d8);
    
    // Duration转换
    println!("\nDuration转换:");
    println!("  {} 秒", d1.as_secs());
    println!("  {} 毫秒", d1.as_millis());
    println!("  {} 微秒", d1.as_micros());
    println!("  {} 纳秒", d1.as_nanos());
    println!("  {:.2} 秒 (浮点)", d1.as_secs_f64());
    
    // Duration比较
    println!("\nDuration比较:");
    println!("  5秒 > 1.5秒: {}", d1 > d2);
    println!("  5秒 == 5000毫秒: {}", d1 == Duration::from_millis(5000));
    
    // 特殊Duration值
    println!("\n特殊Duration值:");
    println!("  零Duration: {:?}", Duration::ZERO);
    println!("  最大Duration: {:?}", Duration::MAX);
    println!("  1秒: {:?}", Duration::SECOND);
    
    // 创建自定义Duration
    let custom = Duration::new(3, 500_000_000); // 3.5秒
    println!("  自定义Duration (3.5秒): {:?}", custom);
}

// Instant 瞬时时间示例
fn instant_examples() {
    // 获取当前时刻
    let now = Instant::now();
    println!("当前时刻: {:?}", now);
    
    // 模拟一些工作
    thread::sleep(Duration::from_millis(100));
    
    // 计算时间差
    let elapsed = now.elapsed();
    println!("经过时间: {:?}", elapsed);
    
    // Instant运算
    let later = now + Duration::from_secs(10);
    println!("10秒后的时刻: {:?}", later);
    
    let earlier = now - Duration::from_secs(5);
    println!("5秒前的时刻: {:?}", earlier);
    
    // Instant比较
    let now2 = Instant::now();
    println!("时间比较:");
    println!("  现在 > 刚才: {}", now2 > now);
    println!("  时间差: {:?}", now2.duration_since(now));
    
    // 检查时间是否已过
    let deadline = Instant::now() + Duration::from_millis(50);
    thread::sleep(Duration::from_millis(100));
    
    if Instant::now() > deadline {
        println!("  截止时间已过");
    }
    
    // 饱和运算（避免溢出）
    let very_early = Instant::now();
    let saturating_sub = very_early.saturating_duration_since(later);
    println!("  饱和减法结果: {:?}", saturating_sub);
    
    // 检查距离时间点的剩余时间
    let future_time = Instant::now() + Duration::from_secs(2);
    match future_time.checked_duration_since(Instant::now()) {
        Some(remaining) => println!("  剩余时间: {:?}", remaining),
        None => println!("  时间已过"),
    }
}

// SystemTime 系统时间示例
fn system_time_examples() {
    // 获取当前系统时间
    let now = SystemTime::now();
    println!("当前系统时间: {:?}", now);
    
    // 转换为Unix时间戳
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let timestamp = duration.as_secs();
            println!("Unix时间戳: {}", timestamp);
            println!("毫秒时间戳: {}", duration.as_millis());
        }
        Err(e) => println!("时间转换错误: {}", e),
    }
    
    // 从Unix时间戳创建SystemTime
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let date_time = UNIX_EPOCH + Duration::from_secs(timestamp);
    println!("从时间戳创建: {:?}", date_time);
    
    // SystemTime运算
    let later = now + Duration::from_secs(3600); // 1小时后
    let earlier = now - Duration::from_secs(86400); // 1天前
    
    println!("系统时间运算:");
    println!("  1小时后: {:?}", later);
    println!("  1天前: {:?}", earlier);
    
    // 计算时间差
    match later.duration_since(now) {
        Ok(diff) => println!("  时间差: {:?}", diff),
        Err(e) => println!("  时间差计算错误: {}", e),
    }
    
    // 系统时间比较
    println!("系统时间比较:");
    println!("  现在 > 1天前: {}", now > earlier);
    println!("  现在 < 1小时后: {}", now < later);
    
    // 检查系统时间调整
    let before = SystemTime::now();
    thread::sleep(Duration::from_millis(10));
    let after = SystemTime::now();
    
    match after.duration_since(before) {
        Ok(diff) => println!("  正常时间流逝: {:?}", diff),
        Err(_) => println!("  检测到系统时间倒退"),
    }
}

// 时间测量和基准测试
fn time_measurement() {
    // 简单的时间测量
    println!("简单时间测量:");
    let start = Instant::now();
    
    // 模拟计算工作
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    
    let duration = start.elapsed();
    println!("  计算耗时: {:?} (结果: {})", duration, sum);
    
    // 多次测量取平均值
    println!("多次测量基准测试:");
    let iterations = 10;
    let mut total_duration = Duration::ZERO;
    
    for i in 0..iterations {
        let start = Instant::now();
        
        // 测试函数
        expensive_operation();
        
        let duration = start.elapsed();
        total_duration += duration;
        println!("  第{}次: {:?}", i + 1, duration);
    }
    
    let average = total_duration / iterations;
    println!("  平均耗时: {:?}", average);
    
    // 比较不同算法的性能
    println!("算法性能比较:");
    compare_algorithms();
    
    // 内存分配性能测试
    println!("内存分配性能测试:");
    memory_allocation_benchmark();
}

// 超时控制
fn timeout_control() {
    // 线程超时等待
    println!("线程超时控制:");
    
    let timeout = Duration::from_millis(500);
    let start = Instant::now();
    
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(300));
        "任务完成"
    });
    
    // 模拟超时检查
    while start.elapsed() < timeout {
        if handle.is_finished() {
            let result = handle.join().unwrap();
            println!("  任务及时完成: {}", result);
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    if start.elapsed() >= timeout {
        println!("  任务超时");
    }
    
    // 重试机制
    println!("重试机制:");
    retry_with_timeout();
    
    // 自适应超时
    println!("自适应超时:");
    adaptive_timeout_example();
}

// 定时器和调度
fn timer_and_scheduling() {
    // 简单定时器
    println!("简单定时器:");
    let timer = SimpleTimer::new();
    
    timer.schedule(Duration::from_millis(100), || {
        println!("  定时器触发 - 100ms");
    });
    
    timer.schedule(Duration::from_millis(200), || {
        println!("  定时器触发 - 200ms");
    });
    
    timer.schedule(Duration::from_millis(300), || {
        println!("  定时器触发 - 300ms");
    });
    
    // 等待所有定时器完成
    thread::sleep(Duration::from_millis(400));
    
    // 周期性任务
    println!("周期性任务:");
    periodic_task_example();
    
    // 心跳机制
    println!("心跳机制:");
    heartbeat_example();
}

// 时间格式化和解析
fn time_formatting() {
    let now = SystemTime::now();
    
    // 基本时间戳显示
    if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
        let timestamp = duration.as_secs();
        println!("Unix时间戳: {}", timestamp);
        
        // 简单的日期计算 (不考虑时区和闰年的简化版本)
        let days_since_epoch = timestamp / 86400;
        let years_since_1970 = days_since_epoch / 365;
        let approximate_year = 1970 + years_since_1970;
        
        println!("大约的年份: {}", approximate_year);
        
        // 一天中的时间
        let seconds_today = timestamp % 86400;
        let hours = seconds_today / 3600;
        let minutes = (seconds_today % 3600) / 60;
        let seconds = seconds_today % 60;
        
        println!("UTC时间: {:02}:{:02}:{:02}", hours, minutes, seconds);
    }
    
    // 时间差的人性化显示
    println!("时间差人性化显示:");
    let durations = vec![
        Duration::from_secs(30),
        Duration::from_secs(90),
        Duration::from_secs(3600),
        Duration::from_secs(7200),
        Duration::from_secs(86400),
        Duration::from_secs(172800),
    ];
    
    for duration in durations {
        println!("  {}: {}", duration.as_secs(), humanize_duration(duration));
    }
    
    // ISO 8601 格式示例
    println!("ISO 8601格式示例:");
    iso8601_example();
}

// 性能分析工具
fn performance_analysis() {
    // 性能分析器
    let mut profiler = SimpleProfiler::new();
    
    profiler.start("数据处理");
    thread::sleep(Duration::from_millis(50));
    profiler.end("数据处理");
    
    profiler.start("网络请求");
    thread::sleep(Duration::from_millis(100));
    profiler.end("网络请求");
    
    profiler.start("数据库查询");
    thread::sleep(Duration::from_millis(75));
    profiler.end("数据库查询");
    
    profiler.report();
    
    // CPU使用率监控
    println!("CPU使用率监控:");
    cpu_usage_monitor();
    
    // 内存使用监控
    println!("内存使用监控:");
    memory_usage_monitor();
}

// 实际应用示例
fn practical_examples() {
    // 日志记录器
    println!("日志记录器:");
    logger_example();
    
    // 缓存过期管理
    println!("缓存过期管理:");
    cache_expiry_example();
    
    // 限流器
    println!("限流器:");
    rate_limiter_example();
    
    // 性能监控
    println!("性能监控:");
    performance_monitor_example();
}

// 最佳实践
fn best_practices() {
    println!("时间处理最佳实践:");
    println!("1. 使用Instant测量时间间隔，避免系统时间调整影响");
    println!("2. 使用SystemTime获取当前时间戳");
    println!("3. 选择合适的时间精度，避免过度精确");
    println!("4. 考虑时区问题，标准库不处理时区");
    println!("5. 使用chrono库处理复杂日期时间操作");
    println!("6. 在性能测试中预热代码");
    println!("7. 多次测量取平均值以减少误差");
    println!("8. 注意闰秒对精确时间计算的影响");
    println!("9. 在并发环境中正确处理时间竞态");
    println!("10. 为超时操作设置合理的默认值");
    
    // 时间处理陷阱
    println!("\n常见陷阱:");
    common_pitfalls();
    
    // 性能优化建议
    println!("\n性能优化建议:");
    performance_tips();
}

// 辅助函数和结构体

// 昂贵的操作用于基准测试
fn expensive_operation() {
    let mut sum = 0;
    for i in 0..100_000 {
        sum += i * i;
    }
    // 防止编译器优化掉计算
    std::hint::black_box(sum);
}

// 比较算法性能
fn compare_algorithms() {
    let data: Vec<i32> = (0..10000).collect();
    
    // 算法1: 简单遍历
    let start = Instant::now();
    let sum1: i32 = data.iter().sum();
    let duration1 = start.elapsed();
    
    // 算法2: 并行计算（模拟）
    let start = Instant::now();
    let sum2: i32 = data.chunks(2500)
        .map(|chunk| chunk.iter().sum::<i32>())
        .sum();
    let duration2 = start.elapsed();
    
    println!("  算法1 (简单遍历): {:?} (结果: {})", duration1, sum1);
    println!("  算法2 (分块计算): {:?} (结果: {})", duration2, sum2);
    
    if duration1 > duration2 {
        println!("  算法2更快");
    } else {
        println!("  算法1更快");
    }
}

// 内存分配基准测试
fn memory_allocation_benchmark() {
    // Vec分配测试
    let start = Instant::now();
    let mut vecs = Vec::new();
    for _ in 0..1000 {
        let v: Vec<i32> = Vec::with_capacity(1000);
        vecs.push(v);
    }
    let vec_duration = start.elapsed();
    
    // HashMap分配测试
    let start = Instant::now();
    let mut maps = Vec::new();
    for _ in 0..1000 {
        let m: HashMap<i32, i32> = HashMap::with_capacity(1000);
        maps.push(m);
    }
    let map_duration = start.elapsed();
    
    println!("  Vec分配: {:?}", vec_duration);
    println!("  HashMap分配: {:?}", map_duration);
    
    // 防止编译器优化
    std::hint::black_box(vecs);
    std::hint::black_box(maps);
}

// 重试机制
fn retry_with_timeout() {
    let max_retries = 3;
    let timeout = Duration::from_millis(100);
    
    for attempt in 1..=max_retries {
        let start = Instant::now();
        
        // 模拟可能失败的操作
        let success = simulate_network_request();
        
        if success {
            println!("  第{}次尝试成功", attempt);
            return;
        }
        
        let elapsed = start.elapsed();
        if elapsed < timeout {
            let wait_time = timeout - elapsed;
            println!("  第{}次尝试失败，等待{:?}后重试", attempt, wait_time);
            thread::sleep(wait_time);
        } else {
            println!("  第{}次尝试失败，已超时", attempt);
        }
    }
    
    println!("  所有重试均失败");
}

// 自适应超时示例
fn adaptive_timeout_example() {
    let mut adaptive_timeout = Duration::from_millis(100);
    let mut response_times = Vec::new();
    
    for i in 1..=5 {
        let start = Instant::now();
        
        // 模拟请求
        let response_time = simulate_request_with_time();
        let actual_time = start.elapsed();
        
        response_times.push(actual_time);
        
        // 调整超时时间
        if response_times.len() >= 3 {
            let avg_time: Duration = response_times.iter().sum::<Duration>() / response_times.len() as u32;
            adaptive_timeout = avg_time * 2; // 设置为平均时间的2倍
        }
        
        println!("  请求{}: 实际{:?}, 模拟{:?}, 新超时{:?}", 
                 i, actual_time, response_time, adaptive_timeout);
        
        thread::sleep(Duration::from_millis(10));
    }
}

// 简单定时器
struct SimpleTimer;

impl SimpleTimer {
    fn new() -> Self {
        SimpleTimer
    }
    
    fn schedule<F>(&self, delay: Duration, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(move || {
            thread::sleep(delay);
            callback();
        });
    }
}

// 周期性任务示例
fn periodic_task_example() {
    let interval = Duration::from_millis(100);
    let mut count = 0;
    let max_count = 3;
    
    while count < max_count {
        let start = Instant::now();
        
        // 执行任务
        println!("  周期性任务执行 #{}", count + 1);
        thread::sleep(Duration::from_millis(20)); // 模拟工作
        
        // 计算下次执行时间
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
        
        count += 1;
    }
}

// 心跳机制示例
fn heartbeat_example() {
    let heartbeat_interval = Duration::from_millis(200);
    let mut last_heartbeat = Instant::now();
    let timeout = Duration::from_millis(500);
    
    // 模拟心跳发送
    for i in 1..=3 {
        thread::sleep(heartbeat_interval);
        println!("  发送心跳 #{}", i);
        last_heartbeat = Instant::now();
        
        // 检查心跳超时
        if last_heartbeat.elapsed() > timeout {
            println!("  心跳超时！");
            break;
        }
    }
}

// 时间差人性化显示
fn humanize_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    
    if secs < 60 {
        format!("{}秒", secs)
    } else if secs < 3600 {
        format!("{}分{}秒", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}小时{}分", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}天{}小时", secs / 86400, (secs % 86400) / 3600)
    }
}

// ISO 8601 格式示例
fn iso8601_example() {
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
        let timestamp = duration.as_secs();
        
        // 简化的ISO 8601格式（仅UTC，不处理时区）
        let days_since_epoch = timestamp / 86400;
        let seconds_today = timestamp % 86400;
        
        // 简化的年月日计算（不考虑闰年）
        let years = days_since_epoch / 365;
        let remaining_days = days_since_epoch % 365;
        let months = remaining_days / 30;
        let days = remaining_days % 30;
        
        let hours = seconds_today / 3600;
        let minutes = (seconds_today % 3600) / 60;
        let seconds = seconds_today % 60;
        
        let iso_date = format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                              1970 + years, months + 1, days + 1,
                              hours, minutes, seconds);
        
        println!("  简化ISO 8601: {}", iso_date);
    }
}

// 简单性能分析器
struct SimpleProfiler {
    start_times: HashMap<String, Instant>,
    durations: HashMap<String, Duration>,
}

impl SimpleProfiler {
    fn new() -> Self {
        SimpleProfiler {
            start_times: HashMap::new(),
            durations: HashMap::new(),
        }
    }
    
    fn start(&mut self, name: &str) {
        self.start_times.insert(name.to_string(), Instant::now());
    }
    
    fn end(&mut self, name: &str) {
        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            self.durations.insert(name.to_string(), duration);
        }
    }
    
    fn report(&self) {
        println!("性能分析报告:");
        let mut items: Vec<_> = self.durations.iter().collect();
        items.sort_by_key(|(_, duration)| *duration);
        items.reverse();
        
        for (name, duration) in items {
            println!("  {}: {:?}", name, duration);
        }
    }
}

// CPU使用率监控（简化版）
fn cpu_usage_monitor() {
    let start = Instant::now();
    let start_time = SystemTime::now();
    
    // 模拟CPU密集型工作
    let mut work_time = Duration::ZERO;
    let monitor_duration = Duration::from_millis(200);
    
    while start.elapsed() < monitor_duration {
        let work_start = Instant::now();
        
        // 模拟工作
        for _ in 0..10000 {
            std::hint::black_box(std::ptr::null::<i32>());
        }
        
        work_time += work_start.elapsed();
        
        // 模拟空闲
        thread::sleep(Duration::from_millis(1));
    }
    
    let total_time = start.elapsed();
    let cpu_usage = (work_time.as_nanos() as f64 / total_time.as_nanos() as f64) * 100.0;
    
    println!("  模拟CPU使用率: {:.1}%", cpu_usage);
}

// 内存使用监控（概念演示）
fn memory_usage_monitor() {
    // 获取内存使用情况（Rust标准库无法直接获取）
    println!("  内存监控需要外部库支持");
    println!("  可以使用 sys-info 或 psutil 等库");
    
    // 演示内存分配跟踪
    let mut allocated_bytes = 0;
    let start = Instant::now();
    
    let mut data = Vec::new();
    for _ in 0..1000 {
        let chunk = vec![0u8; 1024]; // 1KB
        allocated_bytes += 1024;
        data.push(chunk);
    }
    
    let allocation_time = start.elapsed();
    println!("  分配了 {} KB 内存，耗时 {:?}", allocated_bytes / 1024, allocation_time);
    
    // 防止编译器优化
    std::hint::black_box(data);
}

// 日志记录器示例
fn logger_example() {
    struct SimpleLogger;
    
    impl SimpleLogger {
        fn log(&self, level: &str, message: &str) {
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let timestamp = duration.as_secs();
                println!("  [{}] {} - {}", timestamp, level, message);
            }
        }
    }
    
    let logger = SimpleLogger;
    logger.log("INFO", "应用程序启动");
    thread::sleep(Duration::from_millis(10));
    logger.log("WARN", "配置文件缺失，使用默认值");
    thread::sleep(Duration::from_millis(10));
    logger.log("ERROR", "数据库连接失败");
}

// 缓存过期管理
fn cache_expiry_example() {
    struct ExpiringCache<T> {
        data: HashMap<String, (T, SystemTime)>,
        ttl: Duration,
    }
    
    impl<T> ExpiringCache<T> {
        fn new(ttl: Duration) -> Self {
            ExpiringCache {
                data: HashMap::new(),
                ttl,
            }
        }
        
        fn insert(&mut self, key: String, value: T) {
            let expiry = SystemTime::now() + self.ttl;
            self.data.insert(key, (value, expiry));
        }
        
        fn get(&self, key: &str) -> Option<&T> {
            if let Some((value, expiry)) = self.data.get(key) {
                if SystemTime::now() < *expiry {
                    Some(value)
                } else {
                    None // 已过期
                }
            } else {
                None
            }
        }
        
        fn cleanup(&mut self) {
            let now = SystemTime::now();
            self.data.retain(|_, (_, expiry)| now < *expiry);
        }
    }
    
    let mut cache = ExpiringCache::new(Duration::from_millis(100));
    
    cache.insert("key1".to_string(), "value1");
    println!("  插入 key1");
    
    if let Some(value) = cache.get("key1") {
        println!("  获取 key1: {}", value);
    }
    
    thread::sleep(Duration::from_millis(150));
    
    if cache.get("key1").is_none() {
        println!("  key1 已过期");
    }
    
    cache.cleanup();
    println!("  清理过期项");
}

// 限流器示例
fn rate_limiter_example() {
    struct RateLimiter {
        max_requests: usize,
        window: Duration,
        requests: Vec<Instant>,
    }
    
    impl RateLimiter {
        fn new(max_requests: usize, window: Duration) -> Self {
            RateLimiter {
                max_requests,
                window,
                requests: Vec::new(),
            }
        }
        
        fn allow_request(&mut self) -> bool {
            let now = Instant::now();
            
            // 清理过期的请求记录
            self.requests.retain(|&time| now.duration_since(time) < self.window);
            
            if self.requests.len() < self.max_requests {
                self.requests.push(now);
                true
            } else {
                false
            }
        }
    }
    
    let mut limiter = RateLimiter::new(3, Duration::from_millis(200));
    
    for i in 1..=6 {
        if limiter.allow_request() {
            println!("  请求 {} 允许", i);
        } else {
            println!("  请求 {} 被限流", i);
        }
        thread::sleep(Duration::from_millis(50));
    }
}

// 性能监控示例
fn performance_monitor_example() {
    struct PerformanceMonitor {
        metrics: HashMap<String, Vec<Duration>>,
    }
    
    impl PerformanceMonitor {
        fn new() -> Self {
            PerformanceMonitor {
                metrics: HashMap::new(),
            }
        }
        
        fn record(&mut self, operation: &str, duration: Duration) {
            self.metrics.entry(operation.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
        
        fn report(&self) {
            for (operation, durations) in &self.metrics {
                if !durations.is_empty() {
                    let total: Duration = durations.iter().sum();
                    let avg = total / durations.len() as u32;
                    let min = *durations.iter().min().unwrap();
                    let max = *durations.iter().max().unwrap();
                    
                    println!("  {}: 平均{:?}, 最小{:?}, 最大{:?} ({} 次)",
                             operation, avg, min, max, durations.len());
                }
            }
        }
    }
    
    let mut monitor = PerformanceMonitor::new();
    
    // 记录一些操作
    for i in 0..5 {
        let start = Instant::now();
        thread::sleep(Duration::from_millis(10 + i * 5));
        monitor.record("task", start.elapsed());
    }
    
    monitor.report();
}

// 常见陷阱
fn common_pitfalls() {
    println!("1. 使用SystemTime测量时间间隔可能受系统时间调整影响");
    println!("2. Duration运算可能溢出，使用饱和运算避免panic");
    println!("3. 在循环中频繁调用Instant::now()影响性能");
    println!("4. 忽略时区差异，标准库不处理时区");
    println!("5. 精度损失，将纳秒转换为秒时注意精度");
}

// 性能优化建议
fn performance_tips() {
    println!("1. 缓存Instant::now()的调用结果");
    println!("2. 使用适当的时间精度，避免过度精确");
    println!("3. 批量处理时间相关操作");
    println!("4. 在性能关键路径上避免时间转换");
    println!("5. 考虑使用单调时钟进行性能测量");
}

// 模拟函数
fn simulate_network_request() -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let random = hasher.finish() % 100;
    
    thread::sleep(Duration::from_millis(20 + random % 50));
    random > 30 // 70% 成功率
}

fn simulate_request_with_time() -> Duration {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let random = hasher.finish() % 100;
    
    let duration = Duration::from_millis(50 + random % 100);
    thread::sleep(duration);
    duration
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_duration_operations() {
        let d1 = Duration::from_secs(5);
        let d2 = Duration::from_millis(1500);
        
        let sum = d1 + d2;
        assert_eq!(sum, Duration::from_millis(6500));
        
        let diff = d1 - Duration::from_millis(500);
        assert_eq!(diff, Duration::from_millis(4500));
    }
    
    #[test]
    fn test_instant_measurement() {
        let start = Instant::now();
        thread::sleep(Duration::from_millis(10));
        let elapsed = start.elapsed();
        
        assert!(elapsed >= Duration::from_millis(10));
        assert!(elapsed < Duration::from_millis(50)); // 给一些缓冲
    }
    
    #[test]
    fn test_system_time() {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).unwrap();
        
        // 检查时间戳是否合理（2020年之后）
        assert!(timestamp.as_secs() > 1577836800); // 2020-01-01
    }
    
    #[test]
    fn test_time_formatting() {
        let duration = Duration::from_secs(3661); // 1小时1分1秒
        let formatted = humanize_duration(duration);
        assert_eq!(formatted, "1小时1分");
    }
    
    #[test]
    fn test_profiler() {
        let mut profiler = SimpleProfiler::new();
        
        profiler.start("test");
        thread::sleep(Duration::from_millis(10));
        profiler.end("test");
        
        assert!(profiler.durations.contains_key("test"));
        let duration = profiler.durations.get("test").unwrap();
        assert!(*duration >= Duration::from_millis(10));
    }
    
    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, Duration::from_millis(100));
        
        assert!(limiter.allow_request());
        assert!(limiter.allow_request());
        assert!(!limiter.allow_request()); // 第三个请求应该被限制
        
        thread::sleep(Duration::from_millis(110));
        assert!(limiter.allow_request()); // 窗口重置后应该允许
    }
    
    #[test]
    fn test_expiring_cache() {
        let mut cache = ExpiringCache::new(Duration::from_millis(50));
        
        cache.insert("key".to_string(), "value");
        assert_eq!(cache.get("key"), Some(&"value"));
        
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get("key"), None); // 应该已过期
    }
}