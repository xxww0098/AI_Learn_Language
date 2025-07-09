# regex-syntax 0.8.5 详细中文使用教程

## 简介

`regex-syntax` 是一个正则表达式解析器库，用于解析和分析正则表达式的语法结构。它是 `regex` 库的核心组件，提供了对正则表达式的语法分析、抽象语法树 (AST) 生成和高级接口层 (HIR) 功能。

## 基本信息

- **版本**: 0.8.5
- **许可证**: MIT OR Apache-2.0
- **文档**: https://docs.rs/regex-syntax
- **仓库**: https://github.com/rust-lang/regex/tree/master/regex-syntax
- **下载量**: 610,430,870 (极高人气)

## 快速开始

### 1. 添加依赖

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
regex-syntax = "0.8.5"
```

### 2. 基本使用

```rust
use regex_syntax::{Parser, ast, hir};

fn main() {
    // 解析简单的正则表达式
    let pattern = r"hello\d+";
    
    // 创建解析器
    let mut parser = Parser::new();
    
    // 解析为 AST
    match parser.parse(pattern) {
        Ok(ast) => {
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("解析错误: {}", e);
        }
    }
    
    // 解析为 HIR
    let translator = hir::translate::Translator::new();
    match parser.parse(pattern) {
        Ok(ast) => {
            match translator.translate(pattern, &ast) {
                Ok(hir) => {
                    println!("HIR: {:?}", hir);
                }
                Err(e) => {
                    println!("HIR 转换错误: {}", e);
                }
            }
        }
        Err(e) => {
            println!("解析错误: {}", e);
        }
    }
}
```

## 核心概念

### 1. 抽象语法树 (AST)

AST 是正则表达式的直接语法表示：

```rust
use regex_syntax::{Parser, ast::Ast};

fn ast_examples() {
    let mut parser = Parser::new();
    
    let patterns = vec![
        r"hello",
        r"\d+",
        r"[a-z]+",
        r"(hello|world)",
        r"hello{2,4}",
        r"hello*",
        r"hello+",
        r"hello?",
    ];
    
    for pattern in patterns {
        match parser.parse(pattern) {
            Ok(ast) => {
                println!("模式: {} -> AST: {:?}", pattern, ast);
                analyze_ast(&ast);
            }
            Err(e) => {
                println!("模式: {} -> 错误: {}", pattern, e);
            }
        }
    }
}

fn analyze_ast(ast: &Ast) {
    match ast {
        Ast::Literal(literal) => {
            println!("  字面量: {:?}", literal.c);
        }
        Ast::Dot(_) => {
            println!("  点号 (任意字符)");
        }
        Ast::Assertion(assertion) => {
            println!("  断言: {:?}", assertion.kind);
        }
        Ast::ClassUnicode(class) => {
            println!("  Unicode 字符类");
        }
        Ast::ClassPerl(class) => {
            println!("  Perl 字符类: {:?}", class.kind);
        }
        Ast::ClassBracketed(class) => {
            println!("  括号字符类，否定: {}", class.negated);
        }
        Ast::Repetition(rep) => {
            println!("  重复: {:?}", rep.op);
        }
        Ast::Group(group) => {
            println!("  分组: {:?}", group.kind);
        }
        Ast::Alternation(alt) => {
            println!("  选择 (有 {} 个选项)", alt.asts.len());
        }
        Ast::Concat(concat) => {
            println!("  连接 (有 {} 个部分)", concat.asts.len());
        }
        _ => {
            println!("  其他类型");
        }
    }
}
```

### 2. 高级接口层 (HIR)

HIR 是 AST 的高级表示，经过了优化和语义分析：

```rust
use regex_syntax::{Parser, hir::{self, Hir}};

fn hir_examples() {
    let mut parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let patterns = vec![
        r"hello",
        r"\d+",
        r"[a-zA-Z0-9]+",
        r"(hello|world)*",
        r"^hello$",
        r"\bhello\b",
    ];
    
    for pattern in patterns {
        match parser.parse(pattern) {
            Ok(ast) => {
                match translator.translate(pattern, &ast) {
                    Ok(hir) => {
                        println!("模式: {} -> HIR: {:?}", pattern, hir);
                        analyze_hir(&hir);
                    }
                    Err(e) => {
                        println!("模式: {} -> HIR 错误: {}", pattern, e);
                    }
                }
            }
            Err(e) => {
                println!("模式: {} -> 解析错误: {}", pattern, e);
            }
        }
    }
}

