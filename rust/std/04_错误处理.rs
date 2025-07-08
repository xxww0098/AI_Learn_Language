// 04_错误处理.rs
// Rust标准库错误处理详解

/*
Rust的错误处理系统基于两个核心类型：
1. Result<T, E> - 可恢复的错误，包含Ok(T)和Err(E)
2. Option<T> - 表示可能存在或不存在的值，包含Some(T)和None

Rust的错误处理特点：
- 编译时错误检查：强制处理所有可能的错误
- 零成本抽象：错误处理不会产生运行时开销
- 显式错误处理：错误必须显式处理，不能被忽略
- 类型安全：错误类型在编译时确定

错误处理模式：
- match 表达式：完全控制错误处理流程
- unwrap() 和 expect()：快速原型开发，生产环境慎用
- ? 操作符：简洁的错误传播
- map() 和 and_then()：函数式错误处理
- 自定义错误类型：结构化错误信息

panic! 机制：
- 用于不可恢复的错误
- 会展开调用栈或直接终止程序
- 可以通过 std::panic::catch_unwind 捕获
*/

use std::fmt;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    println!("=== Rust标准库错误处理 ===");
    
    // 1. Result<T, E> 基础用法
    println!("\n1. Result<T, E> 基础用法：");
    result_basics();
    
    // 2. Option<T> 基础用法
    println!("\n2. Option<T> 基础用法：");
    option_basics();
    
    // 3. 错误处理模式
    println!("\n3. 错误处理模式：");
    error_handling_patterns();
    
    // 4. ? 操作符的使用
    println!("\n4. ? 操作符的使用：");
    question_mark_operator();
    
    // 5. 自定义错误类型
    println!("\n5. 自定义错误类型：");
    custom_error_types();
    
    // 6. 错误链和上下文
    println!("\n6. 错误链和上下文：");
    error_context();
    
    // 7. panic! 和恢复
    println!("\n7. panic! 和恢复：");
    panic_and_recovery();
    
    // 8. 实际应用示例
    println!("\n8. 实际应用示例：");
    practical_examples();
    
    // 9. 错误处理最佳实践
    println!("\n9. 错误处理最佳实践：");
    best_practices();
    
    println!("\n=== 错误处理学习完成 ===");
}

// Result<T, E> 基础用法
fn result_basics() {
    // 基本的Result使用
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // 处理错误情况
    let error_result = divide(10, 0);
    match error_result {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // 使用 unwrap() (不推荐在生产环境使用)
    let success = divide(20, 4);
    println!("20 / 4 = {}", success.unwrap());
    
    // 使用 expect() 提供更好的错误消息
    let success2 = divide(15, 3);
    println!("15 / 3 = {}", success2.expect("除法计算失败"));
    
    // 使用 unwrap_or() 提供默认值
    let error_with_default = divide(10, 0);
    println!("10 / 0 的默认值 = {}", error_with_default.unwrap_or(0));
    
    // 使用 unwrap_or_else() 提供默认值的计算函数
    let error_with_calc = divide(10, 0);
    let default = error_with_calc.unwrap_or_else(|_| {
        println!("计算默认值中...");
        -1
    });
    println!("计算的默认值 = {}", default);
}

// Option<T> 基础用法
fn option_basics() {
    // Some 和 None
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    // 使用 match 处理 Option
    match some_value {
        Some(value) => println!("找到值: {}", value),
        None => println!("没有值"),
    }
    
    match none_value {
        Some(value) => println!("找到值: {}", value),
        None => println!("没有值"),
    }
    
    // 使用 if let 简化匹配
    if let Some(value) = some_value {
        println!("if let 找到值: {}", value);
    }
    
    // Option 的常用方法
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 安全索引访问
    match numbers.get(2) {
        Some(value) => println!("索引2的值: {}", value),
        None => println!("索引2超出范围"),
    }
    
    // 查找元素
    match numbers.iter().find(|&&x| x > 3) {
        Some(value) => println!("找到第一个大于3的值: {}", value),
        None => println!("没有找到大于3的值"),
    }
    
    // Option 的转换方法
    let opt_str = Some("42");
    let opt_num = opt_str.and_then(|s| s.parse::<i32>().ok());
    println!("字符串转数字: {:?}", opt_num);
    
    // 使用 map 转换值
    let doubled = some_value.map(|x| x * 2);
    println!("翻倍后的值: {:?}", doubled);
    
    // 使用 filter 过滤值
    let filtered = some_value.filter(|&x| x > 40);
    println!("过滤后的值: {:?}", filtered);
}

// 错误处理模式
fn error_handling_patterns() {
    // 1. 早期返回模式
    println!("早期返回模式：");
    match process_data("valid") {
        Ok(result) => println!("处理成功: {}", result),
        Err(e) => println!("处理失败: {}", e),
    }
    
    // 2. 链式处理
    println!("链式处理：");
    let result = Some("123")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|n| n * 2)
        .filter(|&n| n > 100);
    println!("链式处理结果: {:?}", result);
    
    // 3. 错误合并
    println!("错误合并：");
    let combined = combine_results(Ok(10), Ok(20));
    println!("合并结果: {:?}", combined);
    
    let combined_error = combine_results(Ok(10), Err("错误".to_string()));
    println!("合并错误: {:?}", combined_error);
    
    // 4. 错误映射
    println!("错误映射：");
    let mapped = parse_number("abc").map_err(|e| format!("解析错误: {}", e));
    println!("映射后的错误: {:?}", mapped);
}

