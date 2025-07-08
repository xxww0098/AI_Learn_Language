// 06 Rust所有权 - 理解Rust的核心概念：所有权、借用和生命周期
// 本章介绍Rust最重要的概念：所有权系统

fn main() {
    // 所有权基础
    ownership_basics();
    
    // 移动语义
    move_semantics();
    
    // 借用和引用
    borrowing_examples();
}

// 所有权基础概念
fn ownership_basics() {
    println!("=== 所有权基础 ===");
    
    // 字符串字面量存储在程序的二进制文件中
    let s = "hello";  // 字符串字面量，存储在栈上
    println!("字符串字面量: {}", s);
    
    // String类型存储在堆上
    let mut s = String::from("hello");  // 在堆上分配内存
    s.push_str(", world!");  // 可以修改
    println!("String类型: {}", s);
    
    // 作用域结束时，Rust自动调用drop释放内存
} // s在这里被释放

// 案例1：移动语义演示
fn move_semantics() {
    println!("\n=== 移动语义 ===");
    
    // 整数类型的复制
    let x = 5;
    let y = x;  // 复制x的值给y
    println!("x = {}, y = {}", x, y);  // 两个变量都可以使用
    
    // String类型的移动
    let s1 = String::from("hello");
    let s2 = s1;  // s1的所有权移动给s2
    // println!("{}", s1);  // 编译错误！s1不再有效
    println!("s2 = {}", s2);  // 只有s2可以使用
    
    // 克隆数据
    let s3 = String::from("hello");
    let s4 = s3.clone();  // 深拷贝
    println!("s3 = {}, s4 = {}", s3, s4);  // 两个都可以使用
    
    // 函数调用中的所有权转移
    let s = String::from("hello");
    takes_ownership(s);  // s的所有权移动到函数中
    // println!("{}", s);  // 编译错误！s不再有效
    
    let x = 5;
    makes_copy(x);  // i32实现了Copy trait，所以是复制
    println!("x = {}", x);  // x仍然可以使用
}

// 接收String所有权的函数
fn takes_ownership(some_string: String) {
    println!("函数接收到: {}", some_string);
}  // some_string在这里被销毁

// 接收i32副本的函数
fn makes_copy(some_integer: i32) {
    println!("函数接收到: {}", some_integer);
}

// 案例2：借用和引用
fn borrowing_examples() {
    println!("\n=== 借用和引用 ===");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 借用s1
    println!("'{}' 的长度是 {}", s1, len);  // s1仍然可以使用
    
    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);  // 可变借用
    println!("修改后的字符串: {}", s);
    
    // 多个不可变引用
    let s = String::from("hello");
    let r1 = &s;  // 不可变引用
    let r2 = &s;  // 不可变引用
    println!("r1: {}, r2: {}", r1, r2);
    
    // 可变引用和不可变引用不能同时存在
    let mut s = String::from("hello");
    let r1 = &s;      // 不可变引用
    let r2 = &s;      // 不可变引用
    println!("r1: {}, r2: {}", r1, r2);
    // r1和r2在这里不再被使用
    
    let r3 = &mut s;  // 可变引用
    println!("r3: {}", r3);
}

// 计算字符串长度而不获取所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s是引用，不会被销毁

// 修改字符串内容
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 返回引用的函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 所有权规则演示
fn ownership_rules() {
    println!("\n=== 所有权规则 ===");
    
    // 规则1：每个值都有一个所有者
    let s = String::from("hello");  // s是"hello"的所有者
    
    // 规则2：在任意时刻，值只能有一个所有者
    let s2 = s;  // 所有权从s转移到s2
    
    // 规则3：当所有者离开作用域时，值将被销毁
    {
        let s3 = String::from("temporary");
        println!("s3: {}", s3);
    }  // s3在这里被销毁
    
    println!("s2: {}", s2);
}

// 字符串切片示例
fn string_slices() {
    println!("\n=== 字符串切片 ===");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];   // 或 &s[..5]
    let world = &s[6..11];  // 或 &s[6..]
    let whole = &s[..];     // 整个字符串的切片
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("whole: {}", whole);
    
    // 使用first_word函数
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 字符串字面量就是切片
    let s = "Hello, world!";  // s的类型是&str
    let word = first_word(s);
    println!("字面量的第一个单词: {}", word);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_basics() {
        ownership_basics();
    }

    #[test]
    fn test_move_semantics() {
        move_semantics();
    }

    #[test]
    fn test_borrowing() {
        borrowing_examples();
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
        
        let s = String::from("hello");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_string_slices() {
        string_slices();
    }

    #[test]
    fn test_ownership_rules() {
        ownership_rules();
    }
}

// 所有权要点总结：
// 1. 每个值都有一个所有者
// 2. 在任意时刻，值只能有一个所有者
// 3. 当所有者离开作用域时，值将被销毁
// 4. 移动语义防止数据竞争
// 5. 借用允许使用值而不获取所有权
// 6. 引用默认是不可变的
// 7. 可变引用和不可变引用不能同时存在