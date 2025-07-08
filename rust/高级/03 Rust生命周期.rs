// 03 Rust生命周期 - 理解和使用生命周期参数
// 本章介绍Rust的生命周期系统：生命周期注解、函数签名和结构体中的生命周期

fn main() {
    // 基本生命周期示例
    basic_lifetimes();
    
    // 函数中的生命周期
    function_lifetimes();
    
    // 结构体中的生命周期
    struct_lifetimes();
    
    // 静态生命周期示例
    static_lifetimes();
}

// 案例1：基本生命周期概念
fn basic_lifetimes() {
    println!("=== 基本生命周期示例 ===");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("较长的字符串是: {}", result);
    
    // 生命周期的作用域
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("在内部作用域中，较长的字符串是: {}", result);
    }
    // string2在这里已经被销毁，但这是安全的，因为result没有逃出作用域
    
    // 字符串切片示例
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("第一个单词: {}", first);
    
    let whole = get_whole_string(&s);
    println!("整个字符串: {}", whole);
}

// 生命周期注解：指定参数和返回值的生命周期关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 只有一个引用参数时，通常不需要生命周期注解（生命周期省略）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 返回值的生命周期与输入参数相同
fn get_whole_string(s: &str) -> &str {
    s
}

// 案例2：函数中的复杂生命周期
fn function_lifetimes() {
    println!("\n=== 函数生命周期示例 ===");
    
    let string1 = String::from("Hello");
    let string2 = String::from("World!");
    
    // 比较字符串
    let longer = longest_with_an_announcement(
        string1.as_str(),
        string2.as_str(),
        "今天是比较字符串的日子",
    );
    println!("较长的字符串: {}", longer);
    
    // 分析文本
    let text = "Hello, world! This is a test.";
    let analysis = analyze_text(text);
    println!("文本分析: {:?}", analysis);
    
    // 提取信息
    let data = "name:Alice,age:30,city:Beijing";
    if let Some(name) = extract_field(data, "name") {
        println!("提取的姓名: {}", name);
    }
    
    // 查找模式
    let content = "The quick brown fox jumps over the lazy dog";
    let patterns = vec!["quick", "lazy", "cat"];
    if let Some(found) = find_first_pattern(content, &patterns) {
        println!("找到的模式: {}", found);
    }
}

// 多个生命周期参数和泛型
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("公告: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 返回元组，包含多个引用
fn analyze_text(text: &str) -> (&str, usize, &str) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let first_word = words.first().unwrap_or(&"");
    let last_word = words.last().unwrap_or(&"");
    let word_count = words.len();
    
    (first_word, word_count, last_word)
}

// 在字符串中查找字段值
fn extract_field<'a>(data: &'a str, field: &str) -> Option<&'a str> {
    for part in data.split(',') {
        if let Some(colon_pos) = part.find(':') {
            let key = &part[..colon_pos];
            let value = &part[colon_pos + 1..];
            if key == field {
                return Some(value);
            }
        }
    }
    None
}

// 在文本中查找第一个匹配的模式
fn find_first_pattern<'a>(text: &'a str, patterns: &[&str]) -> Option<&'a str> {
    for &pattern in patterns {
        if text.contains(pattern) {
            return Some(pattern);
        }
    }
    None
}

// 结构体中的生命周期
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 方法中的生命周期注解
    fn level(&self) -> i32 {
        3
    }
    
    // 返回引用的方法
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
    
    // 多个生命周期参数的方法
    fn compare_parts(&self, other: &ImportantExcerpt) -> &str {
        if self.part.len() > other.part.len() {
            self.part
        } else {
            other.part
        }
    }
}

// 带有多个引用字段的结构体
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    isbn: &'a str,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str, isbn: &'a str) -> Self {
        Book { title, author, isbn }
    }
    
    fn get_info(&self) -> String {
        format!("《{}》 by {} (ISBN: {})", self.title, self.author, self.isbn)
    }
    
    fn title(&self) -> &str {
        self.title
    }
}

// 案例3：结构体生命周期
fn struct_lifetimes() {
    println!("\n=== 结构体生命周期示例 ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {:?}", excerpt);
    println!("重要程度: {}", excerpt.level());
    
    let returned_part = excerpt.announce_and_return_part("这是一个重要的摘录");
    println!("返回的部分: {}", returned_part);
    
    // 创建书籍
    let title = "Rust程序设计语言";
    let author = "Steve Klabnik & Carol Nichols";
    let isbn = "978-7-121-32718-2";
    
    let book = Book::new(title, author, isbn);
    println!("书籍信息: {}", book.get_info());
    println!("书名: {}", book.title());
    
    // 比较两个摘录
    let text1 = "短文本";
    let text2 = "这是一个比较长的文本内容";
    
    let excerpt1 = ImportantExcerpt { part: text1 };
    let excerpt2 = ImportantExcerpt { part: text2 };
    
    let longer_part = excerpt1.compare_parts(&excerpt2);
    println!("更长的部分: {}", longer_part);
}

// 静态生命周期
static GLOBAL_STRING: &str = "这是一个全局字符串";

