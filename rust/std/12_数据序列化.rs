// 12_数据序列化.rs
// Rust标准库数据序列化与反序列化详解

/*
虽然Rust标准库没有直接提供JSON、XML等格式的序列化支持，
但它提供了基础的数据转换和格式化功能：

核心功能：
- Debug trait：调试输出格式化
- Display trait：用户友好的显示格式
- ToString trait：转换为字符串
- FromStr trait：从字符串解析
- 二进制数据处理

重要trait：
- std::fmt 模块提供格式化功能
- std::str 模块提供字符串处理
- std::convert 模块提供类型转换

实际应用中常用的序列化库：
- serde：最流行的序列化框架
- serde_json：JSON支持
- serde_derive：自动生成序列化代码
- bincode：二进制序列化
- rmp：MessagePack格式

本教程重点介绍标准库的基础序列化能力和手动实现方法。
*/

use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("=== Rust标准库数据序列化详解 ===");
    
    // 1. 基础格式化输出
    println!("\n1. 基础格式化输出：");
    basic_formatting();
    
    // 2. 自定义Display实现
    println!("\n2. 自定义Display实现：");
    custom_display();
    
    // 3. Debug格式化
    println!("\n3. Debug格式化：");
    debug_formatting();
    
    // 4. 字符串转换
    println!("\n4. 字符串转换：");
    string_conversion();
    
    // 5. 二进制数据处理
    println!("\n5. 二进制数据处理：");
    binary_data_handling();
    
    // 6. 自定义序列化格式
    println!("\n6. 自定义序列化格式：");
    custom_serialization();
    
    // 7. CSV格式处理
    println!("\n7. CSV格式处理：");
    csv_processing();
    
    // 8. 配置文件格式
    println!("\n8. 配置文件格式：");
    config_file_formats();
    
    // 9. 网络协议序列化
    println!("\n9. 网络协议序列化：");
    network_protocol_serialization();
    
    // 10. 最佳实践
    println!("\n10. 最佳实践：");
    best_practices();
    
    println!("\n=== 数据序列化学习完成 ===");
}

// 基础格式化输出
fn basic_formatting() {
    let number = 42;
    let float = 3.14159;
    let text = "Hello, Rust!";
    let boolean = true;
    
    // 基本格式化
    println!("基本格式化：");
    println!("  整数: {}", number);
    println!("  浮点数: {}", float);
    println!("  字符串: {}", text);
    println!("  布尔值: {}", boolean);
    
    // 数字格式化
    println!("数字格式化：");
    println!("  二进制: {:b}", number);
    println!("  八进制: {:o}", number);
    println!("  十六进制: {:x}", number);
    println!("  十六进制(大写): {:X}", number);
    println!("  指数表示: {:e}", float);
    println!("  固定小数位: {:.2}", float);
    
    // 对齐和填充
    println!("对齐和填充：");
    println!("  左对齐: '{:<10}'", text);
    println!("  右对齐: '{:>10}'", text);
    println!("  居中: '{:^10}'", text);
    println!("  填充字符: '{:*^15}'", "Rust");
    
    // 宽度和精度
    println!("宽度和精度：");
    println!("  最小宽度: '{:8}'", number);
    println!("  零填充: '{:08}'", number);
    println!("  精度控制: '{:.3}'", float);
    println!("  宽度和精度: '{:8.2}'", float);
}

// 自定义Display实现
fn custom_display() {
    // 简单结构体的Display实现
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }
    
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({} 岁) - {}", self.name, self.age, self.email)
        }
    }
    
    let person = Person {
        name: "张三".to_string(),
        age: 30,
        email: "zhangsan@example.com".to_string(),
    };
    
    println!("Person Display: {}", person);
    println!("Person Debug: {:?}", person);
    
    // 复杂结构体的格式化
    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }
    
    impl fmt::Display for Address {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}, {}, {}", self.street, self.city, self.country)
        }
    }
    
    #[derive(Debug)]
    struct Employee {
        person: Person,
        address: Address,
        department: String,
        salary: f64,
    }
    
    impl fmt::Display for Employee {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "员工信息:")?;
            writeln!(f, "  个人: {}", self.person)?;
            writeln!(f, "  地址: {}", self.address)?;
            writeln!(f, "  部门: {}", self.department)?;
            write!(f, "  薪资: {:.2}", self.salary)
        }
    }
    
    let employee = Employee {
        person: Person {
            name: "李四".to_string(),
            age: 28,
            email: "lisi@company.com".to_string(),
        },
        address: Address {
            street: "科技大道123号".to_string(),
            city: "深圳".to_string(),
            country: "中国".to_string(),
        },
        department: "软件开发".to_string(),
        salary: 15000.0,
    };
    
    println!("员工信息显示：\n{}", employee);
}

