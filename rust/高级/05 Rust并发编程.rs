// 05 Rust并发编程 - 线程、消息传递和共享状态
// 本章介绍Rust的并发编程：线程创建、消息传递、互斥锁和原子操作

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // 基本线程示例
    basic_threads();
    
    // 消息传递示例
    message_passing();
    
    // 共享状态示例
    shared_state();
    
    // 原子操作示例
    atomic_operations();
}

// 案例1：基本线程操作
fn basic_threads() {
    println!("=== 基本线程示例 ===");
    
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("线程中的数字: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 主线程也做一些工作
    for i in 1..5 {
        println!("主线程中的数字: {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    // 等待线程完成
    handle.join().unwrap();
    println!("线程执行完成");
    
    // 线程与闭包捕获
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("线程中的向量: {:?}", v);
    });
    
    handle.join().unwrap();
    
    // 多个线程
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("线程 {} 开始执行", i);
            thread::sleep(Duration::from_millis(i * 100));
            println!("线程 {} 执行完成", i);
            i * i  // 返回值
        });
        handles.push(handle);
    }
    
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("线程 {} 的结果: {}", i, result);
    }
}

// 案例2：消息传递并发
fn message_passing() {
    println!("\n=== 消息传递示例 ===");
    
    // 基本通道使用
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hello from thread");
        tx.send(val).unwrap();
        println!("发送方: 消息已发送");
    });
    
    let received = rx.recv().unwrap();
    println!("接收方: 收到消息 '{}'", received);
    
    // 发送多个消息
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("message 1"),
            String::from("message 2"),
            String::from("message 3"),
            String::from("message 4"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
    
    // 多个发送者
    multiple_producers();
    
    // 工作者线程池
    worker_thread_pool();
}

fn multiple_producers() {
    println!("\n--- 多发送者示例 ---");
    
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("发送者1: 消息A"),
            String::from("发送者1: 消息B"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("发送者2: 消息X"),
            String::from("发送者2: 消息Y"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(400));
        }
    });
    
    for received in rx {
        println!("多发送者: {}", received);
    }
}

fn worker_thread_pool() {
    println!("\n--- 工作者线程池示例 ---");
    
    let (job_tx, job_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    
    // 创建工作者线程
    let num_workers = 3;
    let job_rx = Arc::new(Mutex::new(job_rx));
    
    for worker_id in 0..num_workers {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        
        thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                
                match job {
                    Ok(job) => {
                        println!("工作者 {} 处理任务: {}", worker_id, job);
                        thread::sleep(Duration::from_millis(500));
                        let result = format!("工作者 {} 完成任务 {}", worker_id, job);
                        result_tx.send(result).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
    }
    
    // 发送任务
    for i in 1..=5 {
        job_tx.send(format!("任务{}", i)).unwrap();
    }
    
    drop(job_tx);  // 关闭发送端
    drop(result_tx);  // 关闭结果发送端
    
    // 收集结果
    for result in result_rx {
        println!("收到结果: {}", result);
    }
}

// 案例3：共享状态并发
fn shared_state() {
    println!("\n=== 共享状态示例 ===");
    
    // 基本互斥锁使用
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    
    println!("互斥锁中的值: {:?}", m);
    
    // 多线程共享计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("计数器最终值: {}", *counter.lock().unwrap());
    
    // 银行账户转账示例
    bank_transfer_example();
    
    // 生产者消费者模式
    producer_consumer_pattern();
}

// 银行转账示例
#[derive(Debug)]
struct BankAccount {
    balance: Mutex<f64>,
}

impl BankAccount {
    fn new(initial_balance: f64) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
        }
    }
    
    fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("存款 {:.2}，余额: {:.2}", amount, *balance);
    }
    
    fn withdraw(&self, amount: f64) -> Result<(), String> {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            println!("取款 {:.2}，余额: {:.2}", amount, *balance);
            Ok(())
        } else {
            Err("余额不足".to_string())
        }
    }
    
    fn get_balance(&self) -> f64 {
        *self.balance.lock().unwrap()
    }
}

