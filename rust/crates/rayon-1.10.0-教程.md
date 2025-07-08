# Rayon 1.10.0 - Rust 简单并行编程完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [并行迭代器](#并行迭代器)
- [任务调度](#任务调度)
- [数据并行](#数据并行)
- [线程池管理](#线程池管理)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)
- [故障排除](#故障排除)

## 概述

Rayon 是 Rust 中最重要的并行编程库，通过工作窃取（work-stealing）算法提供简单而高效的数据并行处理能力。

### 核心特性
- **工作窃取调度**: 智能的任务分配和负载均衡
- **数据并行**: 类似迭代器的并行处理 API
- **零成本抽象**: 高性能的并行计算
- **线程安全**: 自动处理线程同步和数据竞争
- **易于使用**: 只需将 `.iter()` 改为 `.par_iter()`

### 版本信息
- **当前版本**: 1.10.0
- **发布时间**: 2024-03-24
- **下载次数**: 216,828,885+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
rayon = "1.10.0"
```

### 基本示例

```rust
use rayon::prelude::*;

fn main() {
    // 顺序处理
    let numbers: Vec<i32> = (0..1000000).collect();
    let start = std::time::Instant::now();
    let sum: i32 = numbers.iter().map(|x| x * x).sum();
    println!("顺序处理: {} 耗时: {:?}", sum, start.elapsed());
    
    // 并行处理
    let start = std::time::Instant::now();
    let par_sum: i32 = numbers.par_iter().map(|x| x * x).sum();
    println!("并行处理: {} 耗时: {:?}", par_sum, start.elapsed());
    
    // 并行搜索
    let target = 500000;
    let found = numbers.par_iter().find_any(|&&x| x == target);
    println!("并行搜索结果: {:?}", found);
    
    // 并行过滤
    let evens: Vec<_> = numbers.par_iter()
        .filter(|&&x| x % 2 == 0)
        .take(10)
        .collect();
    println!("前10个偶数: {:?}", evens);
}
```

## 核心概念

### 工作窃取算法

```rust
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn work_stealing_demo() {
    let counter = Arc::new(AtomicUsize::new(0));
    let thread_counter = Arc::new(AtomicUsize::new(0));
    
    // 并行任务会自动分配到不同线程
    (0..100).into_par_iter().for_each(|i| {
        let thread_id = std::thread::current().id();
        thread_counter.fetch_add(1, Ordering::Relaxed);
        
        // 模拟不同的工作负载
        let work_amount = if i % 10 == 0 { 10 } else { 1 };
        
        for _ in 0..work_amount {
            counter.fetch_add(1, Ordering::Relaxed);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        
        println!("任务 {} 在线程 {:?} 上完成", i, thread_id);
    });
    
    println!("总计数: {}", counter.load(Ordering::Relaxed));
    println!("线程使用次数: {}", thread_counter.load(Ordering::Relaxed));
}
```

### 数据并行模型

```rust
use rayon::prelude::*;

fn data_parallel_model() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 数据并行：每个元素独立处理
    let results: Vec<_> = data.par_iter()
        .map(|&x| expensive_computation(x))
        .collect();
    
    println!("并行计算结果: {:?}", results);
    
    // 归约操作
    let sum = data.par_iter().sum::<i32>();
    let product = data.par_iter().product::<i32>();
    let max = data.par_iter().max();
    
    println!("并行求和: {}", sum);
    println!("并行求积: {}", product);
    println!("并行最大值: {:?}", max);
    
    // 自定义归约
    let custom_reduce = data.par_iter()
        .map(|&x| x * x)
        .reduce(|| 0, |a, b| a + b);
    
    println!("平方和: {}", custom_reduce);
}

fn expensive_computation(x: i32) -> i32 {
    // 模拟复杂计算
    std::thread::sleep(std::time::Duration::from_millis(10));
    x * x + x
}
```

### 线程池概念

```rust
use rayon::prelude::*;

fn thread_pool_concept() {
    // 查看当前线程池信息
    let pool = rayon::current_thread_pool();
    if let Some(pool) = pool {
        println!("当前线程池大小: {}", rayon::current_num_threads());
    }
    
    // 展示线程复用
    let tasks = (0..20).collect::<Vec<_>>();
    
    tasks.par_iter().for_each(|&i| {
        let thread_id = std::thread::current().id();
        println!("任务 {} 在线程 {:?} 上执行", i, thread_id);
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
}
```

## 并行迭代器

### 基本并行迭代器

```rust
use rayon::prelude::*;

fn basic_parallel_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // par_iter() - 并行迭代不可变引用
    let sum: i32 = numbers.par_iter().sum();
    println!("并行求和: {}", sum);
    
    // par_iter_mut() - 并行迭代可变引用
    let mut numbers = numbers;
    numbers.par_iter_mut().for_each(|x| *x *= 2);
    println!("并行修改后: {:?}", numbers);
    
    // into_par_iter() - 消费迭代器
    let doubled: Vec<_> = numbers.into_par_iter()
        .map(|x| x * 2)
        .collect();
    println!("消费式并行: {:?}", doubled);
    
    // 范围并行迭代
    let range_sum: i32 = (0..1000).into_par_iter()
        .filter(|&x| x % 2 == 0)
        .sum();
    println!("范围并行求和: {}", range_sum);
}
```

### 并行迭代器适配器

```rust
use rayon::prelude::*;

fn parallel_iterator_adapters() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    
    // map - 并行映射
    let squares: Vec<_> = data.par_iter()
        .map(|&x| x * x)
        .collect();
    println!("并行平方: {:?}", squares);
    
    // filter - 并行过滤
    let evens: Vec<_> = data.par_iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("并行过滤偶数: {:?}", evens);
    
    // filter_map - 并行过滤映射
    let sqrt_evens: Vec<_> = data.par_iter()
        .filter_map(|&x| {
            if x % 2 == 0 {
                Some((x as f64).sqrt())
            } else {
                None
            }
        })
        .collect();
    println!("偶数平方根: {:?}", sqrt_evens);
    
    // enumerate - 并行枚举
    let enumerated: Vec<_> = data.par_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .collect();
    println!("枚举过滤: {:?}", enumerated);
    
    // zip - 并行压缩
    let other = vec![10, 20, 30, 40, 50];
    let zipped: Vec<_> = data.par_iter()
        .zip(other.par_iter())
        .map(|(a, b)| a + b)
        .collect();
    println!("并行压缩求和: {:?}", zipped);
}
```

### 并行迭代器消费者

```rust
use rayon::prelude::*;

fn parallel_iterator_consumers() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // collect - 收集结果
    let collected: Vec<_> = data.par_iter()
        .map(|&x| x * 2)
        .collect();
    println!("收集结果: {:?}", collected);
    
    // reduce - 归约
    let sum = data.par_iter()
        .map(|&x| x * x)
        .reduce(|| 0, |a, b| a + b);
    println!("归约求和: {}", sum);
    
    // fold - 并行折叠
    let sum = data.par_iter()
        .fold(|| 0, |acc, &x| acc + x)
        .sum::<i32>();
    println!("折叠求和: {}", sum);
    
    // for_each - 并行遍历
    println!("并行遍历:");
    data.par_iter().for_each(|&x| {
        println!("  处理: {}", x);
    });
    
    // find_any - 并行查找
    let found = data.par_iter()
        .find_any(|&&x| x > 5);
    println!("找到大于5的数: {:?}", found);
    
    // any/all - 并行谓词
    let has_even = data.par_iter().any(|&x| x % 2 == 0);
    let all_positive = data.par_iter().all(|&x| x > 0);
    println!("包含偶数: {}, 全部正数: {}", has_even, all_positive);
    
    // min/max - 并行最值
    let min_val = data.par_iter().min();
    let max_val = data.par_iter().max();
    println!("最小值: {:?}, 最大值: {:?}", min_val, max_val);
}
```

## 任务调度

### 基本任务调度

```rust
use rayon::prelude::*;

fn basic_task_scheduling() {
    // spawn - 创建异步任务
    let task1 = rayon::spawn(|| {
        println!("任务1开始");
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("任务1完成");
        42
    });
    
    let task2 = rayon::spawn(|| {
        println!("任务2开始");
        std::thread::sleep(std::time::Duration::from_millis(150));
        println!("任务2完成");
        "result"
    });
    
    // 等待任务完成
    println!("等待任务完成...");
    let result1 = task1.join();
    let result2 = task2.join();
    
    println!("任务1结果: {}", result1);
    println!("任务2结果: {}", result2);
}
```

### 分治算法

```rust
use rayon::prelude::*;

fn divide_and_conquer() {
    // 并行快速排序
    fn parallel_quicksort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }
        
        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        
        // 并行处理左右两部分
        rayon::join(
            || parallel_quicksort(left),
            || parallel_quicksort(&mut right[1..])
        );
    }
    
    fn partition(arr: &mut [i32]) -> usize {
        let pivot = arr.len() - 1;
        let mut i = 0;
        
        for j in 0..pivot {
            if arr[j] <= arr[pivot] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot);
        i
    }
    
    let mut data = vec![64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42];
    println!("排序前: {:?}", data);
    
    parallel_quicksort(&mut data);
    println!("排序后: {:?}", data);
}
```

### 并行递归

```rust
use rayon::prelude::*;

fn parallel_recursion() {
    // 并行斐波那契数列
    fn parallel_fibonacci(n: u32) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        if n < 20 {
            // 小数值使用顺序计算
            return sequential_fibonacci(n);
        }
        
        // 大数值使用并行计算
        let (a, b) = rayon::join(
            || parallel_fibonacci(n - 1),
            || parallel_fibonacci(n - 2)
        );
        
        a + b
    }
    
    fn sequential_fibonacci(n: u32) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        let mut a = 0;
        let mut b = 1;
        
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        b
    }
    
    println!("开始计算斐波那契数列...");
    let start = std::time::Instant::now();
    let result = parallel_fibonacci(30);
    println!("斐波那契(30) = {} 耗时: {:?}", result, start.elapsed());
    
    // 并行归并排序
    fn parallel_merge_sort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }
        
        let mid = arr.len() / 2;
        let (left, right) = arr.split_at_mut(mid);
        
        rayon::join(
            || parallel_merge_sort(left),
            || parallel_merge_sort(right)
        );
        
        merge(left, right);
    }
    
    fn merge(left: &mut [i32], right: &mut [i32]) {
        let mut temp = Vec::with_capacity(left.len() + right.len());
        let mut i = 0;
        let mut j = 0;
        
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                temp.push(left[i]);
                i += 1;
            } else {
                temp.push(right[j]);
                j += 1;
            }
        }
        
        temp.extend_from_slice(&left[i..]);
        temp.extend_from_slice(&right[j..]);
        
        // 复制回原数组
        for (idx, &val) in temp.iter().enumerate() {
            if idx < left.len() {
                left[idx] = val;
            } else {
                right[idx - left.len()] = val;
            }
        }
    }
    
    let mut data = vec![64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42];
    println!("归并排序前: {:?}", data);
    
    parallel_merge_sort(&mut data);
    println!("归并排序后: {:?}", data);
}
```

## 数据并行

### 数组并行处理

```rust
use rayon::prelude::*;