fn analyze_hir(hir: &Hir) {
    match hir.kind() {
        hir::HirKind::Empty => {
            println!("  空模式");
        }
        hir::HirKind::Literal(literal) => {
            println!("  字面量: {:?}", literal);
        }
        hir::HirKind::Class(class) => {
            println!("  字符类，范围数: {}", class.ranges().len());
        }
        hir::HirKind::Anchor(anchor) => {
            println!("  锚点: {:?}", anchor);
        }
        hir::HirKind::WordBoundary(boundary) => {
            println!("  词边界: {:?}", boundary);
        }
        hir::HirKind::Repetition(rep) => {
            println!("  重复: {:?}", rep.kind);
        }
        hir::HirKind::Group(group) => {
            println!("  分组: {:?}", group.kind);
        }
        hir::HirKind::Concat(concat) => {
            println!("  连接: {} 个元素", concat.len());
        }
        hir::HirKind::Alternation(alt) => {
            println!("  选择: {} 个选项", alt.len());
        }
        _ => {
            println!("  其他类型");
        }
    }
}
```

## 解析器配置

### 1. 解析器选项

```rust
use regex_syntax::{Parser, ParserBuilder};

fn parser_configuration() {
    // 默认解析器
    let default_parser = Parser::new();
    
    // 自定义解析器
    let custom_parser = ParserBuilder::new()
        .case_insensitive(true)         // 大小写不敏感
        .multi_line(true)               // 多行模式
        .dot_matches_new_line(true)     // 点号匹配换行符
        .swap_greed(true)               // 交换贪婪性
        .ignore_whitespace(true)        // 忽略空白字符
        .unicode(true)                  // 启用 Unicode 支持
        .utf8(true)                     // UTF-8 模式
        .nest_limit(100)                // 嵌套限制
        .octal(true)                    // 八进制转义
        .build();
    
    let pattern = r"Hello\s+World";
    
    // 使用默认解析器
    match default_parser.parse(pattern) {
        Ok(ast) => println!("默认解析成功: {:?}", ast),
        Err(e) => println!("默认解析错误: {}", e),
    }
    
    // 使用自定义解析器
    match custom_parser.parse(pattern) {
        Ok(ast) => println!("自定义解析成功: {:?}", ast),
        Err(e) => println!("自定义解析错误: {}", e),
    }
}
```

### 2. 语法特性控制

```rust
use regex_syntax::ParserBuilder;

fn syntax_features() {
    // 启用所有特性
    let full_parser = ParserBuilder::new()
        .allow_invalid_utf8(true)
        .case_insensitive(true)
        .multi_line(true)
        .dot_matches_new_line(true)
        .swap_greed(true)
        .ignore_whitespace(true)
        .unicode(true)
        .utf8(true)
        .octal(true)
        .build();
    
    // 限制特性
    let limited_parser = ParserBuilder::new()
        .unicode(false)                  // 禁用 Unicode
        .utf8(false)                     // 禁用 UTF-8
        .case_insensitive(false)         // 禁用大小写不敏感
        .multi_line(false)               // 禁用多行模式
        .build();
    
    let unicode_pattern = r"\p{L}+";
    
    // 完整解析器支持 Unicode
    match full_parser.parse(unicode_pattern) {
        Ok(_) => println!("完整解析器支持 Unicode"),
        Err(e) => println!("完整解析器错误: {}", e),
    }
    
    // 限制解析器不支持 Unicode
    match limited_parser.parse(unicode_pattern) {
        Ok(_) => println!("限制解析器意外支持 Unicode"),
        Err(e) => println!("限制解析器预期错误: {}", e),
    }
}
```

## 错误处理

### 1. 解析错误

```rust
use regex_syntax::{Parser, Error};

