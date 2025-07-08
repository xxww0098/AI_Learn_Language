# Regex 1.11.1 - Rust 正则表达式完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [基本用法](#基本用法)
- [模式匹配](#模式匹配)
- [捕获组](#捕获组)
- [替换操作](#替换操作)
- [分割操作](#分割操作)
- [高级特性](#高级特性)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Regex 是 Rust 中最重要的正则表达式库，使用有限自动机实现，保证在所有输入上都有线性时间复杂度，避免了灾难性回溯。

### 核心特性
- **线性时间复杂度**: 使用有限自动机，避免指数级回溯
- **Unicode 支持**: 完整的 Unicode 字符类和属性支持
- **高性能**: 经过优化的匹配算法和编译策略
- **安全性**: 防止正则表达式拒绝服务攻击（ReDoS）
- **功能丰富**: 支持前瞻、后顾、命名捕获组等高级特性

### 版本信息
- **当前版本**: 1.11.1
- **发布时间**: 2024-10-24
- **下载次数**: 475,045,739+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
regex = "1.11.1"
```

### 基本示例

```rust
use regex::Regex;

fn main() {
    // 编译正则表达式
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    
    // 测试匹配
    let text = "今天是2024-03-15，明天是2024-03-16";
    
    // 检查是否匹配
    if re.is_match(text) {
        println!("找到日期格式!");
    }
    
    // 查找第一个匹配
    if let Some(mat) = re.find(text) {
        println!("第一个日期: {}", mat.as_str());
    }
    
    // 查找所有匹配
    for mat in re.find_iter(text) {
        println!("找到日期: {}", mat.as_str());
    }
    
    // 替换匹配项
    let replaced = re.replace_all(text, "XXXX-XX-XX");
    println!("替换后: {}", replaced);
}
```

## 基本用法

### 编译和验证

```rust
use regex::Regex;

fn compilation_and_validation() {
    // 成功编译
    match Regex::new(r"\d+") {
        Ok(re) => println!("正则表达式编译成功"),
        Err(e) => println!("编译失败: {}", e),
    }
    
    // 编译失败的示例
    match Regex::new(r"[") {
        Ok(re) => println!("编译成功"),
        Err(e) => println!("编译失败: {}", e),
    }
    
    // 预编译静态正则表达式
    use regex::Regex;
    use std::sync::OnceLock;
    
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    
    fn get_email_regex() -> &'static Regex {
        EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        })
    }
    
    let email = "user@example.com";
    if get_email_regex().is_match(email) {
        println!("有效的邮箱地址");
    }
}
```

### 基本匹配方法

```rust
use regex::Regex;

fn basic_matching() {
    let re = Regex::new(r"\b\w+\b").unwrap();
    let text = "Hello world! 你好世界!";
    
    // is_match - 检查是否包含匹配
    println!("包含单词: {}", re.is_match(text));
    
    // find - 查找第一个匹配
    if let Some(mat) = re.find(text) {
        println!("第一个匹配: '{}' at {}-{}", 
                 mat.as_str(), mat.start(), mat.end());
    }
    
    // find_iter - 迭代所有匹配
    println!("所有匹配:");
    for (i, mat) in re.find_iter(text).enumerate() {
        println!("  {}: '{}' at {}-{}", 
                 i, mat.as_str(), mat.start(), mat.end());
    }
    
    // find_at - 从指定位置开始查找
    if let Some(mat) = re.find_at(text, 6) {
        println!("从位置6开始的第一个匹配: '{}'", mat.as_str());
    }
}
```

### 字符类和量词

```rust
use regex::Regex;

fn character_classes_and_quantifiers() {
    let examples = vec![
        // 字符类
        (r"\d+", "数字123abc", "匹配数字"),
        (r"\w+", "hello_world", "匹配单词字符"),
        (r"\s+", "hello world", "匹配空白字符"),
        (r"[a-z]+", "Hello World", "匹配小写字母"),
        (r"[A-Z]+", "Hello World", "匹配大写字母"),
        (r"[0-9]+", "abc123def", "匹配数字范围"),
        
        // 量词
        (r"a?", "bac", "匹配0或1个a"),
        (r"a+", "aaabbb", "匹配1个或多个a"),
        (r"a*", "bbbccc", "匹配0个或多个a"),
        (r"a{2,4}", "aaaa", "匹配2-4个a"),
        (r"a{3}", "aaaa", "匹配恰好3个a"),
        
        // 贪婪和非贪婪
        (r"<.*>", "<tag>content</tag>", "贪婪匹配"),
        (r"<.*?>", "<tag>content</tag>", "非贪婪匹配"),
    ];
    
    for (pattern, text, description) in examples {
        let re = Regex::new(pattern).unwrap();
        println!("{}: '{}'", description, pattern);
        if let Some(mat) = re.find(text) {
            println!("  在 '{}' 中找到: '{}'", text, mat.as_str());
        } else {
            println!("  在 '{}' 中未找到匹配", text);
        }
        println!();
    }
}
```

## 模式匹配

### 锚点和边界

```rust
use regex::Regex;

fn anchors_and_boundaries() {
    let test_cases = vec![
        // 行首和行尾
        (r"^Hello", "Hello world\nHello again", "行首匹配"),
        (r"world$", "Hello world\nHello again", "行尾匹配"),
        
        // 单词边界
        (r"\bcat\b", "cat catch", "单词边界"),
        (r"\Bcat\B", "concatenate", "非单词边界"),
        
        // 字符串开始和结束
        (r"\AHello", "Hello world", "字符串开始"),
        (r"world\z", "Hello world", "字符串结束"),
        
        // 多行模式
        (r"(?m)^Hello", "World\nHello\nThere", "多行模式行首"),
        (r"(?m)Hello$", "Say Hello\nWorld", "多行模式行尾"),
    ];
    
    for (pattern, text, description) in test_cases {
        let re = Regex::new(pattern).unwrap();
        println!("{}: '{}'", description, pattern);
        println!("  文本: '{}'", text);
        
        let matches: Vec<_> = re.find_iter(text).collect();
        if matches.is_empty() {
            println!("  无匹配");
        } else {
            for mat in matches {
                println!("  匹配: '{}'", mat.as_str());
            }
        }
        println!();
    }
}
```

### 字符组和范围

```rust
use regex::Regex;

fn character_groups_and_ranges() {
    let examples = vec![
        // 基本字符组
        (r"[abc]", "abcdef", "匹配a、b或c"),
        (r"[^abc]", "abcdef", "匹配除a、b、c外的字符"),
        (r"[a-z]", "Hello123", "匹配小写字母"),
        (r"[A-Z]", "Hello123", "匹配大写字母"),
        (r"[0-9]", "Hello123", "匹配数字"),
        
        // 组合字符组
        (r"[a-zA-Z]", "Hello123", "匹配字母"),
        (r"[a-zA-Z0-9]", "Hello123!", "匹配字母数字"),
        (r"[a-zA-Z0-9_]", "var_name123", "匹配标识符字符"),
        
        // 特殊字符组
        (r"[\w]", "hello_world", "单词字符"),
        (r"[\W]", "hello world!", "非单词字符"),
        (r"[\d]", "abc123", "数字字符"),
        (r"[\D]", "abc123", "非数字字符"),
        (r"[\s]", "hello world", "空白字符"),
        (r"[\S]", "hello world", "非空白字符"),
        
        // 转义字符
        (r"[\[\]]", "array[0]", "匹配方括号"),
        (r"[.*+?^${}()|\\]", "a+b*c", "匹配特殊字符"),
    ];
    
    for (pattern, text, description) in examples {
        let re = Regex::new(pattern).unwrap();
        println!("{}: '{}'", description, pattern);
        
        let matches: Vec<_> = re.find_iter(text).collect();
        if matches.is_empty() {
            println!("  在 '{}' 中无匹配", text);
        } else {
            print!("  在 '{}' 中匹配: ", text);
            for mat in matches {
                print!("'{}' ", mat.as_str());
            }
            println!();
        }
        println!();
    }
}
```

### 选择和分组

```rust
use regex::Regex;

fn alternation_and_grouping() {
    let examples = vec![
        // 选择
        (r"cat|dog", "I have a cat and a dog", "选择匹配"),
        (r"gr(a|e)y", "gray grey", "组内选择"),
        (r"colou?r", "color colour", "可选字符"),
        
        // 分组
        (r"(ab)+", "abab", "重复分组"),
        (r"(ha){2,3}", "hahaha", "分组量词"),
        (r"(\d{1,3}\.){3}\d{1,3}", "192.168.1.1", "IP地址"),
        
        // 非捕获组
        (r"(?:ab)+", "abab", "非捕获组"),
        (r"(?:https?://)?www\.", "https://www.example.com", "可选协议"),
        
        // 原子组
        (r"(?>ab|a)b", "ab", "原子组"),
    ];
    
    for (pattern, text, description) in examples {
        let re = Regex::new(pattern).unwrap();
        println!("{}: '{}'", description, pattern);
        
        for mat in re.find_iter(text) {
            println!("  在 '{}' 中匹配: '{}'", text, mat.as_str());
        }
        println!();
    }
}
```

## 捕获组

### 基本捕获

```rust
use regex::Regex;

fn basic_captures() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "今天是2024-03-15";
    
    if let Some(caps) = re.captures(text) {
        println!("完整匹配: {}", caps.get(0).unwrap().as_str());
        println!("年: {}", caps.get(1).unwrap().as_str());
        println!("月: {}", caps.get(2).unwrap().as_str());
        println!("日: {}", caps.get(3).unwrap().as_str());
    }
    
    // 处理多个匹配
    let text = "2024-03-15 和 2023-12-25 是两个日期";
    for caps in re.captures_iter(text) {
        println!("日期: {}-{}-{}", 
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(), 
                 caps.get(3).unwrap().as_str());
    }
}
```

### 命名捕获组

```rust
use regex::Regex;

fn named_captures() {
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let text = "生日是1990-05-15";
    
    if let Some(caps) = re.captures(text) {
        println!("完整匹配: {}", &caps[0]);
        println!("年: {}", &caps["year"]);
        println!("月: {}", &caps["month"]);
        println!("日: {}", &caps["day"]);
        
        // 使用get方法安全访问
        if let Some(year) = caps.name("year") {
            println!("年份位置: {}-{}", year.start(), year.end());
        }
    }
    
    // 解析URL示例
    let url_re = Regex::new(r"(?P<scheme>https?://)?(?P<domain>[^/]+)(?P<path>/.*)?").unwrap();
    let urls = vec![
        "https://www.example.com/path",
        "http://example.com",
        "www.example.com/path",
    ];
    
    for url in urls {
        if let Some(caps) = url_re.captures(url) {
            println!("URL: {}", url);
            println!("  协议: {}", caps.name("scheme").map(|m| m.as_str()).unwrap_or("无"));
            println!("  域名: {}", &caps["domain"]);
            println!("  路径: {}", caps.name("path").map(|m| m.as_str()).unwrap_or("无"));
        }
    }
}
```

### 嵌套捕获组

```rust
use regex::Regex;

fn nested_captures() {
    // 解析复杂的数据结构
    let re = Regex::new(r"((\w+)=(\d+)(?:,(\w+)=(\d+))?(?:,(\w+)=(\d+))?)").unwrap();
    let text = "name=John,age=30,score=95";
    
    if let Some(caps) = re.captures(text) {
        println!("完整匹配: {}", caps.get(0).unwrap().as_str());
        
        // 遍历所有捕获组
        for (i, cap) in caps.iter().enumerate() {
            if let Some(cap) = cap {
                println!("组 {}: '{}'", i, cap.as_str());
            }
        }
    }
    
    // 解析HTML标签
    let html_re = Regex::new(r"<((\w+)(\s+[^>]*)?)>([^<]*)</\2>").unwrap();
    let html = "<div class='content'>Hello World</div>";
    
    if let Some(caps) = html_re.captures(html) {
        println!("HTML解析:");
        println!("  标签: {}", caps.get(2).unwrap().as_str());
        println!("  属性: {}", caps.get(3).map(|m| m.as_str()).unwrap_or("无"));
        println!("  内容: {}", caps.get(4).unwrap().as_str());
    }
}
```

## 替换操作

### 基本替换

```rust
use regex::Regex;

fn basic_replacement() {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let text = "我有3个苹果和5个橙子";
    
    // 替换第一个匹配
    let result = re.replace(text, "X");
    println!("替换第一个: {}", result);
    
    // 替换所有匹配
    let result = re.replace_all(text, "X");
    println!("替换所有: {}", result);
    
    // 使用函数进行替换
    let result = re.replace_all(text, |caps: &regex::Captures| {
        let num: i32 = caps[0].parse().unwrap();
        format!("[{}]", num * 2)
    });
    println!("函数替换: {}", result);
}
```

### 使用捕获组替换

```rust
use regex::Regex;

fn capture_group_replacement() {
    // 交换姓名顺序
    let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
    let names = "John Smith";
    let result = re.replace(names, "$2, $1");
    println!("姓名交换: {}", result);
    
    // 格式化日期
    let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let dates = "今天是2024-03-15，明天是2024-03-16";
    let result = date_re.replace_all(dates, "$3/$2/$1");
    println!("日期格式化: {}", result);
    
    // 使用命名组替换
    let email_re = Regex::new(r"(?P<user>\w+)@(?P<domain>\w+\.\w+)").unwrap();
    let email = "用户user@example.com发送了邮件";
    let result = email_re.replace(email, "${user} at ${domain}");
    println!("邮箱格式化: {}", result);
}
```

### 条件替换

```rust
use regex::Regex;

fn conditional_replacement() {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let text = "价格是100元，折扣是20%";
    
    // 条件替换：只替换大于50的数字
    let result = re.replace_all(text, |caps: &regex::Captures| {
        let num: i32 = caps[0].parse().unwrap();
        if num > 50 {
            format!("**{}**", num)
        } else {
            caps[0].to_string()
        }
    });
    println!("条件替换: {}", result);
    
    // 复杂的条件替换
    let word_re = Regex::new(r"\b\w+\b").unwrap();
    let text = "这是一个测试句子 This is a test sentence";
    
    let result = word_re.replace_all(text, |caps: &regex::Captures| {
        let word = caps[0].to_string();
        // 检查是否包含中文字符
        if word.chars().any(|c| c as u32 > 127) {
            format!("[中文:{}]", word)
        } else if word.len() > 4 {
            format!("[长词:{}]", word)
        } else {
            word
        }
    });
    println!("复杂替换: {}", result);
}
```

## 分割操作

### 基本分割

```rust
use regex::Regex;

fn basic_splitting() {
    // 按空白字符分割
    let re = Regex::new(r"\s+").unwrap();
    let text = "hello   world\t\tRust\nregex";
    let parts: Vec<&str> = re.split(text).collect();
    println!("空白分割: {:?}", parts);
    
    // 按标点符号分割
    let punct_re = Regex::new(r"[,.;!?]+").unwrap();
    let sentence = "Hello, world! How are you? I'm fine.";
    let parts: Vec<&str> = punct_re.split(sentence).collect();
    println!("标点分割: {:?}", parts);
    
    // 限制分割数量
    let comma_re = Regex::new(r",").unwrap();
    let csv = "apple,banana,cherry,date,elderberry";
    let parts: Vec<&str> = comma_re.splitn(csv, 3).collect();
    println!("限制分割: {:?}", parts);
}
```

### 高级分割

```rust
use regex::Regex;

fn advanced_splitting() {
    // 保留分隔符
    let re = Regex::new(r"(\s+)").unwrap();
    let text = "hello world rust";
    let parts: Vec<&str> = re.split(text).collect();
    println!("普通分割: {:?}", parts);
    
    // 手动实现保留分隔符的分割
    let mut last_end = 0;
    let mut result = Vec::new();
    
    for mat in re.find_iter(text) {
        if last_end < mat.start() {
            result.push(&text[last_end..mat.start()]);
        }
        result.push(mat.as_str());
        last_end = mat.end();
    }
    if last_end < text.len() {
        result.push(&text[last_end..]);
    }
    println!("保留分隔符: {:?}", result);
    
    // 复杂的分割规则
    let complex_re = Regex::new(r"(?:and|or|but)\s+").unwrap();
    let text = "I like apples and oranges but not bananas or grapes";
    let parts: Vec<&str> = complex_re.split(text).collect();
    println!("复杂分割: {:?}", parts);
}
```

## 高级特性

### 前瞻和后顾

```rust
use regex::Regex;

fn lookahead_and_lookbehind() {
    // 正向前瞻
    let re = Regex::new(r"\d+(?=\s*元)").unwrap();
    let text = "价格是100元，数量是50个";
    for mat in re.find_iter(text) {
        println!("价格数字: {}", mat.as_str());
    }
    
    // 负向前瞻
    let re = Regex::new(r"\d+(?!\s*元)").unwrap();
    let text = "价格是100元，数量是50个";
    for mat in re.find_iter(text) {
        println!("非价格数字: {}", mat.as_str());
    }
    
    // 正向后顾
    let re = Regex::new(r"(?<=\$)\d+").unwrap();
    let text = "价格$100，费用￥200";
    for mat in re.find_iter(text) {
        println!("美元金额: {}", mat.as_str());
    }
    
    // 负向后顾
    let re = Regex::new(r"(?<!\$)\d+").unwrap();
    let text = "价格$100，数量50个";
    for mat in re.find_iter(text) {
        println!("非美元数字: {}", mat.as_str());
    }
}
```

### 修饰符和标志

```rust
use regex::Regex;

fn flags_and_modifiers() {
    // 不区分大小写
    let re = Regex::new(r"(?i)hello").unwrap();
    let text = "Hello HELLO hello HeLLo";
    let matches: Vec<_> = re.find_iter(text).collect();
    println!("不区分大小写匹配: {:?}", matches.len());
    
    // 多行模式
    let re = Regex::new(r"(?m)^hello").unwrap();
    let text = "world\nhello\nthere";
    for mat in re.find_iter(text) {
        println!("多行模式匹配: {}", mat.as_str());
    }
    
    // 单行模式（. 匹配换行符）
    let re = Regex::new(r"(?s)start.*end").unwrap();
    let text = "start\nmiddle\nend";
    if let Some(mat) = re.find(text) {
        println!("单行模式匹配: {:?}", mat.as_str());
    }
    
    // 忽略模式中的空白
    let re = Regex::new(r"(?x)
        \d{4}  # 年
        -      # 分隔符
        \d{2}  # 月
        -      # 分隔符
        \d{2}  # 日
    ").unwrap();
    let text = "2024-03-15";
    if re.is_match(text) {
        println!("详细模式匹配成功");
    }
}
```

### RegexSet 多模式匹配

```rust
use regex::RegexSet;

fn regex_set_matching() {
    let set = RegexSet::new(&[
        r"\d+",           // 数字
        r"[a-zA-Z]+",     // 字母
        r"[!@#$%^&*()]+", // 特殊字符
    ]).unwrap();
    
    let texts = vec![
        "hello123!",
        "world",
        "12345",
        "!@#$%",
        "mixed123text!",
    ];
    
    for text in texts {
        let matches: Vec<_> = set.matches(text).into_iter().collect();
        println!("文本 '{}' 匹配模式: {:?}", text, matches);
        
        // 检查特定模式
        if set.is_match(text) {
            println!("  包含至少一个模式");
        }
    }
    
    // 检查所有模式
    let test_text = "Hello123World!";
    let matches = set.matches(test_text);
    println!("文本 '{}' 的匹配结果:", test_text);
    for i in matches {
        match i {
            0 => println!("  包含数字"),
            1 => println!("  包含字母"),
            2 => println!("  包含特殊字符"),
            _ => {}
        }
    }
}
```

## 性能优化

### 编译时优化

```rust
use regex::Regex;
use std::sync::OnceLock;

// 使用静态编译避免重复编译
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

fn get_phone_regex() -> &'static Regex {
    PHONE_REGEX.get_or_init(|| {
        Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap()
    })
}

fn compile_time_optimization() {
    let emails = vec![
        "user@example.com",
        "invalid-email",
        "another@test.org",
    ];
    
    for email in emails {
        if get_email_regex().is_match(email) {
            println!("有效邮箱: {}", email);
        }
    }
    
    let phones = vec![
        "123-456-7890",
        "invalid-phone",
        "987-654-3210",
    ];
    
    for phone in phones {
        if get_phone_regex().is_match(phone) {
            println!("有效电话: {}", phone);
        }
    }
}
```

### 运行时优化

```rust
use regex::Regex;
use std::time::Instant;

fn runtime_optimization() {
    let text = "This is a test string with numbers 123 and 456 and words hello world";
    
    // 避免不必要的分配
    let re = Regex::new(r"\d+").unwrap();
    
    // 使用 find_iter 而不是 find_all
    let start = Instant::now();
    let count = re.find_iter(text).count();
    println!("find_iter 耗时: {:?}, 找到 {} 个匹配", start.elapsed(), count);
    
    // 使用 is_match 进行快速检查
    let start = Instant::now();
    let has_numbers = re.is_match(text);
    println!("is_match 耗时: {:?}, 结果: {}", start.elapsed(), has_numbers);
    
    // 预编译复杂正则表达式
    let complex_re = Regex::new(r"(?i)(?:the|a|an)\s+\w+").unwrap();
    let start = Instant::now();
    let matches: Vec<_> = complex_re.find_iter(text).collect();
    println!("复杂匹配耗时: {:?}, 找到 {} 个匹配", start.elapsed(), matches.len());
    
    // 使用字符串方法替代简单模式
    let start = Instant::now();
    let simple_count = text.matches("test").count();
    println!("字符串匹配耗时: {:?}, 找到 {} 个匹配", start.elapsed(), simple_count);
}
```

### 内存使用优化

```rust
use regex::Regex;

fn memory_optimization() {
    let re = Regex::new(r"\w+").unwrap();
    let text = "这是一个很长的文本，包含很多单词需要匹配和处理";
    
    // 避免收集所有匹配到向量中
    let mut word_count = 0;
    let mut total_length = 0;
    
    for mat in re.find_iter(text) {
        word_count += 1;
        total_length += mat.len();
        // 直接处理匹配，不存储
    }
    
    println!("单词数: {}, 总长度: {}", word_count, total_length);
    
    // 使用 captures_iter 时避免克隆
    let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let dates_text = "日期包括2024-03-15和2023-12-25";
    
    for caps in date_re.captures_iter(dates_text) {
        // 直接使用引用，避免克隆
        let year = caps.get(1).unwrap().as_str();
        let month = caps.get(2).unwrap().as_str();
        let day = caps.get(3).unwrap().as_str();
        println!("处理日期: {}-{}-{}", year, month, day);
    }
}
```

## 实战案例

### 数据验证

```rust
use regex::Regex;

struct DataValidator {
    email_re: Regex,
    phone_re: Regex,
    url_re: Regex,
    ip_re: Regex,
    password_re: Regex,
}

impl DataValidator {
    fn new() -> Self {
        DataValidator {
            email_re: Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap(),
            phone_re: Regex::new(r"^(\+86\s?)?1[3-9]\d{9}$").unwrap(),
            url_re: Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap(),
            ip_re: Regex::new(r"^((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)$").unwrap(),
            password_re: Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap(),
        }
    }
    
    fn validate_email(&self, email: &str) -> bool {
        self.email_re.is_match(email)
    }
    
    fn validate_phone(&self, phone: &str) -> bool {
        self.phone_re.is_match(phone)
    }
    
    fn validate_url(&self, url: &str) -> bool {
        self.url_re.is_match(url)
    }
    
    fn validate_ip(&self, ip: &str) -> bool {
        self.ip_re.is_match(ip)
    }
    
    fn validate_password(&self, password: &str) -> bool {
        self.password_re.is_match(password)
    }
    
    fn validate_all(&self, data: &UserData) -> ValidationResult {
        ValidationResult {
            email_valid: self.validate_email(&data.email),
            phone_valid: self.validate_phone(&data.phone),
            url_valid: data.website.as_ref().map_or(true, |url| self.validate_url(url)),
            password_valid: self.validate_password(&data.password),
        }
    }
}

#[derive(Debug)]
struct UserData {
    email: String,
    phone: String,
    website: Option<String>,
    password: String,
}

#[derive(Debug)]
struct ValidationResult {
    email_valid: bool,
    phone_valid: bool,
    url_valid: bool,
    password_valid: bool,
}

impl ValidationResult {
    fn is_valid(&self) -> bool {
        self.email_valid && self.phone_valid && self.url_valid && self.password_valid
    }
}

fn data_validation_demo() {
    let validator = DataValidator::new();
    
    let test_cases = vec![
        UserData {
            email: "user@example.com".to_string(),
            phone: "13812345678".to_string(),
            website: Some("https://example.com".to_string()),
            password: "Password123!".to_string(),
        },
        UserData {
            email: "invalid-email".to_string(),
            phone: "123456".to_string(),
            website: Some("not-a-url".to_string()),
            password: "weak".to_string(),
        },
    ];
    
    for (i, data) in test_cases.iter().enumerate() {
        let result = validator.validate_all(data);
        println!("用户数据 {}: {:?}", i + 1, data);
        println!("验证结果: {:?}", result);
        println!("总体有效: {}", result.is_valid());
        println!();
    }
}
```

### 文本解析

```rust
use regex::Regex;
use std::collections::HashMap;

struct LogParser {
    access_log_re: Regex,
    error_log_re: Regex,
    sql_log_re: Regex,
}

impl LogParser {
    fn new() -> Self {
        LogParser {
            access_log_re: Regex::new(
                r#"^(?P<ip>\d+\.\d+\.\d+\.\d+) - - \[(?P<datetime>[^\]]+)\] "(?P<method>\w+) (?P<path>[^\s]+) HTTP/[\d.]+" (?P<status>\d+) (?P<size>\d+)"#
            ).unwrap(),
            error_log_re: Regex::new(
                r#"^\[(?P<datetime>[^\]]+)\] \[(?P<level>\w+)\] (?P<message>.*)"#
            ).unwrap(),
            sql_log_re: Regex::new(
                r#"^\[(?P<datetime>[^\]]+)\] (?P<duration>[\d.]+)ms: (?P<query>.*)"#
            ).unwrap(),
        }
    }
    
    fn parse_access_log(&self, line: &str) -> Option<AccessLogEntry> {
        self.access_log_re.captures(line).map(|caps| {
            AccessLogEntry {
                ip: caps["ip"].to_string(),
                datetime: caps["datetime"].to_string(),
                method: caps["method"].to_string(),
                path: caps["path"].to_string(),
                status: caps["status"].parse().unwrap_or(0),
                size: caps["size"].parse().unwrap_or(0),
            }
        })
    }
    
    fn parse_error_log(&self, line: &str) -> Option<ErrorLogEntry> {
        self.error_log_re.captures(line).map(|caps| {
            ErrorLogEntry {
                datetime: caps["datetime"].to_string(),
                level: caps["level"].to_string(),
                message: caps["message"].to_string(),
            }
        })
    }
    
    fn parse_sql_log(&self, line: &str) -> Option<SqlLogEntry> {
        self.sql_log_re.captures(line).map(|caps| {
            SqlLogEntry {
                datetime: caps["datetime"].to_string(),
                duration: caps["duration"].parse().unwrap_or(0.0),
                query: caps["query"].to_string(),
            }
        })
    }
}

#[derive(Debug)]
struct AccessLogEntry {
    ip: String,
    datetime: String,
    method: String,
    path: String,
    status: u16,
    size: u64,
}

#[derive(Debug)]
struct ErrorLogEntry {
    datetime: String,
    level: String,
    message: String,
}

#[derive(Debug)]
struct SqlLogEntry {
    datetime: String,
    duration: f64,
    query: String,
}

fn log_parsing_demo() {
    let parser = LogParser::new();
    
    let access_logs = vec![
        r#"127.0.0.1 - - [25/Dec/2023:10:00:00 +0800] "GET /index.html HTTP/1.1" 200 1234"#,
        r#"192.168.1.1 - - [25/Dec/2023:10:00:01 +0800] "POST /api/users HTTP/1.1" 201 567"#,
    ];
    
    let error_logs = vec![
        r#"[25/Dec/2023:10:00:00 +0800] [ERROR] Database connection failed"#,
        r#"[25/Dec/2023:10:00:01 +0800] [WARNING] High memory usage detected"#,
    ];
    
    let sql_logs = vec![
        r#"[25/Dec/2023:10:00:00 +0800] 150.5ms: SELECT * FROM users WHERE id = 1"#,
        r#"[25/Dec/2023:10:00:01 +0800] 50.2ms: INSERT INTO logs (message) VALUES ('test')"#,
    ];
    
    println!("=== 访问日志解析 ===");
    for log in access_logs {
        if let Some(entry) = parser.parse_access_log(log) {
            println!("{:?}", entry);
        }
    }
    
    println!("\n=== 错误日志解析 ===");
    for log in error_logs {
        if let Some(entry) = parser.parse_error_log(log) {
            println!("{:?}", entry);
        }
    }
    
    println!("\n=== SQL日志解析 ===");
    for log in sql_logs {
        if let Some(entry) = parser.parse_sql_log(log) {
            println!("{:?}", entry);
        }
    }
}
```

### 模板引擎

```rust
use regex::Regex;
use std::collections::HashMap;

struct SimpleTemplateEngine {
    variable_re: Regex,
    loop_re: Regex,
    conditional_re: Regex,
}

impl SimpleTemplateEngine {
    fn new() -> Self {
        SimpleTemplateEngine {
            variable_re: Regex::new(r"\{\{(\w+)\}\}").unwrap(),
            loop_re: Regex::new(r"\{\{#each\s+(\w+)\}\}(.*?)\{\{/each\}\}").unwrap(),
            conditional_re: Regex::new(r"\{\{#if\s+(\w+)\}\}(.*?)\{\{/if\}\}").unwrap(),
        }
    }
    
    fn render(&self, template: &str, context: &HashMap<String, Value>) -> String {
        let mut result = template.to_string();
        
        // 处理条件语句
        result = self.conditional_re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let content = &caps[2];
            
            if let Some(value) = context.get(var_name) {
                if value.is_truthy() {
                    content.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }).to_string();
        
        // 处理循环
        result = self.loop_re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let template = &caps[2];
            
            if let Some(Value::Array(items)) = context.get(var_name) {
                items.iter()
                    .map(|item| {
                        if let Value::Object(obj) = item {
                            let mut item_result = template.to_string();
                            for (key, value) in obj {
                                item_result = item_result.replace(
                                    &format!("{{{{{}}}}}", key),
                                    &value.to_string()
                                );
                            }
                            item_result
                        } else {
                            template.replace("{{.}}", &item.to_string())
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            } else {
                String::new()
            }
        }).to_string();
        
        // 处理变量替换
        result = self.variable_re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            context.get(var_name)
                .map(|v| v.to_string())
                .unwrap_or_else(|| format!("{{{{{}}}}}", var_name))
        }).to_string();
        
        result
    }
}

