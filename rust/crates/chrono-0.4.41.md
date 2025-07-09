# Chrono 0.4.41 中文教程

## 简介

Chrono 是 Rust 中最流行的日期和时间处理库。它提供了丰富的 API 来处理日期、时间、时区、持续时间等，支持多种日期格式的解析和格式化，是构建任何需要处理时间数据的 Rust 应用程序的首选工具。

## 核心类型

### 1. 主要类型概览

- **DateTime<Tz>**: 带时区的日期时间
- **NaiveDateTime**: 不带时区的日期时间
- **NaiveDate**: 不带时区的日期
- **NaiveTime**: 不带时区的时间
- **Duration**: 时间间隔
- **TimeZone**: 时区信息

### 2. 时区类型

- **Utc**: UTC 时区
- **Local**: 本地时区
- **FixedOffset**: 固定偏移时区

## 基本用法

### 1. 获取当前时间

```rust
use chrono::{DateTime, Utc, Local, NaiveDateTime};

fn main() {
    // 获取当前 UTC 时间
    let utc_now = Utc::now();
    println!("UTC 时间: {}", utc_now);
    
    // 获取当前本地时间
    let local_now = Local::now();
    println!("本地时间: {}", local_now);
    
    // 不带时区的当前时间
    let naive_now = Local::now().naive_local();
    println!("不带时区的时间: {}", naive_now);
    
    // 获取时间戳
    let timestamp = utc_now.timestamp();
    println!("时间戳: {}", timestamp);
    
    // 获取毫秒时间戳
    let timestamp_millis = utc_now.timestamp_millis();
    println!("毫秒时间戳: {}", timestamp_millis);
}
```

### 2. 创建特定时间

```rust
use chrono::{DateTime, Utc, TimeZone, NaiveDate, NaiveTime};

fn main() {
    // 创建特定的 UTC 时间
    let specific_utc = Utc.ymd(2023, 12, 25).and_hms(15, 30, 0);
    println!("特定 UTC 时间: {}", specific_utc);
    
    // 创建带毫秒的时间
    let precise_time = Utc.ymd(2023, 12, 25).and_hms_milli(15, 30, 0, 500);
    println!("精确时间: {}", precise_time);
    
    // 使用 NaiveDate 和 NaiveTime 创建
    let date = NaiveDate::from_ymd(2023, 12, 25);
    let time = NaiveTime::from_hms(15, 30, 0);
    let datetime = date.and_time(time);
    println!("组合日期时间: {}", datetime);
    
    // 从时间戳创建
    let from_timestamp = Utc.timestamp(1703521800, 0);
    println!("从时间戳创建: {}", from_timestamp);
    
    // 从毫秒时间戳创建
    let from_millis = Utc.timestamp_millis(1703521800000);
    println!("从毫秒时间戳创建: {}", from_millis);
}
```

### 3. 时间格式化

```rust
use chrono::{DateTime, Utc, Local, TimeZone};

fn main() {
    let now = Utc::now();
    
    // 标准格式
    println!("标准格式: {}", now);
    
    // RFC 3339 格式
    println!("RFC 3339: {}", now.to_rfc3339());
    
    // RFC 2822 格式
    println!("RFC 2822: {}", now.to_rfc2822());
    
    // 自定义格式
    println!("自定义格式: {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("中文格式: {}", now.format("%Y年%m月%d日 %H:%M:%S"));
    
    // 常用格式
    println!("日期: {}", now.format("%Y-%m-%d"));
    println!("时间: {}", now.format("%H:%M:%S"));
    println!("年份: {}", now.format("%Y"));
    println!("月份: {}", now.format("%m"));
    println!("日期: {}", now.format("%d"));
    
    // 星期和月份名称
    println!("星期: {}", now.format("%A"));
    println!("月份名称: {}", now.format("%B"));
    
    // 12小时制
    println!("12小时制: {}", now.format("%I:%M:%S %p"));
}
```