fn error_handling() {
    let parser = Parser::new();
    
    let invalid_patterns = vec![
        r"[",           // 未闭合的字符类
        r"(?P<>)",      // 空的命名组
        r"(?P<name>",   // 未闭合的命名组
        r"*",           // 开头的重复符
        r"(?",          // 未完成的组
        r"\p{InvalidCategory}", // 无效的 Unicode 类别
        r"(?P<123>hello)",      // 无效的组名
    ];
    
    for pattern in invalid_patterns {
        match parser.parse(pattern) {
            Ok(_) => {
                println!("模式 '{}' 意外成功", pattern);
            }
            Err(e) => {
                println!("模式 '{}' 错误: {}", pattern, e);
                analyze_error(&e);
            }
        }
    }
}

fn analyze_error(error: &Error) {
    match error.kind() {
        regex_syntax::ErrorKind::ClassUnclosed => {
            println!("  错误类型: 字符类未闭合");
        }
        regex_syntax::ErrorKind::GroupUnclosed => {
            println!("  错误类型: 分组未闭合");
        }
        regex_syntax::ErrorKind::GroupNameInvalid => {
            println!("  错误类型: 分组名称无效");
        }
        regex_syntax::ErrorKind::RepetitionMissing => {
            println!("  错误类型: 重复操作符缺少目标");
        }
        regex_syntax::ErrorKind::UnicodeClassInvalid => {
            println!("  错误类型: Unicode 类别无效");
        }
        _ => {
            println!("  错误类型: 其他");
        }
    }
    
    println!("  错误位置: {}", error.span());
    println!("  原始模式: {}", error.pattern());
}
```

### 2. 错误恢复

```rust
use regex_syntax::{Parser, Error};

fn error_recovery() {
    let parser = Parser::new();
    
    let patterns_with_fixes = vec![
        (r"[a-z", r"[a-z]"),           // 添加闭合括号
        (r"(?P<>hello)", r"(?P<name>hello)"), // 添加组名
        (r"*hello", r".*hello"),       // 添加目标
        (r"hello(?", r"hello"),        // 移除不完整的组
    ];
    
    for (broken, fixed) in patterns_with_fixes {
        println!("尝试修复模式: '{}' -> '{}'", broken, fixed);
        
        match parser.parse(broken) {
            Ok(_) => println!("  原始模式意外成功"),
            Err(e) => {
                println!("  原始模式错误: {}", e);
                
                match parser.parse(fixed) {
                    Ok(_) => println!("  修复模式成功"),
                    Err(e) => println!("  修复模式仍然错误: {}", e),
                }
            }
        }
    }
}
```

## 字符类和 Unicode

### 1. 字符类分析

```rust
use regex_syntax::{Parser, hir};

fn character_class_analysis() {
    let mut parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let char_classes = vec![
        r"[a-z]",
        r"[A-Z]",
        r"[0-9]",
        r"[a-zA-Z0-9]",
        r"[^a-z]",
        r"[\w]",
        r"[\d]",
        r"[\s]",
        r"[[:alpha:]]",
        r"[[:digit:]]",
        r"[[:space:]]",
    ];
    
    for pattern in char_classes {
        match parser.parse(pattern) {
            Ok(ast) => {
                match translator.translate(pattern, &ast) {
                    Ok(hir) => {
                        println!("字符类: {}", pattern);
                        if let hir::HirKind::Class(class) = hir.kind() {
                            println!("  范围数: {}", class.ranges().len());
                            for range in class.ranges() {
                                println!("    范围: {:?} - {:?}", range.start(), range.end());
                            }
                        }
                    }
                    Err(e) => {
                        println!("字符类 {} HIR 错误: {}", pattern, e);
                    }
                }
            }
            Err(e) => {
                println!("字符类 {} 解析错误: {}", pattern, e);
            }
        }
    }
}
```

### 2. Unicode 支持

```rust
use regex_syntax::{Parser, hir};