fn array_parallel_processing() {
    let size = 1000000;
    let mut matrix: Vec<Vec<f64>> = (0..size)
        .map(|i| (0..size).map(|j| (i * j) as f64).collect())
        .collect();
    
    // 并行矩阵运算
    let start = std::time::Instant::now();
    matrix.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|cell| {
            *cell = cell.sqrt() + 1.0;
        });
    });
    println!("并行矩阵运算耗时: {:?}", start.elapsed());
    
    // 并行向量运算
    let vec1: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let vec2: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();
    
    let start = std::time::Instant::now();
    let dot_product: f64 = vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(a, b)| a * b)
        .sum();
    println!("并行点积: {} 耗时: {:?}", dot_product, start.elapsed());
    
    // 并行数组求和
    let arr: Vec<i32> = (0..size as i32).collect();
    let sum: i64 = arr.par_iter().map(|&x| x as i64).sum();
    println!("并行数组求和: {}", sum);
}
```

### 字符串并行处理

```rust
use rayon::prelude::*;

fn string_parallel_processing() {
    let text = "这是一个测试字符串，用于演示并行字符串处理功能。
    我们将对这个字符串进行各种并行操作，包括查找、替换、分析等。
    Rayon 库可以很好地处理这些操作。";
    
    // 并行字符统计
    let char_count: usize = text.par_chars().count();
    println!("字符总数: {}", char_count);
    
    // 并行查找
    let contains_test = text.par_chars().any(|c| c == '测');
    println!("包含'测'字符: {}", contains_test);
    
    // 并行字符分类
    let (letters, digits, others): (Vec<_>, Vec<_>, Vec<_>) = text.par_chars()
        .partition_map(|c| {
            if c.is_alphabetic() {
                rayon::iter::Either::Left(c)
            } else if c.is_numeric() {
                rayon::iter::Either::Right(rayon::iter::Either::Left(c))
            } else {
                rayon::iter::Either::Right(rayon::iter::Either::Right(c))
            }
        });
    
    println!("字母数量: {}", letters.len());
    println!("数字数量: {}", digits.len());
    println!("其他字符数量: {}", others.len());
    
    // 并行单词处理
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_lengths: Vec<_> = words.par_iter()
        .map(|word| word.len())
        .collect();
    
    let total_length: usize = word_lengths.par_iter().sum();
    let avg_length = total_length as f64 / words.len() as f64;
    
    println!("单词总数: {}", words.len());
    println!("平均单词长度: {:.2}", avg_length);
}
```

### 集合并行处理

```rust
use rayon::prelude::*;
use std::collections::HashMap;