### 4. 时间解析

```rust
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析 RFC 3339 格式
    let rfc3339_str = "2023-12-25T15:30:00Z";
    let parsed_rfc3339 = DateTime::parse_from_rfc3339(rfc3339_str)?;
    println!("解析 RFC 3339: {}", parsed_rfc3339);
    
    // 解析 RFC 2822 格式
    let rfc2822_str = "Mon, 25 Dec 2023 15:30:00 +0000";
    let parsed_rfc2822 = DateTime::parse_from_rfc2822(rfc2822_str)?;
    println!("解析 RFC 2822: {}", parsed_rfc2822);
    
    // 解析自定义格式
    let custom_str = "2023-12-25 15:30:00";
    let parsed_custom = NaiveDateTime::parse_from_str(custom_str, "%Y-%m-%d %H:%M:%S")?;
    println!("解析自定义格式: {}", parsed_custom);
    
    // 解析中文格式
    let chinese_str = "2023年12月25日 15:30:00";
    let parsed_chinese = NaiveDateTime::parse_from_str(chinese_str, "%Y年%m月%d日 %H:%M:%S")?;
    println!("解析中文格式: {}", parsed_chinese);
    
    // 解析多种格式
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
    ];
    
    let time_strings = [
        "2023-12-25 15:30:00",
        "2023/12/25 15:30:00", 
        "25/12/2023 15:30:00",
        "2023-12-25T15:30:00",
    ];
    
    for time_str in time_strings {
        for format in formats {
            if let Ok(parsed) = NaiveDateTime::parse_from_str(time_str, format) {
                println!("解析 '{}' 使用格式 '{}': {}", time_str, format, parsed);
                break;
            }
        }
    }
    
    Ok(())
}
```

## 时间操作

### 1. 时间运算

```rust
use chrono::{DateTime, Utc, Duration};

fn main() {
    let now = Utc::now();
    
    // 加法运算
    let future = now + Duration::days(7);
    println!("一周后: {}", future);
    
    let later = now + Duration::hours(3);
    println!("三小时后: {}", later);
    
    let soon = now + Duration::minutes(30);
    println!("半小时后: {}", soon);
    
    // 减法运算
    let past = now - Duration::days(1);
    println!("昨天: {}", past);
    
    let earlier = now - Duration::hours(2);
    println!("两小时前: {}", earlier);
    
    // 计算时间差
    let diff = future - now;
    println!("时间差: {} 天", diff.num_days());
    println!("时间差: {} 小时", diff.num_hours());
    println!("时间差: {} 分钟", diff.num_minutes());
    println!("时间差: {} 秒", diff.num_seconds());
    
    // 组合运算
    let complex_time = now + Duration::days(2) + Duration::hours(5) - Duration::minutes(30);
    println!("复杂运算结果: {}", complex_time);
}
```

### 2. 时间比较

```rust
use chrono::{DateTime, Utc, Duration};

fn main() {
    let now = Utc::now();
    let future = now + Duration::hours(1);
    let past = now - Duration::hours(1);
    
    // 基本比较
    println!("future > now: {}", future > now);
    println!("past < now: {}", past < now);
    println!("now == now: {}", now == now);
    
    // 时间范围检查
    let start = now - Duration::hours(2);
    let end = now + Duration::hours(2);
    let test_time = now + Duration::minutes(30);
    
    if test_time > start && test_time < end {
        println!("test_time 在时间范围内");
    }
    
    // 使用 min 和 max
    let earlier = std::cmp::min(now, future);
    let later = std::cmp::max(now, future);
    println!("较早时间: {}", earlier);
    println!("较晚时间: {}", later);
    
    // 时间距离
    let time_diff = (future - now).num_minutes().abs();
    println!("时间距离: {} 分钟", time_diff);
}
```

### 3. 时间组件操作