// Debug格式化
fn debug_formatting() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }
    
    let rect = Rectangle {
        top_left: Point { x: 10, y: 20 },
        bottom_right: Point { x: 30, y: 40 },
    };
    
    println!("Debug格式化：");
    println!("  标准debug: {:?}", rect);
    println!("  美化debug: {:#?}", rect);
    
    // 集合的Debug格式化
    let numbers = vec![1, 2, 3, 4, 5];
    let map: HashMap<&str, i32> = [("a", 1), ("b", 2), ("c", 3)].iter().cloned().collect();
    
    println!("集合Debug格式化：");
    println!("  Vec: {:?}", numbers);
    println!("  Vec (美化): {:#?}", numbers);
    println!("  HashMap: {:?}", map);
    println!("  HashMap (美化): {:#?}", map);
    
    // 自定义Debug实现
    struct CustomDebug {
        data: Vec<i32>,
    }
    
    impl fmt::Debug for CustomDebug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomDebug")
                .field("data", &self.data)
                .field("length", &self.data.len())
                .field("sum", &self.data.iter().sum::<i32>())
                .finish()
        }
    }
    
    let custom = CustomDebug { data: vec![1, 2, 3, 4, 5] };
    println!("自定义Debug: {:#?}", custom);
}

// 字符串转换
fn string_conversion() {
    // ToString trait
    let number = 42;
    let float = 3.14;
    let boolean = true;
    
    println!("ToString转换：");
    println!("  数字转字符串: {}", number.to_string());
    println!("  浮点数转字符串: {}", float.to_string());
    println!("  布尔值转字符串: {}", boolean.to_string());
    
    // FromStr trait
    println!("FromStr解析：");
    
    let number_str = "123";
    match number_str.parse::<i32>() {
        Ok(num) => println!("  解析数字成功: {}", num),
        Err(e) => println!("  解析数字失败: {}", e),
    }
    
    let float_str = "3.14159";
    match float_str.parse::<f64>() {
        Ok(num) => println!("  解析浮点数成功: {}", num),
        Err(e) => println!("  解析浮点数失败: {}", e),
    }
    
    let bool_str = "true";
    match bool_str.parse::<bool>() {
        Ok(b) => println!("  解析布尔值成功: {}", b),
        Err(e) => println!("  解析布尔值失败: {}", e),
    }
    
    // 自定义FromStr实现
    custom_fromstr_example();
    
    // 复杂数据的字符串表示
    complex_string_representation();
}

// 自定义FromStr实现
fn custom_fromstr_example() {
    #[derive(Debug, PartialEq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    
    impl FromStr for Color {
        type Err = String;
        
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.starts_with('#') || s.len() != 7 {
                return Err("颜色格式错误，应为 #RRGGBB".to_string());
            }
            
            let r = u8::from_str_radix(&s[1..3], 16)
                .map_err(|_| "红色分量解析失败")?;
            let g = u8::from_str_radix(&s[3..5], 16)
                .map_err(|_| "绿色分量解析失败")?;
            let b = u8::from_str_radix(&s[5..7], 16)
                .map_err(|_| "蓝色分量解析失败")?;
            
            Ok(Color { r, g, b })
        }
    }
    
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
        }
    }
    
    println!("自定义FromStr示例：");
    
    let colors = ["#FF0000", "#00FF00", "#0000FF", "#INVALID"];
    for color_str in &colors {
        match color_str.parse::<Color>() {
            Ok(color) => println!("  {} -> {}", color_str, color),
            Err(e) => println!("  {} -> 错误: {}", color_str, e),
        }
    }
}