fn collection_parallel_processing() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 并行分组
    let groups: HashMap<bool, Vec<_>> = data.par_iter()
        .partition_map(|&x| {
            if x % 2 == 0 {
                rayon::iter::Either::Left((true, x))
            } else {
                rayon::iter::Either::Right((false, x))
            }
        });
    
    println!("分组结果: {:?}", groups);
    
    // 并行去重
    let duplicates = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique: Vec<_> = duplicates.par_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    println!("去重结果: {:?}", unique);
    
    // 并行映射到HashMap
    let word_counts: HashMap<char, usize> = "hello world".par_chars()
        .fold(HashMap::new, |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        })
        .reduce(HashMap::new, |mut map1, map2| {
            for (k, v) in map2 {
                *map1.entry(k).or_insert(0) += v;
            }
            map1
        });
    
    println!("字符计数: {:?}", word_counts);
}
```

## 线程池管理

### 自定义线程池

```rust
use rayon::ThreadPoolBuilder;

fn custom_thread_pool() {
    // 创建自定义线程池
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|index| format!("my-thread-{}", index))
        .build()
        .unwrap();
    
    // 在自定义线程池中执行任务
    let result = pool.install(|| {
        (0..100).into_par_iter()
            .map(|x| x * x)
            .sum::<i32>()
    });
    
    println!("自定义线程池结果: {}", result);
    
    // 检查线程池状态
    println!("线程池大小: {}", pool.current_num_threads());
    
    // 异步任务
    let future = pool.spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(100));
        "异步任务完成"
    });
    
    println!("异步任务结果: {}", future.join());
}
```

### 全局线程池配置

```rust
use rayon::ThreadPoolBuilder;

