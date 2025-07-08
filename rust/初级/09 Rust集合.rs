// 09 Rust集合 - 向量、字符串和哈希映射
// 本章介绍Rust标准库中的集合类型：Vec、String和HashMap

use std::collections::HashMap;

fn main() {
    // 向量示例
    vector_examples();
    
    // 字符串示例
    string_examples();
    
    // 哈希映射示例
    hashmap_examples();
}

// 案例1：向量（Vec）的使用
fn vector_examples() {
    println!("=== 向量（Vec）示例 ===");
    
    // 创建向量
    let mut v1: Vec<i32> = Vec::new();  // 空向量
    let mut v2 = vec![1, 2, 3];         // 使用宏创建
    
    // 添加元素
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    
    // 访问元素
    let third: &i32 = &v2[2];  // 使用索引
    println!("第三个元素: {}", third);
    
    match v2.get(2) {          // 使用get方法，更安全
        Some(third) => println!("第三个元素: {}", third),
        None => println!("没有第三个元素"),
    }
    
    // 遍历向量
    println!("遍历v2:");
    for i in &v2 {
        println!("  {}", i);
    }
    
    // 修改向量元素
    for i in &mut v2 {
        *i += 50;
    }
    println!("修改后的v2: {:?}", v2);
    
    // 存储不同类型（使用枚举）
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    println!("电子表格行: {:?}", row);
}

// 案例2：字符串（String）的使用
fn string_examples() {
    println!("\n=== 字符串（String）示例 ===");
    
    // 创建字符串
    let mut s1 = String::new();                    // 空字符串
    let s2 = "initial contents".to_string();       // 从字面量创建
    let s3 = String::from("initial contents");     // 从字面量创建
    
    // 添加内容
    s1.push_str("hello");
    s1.push(' ');
    s1.push_str("world");
    println!("s1: {}", s1);
    
    // 字符串连接
    let s4 = s2 + " " + &s3;  // s2被移动，不能再使用
    println!("连接后的字符串: {}", s4);
    
    // 使用format!宏
    let s5 = String::from("hello");
    let s6 = String::from("world");
    let s7 = format!("{} {}", s5, s6);  // s5和s6仍然可用
    println!("使用format!: {}", s7);
    
    // 字符串切片
    let hello = "hello";
    let world = "world";
    let hello_world = format!("{} {}", hello, world);
    
    // 访问字符串字符
    for c in "नमस्ते".chars() {
        println!("字符: {}", c);
    }
    
    // 访问字符串字节
    for b in "hello".bytes() {
        println!("字节: {}", b);
    }
    
    // 字符串操作
    string_operations();
}

fn string_operations() {
    println!("\n--- 字符串操作 ---");
    
    let mut s = String::from("Hello, World!");
    
    // 长度和容量
    println!("长度: {}", s.len());
    println!("容量: {}", s.capacity());
    
    // 检查是否为空
    println!("是否为空: {}", s.is_empty());
    
    // 包含检查
    println!("包含'World': {}", s.contains("World"));
    
    // 开始和结束检查
    println!("以'Hello'开始: {}", s.starts_with("Hello"));
    println!("以'!'结束: {}", s.ends_with("!"));
    
    // 替换
    let new_s = s.replace("World", "Rust");
    println!("替换后: {}", new_s);
    
    // 大小写转换
    println!("大写: {}", s.to_uppercase());
    println!("小写: {}", s.to_lowercase());
    
    // 去除空白
    let padded = "  hello world  ";
    println!("原始: '{}'", padded);
    println!("去除空白: '{}'", padded.trim());
    
    // 分割字符串
    let data = "one,two,three";
    for part in data.split(',') {
        println!("部分: {}", part);
    }
}

// 哈希映射（HashMap）的使用
fn hashmap_examples() {
    println!("\n=== 哈希映射（HashMap）示例 ===");
    
    // 创建哈希映射
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("分数: {:?}", scores);
    
    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue队的分数: {}", s),
        None => println!("Blue队没有分数"),
    }
    
    // 遍历哈希映射
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 只在键不存在时插入
    scores.entry(String::from("Red")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(25);  // Blue已存在，不会更新
    
    println!("添加Red后: {:?}", scores);
    
    // 根据旧值更新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("单词计数: {:?}", map);
    
    // 更多哈希映射操作
    hashmap_operations();
}

