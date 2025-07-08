// 01 Rust基础 - Rust语言基础入门
// 本章介绍Rust的基本语法和概念

// 主函数（程序入口点）
fn main() {
    // 这是Rust程序的入口点
    println!("Hello, Rust!");
}

// 案例1：简单的Hello World程序
fn hello_world() {
    println!("你好，世界！");
    println!("这是我的第一个Rust程序");
}

// 案例2：基本的函数定义和调用
fn greet(name: &str) {
    println!("你好，{}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }

    #[test]
    fn test_greet() {
        greet("Alice");
    }
}

// 运行指南：
// 1. 使用 cargo run 运行程序
// 2. 使用 cargo test 运行测试
// 3. 每个Rust程序都必须有一个main函数作为入口点