fn bank_transfer_example() {
    println!("\n--- 银行转账示例 ---");
    
    let account1 = Arc::new(BankAccount::new(1000.0));
    let account2 = Arc::new(BankAccount::new(500.0));
    
    let mut handles = vec![];
    
    // 账户1存款
    {
        let account = Arc::clone(&account1);
        let handle = thread::spawn(move || {
            account.deposit(200.0);
        });
        handles.push(handle);
    }
    
    // 账户2取款
    {
        let account = Arc::clone(&account2);
        let handle = thread::spawn(move || {
            if let Err(e) = account.withdraw(600.0) {
                println!("取款失败: {}", e);
            }
        });
        handles.push(handle);
    }
    
    // 从账户1转账到账户2
    {
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);
        let handle = thread::spawn(move || {
            if acc1.withdraw(300.0).is_ok() {
                acc2.deposit(300.0);
                println!("转账成功");
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终余额 - 账户1: {:.2}, 账户2: {:.2}", 
             account1.get_balance(), account2.get_balance());
}

// 生产者消费者模式
fn producer_consumer_pattern() {
    println!("\n--- 生产者消费者模式 ---");
    
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_size = 5;
    
    // 生产者
    let producer_buffer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            loop {
                let mut buf = producer_buffer.lock().unwrap();
                if buf.len() < buffer_size {
                    buf.push(i);
                    println!("生产者: 生产 {}, 缓冲区大小: {}", i, buf.len());
                    break;
                } else {
                    drop(buf);
                    thread::sleep(Duration::from_millis(100));
                }
            }
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // 消费者
    let consumer_buffer = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        loop {
            let item = {
                let mut buf = consumer_buffer.lock().unwrap();
                buf.pop()
            };
            
            if let Some(item) = item {
                println!("消费者: 消费 {}", item);
                thread::sleep(Duration::from_millis(300));
            } else {
                thread::sleep(Duration::from_millis(100));
            }
            
            // 简单的退出条件
            if item == Some(10) {
                break;
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

// 案例4：原子操作
static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn atomic_operations() {
    println!("\n=== 原子操作示例 ===");
    
    // 基本原子操作
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&atomic_counter);
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
    
    println!("原子计数器最终值: {}", atomic_counter.load(Ordering::SeqCst));
    
    // 全局原子变量
    let mut handles = vec![];
    
    for _ in 0..5 {
        let handle = thread::spawn(|| {
            for _ in 0..100 {
                GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("全局原子计数器值: {}", GLOBAL_COUNTER.load(Ordering::SeqCst));
    
    // 比较并交换操作
    compare_and_swap_example();
    
    // 性能对比
    performance_comparison();
}

fn compare_and_swap_example() {
    println!("\n--- 比较并交换示例 ---");
    
    let atomic_value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let value = Arc::clone(&atomic_value);
        let handle = thread::spawn(move || {
            // 尝试将0设置为i+1
            let old_value = value.compare_exchange(
                0,
                i + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            
            match old_value {
                Ok(val) => println!("线程 {} 成功设置值为 {}", i, i + 1),
                Err(val) => println!("线程 {} 设置失败，当前值为 {}", i, val),
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终原子值: {}", atomic_value.load(Ordering::SeqCst));
}

fn performance_comparison() {
    println!("\n--- 性能对比示例 ---");
    
    use std::time::Instant;
    
    let iterations = 1_000_000;
    
    // 互斥锁性能测试
    let mutex_counter = Arc::new(Mutex::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let counter = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..iterations/4 {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    println!("互斥锁 {} 次操作耗时: {:?}", iterations, mutex_time);
    
    // 原子操作性能测试
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..iterations/4 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_time = start.elapsed();
    println!("原子操作 {} 次操作耗时: {:?}", iterations, atomic_time);
    
    println!("性能提升: {:.2}倍", 
             mutex_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
}

// 异步任务调度器示例
struct TaskScheduler {
    tasks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl TaskScheduler {
    fn new(num_workers: usize) -> Self {
        let tasks = Arc::new(Mutex::new(Vec::new()));
        let mut workers = Vec::new();
        
        for worker_id in 0..num_workers {
            let tasks = Arc::clone(&tasks);
            let worker = thread::spawn(move || {
                loop {
                    let task = {
                        let mut task_queue = tasks.lock().unwrap();
                        task_queue.pop()
                    };
                    
                    if let Some(task) = task {
                        println!("工作者 {} 执行任务", worker_id);
                        task();
                    } else {
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            });
            workers.push(worker);
        }
        
        TaskScheduler { tasks, workers }
    }
    
    fn submit<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut task_queue = self.tasks.lock().unwrap();
        task_queue.push(Box::new(task));
    }
}

fn task_scheduler_example() {
    println!("\n=== 任务调度器示例 ===");
    
    let scheduler = TaskScheduler::new(2);
    
    for i in 1..=5 {
        scheduler.submit(move || {
            println!("执行任务 {}", i);
            thread::sleep(Duration::from_millis(500));
            println!("任务 {} 完成", i);
        });
    }
    
    thread::sleep(Duration::from_secs(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_thread() {
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
            tx.send(42).unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_eq!(received, 42);
    }

    #[test]
    fn test_shared_state() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num = 42;
        });
        
        handle.join().unwrap();
        assert_eq!(*counter.lock().unwrap(), 42);
    }

    #[test]
    fn test_atomic_operations() {
        let atomic = AtomicUsize::new(0);
        
        atomic.store(42, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), 42);
        
        let old_value = atomic.fetch_add(1, Ordering::SeqCst);
        assert_eq!(old_value, 42);
        assert_eq!(atomic.load(Ordering::SeqCst), 43);
    }

    #[test]
    fn test_bank_account() {
        let account = BankAccount::new(1000.0);
        
        account.deposit(500.0);
        assert_eq!(account.get_balance(), 1500.0);
        
        assert!(account.withdraw(200.0).is_ok());
        assert_eq!(account.get_balance(), 1300.0);
        
        assert!(account.withdraw(2000.0).is_err());
    }

    #[test]
    fn test_examples() {
        basic_threads();
        message_passing();
        shared_state();
        atomic_operations();
        task_scheduler_example();
    }
}

// 并发编程要点总结：
// 1. 使用thread::spawn创建线程
// 2. mpsc通道实现线程间消息传递
// 3. Arc+Mutex实现共享状态的安全访问
// 4. 原子类型提供无锁并发操作
// 5. 所有权系统确保线程安全
// 6. Send和Sync trait标记线程安全的类型
// 7. 避免数据竞争和死锁
// 8. 合理选择并发原语提高性能