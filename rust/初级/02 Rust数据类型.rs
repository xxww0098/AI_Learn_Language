// 02 Rust数据类型 - 基本数据类型和变量
// 本章介绍Rust的基本数据类型和变量声明

fn main() {
    // 整数类型
    let x: i32 = 42;        // 有符号32位整数
    let y: u32 = 42;        // 无符号32位整数
    
    // 浮点数类型
    let f1: f64 = 3.14;     // 64位浮点数（默认）
    let f2: f32 = 3.14;     // 32位浮点数
    
    // 布尔类型
    let is_true: bool = true;
    let is_false: bool = false;
    
    // 字符类型
    let letter: char = 'A';
    let emoji: char = '😀';
    
    // 字符串类型
    let str_slice: &str = "Hello";      // 字符串切片
    let string: String = String::from("World");  // 字符串
    
    println!("整数: {}, {}", x, y);
    println!("浮点数: {}, {}", f1, f2);
    println!("布尔值: {}, {}", is_true, is_false);
    println!("字符: {}, {}", letter, emoji);
    println!("字符串: {} {}", str_slice, string);
}

// 案例1：基本数学运算
fn basic_math() {
    let a = 5;
    let b = 3;
    
    // 基本运算
    let sum = a + b;        // 加法
    let difference = a - b;  // 减法
    let product = a * b;     // 乘法
    let quotient = a / b;    // 除法
    let remainder = a % b;   // 取余
    
    println!("{}+{}={}", a, b, sum);
    println!("{}*{}={}", a, b, product);
    println!("{}%{}={}", a, b, remainder);
}

// 案例2：变量可变性
fn variable_mutability() {
    let x = 5;              // 不可变变量
    println!("x的值是: {}", x);
    
    // x = 6;  // 这会导致编译错误！
    
    let mut y = 5;          // 可变变量
    println!("y的值是: {}", y);
    y = 6;                  // 这是允许的
    println!("y的新值是: {}", y);
    
    // 变量遮蔽（shadowing）
    let z = 5;
    let z = z * 2;          // 创建新的变量z
    println!("z的值是: {}", z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        basic_math();
    }

    #[test]
    fn test_variable_mutability() {
        variable_mutability();
    }
}

// 数据类型总结：
// 1. 整数类型：i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
// 2. 浮点数类型：f32, f64
// 3. 布尔类型：bool
// 4. 字符类型：char
// 5. 字符串类型：&str, String