// ? 操作符的使用
fn question_mark_operator() {
    // 演示 ? 操作符
    match read_file_content("example.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取文件失败: {}", e),
    }
    
    // 多个 ? 操作符的使用
    match complex_operation() {
        Ok(result) => println!("复杂操作结果: {}", result),
        Err(e) => println!("复杂操作失败: {}", e),
    }
    
    // 在不同错误类型间使用 ?
    match mixed_error_types() {
        Ok(result) => println!("混合错误类型结果: {}", result),
        Err(e) => println!("混合错误类型失败: {}", e),
    }
}

// 自定义错误类型
fn custom_error_types() {
    // 使用自定义错误
    match validate_age(25) {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("年龄验证失败: {}", e),
    }
    
    match validate_age(150) {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("年龄验证失败: {}", e),
    }
    
    // 错误类型匹配
    match validate_age(-5) {
        Ok(age) => println!("有效年龄: {}", age),
        Err(ValidationError::TooYoung) => println!("年龄太小"),
        Err(ValidationError::TooOld) => println!("年龄太大"),
        Err(ValidationError::InvalidInput(msg)) => println!("无效输入: {}", msg),
    }
}

// 错误链和上下文
fn error_context() {
    // 创建错误链
    match process_user_data("") {
        Ok(result) => println!("处理用户数据成功: {}", result),
        Err(e) => {
            println!("处理用户数据失败: {}", e);
            
            // 打印错误链
            let mut source = e.source();
            while let Some(err) = source {
                println!("  原因: {}", err);
                source = err.source();
            }
        }
    }
}

// panic! 和恢复
fn panic_and_recovery() {
    // 捕获 panic
    let result = std::panic::catch_unwind(|| {
        // 这里会 panic
        panic!("这是一个测试 panic");
    });
    
    match result {
        Ok(_) => println!("没有发生 panic"),
        Err(_) => println!("捕获到 panic"),
    }
    
    // 设置自定义 panic hook
    std::panic::set_hook(Box::new(|info| {
        println!("自定义 panic 处理: {}", info);
    }));
    
    // 重置 panic hook
    let _ = std::panic::take_hook();
    
    // 演示可恢复的错误处理
    let results = vec![
        safe_divide(10, 2),
        safe_divide(10, 0),
        safe_divide(20, 4),
    ];
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("结果 {}: {}", i + 1, value),
            Err(e) => println!("错误 {}: {}", i + 1, e),
        }
    }
}

// 实际应用示例
fn practical_examples() {
    // 配置文件处理
    match load_config("config.toml") {
        Ok(config) => println!("配置加载成功: {:?}", config),
        Err(e) => println!("配置加载失败: {}", e),
    }
    
    // 网络请求模拟
    match simulate_network_request("https://api.example.com") {
        Ok(response) => println!("网络请求成功: {}", response),
        Err(e) => println!("网络请求失败: {}", e),
    }
    
    // 数据库操作模拟
    match simulate_database_operation(42) {
        Ok(data) => println!("数据库操作成功: {:?}", data),
        Err(e) => println!("数据库操作失败: {}", e),
    }
}

// 错误处理最佳实践
fn best_practices() {
    println!("错误处理最佳实践：");
    println!("1. 使用 Result<T, E> 处理可恢复错误");
    println!("2. 使用 Option<T> 处理可能为空的值");
    println!("3. 创建有意义的错误消息");
    println!("4. 使用 ? 操作符简化错误传播");
    println!("5. 在库中返回错误，在应用中处理错误");
    println!("6. 使用自定义错误类型提供结构化错误信息");
    println!("7. 避免过度使用 unwrap() 和 expect()");
    println!("8. 在适当的时候使用 panic! 处理编程错误");
    println!("9. 为错误实现 Display 和 Error trait");
    println!("10. 使用错误链提供详细的错误上下文");
}

// 辅助函数定义

// 简单的除法函数
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为0".to_string())
    } else {
        Ok(a / b)
    }
}

