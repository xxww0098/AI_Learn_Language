# proc-macro2 1.0.95 详细中文使用教程

## 简介

`proc-macro2` 是编译器 `proc_macro` API 的替代实现，旨在将基于令牌的库与过程宏用例解耦。它提供了一个稳定的、与 `proc_macro` 兼容的接口，可以在没有过程宏的情况下使用。

## 基本信息

- **版本**: 1.0.95
- **许可证**: MIT OR Apache-2.0
- **文档**: https://docs.rs/proc-macro2
- **仓库**: https://github.com/dtolnay/proc-macro2
- **下载量**: 659,598,847 (极高人气)

## 快速开始

### 1. 添加依赖

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
proc-macro2 = "1.0.95"
```

### 2. 基本用法

```rust
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

fn main() {
    // 创建一个简单的 TokenStream
    let tokens: TokenStream = "let x = 42;".parse().unwrap();
    
    // 打印令牌流
    println!("Tokens: {}", tokens);
    
    // 遍历令牌树
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => println!("标识符: {}", ident),
            TokenTree::Literal(lit) => println!("字面量: {}", lit),
            TokenTree::Punct(punct) => println!("标点: {}", punct),
            TokenTree::Group(group) => println!("分组: {}", group),
        }
    }
}
```

## 核心概念

### 1. TokenStream

`TokenStream` 是令牌的序列，是过程宏的核心数据结构。

```rust
use proc_macro2::{TokenStream, Span};
use quote::quote;

fn token_stream_examples() {
    // 从字符串解析
    let tokens: TokenStream = "fn hello() {}".parse().unwrap();
    
    // 使用 quote! 宏创建
    let tokens = quote! {
        fn hello() {
            println!("Hello, world!");
        }
    };
    
    // 创建空的 TokenStream
    let empty = TokenStream::new();
    
    // 组合 TokenStream
    let combined = quote! {
        #tokens
        #empty
    };
    
    println!("组合的令牌流: {}", combined);
}
```

### 2. TokenTree

`TokenTree` 是令牌流中的单个元素。

```rust
use proc_macro2::{TokenTree, Delimiter, Spacing};

fn token_tree_examples() {
    let tokens: TokenStream = "let x = (1 + 2);".parse().unwrap();
    
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                println!("标识符: {} (span: {:?})", ident, ident.span());
            }
            TokenTree::Literal(lit) => {
                println!("字面量: {}", lit);
            }
            TokenTree::Punct(punct) => {
                println!("标点: {} (spacing: {:?})", punct.as_char(), punct.spacing());
            }
            TokenTree::Group(group) => {
                println!("分组: {} (delimiter: {:?})", group.stream(), group.delimiter());
            }
        }
    }
}
```

### 3. Span

`Span` 表示令牌在源代码中的位置信息。

```rust
use proc_macro2::{Span, Ident};

fn span_examples() {
    // 创建一个调用点的 span
    let span = Span::call_site();
    
    // 创建一个混合 span
    let mixed_span = Span::mixed_site();
    
    // 创建标识符并指定 span
    let ident = Ident::new("my_variable", span);
    
    println!("标识符: {} (span: {:?})", ident, ident.span());
    
    // 获取 span 的源文本（需要 "span-locations" 功能）
    #[cfg(feature = "span-locations")]
    {
        let start = span.start();
        let end = span.end();
        println!("Span 位置: {}:{} 到 {}:{}", 
                 start.line, start.column, end.line, end.column);
    }
}
```

## 高级功能

### 1. 自定义过程宏

```rust
use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;

// 模拟一个简单的 derive 宏
fn derive_debug(input: TokenStream) -> TokenStream {
    // 解析结构体名称（简化版本）
    let struct_name = extract_struct_name(input.clone());
    
    quote! {
        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!(#struct_name))
            }
        }
    }
}

fn extract_struct_name(tokens: TokenStream) -> Ident {
    // 简化的结构体名称提取
    for token in tokens {
        if let proc_macro2::TokenTree::Ident(ident) = token {
            if ident == "struct" {
                continue;
            }
            return ident;
        }
    }
    Ident::new("Unknown", Span::call_site())
}
```

### 2. 令牌流操作

```rust
use proc_macro2::{TokenStream, TokenTree, Delimiter, Group, Punct, Spacing};

fn manipulate_tokens() {
    let input: TokenStream = "fn test() { x + y }".parse().unwrap();
    let mut output = TokenStream::new();
    
    for token in input {
        match token {
            TokenTree::Ident(ident) => {
                // 将所有标识符转换为大写
                let upper_ident = Ident::new(
                    &ident.to_string().to_uppercase(),
                    ident.span()
                );
                output.extend(Some(TokenTree::Ident(upper_ident)));
            }
            TokenTree::Punct(punct) => {
                // 保持标点符号不变
                output.extend(Some(TokenTree::Punct(punct)));
            }
            TokenTree::Literal(lit) => {
                // 保持字面量不变
                output.extend(Some(TokenTree::Literal(lit)));
            }
            TokenTree::Group(group) => {
                // 递归处理分组
                let inner = manipulate_tokens_recursive(group.stream());
                let new_group = Group::new(group.delimiter(), inner);
                output.extend(Some(TokenTree::Group(new_group)));
            }
        }
    }
    
    println!("转换后的令牌流: {}", output);
}

