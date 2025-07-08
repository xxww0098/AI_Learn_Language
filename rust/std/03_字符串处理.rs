// 03_字符串处理.rs
// Rust标准库字符串处理详解

/*
Rust中的字符串系统包含两种主要类型：
1. &str - 字符串切片，不可变引用，通常指向UTF-8编码的文本
2. String - 拥有所有权的字符串，可变、可增长

字符串相关的重要概念：
- UTF-8编码：Rust字符串都是有效的UTF-8序列
- 所有权：String拥有数据，&str借用数据
- 不可变性：&str是不可变的，String可以修改
- 切片：可以从String创建&str切片

常用字符串操作：
- 创建、连接、比较
- 查找、替换、分割
- 格式化输出
- 编码转换
- 正则表达式
*/

use std::fmt;

fn main() {
    println!("=== Rust标准库字符串处理 ===");
    
    // 1. 字符串类型基础
    println!("\n1. 字符串类型基础：");
    string_basics();
    
    // 2. 字符串创建方法
    println!("\n2. 字符串创建方法：");
    string_creation();
    
    // 3. 字符串连接操作
    println!("\n3. 字符串连接操作：");
    string_concatenation();
    
    // 4. 字符串查找和替换
    println!("\n4. 字符串查找和替换：");
    string_search_replace();
    
    // 5. 字符串分割和解析
    println!("\n5. 字符串分割和解析：");
    string_split_parse();
    
    // 6. 字符串格式化
    println!("\n6. 字符串格式化：");
    string_formatting();
    
    // 7. 字符串遍历
    println!("\n7. 字符串遍历：");
    string_iteration();
    
    // 8. 字符串切片操作
    println!("\n8. 字符串切片操作：");
    string_slicing();
    
    // 9. 字符串与其他类型转换
    println!("\n9. 字符串与其他类型转换：");
    string_conversion();
    
    // 10. 字符串性能优化
    println!("\n10. 字符串性能优化：");
    string_performance();
    
    println!("\n=== 字符串处理学习完成 ===");
}

// 字符串类型基础
fn string_basics() {
    // 字符串字面量 (&str)
    let str_literal = "Hello, 世界!";
    println!("字符串字面量: {}", str_literal);
    println!("类型: &str, 长度: {} 字节", str_literal.len());
    
    // 拥有所有权的字符串 (String)
    let mut owned_string = String::from("Hello, Rust!");
    println!("拥有的字符串: {}", owned_string);
    
    // String可以修改
    owned_string.push_str(" 很棒!");
    println!("修改后: {}", owned_string);
    
    // 借用String作为&str
    let borrowed: &str = &owned_string;
    println!("借用的字符串: {}", borrowed);
    
    // 字符串是UTF-8编码
    let chinese = "你好";
    let emoji = "😀🦀";
    println!("中文字符串: {}", chinese);
    println!("表情符号: {}", emoji);
    
    // 字符串字节长度 vs 字符长度
    println!("'你好' 字节长度: {}, 字符长度: {}", 
             chinese.len(), chinese.chars().count());
}

// 字符串创建方法
fn string_creation() {
    // 从字面量创建
    let s1 = String::from("Hello");
    let s2 = "World".to_string();
    let s3 = "Rust".to_owned();
    
    println!("String::from: {}", s1);
    println!("to_string: {}", s2);
    println!("to_owned: {}", s3);
    
    // 创建空字符串
    let mut empty = String::new();
    empty.push_str("动态添加内容");
    println!("空字符串添加内容: {}", empty);
    
    // 预分配容量
    let mut with_capacity = String::with_capacity(50);
    println!("预分配容量: {}", with_capacity.capacity());
    with_capacity.push_str("预分配的字符串");
    println!("添加内容后: {}", with_capacity);
    
    // 从字符向量创建
    let chars: Vec<char> = vec!['H', 'e', 'l', 'l', 'o'];
    let from_chars: String = chars.into_iter().collect();
    println!("从字符向量: {}", from_chars);
    
    // 重复字符串
    let repeated = "Rust ".repeat(3);
    println!("重复字符串: {}", repeated);
}