fn unicode_support() {
    let mut parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let unicode_patterns = vec![
        r"\p{L}",           // 字母
        r"\p{N}",           // 数字
        r"\p{P}",           // 标点
        r"\p{S}",           // 符号
        r"\p{Z}",           // 分隔符
        r"\p{C}",           // 控制字符
        r"\p{Ll}",          // 小写字母
        r"\p{Lu}",          // 大写字母
        r"\p{Lt}",          // 标题字母
        r"\p{Lm}",          // 修饰字母
        r"\p{Lo}",          // 其他字母
        r"\p{Script=Han}",  // 汉字
        r"\p{Block=CJK}",   // CJK 块
    ];
    
    for pattern in unicode_patterns {
        match parser.parse(pattern) {
            Ok(ast) => {
                match translator.translate(pattern, &ast) {
                    Ok(hir) => {
                        println!("Unicode 模式: {} 解析成功", pattern);
                        if let hir::HirKind::Class(class) = hir.kind() {
                            println!("  包含 {} 个字符范围", class.ranges().len());
                        }
                    }
                    Err(e) => {
                        println!("Unicode 模式: {} HIR 错误: {}", pattern, e);
                    }
                }
            }
            Err(e) => {
                println!("Unicode 模式: {} 解析错误: {}", pattern, e);
            }
        }
    }
}
```

## 高级功能

### 1. 正则表达式优化

```rust
use regex_syntax::{Parser, hir};

fn regex_optimization() {
    let mut parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let patterns = vec![
        (r"hello", "简单字面量"),
        (r"hello|hello", "重复选择"),
        (r"a*a*", "重复的重复"),
        (r"[a-z][a-z]", "字符类重复"),
        (r"(hello){1}", "单次重复组"),
        (r"hello{0,1}", "可选重复"),
        (r"hello?", "可选操作符"),
    ];
    
    for (pattern, description) in patterns {
        match parser.parse(pattern) {
            Ok(ast) => {
                match translator.translate(pattern, &ast) {
                    Ok(hir) => {
                        println!("模式: {} ({})", pattern, description);
                        println!("  优化后的 HIR: {:?}", hir);
                        analyze_optimization(&hir);
                    }
                    Err(e) => {
                        println!("模式: {} HIR 错误: {}", pattern, e);
                    }
                }
            }
            Err(e) => {
                println!("模式: {} 解析错误: {}", pattern, e);
            }
        }
    }
}

fn analyze_optimization(hir: &hir::Hir) {
    match hir.kind() {
        hir::HirKind::Literal(literal) => {
            println!("    已优化为字面量");
        }
        hir::HirKind::Class(class) => {
            println!("    已优化为字符类，范围数: {}", class.ranges().len());
        }
        hir::HirKind::Repetition(rep) => {
            println!("    重复操作: {:?}", rep.kind);
        }
        _ => {
            println!("    其他优化");
        }
    }
}
```

### 2. 语法树遍历

```rust
use regex_syntax::{Parser, ast::{self, Ast}};

fn ast_traversal() {
    let mut parser = Parser::new();
    let pattern = r"(hello|world)+\d{2,4}";
    
    match parser.parse(pattern) {
        Ok(ast) => {
            println!("遍历 AST: {}", pattern);
            traverse_ast(&ast, 0);
        }
        Err(e) => {
            println!("解析错误: {}", e);
        }
    }
}