fn hashmap_operations() {
    println!("\n--- 哈希映射操作 ---");
    
    let mut map: HashMap<String, i32> = HashMap::new();
    
    // 插入数据
    map.insert("apple".to_string(), 3);
    map.insert("banana".to_string(), 2);
    map.insert("orange".to_string(), 5);
    
    // 检查是否包含键
    println!("包含'apple': {}", map.contains_key("apple"));
    
    // 获取值或默认值
    let apple_count = map.get("apple").unwrap_or(&0);
    println!("苹果数量: {}", apple_count);
    
    // 删除键值对
    if let Some(value) = map.remove("banana") {
        println!("删除了banana: {}", value);
    }
    
    // 长度
    println!("映射长度: {}", map.len());
    
    // 清空
    let mut temp_map = map.clone();
    temp_map.clear();
    println!("清空后长度: {}", temp_map.len());
    
    // 合并两个HashMap
    let mut map1 = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);
    
    let mut map2 = HashMap::new();
    map2.insert("c", 3);
    map2.insert("d", 4);
    
    for (key, value) in map2 {
        map1.insert(key, value);
    }
    
    println!("合并后的映射: {:?}", map1);
}

// 学生成绩管理系统
fn student_grade_system() {
    println!("\n=== 学生成绩管理系统 ===");
    
    let mut grades: HashMap<String, Vec<i32>> = HashMap::new();
    
    // 添加学生成绩
    grades.insert("Alice".to_string(), vec![85, 90, 78]);
    grades.insert("Bob".to_string(), vec![92, 88, 95]);
    grades.insert("Carol".to_string(), vec![76, 82, 79]);
    
    // 计算平均分
    for (student, scores) in &grades {
        let sum: i32 = scores.iter().sum();
        let average = sum as f64 / scores.len() as f64;
        println!("{}: 成绩 {:?}, 平均分 {:.2}", student, scores, average);
    }
    
    // 添加新成绩
    grades.entry("Alice".to_string()).and_modify(|v| v.push(95));
    
    // 添加新学生
    grades.insert("David".to_string(), vec![88, 91, 87]);
    
    println!("\n更新后的成绩:");
    for (student, scores) in &grades {
        let sum: i32 = scores.iter().sum();
        let average = sum as f64 / scores.len() as f64;
        println!("{}: 平均分 {:.2}", student, average);
    }
}

// 购物车系统
fn shopping_cart_system() {
    println!("\n=== 购物车系统 ===");
    
    let mut cart: HashMap<String, (i32, f64)> = HashMap::new();  // (数量, 价格)
    
    // 添加商品
    cart.insert("苹果".to_string(), (5, 2.5));
    cart.insert("香蕉".to_string(), (3, 1.8));
    cart.insert("橙子".to_string(), (2, 3.0));
    
    // 显示购物车
    println!("购物车内容:");
    let mut total = 0.0;
    for (item, (quantity, price)) in &cart {
        let subtotal = *quantity as f64 * price;
        println!("{}: {}个 × {:.2}元 = {:.2}元", item, quantity, price, subtotal);
        total += subtotal;
    }
    println!("总计: {:.2}元", total);
    
    // 更新商品数量
    cart.entry("苹果".to_string()).and_modify(|(q, _)| *q += 2);
    
    // 删除商品
    cart.remove("香蕉");
    
    println!("\n更新后的购物车:");
    let mut total = 0.0;
    for (item, (quantity, price)) in &cart {
        let subtotal = *quantity as f64 * price;
        println!("{}: {}个 × {:.2}元 = {:.2}元", item, quantity, price, subtotal);
        total += subtotal;
    }
    println!("总计: {:.2}元", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], 1);
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::from("hello");
        s.push_str(" world");
        assert_eq!(s, "hello world");
        assert!(s.contains("world"));
    }

    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_examples() {
        vector_examples();
        string_examples();
        hashmap_examples();
        student_grade_system();
        shopping_cart_system();
    }
}

// 集合要点总结：
// 1. Vec<T>用于存储可变数量的相同类型元素
// 2. String是可变的UTF-8编码字符串
// 3. HashMap<K, V>存储键值对映射
// 4. 使用索引访问可能panic，使用get方法更安全
// 5. 遍历集合时要注意所有权规则
// 6. entry API提供了高效的插入和更新方法
// 7. 集合在离开作用域时会自动清理内存