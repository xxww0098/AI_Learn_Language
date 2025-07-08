// 10 Rust错误处理 - 使用Result和panic处理错误
// 本章介绍Rust的错误处理机制：panic!和Result类型

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    // panic基础
    panic_basics();
    
    // Result基础
    result_basics();
    
    // 错误传播
    error_propagation();
    
    // 自定义错误处理
    custom_error_handling();
}

// panic基础 - 不可恢复的错误
fn panic_basics() {
    println!("=== Panic基础 ===");
    
    // 注意：这些panic!调用会终止程序，所以在实际使用时要小心
    println!("演示panic!的使用（这里只是展示语法）");
    
    // panic!("出现了严重错误！");  // 取消注释会终止程序
    
    // 数组越界会导致panic
    let v = vec![1, 2, 3];
    // let element = v[99];  // 取消注释会panic
    
    // 安全的访问方式
    match v.get(99) {
        Some(element) => println!("元素: {}", element),
        None => println!("索引超出范围"),
    }
    
    println!("程序继续运行...");
}

// 案例1：Result基础使用
fn result_basics() {
    println!("\n=== Result基础 ===");
    
    // 文件操作返回Result
    match File::open("hello.txt") {
        Ok(file) => println!("文件打开成功: {:?}", file),
        Err(error) => println!("文件打开失败: {:?}", error),
    }
    
    // 使用unwrap_or_else处理错误
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        println!("文件打开失败，创建默认文件: {:?}", error);
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("无法创建文件: {:?}", error);
        })
    });
    
    // 字符串解析返回Result
    let number_str = "42";
    match number_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {}", number),
        Err(error) => println!("解析失败: {}", error),
    }
    
    let invalid_str = "abc";
    match invalid_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {}", number),
        Err(error) => println!("解析失败: {}", error),
    }
    
    // 使用?操作符的简化形式
    match parse_and_double("10") {
        Ok(result) => println!("解析并翻倍: {}", result),
        Err(error) => println!("错误: {}", error),
    }
}

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let number = s.parse::<i32>()?;  // ?操作符自动处理错误
    Ok(number * 2)
}

// 案例2：错误传播
fn error_propagation() {
    println!("\n=== 错误传播 ===");
    
    // 读取文件内容
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(error) => println!("读取失败: {}", error),
    }
    
    // 更简洁的版本
    match read_username_from_file_short() {
        Ok(username) => println!("用户名（简洁版）: {}", username),
        Err(error) => println!("读取失败（简洁版）: {}", error),
    }
    
    // 链式调用
    match read_username_from_file_shortest() {
        Ok(username) => println!("用户名（最简版）: {}", username),
        Err(error) => println!("读取失败（最简版）: {}", error),
    }
}

// 传统的错误传播方式
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用?操作符简化
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更简洁的链式调用
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 数学计算错误处理
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("不能除以零".to_string())
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("不能计算负数的平方根".to_string())
    } else {
        Ok(x.sqrt())
    }
}

// 组合多个可能失败的操作
fn complex_calculation(a: f64, b: f64, c: f64) -> Result<f64, String> {
    let division_result = divide(a, b)?;
    let sqrt_result = sqrt(division_result + c)?;
    Ok(sqrt_result * 2.0)
}

// 自定义错误类型
#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "除零错误"),
            CalculatorError::NegativeSquareRoot => write!(f, "负数平方根错误"),
            CalculatorError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
        }
    }
}

impl std::error::Error for CalculatorError {}

fn custom_error_handling() {
    println!("\n=== 自定义错误处理 ===");
    
    // 数学计算示例
    let calculations = vec![
        (10.0, 2.0, 1.0),   // 正常情况
        (10.0, 0.0, 1.0),   // 除零错误
        (10.0, 2.0, -10.0), // 负数平方根
    ];
    
    for (a, b, c) in calculations {
        match complex_calculation(a, b, c) {
            Ok(result) => println!("计算 ({}, {}, {}) = {:.2}", a, b, c, result),
            Err(error) => println!("计算 ({}, {}, {}) 失败: {}", a, b, c, error),
        }
    }
    
    // 使用自定义错误类型
    let operations = vec![
        safe_divide(10.0, 2.0),
        safe_divide(10.0, 0.0),
        safe_sqrt(9.0),
        safe_sqrt(-4.0),
    ];
    
    for operation in operations {
        match operation {
            Ok(result) => println!("操作成功: {:.2}", result),
            Err(error) => println!("操作失败: {}", error),
        }
    }
    
    // 用户输入处理
    user_input_handling();
}