// 字符串连接操作
fn string_concatenation() {
    // 使用 + 运算符
    let hello = String::from("Hello");
    let world = " World";
    let result = hello + world; // hello的所有权被转移
    println!("使用 + 连接: {}", result);
    
    // 使用 += 运算符
    let mut greeting = String::from("Hello");
    greeting += " ";
    greeting += "Rust";
    println!("使用 += 连接: {}", greeting);
    
    // 使用 push 和 push_str
    let mut message = String::from("学习");
    message.push(' ');
    message.push_str("Rust");
    message.push('!');
    println!("使用 push 连接: {}", message);
    
    // 使用 format! 宏
    let name = "张三";
    let age = 25;
    let formatted = format!("我叫{}，今年{}岁", name, age);
    println!("使用 format! 连接: {}", formatted);
    
    // 连接字符串数组
    let words = vec!["Rust", "是", "一门", "系统", "编程", "语言"];
    let sentence = words.join(" ");
    println!("连接字符串数组: {}", sentence);
    
    // 高效连接多个字符串
    let parts = vec!["第一部分", "第二部分", "第三部分"];
    let combined = parts.concat();
    println!("连接多个字符串: {}", combined);
}

// 字符串查找和替换
fn string_search_replace() {
    let text = "Rust编程语言是一门现代系统编程语言，Rust具有内存安全特性";
    
    // 查找子字符串
    if let Some(pos) = text.find("Rust") {
        println!("'Rust' 第一次出现在位置: {}", pos);
    }
    
    // 查找最后一次出现的位置
    if let Some(pos) = text.rfind("Rust") {
        println!("'Rust' 最后一次出现在位置: {}", pos);
    }
    
    // 检查字符串包含
    if text.contains("编程") {
        println!("文本包含 '编程'");
    }
    
    // 检查开始和结束
    if text.starts_with("Rust") {
        println!("文本以 'Rust' 开始");
    }
    
    if text.ends_with("特性") {
        println!("文本以 '特性' 结束");
    }
    
    // 替换字符串
    let replaced = text.replace("Rust", "Python");
    println!("替换后: {}", replaced);
    
    // 只替换第一个匹配项
    let replace_first = text.replacen("Rust", "Go", 1);
    println!("只替换第一个: {}", replace_first);
    
    // 查找所有匹配项
    let matches: Vec<_> = text.match_indices("Rust").collect();
    println!("所有'Rust'的位置: {:?}", matches);
}