fn manipulate_tokens_recursive(tokens: TokenStream) -> TokenStream {
    // 递归处理令牌流的实现
    TokenStream::new()
}
```

### 3. 错误处理

```rust
use proc_macro2::{TokenStream, Span};

// 自定义错误类型
#[derive(Debug)]
struct MacroError {
    message: String,
    span: Span,
}

impl MacroError {
    fn new(message: &str, span: Span) -> Self {
        Self {
            message: message.to_string(),
            span,
        }
    }
    
    fn to_compile_error(&self) -> TokenStream {
        let message = &self.message;
        quote::quote_spanned! { self.span =>
            compile_error!(#message);
        }
    }
}

fn error_handling_example() {
    let input: TokenStream = "invalid syntax".parse().unwrap();
    
    match validate_input(input.clone()) {
        Ok(result) => {
            println!("处理成功: {}", result);
        }
        Err(error) => {
            let error_tokens = error.to_compile_error();
            println!("编译错误: {}", error_tokens);
        }
    }
}

fn validate_input(input: TokenStream) -> Result<TokenStream, MacroError> {
    // 简化的输入验证
    if input.is_empty() {
        return Err(MacroError::new("输入不能为空", Span::call_site()));
    }
    
    Ok(input)
}
```

## 实际应用示例

### 1. 创建 Getter 方法

```rust
use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;

fn generate_getters(struct_name: &str, fields: &[(&str, &str)]) -> TokenStream {
    let struct_ident = Ident::new(struct_name, Span::call_site());
    let mut getters = TokenStream::new();
    
    for (field_name, field_type) in fields {
        let field_ident = Ident::new(field_name, Span::call_site());
        let type_ident = Ident::new(field_type, Span::call_site());
        
        let getter = quote! {
            impl #struct_ident {
                pub fn #field_ident(&self) -> &#type_ident {
                    &self.#field_ident
                }
            }
        };
        
        getters.extend(getter);
    }
    
    getters
}

fn getter_example() {
    let fields = [("name", "String"), ("age", "u32")];
    let getters = generate_getters("Person", &fields);
    
    println!("生成的 getter 方法:\n{}", getters);
}
```

### 2. 配置宏生成器

```rust
use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;

fn generate_config_struct(config_name: &str, fields: &[(&str, &str, &str)]) -> TokenStream {
    let struct_ident = Ident::new(config_name, Span::call_site());
    let mut field_definitions = TokenStream::new();
    let mut default_values = TokenStream::new();
    
    for (field_name, field_type, default_value) in fields {
        let field_ident = Ident::new(field_name, Span::call_site());
        let type_ident = Ident::new(field_type, Span::call_site());
        let default_literal: TokenStream = default_value.parse().unwrap();
        
        let field_def = quote! {
            pub #field_ident: #type_ident,
        };
        
        let default_val = quote! {
            #field_ident: #default_literal,
        };
        
        field_definitions.extend(field_def);
        default_values.extend(default_val);
    }
    
    quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_ident {
            #field_definitions
        }
        
        impl Default for #struct_ident {
            fn default() -> Self {
                Self {
                    #default_values
                }
            }
        }
    }
}

fn config_example() {
    let fields = [
        ("host", "String", "\"localhost\".to_string()"),
        ("port", "u16", "8080"),
        ("debug", "bool", "false"),
    ];
    
    let config_struct = generate_config_struct("ServerConfig", &fields);
    println!("生成的配置结构体:\n{}", config_struct);
}
```

### 3. 枚举工具生成器

```rust
use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;

fn generate_enum_utils(enum_name: &str, variants: &[&str]) -> TokenStream {
    let enum_ident = Ident::new(enum_name, Span::call_site());
    let mut variant_arms = TokenStream::new();
    let mut all_variants = TokenStream::new();
    
    for variant in variants {
        let variant_ident = Ident::new(variant, Span::call_site());
        
        let arm = quote! {
            #enum_ident::#variant_ident => stringify!(#variant_ident),
        };
        
        let variant_ref = quote! {
            #enum_ident::#variant_ident,
        };
        
        variant_arms.extend(arm);
        all_variants.extend(variant_ref);
    }
    
    quote! {
        impl #enum_ident {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #variant_arms
                }
            }
            
            pub fn all() -> &'static [#enum_ident] {
                &[#all_variants]
            }
            
            pub fn count() -> usize {
                Self::all().len()
            }
        }
    }
}