fn safe_divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, CalculatorError> {
    if x < 0.0 {
        Err(CalculatorError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// 模拟用户输入处理
fn user_input_handling() {
    println!("\n--- 用户输入处理 ---");
    
    let inputs = vec!["42", "abc", "3.14", ""];
    
    for input in inputs {
        match process_user_input(input) {
            Ok(number) => println!("输入 '{}' 处理成功: {}", input, number),
            Err(error) => println!("输入 '{}' 处理失败: {}", input, error),
        }
    }
}

fn process_user_input(input: &str) -> Result<i32, CalculatorError> {
    if input.is_empty() {
        return Err(CalculatorError::InvalidInput("输入为空".to_string()));
    }
    
    input.parse::<i32>().map_err(|_| {
        CalculatorError::InvalidInput(format!("无法解析为整数: '{}'", input))
    })
}

// 银行账户系统错误处理
#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    AccountNotFound,
    InvalidAmount,
}

impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BankError::InsufficientFunds => write!(f, "余额不足"),
            BankError::AccountNotFound => write!(f, "账户不存在"),
            BankError::InvalidAmount => write!(f, "无效金额"),
        }
    }
}

#[derive(Debug)]
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new(initial_balance: f64) -> Result<Self, BankError> {
        if initial_balance < 0.0 {
            Err(BankError::InvalidAmount)
        } else {
            Ok(BankAccount { balance: initial_balance })
        }
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        
        if amount > self.balance {
            return Err(BankError::InsufficientFunds);
        }
        
        self.balance -= amount;
        Ok(())
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        
        self.balance += amount;
        Ok(())
    }
    
    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn bank_system_example() {
    println!("\n=== 银行系统示例 ===");
    
    // 创建账户
    let mut account = match BankAccount::new(1000.0) {
        Ok(acc) => acc,
        Err(error) => {
            println!("创建账户失败: {}", error);
            return;
        }
    };
    
    println!("初始余额: {:.2}", account.get_balance());
    
    // 存款操作
    match account.deposit(500.0) {
        Ok(()) => println!("存款成功，余额: {:.2}", account.get_balance()),
        Err(error) => println!("存款失败: {}", error),
    }
    
    // 取款操作
    let withdrawals = vec![200.0, 2000.0, -100.0];
    
    for amount in withdrawals {
        match account.withdraw(amount) {
            Ok(()) => println!("取款 {:.2} 成功，余额: {:.2}", amount, account.get_balance()),
            Err(error) => println!("取款 {:.2} 失败: {}", amount, error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(9.0), Ok(3.0));
        assert!(sqrt(-4.0).is_err());
    }

    #[test]
    fn test_bank_account() {
        let mut account = BankAccount::new(1000.0).unwrap();
        
        assert!(account.deposit(500.0).is_ok());
        assert_eq!(account.get_balance(), 1500.0);
        
        assert!(account.withdraw(200.0).is_ok());
        assert_eq!(account.get_balance(), 1300.0);
        
        assert!(account.withdraw(2000.0).is_err());
        assert!(account.withdraw(-100.0).is_err());
    }

    #[test]
    fn test_examples() {
        result_basics();
        error_propagation();
        custom_error_handling();
        bank_system_example();
    }
}

// 错误处理要点总结：
// 1. panic!用于不可恢复的错误，会终止程序
// 2. Result<T, E>用于可恢复的错误处理
// 3. ?操作符简化错误传播
// 4. match表达式处理Result的两种情况
// 5. unwrap()和expect()会在错误时panic
// 6. unwrap_or()和unwrap_or_else()提供默认值
// 7. 自定义错误类型提供更好的错误信息
// 8. 错误处理是Rust类型系统的一部分，确保安全性