fn traverse_ast(ast: &Ast, depth: usize) {
    let indent = "  ".repeat(depth);
    
    match ast {
        Ast::Empty(_) => println!("{}空", indent),
        Ast::Flags(_) => println!("{}标志", indent),
        Ast::Literal(lit) => println!("{}字面量: {:?}", indent, lit.c),
        Ast::Dot(_) => println!("{}点号", indent),
        Ast::Assertion(assertion) => {
            println!("{}断言: {:?}", indent, assertion.kind);
        }
        Ast::ClassUnicode(class) => {
            println!("{}Unicode 字符类: {:?}", indent, class.kind);
        }
        Ast::ClassPerl(class) => {
            println!("{}Perl 字符类: {:?}", indent, class.kind);
        }
        Ast::ClassBracketed(class) => {
            println!("{}括号字符类 (否定: {})", indent, class.negated);
            for item in &class.kind {
                match item {
                    ast::ClassSetItem::Literal(lit) => {
                        println!("{}  字面量: {:?}", indent, lit.c);
                    }
                    ast::ClassSetItem::Range(range) => {
                        println!("{}  范围: {:?}-{:?}", indent, range.start.c, range.end.c);
                    }
                    ast::ClassSetItem::Ascii(ascii) => {
                        println!("{}  ASCII: {:?}", indent, ascii.kind);
                    }
                    ast::ClassSetItem::Unicode(unicode) => {
                        println!("{}  Unicode: {:?}", indent, unicode.kind);
                    }
                    ast::ClassSetItem::Perl(perl) => {
                        println!("{}  Perl: {:?}", indent, perl.kind);
                    }
                    ast::ClassSetItem::Bracketed(bracketed) => {
                        println!("{}  嵌套括号类", indent);
                    }
                    ast::ClassSetItem::Union(union) => {
                        println!("{}  联合", indent);
                    }
                }
            }
        }
        Ast::Repetition(rep) => {
            println!("{}重复: {:?}", indent, rep.op);
            traverse_ast(&rep.ast, depth + 1);
        }
        Ast::Group(group) => {
            println!("{}分组: {:?}", indent, group.kind);
            traverse_ast(&group.ast, depth + 1);
        }
        Ast::Alternation(alt) => {
            println!("{}选择 ({} 个选项)", indent, alt.asts.len());
            for ast in &alt.asts {
                traverse_ast(ast, depth + 1);
            }
        }
        Ast::Concat(concat) => {
            println!("{}连接 ({} 个部分)", indent, concat.asts.len());
            for ast in &concat.asts {
                traverse_ast(ast, depth + 1);
            }
        }
    }
}
```

## 实际应用示例

### 1. 正则表达式验证器

```rust
use regex_syntax::{Parser, hir};

struct RegexValidator {
    parser: Parser,
    translator: hir::translate::Translator,
}

impl RegexValidator {
    fn new() -> Self {
        Self {
            parser: Parser::new(),
            translator: hir::translate::Translator::new(),
        }
    }
    
    fn validate(&self, pattern: &str) -> Result<ValidationResult, String> {
        // 解析 AST
        let ast = self.parser.parse(pattern)
            .map_err(|e| format!("AST 解析错误: {}", e))?;
        
        // 转换为 HIR
        let hir = self.translator.translate(pattern, &ast)
            .map_err(|e| format!("HIR 转换错误: {}", e))?;
        
        Ok(ValidationResult {
            pattern: pattern.to_string(),
            ast,
            hir,
            complexity: self.calculate_complexity(&hir),
        })
    }
    
    fn calculate_complexity(&self, hir: &hir::Hir) -> u32 {
        match hir.kind() {
            hir::HirKind::Empty => 0,
            hir::HirKind::Literal(_) => 1,
            hir::HirKind::Class(_) => 2,
            hir::HirKind::Anchor(_) => 1,
            hir::HirKind::WordBoundary(_) => 2,
            hir::HirKind::Repetition(rep) => {
                3 + self.calculate_complexity(&rep.hir)
            }
            hir::HirKind::Group(group) => {
                1 + self.calculate_complexity(&group.hir)
            }
            hir::HirKind::Concat(concat) => {
                concat.iter().map(|h| self.calculate_complexity(h)).sum()
            }
            hir::HirKind::Alternation(alt) => {
                2 + alt.iter().map(|h| self.calculate_complexity(h)).sum::<u32>()
            }
            _ => 1,
        }
    }
}

struct ValidationResult {
    pattern: String,
    ast: regex_syntax::ast::Ast,
    hir: hir::Hir,
    complexity: u32,
}

fn regex_validator_example() {
    let validator = RegexValidator::new();
    
    let test_patterns = vec![
        r"hello",
        r"\d+",
        r"[a-zA-Z0-9]+",
        r"(hello|world)*",
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})",
    ];
    
    for pattern in test_patterns {
        match validator.validate(pattern) {
            Ok(result) => {
                println!("模式: {}", result.pattern);
                println!("  复杂度: {}", result.complexity);
                println!("  验证通过");
            }
            Err(e) => {
                println!("模式: {} - 错误: {}", pattern, e);
            }
        }
    }
}
```

### 2. 正则表达式分析器

```rust
use regex_syntax::{Parser, hir, ast};

struct RegexAnalyzer {
    parser: Parser,
    translator: hir::translate::Translator,
}

impl RegexAnalyzer {
    fn new() -> Self {
        Self {
            parser: Parser::new(),
            translator: hir::translate::Translator::new(),
        }
    }
    