fn get_static_str() -> &'static str {
    "这是一个静态字符串"
}

// 生命周期子类型化示例
fn static_lifetimes() {
    println!("\n=== 静态生命周期示例 ===");
    
    let static_str = get_static_str();
    println!("静态字符串: {}", static_str);
    println!("全局字符串: {}", GLOBAL_STRING);
    
    // 字符串字面量具有'static生命周期
    let string_literal = "我是字符串字面量";
    let result = longest(GLOBAL_STRING, string_literal);
    println!("比较结果: {}", result);
}

// 高级生命周期示例：缓存结构
struct Cache<'a> {
    data: std::collections::HashMap<&'a str, i32>,
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Cache {
            data: std::collections::HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: &'a str, value: i32) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&i32> {
        self.data.get(key)
    }
}

// 解析器示例
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn parse_number(&mut self) -> Option<&'a str> {
        let start = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        
        if start < self.position {
            Some(&self.input[start..self.position])
        } else {
            None
        }
    }
    
    fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
}

fn advanced_examples() {
    println!("\n=== 高级生命周期示例 ===");
    
    // 缓存示例
    let key1 = "first";
    let key2 = "second";
    
    let mut cache = Cache::new();
    cache.insert(key1, 100);
    cache.insert(key2, 200);
    
    if let Some(value) = cache.get("first") {
        println!("缓存中的值: {}", value);
    }
    
    // 解析器示例
    let input = "123abc456def";
    let mut parser = Parser::new(input);
    
    while parser.position < input.len() {
        if let Some(number) = parser.parse_number() {
            println!("解析到数字: {}", number);
        } else {
            parser.advance();
        }
    }
}

// 生命周期省略规则示例
fn lifetime_elision_examples() {
    println!("\n=== 生命周期省略规则示例 ===");
    
    // 规则1：每个引用参数都有自己的生命周期
    // fn first_word(s: &str) -> &str  等价于
    // fn first_word<'a>(s: &'a str) -> &'a str
    
    let text = "hello world";
    let word = first_word(text);
    println!("第一个单词: {}", word);
    
    // 规则2：如果只有一个输入生命周期参数，
    // 那么该生命周期被赋予所有输出生命周期参数
    let s = String::from("test");
    let whole = get_whole_string(&s);
    println!("整个字符串: {}", whole);
    
    // 规则3：如果方法有多个输入生命周期参数，
    // 但其中一个是&self或&mut self，
    // 那么self的生命周期被赋予所有输出生命周期参数
    let excerpt = ImportantExcerpt { part: "test" };
    let part = excerpt.announce_and_return_part("测试");
    println!("摘录部分: {}", part);
}

// 协变和逆变示例（高级概念）
fn variance_example() {
    println!("\n=== 协变示例 ===");
    
    // 生命周期是协变的：'long: 'short 意味着可以在需要 'short 的地方使用 'long
    let long_lived = String::from("I live a long time");
    
    {
        let short_lived = String::from("I live a short time");
        // longest函数可以接受不同生命周期的参数
        // 但返回值的生命周期是两者中较短的那个
        let result = longest(&long_lived, &short_lived);
        println!("在短生命周期作用域内: {}", result);
    }
    
    // short_lived在这里已经被销毁
    println!("长生命周期字符串仍然存在: {}", long_lived);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let str1 = "hello";
        let str2 = "world!";
        assert_eq!(longest(str1, str2), "world!");
        
        let str3 = "rust";
        let str4 = "go";
        assert_eq!(longest(str3, str4), "rust");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_important_excerpt() {
        let text = "This is a test";
        let excerpt = ImportantExcerpt { part: text };
        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.announce_and_return_part("test"), text);
    }

    #[test]
    fn test_book() {
        let book = Book::new("Title", "Author", "ISBN");
        assert_eq!(book.title(), "Title");
        assert!(book.get_info().contains("Title"));
    }

    #[test]
    fn test_extract_field() {
        let data = "name:Alice,age:30,city:Beijing";
        assert_eq!(extract_field(data, "name"), Some("Alice"));
        assert_eq!(extract_field(data, "age"), Some("30"));
        assert_eq!(extract_field(data, "unknown"), None);
    }

    #[test]
    fn test_parser() {
        let mut parser = Parser::new("123abc");
        assert_eq!(parser.parse_number(), Some("123"));
        assert_eq!(parser.remaining(), "abc");
    }

    #[test]
    fn test_examples() {
        basic_lifetimes();
        function_lifetimes();
        struct_lifetimes();
        static_lifetimes();
        advanced_examples();
        lifetime_elision_examples();
        variance_example();
    }
}

// 生命周期要点总结：
// 1. 生命周期确保引用的有效性
// 2. 生命周期注解描述引用之间的关系，不改变生命周期
// 3. 生命周期省略规则减少了注解的需要
// 4. 结构体中的引用字段需要生命周期注解
// 5. 'static生命周期表示整个程序运行期间
// 6. 生命周期是协变的
// 7. 生命周期参数是泛型的一种形式
// 8. 编译器通过借用检查器确保内存安全