// 复杂数据的字符串表示
fn complex_string_representation() {
    #[derive(Debug)]
    struct Task {
        id: u32,
        name: String,
        completed: bool,
        priority: Priority,
    }
    
    #[derive(Debug)]
    enum Priority {
        Low,
        Medium,
        High,
        Critical,
    }
    
    impl fmt::Display for Priority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Priority::Low => write!(f, "低"),
                Priority::Medium => write!(f, "中"),
                Priority::High => write!(f, "高"),
                Priority::Critical => write!(f, "紧急"),
            }
        }
    }
    
    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let status = if self.completed { "✓" } else { "○" };
            write!(f, "{} [{}] {} (优先级: {})", 
                   status, self.id, self.name, self.priority)
        }
    }
    
    let tasks = vec![
        Task { id: 1, name: "完成项目文档".to_string(), completed: true, priority: Priority::High },
        Task { id: 2, name: "代码审查".to_string(), completed: false, priority: Priority::Medium },
        Task { id: 3, name: "修复关键Bug".to_string(), completed: false, priority: Priority::Critical },
    ];
    
    println!("任务列表：");
    for task in &tasks {
        println!("  {}", task);
    }
}

// 二进制数据处理
fn binary_data_handling() {
    // 基本二进制序列化
    println!("二进制数据序列化：");
    
    let number: u32 = 0x12345678;
    let bytes = number.to_le_bytes(); // 小端字节序
    println!("  数字 {} 的小端字节: {:02X?}", number, bytes);
    
    let bytes_be = number.to_be_bytes(); // 大端字节序
    println!("  数字 {} 的大端字节: {:02X?}", number, bytes_be);
    
    // 从字节重建数字
    let reconstructed = u32::from_le_bytes(bytes);
    println!("  从小端字节重建: {}", reconstructed);
    
    // 浮点数序列化
    let float: f64 = 3.14159265359;
    let float_bytes = float.to_le_bytes();
    println!("  浮点数 {} 的字节: {:02X?}", float, float_bytes);
    
    let reconstructed_float = f64::from_le_bytes(float_bytes);
    println!("  重建的浮点数: {}", reconstructed_float);
    
    // 字符串的UTF-8字节表示
    let text = "Hello, 世界! 🦀";
    let text_bytes = text.as_bytes();
    println!("  字符串 '{}' 的UTF-8字节: {:02X?}", text, text_bytes);
    
    // 从UTF-8字节重建字符串
    match std::str::from_utf8(text_bytes) {
        Ok(reconstructed_text) => println!("  重建的字符串: '{}'", reconstructed_text),
        Err(e) => println!("  字符串重建失败: {}", e),
    }
    
    // 复杂数据结构的二进制表示
    binary_struct_serialization();
}

// 结构体的二进制序列化
fn binary_struct_serialization() {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    struct Point3D {
        x: f32,
        y: f32,
        z: f32,
    }
    
    impl Point3D {
        fn to_bytes(&self) -> [u8; 12] {
            let mut bytes = [0u8; 12];
            bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
            bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
            bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
            bytes
        }
        
        fn from_bytes(bytes: &[u8; 12]) -> Self {
            let x = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let y = f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let z = f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            Point3D { x, y, z }
        }
    }
    
    let point = Point3D { x: 1.0, y: 2.5, z: -3.7 };
    println!("  原始点: {:?}", point);
    
    let bytes = point.to_bytes();
    println!("  序列化字节: {:02X?}", bytes);
    
    let reconstructed = Point3D::from_bytes(&bytes);
    println!("  反序列化点: {:?}", reconstructed);
    
    // 验证数据完整性
    let epsilon = 0.0001;
    let is_equal = (point.x - reconstructed.x).abs() < epsilon &&
                   (point.y - reconstructed.y).abs() < epsilon &&
                   (point.z - reconstructed.z).abs() < epsilon;
    
    println!("  数据完整性: {}", if is_equal { "✓ 通过" } else { "✗ 失败" });
}