    fn analyze(&self, pattern: &str) -> Result<AnalysisResult, String> {
        let ast = self.parser.parse(pattern)
            .map_err(|e| format!("解析错误: {}", e))?;
        
        let hir = self.translator.translate(pattern, &ast)
            .map_err(|e| format!("HIR 转换错误: {}", e))?;
        
        Ok(AnalysisResult {
            pattern: pattern.to_string(),
            has_anchors: self.has_anchors(&hir),
            has_word_boundaries: self.has_word_boundaries(&hir),
            has_repetitions: self.has_repetitions(&hir),
            has_groups: self.has_groups(&hir),
            has_alternations: self.has_alternations(&hir),
            character_classes: self.count_character_classes(&hir),
            literals: self.extract_literals(&hir),
        })
    }
    
    fn has_anchors(&self, hir: &hir::Hir) -> bool {
        match hir.kind() {
            hir::HirKind::Anchor(_) => true,
            hir::HirKind::Concat(concat) => {
                concat.iter().any(|h| self.has_anchors(h))
            }
            hir::HirKind::Alternation(alt) => {
                alt.iter().any(|h| self.has_anchors(h))
            }
            hir::HirKind::Group(group) => {
                self.has_anchors(&group.hir)
            }
            hir::HirKind::Repetition(rep) => {
                self.has_anchors(&rep.hir)
            }
            _ => false,
        }
    }
    
    fn has_word_boundaries(&self, hir: &hir::Hir) -> bool {
        match hir.kind() {
            hir::HirKind::WordBoundary(_) => true,
            hir::HirKind::Concat(concat) => {
                concat.iter().any(|h| self.has_word_boundaries(h))
            }
            hir::HirKind::Alternation(alt) => {
                alt.iter().any(|h| self.has_word_boundaries(h))
            }
            hir::HirKind::Group(group) => {
                self.has_word_boundaries(&group.hir)
            }
            hir::HirKind::Repetition(rep) => {
                self.has_word_boundaries(&rep.hir)
            }
            _ => false,
        }
    }
    
    fn has_repetitions(&self, hir: &hir::Hir) -> bool {
        match hir.kind() {
            hir::HirKind::Repetition(_) => true,
            hir::HirKind::Concat(concat) => {
                concat.iter().any(|h| self.has_repetitions(h))
            }
            hir::HirKind::Alternation(alt) => {
                alt.iter().any(|h| self.has_repetitions(h))
            }
            hir::HirKind::Group(group) => {
                self.has_repetitions(&group.hir)
            }
            _ => false,
        }
    }
    
    fn has_groups(&self, hir: &hir::Hir) -> bool {
        match hir.kind() {
            hir::HirKind::Group(_) => true,
            hir::HirKind::Concat(concat) => {
                concat.iter().any(|h| self.has_groups(h))
            }
            hir::HirKind::Alternation(alt) => {
                alt.iter().any(|h| self.has_groups(h))
            }
            hir::HirKind::Repetition(rep) => {
                self.has_groups(&rep.hir)
            }
            _ => false,
        }
    }
    
    fn has_alternations(&self, hir: &hir::Hir) -> bool {
        match hir.kind() {
            hir::HirKind::Alternation(_) => true,
            hir::HirKind::Concat(concat) => {
                concat.iter().any(|h| self.has_alternations(h))
            }
            hir::HirKind::Group(group) => {
                self.has_alternations(&group.hir)
            }
            hir::HirKind::Repetition(rep) => {
                self.has_alternations(&rep.hir)
            }
            _ => false,
        }
    }
    
    fn count_character_classes(&self, hir: &hir::Hir) -> u32 {
        match hir.kind() {
            hir::HirKind::Class(_) => 1,
            hir::HirKind::Concat(concat) => {
                concat.iter().map(|h| self.count_character_classes(h)).sum()
            }
            hir::HirKind::Alternation(alt) => {
                alt.iter().map(|h| self.count_character_classes(h)).sum()
            }
            hir::HirKind::Group(group) => {
                self.count_character_classes(&group.hir)
            }
            hir::HirKind::Repetition(rep) => {
                self.count_character_classes(&rep.hir)
            }
            _ => 0,
        }
    }
    