```rust
use chrono::{DateTime, Utc, Datelike, Timelike, Weekday};

fn main() {
    let now = Utc::now();
    
    // 获取时间组件
    println!("年份: {}", now.year());
    println!("月份: {}", now.month());
    println!("日期: {}", now.day());
    println!("小时: {}", now.hour());
    println!("分钟: {}", now.minute());
    println!("秒: {}", now.second());
    
    // 星期相关
    println!("星期几: {:?}", now.weekday());
    println!("是否是周末: {}", matches!(now.weekday(), Weekday::Sat | Weekday::Sun));
    
    // 年份相关
    println!("年中第几天: {}", now.ordinal());
    println!("是否是闰年: {}", now.year() % 4 == 0 && (now.year() % 100 != 0 || now.year() % 400 == 0));
    
    // 时间戳
    println!("Unix 时间戳: {}", now.timestamp());
    println!("毫秒时间戳: {}", now.timestamp_millis());
    println!("微秒时间戳: {}", now.timestamp_micros());
    println!("纳秒时间戳: {}", now.timestamp_nanos());
    
    // 创建特定时间
    let specific = now.with_hour(12).unwrap().with_minute(0).unwrap().with_second(0).unwrap();
    println!("今天中午: {}", specific);
    
    // 月初和月末
    let month_start = now.with_day(1).unwrap();
    let next_month = if now.month() == 12 {
        now.with_year(now.year() + 1).unwrap().with_month(1).unwrap()
    } else {
        now.with_month(now.month() + 1).unwrap()
    };
    let month_end = next_month.with_day(1).unwrap() - Duration::days(1);
    
    println!("月初: {}", month_start);
    println!("月末: {}", month_end);
}
```

## 时区处理

### 1. 时区转换

```rust
use chrono::{DateTime, Utc, Local, FixedOffset, TimeZone};

fn main() {
    let utc_time = Utc::now();
    
    // UTC 转本地时间
    let local_time = utc_time.with_timezone(&Local);
    println!("UTC 时间: {}", utc_time);
    println!("本地时间: {}", local_time);
    
    // 创建固定偏移时区
    let tokyo_offset = FixedOffset::east(9 * 3600); // UTC+9
    let tokyo_time = utc_time.with_timezone(&tokyo_offset);
    println!("东京时间: {}", tokyo_time);
    
    let new_york_offset = FixedOffset::west(5 * 3600); // UTC-5
    let new_york_time = utc_time.with_timezone(&new_york_offset);
    println!("纽约时间: {}", new_york_time);
    
    // 时区比较
    println!("UTC 和本地时间相等: {}", utc_time.timestamp() == local_time.timestamp());
    
    // 解析带时区的时间
    let with_tz = DateTime::parse_from_rfc3339("2023-12-25T15:30:00+08:00").unwrap();
    println!("带时区的时间: {}", with_tz);
    
    // 转换为不同时区
    let as_utc = with_tz.with_timezone(&Utc);
    let as_local = with_tz.with_timezone(&Local);
    println!("转换为 UTC: {}", as_utc);
    println!("转换为本地: {}", as_local);
}
```

### 2. 时区感知的时间处理

```rust
use chrono::{DateTime, Utc, Local, FixedOffset, TimeZone, Duration};

fn main() {
    // 创建不同时区的时间
    let utc_time = Utc.ymd(2023, 12, 25).and_hms(12, 0, 0);
    let beijing_offset = FixedOffset::east(8 * 3600); // UTC+8
    let beijing_time = beijing_offset.ymd(2023, 12, 25).and_hms(20, 0, 0);
    
    println!("UTC 时间: {}", utc_time);
    println!("北京时间: {}", beijing_time);
    
    // 比较不同时区的时间
    println!("时间相等: {}", utc_time == beijing_time);
    
    // 时区转换
    let beijing_as_utc = beijing_time.with_timezone(&Utc);
    println!("北京时间转 UTC: {}", beijing_as_utc);
    
    // 夏令时处理示例
    let dst_start = Utc.ymd(2023, 3, 12).and_hms(7, 0, 0); // 美国夏令时开始
    let dst_end = Utc.ymd(2023, 11, 5).and_hms(6, 0, 0);   // 美国夏令时结束
    
    let eastern_standard = FixedOffset::west(5 * 3600); // UTC-5
    let eastern_daylight = FixedOffset::west(4 * 3600); // UTC-4
    
    println!("DST 开始: {}", dst_start.with_timezone(&eastern_daylight));
    println!("DST 结束: {}", dst_end.with_timezone(&eastern_standard));
}
```

