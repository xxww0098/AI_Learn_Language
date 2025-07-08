// 07 Rust宏编程 - 声明式宏和过程宏
// 本章介绍Rust的宏系统：macro_rules!声明式宏和过程宏

fn main() {
    // 声明式宏示例
    declarative_macros();
    
    // 高级宏模式
    advanced_macro_patterns();
    
    // 实用宏示例
    practical_macros();
    
    // 元编程示例
    metaprogramming_examples();
}

// 案例1：基本声明式宏
macro_rules! say_hello {
    () => {
        println!("Hello, macro world!");
    };
}

// 带参数的宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("你调用了函数: {}", stringify!($func_name));
        }
    };
}

// 多参数宏
macro_rules! print_result {
    ($expression:expr) => {
        println!("{} = {}", stringify!($expression), $expression);
    };
}

// 可变参数宏
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    );
}

// 创建结构体的宏
macro_rules! create_struct {
    ($name:ident {$($field:ident: $type:ty),*}) => {
        struct $name {
            $($field: $type),*
        }
        
        impl $name {
            fn new($($field: $type),*) -> Self {
                $name {
                    $($field),*
                }
            }
        }
    };
}

fn declarative_macros() {
    println!("=== 声明式宏示例 ===");
    
    // 基本宏调用
    say_hello!();
    
    // 创建函数
    create_function!(foo);
    foo();
    
    // 表达式宏
    print_result!(1 + 2);
    print_result!(3 * 4);
    
    // 可变参数宏
    println!("最小值: {}", find_min!(3));
    println!("最小值: {}", find_min!(3, 1, 4, 1, 5));
    
    // 创建结构体
    create_struct!(Person {
        name: String,
        age: u32
    });
    
    let person = Person::new("Alice".to_string(), 25);
    println!("Person: {} ({}岁)", person.name, person.age);
}

// 案例2：高级宏模式
macro_rules! match_type {
    ($x:expr, i32) => {
        println!("{} 是 i32 类型", $x);
    };
    ($x:expr, String) => {
        println!("{} 是 String 类型", $x);
    };
    ($x:expr, $t:ty) => {
        println!("{} 是 {} 类型", stringify!($x), stringify!($t));
    };
}

// 重复模式宏
macro_rules! create_hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// 条件编译宏
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

// DSL宏示例
macro_rules! html {
    ($tag:ident { $($content:tt)* }) => {
        format!("<{}>{}</{}>", stringify!($tag), html_content!($($content)*), stringify!($tag))
    };
    ($tag:ident) => {
        format!("<{} />", stringify!($tag))
    };
}

macro_rules! html_content {
    ($text:literal) => {
        $text
    };
    ($($content:tt)*) => {
        format!("{}", stringify!($($content)*))
    };
}

// 数学表达式解析宏
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
            val
        }
    };
}

// 测试生成宏
macro_rules! test_case {
    ($name:ident: $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($input, $expected);
        }
    };
}

fn advanced_macro_patterns() {
    println!("\n=== 高级宏模式示例 ===");
    
    // 类型匹配
    let x = 42i32;
    let y = "hello".to_string();
    match_type!(x, i32);
    match_type!(y, String);
    
    // 创建HashMap
    let map = create_hash_map!(
        "name" => "Alice",
        "city" => "Beijing",
        "country" => "China"
    );
    println!("Map: {:?}", map);
    
    // 调试打印
    debug_print!("这是调试信息: {}", 42);
    
    // HTML DSL
    let html_content = html!(div { "Hello, World!" });
    println!("HTML: {}", html_content);
    
    let empty_tag = html!(br);
    println!("空标签: {}", empty_tag);
    
    // 数学计算
    calculate!(eval 1 + 2);
    calculate!(eval 3 * 4);
}

// 案例3：实用宏
macro_rules! time_it {
    ($e:expr) => {
        {
            let start = std::time::Instant::now();
            let result = $e;
            let duration = start.elapsed();
            println!("执行时间: {:?}", duration);
            result
        }
    };
}

// 错误处理宏
macro_rules! try_with_log {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                println!("错误: {:?}", err);
                return Err(err.into());
            }
        }
    };
}

// 配置生成宏
macro_rules! config {
    ($($key:ident: $value:expr),*) => {
        {
            #[derive(Debug)]
            struct Config {
                $($key: String),*
            }
            
            Config {
                $($key: $value.to_string()),*
            }
        }
    };
}

// 枚举生成宏
macro_rules! enum_with_display {
    ($name:ident { $($variant:ident),* }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $($variant),*
        }
        
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, stringify!($variant))),*
                }
            }
        }
        
        impl $name {
            fn all_variants() -> Vec<Self> {
                vec![$(Self::$variant),*]
            }
        }
    };
}