    fn extract_literals(&self, hir: &hir::Hir) -> Vec<String> {
        let mut literals = Vec::new();
        self.extract_literals_recursive(hir, &mut literals);
        literals
    }
    
    fn extract_literals_recursive(&self, hir: &hir::Hir, literals: &mut Vec<String>) {
        match hir.kind() {
            hir::HirKind::Literal(literal) => {
                literals.push(String::from_utf8_lossy(literal).to_string());
            }
            hir::HirKind::Concat(concat) => {
                for h in concat {
                    self.extract_literals_recursive(h, literals);
                }
            }
            hir::HirKind::Alternation(alt) => {
                for h in alt {
                    self.extract_literals_recursive(h, literals);
                }
            }
            hir::HirKind::Group(group) => {
                self.extract_literals_recursive(&group.hir, literals);
            }
            hir::HirKind::Repetition(rep) => {
                self.extract_literals_recursive(&rep.hir, literals);
            }
            _ => {}
        }
    }
}

struct AnalysisResult {
    pattern: String,
    has_anchors: bool,
    has_word_boundaries: bool,
    has_repetitions: bool,
    has_groups: bool,
    has_alternations: bool,
    character_classes: u32,
    literals: Vec<String>,
}

fn regex_analyzer_example() {
    let analyzer = RegexAnalyzer::new();
    
    let test_patterns = vec![
        r"hello",
        r"^hello$",
        r"\bhello\b",
        r"hello+",
        r"(hello|world)",
        r"[a-z]+",
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
    ];
    
    for pattern in test_patterns {
        match analyzer.analyze(pattern) {
            Ok(result) => {
                println!("分析结果: {}", result.pattern);
                println!("  有锚点: {}", result.has_anchors);
                println!("  有词边界: {}", result.has_word_boundaries);
                println!("  有重复: {}", result.has_repetitions);
                println!("  有分组: {}", result.has_groups);
                println!("  有选择: {}", result.has_alternations);
                println!("  字符类数量: {}", result.character_classes);
                println!("  字面量: {:?}", result.literals);
            }
            Err(e) => {
                println!("分析错误: {} - {}", pattern, e);
            }
        }
    }
}
```

## 性能优化

### 1. 解析器重用

```rust
use regex_syntax::{Parser, hir};

struct RegexProcessor {
    parser: Parser,
    translator: hir::translate::Translator,
}

impl RegexProcessor {
    fn new() -> Self {
        Self {
            parser: Parser::new(),
            translator: hir::translate::Translator::new(),
        }
    }
    
    fn process_batch(&self, patterns: &[&str]) -> Vec<Result<hir::Hir, String>> {
        patterns.iter().map(|pattern| {
            let ast = self.parser.parse(pattern)
                .map_err(|e| format!("解析错误: {}", e))?;
            
            self.translator.translate(pattern, &ast)
                .map_err(|e| format!("HIR 转换错误: {}", e))
        }).collect()
    }
}

fn batch_processing_example() {
    let processor = RegexProcessor::new();
    
    let patterns = vec![
        r"hello",
        r"\d+",
        r"[a-z]+",
        r"(hello|world)*",
        r"^start",
        r"end$",
    ];
    
    let results = processor.process_batch(&patterns);
    
    for (pattern, result) in patterns.iter().zip(results) {
        match result {
            Ok(hir) => {
                println!("模式: {} - 处理成功", pattern);
            }
            Err(e) => {
                println!("模式: {} - 处理失败: {}", pattern, e);
            }
        }
    }
}
```

### 2. 缓存机制

```rust
use regex_syntax::{Parser, hir};
use std::collections::HashMap;

struct CachedRegexProcessor {
    parser: Parser,
    translator: hir::translate::Translator,
    cache: HashMap<String, Result<hir::Hir, String>>,
}

impl CachedRegexProcessor {
    fn new() -> Self {
        Self {
            parser: Parser::new(),
            translator: hir::translate::Translator::new(),
            cache: HashMap::new(),
        }
    }
    
    fn process(&mut self, pattern: &str) -> Result<hir::Hir, String> {
        if let Some(cached) = self.cache.get(pattern) {
            return cached.clone();
        }
        
        let result = self.parse_pattern(pattern);
        self.cache.insert(pattern.to_string(), result.clone());
        result
    }
    