fn global_thread_pool_config() {
    // 配置全局线程池
    ThreadPoolBuilder::new()
        .num_threads(8)
        .thread_name(|index| format!("global-worker-{}", index))
        .build_global()
        .unwrap();
    
    println!("全局线程池配置完成");
    println!("当前线程数: {}", rayon::current_num_threads());
    
    // 使用全局线程池
    let result: i32 = (0..1000).into_par_iter()
        .map(|x| x * x)
        .sum();
    
    println!("全局线程池计算结果: {}", result);
}
```

### 线程池隔离

```rust
use rayon::ThreadPoolBuilder;

fn thread_pool_isolation() {
    // 创建不同的线程池用于不同任务
    let cpu_pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("cpu-{}", i))
        .build()
        .unwrap();
    
    let io_pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .thread_name(|i| format!("io-{}", i))
        .build()
        .unwrap();
    
    // CPU密集型任务
    let cpu_result = cpu_pool.install(|| {
        (0..1000000).into_par_iter()
            .map(|x| expensive_cpu_task(x))
            .sum::<i32>()
    });
    
    // IO密集型任务
    let io_result = io_pool.install(|| {
        (0..10).into_par_iter()
            .map(|x| expensive_io_task(x))
            .collect::<Vec<_>>()
    });
    
    println!("CPU任务结果: {}", cpu_result);
    println!("IO任务结果: {:?}", io_result);
}

fn expensive_cpu_task(x: i32) -> i32 {
    // 模拟CPU密集计算
    (0..x % 1000).sum::<i32>()
}

fn expensive_io_task(x: i32) -> String {
    // 模拟IO操作
    std::thread::sleep(std::time::Duration::from_millis(10));
    format!("IO-{}", x)
}
```

## 性能优化

### 并行度控制

```rust
use rayon::prelude::*;

fn parallelism_control() {
    let data: Vec<i32> = (0..1000000).collect();
    
    // 测试不同并行度
    for chunk_size in [1000, 10000, 100000] {
        let start = std::time::Instant::now();
        
        let sum: i32 = data.par_chunks(chunk_size)
            .map(|chunk| chunk.iter().sum::<i32>())
            .sum();
        
        println!("块大小 {}: 结果 {}, 耗时 {:?}", 
                 chunk_size, sum, start.elapsed());
    }
    
    // 使用with_min_len控制最小并行粒度
    let start = std::time::Instant::now();
    let sum: i32 = data.par_iter()
        .with_min_len(1000)
        .map(|&x| x * x)
        .sum();
    println!("最小粒度控制: 结果 {}, 耗时 {:?}", sum, start.elapsed());
}
```

### 缓存友好性

```rust
use rayon::prelude::*;

fn cache_friendly_processing() {
    let size = 1000;
    let mut matrix = vec![vec![0f64; size]; size];
    
    // 初始化矩阵
    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = (i * j) as f64;
        }
    }
    
    // 缓存友好的行优先处理
    let start = std::time::Instant::now();
    matrix.par_iter_mut().for_each(|row| {
        for cell in row.iter_mut() {
            *cell = cell.sqrt() + 1.0;
        }
    });
    println!("行优先处理耗时: {:?}", start.elapsed());
    
    // 对比：列优先处理（缓存不友好）
    let start = std::time::Instant::now();
    (0..size).into_par_iter().for_each(|j| {
        for i in 0..size {
            matrix[i][j] = matrix[i][j].sqrt() + 1.0;
        }
    });
    println!("列优先处理耗时: {:?}", start.elapsed());
}
```

### 内存分配优化

```rust
use rayon::prelude::*;