// 日志宏
macro_rules! log {
    (info: $($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
    (warn: $($arg:tt)*) => {
        println!("[WARN] {}", format!($($arg)*));
    };
    (error: $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
}

// Builder模式宏
macro_rules! builder {
    ($struct_name:ident {
        $($field:ident: $field_type:ty),*
    }) => {
        #[derive(Debug, Default)]
        struct $struct_name {
            $($field: Option<$field_type>),*
        }
        
        impl $struct_name {
            fn new() -> Self {
                Self::default()
            }
            
            $(
                fn $field(mut self, value: $field_type) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
            
            fn build(self) -> Result<Built<$struct_name>, String> {
                $(
                    let $field = self.$field.ok_or(format!("Missing field: {}", stringify!($field)))?;
                )*
                
                Ok(Built {
                    $($field),*
                })
            }
        }
        
        #[derive(Debug)]
        struct Built<$struct_name> {
            $($field: $field_type),*
        }
    };
}

fn practical_macros() {
    println!("\n=== 实用宏示例 ===");
    
    // 计时宏
    let result = time_it!({
        let mut sum = 0;
        for i in 1..=1000 {
            sum += i;
        }
        sum
    });
    println!("计算结果: {}", result);
    
    // 配置宏
    let config = config!(
        host: "localhost",
        port: "8080",
        database: "mydb"
    );
    println!("配置: {:?}", config);
    
    // 枚举宏
    enum_with_display!(Color { Red, Green, Blue });
    
    let color = Color::Red;
    println!("颜色: {}", color);
    println!("所有颜色: {:?}", Color::all_variants());
    
    // 日志宏
    log!(info: "应用程序启动");
    log!(warn: "这是一个警告: {}", "内存使用率高");
    log!(error: "发生错误: {}", "连接失败");
    
    // Builder模式宏
    builder!(User {
        name: String,
        email: String,
        age: u32
    });
    
    match User::new()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .build()
    {
        Ok(user) => println!("用户构建成功: {:?}", user),
        Err(e) => println!("构建失败: {}", e),
    }
}

// 元编程示例
macro_rules! impl_ops {
    ($struct_name:ident, $($op:ident => $method:ident),*) => {
        $(
            impl std::ops::$op for $struct_name {
                type Output = Self;
                
                fn $method(self, other: Self) -> Self::Output {
                    $struct_name {
                        x: self.x.$method(other.x),
                        y: self.y.$method(other.y),
                    }
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

// 为Point实现运算符
impl_ops!(Point, Add => add, Sub => sub, Mul => mul, Div => div);

// 序列化宏
macro_rules! serializable {
    ($struct_name:ident {
        $($field:ident: $field_type:ty),*
    }) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field: $field_type),*
        }
        
        impl $struct_name {
            fn to_json(&self) -> String {
                let mut fields = Vec::new();
                $(
                    fields.push(format!("\"{}\":{:?}", stringify!($field), self.$field));
                )*
                format!("{{{}}}", fields.join(","))
            }
            
            fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($field)),*]
            }
        }
    };
}

// 状态机宏
macro_rules! state_machine {
    ($name:ident {
        states: [$($state:ident),*],
        events: [$($event:ident),*],
        transitions: {
            $($from:ident + $evt:ident => $to:ident),*
        }
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum State {
            $($state),*
        }
        
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum Event {
            $($event),*
        }
        
        #[derive(Debug)]
        struct $name {
            current_state: State,
        }
        
        impl $name {
            fn new(initial_state: State) -> Self {
                $name {
                    current_state: initial_state,
                }
            }
            
            fn handle_event(&mut self, event: Event) -> Result<(), String> {
                let new_state = match (self.current_state, event) {
                    $((State::$from, Event::$evt) => State::$to,)*
                    _ => return Err(format!("Invalid transition: {:?} + {:?}", self.current_state, event)),
                };
                
                println!("状态转换: {:?} + {:?} => {:?}", self.current_state, event, new_state);
                self.current_state = new_state;
                Ok(())
            }
            
            fn current_state(&self) -> State {
                self.current_state
            }
        }
    };
}

fn metaprogramming_examples() {
    println!("\n=== 元编程示例 ===");
    
    // 运算符重载
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);
    
    let p3 = p1 + p2;
    let p4 = p1 - p2;
    let p5 = p1 * p2;
    let p6 = p1 / p2;
    
    println!("点运算:");
    println!("{:?} + {:?} = {:?}", p1, p2, p3);
    println!("{:?} - {:?} = {:?}", p1, p2, p4);
    println!("{:?} * {:?} = {:?}", p1, p2, p5);
    println!("{:?} / {:?} = {:?}", p1, p2, p6);
    
    // 序列化
    serializable!(Product {
        id: u32,
        name: String,
        price: f64
    });
    
    let product = Product {
        id: 1,
        name: "Rust书籍".to_string(),
        price: 59.99,
    };
    
    println!("产品JSON: {}", product.to_json());
    println!("字段名称: {:?}", Product::field_names());
    
    // 状态机
    state_machine!(TrafficLight {
        states: [Red, Yellow, Green],
        events: [Timer, Emergency],
        transitions: {
            Red + Timer => Green,
            Green + Timer => Yellow,
            Yellow + Timer => Red,
            Red + Emergency => Yellow,
            Green + Emergency => Yellow,
            Yellow + Emergency => Red
        }
    });
    
    let mut light = TrafficLight::new(State::Red);
    println!("初始状态: {:?}", light.current_state());
    
    light.handle_event(Event::Timer).unwrap();
    light.handle_event(Event::Timer).unwrap();
    light.handle_event(Event::Emergency).unwrap();
    
    if let Err(e) = light.handle_event(Event::Timer) {
        println!("无效转换: {}", e);
    }
}

// 过程宏示例（注释掉，因为需要单独的crate）
/*
// 这些是过程宏的示例，需要在单独的proc-macro crate中定义

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// 自定义派生宏
#[proc_macro_derive(MyDebug)]
pub fn my_debug_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "MyDebug({})", stringify!(#name))
            }
        }
    };
    
    TokenStream::from(expanded)
}

// 属性宏
#[proc_macro_attribute]
pub fn benchmark(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    
    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let duration = start.elapsed();
            println!("函数 {} 执行时间: {:?}", stringify!(#fn_name), duration);
            result
        }
    };
    
    TokenStream::from(expanded)
}

// 函数式宏
#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
*/

// 宏调试和测试
macro_rules! debug_macro {
    ($($arg:tt)*) => {
        {
            println!("宏输入: {}", stringify!($($arg)*));
            $($arg)*
        }
    };
}

fn macro_debugging() {
    println!("\n=== 宏调试示例 ===");
    
    let result = debug_macro!(1 + 2 * 3);
    println!("结果: {}", result);
    
    debug_macro!({
        let x = 10;
        let y = 20;
        println!("x + y = {}", x + y);
    });
}

// 条件宏
macro_rules! platform_specific {
    () => {
        #[cfg(target_os = "windows")]
        {
            println!("运行在Windows上");
        }
        
        #[cfg(target_os = "linux")]
        {
            println!("运行在Linux上");
        }
        
        #[cfg(target_os = "macos")]
        {
            println!("运行在macOS上");
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            println!("运行在其他平台上");
        }
    };
}

fn conditional_compilation() {
    println!("\n=== 条件编译示例 ===");
    platform_specific!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // 使用测试生成宏
    test_case!(test_addition: 2 + 2 => 4);
    test_case!(test_multiplication: 3 * 4 => 12);
    
    #[test]
    fn test_find_min() {
        assert_eq!(find_min!(1), 1);
        assert_eq!(find_min!(1, 2, 3), 1);
        assert_eq!(find_min!(3, 1, 4, 1, 5), 1);
    }
    
    #[test]
    fn test_point_operations() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = p1 + p2;
        
        assert_eq!(p3.x, 4.0);
        assert_eq!(p3.y, 6.0);
    }
    
    #[test]
    fn test_state_machine() {
        state_machine!(TestMachine {
            states: [A, B],
            events: [Go],
            transitions: {
                A + Go => B,
                B + Go => A
            }
        });
        
        let mut machine = TestMachine::new(State::A);
        assert_eq!(machine.current_state(), State::A);
        
        machine.handle_event(Event::Go).unwrap();
        assert_eq!(machine.current_state(), State::B);
    }
    
    #[test]
    fn test_examples() {
        declarative_macros();
        advanced_macro_patterns();
        practical_macros();
        metaprogramming_examples();
        macro_debugging();
        conditional_compilation();
    }
}

// 宏编程要点总结：
// 1. macro_rules!用于声明式宏定义
// 2. 宏在编译时展开，实现代码生成
// 3. 使用模式匹配处理不同的输入
// 4. 重复模式($(...)*) 处理可变参数
// 5. 宏可以生成结构体、函数、trait实现等
// 6. 过程宏提供更强大的元编程能力
// 7. 宏是零成本抽象，不影响运行时性能
// 8. 合理使用宏可以减少代码重复，提高开发效率