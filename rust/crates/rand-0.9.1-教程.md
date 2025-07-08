# Rand 0.9.1 - Rust 随机数生成完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [基本用法](#基本用法)
- [随机数生成器](#随机数生成器)
- [分布采样](#分布采样)
- [高级特性](#高级特性)
- [密码学安全](#密码学安全)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Rand 是 Rust 生态系统中最重要的随机数生成库，提供了高质量的随机数生成器、概率分布采样和随机性相关的工具。

### 核心特性
- **多种随机数生成器**: 提供密码学安全和高性能的 RNG
- **丰富的分布**: 支持各种概率分布的随机采样
- **线程安全**: 提供线程安全的全局随机数生成器
- **无分配**: 零分配的随机数生成
- **可序列化**: 支持 RNG 状态的序列化和反序列化

### 版本信息
- **当前版本**: 0.9.1
- **发布时间**: 2025-04-17
- **下载次数**: 582,315,521+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
rand = "0.9.1"

# 额外的分布支持
rand_distr = "0.4"

# 其他随机数生成器
rand_chacha = "0.3"
rand_pcg = "0.3"
```

### 基本示例

```rust
use rand::prelude::*;

fn main() {
    // 生成随机数
    let x: f64 = random();
    println!("随机浮点数: {}", x);
    
    let y: u32 = random();
    println!("随机整数: {}", y);
    
    // 使用线程本地随机数生成器
    let mut rng = thread_rng();
    
    // 生成范围内的随机数
    let n = rng.gen_range(1..=100);
    println!("1-100之间的随机数: {}", n);
    
    // 随机布尔值
    let coin_flip = rng.gen_bool(0.5);
    println!("掷硬币结果: {}", if coin_flip { "正面" } else { "反面" });
    
    // 从数组中随机选择
    let colors = ["红", "绿", "蓝", "黄"];
    let chosen_color = colors.choose(&mut rng).unwrap();
    println!("随机颜色: {}", chosen_color);
    
    // 打乱数组
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.shuffle(&mut rng);
    println!("打乱后的数字: {:?}", numbers);
}
```

## 核心概念

### Rng Trait

```rust
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;

fn rng_trait_demo() {
    // 所有随机数生成器都实现 RngCore trait
    let mut rng = StdRng::from_entropy();
    
    // 生成原始随机字节
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    println!("随机字节: {:?}", &bytes[..8]);
    
    // 生成 u32 和 u64
    let x = rng.next_u32();
    let y = rng.next_u64();
    println!("随机 u32: {}, u64: {}", x, y);
    
    // Rng trait 提供高级方法
    let range_value = rng.gen_range(10..20);
    println!("10-19之间的随机数: {}", range_value);
    
    let boolean = rng.gen::<bool>();
    println!("随机布尔值: {}", boolean);
}
```

### 种子和确定性

```rust
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

fn deterministic_random() {
    // 使用固定种子创建确定性随机数生成器
    let seed = [1u8; 32];
    let mut rng1 = StdRng::from_seed(seed);
    let mut rng2 = StdRng::from_seed(seed);
    
    // 两个生成器会产生相同的序列
    for i in 0..5 {
        let x1 = rng1.gen::<u32>();
        let x2 = rng2.gen::<u32>();
        println!("步骤 {}: rng1={}, rng2={}, 相等={}", i, x1, x2, x1 == x2);
    }
    
    // 从熵创建非确定性生成器
    let mut entropy_rng = StdRng::from_entropy();
    println!("从熵生成: {}", entropy_rng.gen::<u32>());
}
```

### 线程安全性

```rust
use rand::{thread_rng, random};
use std::thread;

fn thread_safety_demo() {
    // thread_rng() 返回线程本地的随机数生成器
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let mut rng = thread_rng();
                let value = rng.gen_range(1..100);
                println!("线程 {} 生成: {}", i, value);
                value
            })
        })
        .collect();
    
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("所有线程结果: {:?}", results);
    
    // random() 函数使用全局线程安全 RNG
    for i in 0..3 {
        println!("全局随机 {}: {}", i, random::<f64>());
    }
}
```

## 基本用法

### 生成基本类型

```rust
use rand::{thread_rng, Rng};

fn generate_basic_types() {
    let mut rng = thread_rng();
    
    // 整数类型
    let i8_val: i8 = rng.gen();
    let u16_val: u16 = rng.gen();
    let i32_val: i32 = rng.gen();
    let u64_val: u64 = rng.gen();
    println!("整数: i8={}, u16={}, i32={}, u64={}", i8_val, u16_val, i32_val, u64_val);
    
    // 浮点类型
    let f32_val: f32 = rng.gen(); // 0.0 到 1.0
    let f64_val: f64 = rng.gen(); // 0.0 到 1.0
    println!("浮点数: f32={:.4}, f64={:.6}", f32_val, f64_val);
    
    // 布尔值
    let bool_val: bool = rng.gen();
    println!("布尔值: {}", bool_val);
    
    // 字符
    let char_val: char = rng.gen();
    println!("随机字符: {}", char_val);
}
```

### 范围生成

```rust
use rand::{thread_rng, Rng};