// 自定义序列化格式
fn custom_serialization() {
    // 键值对格式
    key_value_serialization();
    
    // 简单的JSON风格序列化
    json_style_serialization();
    
    // XML风格序列化
    xml_style_serialization();
}

// 键值对序列化
fn key_value_serialization() {
    println!("键值对序列化：");
    
    struct Config {
        host: String,
        port: u16,
        debug: bool,
        timeout: f64,
    }
    
    impl Config {
        fn serialize(&self) -> String {
            format!("host={}\nport={}\ndebug={}\ntimeout={}", 
                    self.host, self.port, self.debug, self.timeout)
        }
        
        fn deserialize(data: &str) -> Result<Self, String> {
            let mut host = String::new();
            let mut port = 0;
            let mut debug = false;
            let mut timeout = 0.0;
            
            for line in data.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    match key {
                        "host" => host = value.to_string(),
                        "port" => port = value.parse().map_err(|_| "端口解析失败")?,
                        "debug" => debug = value.parse().map_err(|_| "调试标志解析失败")?,
                        "timeout" => timeout = value.parse().map_err(|_| "超时时间解析失败")?,
                        _ => return Err(format!("未知配置项: {}", key)),
                    }
                }
            }
            
            Ok(Config { host, port, debug, timeout })
        }
    }
    
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
        timeout: 30.0,
    };
    
    let serialized = config.serialize();
    println!("  序列化配置:\n{}", serialized);
    
    match Config::deserialize(&serialized) {
        Ok(deserialized) => {
            println!("  反序列化成功:");
            println!("    host: {}", deserialized.host);
            println!("    port: {}", deserialized.port);
            println!("    debug: {}", deserialized.debug);
            println!("    timeout: {}", deserialized.timeout);
        }
        Err(e) => println!("  反序列化失败: {}", e),
    }
}

// JSON风格序列化
fn json_style_serialization() {
    println!("JSON风格序列化：");
    
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
        active: bool,
    }
    
    impl User {
        fn to_json(&self) -> String {
            format!(r#"{{
  "id": {},
  "name": "{}",
  "email": "{}",
  "active": {}
}}"#, self.id, self.name, self.email, self.active)
        }
        
        // 简化的JSON解析（实际应用中应使用专门的JSON库）
        fn from_json_simple(json: &str) -> Result<Self, String> {
            // 这是一个非常简化的解析器，仅用于演示
            let mut id = 0;
            let mut name = String::new();
            let mut email = String::new();
            let mut active = false;
            
            for line in json.lines() {
                let line = line.trim();
                if line.starts_with('"') && line.contains(':') {
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim().trim_matches('"');
                        let value = value.trim().trim_end_matches(',');
                        
                        match key {
                            "id" => id = value.parse().map_err(|_| "ID解析失败")?,
                            "name" => name = value.trim_matches('"').to_string(),
                            "email" => email = value.trim_matches('"').to_string(),
                            "active" => active = value.parse().map_err(|_| "active解析失败")?,
                            _ => {}
                        }
                    }
                }
            }
            
            Ok(User { id, name, email, active })
        }
    }
    
    let user = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        active: true,
    };
    
    let json = user.to_json();
    println!("  序列化为JSON:\n{}", json);
    
    match User::from_json_simple(&json) {
        Ok(parsed_user) => {
            println!("  反序列化成功: {:?}", parsed_user);
        }
        Err(e) => println!("  反序列化失败: {}", e),
    }
}