    fn parse_pattern(&self, pattern: &str) -> Result<hir::Hir, String> {
        let ast = self.parser.parse(pattern)
            .map_err(|e| format!("解析错误: {}", e))?;
        
        self.translator.translate(pattern, &ast)
            .map_err(|e| format!("HIR 转换错误: {}", e))
    }
    
    fn cache_stats(&self) -> (usize, usize) {
        let total = self.cache.len();
        let successful = self.cache.values().filter(|r| r.is_ok()).count();
        (successful, total)
    }
}

fn cached_processing_example() {
    let mut processor = CachedRegexProcessor::new();
    
    let patterns = vec![
        r"hello",
        r"\d+",
        r"hello",     // 重复模式
        r"[a-z]+",
        r"\d+",       // 重复模式
        r"world",
    ];
    
    for pattern in patterns {
        match processor.process(pattern) {
            Ok(_) => {
                println!("模式: {} - 处理成功", pattern);
            }
            Err(e) => {
                println!("模式: {} - 处理失败: {}", pattern, e);
            }
        }
    }
    
    let (successful, total) = processor.cache_stats();
    println!("缓存统计: {}/{} 成功", successful, total);
}
```

## 最佳实践

1. **错误处理**: 始终处理解析和转换错误
2. **性能**: 重用 Parser 和 Translator 实例
3. **缓存**: 对频繁使用的模式使用缓存
4. **验证**: 在处理用户输入前验证正则表达式
5. **配置**: 根据需求配置解析器选项

## 调试技巧

### 1. 调试输出

```rust
use regex_syntax::{Parser, hir};

fn debug_regex_parsing() {
    let parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let pattern = r"(hello|world)+\d{2,4}";
    
    println!("调试模式: {}", pattern);
    
    // 解析 AST
    match parser.parse(pattern) {
        Ok(ast) => {
            println!("AST 解析成功:");
            println!("{:#?}", ast);
            
            // 转换为 HIR
            match translator.translate(pattern, &ast) {
                Ok(hir) => {
                    println!("HIR 转换成功:");
                    println!("{:#?}", hir);
                }
                Err(e) => {
                    println!("HIR 转换失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("AST 解析失败: {}", e);
        }
    }
}
```

### 2. 性能测试

```rust
use regex_syntax::{Parser, hir};
use std::time::Instant;

fn performance_testing() {
    let parser = Parser::new();
    let translator = hir::translate::Translator::new();
    
    let complex_pattern = r"^(?P<scheme>[a-zA-Z][a-zA-Z0-9+.-]*):(?P<authority>//(?P<userinfo>(?P<username>[^:@]+)(?::(?P<password>[^@]+))?@)?(?P<host>[^:/?#]+)(?::(?P<port>\d+))?)?(?P<path>[^?#]*)(?P<query>\?[^#]*)?(?P<fragment>#.*)?$";
    
    let iterations = 1000;
    
    // 测试解析性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parser.parse(complex_pattern);
    }
    let parse_time = start.elapsed();
    
    // 测试完整处理性能
    let start = Instant::now();
    for _ in 0..iterations {
        if let Ok(ast) = parser.parse(complex_pattern) {
            let _ = translator.translate(complex_pattern, &ast);
        }
    }
    let full_time = start.elapsed();
    
    println!("性能测试结果 ({} 次迭代):", iterations);
    println!("  解析时间: {:?}", parse_time);
    println!("  完整处理时间: {:?}", full_time);
    println!("  平均解析时间: {:?}", parse_time / iterations);
    println!("  平均完整处理时间: {:?}", full_time / iterations);
}
```

## 总结

`regex-syntax` 是一个功能强大的正则表达式解析库，提供了：

- **完整的语法支持**: 支持现代正则表达式的所有特性
- **灵活的配置**: 可配置的解析选项
- **高性能**: 优化的解析和转换算法
- **丰富的 API**: AST 和 HIR 两层抽象
- **Unicode 支持**: 全面的 Unicode 字符类支持

通过掌握 `regex-syntax`，您可以构建复杂的正则表达式工具，进行模式分析、验证和优化，为正则表达式处理提供强大的基础支持。