// 数据处理函数
fn process_data(data: &str) -> Result<String, String> {
    if data.is_empty() {
        return Err("数据不能为空".to_string());
    }
    
    if data == "invalid" {
        return Err("数据格式无效".to_string());
    }
    
    Ok(format!("处理后的数据: {}", data.to_uppercase()))
}

// 合并两个Result
fn combine_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    match (a, b) {
        (Ok(x), Ok(y)) => Ok(x + y),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

// 解析数字函数
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// 读取文件内容函数 (模拟)
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    // 模拟文件读取
    if filename.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "文件名不能为空"));
    }
    
    if filename == "nonexistent.txt" {
        return Err(io::Error::new(io::ErrorKind::NotFound, "文件不存在"));
    }
    
    Ok(format!("文件 {} 的内容", filename))
}

// 复杂操作函数
fn complex_operation() -> Result<String, Box<dyn Error>> {
    let data = read_file_content("data.txt")?;
    let number = parse_number("42")?;
    Ok(format!("数据: {}, 数字: {}", data, number))
}

// 混合错误类型处理
fn mixed_error_types() -> Result<i32, Box<dyn Error>> {
    let content = read_file_content("numbers.txt")?;
    let number = content.trim().parse::<i32>()?;
    Ok(number * 2)
}

// 自定义错误类型
#[derive(Debug)]
enum ValidationError {
    TooYoung,
    TooOld,
    InvalidInput(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::TooYoung => write!(f, "年龄太小"),
            ValidationError::TooOld => write!(f, "年龄太大"),
            ValidationError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
        }
    }
}

impl Error for ValidationError {}

// 年龄验证函数
fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError::InvalidInput("年龄不能为负数".to_string()))
    } else if age < 18 {
        Err(ValidationError::TooYoung)
    } else if age > 120 {
        Err(ValidationError::TooOld)
    } else {
        Ok(age)
    }
}

// 包装错误类型
#[derive(Debug)]
struct ProcessError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "处理错误: {}", self.message)
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

// 处理用户数据函数
fn process_user_data(data: &str) -> Result<String, ProcessError> {
    if data.is_empty() {
        return Err(ProcessError {
            message: "用户数据为空".to_string(),
            source: Some(Box::new(ValidationError::InvalidInput("数据不能为空".to_string()))),
        });
    }
    
    Ok(format!("处理后的用户数据: {}", data))
}

// 安全除法函数
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为0".to_string())
    } else {
        Ok(a / b)
    }
}

// 配置结构体
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
}

// 加载配置函数 (模拟)
fn load_config(filename: &str) -> Result<Config, String> {
    if filename.is_empty() {
        return Err("配置文件名不能为空".to_string());
    }
    
    if !filename.ends_with(".toml") {
        return Err("配置文件必须是TOML格式".to_string());
    }
    
    Ok(Config {
        name: "示例应用".to_string(),
        version: "1.0.0".to_string(),
    })
}

// 模拟网络请求
fn simulate_network_request(url: &str) -> Result<String, String> {
    if url.is_empty() {
        return Err("URL不能为空".to_string());
    }
    
    if !url.starts_with("https://") {
        return Err("URL必须使用HTTPS".to_string());
    }
    
    Ok(format!("来自 {} 的响应数据", url))
}

// 模拟数据库操作
fn simulate_database_operation(id: i32) -> Result<String, String> {
    if id <= 0 {
        return Err("ID必须是正数".to_string());
    }
    
    if id > 1000 {
        return Err("ID超出范围".to_string());
    }
    
    Ok(format!("数据库记录 ID: {}", id))
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
    fn test_option_some() {
        let opt = Some(42);
        assert_eq!(opt.unwrap(), 42);
    }
    
    #[test]
    fn test_option_none() {
        let opt: Option<i32> = None;
        assert_eq!(opt.unwrap_or(0), 0);
    }
    
    #[test]
    fn test_validation_error() {
        match validate_age(-5) {
            Err(ValidationError::InvalidInput(_)) => (),
            _ => panic!("期望 InvalidInput 错误"),
        }
    }
    
    #[test]
    fn test_process_data() {
        assert_eq!(process_data("test"), Ok("处理后的数据: TEST".to_string()));
        assert_eq!(process_data(""), Err("数据不能为空".to_string()));
    }
    
    #[test]
    fn test_combine_results() {
        assert_eq!(combine_results(Ok(1), Ok(2)), Ok(3));
        assert_eq!(combine_results(Ok(1), Err("错误".to_string())), Err("错误".to_string()));
    }
    
    #[test]
    fn test_config_loading() {
        assert!(load_config("config.toml").is_ok());
        assert!(load_config("config.txt").is_err());
    }
}