fn memory_allocation_optimization() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 预分配容量
    let start = std::time::Instant::now();
    let mut results = Vec::with_capacity(data.len());
    data.par_iter()
        .map(|&x| x * x)
        .collect_into_vec(&mut results);
    println!("预分配结果: {:?}, 耗时: {:?}", results, start.elapsed());
    
    // 使用fold减少分配
    let start = std::time::Instant::now();
    let sum = data.par_iter()
        .fold(|| 0, |acc, &x| acc + x * x)
        .sum::<i32>();
    println!("fold优化: {}, 耗时: {:?}", sum, start.elapsed());
    
    // 避免不必要的collect
    let start = std::time::Instant::now();
    let has_large = data.par_iter().any(|&x| x > 5);
    println!("避免collect: {}, 耗时: {:?}", has_large, start.elapsed());
}
```

## 实战案例

### 图像处理

```rust
use rayon::prelude::*;

struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![0; width * height * 3], // RGB
        }
    }
    
    fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let idx = (y * self.width + x) * 3;
        (self.data[idx], self.data[idx + 1], self.data[idx + 2])
    }
    
    fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let idx = (y * self.width + x) * 3;
        self.data[idx] = rgb.0;
        self.data[idx + 1] = rgb.1;
        self.data[idx + 2] = rgb.2;
    }
    
    // 并行灰度转换
    fn to_grayscale(&mut self) {
        self.data.par_chunks_mut(3).for_each(|pixel| {
            let gray = (0.299 * pixel[0] as f32 + 
                       0.587 * pixel[1] as f32 + 
                       0.114 * pixel[2] as f32) as u8;
            pixel[0] = gray;
            pixel[1] = gray;
            pixel[2] = gray;
        });
    }
    
    // 并行亮度调整
    fn adjust_brightness(&mut self, factor: f32) {
        self.data.par_iter_mut().for_each(|pixel| {
            *pixel = ((*pixel as f32 * factor).clamp(0.0, 255.0)) as u8;
        });
    }
    
    // 并行模糊效果
    fn blur(&mut self, radius: usize) {
        let original = self.data.clone();
        
        (0..self.height).into_par_iter().for_each(|y| {
            for x in 0..self.width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut count = 0u32;
                
                for dy in -(radius as i32)..=(radius as i32) {
                    for dx in -(radius as i32)..=(radius as i32) {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        
                        if ny >= 0 && ny < self.height as i32 && 
                           nx >= 0 && nx < self.width as i32 {
                            let idx = ((ny as usize) * self.width + (nx as usize)) * 3;
                            r_sum += original[idx] as u32;
                            g_sum += original[idx + 1] as u32;
                            b_sum += original[idx + 2] as u32;
                            count += 1;
                        }
                    }
                }
                
                let idx = (y * self.width + x) * 3;
                unsafe {
                    let data_ptr = self.data.as_mut_ptr();
                    *data_ptr.add(idx) = (r_sum / count) as u8;
                    *data_ptr.add(idx + 1) = (g_sum / count) as u8;
                    *data_ptr.add(idx + 2) = (b_sum / count) as u8;
                }
            }
        });
    }
}

fn image_processing_demo() {
    let mut image = Image::new(1000, 1000);
    
    // 生成测试图像
    for y in 0..image.height {
        for x in 0..image.width {
            let r = (x * 255 / image.width) as u8;
            let g = (y * 255 / image.height) as u8;
            let b = ((x + y) * 255 / (image.width + image.height)) as u8;
            image.set_pixel(x, y, (r, g, b));
        }
    }
    
    println!("图像处理开始...");
    
    // 并行灰度转换
    let start = std::time::Instant::now();
    image.to_grayscale();
    println!("灰度转换耗时: {:?}", start.elapsed());
    
    // 并行亮度调整
    let start = std::time::Instant::now();
    image.adjust_brightness(1.2);
    println!("亮度调整耗时: {:?}", start.elapsed());
    
    // 并行模糊
    let start = std::time::Instant::now();
    image.blur(2);
    println!("模糊效果耗时: {:?}", start.elapsed());
}
```

### 数据分析

```rust
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DataPoint {
    id: usize,
    value: f64,
    category: String,
    timestamp: u64,
}

struct DataAnalyzer {
    data: Vec<DataPoint>,
}

impl DataAnalyzer {
    fn new(data: Vec<DataPoint>) -> Self {
        DataAnalyzer { data }
    }
    