## 实际应用示例

### 1. 日志系统

```rust
use chrono::{DateTime, Utc, Local};
use std::fmt;

#[derive(Debug)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN "),
            LogLevel::Info => write!(f, "INFO "),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

struct LogEntry {
    timestamp: DateTime<Utc>,
    level: LogLevel,
    message: String,
    module: String,
}

impl LogEntry {
    fn new(level: LogLevel, message: String, module: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message,
            module,
        }
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] {} [{}] {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            self.level,
            self.module,
            self.message
        )
    }
}

struct Logger {
    entries: Vec<LogEntry>,
}

impl Logger {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    fn log(&mut self, level: LogLevel, message: String, module: String) {
        let entry = LogEntry::new(level, message, module);
        println!("{}", entry);
        self.entries.push(entry);
    }
    
    fn error(&mut self, message: String, module: String) {
        self.log(LogLevel::Error, message, module);
    }
    
    fn warn(&mut self, message: String, module: String) {
        self.log(LogLevel::Warn, message, module);
    }
    
    fn info(&mut self, message: String, module: String) {
        self.log(LogLevel::Info, message, module);
    }
    
    fn debug(&mut self, message: String, module: String) {
        self.log(LogLevel::Debug, message, module);
    }
    
    fn get_logs_since(&self, since: DateTime<Utc>) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.timestamp > since)
            .collect()
    }
    
    fn get_logs_between(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .collect()
    }
}

fn main() {
    let mut logger = Logger::new();
    
    // 记录不同级别的日志
    logger.info("应用程序启动".to_string(), "main".to_string());
    logger.debug("初始化数据库连接".to_string(), "database".to_string());
    logger.warn("配置文件中的某个参数已弃用".to_string(), "config".to_string());
    logger.error("数据库连接失败".to_string(), "database".to_string());
    
    // 查询特定时间段的日志
    let five_seconds_ago = Utc::now() - chrono::Duration::seconds(5);
    let recent_logs = logger.get_logs_since(five_seconds_ago);
    
    println!("\n最近5秒的日志:");
    for log in recent_logs {
        println!("{}", log);
    }
}
```

### 2. 任务调度系统