#[derive(Clone, Debug)]
enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => *n != 0.0,
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(_) => write!(f, "[Array]"),
            Value::Object(_) => write!(f, "[Object]"),
        }
    }
}

fn template_engine_demo() {
    let engine = SimpleTemplateEngine::new();
    
    let template = r#"
<h1>{{title}}</h1>
{{#if show_content}}
<div class="content">
    <p>欢迎, {{username}}!</p>
    <ul>
    {{#each items}}
        <li>{{name}} - {{price}}</li>
    {{/each}}
    </ul>
</div>
{{/if}}
"#;
    
    let mut context = HashMap::new();
    context.insert("title".to_string(), Value::String("商品列表".to_string()));
    context.insert("username".to_string(), Value::String("张三".to_string()));
    context.insert("show_content".to_string(), Value::Bool(true));
    
    let mut item1 = HashMap::new();
    item1.insert("name".to_string(), Value::String("苹果".to_string()));
    item1.insert("price".to_string(), Value::String("5元".to_string()));
    
    let mut item2 = HashMap::new();
    item2.insert("name".to_string(), Value::String("香蕉".to_string()));
    item2.insert("price".to_string(), Value::String("3元".to_string()));
    
    context.insert("items".to_string(), Value::Array(vec![
        Value::Object(item1),
        Value::Object(item2),
    ]));
    
    let result = engine.render(template, &context);
    println!("渲染结果:\n{}", result);
}
```

## 最佳实践

### 1. 性能优化

```rust
use regex::Regex;
use std::sync::OnceLock;

// 使用静态变量缓存编译的正则表达式
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

fn performance_best_practices() {
    // 1. 预编译正则表达式
    let emails = vec!["user@example.com", "invalid", "test@test.org"];
    for email in emails {
        if get_email_regex().is_match(email) {
            println!("有效邮箱: {}", email);
        }
    }
    
    // 2. 使用 is_match 进行快速检查
    let text = "This contains numbers 123";
    let has_numbers = Regex::new(r"\d+").unwrap().is_match(text);
    if has_numbers {
        // 只有在确实包含数字时才进行更复杂的操作
        println!("包含数字");
    }
    
    // 3. 避免不必要的捕获组
    let re_with_capture = Regex::new(r"(\d+)").unwrap();    // 有捕获组
    let re_without_capture = Regex::new(r"\d+").unwrap();   // 无捕获组
    
    // 如果不需要捕获，使用无捕获组版本
    let count = re_without_capture.find_iter(text).count();
    println!("数字数量: {}", count);
}
```

### 2. 错误处理

```rust
use regex::Regex;

fn error_handling_best_practices() {
    // 1. 处理编译错误
    fn compile_regex(pattern: &str) -> Result<Regex, regex::Error> {
        Regex::new(pattern)
    }
    
    let patterns = vec![
        r"\d+",     // 有效模式
        r"[",       // 无效模式
        r"(?P<name>\w+)",  // 命名捕获组
    ];
    
    for pattern in patterns {
        match compile_regex(pattern) {
            Ok(re) => println!("模式 '{}' 编译成功", pattern),
            Err(e) => println!("模式 '{}' 编译失败: {}", pattern, e),
        }
    }
    
    // 2. 安全地访问捕获组
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2024-03-15";
    
    if let Some(caps) = re.captures(text) {
        // 使用 get() 方法安全访问
        if let Some(year) = caps.get(1) {
            println!("年: {}", year.as_str());
        }
        
        // 或者使用 get() 和 map()
        let month = caps.get(2).map(|m| m.as_str()).unwrap_or("未知");
        println!("月: {}", month);
    }
}
```

### 3. 可维护性

```rust
use regex::Regex;

fn maintainability_best_practices() {
    // 1. 使用命名捕获组提高可读性
    let date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    
    let text = "2024-03-15";
    if let Some(caps) = date_re.captures(text) {
        println!("年: {}", &caps["year"]);
        println!("月: {}", &caps["month"]);
        println!("日: {}", &caps["day"]);
    }
    
    // 2. 将复杂正则表达式分解为部分
    let year_pattern = r"(?P<year>\d{4})";
    let month_pattern = r"(?P<month>\d{2})";
    let day_pattern = r"(?P<day>\d{2})";
    let date_pattern = format!("{}-{}-{}", year_pattern, month_pattern, day_pattern);
    
    let re = Regex::new(&date_pattern).unwrap();
    println!("组合模式: {}", date_pattern);
    
    // 3. 使用常量定义常用模式
    const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    const PHONE_PATTERN: &str = r"^\d{3}-\d{3}-\d{4}$";
    const URL_PATTERN: &str = r"^https?://[^\s/$.?#].[^\s]*$";
    
    let email_re = Regex::new(EMAIL_PATTERN).unwrap();
    let phone_re = Regex::new(PHONE_PATTERN).unwrap();
    let url_re = Regex::new(URL_PATTERN).unwrap();
    
    println!("预定义模式创建成功");
}
```

### 4. 测试策略

```rust
use regex::Regex;

fn testing_best_practices() {
    // 1. 测试正面和负面案例
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
    let valid_emails = vec![
        "user@example.com",
        "test.email+tag@example.co.uk",
        "user123@test-domain.com",
    ];
    
    let invalid_emails = vec![
        "invalid-email",
        "@example.com",
        "user@",
        "user@.com",
    ];
    
    println!("测试有效邮箱:");
    for email in valid_emails {
        assert!(email_re.is_match(email), "应该匹配: {}", email);
        println!("  ✓ {}", email);
    }
    
    println!("测试无效邮箱:");
    for email in invalid_emails {
        assert!(!email_re.is_match(email), "不应该匹配: {}", email);
        println!("  ✗ {}", email);
    }
    
    // 2. 测试边界情况
    let number_re = Regex::new(r"\d+").unwrap();
    let edge_cases = vec![
        ("", false),
        ("123", true),
        ("abc", false),
        ("123abc", true),
        ("abc123", true),
    ];
    
    for (input, expected) in edge_cases {
        let result = number_re.is_match(input);
        assert_eq!(result, expected, "输入: '{}', 期望: {}, 实际: {}", input, expected, result);
    }
}
```

## 总结

Regex 是 Rust 中功能强大且高效的正则表达式库。通过本教程，您应该能够：

1. 理解正则表达式的基本概念和语法
2. 掌握 Rust regex 库的核心 API
3. 使用捕获组进行复杂的文本提取
4. 实现高效的文本替换和分割
5. 优化正则表达式的性能
6. 在实际项目中应用正则表达式

关键要点：
- 预编译正则表达式以提高性能
- 使用命名捕获组提高代码可读性
- 注意正则表达式的线性时间复杂度保证
- 合理使用 is_match、find 和 captures 方法
- 在复杂场景中组合使用多个正则表达式

Regex 库的设计既保证了性能又提供了丰富的功能，是处理文本数据的重要工具。掌握它将大大提升您的 Rust 文本处理能力。