// 07_线程并发.rs
// Rust标准库线程和并发编程详解

/*
Rust的并发编程模型基于所有权系统，提供了内存安全的并发：

核心概念：
- 线程：std::thread模块提供线程创建和管理
- 消息传递：通道(channel)用于线程间通信
- 共享状态：Mutex、RwLock等同步原语
- 原子操作：std::sync::atomic模块

线程安全特征：
- Send：可以在线程间转移所有权
- Sync：可以在线程间共享引用

同步原语：
- Mutex<T>：互斥锁，确保同一时间只有一个线程访问数据
- RwLock<T>：读写锁，允许多个读取者或一个写入者
- Arc<T>：原子引用计数，用于多线程共享数据
- Barrier：屏障，让多个线程在某个点同步
- Condvar：条件变量，线程间的通知机制

通道类型：
- mpsc::channel：多生产者单消费者通道
- mpsc::sync_channel：同步通道（有缓冲区限制）

原子类型：
- AtomicBool、AtomicI32、AtomicUsize等
- 提供无锁的原子操作
*/

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::collections::HashMap;

fn main() {
    println!("=== Rust标准库线程和并发编程 ===");
    
    // 1. 基础线程操作
    println!("\n1. 基础线程操作：");
    basic_threading();
    
    // 2. 线程间消息传递
    println!("\n2. 线程间消息传递：");
    message_passing();
    
    // 3. 共享状态并发
    println!("\n3. 共享状态并发：");
    shared_state_concurrency();
    
    // 4. 原子操作
    println!("\n4. 原子操作：");
    atomic_operations();
    
    // 5. 高级同步原语
    println!("\n5. 高级同步原语：");
    advanced_synchronization();
    
    // 6. 线程池模式
    println!("\n6. 线程池模式：");
    thread_pool_pattern();
    
    // 7. 并发数据结构
    println!("\n7. 并发数据结构：");
    concurrent_data_structures();
    
    // 8. 性能测试和基准
    println!("\n8. 性能测试和基准：");
    performance_benchmarks();
    
    // 9. 实际应用场景
    println!("\n9. 实际应用场景：");
    practical_applications();
    
    // 10. 并发编程最佳实践
    println!("\n10. 并发编程最佳实践：");
    best_practices();
    
    println!("\n=== 线程并发学习完成 ===");
}

// 基础线程操作
fn basic_threading() {
    // 创建简单线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程计数: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        "子线程完成"
    });
    
    // 主线程工作
    for i in 1..=3 {
        println!("主线程计数: {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    // 等待子线程完成
    match handle.join() {
        Ok(result) => println!("线程结果: {}", result),
        Err(_) => println!("线程执行出错"),
    }
    
    // 带参数的线程
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("向量元素和: {}", sum);
        sum
    });
    
    let result = handle.join().unwrap();
    println!("计算结果: {}", result);
    
    // 线程构建器
    let builder = thread::Builder::new()
        .name("worker-thread".into())
        .stack_size(32 * 1024); // 32KB栈
    
    let handle = builder.spawn(|| {
        println!("工作线程名称: {:?}", thread::current().name());
        println!("线程ID: {:?}", thread::current().id());
    }).unwrap();
    
    handle.join().unwrap();
    
    // 获取当前线程信息
    println!("主线程名称: {:?}", thread::current().name());
    println!("主线程ID: {:?}", thread::current().id());
    
    // 让出CPU时间片
    thread::yield_now();
    
    // 检查线程是否可以暂停
    if thread::park_timeout(Duration::from_millis(10)).is_timeout() {
        println!("线程暂停超时");
    }
}