```rust
use chrono::{DateTime, Utc, Duration};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    scheduled_time: DateTime<Utc>,
    interval: Option<Duration>,
    handler: fn(&Task),
}

impl Task {
    fn new(id: u64, name: String, scheduled_time: DateTime<Utc>, handler: fn(&Task)) -> Self {
        Self {
            id,
            name,
            scheduled_time,
            interval: None,
            handler,
        }
    }
    
    fn new_recurring(id: u64, name: String, scheduled_time: DateTime<Utc>, interval: Duration, handler: fn(&Task)) -> Self {
        Self {
            id,
            name,
            scheduled_time,
            interval: Some(interval),
            handler,
        }
    }
    
    fn execute(&self) {
        println!("执行任务: {} (ID: {}) 在 {}", self.name, self.id, self.scheduled_time);
        (self.handler)(self);
    }
    
    fn next_execution(&self) -> Option<Task> {
        if let Some(interval) = self.interval {
            let mut next_task = self.clone();
            next_task.scheduled_time = self.scheduled_time + interval;
            Some(next_task)
        } else {
            None
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反向排序，以便最早的任务在堆顶
        other.scheduled_time.cmp(&self.scheduled_time)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

struct TaskScheduler {
    tasks: BinaryHeap<Task>,
    next_id: u64,
}

impl TaskScheduler {
    fn new() -> Self {
        Self {
            tasks: BinaryHeap::new(),
            next_id: 1,
        }
    }
    
    fn schedule_task(&mut self, name: String, scheduled_time: DateTime<Utc>, handler: fn(&Task)) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = Task::new(id, name, scheduled_time, handler);
        self.tasks.push(task);
        
        id
    }
    
    fn schedule_recurring_task(&mut self, name: String, first_run: DateTime<Utc>, interval: Duration, handler: fn(&Task)) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = Task::new_recurring(id, name, first_run, interval, handler);
        self.tasks.push(task);
        
        id
    }
    
    fn schedule_delayed_task(&mut self, name: String, delay: Duration, handler: fn(&Task)) -> u64 {
        let scheduled_time = Utc::now() + delay;
        self.schedule_task(name, scheduled_time, handler)
    }
    
    fn run_pending_tasks(&mut self) {
        let now = Utc::now();
        let mut executed_tasks = Vec::new();
        
        while let Some(task) = self.tasks.peek() {
            if task.scheduled_time <= now {
                let task = self.tasks.pop().unwrap();
                task.execute();
                executed_tasks.push(task);
            } else {
                break;
            }
        }
        
        // 重新调度重复任务
        for task in executed_tasks {
            if let Some(next_task) = task.next_execution() {
                self.tasks.push(next_task);
            }
        }
    }
    
    fn get_next_task_time(&self) -> Option<DateTime<Utc>> {
        self.tasks.peek().map(|task| task.scheduled_time)
    }
    
    fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

// 任务处理函数
fn backup_handler(task: &Task) {
    println!("执行备份任务: {}", task.name);
}

fn cleanup_handler(task: &Task) {
    println!("执行清理任务: {}", task.name);
}

fn report_handler(task: &Task) {
    println!("生成报告: {}", task.name);
}

fn main() {
    let mut scheduler = TaskScheduler::new();
    
    // 调度一次性任务
    let now = Utc::now();
    scheduler.schedule_task(
        "一次性任务".to_string(),
        now + Duration::seconds(5),
        backup_handler,
    );
    
    // 调度重复任务
    scheduler.schedule_recurring_task(
        "每分钟清理".to_string(),
        now + Duration::seconds(10),
        Duration::minutes(1),
        cleanup_handler,
    );
    
    // 调度延迟任务
    scheduler.schedule_delayed_task(
        "延迟报告".to_string(),
        Duration::seconds(15),
        report_handler,
    );
    
    // 调度每日任务
    let tomorrow_midnight = (now + Duration::days(1))
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap();
    
    scheduler.schedule_recurring_task(
        "每日备份".to_string(),
        tomorrow_midnight,
        Duration::days(1),
        backup_handler,
    );
    
    println!("任务调度器启动，当前时间: {}", now);
    println!("总任务数: {}", scheduler.task_count());
    
    if let Some(next_time) = scheduler.get_next_task_time() {
        println!("下一个任务时间: {}", next_time);
    }
    
    // 模拟运行调度器
    println!("\n开始执行任务...");
    for i in 0..20 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        scheduler.run_pending_tasks();
        
        if i % 5 == 0 {
            println!("当前时间: {}, 剩余任务: {}", Utc::now(), scheduler.task_count());
        }
    }
}
```

### 3. 时间跟踪和分析