// XML风格序列化
fn xml_style_serialization() {
    println!("XML风格序列化：");
    
    #[derive(Debug)]
    struct Book {
        title: String,
        author: String,
        year: u16,
        pages: u32,
    }
    
    impl Book {
        fn to_xml(&self) -> String {
            format!(r#"<book>
  <title>{}</title>
  <author>{}</author>
  <year>{}</year>
  <pages>{}</pages>
</book>"#, self.title, self.author, self.year, self.pages)
        }
    }
    
    let book = Book {
        title: "Rust编程指南".to_string(),
        author: "Rust专家".to_string(),
        year: 2023,
        pages: 500,
    };
    
    let xml = book.to_xml();
    println!("  序列化为XML:\n{}", xml);
}

// CSV格式处理
fn csv_processing() {
    println!("CSV格式处理：");
    
    #[derive(Debug)]
    struct Student {
        id: u32,
        name: String,
        age: u8,
        grade: f64,
    }
    
    impl Student {
        fn to_csv_header() -> String {
            "ID,姓名,年龄,成绩".to_string()
        }
        
        fn to_csv(&self) -> String {
            format!("{},{},{},{}", self.id, self.name, self.age, self.grade)
        }
        
        fn from_csv(line: &str) -> Result<Self, String> {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() != 4 {
                return Err("CSV字段数量不正确".to_string());
            }
            
            let id = fields[0].parse().map_err(|_| "ID解析失败")?;
            let name = fields[1].to_string();
            let age = fields[2].parse().map_err(|_| "年龄解析失败")?;
            let grade = fields[3].parse().map_err(|_| "成绩解析失败")?;
            
            Ok(Student { id, name, age, grade })
        }
    }
    
    let students = vec![
        Student { id: 1, name: "张三".to_string(), age: 20, grade: 85.5 },
        Student { id: 2, name: "李四".to_string(), age: 21, grade: 92.0 },
        Student { id: 3, name: "王五".to_string(), age: 19, grade: 78.5 },
    ];
    
    // 序列化为CSV
    let mut csv_data = String::new();
    csv_data.push_str(&Student::to_csv_header());
    csv_data.push('\n');
    
    for student in &students {
        csv_data.push_str(&student.to_csv());
        csv_data.push('\n');
    }
    
    println!("  CSV数据:\n{}", csv_data);
    
    // 从CSV反序列化
    println!("  反序列化结果:");
    let lines: Vec<&str> = csv_data.lines().collect();
    for (i, line) in lines.iter().skip(1).enumerate() {
        match Student::from_csv(line) {
            Ok(student) => println!("    学生{}: {:?}", i + 1, student),
            Err(e) => println!("    解析第{}行失败: {}", i + 1, e),
        }
    }
}

// 配置文件格式
fn config_file_formats() {
    // INI格式
    ini_format_example();
    
    // TOML风格格式
    toml_style_example();
    
    // 环境变量风格
    env_style_example();
}

// INI格式示例
fn ini_format_example() {
    println!("INI格式示例：");
    
    struct IniConfig {
        sections: HashMap<String, HashMap<String, String>>,
    }
    
    impl IniConfig {
        fn new() -> Self {
            IniConfig { sections: HashMap::new() }
        }
        
        fn set(&mut self, section: &str, key: &str, value: &str) {
            self.sections
                .entry(section.to_string())
                .or_insert_with(HashMap::new)
                .insert(key.to_string(), value.to_string());
        }
        
        fn to_ini(&self) -> String {
            let mut result = String::new();
            for (section, kvs) in &self.sections {
                result.push_str(&format!("[{}]\n", section));
                for (key, value) in kvs {
                    result.push_str(&format!("{}={}\n", key, value));
                }
                result.push('\n');
            }
            result
        }
    }
    
    let mut config = IniConfig::new();
    config.set("database", "host", "localhost");
    config.set("database", "port", "5432");
    config.set("database", "name", "myapp");
    config.set("server", "host", "0.0.0.0");
    config.set("server", "port", "8080");
    
    let ini_data = config.to_ini();
    println!("  INI配置:\n{}", ini_data);
}

// TOML风格示例
fn toml_style_example() {
    println!("TOML风格示例：");
    
    let toml_config = r#"[package]
name = "my-app"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = "1.0"

[dev-dependencies]
criterion = "0.4"
"#;
    
    println!("  TOML配置:\n{}", toml_config);
}

// 环境变量风格示例
fn env_style_example() {
    println!("环境变量风格示例：");
    
    let env_config = r#"DATABASE_URL=postgresql://localhost/myapp
REDIS_URL=redis://localhost:6379
LOG_LEVEL=info
DEBUG=false
PORT=8080
"#;
    
    println!("  环境变量配置:\n{}", env_config);
    
    // 解析环境变量格式
    println!("  解析结果:");
    for line in env_config.lines() {
        if let Some((key, value)) = line.split_once('=') {
            println!("    {} = {}", key, value);
        }
    }
}

// 网络协议序列化
fn network_protocol_serialization() {
    // HTTP风格消息
    http_message_example();
    
    // 自定义协议
    custom_protocol_example();
    
    // 长度前缀协议
    length_prefixed_protocol();
}

// HTTP消息示例
fn http_message_example() {
    println!("HTTP消息序列化：");
    
    struct HttpRequest {
        method: String,
        path: String,
        version: String,
        headers: HashMap<String, String>,
        body: String,
    }
    
    impl HttpRequest {
        fn serialize(&self) -> String {
            let mut result = format!("{} {} {}\r\n", self.method, self.path, self.version);
            
            for (key, value) in &self.headers {
                result.push_str(&format!("{}: {}\r\n", key, value));
            }
            
            result.push_str("\r\n");
            result.push_str(&self.body);
            
            result
        }
    }
    
    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), "example.com".to_string());
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Content-Length".to_string(), "13".to_string());
    
    let request = HttpRequest {
        method: "POST".to_string(),
        path: "/api/users".to_string(),
        version: "HTTP/1.1".to_string(),
        headers,
        body: r#"{"name":"张三"}"#.to_string(),
    };
    
    let serialized = request.serialize();
    println!("  HTTP请求:\n{}", serialized);
}