    // 并行统计分析
    fn analyze(&self) -> AnalysisResult {
        let start = std::time::Instant::now();
        
        // 并行计算基本统计量
        let (sum, count, min, max) = self.data.par_iter()
            .fold(
                || (0.0, 0, f64::INFINITY, f64::NEG_INFINITY),
                |acc, item| {
                    (
                        acc.0 + item.value,
                        acc.1 + 1,
                        acc.2.min(item.value),
                        acc.3.max(item.value),
                    )
                }
            )
            .reduce(
                || (0.0, 0, f64::INFINITY, f64::NEG_INFINITY),
                |acc1, acc2| {
                    (
                        acc1.0 + acc2.0,
                        acc1.1 + acc2.1,
                        acc1.2.min(acc2.2),
                        acc1.3.max(acc2.3),
                    )
                }
            );
        
        let mean = sum / count as f64;
        
        // 并行计算方差
        let variance = self.data.par_iter()
            .map(|item| (item.value - mean).powi(2))
            .sum::<f64>() / count as f64;
        
        // 并行分类统计
        let category_stats: HashMap<String, (f64, usize)> = self.data.par_iter()
            .fold(
                HashMap::new,
                |mut map, item| {
                    let entry = map.entry(item.category.clone()).or_insert((0.0, 0));
                    entry.0 += item.value;
                    entry.1 += 1;
                    map
                }
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (key, (sum, count)) in map2 {
                        let entry = map1.entry(key).or_insert((0.0, 0));
                        entry.0 += sum;
                        entry.1 += count;
                    }
                    map1
                }
            );
        
        // 并行时间序列分析
        let mut time_series: Vec<_> = self.data.par_iter()
            .map(|item| (item.timestamp, item.value))
            .collect();
        
        time_series.sort_by_key(|&(timestamp, _)| timestamp);
        
        let analysis_time = start.elapsed();
        
        AnalysisResult {
            count,
            sum,
            mean,
            variance,
            std_dev: variance.sqrt(),
            min,
            max,
            category_stats,
            time_series,
            analysis_time,
        }
    }
    
    // 并行过滤
    fn filter_by_category(&self, category: &str) -> Vec<DataPoint> {
        self.data.par_iter()
            .filter(|item| item.category == category)
            .cloned()
            .collect()
    }
    
    // 并行聚合
    fn aggregate_by_time_window(&self, window_size: u64) -> Vec<(u64, f64, usize)> {
        // 按时间窗口分组
        let mut windows: HashMap<u64, Vec<f64>> = HashMap::new();
        
        for item in &self.data {
            let window = item.timestamp / window_size * window_size;
            windows.entry(window).or_insert_with(Vec::new).push(item.value);
        }
        
        // 并行计算每个窗口的统计量
        windows.into_par_iter()
            .map(|(window, values)| {
                let sum: f64 = values.par_iter().sum();
                let count = values.len();
                (window, sum / count as f64, count)
            })
            .collect()
    }
}

#[derive(Debug)]
struct AnalysisResult {
    count: usize,
    sum: f64,
    mean: f64,
    variance: f64,
    std_dev: f64,
    min: f64,
    max: f64,
    category_stats: HashMap<String, (f64, usize)>,
    time_series: Vec<(u64, f64)>,
    analysis_time: std::time::Duration,
}

fn data_analysis_demo() {
    // 生成测试数据
    let data: Vec<DataPoint> = (0..100000)
        .map(|i| DataPoint {
            id: i,
            value: (i as f64 * 0.1).sin() * 100.0 + (i as f64 * 0.01).cos() * 50.0,
            category: match i % 5 {
                0 => "A".to_string(),
                1 => "B".to_string(),
                2 => "C".to_string(),
                3 => "D".to_string(),
                _ => "E".to_string(),
            },
            timestamp: i as u64,
        })
        .collect();
    
    let analyzer = DataAnalyzer::new(data);
    
    // 执行分析
    let result = analyzer.analyze();
    
    println!("数据分析结果:");
    println!("  数据量: {}", result.count);
    println!("  均值: {:.2}", result.mean);
    println!("  标准差: {:.2}", result.std_dev);
    println!("  最小值: {:.2}", result.min);
    println!("  最大值: {:.2}", result.max);
    println!("  分析耗时: {:?}", result.analysis_time);
    
    println!("\n分类统计:");
    for (category, (sum, count)) in result.category_stats {
        println!("  {}: 平均值 {:.2}, 数量 {}", 
                 category, sum / count as f64, count);
    }
    
    // 过滤示例
    let category_a = analyzer.filter_by_category("A");
    println!("\n类别A数据量: {}", category_a.len());
    
    // 时间窗口聚合示例
    let windows = analyzer.aggregate_by_time_window(1000);
    println!("\n时间窗口聚合结果数量: {}", windows.len());
}
```

### 文件处理

```rust
use rayon::prelude::*;
use std::fs;
use std::path::Path;