```rust
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TimeEntry {
    id: u64,
    project: String,
    task: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    description: String,
}

impl TimeEntry {
    fn new(id: u64, project: String, task: String, description: String) -> Self {
        Self {
            id,
            project,
            task,
            start_time: Utc::now(),
            end_time: None,
            description,
        }
    }
    
    fn stop(&mut self) {
        self.end_time = Some(Utc::now());
    }
    
    fn duration(&self) -> Option<Duration> {
        self.end_time.map(|end| end - self.start_time)
    }
    
    fn is_active(&self) -> bool {
        self.end_time.is_none()
    }
}

struct TimeTracker {
    entries: Vec<TimeEntry>,
    next_id: u64,
}

impl TimeTracker {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }
    
    fn start_task(&mut self, project: String, task: String, description: String) -> u64 {
        // 停止所有活动的任务
        self.stop_all_active_tasks();
        
        let id = self.next_id;
        self.next_id += 1;
        
        let entry = TimeEntry::new(id, project, task, description);
        self.entries.push(entry);
        
        println!("开始任务: {} - {} (ID: {})", project, task, id);
        id
    }
    
    fn stop_task(&mut self, id: u64) -> Result<Duration, String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            if entry.is_active() {
                entry.stop();
                let duration = entry.duration().unwrap();
                println!("停止任务: {} - {} (持续时间: {})", entry.project, entry.task, format_duration(&duration));
                Ok(duration)
            } else {
                Err("任务已经停止".to_string())
            }
        } else {
            Err("找不到指定的任务".to_string())
        }
    }
    
    fn stop_all_active_tasks(&mut self) {
        for entry in self.entries.iter_mut() {
            if entry.is_active() {
                entry.stop();
            }
        }
    }
    
    fn get_active_task(&self) -> Option<&TimeEntry> {
        self.entries.iter().find(|e| e.is_active())
    }
    
    fn get_total_time_for_project(&self, project: &str) -> Duration {
        self.entries
            .iter()
            .filter(|e| e.project == project && e.duration().is_some())
            .fold(Duration::zero(), |acc, e| acc + e.duration().unwrap())
    }
    
    fn get_total_time_for_task(&self, project: &str, task: &str) -> Duration {
        self.entries
            .iter()
            .filter(|e| e.project == project && e.task == task && e.duration().is_some())
            .fold(Duration::zero(), |acc, e| acc + e.duration().unwrap())
    }
    
    fn get_daily_summary(&self, date: DateTime<Utc>) -> HashMap<String, Duration> {
        let start_of_day = date.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap();
        let end_of_day = start_of_day + Duration::days(1);
        
        let mut summary = HashMap::new();
        
        for entry in &self.entries {
            if entry.start_time >= start_of_day && entry.start_time < end_of_day {
                if let Some(duration) = entry.duration() {
                    let key = format!("{} - {}", entry.project, entry.task);
                    let total = summary.entry(key).or_insert(Duration::zero());
                    *total = *total + duration;
                }
            }
        }
        
        summary
    }
    
    fn get_weekly_summary(&self, week_start: DateTime<Utc>) -> HashMap<String, Duration> {
        let week_end = week_start + Duration::weeks(1);
        
        let mut summary = HashMap::new();
        
        for entry in &self.entries {
            if entry.start_time >= week_start && entry.start_time < week_end {
                if let Some(duration) = entry.duration() {
                    let total = summary.entry(entry.project.clone()).or_insert(Duration::zero());
                    *total = *total + duration;
                }
            }
        }
        
        summary
    }
}

fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    let mut tracker = TimeTracker::new();
    
    // 开始工作
    let task1 = tracker.start_task("项目A".to_string(), "功能开发".to_string(), "实现用户登录功能".to_string());
    
    // 模拟工作时间
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // 停止任务
    tracker.stop_task(task1).unwrap();
    
    // 开始另一个任务
    let task2 = tracker.start_task("项目B".to_string(), "bug修复".to_string(), "修复数据库连接问题".to_string());
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    // 检查活动任务
    if let Some(active) = tracker.get_active_task() {
        println!("当前活动任务: {} - {}", active.project, active.task);
    }
    
    tracker.stop_task(task2).unwrap();
    
    // 更多任务
    let task3 = tracker.start_task("项目A".to_string(), "测试".to_string(), "编写单元测试".to_string());
    std::thread::sleep(std::time::Duration::from_secs(1));
    tracker.stop_task(task3).unwrap();
    
    // 生成报告
    println!("\n=== 项目时间统计 ===");
    let project_a_time = tracker.get_total_time_for_project("项目A");
    let project_b_time = tracker.get_total_time_for_project("项目B");
    
    println!("项目A 总时间: {}", format_duration(&project_a_time));
    println!("项目B 总时间: {}", format_duration(&project_b_time));
    
    // 今日总结
    println!("\n=== 今日总结 ===");
    let today = Utc::now();
    let daily_summary = tracker.get_daily_summary(today);
    
    for (task, duration) in daily_summary {
        println!("{}: {}", task, format_duration(&duration));
    }
    
    // 本周总结
    println!("\n=== 本周总结 ===");
    let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let weekly_summary = tracker.get_weekly_summary(week_start);
    
    for (project, duration) in weekly_summary {
        println!("{}: {}", project, format_duration(&duration));
    }
}
```