fn range_generation() {
    let mut rng = thread_rng();
    
    // 整数范围 (不包含上界)
    let dice_roll = rng.gen_range(1..7);
    println!("骰子点数: {}", dice_roll);
    
    // 包含上界的范围
    let percentage = rng.gen_range(0..=100);
    println!("百分比: {}%", percentage);
    
    // 浮点数范围
    let temperature = rng.gen_range(-10.0..35.0);
    println!("温度: {:.1}°C", temperature);
    
    // 字符范围
    let letter = rng.gen_range('a'..='z');
    println!("小写字母: {}", letter);
    
    // 自定义范围函数
    fn random_in_range<T>(min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform + Copy,
    {
        thread_rng().gen_range(min..max)
    }
    
    println!("自定义范围: {}", random_in_range(10, 20));
}
```

### 序列操作

```rust
use rand::{thread_rng, seq::SliceRandom};

fn sequence_operations() {
    let mut rng = thread_rng();
    
    // 从切片中选择
    let fruits = ["苹果", "香蕉", "橙子", "葡萄", "草莓"];
    
    // 选择一个元素
    if let Some(fruit) = fruits.choose(&mut rng) {
        println!("随机水果: {}", fruit);
    }
    
    // 选择多个元素
    let selected: Vec<_> = fruits.choose_multiple(&mut rng, 3).collect();
    println!("选中的水果: {:?}", selected);
    
    // 加权选择
    let weights = [1, 2, 3, 4, 5]; // 对应水果的权重
    if let Ok(fruit) = fruits.choose_weighted(&mut rng, |item| {
        weights[fruits.iter().position(|&x| x == *item).unwrap()]
    }) {
        println!("加权选择的水果: {}", fruit);
    }
    
    // 打乱序列
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    numbers.shuffle(&mut rng);
    println!("打乱的数字: {:?}", numbers);
    
    // 部分打乱
    let mut deck: Vec<i32> = (1..=52).collect();
    deck.partial_shuffle(&mut rng, 5);
    println!("部分打乱的前5张牌: {:?}", &deck[..5]);
}
```

### 随机字符串生成

```rust
use rand::{thread_rng, Rng, distributions::Alphanumeric};

fn generate_random_strings() {
    let mut rng = thread_rng();
    
    // 生成随机字母数字字符串
    let random_string: String = (0..10)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    println!("随机字符串: {}", random_string);
    
    // 生成指定字符集的字符串
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let password: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    println!("随机密码: {}", password);
    
    // 生成UUID风格的字符串
    let uuid_like: String = format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        rng.gen::<u32>(),
        rng.gen::<u16>(),
        rng.gen::<u16>(),
        rng.gen::<u16>(),
        rng.gen::<u64>() & 0xffffffffffff
    );
    println!("UUID风格: {}", uuid_like);
    
    // 生成随机句子
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog"];
    let sentence_length = rng.gen_range(3..8);
    let sentence: Vec<_> = (0..sentence_length)
        .map(|_| words.choose(&mut rng).unwrap())
        .collect();
    println!("随机句子: {}", sentence.join(" "));
}
```

## 随机数生成器

### 不同类型的RNG

```rust
use rand::prelude::*;
use rand::rngs::{StdRng, SmallRng, ThreadRng};

fn different_rngs() {
    // StdRng: 密码学安全，高质量
    let mut std_rng = StdRng::from_entropy();
    println!("StdRng: {}", std_rng.gen::<u32>());
    
    // SmallRng: 快速，但不一定密码学安全
    let mut small_rng = SmallRng::from_entropy();
    println!("SmallRng: {}", small_rng.gen::<u32>());
    
    // ThreadRng: 线程本地，方便使用
    let mut thread_rng = thread_rng();
    println!("ThreadRng: {}", thread_rng.gen::<u32>());
    
    // 性能对比示例
    let iterations = 1_000_000;
    
    let start = std::time::Instant::now();
    let mut sum = 0u64;
    for _ in 0..iterations {
        sum += std_rng.gen::<u32>() as u64;
    }
    println!("StdRng 耗时: {:?}, 校验和: {}", start.elapsed(), sum);
    
    let start = std::time::Instant::now();
    let mut sum = 0u64;
    for _ in 0..iterations {
        sum += small_rng.gen::<u32>() as u64;
    }
    println!("SmallRng 耗时: {:?}, 校验和: {}", start.elapsed(), sum);
}
```

### 可重现的随机序列

```rust
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

fn reproducible_sequences() {
    // 创建可重现的随机序列
    let seed = 12345u64;
    let mut rng1 = StdRng::seed_from_u64(seed);
    let mut rng2 = StdRng::seed_from_u64(seed);
    
    println!("使用种子 {} 的两个生成器:", seed);
    for i in 0..5 {
        let val1 = rng1.gen_range(1..100);
        let val2 = rng2.gen_range(1..100);
        println!("  步骤 {}: {} == {} -> {}", i, val1, val2, val1 == val2);
    }
    
    // 保存和恢复RNG状态
    let checkpoint_rng = StdRng::seed_from_u64(67890);
    let saved_state = checkpoint_rng.clone();
    
    let mut working_rng = checkpoint_rng;
    println!("\n工作序列:");
    for i in 0..3 {
        println!("  值 {}: {}", i, working_rng.gen::<u32>());
    }
    
    // 从保存的状态恢复
    let mut restored_rng = saved_state;
    println!("恢复的序列:");
    for i in 0..3 {
        println!("  值 {}: {}", i, restored_rng.gen::<u32>());
    }
}
```

### 自定义RNG

```rust
use rand::{RngCore, Error};

// 简单的线性同余生成器
struct LinearCongruentialRng {
    state: u64,
}

impl LinearCongruentialRng {
    fn new(seed: u64) -> Self {
        LinearCongruentialRng {
            state: seed,
        }
    }
}