// 线程间消息传递
fn message_passing() {
    // 基本通道使用
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["消息1", "消息2", "消息3"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到消息: {}", received);
    }
    
    // 多生产者单消费者
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("线程{} 消息{}", i, j);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    // 关闭原始发送者
    drop(tx);
    
    // 等待所有生产者完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 接收所有消息
    for received in rx {
        println!("多生产者消息: {}", received);
    }
    
    // 同步通道（有界通道）
    let (tx, rx) = mpsc::sync_channel(2); // 缓冲区大小为2
    
    let sender = thread::spawn(move || {
        for i in 0..5 {
            println!("发送消息 {}", i);
            tx.send(i).unwrap();
            println!("消息 {} 已发送", i);
        }
    });
    
    thread::sleep(Duration::from_millis(200));
    
    let receiver = thread::spawn(move || {
        for received in rx {
            println!("接收到: {}", received);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
    
    // 选择性接收
    demonstrate_channel_selection();
}

// 共享状态并发
fn shared_state_concurrency() {
    // Mutex 互斥锁
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Mutex计数器最终值: {}", *counter.lock().unwrap());
    
    // RwLock 读写锁
    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut map = data.write().unwrap();
            map.insert(format!("key{}", i), i * 10);
            println!("写入 key{} = {}", i, i * 10);
        });
        handles.push(handle);
    }
    
    // 等待写入完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 读取线程
    let mut handles = vec![];
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let map = data.read().unwrap();
            for (key, value) in map.iter() {
                println!("读取线程{}: {} = {}", i, key, value);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 条件变量
    demonstrate_condvar();
}

// 原子操作
fn atomic_operations() {
    // AtomicUsize 计数器
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("原子计数器最终值: {}", counter.load(Ordering::SeqCst));
    
    // AtomicBool 标志
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);
    
    // 设置标志的线程
    let setter = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        flag_clone.store(true, Ordering::SeqCst);
        println!("标志已设置为 true");
    });
    
    // 等待标志的线程
    let waiter = thread::spawn(move || {
        while !flag.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }
        println!("检测到标志为 true");
    });
    
    setter.join().unwrap();
    waiter.join().unwrap();
    
    // 原子操作的内存序
    demonstrate_memory_ordering();
    
    // Compare and Swap (CAS) 操作
    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            loop {
                let current = value.load(Ordering::SeqCst);
                let new_value = current + (i + 1) * 10;
                
                match value.compare_exchange(
                    current, 
                    new_value, 
                    Ordering::SeqCst, 
                    Ordering::SeqCst
                ) {
                    Ok(_) => {
                        println!("线程{} 成功更新: {} -> {}", i, current, new_value);
                        break;
                    }
                    Err(actual) => {
                        println!("线程{} CAS失败，期望{}, 实际{}", i, current, actual);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("CAS最终值: {}", value.load(Ordering::SeqCst));
}

// 高级同步原语
fn advanced_synchronization() {
    // Barrier 屏障
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程{} 开始工作", i);
            thread::sleep(Duration::from_millis((i as u64) * 100));
            println!("线程{} 到达屏障", i);
            
            let wait_result = barrier.wait();
            if wait_result.is_leader() {
                println!("线程{} 是屏障的领导者", i);
            }
            
            println!("线程{} 通过屏障，继续执行", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Once 一次性初始化
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    static mut CONFIG: Option<String> = None;
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            INIT.call_once(|| {
                unsafe {
                    CONFIG = Some(format!("由线程{}初始化的配置", i));
                }
                println!("线程{} 执行了初始化", i);
            });
            
            unsafe {
                if let Some(ref config) = CONFIG {
                    println!("线程{} 读取配置: {}", i, config);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// 线程池模式
fn thread_pool_pattern() {
    println!("线程池模式演示:");
    
    // 简单线程池实现
    let pool = SimpleThreadPool::new(4);
    
    for i in 0..10 {
        pool.execute(move || {
            println!("任务{} 开始执行", i);
            thread::sleep(Duration::from_millis(100));
            println!("任务{} 执行完成", i);
        });
    }
    
    thread::sleep(Duration::from_millis(1500));
    println!("所有任务提交完成");
}

// 并发数据结构
fn concurrent_data_structures() {
    // 并发HashMap示例
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    // 多线程写入
    for i in 0..5 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let key = format!("thread{}_{}", i, j);
                let value = i * 10 + j;
                
                let mut m = map.lock().unwrap();
                m.insert(key.clone(), value);
                println!("插入: {} = {}", key, value);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 读取所有数据
    let map = map.lock().unwrap();
    println!("并发HashMap最终内容:");
    for (key, value) in map.iter() {
        println!("  {} = {}", key, value);
    }
    
    // 无锁队列概念演示
    demonstrate_lockfree_concepts();
}

// 性能测试和基准
fn performance_benchmarks() {
    println!("并发性能测试:");
    
    // 测试不同同步机制的性能
    let iterations = 1_000_000;
    
    // Mutex性能测试
    let start = std::time::Instant::now();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..iterations / 4 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    println!("Mutex性能: {:?} ({} 次操作)", mutex_duration, iterations);
    
    // 原子操作性能测试
    let start = std::time::Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..iterations / 4 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    println!("原子操作性能: {:?} ({} 次操作)", atomic_duration, iterations);
    
    println!("原子操作比Mutex快 {:.2} 倍", 
             mutex_duration.as_nanos() as f64 / atomic_duration.as_nanos() as f64);
}

// 实际应用场景
fn practical_applications() {
    // 生产者-消费者模式
    println!("生产者-消费者模式:");
    producer_consumer_example();
    
    // 工作窃取模式
    println!("工作窃取模式概念:");
    work_stealing_concept();
    
    // MapReduce模式
    println!("MapReduce模式:");
    map_reduce_example();
}

// 并发编程最佳实践
fn best_practices() {
    println!("并发编程最佳实践:");
    println!("1. 优先使用消息传递而非共享状态");
    println!("2. 使用Arc<Mutex<T>>共享可变数据");
    println!("3. 使用Arc<RwLock<T>>进行读多写少的场景");
    println!("4. 原子操作适用于简单数值计算");
    println!("5. 避免死锁：总是以相同顺序获取锁");
    println!("6. 使用RAII确保锁的正确释放");
    println!("7. 考虑使用crossbeam等第三方库");
    println!("8. 合理设置线程数量，通常等于CPU核心数");
    println!("9. 避免在持有锁时进行I/O操作");
    println!("10. 使用线程池而非频繁创建线程");
    
    // 死锁预防示例
    deadlock_prevention_example();
    
    // 性能分析建议
    println!("\n性能分析建议:");
    println!("- 使用profiler分析锁竞争");
    println!("- 监控线程利用率");
    println!("- 测试不同并发级别");
    println!("- 考虑NUMA架构影响");
}

// 辅助函数和结构体

// 简单线程池实现
struct SimpleThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl SimpleThreadPool {
    fn new(size: usize) -> SimpleThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        SimpleThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for SimpleThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                
                match job {
                    Ok(job) => {
                        println!("Worker {} 开始执行任务", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} 断开连接，停止工作", id);
                        break;
                    }
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// 条件变量演示
fn demonstrate_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        
        thread::sleep(Duration::from_millis(200));
        
        *started = true;
        cvar.notify_one();
        println!("条件变量已通知");
    });
    
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    
    while !*started {
        println!("等待条件变量...");
        started = cvar.wait(started).unwrap();
    }
    
    println!("条件变量收到通知，继续执行");
}

// 通道选择演示
fn demonstrate_channel_selection() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx1.send("来自通道1").unwrap();
    });
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx2.send("来自通道2").unwrap();
    });
    
    // 注意：标准库没有内置的select!宏
    // 这里演示基本的轮询接收
    let mut received = 0;
    let start = std::time::Instant::now();
    
    while received < 2 && start.elapsed() < Duration::from_millis(500) {
        if let Ok(msg) = rx1.try_recv() {
            println!("选择性接收: {}", msg);
            received += 1;
        }
        
        if let Ok(msg) = rx2.try_recv() {
            println!("选择性接收: {}", msg);
            received += 1;
        }
        
        thread::sleep(Duration::from_millis(10));
    }
}