fn file_processing_demo() {
    // 并行读取目录
    let dir = ".";
    let entries: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    // 并行处理文件
    let file_info: Vec<_> = entries.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                let metadata = fs::metadata(&path).ok()?;
                Some(FileInfo {
                    path: path.to_string_lossy().to_string(),
                    size: metadata.len(),
                    extension: path.extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("无")
                        .to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    
    println!("文件处理结果:");
    for info in file_info {
        println!("  {}: {} bytes, 扩展名: {}", 
                 info.path, info.size, info.extension);
    }
    
    // 并行文本处理
    let text_files = vec!["file1.txt", "file2.txt", "file3.txt"];
    
    let word_counts: Vec<_> = text_files.par_iter()
        .filter_map(|filename| {
            // 这里应该是实际的文件读取，为演示使用模拟数据
            let content = match *filename {
                "file1.txt" => "hello world rust programming",
                "file2.txt" => "parallel computing with rayon",
                "file3.txt" => "data processing and analysis",
                _ => return None,
            };
            
            let word_count = content.split_whitespace().count();
            Some((*filename, word_count))
        })
        .collect();
    
    println!("\n文本文件单词统计:");
    for (filename, count) in word_counts {
        println!("  {}: {} 个单词", filename, count);
    }
}

#[derive(Debug)]
struct FileInfo {
    path: String,
    size: u64,
    extension: String,
}
```

## 最佳实践

### 1. 选择合适的并行度

```rust
use rayon::prelude::*;

fn choose_appropriate_parallelism() {
    let data: Vec<i32> = (0..1000000).collect();
    
    // 对于简单操作，使用默认并行度
    let sum: i32 = data.par_iter().sum();
    println!("默认并行求和: {}", sum);
    
    // 对于复杂操作，控制最小并行粒度
    let complex_sum: i32 = data.par_iter()
        .with_min_len(1000) // 每个任务至少处理1000个元素
        .map(|&x| expensive_computation(x))
        .sum();
    println!("复杂操作求和: {}", complex_sum);
    
    // 对于非常轻量的操作，考虑使用顺序处理
    let light_sum: i32 = data.iter().map(|&x| x + 1).sum();
    println!("轻量操作求和: {}", light_sum);
}

fn expensive_computation(x: i32) -> i32 {
    // 模拟复杂计算
    (0..x % 100).sum::<i32>()
}
```

### 2. 避免过度并行化

```rust
use rayon::prelude::*;

fn avoid_over_parallelization() {
    let small_data = vec![1, 2, 3, 4, 5];
    
    // 对于小数据集，顺序处理可能更快
    let sequential_sum: i32 = small_data.iter().sum();
    println!("顺序求和: {}", sequential_sum);
    
    // 并行处理的开销可能大于收益
    let parallel_sum: i32 = small_data.par_iter().sum();
    println!("并行求和: {}", parallel_sum);
    
    // 基准测试以确定阈值
    let threshold = 1000;
    let data: Vec<i32> = (0..threshold).collect();
    
    let start = std::time::Instant::now();
    let seq_result: i32 = data.iter().map(|&x| x * x).sum();
    let seq_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let par_result: i32 = data.par_iter().map(|&x| x * x).sum();
    let par_time = start.elapsed();
    
    println!("顺序处理: {} 耗时: {:?}", seq_result, seq_time);
    println!("并行处理: {} 耗时: {:?}", par_result, par_time);
}
```

### 3. 内存访问模式优化

```rust
use rayon::prelude::*;

fn optimize_memory_access() {
    let size = 1000;
    let mut matrix = vec![vec![0; size]; size];
    
    // 好的做法：按行访问（缓存友好）
    let start = std::time::Instant::now();
    matrix.par_iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, cell)| {
            *cell = i * j;
        });
    });
    println!("按行访问耗时: {:?}", start.elapsed());
    
    // 避免的做法：按列访问（缓存不友好）
    let start = std::time::Instant::now();
    (0..size).into_par_iter().for_each(|j| {
        for i in 0..size {
            matrix[i][j] = i * j;
        }
    });
    println!("按列访问耗时: {:?}", start.elapsed());
    
    // 块状访问模式
    let block_size = 64;
    let start = std::time::Instant::now();
    (0..size).into_par_iter().step_by(block_size).for_each(|block_i| {
        for block_j in (0..size).step_by(block_size) {
            for i in block_i..std::cmp::min(block_i + block_size, size) {
                for j in block_j..std::cmp::min(block_j + block_size, size) {
                    matrix[i][j] = i * j;
                }
            }
        }
    });
    println!("块状访问耗时: {:?}", start.elapsed());
}
```

### 4. 错误处理

```rust
use rayon::prelude::*;