impl RngCore for LinearCongruentialRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    
    fn next_u64(&mut self) -> u64 {
        // 标准LCG参数
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand::impls::fill_bytes_via_next(self, dest);
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn custom_rng_demo() {
    use rand::Rng;
    
    let mut custom_rng = LinearCongruentialRng::new(1);
    
    println!("自定义RNG生成的数字:");
    for i in 0..10 {
        let value = custom_rng.gen_range(1..100);
        println!("  {}: {}", i, value);
    }
    
    // 注意：这个简单的LCG不适合生产使用
    println!("警告: 这个简单的LCG仅用于演示，实际应用请使用经过验证的RNG");
}
```

## 分布采样

### 常用分布

```rust
use rand::prelude::*;
use rand_distr::*;

fn common_distributions() {
    let mut rng = thread_rng();
    
    // 正态分布
    let normal = Normal::new(0.0, 1.0).unwrap();
    let normal_samples: Vec<f64> = (0..5)
        .map(|_| normal.sample(&mut rng))
        .collect();
    println!("正态分布样本: {:?}", normal_samples);
    
    // 均匀分布
    let uniform = Uniform::from(10..20);
    let uniform_samples: Vec<i32> = (0..5)
        .map(|_| uniform.sample(&mut rng))
        .collect();
    println!("均匀分布样本: {:?}", uniform_samples);
    
    // 指数分布
    let exponential = Exp::new(1.0).unwrap();
    let exp_samples: Vec<f64> = (0..5)
        .map(|_| exponential.sample(&mut rng))
        .collect();
    println!("指数分布样本: {:?}", exp_samples);
    
    // 泊松分布
    let poisson = Poisson::new(3.0).unwrap();
    let poisson_samples: Vec<f64> = (0..5)
        .map(|_| poisson.sample(&mut rng))
        .collect();
    println!("泊松分布样本: {:?}", poisson_samples);
    
    // 伽马分布
    let gamma = Gamma::new(2.0, 3.0).unwrap();
    let gamma_samples: Vec<f64> = (0..5)
        .map(|_| gamma.sample(&mut rng))
        .collect();
    println!("伽马分布样本: {:?}", gamma_samples);
}
```

### 离散分布

```rust
use rand::prelude::*;
use rand_distr::*;

fn discrete_distributions() {
    let mut rng = thread_rng();
    
    // 伯努利分布
    let bernoulli = Bernoulli::new(0.3).unwrap();
    let bernoulli_samples: Vec<bool> = (0..10)
        .map(|_| bernoulli.sample(&mut rng))
        .collect();
    println!("伯努利分布 (p=0.3): {:?}", bernoulli_samples);
    
    // 二项分布
    let binomial = Binomial::new(10, 0.5).unwrap();
    let binomial_samples: Vec<u64> = (0..5)
        .map(|_| binomial.sample(&mut rng))
        .collect();
    println!("二项分布 (n=10, p=0.5): {:?}", binomial_samples);
    
    // 几何分布
    let geometric = Geometric::new(0.1).unwrap();
    let geometric_samples: Vec<u64> = (0..5)
        .map(|_| geometric.sample(&mut rng))
        .collect();
    println!("几何分布 (p=0.1): {:?}", geometric_samples);
    
    // 加权选择分布
    let weights = [10, 20, 30, 40];
    let weighted = WeightedIndex::new(&weights).unwrap();
    let weighted_samples: Vec<usize> = (0..10)
        .map(|_| weighted.sample(&mut rng))
        .collect();
    println!("加权分布: {:?}", weighted_samples);
    
    // 自定义离散分布
    let items = ["稀有", "普通", "常见"];
    let probabilities = [0.1, 0.3, 0.6];
    let dist = WeightedIndex::new(&probabilities).unwrap();
    
    let mut counts = [0; 3];
    for _ in 0..1000 {
        let index = dist.sample(&mut rng);
        counts[index] += 1;
    }
    
    println!("1000次采样结果:");
    for (i, (item, count)) in items.iter().zip(counts.iter()).enumerate() {
        let expected = probabilities[i] * 1000.0;
        println!("  {}: {} (期望: {:.0})", item, count, expected);
    }
}
```

### 多维分布

```rust
use rand::prelude::*;
use rand_distr::*;

fn multidimensional_distributions() {
    let mut rng = thread_rng();
    
    // 二维正态分布
    let normal_2d = Normal::new(0.0, 1.0).unwrap();
    let points_2d: Vec<(f64, f64)> = (0..5)
        .map(|_| (normal_2d.sample(&mut rng), normal_2d.sample(&mut rng)))
        .collect();
    println!("二维正态分布点: {:?}", points_2d);
    
    // 单位圆内的均匀分布
    let unit_circle: Vec<(f64, f64)> = (0..5)
        .map(|_| {
            let UnitCircle(x, y) = UnitCircle.sample(&mut rng);
            (x, y)
        })
        .collect();
    println!("单位圆内的点: {:?}", unit_circle);
    
    // 单位球面上的均匀分布
    let unit_sphere: Vec<[f64; 3]> = (0..3)
        .map(|_| UnitSphere.sample(&mut rng))
        .collect();
    println!("单位球面上的点: {:?}", unit_sphere);
    
    // 自定义多元分布：在正方形中随机选择点
    fn random_square_point(rng: &mut impl Rng, size: f64) -> (f64, f64) {
        let x = rng.gen_range(-size/2.0..size/2.0);
        let y = rng.gen_range(-size/2.0..size/2.0);
        (x, y)
    }
    
    let square_points: Vec<(f64, f64)> = (0..5)
        .map(|_| random_square_point(&mut rng, 10.0))
        .collect();
    println!("正方形内的点: {:?}", square_points);
}
```

## 高级特性

### 快速随机数生成

```rust
use rand::prelude::*;
use rand::rngs::SmallRng;

fn fast_random_generation() {
    // 使用SmallRng获得更好的性能
    let mut fast_rng = SmallRng::from_entropy();
    
    // 批量生成随机数
    let batch_size = 1000;
    let start = std::time::Instant::now();
    
    let random_numbers: Vec<u32> = (0..batch_size)
        .map(|_| fast_rng.gen())
        .collect();
    
    println!("生成 {} 个随机数耗时: {:?}", batch_size, start.elapsed());
    println!("前10个数字: {:?}", &random_numbers[..10]);
    
    // 使用fill_bytes获得更好的性能
    let start = std::time::Instant::now();
    let mut bytes = vec![0u8; batch_size * 4]; // 4 bytes per u32
    fast_rng.fill_bytes(&mut bytes);
    println!("填充 {} 字节耗时: {:?}", bytes.len(), start.elapsed());
    
    // 转换为u32数组
    let u32_array: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    println!("前10个转换的数字: {:?}", &u32_array[..10]);
}
```

### 随机数流

```rust
use rand::prelude::*;
use rand::rngs::StdRng;

struct RandomStream {
    rng: StdRng,
    buffer: Vec<u8>,
    position: usize,
}

impl RandomStream {
    fn new(seed: u64) -> Self {
        RandomStream {
            rng: StdRng::seed_from_u64(seed),
            buffer: vec![0u8; 1024], // 1KB buffer
            position: 1024, // Force initial fill
        }
    }
    
    fn next_byte(&mut self) -> u8 {
        if self.position >= self.buffer.len() {
            self.rng.fill_bytes(&mut self.buffer);
            self.position = 0;
        }
        
        let byte = self.buffer[self.position];
        self.position += 1;
        byte
    }
    
    fn next_u32(&mut self) -> u32 {
        let b1 = self.next_byte() as u32;
        let b2 = self.next_byte() as u32;
        let b3 = self.next_byte() as u32;
        let b4 = self.next_byte() as u32;
        
        (b4 << 24) | (b3 << 16) | (b2 << 8) | b1
    }
    
    fn next_float(&mut self) -> f32 {
        // 将u32转换为[0, 1)范围的浮点数
        self.next_u32() as f32 / (u32::MAX as f32 + 1.0)
    }
}

fn random_stream_demo() {
    let mut stream = RandomStream::new(42);
    
    println!("随机字节流:");
    for i in 0..10 {
        println!("  字节 {}: {}", i, stream.next_byte());
    }
    
    println!("随机整数流:");
    for i in 0..5 {
        println!("  整数 {}: {}", i, stream.next_u32());
    }
    
    println!("随机浮点数流:");
    for i in 0..5 {
        println!("  浮点 {}: {:.6}", i, stream.next_float());
    }
}
```

### 并行随机数生成

```rust
use rand::prelude::*;
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};
use std::thread;

