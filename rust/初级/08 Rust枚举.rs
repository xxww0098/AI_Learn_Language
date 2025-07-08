// 08 Rust枚举 - 定义和使用枚举类型
// 本章介绍枚举的定义、使用和模式匹配

// 基本枚举定义
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// 更复杂的枚举
#[derive(Debug)]
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },    // 匿名结构体
    Write(String),              // 字符串
    ChangeColor(i32, i32, i32), // 三个i32
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // 基本枚举使用
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    
    // 带数据的枚举
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("本地地址: {:?}", home);
    println!("回环地址: {:?}", loopback);
    
    // 复杂枚举示例
    message_example();
    
    // Option枚举示例
    option_example();
    
    // Result枚举示例
    result_example();
}

// 案例1：消息处理系统
fn message_example() {
    println!("\n=== 消息处理示例 ===");
    
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in messages {
        message.call();
    }
}

// 案例2：Option枚举使用
fn option_example() {
    println!("\n=== Option枚举示例 ===");
    
    // Option用于表示可能为空的值
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // 使用match处理Option
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    println!("x的处理结果: {}", process_option(x));
    println!("y的处理结果: {}", process_option(y));
    
    // 使用if let简化匹配
    if let Some(value) = some_number {
        println!("从some_number中提取的值: {}", value);
    }
    
    // Option的常用方法
    let number = Some(42);
    println!("是否有值: {}", number.is_some());
    println!("是否为空: {}", number.is_none());
    println!("解包或默认值: {}", number.unwrap_or(0));
    
    let empty: Option<i32> = None;
    println!("空值解包或默认值: {}", empty.unwrap_or(0));
}

fn process_option(x: Option<i32>) -> i32 {
    match x {
        Some(i) => i + 1,
        None => 0,
    }
}

// 硬币枚举示例
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士！");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的25美分硬币！", state);
            25
        },
    }
}

// Result枚举示例
fn result_example() {
    println!("\n=== Result枚举示例 ===");
    
    // Result用于错误处理
    let good_result: Result<i32, &str> = Ok(42);
    let bad_result: Result<i32, &str> = Err("出错了");
    
    println!("好的结果: {:?}", good_result);
    println!("坏的结果: {:?}", bad_result);
    
    // 使用match处理Result
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(error) => println!("错误: {}", error),
    }
    
    // 使用if let处理成功情况
    if let Ok(value) = divide(20, 4) {
        println!("20 / 4 = {}", value);
    }
    
    // 使用unwrap_or处理错误
    let result1 = divide(15, 3).unwrap_or(0);
    let result2 = divide(15, 0).unwrap_or(0);
    println!("15 / 3 (带默认值): {}", result1);
    println!("15 / 0 (带默认值): {}", result2);
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("不能除以零")
    } else {
        Ok(a / b)
    }
}

// 状态机示例
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn traffic_light_example() {
    println!("\n=== 交通灯示例 ===");
    
    let mut light = TrafficLight::Red;
    
    for _ in 0..5 {
        println!("当前灯: {:?}, 持续时间: {}秒", light, light.duration());
        light = light.next();
    }
}

// 计算器操作枚举
#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> Result<f64, &'static str> {
        match self {
            Operation::Add(a, b) => Ok(a + b),
            Operation::Subtract(a, b) => Ok(a - b),
            Operation::Multiply(a, b) => Ok(a * b),
            Operation::Divide(a, b) => {
                if *b == 0.0 {
                    Err("不能除以零")
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

fn calculator_example() {
    println!("\n=== 计算器示例 ===");
    
    let operations = vec![
        Operation::Add(5.0, 3.0),
        Operation::Subtract(10.0, 4.0),
        Operation::Multiply(3.0, 7.0),
        Operation::Divide(15.0, 3.0),
        Operation::Divide(10.0, 0.0),
    ];
    
    for op in operations {
        match op.calculate() {
            Ok(result) => println!("{:?} = {}", op, result),
            Err(error) => println!("{:?} 错误: {}", op, error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_value() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alaska)), 25);
    }

    #[test]
    fn test_option_processing() {
        assert_eq!(process_option(Some(5)), 6);
        assert_eq!(process_option(None), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("不能除以零"));
    }

    #[test]
    fn test_traffic_light() {
        let red = TrafficLight::Red;
        let green = red.next();
        let yellow = green.next();
        let red_again = yellow.next();
        
        assert!(matches!(green, TrafficLight::Green));
        assert!(matches!(yellow, TrafficLight::Yellow));
        assert!(matches!(red_again, TrafficLight::Red));
    }

    #[test]
    fn test_calculator() {
        let add = Operation::Add(5.0, 3.0);
        assert_eq!(add.calculate(), Ok(8.0));
        
        let divide_by_zero = Operation::Divide(10.0, 0.0);
        assert_eq!(divide_by_zero.calculate(), Err("不能除以零"));
    }

    #[test]
    fn test_examples() {
        message_example();
        option_example();
        result_example();
        traffic_light_example();
        calculator_example();
    }
}

// 枚举要点总结：
// 1. 枚举用于定义一组相关的值
// 2. 枚举变体可以携带不同类型的数据
// 3. match表达式用于处理枚举的所有可能情况
// 4. Option<T>用于处理可能为空的值
// 5. Result<T, E>用于错误处理
// 6. 可以为枚举实现方法
// 7. if let提供了简化的模式匹配语法