fn error_handling_best_practices() {
    let data = vec![1, 2, 3, 4, 5, 0, 7, 8, 9, 10];
    
    // 使用 try_fold 进行错误处理
    let result: Result<i32, &str> = data.par_iter()
        .try_fold(
            || 0,
            |acc, &x| {
                if x == 0 {
                    Err("发现零值")
                } else {
                    Ok(acc + 100 / x)
                }
            }
        )
        .try_reduce(|| 0, |a, b| Ok(a + b));
    
    match result {
        Ok(sum) => println!("成功计算: {}", sum),
        Err(e) => println!("计算失败: {}", e),
    }
    
    // 使用 filter_map 跳过错误
    let safe_results: Vec<i32> = data.par_iter()
        .filter_map(|&x| {
            if x == 0 {
                None
            } else {
                Some(100 / x)
            }
        })
        .collect();
    
    println!("安全计算结果: {:?}", safe_results);
}
```

## 故障排除

### 常见问题诊断

```rust
use rayon::prelude::*;

fn troubleshooting_common_issues() {
    // 问题1: 数据竞争
    use std::sync::Mutex;
    
    let counter = Mutex::new(0);
    
    // 错误的做法：在并行迭代中使用锁
    let start = std::time::Instant::now();
    (0..10000).into_par_iter().for_each(|_| {
        let mut count = counter.lock().unwrap();
        *count += 1;
    });
    println!("使用锁耗时: {:?}", start.elapsed());
    
    // 正确的做法：使用 fold 和 reduce
    let start = std::time::Instant::now();
    let result = (0..10000).into_par_iter()
        .fold(|| 0, |acc, _| acc + 1)
        .sum::<i32>();
    println!("使用fold耗时: {:?}, 结果: {}", start.elapsed(), result);
    
    // 问题2: 不均匀的工作负载
    let uneven_data = vec![1, 100, 1, 100, 1, 100, 1, 100];
    
    // 使用 par_chunks 可能导致负载不均
    let start = std::time::Instant::now();
    let chunk_result: Vec<i32> = uneven_data.par_chunks(2)
        .map(|chunk| chunk.iter().map(|&x| expensive_work(x)).sum())
        .collect();
    println!("块处理耗时: {:?}, 结果: {:?}", start.elapsed(), chunk_result);
    
    // 使用 par_iter 让工作窃取处理负载均衡
    let start = std::time::Instant::now();
    let balanced_result: Vec<i32> = uneven_data.par_iter()
        .map(|&x| expensive_work(x))
        .collect();
    println!("平衡处理耗时: {:?}, 结果: {:?}", start.elapsed(), balanced_result);
}

fn expensive_work(x: i32) -> i32 {
    // 模拟工作量与输入成比例的计算
    (0..x * 100).sum::<i32>()
}
```

### 性能调试

```rust
use rayon::prelude::*;

fn performance_debugging() {
    let data: Vec<i32> = (0..1000000).collect();
    
    // 使用不同的并行策略进行对比
    let strategies = vec![
        ("顺序处理", Strategy::Sequential),
        ("默认并行", Strategy::Parallel),
        ("分块并行", Strategy::Chunked(1000)),
        ("最小粒度", Strategy::MinLen(10000)),
    ];
    
    for (name, strategy) in strategies {
        let start = std::time::Instant::now();
        let result = match strategy {
            Strategy::Sequential => {
                data.iter().map(|&x| x * x).sum::<i32>()
            }
            Strategy::Parallel => {
                data.par_iter().map(|&x| x * x).sum::<i32>()
            }
            Strategy::Chunked(size) => {
                data.par_chunks(size)
                    .map(|chunk| chunk.iter().map(|&x| x * x).sum::<i32>())
                    .sum::<i32>()
            }
            Strategy::MinLen(len) => {
                data.par_iter()
                    .with_min_len(len)
                    .map(|&x| x * x)
                    .sum::<i32>()
            }
        };
        
        let elapsed = start.elapsed();
        println!("{}: 结果 {}, 耗时 {:?}", name, result, elapsed);
    }
}

#[derive(Clone)]
enum Strategy {
    Sequential,
    Parallel,
    Chunked(usize),
    MinLen(usize),
}
```

## 总结

Rayon 是 Rust 中最优秀的并行编程库，提供了简单而强大的数据并行处理能力。通过本教程，您应该能够：

1. 理解工作窃取算法和数据并行模型
2. 熟练使用并行迭代器进行数据处理
3. 掌握任务调度和线程池管理
4. 实现高效的并行算法
5. 优化并行程序的性能

关键要点：
- 只需将 `.iter()` 改为 `.par_iter()` 即可获得并行处理
- 合理选择并行度，避免过度并行化
- 注意内存访问模式和缓存友好性
- 使用 `fold` 和 `reduce` 避免锁竞争
- 针对不同场景选择合适的并行策略

Rayon 的设计哲学是让并行编程变得简单安全，掌握它将大大提升您的 Rust 程序性能。在处理大数据、科学计算、图像处理等需要高性能的场景中，Rayon 是不可或缺的工具。