fn parallel_random_generation() {
    // 方法1: 每个线程使用独立的RNG
    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            thread::spawn(move || {
                let mut rng = StdRng::seed_from_u64(thread_id as u64);
                let numbers: Vec<u32> = (0..5)
                    .map(|_| rng.gen_range(1..100))
                    .collect();
                println!("线程 {} 生成: {:?}", thread_id, numbers);
                numbers
            })
        })
        .collect();
    
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("所有线程结果: {:?}", results);
    
    // 方法2: 共享RNG (不推荐，性能差)
    let shared_rng = Arc::new(Mutex::new(StdRng::from_entropy()));
    
    let handles: Vec<_> = (0..3)
        .map(|thread_id| {
            let rng = shared_rng.clone();
            thread::spawn(move || {
                let value = {
                    let mut rng = rng.lock().unwrap();
                    rng.gen_range(1..100)
                };
                println!("共享RNG线程 {} 生成: {}", thread_id, value);
                value
            })
        })
        .collect();
    
    let shared_results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("共享RNG结果: {:?}", shared_results);
}
```

## 密码学安全

### 密码学安全的随机数

```rust
use rand::prelude::*;
use rand::rngs::OsRng;

fn cryptographically_secure_random() {
    // OsRng 使用操作系统的密码学安全随机数生成器
    let mut os_rng = OsRng;
    
    // 生成密码学安全的随机数
    let secure_u64 = os_rng.gen::<u64>();
    println!("密码学安全的随机数: {}", secure_u64);
    
    // 生成随机密钥
    let mut key = [0u8; 32]; // 256-bit key
    os_rng.fill_bytes(&mut key);
    println!("随机密钥 (hex): {}", hex::encode(&key));
    
    // 生成随机IV
    let mut iv = [0u8; 16]; // 128-bit IV
    os_rng.fill_bytes(&mut iv);
    println!("随机IV (hex): {}", hex::encode(&iv));
    
    // 生成随机盐值
    let mut salt = [0u8; 16];
    os_rng.fill_bytes(&mut salt);
    println!("随机盐值 (hex): {}", hex::encode(&salt));
}

// 注意：需要在Cargo.toml中添加hex依赖
// hex = "0.4"
```

### 安全令牌生成

```rust
use rand::prelude::*;
use rand::rngs::OsRng;
use rand::distributions::Alphanumeric;

fn generate_secure_tokens() {
    let mut rng = OsRng;
    
    // 生成安全的API密钥
    let api_key: String = (0..32)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    println!("API密钥: {}", api_key);
    
    // 生成会话ID
    let session_id: String = (0..64)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    println!("会话ID: {}", session_id);
    
    // 生成随机密码
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
    let password: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    println!("随机密码: {}", password);
    
    // 生成验证码
    let verification_code: String = (0..6)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    println!("验证码: {}", verification_code);
    
    // 生成UUID v4
    let uuid = format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        rng.gen::<u32>(),
        rng.gen::<u16>(),
        rng.gen::<u16>() & 0x0fff,
        (rng.gen::<u16>() & 0x3fff) | 0x8000,
        rng.gen::<u64>() & 0xffffffffffff
    );
    println!("UUID v4: {}", uuid);
}
```

## 性能优化

### 批量生成优化

```rust
use rand::prelude::*;
use rand::rngs::SmallRng;

fn batch_generation_optimization() {
    let mut rng = SmallRng::from_entropy();
    let n = 1_000_000;
    
    // 方法1: 逐个生成 (较慢)
    let start = std::time::Instant::now();
    let individual: Vec<u32> = (0..n).map(|_| rng.gen()).collect();
    let individual_time = start.elapsed();
    
    // 方法2: 批量填充字节 (更快)
    let start = std::time::Instant::now();
    let mut bytes = vec![0u8; n * 4];
    rng.fill_bytes(&mut bytes);
    let batch_bytes: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    let batch_time = start.elapsed();
    
    println!("逐个生成 {} 个数字耗时: {:?}", n, individual_time);
    println!("批量生成 {} 个数字耗时: {:?}", n, batch_time);
    println!("性能提升: {:.2}x", individual_time.as_nanos() as f64 / batch_time.as_nanos() as f64);
    
    // 验证结果长度
    assert_eq!(individual.len(), batch_bytes.len());
}
```

### 内存友好的生成

```rust
use rand::prelude::*;
use rand::rngs::SmallRng;