// 自定义协议示例
fn custom_protocol_example() {
    println!("自定义协议示例：");
    
    #[derive(Debug)]
    struct Message {
        msg_type: u8,
        sequence: u32,
        payload: Vec<u8>,
    }
    
    impl Message {
        fn serialize(&self) -> Vec<u8> {
            let mut result = Vec::new();
            
            // 消息类型 (1字节)
            result.push(self.msg_type);
            
            // 序列号 (4字节，大端)
            result.extend_from_slice(&self.sequence.to_be_bytes());
            
            // 载荷长度 (4字节，大端)
            result.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
            
            // 载荷数据
            result.extend_from_slice(&self.payload);
            
            result
        }
        
        fn deserialize(data: &[u8]) -> Result<Self, String> {
            if data.len() < 9 {
                return Err("数据太短".to_string());
            }
            
            let msg_type = data[0];
            let sequence = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
            let payload_len = u32::from_be_bytes([data[5], data[6], data[7], data[8]]) as usize;
            
            if data.len() < 9 + payload_len {
                return Err("载荷数据不完整".to_string());
            }
            
            let payload = data[9..9 + payload_len].to_vec();
            
            Ok(Message { msg_type, sequence, payload })
        }
    }
    
    let message = Message {
        msg_type: 1,
        sequence: 12345,
        payload: "Hello, World!".as_bytes().to_vec(),
    };
    
    let serialized = message.serialize();
    println!("  序列化消息: {:02X?}", serialized);
    
    match Message::deserialize(&serialized) {
        Ok(deserialized) => {
            println!("  反序列化成功: {:?}", deserialized);
            println!("  载荷内容: {}", String::from_utf8_lossy(&deserialized.payload));
        }
        Err(e) => println!("  反序列化失败: {}", e),
    }
}

// 长度前缀协议
fn length_prefixed_protocol() {
    println!("长度前缀协议：");
    
    fn encode_string(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        
        // 长度前缀 (4字节)
        result.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        
        // 字符串数据
        result.extend_from_slice(bytes);
        
        result
    }
    
    fn decode_string(data: &[u8]) -> Result<(String, usize), String> {
        if data.len() < 4 {
            return Err("数据太短".to_string());
        }
        
        let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        
        if data.len() < 4 + length {
            return Err("字符串数据不完整".to_string());
        }
        
        let string_bytes = &data[4..4 + length];
        let string = String::from_utf8(string_bytes.to_vec())
            .map_err(|_| "无效的UTF-8数据")?;
        
        Ok((string, 4 + length))
    }
    
    let messages = ["Hello", "World", "Rust编程"];
    let mut encoded_data = Vec::new();
    
    for msg in &messages {
        encoded_data.extend_from_slice(&encode_string(msg));
    }
    
    println!("  编码数据: {:02X?}", encoded_data);
    
    // 解码
    let mut offset = 0;
    let mut decoded_messages = Vec::new();
    
    while offset < encoded_data.len() {
        match decode_string(&encoded_data[offset..]) {
            Ok((message, consumed)) => {
                decoded_messages.push(message);
                offset += consumed;
            }
            Err(e) => {
                println!("  解码失败: {}", e);
                break;
            }
        }
    }
    
    println!("  解码消息: {:?}", decoded_messages);
}

