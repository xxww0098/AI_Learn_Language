// 05 Rust控制流 - if语句、循环和模式匹配
// 本章介绍Rust中的控制流结构：条件语句、循环和模式匹配

fn main() {
    // 基本的if语句
    let number = 3;
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    
    // if表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number的值是: {}", number);
    
    // 循环示例
    basic_loops();
    
    // 模式匹配示例
    pattern_matching();
}

// 案例1：条件语句和循环
fn basic_loops() {
    println!("\n=== 循环示例 ===");
    
    // loop循环（无限循环）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;  // 返回值给result
        }
    };
    println!("loop结果: {}", result);
    
    // while循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");
    
    // for循环遍历数组
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("值是: {}", element);
    }
    
    // for循环使用范围
    for number in 1..4 {
        println!("{}!", number);
    }
    
    // for循环倒序
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

// 案例2：模式匹配和高级控制流
fn pattern_matching() {
    println!("\n=== 模式匹配示例 ===");
    
    // 基本的if-else链
    let number = 6;
    if number % 4 == 0 {
        println!("数字能被4整除");
    } else if number % 3 == 0 {
        println!("数字能被3整除");
    } else if number % 2 == 0 {
        println!("数字能被2整除");
    } else {
        println!("数字不能被4、3或2整除");
    }
    
    // 使用match进行模式匹配
    let day = 3;
    let day_name = match day {
        1 => "星期一",
        2 => "星期二",
        3 => "星期三",
        4 => "星期四",
        5 => "星期五",
        6 | 7 => "周末",  // 多个值的匹配
        _ => "无效的天数",  // 默认情况
    };
    println!("今天是: {}", day_name);
    
    // 匹配范围
    let age = 25;
    match age {
        0..=12 => println!("儿童"),
        13..=19 => println!("青少年"),
        20..=59 => println!("成人"),
        60.. => println!("老年人"),
    }
}

// 循环标签和嵌套循环
fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // 跳出外层循环
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("结束计数 = {}", count);
}

// 条件函数示例
fn check_number(n: i32) -> &'static str {
    if n > 0 {
        "正数"
    } else if n < 0 {
        "负数"
    } else {
        "零"
    }
}

// 分数等级判断
fn get_grade(score: i32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

// 循环处理数组
fn process_array() {
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;
    
    for number in numbers {
        sum += number;
    }
    
    println!("数组总和: {}", sum);
    
    // 使用enumerate获取索引
    for (index, value) in numbers.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }
}

// 猜数字游戏简化版
fn guess_number_game() {
    let secret_number = 7;
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        let guess = attempts * 2;  // 模拟猜测
        
        println!("第{}次猜测: {}", attempts, guess);
        
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了！"),
            std::cmp::Ordering::Greater => println!("太大了！"),
            std::cmp::Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
        }
        
        if attempts >= 5 {
            println!("超过最大尝试次数");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_loops() {
        basic_loops();
    }

    #[test]
    fn test_pattern_matching() {
        pattern_matching();
    }

    #[test]
    fn test_nested_loops() {
        nested_loops();
    }

    #[test]
    fn test_check_number() {
        assert_eq!(check_number(5), "正数");
        assert_eq!(check_number(-3), "负数");
        assert_eq!(check_number(0), "零");
    }

    #[test]
    fn test_get_grade() {
        assert_eq!(get_grade(95), 'A');
        assert_eq!(get_grade(85), 'B');
        assert_eq!(get_grade(50), 'F');
    }

    #[test]
    fn test_process_array() {
        process_array();
    }

    #[test]
    fn test_guess_number_game() {
        guess_number_game();
    }
}

// 控制流要点总结：
// 1. if是表达式，可以返回值
// 2. loop创建无限循环，while有条件循环
// 3. for用于遍历集合或范围
// 4. match用于模式匹配，必须覆盖所有可能的情况
// 5. 可以使用标签跳出嵌套循环
// 6. break和continue控制循环流程