fn memory_friendly_generation() {
    let mut rng = SmallRng::from_entropy();
    
    // 使用迭代器避免分配大型集合
    let sum: u64 = (0..1_000_000)
        .map(|_| rng.gen::<u32>() as u64)
        .sum();
    
    println!("1百万随机数的和: {}", sum);
    
    // 流式处理大量随机数
    let mut count = 0;
    let mut sum = 0u64;
    
    for _ in 0..1_000_000 {
        let value = rng.gen::<u32>();
        if value % 2 == 0 {
            count += 1;
            sum += value as u64;
        }
    }
    
    println!("偶数个数: {}, 偶数和: {}", count, sum);
    
    // 分块处理
    const CHUNK_SIZE: usize = 10000;
    let mut total_sum = 0u64;
    
    for chunk_id in 0..100 {
        let chunk_sum: u64 = (0..CHUNK_SIZE)
            .map(|_| rng.gen::<u32>() as u64)
            .sum();
        total_sum += chunk_sum;
        
        if chunk_id % 10 == 0 {
            println!("处理块 {}: 当前总和 {}", chunk_id, total_sum);
        }
    }
    
    println!("最终总和: {}", total_sum);
}
```

### RNG选择指南

```rust
use rand::prelude::*;
use rand::rngs::{StdRng, SmallRng, ThreadRng, OsRng};

fn rng_selection_guide() {
    println!("RNG选择指南:");
    println!();
    
    // StdRng - 密码学安全，高质量
    println!("1. StdRng (ChaCha20):");
    println!("   - 用途: 需要密码学安全性的场景");
    println!("   - 优点: 安全性高，质量好，可重现");
    println!("   - 缺点: 相对较慢");
    
    // SmallRng - 快速，小状态
    println!("2. SmallRng:");
    println!("   - 用途: 游戏、模拟、性能关键的应用");
    println!("   - 优点: 速度快，状态小");
    println!("   - 缺点: 可能不够安全用于密码学");
    
    // ThreadRng - 便利的全局RNG
    println!("3. ThreadRng:");
    println!("   - 用途: 一般用途，便利性");
    println!("   - 优点: 线程安全，自动播种");
    println!("   - 缺点: 不可重现");
    
    // OsRng - 系统熵源
    println!("4. OsRng:");
    println!("   - 用途: 生成密钥、种子、密码学应用");
    println!("   - 优点: 最高安全性");
    println!("   - 缺点: 可能阻塞，较慢");
    
    // 性能比较
    let iterations = 100_000;
    
    // StdRng
    let mut std_rng = StdRng::from_entropy();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std_rng.gen::<u32>();
    }
    println!("\nStdRng {} 次生成耗时: {:?}", iterations, start.elapsed());
    
    // SmallRng
    let mut small_rng = SmallRng::from_entropy();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        small_rng.gen::<u32>();
    }
    println!("SmallRng {} 次生成耗时: {:?}", iterations, start.elapsed());
    
    // ThreadRng
    let mut thread_rng = thread_rng();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        thread_rng.gen::<u32>();
    }
    println!("ThreadRng {} 次生成耗时: {:?}", iterations, start.elapsed());
}
```

## 实战案例

### 游戏开发中的随机系统

```rust
use rand::prelude::*;
use rand_distr::*;

struct GameRandom {
    rng: SmallRng,
}

impl GameRandom {
    fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => SmallRng::seed_from_u64(s),
            None => SmallRng::from_entropy(),
        };
        
        GameRandom { rng }
    }
    
    // 暴击判定
    fn is_critical_hit(&mut self, crit_chance: f64) -> bool {
        self.rng.gen_bool(crit_chance)
    }
    
    // 伤害计算
    fn calculate_damage(&mut self, base_damage: f64, variance: f64) -> f64 {
        let normal = Normal::new(base_damage, variance).unwrap();
        normal.sample(&mut self.rng).max(0.0)
    }
    
    // 掉落物品
    fn generate_loot(&mut self) -> Option<&'static str> {
        let items = [
            ("普通剑", 0.4),
            ("魔法剑", 0.25),
            ("稀有剑", 0.15),
            ("史诗剑", 0.08),
            ("传说剑", 0.02),
        ];
        
        let total_weight: f64 = items.iter().map(|(_, w)| w).sum();
        let roll = self.rng.gen::<f64>() * total_weight;
        
        let mut cumulative = 0.0;
        for (item, weight) in &items {
            cumulative += weight;
            if roll <= cumulative {
                return Some(item);
            }
        }
        
        None
    }
    
    // 随机遭遇
    fn random_encounter(&mut self) -> &'static str {
        let encounters = ["哥布林", "兽人", "巨魔", "龙", "宝箱"];
        encounters.choose(&mut self.rng).unwrap()
    }
    
    // 随机地牢生成
    fn generate_dungeon_layout(&mut self, size: usize) -> Vec<Vec<char>> {
        let mut layout = vec![vec!['#'; size]; size]; // 墙壁
        
        // 生成房间
        for _ in 0..self.rng.gen_range(3..8) {
            let room_size = self.rng.gen_range(3..8);
            let x = self.rng.gen_range(1..size-room_size);
            let y = self.rng.gen_range(1..size-room_size);
            
            for i in x..x+room_size {
                for j in y..y+room_size {
                    layout[i][j] = '.'; // 地板
                }
            }
        }
        
        // 放置特殊物品
        for _ in 0..self.rng.gen_range(1..4) {
            let x = self.rng.gen_range(1..size-1);
            let y = self.rng.gen_range(1..size-1);
            if layout[x][y] == '.' {
                layout[x][y] = 'T'; // 宝箱
            }
        }
        
        layout
    }
}