## 性能优化

### 1. 避免不必要的转换

```rust
use chrono::{DateTime, Utc, NaiveDateTime};

fn main() {
    let now = Utc::now();
    
    // 好的做法：直接使用 DateTime<Utc>
    let timestamp = now.timestamp();
    
    // 避免不必要的转换
    // 不好的做法：
    // let naive = now.naive_utc();
    // let back_to_utc = DateTime::<Utc>::from_utc(naive, Utc);
    
    // 批量处理时间
    let times: Vec<DateTime<Utc>> = (0..1000)
        .map(|i| now + chrono::Duration::seconds(i))
        .collect();
    
    println!("生成 {} 个时间戳", times.len());
}
```

### 2. 使用适当的时间类型

```rust
use chrono::{DateTime, Utc, NaiveDateTime, Duration};

// 对于不需要时区的场景，使用 NaiveDateTime
fn process_local_times(times: &[NaiveDateTime]) {
    for time in times {
        // 处理本地时间，无需时区转换开销
        println!("处理时间: {}", time.format("%Y-%m-%d %H:%M:%S"));
    }
}

// 对于需要时区的场景，使用 DateTime<Utc>
fn process_utc_times(times: &[DateTime<Utc>]) {
    for time in times {
        // 处理 UTC 时间
        println!("处理 UTC 时间: {}", time.format("%Y-%m-%d %H:%M:%S"));
    }
}

fn main() {
    let naive_times: Vec<NaiveDateTime> = vec![
        NaiveDateTime::from_timestamp(1703521800, 0),
        NaiveDateTime::from_timestamp(1703525400, 0),
        NaiveDateTime::from_timestamp(1703529000, 0),
    ];
    
    let utc_times: Vec<DateTime<Utc>> = vec![
        Utc.timestamp(1703521800, 0),
        Utc.timestamp(1703525400, 0),
        Utc.timestamp(1703529000, 0),
    ];
    
    process_local_times(&naive_times);
    process_utc_times(&utc_times);
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
chrono = { version = "0.4.41", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 总结

Chrono 是 Rust 中功能最全面的日期时间处理库，提供了丰富的 API 来处理各种时间相关的任务。从简单的时间获取到复杂的时区转换，从时间格式化到时间运算，Chrono 都能提供优雅的解决方案。

主要特性：
- 🕒 完整的日期时间处理
- 🌍 强大的时区支持
- 📅 灵活的格式化和解析
- ⚡ 高性能的时间运算
- 🔧 与 Serde 无缝集成
- 📊 丰富的时间组件操作

无论是构建日志系统、任务调度器，还是时间跟踪应用，Chrono 都是处理时间数据的最佳选择。