// 字符串分割和解析
fn string_split_parse() {
    let data = "苹果,香蕉,橙子,葡萄";
    
    // 分割字符串
    let fruits: Vec<&str> = data.split(',').collect();
    println!("分割后的水果: {:?}", fruits);
    
    // 按空白字符分割
    let sentence = "Hello   world\tRust\nProgramming";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("按空白分割: {:?}", words);
    
    // 分割成指定数量的部分
    let limited: Vec<&str> = data.splitn(3, ',').collect();
    println!("限制分割数量: {:?}", limited);
    
    // 按行分割
    let multiline = "第一行\n第二行\r\n第三行";
    let lines: Vec<&str> = multiline.lines().collect();
    println!("按行分割: {:?}", lines);
    
    // 字符串解析为数字
    let numbers_str = "123 456 789";
    let numbers: Vec<i32> = numbers_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("解析为数字: {:?}", numbers);
    
    // 安全解析
    let maybe_number = "42";
    match maybe_number.parse::<i32>() {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // 解析布尔值
    let bool_str = "true";
    match bool_str.parse::<bool>() {
        Ok(b) => println!("解析为布尔值: {}", b),
        Err(e) => println!("解析失败: {}", e),
    }
}

// 字符串格式化
fn string_formatting() {
    let name = "李四";
    let age = 30;
    let score = 95.5;
    
    // 基本格式化
    println!("姓名: {}, 年龄: {}, 分数: {}", name, age, score);
    
    // 位置参数
    println!("{0} 今年 {1} 岁，{0} 的分数是 {2}", name, age, score);
    
    // 命名参数
    println!("{name} 今年 {age} 岁，分数是 {score}",
             name = name, age = age, score = score);
    
    // 数字格式化
    let pi = 3.14159265359;
    println!("π = {:.2}", pi);  // 保留2位小数
    println!("π = {:.5}", pi);  // 保留5位小数
    
    // 宽度和对齐
    println!("左对齐: '{:<10}'", "Hello");
    println!("右对齐: '{:>10}'", "Hello");
    println!("居中对齐: '{:^10}'", "Hello");
    
    // 填充字符
    println!("填充字符: '{:*^10}'", "Hello");
    
    // 进制转换
    let number = 255;
    println!("十进制: {}", number);
    println!("二进制: {:b}", number);
    println!("八进制: {:o}", number);
    println!("十六进制: {:x}", number);
    println!("十六进制(大写): {:X}", number);
    
    // 使用 format! 宏创建字符串
    let formatted = format!("Hello, {}! 你的分数是 {:.1}", name, score);
    println!("格式化字符串: {}", formatted);
}

// 字符串遍历
fn string_iteration() {
    let text = "Hello, 世界! 🦀";
    
    // 按字符遍历
    println!("按字符遍历:");
    for ch in text.chars() {
        println!("  字符: '{}' (Unicode: U+{:04X})", ch, ch as u32);
    }
    
    // 按字节遍历
    println!("\n按字节遍历:");
    for byte in text.bytes() {
        println!("  字节: {} (0x{:02X})", byte, byte);
    }
    
    // 按字符索引遍历
    println!("\n按字符索引遍历:");
    for (i, ch) in text.char_indices() {
        println!("  索引 {}: '{}'", i, ch);
    }
    
    // 字符串反转
    let reversed: String = text.chars().rev().collect();
    println!("\n反转字符串: {}", reversed);
    
    // 字符过滤
    let only_letters: String = text.chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("只保留字母: {}", only_letters);
    
    // 字符统计
    let char_count = text.chars().count();
    let byte_count = text.len();
    println!("字符数: {}, 字节数: {}", char_count, byte_count);
}

// 字符串切片操作
fn string_slicing() {
    let text = "Hello, 世界! Programming";
    
    // 字节切片 (需要小心UTF-8边界)
    let slice1 = &text[0..5];
    println!("字节切片 [0..5]: '{}'", slice1);
    
    // 获取子字符串 (安全方式)
    if let Some(substring) = text.get(7..13) {
        println!("安全切片 [7..13]: '{}'", substring);
    }
    
    // 字符切片 (更安全)
    let chars: Vec<char> = text.chars().collect();
    let char_slice: String = chars[7..9].iter().collect();
    println!("字符切片: '{}'", char_slice);
    
    // 去除空白字符
    let with_spaces = "  Hello, Rust!  ";
    println!("原始: '{}'", with_spaces);
    println!("去除前后空白: '{}'", with_spaces.trim());
    println!("去除前面空白: '{}'", with_spaces.trim_start());
    println!("去除后面空白: '{}'", with_spaces.trim_end());
    
    // 去除指定字符
    let with_dots = "...Hello...";
    println!("去除点号: '{}'", with_dots.trim_matches('.'));
    
    // 字符串截断
    let long_text = "这是一个很长的字符串，需要截断处理";
    let truncated = if long_text.len() > 15 {
        let mut truncated = String::new();
        for ch in long_text.chars().take(5) {
            truncated.push(ch);
        }
        truncated.push_str("...");
        truncated
    } else {
        long_text.to_string()
    };
    println!("截断后: '{}'", truncated);
}

// 字符串与其他类型转换
fn string_conversion() {
    // 数字转字符串
    let number = 42;
    let number_str = number.to_string();
    println!("数字转字符串: {}", number_str);
    
    // 字符串转数字
    let str_number = "123";
    match str_number.parse::<i32>() {
        Ok(num) => println!("字符串转数字: {}", num),
        Err(e) => println!("转换失败: {}", e),
    }
    
    // 布尔值转换
    let bool_val = true;
    println!("布尔值转字符串: {}", bool_val.to_string());
    
    // 字符转换
    let ch = 'A';
    println!("字符转字符串: {}", ch.to_string());
    
    // 字符串转字符数组
    let text = "Hello";
    let chars: Vec<char> = text.chars().collect();
    println!("字符串转字符数组: {:?}", chars);
    
    // 字符数组转字符串
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let text: String = chars.into_iter().collect();
    println!("字符数组转字符串: {}", text);
    
    // 字节数组转字符串
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello" 的UTF-8字节
    match String::from_utf8(bytes) {
        Ok(s) => println!("字节数组转字符串: {}", s),
        Err(e) => println!("转换失败: {}", e),
    }
}

// 字符串性能优化
fn string_performance() {
    println!("字符串性能优化建议:");
    
    // 1. 预分配容量
    let mut s = String::with_capacity(100);
    s.push_str("预分配容量可以减少内存重新分配");
    println!("预分配容量: {}", s.capacity());
    
    // 2. 使用 &str 而不是 String 作为参数
    fn process_text(text: &str) -> usize {
        text.len()
    }
    
    let text = String::from("Hello");
    let len = process_text(&text); // 传递引用
    println!("处理文本长度: {}", len);
    
    // 3. 避免不必要的克隆
    let original = "Hello, World!";
    let reference = original; // 复制引用，不是数据
    let owned = original.to_owned(); // 只在需要拥有权时使用
    
    println!("原始: {}", original);
    println!("引用: {}", reference);
    println!("拥有: {}", owned);
    
    // 4. 使用 Cow (Clone on Write) 优化
    use std::borrow::Cow;
    
    fn process_maybe_owned(input: &str) -> Cow<str> {
        if input.contains("特殊") {
            Cow::Owned(input.replace("特殊", "普通"))
        } else {
            Cow::Borrowed(input)
        }
    }
    
    let text1 = "普通文本";
    let text2 = "特殊文本";
    
    let result1 = process_maybe_owned(text1);
    let result2 = process_maybe_owned(text2);
    
    println!("COW 结果1: {}", result1);
    println!("COW 结果2: {}", result2);
    
    // 5. 字符串连接性能对比
    performance_comparison();
}

// 性能对比函数
fn performance_comparison() {
    println!("\n字符串连接性能对比:");
    println!("1. String + &str: 适用于简单连接");
    println!("2. String.push_str(): 适用于逐步构建");
    println!("3. format!(): 适用于格式化");
    println!("4. join(): 适用于数组连接");
    println!("5. Vec<String>::concat(): 适用于大量字符串");
    
    // 示例：连接多个字符串的不同方法
    let words = vec!["Hello", " ", "World", "!"];
    
    // 方法1: 迭代连接
    let mut result1 = String::new();
    for word in &words {
        result1.push_str(word);
    }
    
    // 方法2: 使用 join
    let result2 = words.join("");
    
    // 方法3: 使用 concat
    let result3 = words.concat();
    
    println!("结果1: {}", result1);
    println!("结果2: {}", result2);
    println!("结果3: {}", result3);
}

// 自定义类型的字符串表示
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grade: f64,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "学生: {}, 年龄: {}, 成绩: {:.1}", self.name, self.age, self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_creation() {
        let s1 = String::from("Hello");
        let s2 = "Hello".to_string();
        assert_eq!(s1, s2);
    }
    
    #[test]
    fn test_string_manipulation() {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        assert_eq!(s, "Hello, World!");
    }
    
    #[test]
    fn test_string_search() {
        let text = "Hello, World!";
        assert_eq!(text.find("World"), Some(7));
        assert_eq!(text.find("Rust"), None);
    }
    
    #[test]
    fn test_string_parsing() {
        let number_str = "42";
        let number: i32 = number_str.parse().unwrap();
        assert_eq!(number, 42);
    }
    
    #[test]
    fn test_string_formatting() {
        let name = "Alice";
        let age = 30;
        let formatted = format!("Name: {}, Age: {}", name, age);
        assert_eq!(formatted, "Name: Alice, Age: 30");
    }
    
    #[test]
    fn test_student_display() {
        let student = Student {
            name: "张三".to_string(),
            age: 20,
            grade: 89.5,
        };
        let display = format!("{}", student);
        assert_eq!(display, "学生: 张三, 年龄: 20, 成绩: 89.5");
    }
}