// 最佳实践
fn best_practices() {
    println!("序列化最佳实践：");
    println!("1. 选择合适的序列化格式");
    println!("   - JSON: 人类可读，广泛支持，但较大");
    println!("   - 二进制: 紧凑高效，但不可读");
    println!("   - MessagePack: 紧凑且结构化");
    println!("   - Protocol Buffers: 强类型，向后兼容");
    
    println!("2. 错误处理");
    println!("   - 优雅处理序列化/反序列化错误");
    println!("   - 提供有意义的错误消息");
    println!("   - 验证数据完整性");
    
    println!("3. 性能考虑");
    println!("   - 预分配缓冲区大小");
    println!("   - 使用零拷贝序列化");
    println!("   - 批量处理提高效率");
    
    println!("4. 安全性");
    println!("   - 验证输入数据");
    println!("   - 防止缓冲区溢出");
    println!("   - 限制递归深度");
    
    println!("5. 版本兼容性");
    println!("   - 设计可扩展的格式");
    println!("   - 支持版本迁移");
    println!("   - 保持向后兼容");
    
    // 实际建议
    practical_recommendations();
}

// 实际建议
fn practical_recommendations() {
    println!("\n实际使用建议：");
    println!("推荐的序列化库：");
    println!("  - serde: 最全面的序列化框架");
    println!("  - serde_json: JSON支持");
    println!("  - bincode: 高效二进制序列化");
    println!("  - postcard: 嵌入式友好的序列化");
    println!("  - rmp-serde: MessagePack支持");
    
    println!("\n使用场景：");
    println!("  - Web API: JSON");
    println!("  - 配置文件: TOML/YAML");
    println!("  - 数据库存储: 二进制格式");
    println!("  - 网络协议: 自定义二进制格式");
    println!("  - 日志记录: 结构化文本格式");
    
    println!("\n示例Cargo.toml依赖：");
    println!(r#"[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
bincode = "1.3"
toml = "0.8"
"#);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_number_serialization() {
        let number: u32 = 0x12345678;
        let bytes = number.to_le_bytes();
        let reconstructed = u32::from_le_bytes(bytes);
        assert_eq!(number, reconstructed);
    }
    
    #[test]
    fn test_string_conversion() {
        let number = 42;
        let str_repr = number.to_string();
        let parsed: i32 = str_repr.parse().unwrap();
        assert_eq!(number, parsed);
    }
    
    #[test]
    fn test_custom_serialization() {
        #[derive(Debug, PartialEq)]
        struct Point { x: i32, y: i32 }
        
        impl Point {
            fn serialize(&self) -> String {
                format!("{},{}", self.x, self.y)
            }
            
            fn deserialize(s: &str) -> Result<Self, String> {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 2 {
                    return Err("格式错误".to_string());
                }
                
                let x = parts[0].parse().map_err(|_| "x解析失败")?;
                let y = parts[1].parse().map_err(|_| "y解析失败")?;
                
                Ok(Point { x, y })
            }
        }
        
        let point = Point { x: 10, y: 20 };
        let serialized = point.serialize();
        let deserialized = Point::deserialize(&serialized).unwrap();
        
        assert_eq!(point, deserialized);
    }
    
    #[test]
    fn test_binary_serialization() {
        let data = vec![1u8, 2, 3, 4, 5];
        let serialized = data.clone();
        let deserialized = serialized;
        
        assert_eq!(data, deserialized);
    }
}