// 01_标准库概述.rs
// Rust标准库(std)完整概述与学习指南

/*
=== Rust标准库(std)完整教程系列 ===

本教程系列提供了Rust标准库的全面学习资源，涵盖12个核心模块：

📚 教程结构：
01_标准库概述.rs     - 标准库整体概况和基础概念 (本文件)
02_集合类型.rs       - Vec、HashMap、HashSet等集合数据结构
03_字符串处理.rs     - String、&str和字符串操作方法
04_错误处理.rs       - Result、Option和错误处理模式
05_输入输出.rs       - 文件I/O、标准输入输出操作
06_网络编程.rs       - TCP/UDP网络编程和Socket操作
07_线程并发.rs       - 多线程编程和并发控制
08_时间日期.rs       - 时间处理、Duration和性能测量
09_内存管理.rs       - 所有权系统、智能指针和内存优化
10_文件系统操作.rs   - 文件系统API、路径处理和元数据
11_进程与环境.rs     - 进程管理、环境变量和系统信息
12_数据序列化.rs     - 数据格式化、序列化和反序列化

🎯 std库的核心模块分类：

1. 📊 数据结构模块：
   - std::collections - 集合类型 (Vec, HashMap, BTreeMap等)
   - std::string - 字符串类型 (String)
   - std::vec - 动态数组
   
2. 🔧 内存管理模块：
   - std::boxed - 堆分配 (Box<T>)
   - std::rc - 引用计数 (Rc<T>, Weak<T>)
   - std::sync - 同步原语 (Arc<T>, Mutex<T>)
   - std::cell - 内部可变性 (RefCell<T>, Cell<T>)
   - std::mem - 内存操作工具
   
3. 🔀 错误处理模块：
   - std::result - Result<T, E> 类型
   - std::option - Option<T> 类型
   - std::error - 错误trait定义
   - std::panic - panic处理机制
   
4. 💾 I/O操作模块：
   - std::io - 输入输出trait和工具
   - std::fs - 文件系统操作
   - std::net - 网络编程API
   - std::path - 路径处理
   
5. ⚡ 并发编程模块：
   - std::thread - 线程管理
   - std::sync::mpsc - 消息传递通道
   - std::sync - 同步原语 (Mutex, RwLock, Barrier等)
   - std::sync::atomic - 原子类型
   
6. ⏰ 时间处理模块：
   - std::time - 时间和持续时间
   - std::thread::sleep - 线程休眠
   
7. 🔄 格式化和转换：
   - std::fmt - 格式化trait
   - std::str - 字符串切片工具
   - std::convert - 类型转换trait
   
8. 🖥️ 系统交互模块：
   - std::env - 环境变量和程序参数
   - std::process - 进程管理
   - std::ffi - 外部函数接口
   
9. 🧮 数学和工具：
   - std::cmp - 比较trait
   - std::ops - 运算符重载
   - std::iter - 迭代器trait和工具
   - std::marker - 标记trait

📋 std库的重要特点：

🛡️ 安全性保证：
- 内存安全：防止缓冲区溢出、悬挂指针
- 线程安全：数据竞争编译时检查
- 类型安全：强类型系统防止错误

⚡ 性能特点：
- 零成本抽象：高级特性无运行时开销  
- 编译时优化：大部分检查在编译期完成
- 高效内存管理：RAII和确定性析构

🌍 跨平台支持：
- 统一API：不同操作系统提供一致接口
- 条件编译：平台特定功能的优雅处理
- 标准化：遵循现代系统编程标准

🔧 使用指南：

入门学习路径：
1. 基础 (01-05)：概述 → 集合 → 字符串 → 错误处理 → I/O
2. 网络并发 (06-07)：网络编程 → 线程并发  
3. 系统级 (08-12)：时间 → 内存 → 文件系统 → 进程 → 序列化

实践建议：
- 每个教程都包含完整的代码示例
- 运行测试用例验证理解
- 结合实际项目加深学习
- 参考标准库文档深入了解

🔗 相关资源：
- 官方文档：https://doc.rust-lang.org/std/
- Rust程序设计语言：https://kaisery.github.io/trpl-zh-cn/
- Rust参考手册：https://doc.rust-lang.org/reference/
*/