fn game_random_demo() {
    let mut game_rng = GameRandom::new(Some(12345));
    
    // 战斗系统演示
    println!("=== 战斗系统 ===");
    for round in 1..=5 {
        let base_damage = 50.0;
        let damage = game_rng.calculate_damage(base_damage, 10.0);
        let is_crit = game_rng.is_critical_hit(0.15);
        
        let final_damage = if is_crit { damage * 2.0 } else { damage };
        
        println!("回合 {}: 伤害 {:.1}{}", 
                round, final_damage, if is_crit { " (暴击!)" } else { "" });
    }
    
    // 掉落系统演示
    println!("\n=== 掉落系统 ===");
    let mut loot_counts = std::collections::HashMap::new();
    for _ in 0..100 {
        if let Some(item) = game_rng.generate_loot() {
            *loot_counts.entry(item).or_insert(0) += 1;
        }
    }
    
    for (item, count) in &loot_counts {
        println!("{}: {} 次", item, count);
    }
    
    // 随机遭遇演示
    println!("\n=== 随机遭遇 ===");
    for i in 1..=5 {
        let encounter = game_rng.random_encounter();
        println!("遭遇 {}: {}", i, encounter);
    }
    
    // 地牢生成演示
    println!("\n=== 地牢布局 ===");
    let dungeon = game_rng.generate_dungeon_layout(15);
    for row in &dungeon {
        println!("{}", row.iter().collect::<String>());
    }
}
```

### 数据生成和测试

```rust
use rand::prelude::*;
use rand_distr::*;

struct TestDataGenerator {
    rng: StdRng,
}

impl TestDataGenerator {
    fn new(seed: u64) -> Self {
        TestDataGenerator {
            rng: StdRng::seed_from_u64(seed),
        }
    }
    
    // 生成用户数据
    fn generate_user(&mut self) -> User {
        let first_names = ["张", "李", "王", "刘", "陈", "杨", "赵", "黄", "周", "吴"];
        let last_names = ["伟", "芳", "娜", "敏", "静", "丽", "强", "磊", "军", "洋"];
        let domains = ["example.com", "test.org", "demo.net"];
        
        let first_name = first_names.choose(&mut self.rng).unwrap();
        let last_name = last_names.choose(&mut self.rng).unwrap();
        let name = format!("{}{}", first_name, last_name);
        
        let age_dist = Normal::new(35.0, 12.0).unwrap();
        let age = age_dist.sample(&mut self.rng).max(18.0).min(80.0) as u32;
        
        let email = format!("{}{}@{}", 
                           name.to_lowercase(),
                           self.rng.gen_range(1..1000),
                           domains.choose(&mut self.rng).unwrap());
        
        let salary_dist = LogNormal::new(10.5, 0.5).unwrap();
        let salary = salary_dist.sample(&mut self.rng).round() as u32;
        
        User { name, age, email, salary }
    }
    
    // 生成时间序列数据
    fn generate_time_series(&mut self, days: usize) -> Vec<f64> {
        let mut values = Vec::with_capacity(days);
        let mut current_value = 100.0;
        
        let noise_dist = Normal::new(0.0, 5.0).unwrap();
        let trend_dist = Normal::new(0.1, 2.0).unwrap();
        
        for _ in 0..days {
            // 添加趋势和噪声
            let trend = trend_dist.sample(&mut self.rng);
            let noise = noise_dist.sample(&mut self.rng);
            
            current_value += trend + noise;
            current_value = current_value.max(0.0); // 确保非负
            
            values.push(current_value);
        }
        
        values
    }
    
    // 生成网络流量数据
    fn generate_network_traffic(&mut self, hours: usize) -> Vec<NetworkMetric> {
        let mut metrics = Vec::with_capacity(hours);
        
        for hour in 0..hours {
            // 模拟日周期性
            let base_traffic = 50.0 + 30.0 * (hour as f64 * std::f64::consts::PI / 12.0).sin();
            
            // 添加随机变化
            let traffic_dist = Normal::new(base_traffic, 10.0).unwrap();
            let bytes_sent = traffic_dist.sample(&mut self.rng).max(0.0) as u64;
            let bytes_received = traffic_dist.sample(&mut self.rng).max(0.0) as u64;
            
            // 错误率随机变化
            let error_rate = self.rng.gen_range(0.0..0.05);
            
            metrics.push(NetworkMetric {
                hour,
                bytes_sent,
                bytes_received,
                error_rate,
            });
        }
        
        metrics
    }
    
