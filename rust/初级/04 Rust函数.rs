// 04 Rust函数 - 函数定义、参数和返回值
// 本章介绍Rust中函数的定义、参数传递和返回值

fn main() {
    // 调用无参数函数
    print_hello();
    
    // 调用有参数函数
    print_number(42);
    
    // 调用有返回值函数
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
    
    // 调用多参数函数
    print_labeled_measurement(5, 'h');
}

// 基本函数定义
fn print_hello() {
    println!("Hello from a function!");
}

// 带参数的函数
fn print_number(x: i32) {
    println!("数字的值是: {}", x);
}

// 带返回值的函数
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // 没有分号，这是返回值
}

// 多个参数的函数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("测量值是: {}{}", value, unit_label);
}

// 案例1：数学计算函数
fn math_functions() {
    // 基本数学运算
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
    
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    fn divide(a: f64, b: f64) -> f64 {
        if b != 0.0 {
            a / b
        } else {
            0.0  // 避免除零错误
        }
    }
    
    println!("加法: 5 + 3 = {}", add(5, 3));
    println!("减法: 5 - 3 = {}", subtract(5, 3));
    println!("乘法: 5 * 3 = {}", multiply(5, 3));
    println!("除法: 5.0 / 3.0 = {}", divide(5.0, 3.0));
}

// 案例2：表达式和语句
fn expressions_and_statements() {
    // 语句不返回值
    let y = 6;  // 这是一个语句
    
    // 表达式返回值
    let x = {
        let y = 3;
        y + 1  // 表达式，没有分号
    };
    
    println!("x的值是: {}", x);
    
    // 带返回值的函数
    fn five() -> i32 {
        5  // 表达式，返回5
    }
    
    fn plus_one(x: i32) -> i32 {
        x + 1  // 表达式，返回x+1
    }
    
    let num = five();
    println!("five()返回: {}", num);
    
    let result = plus_one(5);
    println!("plus_one(5)返回: {}", result);
}

// 字符串处理函数
fn string_length(s: &str) -> usize {
    s.len()
}

fn greet_person(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 条件函数
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// 递归函数示例
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(5, 3), 8);
        assert_eq!(add_numbers(-2, 7), 5);
    }

    #[test]
    fn test_math_functions() {
        math_functions();
    }

    #[test]
    fn test_expressions_statements() {
        expressions_and_statements();
    }

    #[test]
    fn test_string_functions() {
        assert_eq!(string_length("hello"), 5);
        assert_eq!(greet_person("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_conditional_functions() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(5), false);
        assert_eq!(max(5, 3), 5);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}

// 函数要点总结：
// 1. 函数使用fn关键字定义
// 2. 参数必须指定类型
// 3. 返回值类型使用->指定
// 4. 函数体中最后一个表达式是返回值（不加分号）
// 5. 使用return关键字可以提前返回
// 6. 函数名使用snake_case命名规范