fn enum_utils_example() {
    let variants = ["Red", "Green", "Blue"];
    let enum_utils = generate_enum_utils("Color", &variants);
    
    println!("生成的枚举工具:\n{}", enum_utils);
}
```

## 与其他库的集成

### 1. 与 syn 集成

```rust
use proc_macro2::TokenStream;
use syn::{DeriveInput, Data, Fields};
use quote::quote;

fn parse_with_syn(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let name = &input.ident;
    
    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields.named.iter()
                        .map(|f| &f.ident)
                        .collect();
                    
                    Ok(quote! {
                        impl #name {
                            pub fn field_names() -> &'static [&'static str] {
                                &[#(stringify!(#field_names)),*]
                            }
                        }
                    })
                }
                _ => Err(syn::Error::new_spanned(name, "只支持命名字段"))
            }
        }
        _ => Err(syn::Error::new_spanned(name, "只支持结构体"))
    }
}
```

### 2. 与 quote 集成

```rust
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

fn advanced_quote_usage() {
    let struct_name = format_ident!("MyStruct");
    let field_count = 3;
    
    let tokens = quote! {
        pub struct #struct_name {
            #(
                #(format_ident!("field_{}", i)): i32,
            )*
        }
    };
    
    println!("高级 quote 用法: {}", tokens);
}
```

## 性能优化

### 1. 令牌流缓存

```rust
use proc_macro2::TokenStream;
use std::collections::HashMap;

struct TokenCache {
    cache: HashMap<String, TokenStream>,
}

impl TokenCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    fn get_or_generate<F>(&mut self, key: &str, generator: F) -> TokenStream
    where
        F: FnOnce() -> TokenStream,
    {
        self.cache.entry(key.to_string())
            .or_insert_with(generator)
            .clone()
    }
}
```

### 2. 延迟评估

```rust
use proc_macro2::TokenStream;

struct LazyTokens {
    generator: Box<dyn Fn() -> TokenStream>,
}

impl LazyTokens {
    fn new<F>(generator: F) -> Self
    where
        F: Fn() -> TokenStream + 'static,
    {
        Self {
            generator: Box::new(generator),
        }
    }
    
    fn evaluate(&self) -> TokenStream {
        (self.generator)()
    }
}
```

## 调试技巧

### 1. 令牌流调试

```rust
use proc_macro2::TokenStream;

fn debug_tokens(tokens: &TokenStream) {
    println!("令牌流调试信息:");
    println!("  内容: {}", tokens);
    println!("  长度: {}", tokens.clone().into_iter().count());
    
    for (i, token) in tokens.clone().into_iter().enumerate() {
        println!("  [{:2}] {:?}", i, token);
    }
}
```

### 2. 语法高亮输出

```rust
use proc_macro2::TokenStream;

fn pretty_print_tokens(tokens: &TokenStream) {
    let formatted = prettyplease::unparse(&syn::parse2(tokens.clone()).unwrap());
    println!("格式化的代码:\n{}", formatted);
}
```

## 最佳实践

1. **错误处理**: 始终提供清晰的错误消息和正确的 span 信息
2. **性能**: 避免不必要的令牌流克隆
3. **可读性**: 使用 `quote!` 宏生成可读的代码
4. **测试**: 为生成的代码编写全面的测试
5. **文档**: 为复杂的宏提供详细的文档和示例

## 常见问题

### 1. Span 信息丢失

```rust
use proc_macro2::{TokenStream, Span};

fn preserve_spans(input: TokenStream) -> TokenStream {
    // 错误：丢失 span 信息
    // let output = quote! { fn new_function() {} };
    
    // 正确：保持 span 信息
    let span = input.clone().into_iter().next()
        .map(|t| t.span())
        .unwrap_or_else(Span::call_site);
    
    quote::quote_spanned! { span =>
        fn new_function() {}
    }
}
```

### 2. 令牌流解析失败

```rust
use proc_macro2::TokenStream;

fn safe_parse(input: &str) -> Result<TokenStream, proc_macro2::LexError> {
    input.parse()
}

fn parse_with_fallback(input: &str) -> TokenStream {
    input.parse().unwrap_or_else(|_| {
        quote::quote! { compile_error!("无法解析输入"); }
    })
}
```

## 总结

`proc-macro2` 是 Rust 元编程的核心库，提供了强大的令牌操作功能。它的主要优势包括：

- **稳定性**: 提供稳定的 API，独立于编译器版本
- **灵活性**: 可以在非过程宏环境中使用
- **性能**: 高效的令牌流操作
- **生态系统**: 与 `syn` 和 `quote` 完美集成

通过掌握 `proc-macro2`，您可以创建强大的代码生成工具和过程宏，大大提高 Rust 开发的效率和代码的可维护性。