use std::fmt;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Rust标准库概述 ===");
    
    // 1. 基本数据结构示例
    println!("\n1. 基本数据结构：");
    
    // Vec - 动态数组
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("Vec示例: {:?}", numbers);
    
    // HashMap - 哈希表
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("苹果".to_string(), 10);
    map.insert("香蕉".to_string(), 20);
    println!("HashMap示例: {:?}", map);
    
    // 2. 字符串处理
    println!("\n2. 字符串处理：");
    let greeting = "你好，世界！";
    let owned_string = String::from("Rust编程");
    println!("字符串切片: {}", greeting);
    println!("拥有的字符串: {}", owned_string);
    
    // 3. 错误处理
    println!("\n3. 错误处理：");
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    // 4. Option类型
    println!("\n4. Option类型：");
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    match some_value {
        Some(value) => println!("找到值: {}", value),
        None => println!("没有值"),
    }
    
    match none_value {
        Some(value) => println!("找到值: {}", value),
        None => println!("没有值"),
    }
    
    // 5. 迭代器示例
    println!("\n5. 迭代器：");
    let data = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("原始数据: {:?}", data);
    println!("翻倍后: {:?}", doubled);
    
    // 6. 时间处理
    println!("\n6. 时间处理：");
    let now = std::time::SystemTime::now();
    println!("当前时间: {:?}", now);
    
    // 7. 线程简单示例
    println!("\n7. 线程示例：");
    let handle = thread::spawn(|| {
        println!("在子线程中运行");
        thread::sleep(Duration::from_millis(100));
        "子线程完成"
    });
    
    match handle.join() {
        Ok(result) => println!("子线程结果: {}", result),
        Err(_) => println!("子线程执行失败"),
    }
    
    // 8. 模块导览
    println!("\n8. 后续学习模块：");
    module_overview();
    
    println!("\n=== 标准库概述完成 ===");
    println!("🎓 恭喜完成Rust标准库概述学习！");
    println!("📖 建议继续学习：02_集合类型.rs");
}
}

// 模块导览函数
fn module_overview() {
    println!("📚 完整教程系列包含12个核心模块：");
    
    let modules = [
        ("02_集合类型.rs", "📊", "Vec、HashMap、HashSet等数据结构"),
        ("03_字符串处理.rs", "📝", "String、&str和字符串操作"),
        ("04_错误处理.rs", "⚠️", "Result、Option和错误模式"),
        ("05_输入输出.rs", "💾", "文件I/O、标准输入输出"),
        ("06_网络编程.rs", "🌐", "TCP/UDP网络编程"),
        ("07_线程并发.rs", "⚡", "多线程和并发控制"),
        ("08_时间日期.rs", "⏰", "时间处理和性能测量"),
        ("09_内存管理.rs", "🧠", "所有权、智能指针和内存优化"),
        ("10_文件系统操作.rs", "📁", "文件系统API和路径处理"),
        ("11_进程与环境.rs", "🖥️", "进程管理和系统交互"),
        ("12_数据序列化.rs", "🔄", "数据格式化和序列化"),
    ];
    
    for (filename, icon, description) in &modules {
        println!("  {} {} - {}", icon, filename, description);
    }
    
    println!("\n🎯 学习建议：");
    println!("  • 初学者：按顺序学习 02→05 基础模块");
    println!("  • 进阶者：重点学习 06→07 网络并发模块");
    println!("  • 系统编程：深入学习 08→12 系统级模块");
    println!("  • 每个模块都有完整的代码示例和测试");
    println!("  • 建议动手运行代码加深理解");
}

// 错误处理示例函数
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为0".to_string())
    } else {
        Ok(a / b)
    }
}

// 自定义结构体实现Display trait
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "姓名: {}, 年龄: {}", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }
    
    #[test]
    fn test_divide_error() {
        assert_eq!(divide(10, 0), Err("除数不能为0".to_string()));
    }
    
    #[test]
    fn test_person_display() {
        let person = Person {
            name: "张三".to_string(),
            age: 30,
        };
        assert_eq!(format!("{}", person), "姓名: 张三, 年龄: 30");
    }
}