    // 生成测试文本
    fn generate_text(&mut self, word_count: usize) -> String {
        let words = [
            "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
            "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
            "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
        ];
        
        (0..word_count)
            .map(|_| words.choose(&mut self.rng).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
    salary: u32,
}

#[derive(Debug)]
struct NetworkMetric {
    hour: usize,
    bytes_sent: u64,
    bytes_received: u64,
    error_rate: f64,
}

fn test_data_generation_demo() {
    let mut generator = TestDataGenerator::new(54321);
    
    // 生成用户数据
    println!("=== 测试用户数据 ===");
    for i in 1..=5 {
        let user = generator.generate_user();
        println!("用户 {}: {:?}", i, user);
    }
    
    // 生成时间序列数据
    println!("\n=== 时间序列数据 ===");
    let time_series = generator.generate_time_series(10);
    for (day, value) in time_series.iter().enumerate() {
        println!("第 {} 天: {:.2}", day + 1, value);
    }
    
    // 生成网络流量数据
    println!("\n=== 网络流量数据 ===");
    let network_data = generator.generate_network_traffic(5);
    for metric in &network_data {
        println!("小时 {}: 发送 {}MB, 接收 {}MB, 错误率 {:.2}%",
                metric.hour,
                metric.bytes_sent / (1024 * 1024),
                metric.bytes_received / (1024 * 1024),
                metric.error_rate * 100.0);
    }
    
    // 生成测试文本
    println!("\n=== 测试文本 ===");
    let text = generator.generate_text(20);
    println!("{}", text);
}
```

### 蒙特卡罗模拟

```rust
use rand::prelude::*;
use rand_distr::*;

struct MonteCarloSimulation {
    rng: StdRng,
}

impl MonteCarloSimulation {
    fn new(seed: u64) -> Self {
        MonteCarloSimulation {
            rng: StdRng::seed_from_u64(seed),
        }
    }
    
    // 估算π值
    fn estimate_pi(&mut self, samples: usize) -> f64 {
        let mut inside_circle = 0;
        
        for _ in 0..samples {
            let x: f64 = self.rng.gen_range(-1.0..1.0);
            let y: f64 = self.rng.gen_range(-1.0..1.0);
            
            if x * x + y * y <= 1.0 {
                inside_circle += 1;
            }
        }
        
        4.0 * (inside_circle as f64) / (samples as f64)
    }
    
    // 股票价格模拟 (几何布朗运动)
    fn simulate_stock_price(&mut self, 
                           initial_price: f64, 
                           mu: f64,        // 漂移率
                           sigma: f64,     // 波动率
                           days: usize) -> Vec<f64> {
        let mut prices = vec![initial_price];
        let dt = 1.0 / 365.0; // 日时间步长
        let normal = Normal::new(0.0, 1.0).unwrap();
        
        for _ in 1..days {
            let last_price = prices.last().unwrap();
            let random_shock = normal.sample(&mut self.rng);
            
            let price_change = mu * last_price * dt + 
                              sigma * last_price * random_shock * dt.sqrt();
            
            let new_price = last_price + price_change;
            prices.push(new_price.max(0.01)); // 确保价格为正
        }
        
        prices
    }
    
    // 期权定价 (Black-Scholes 蒙特卡罗)
    fn option_pricing(&mut self,
                     spot_price: f64,    // 当前价格
                     strike_price: f64,  // 执行价格
                     risk_free_rate: f64, // 无风险利率
                     volatility: f64,    // 波动率
                     time_to_expiry: f64, // 到期时间
                     simulations: usize) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut payoffs = 0.0;
        
        for _ in 0..simulations {
            let random_value = normal.sample(&mut self.rng);
            
            // 几何布朗运动的解析解
            let stock_price = spot_price * 
                ((risk_free_rate - 0.5 * volatility * volatility) * time_to_expiry + 
                 volatility * time_to_expiry.sqrt() * random_value).exp();
            
            // 看涨期权的收益
            let payoff = (stock_price - strike_price).max(0.0);
            payoffs += payoff;
        }
        
        // 贴现到现值
        let option_price = (payoffs / simulations as f64) * 
                          (-risk_free_rate * time_to_expiry).exp();
        
        option_price
    }
    
    // 排队系统模拟
    fn simulate_queue(&mut self, 
                     arrival_rate: f64,    // 到达率 (每分钟)
                     service_rate: f64,    // 服务率 (每分钟)
                     simulation_time: f64) -> QueueStats {
        let arrival_dist = Exp::new(arrival_rate).unwrap();
        let service_dist = Exp::new(service_rate).unwrap();
        
        let mut current_time = 0.0;
        let mut queue_length = 0;
        let mut total_wait_time = 0.0;
        let mut customers_served = 0;
        let mut service_end_time = 0.0;
        
        while current_time < simulation_time {
            // 下一个到达时间
            let next_arrival = current_time + arrival_dist.sample(&mut self.rng);
            
            if next_arrival < simulation_time {
                current_time = next_arrival;
                
                if current_time >= service_end_time {
                    // 服务器空闲，立即开始服务
                    let service_time = service_dist.sample(&mut self.rng);
                    service_end_time = current_time + service_time;
                    customers_served += 1;
                } else {
                    // 需要排队
                    queue_length += 1;
                    let wait_time = service_end_time - current_time;
                    total_wait_time += wait_time;
                    
                    // 开始服务
                    let service_time = service_dist.sample(&mut self.rng);
                    service_end_time += service_time;
                    queue_length -= 1;
                    customers_served += 1;
                }
            } else {
                break;
            }
        }
        
        QueueStats {
            customers_served,
            average_wait_time: if customers_served > 0 { 
                total_wait_time / customers_served as f64 
            } else { 
                0.0 
            },
            utilization: (simulation_time - (simulation_time - service_end_time).max(0.0)) / simulation_time,
        }
    }
}

#[derive(Debug)]
struct QueueStats {
    customers_served: usize,
    average_wait_time: f64,
    utilization: f64,
}

fn monte_carlo_demo() {
    let mut sim = MonteCarloSimulation::new(98765);
    
    // π值估算
    println!("=== π值估算 ===");
    for &samples in &[1000, 10000, 100000, 1000000] {
        let pi_estimate = sim.estimate_pi(samples);
        let error = (pi_estimate - std::f64::consts::PI).abs();
        println!("{} 样本: π ≈ {:.6}, 误差: {:.6}", samples, pi_estimate, error);
    }
    
    // 股票价格模拟
    println!("\n=== 股票价格模拟 ===");
    let stock_prices = sim.simulate_stock_price(100.0, 0.05, 0.2, 30);
    println!("初始价格: {:.2}", stock_prices[0]);
    println!("30天后价格: {:.2}", stock_prices.last().unwrap());
    println!("最高价格: {:.2}", stock_prices.iter().fold(0.0f64, |a, &b| a.max(b)));
    println!("最低价格: {:.2}", stock_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
    
    // 期权定价
    println!("\n=== 期权定价 ===");
    let option_price = sim.option_pricing(100.0, 105.0, 0.05, 0.2, 0.25, 100000);
    println!("看涨期权价格: {:.4}", option_price);
    
    // 排队系统模拟
    println!("\n=== 排队系统模拟 ===");
    let queue_stats = sim.simulate_queue(2.0, 3.0, 480.0); // 8小时
    println!("排队统计: {:?}", queue_stats);
}
```

## 最佳实践

### 1. 选择合适的RNG

```rust
use rand::prelude::*;
use rand::rngs::{StdRng, SmallRng, OsRng};

fn rng_selection_best_practices() {
    // 用于密码学：使用OsRng或StdRng
    let mut crypto_rng = OsRng;
    let mut key = [0u8; 32];
    crypto_rng.fill_bytes(&mut key);
    
    // 用于游戏和模拟：使用SmallRng
    let mut game_rng = SmallRng::from_entropy();
    let dice_roll = game_rng.gen_range(1..=6);
    
    // 用于测试：使用固定种子的StdRng
    let mut test_rng = StdRng::seed_from_u64(12345);
    let test_value = test_rng.gen::<f64>();
    
    println!("密码学密钥已生成");
    println!("游戏骰子点数: {}", dice_roll);
    println!("测试值: {:.6}", test_value);
}
```

### 2. 处理边界情况

```rust
use rand::prelude::*;
use rand_distr::*;

fn handle_edge_cases() {
    let mut rng = thread_rng();
    
    // 安全的范围生成
    fn safe_range(min: i32, max: i32) -> Option<i32> {
        if min >= max {
            return None;
        }
        Some(thread_rng().gen_range(min..max))
    }
    
    // 安全的分布采样
    fn safe_normal_sample(mean: f64, std_dev: f64) -> Option<f64> {
        if std_dev <= 0.0 {
            return None;
        }
        
        Normal::new(mean, std_dev)
            .ok()
            .map(|dist| dist.sample(&mut thread_rng()))
    }
    
    // 安全的序列选择
    fn safe_choose<T>(items: &[T]) -> Option<&T> {
        if items.is_empty() {
            return None;
        }
        items.choose(&mut thread_rng())
    }
    
    // 测试边界情况
    println!("有效范围: {:?}", safe_range(1, 10));
    println!("无效范围: {:?}", safe_range(10, 1));
    
    println!("有效正态分布: {:?}", safe_normal_sample(0.0, 1.0));
    println!("无效正态分布: {:?}", safe_normal_sample(0.0, -1.0));
    
    let items = vec![1, 2, 3];
    let empty: Vec<i32> = vec![];
    println!("非空选择: {:?}", safe_choose(&items));
    println!("空序列选择: {:?}", safe_choose(&empty));
}
```

### 3. 性能优化技巧

```rust
use rand::prelude::*;
use rand::rngs::SmallRng;

fn performance_tips() {
    // 1. 重用RNG实例
    let mut rng = SmallRng::from_entropy();
    
    // 2. 批量生成
    let mut buffer = [0u8; 1024];
    rng.fill_bytes(&mut buffer);
    
    // 3. 避免重复创建分布
    let uniform_dist = rand::distributions::Uniform::new(0, 100);
    let values: Vec<i32> = (0..1000)
        .map(|_| uniform_dist.sample(&mut rng))
        .collect();
    
    // 4. 对于简单用例，使用gen_range
    let simple_random = rng.gen_range(1..100);
    
    println!("生成了 {} 个值", values.len());
    println!("简单随机数: {}", simple_random);
}
```

### 4. 测试和调试

```rust
use rand::prelude::*;
use rand::rngs::StdRng;

fn testing_and_debugging() {
    // 使用固定种子进行可重现的测试
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);
    
    // 测试随机函数的分布
    fn test_distribution() {
        let mut rng = StdRng::seed_from_u64(123);
        let samples = 10000;
        let mut counts = [0; 10];
        
        for _ in 0..samples {
            let value = rng.gen_range(0..10);
            counts[value] += 1;
        }
        
        println!("分布测试 (期望每个约 {}):", samples / 10);
        for (i, &count) in counts.iter().enumerate() {
            println!("  {}: {}", i, count);
        }
        
        // 检查分布是否合理
        let expected = samples / 10;
        let max_deviation = expected / 5; // 20% 容差
        
        for (i, &count) in counts.iter().enumerate() {
            let deviation = (count as i32 - expected as i32).abs();
            if deviation > max_deviation as i32 {
                println!("警告: 值 {} 的分布偏差较大: {}", i, deviation);
            }
        }
    }
    
    test_distribution();
    
    // 记录随机数种子以便重现
    println!("使用种子 {} 进行测试", seed);
    for i in 0..5 {
        println!("值 {}: {}", i, rng.gen::<u32>());
    }
}
```

### 5. 错误处理

```rust
use rand::prelude::*;
use rand_distr::*;

fn error_handling_best_practices() {
    let mut rng = thread_rng();
    
    // 安全地创建分布
    match Normal::new(0.0, 1.0) {
        Ok(dist) => {
            let sample = dist.sample(&mut rng);
            println!("正态分布样本: {:.4}", sample);
        }
        Err(e) => {
            eprintln!("创建正态分布失败: {}", e);
        }
    }
    
    // 处理加权选择错误
    let items = ["a", "b", "c"];
    let weights = [1.0, 2.0, 3.0];
    
    match WeightedIndex::new(&weights) {
        Ok(dist) => {
            let index = dist.sample(&mut rng);
            println!("加权选择: {}", items[index]);
        }
        Err(e) => {
            eprintln!("创建加权分布失败: {}", e);
        }
    }
    
    // 处理空序列
    let empty_vec: Vec<i32> = vec![];
    match empty_vec.choose(&mut rng) {
        Some(value) => println!("选择的值: {}", value),
        None => println!("空序列，无法选择"),
    }
}
```

## 总结

Rand 是 Rust 生态系统中功能最完整的随机数生成库。通过本教程，您应该能够：

1. 理解不同类型的随机数生成器及其适用场景
2. 正确使用各种概率分布进行采样
3. 实现高性能的随机数生成
4. 处理密码学安全的随机数需求
5. 在游戏、模拟和数据生成中应用随机技术

关键要点：
- 根据安全需求选择合适的RNG
- 使用固定种子实现可重现的随机序列
- 利用批量操作优化性能
- 正确处理边界情况和错误
- 在测试中使用确定性随机数

Rand 库的设计既注重性能又兼顾安全性，掌握它将帮助您构建可靠的 Rust 应用程序，无论是游戏开发、科学计算还是密码学应用。