// 内存序演示
fn demonstrate_memory_ordering() {
    println!("内存序演示:");
    println!("- Relaxed: 最弱的序，只保证原子性");
    println!("- Acquire/Release: 获取-释放语义");
    println!("- AcqRel: 同时具有获取和释放语义");
    println!("- SeqCst: 顺序一致性，最强的序");
    
    // 演示Release-Acquire模式
    let data = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    
    let data_clone = Arc::clone(&data);
    let flag_clone = Arc::clone(&flag);
    
    // 写入线程
    let writer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        flag_clone.store(true, Ordering::Release); // Release
    });
    
    // 读取线程
    let reader = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) { // Acquire
            thread::yield_now();
        }
        let value = data.load(Ordering::Relaxed);
        println!("读取到数据: {}", value);
    });
    
    writer.join().unwrap();
    reader.join().unwrap();
}

// 无锁概念演示
fn demonstrate_lockfree_concepts() {
    println!("无锁数据结构概念:");
    println!("- 使用原子操作和CAS");
    println!("- 避免阻塞，提高性能");
    println!("- 实现复杂，需要careful memory ordering");
    println!("- 常见的有：无锁队列、无锁链表、无锁哈希表");
    
    // 简单的无锁计数器
    struct LockFreeCounter {
        count: AtomicUsize,
    }
    
    impl LockFreeCounter {
        fn new() -> Self {
            LockFreeCounter {
                count: AtomicUsize::new(0),
            }
        }
        
        fn increment(&self) -> usize {
            self.count.fetch_add(1, Ordering::SeqCst)
        }
        
        fn get(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }
    }
    
    let counter = Arc::new(LockFreeCounter::new());
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("无锁计数器最终值: {}", counter.get());
}

