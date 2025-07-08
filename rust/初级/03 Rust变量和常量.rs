// 03 Rust变量和常量 - 变量声明、可变性和常量
// 本章介绍变量的声明、可变性、遮蔽和常量的使用

// 常量声明（全局作用域）
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    // 不可变变量
    let x = 5;
    println!("x的值是: {}", x);
    
    // 可变变量
    let mut y = 5;
    println!("y的初始值是: {}", y);
    y = 6;
    println!("y的新值是: {}", y);
    
    // 使用常量
    println!("最大分数: {}", MAX_POINTS);
    println!("π的值: {}", PI);
}

// 案例1：变量遮蔽（Shadowing）
fn shadowing_example() {
    let x = 5;
    println!("第一个x的值: {}", x);
    
    let x = x + 1;  // 遮蔽之前的x
    println!("第二个x的值: {}", x);
    
    {
        let x = x * 2;  // 在内部作用域中遮蔽x
        println!("内部作用域中x的值: {}", x);
    }
    
    println!("回到外部作用域，x的值: {}", x);
    
    // 遮蔽允许改变类型
    let spaces = "   ";         // 字符串类型
    let spaces = spaces.len();  // 数字类型
    println!("空格的数量: {}", spaces);
}

// 案例2：类型转换和类型推断
fn type_conversion_example() {
    // 类型推断
    let guess = "42";
    let guess: u32 = guess.parse().expect("不是一个数字！");
    println!("猜测的数字: {}", guess);
    
    // 显式类型声明
    let x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;
    
    // 类型转换
    let a = x as f64;  // 将i32转换为f64
    println!("转换后的值: {}", a);
    
    // 数值字面量
    let decimal = 98_222;      // 十进制
    let hex = 0xff;            // 十六进制
    let octal = 0o77;          // 八进制
    let binary = 0b1111_0000;  // 二进制
    let byte = b'A';           // 字节（仅限u8）
    
    println!("不同进制的数字:");
    println!("十进制: {}, 十六进制: {}, 八进制: {}, 二进制: {}, 字节: {}", 
             decimal, hex, octal, binary, byte);
}

// 静态变量（类似于全局变量）
static GLOBAL_COUNT: u32 = 0;

fn demonstrate_static() {
    println!("静态变量的值: {}", GLOBAL_COUNT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadowing() {
        shadowing_example();
    }

    #[test]
    fn test_type_conversion() {
        type_conversion_example();
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_POINTS, 100_000);
        assert_eq!(PI, 3.14159);
    }

    #[test]
    fn test_static() {
        demonstrate_static();
    }
}

// 要点总结：
// 1. 变量默认是不可变的，使用mut关键字使其可变
// 2. 常量使用const关键字声明，必须指定类型
// 3. 静态变量使用static关键字声明
// 4. 遮蔽允许重新声明同名变量，甚至可以改变类型
// 5. Rust具有强大的类型推断能力