// 生产者消费者示例
fn producer_consumer_example() {
    let (tx, rx) = mpsc::sync_channel(10); // 缓冲区大小10
    
    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 0..20 {
            let item = format!("产品{}", i);
            println!("生产: {}", item);
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        for item in rx {
            println!("消费: {}", item);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

// 工作窃取概念
fn work_stealing_concept() {
    println!("工作窃取模式特点:");
    println!("- 每个线程维护自己的工作队列");
    println!("- 空闲线程从其他线程队列窃取任务");
    println!("- 减少线程间竞争，提高效率");
    println!("- 适用于分治算法和任务并行");
}

// MapReduce示例
fn map_reduce_example() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 3;
    let (tx, rx) = mpsc::channel();
    
    // Map阶段：并行处理数据块
    for chunk in data.chunks(chunk_size) {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        
        thread::spawn(move || {
            let sum: i32 = chunk.iter().map(|x| x * x).sum(); // 平方和
            tx.send(sum).unwrap();
        });
    }
    
    drop(tx); // 关闭发送端
    
    // Reduce阶段：收集并汇总结果
    let total: i32 = rx.iter().sum();
    println!("MapReduce结果 (平方和): {}", total);
}

// 死锁预防示例
fn deadlock_prevention_example() {
    println!("死锁预防示例:");
    
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        // 总是先获取resource1，再获取resource2
        let _lock1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap();
        println!("线程1 获取了两个资源");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        // 保持相同的获取顺序
        let _lock1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap();
        println!("线程2 获取了两个资源");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("成功避免死锁");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_threading() {
        let handle = thread::spawn(|| {
            42
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            tx.send("测试消息").unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_eq!(received, "测试消息");
    }
    
    #[test]
    fn test_mutex() {
        let data = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let mut num = data.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*data.lock().unwrap(), 10);
    }
    
    #[test]
    fn test_atomic_operations() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }
    
    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = vec![];
        
        for i in 0..3 {
            let barrier = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                barrier.wait();
                i
            });
            handles.push(handle);
        }
        
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_rwlock() {
        let data = Arc::new(RwLock::new(5));
        
        // 测试读锁
        {
            let r1 = data.read().unwrap();
            let r2 = data.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
        
        // 测试写锁
        {
            let mut w = data.write().unwrap();
            *w = 10;
        }
        
        let r = data.read().unwrap();
        assert_eq!(*r, 10);
    }
    
    #[test]
    fn test_thread_pool() {
        let pool = SimpleThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }
        
        // 等待任务完成
        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }
}