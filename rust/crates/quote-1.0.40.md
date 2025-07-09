# Quote 1.0.40 - Rust 准引用宏完全教程

## 目录
- [概述](#概述)
- [快速开始](#快速开始)
- [基本概念](#基本概念)
- [quote! 宏语法](#quote-宏语法)
- [变量插值](#变量插值)
- [重复和循环](#重复和循环)
- [条件生成](#条件生成)
- [类型和标识符](#类型和标识符)
- [过程宏集成](#过程宏集成)
- [高级技巧](#高级技巧)
- [错误处理](#错误处理)
- [性能优化](#性能优化)
- [实战案例](#实战案例)
- [最佳实践](#最佳实践)

## 概述

Quote 是一个用于生成 Rust 代码的准引用宏库，它是开发过程宏 (procedural macros) 的核心工具。Quote 提供了简洁的语法来构建和操作 Rust 代码的抽象语法树 (AST)。

### 核心特性
- **准引用语法**: 直观的代码生成语法
- **变量插值**: 将 Rust 值嵌入到生成的代码中
- **重复机制**: 支持循环和条件代码生成
- **类型安全**: 编译时保证生成代码的正确性
- **高性能**: 优化的代码生成性能
- **与 syn 配合**: 完美集成语法解析

### 版本信息
- **当前版本**: 1.0.40
- **发布时间**: 2025-03-11
- **下载次数**: 644,708,875+
- **许可证**: MIT OR Apache-2.0

## 快速开始

### 安装配置

```toml
[dependencies]
quote = "1.0.40"
proc-macro2 = "1.0"
syn = { version = "2.0", features = ["full"] }
```

### 基本示例

```rust
use quote::quote;
use proc_macro2::TokenStream;

fn main() {
    let name = "hello";
    let tokens = quote! {
        fn #name() {
            println!("Hello, world!");
        }
    };
    
    println!("{}", tokens);
}
```

### 过程宏基础

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl MyTrait for #name {
            fn my_method(&self) {
                println!("MyTrait implementation for {}", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

## 基本概念

### TokenStream 理解

```rust
use quote::quote;
use proc_macro2::TokenStream;

fn tokenstream_basics() {
    // 生成简单的 token stream
    let tokens = quote! { let x = 42; };
    println!("Tokens: {}", tokens);
    
    // 合并多个 token streams
    let part1 = quote! { let x = };
    let part2 = quote! { 42; };
    let combined = quote! { #part1 #part2 };
    println!("Combined: {}", combined);
    
    // 空的 token stream
    let empty = quote! {};
    assert!(empty.is_empty());
    
    // 从字符串解析
    let parsed: TokenStream = "let y = 100;".parse().unwrap();
    let quoted = quote! { #parsed };
    println!("Parsed: {}", quoted);
}
```

### 语法树操作

```rust
use quote::{quote, ToTokens};
use syn::{Ident, Type, Expr};
use proc_macro2::Span;

fn syntax_tree_operations() {
    // 创建标识符
    let ident = Ident::new("my_var", Span::call_site());
    let tokens = quote! { let #ident = 42; };
    println!("Identifier: {}", tokens);
    
    // 处理类型
    let ty: Type = syn::parse_quote!(Vec<String>);
    let tokens = quote! { 
        let data: #ty = Vec::new();
    };
    println!("Type: {}", tokens);
    
    // 处理表达式
    let expr: Expr = syn::parse_quote!(1 + 2 * 3);
    let tokens = quote! {
        let result = #expr;
    };
    println!("Expression: {}", tokens);
}
```

### quote! 宏的工作原理

```rust
use quote::quote;
use proc_macro2::TokenStream;

fn quote_mechanism() {
    // quote! 将 Rust 代码转换为 TokenStream
    let tokens = quote! {
        struct Point {
            x: f64,
            y: f64,
        }
    };
    
    // 可以将 TokenStream 转换为字符串
    let code_string = tokens.to_string();
    println!("Generated code:\n{}", code_string);
    
    // 可以解析和重新生成
    let parsed: TokenStream = code_string.parse().unwrap();
    let re_quoted = quote! { #parsed };
    
    assert_eq!(tokens.to_string(), re_quoted.to_string());
}
```

## quote! 宏语法

### 基本语法元素

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn basic_syntax_elements() {
    // 字面量
    let tokens = quote! {
        let number = 42;
        let text = "hello";
        let boolean = true;
        let float = 3.14;
    };
    
    // 标识符
    let var_name = Ident::new("my_variable", Span::call_site());
    let tokens = quote! {
        let #var_name = 100;
    };
    
    // 运算符和符号
    let tokens = quote! {
        let result = a + b * c / d;
        let comparison = x > y && z < w;
    };
    
    // 关键字
    let tokens = quote! {
        pub struct MyStruct {
            pub field: i32,
        }
        
        impl MyStruct {
            pub fn new() -> Self {
                Self { field: 0 }
            }
        }
    };
    
    println!("Basic syntax: {}", tokens);
}
```

### 结构化代码生成

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

fn structured_code_generation() {
    let struct_name = Ident::new("Person", Span::call_site());
    let field_name = Ident::new("name", Span::call_site());
    let field_type: Type = syn::parse_quote!(String);
    
    let tokens = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            pub #field_name: #field_type,
            pub age: u32,
        }
        
        impl #struct_name {
            pub fn new(#field_name: #field_type, age: u32) -> Self {
                Self {
                    #field_name,
                    age,
                }
            }
            
            pub fn greet(&self) {
                println!("Hello, my name is {}", self.#field_name);
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #field_name: String::new(),
                    age: 0,
                }
            }
        }
    };
    
    println!("Structured code:\n{}", tokens);
}
```

### 函数和方法生成

```rust
use quote::quote;
use syn::{Ident, Type, ReturnType};
use proc_macro2::Span;

fn function_generation() {
    let func_name = Ident::new("calculate", Span::call_site());
    let param_name = Ident::new("value", Span::call_site());
    let param_type: Type = syn::parse_quote!(i32);
    let return_type: Type = syn::parse_quote!(i32);
    
    let tokens = quote! {
        pub fn #func_name(#param_name: #param_type) -> #return_type {
            #param_name * 2 + 1
        }
        
        pub async fn async_#func_name(#param_name: #param_type) -> Result<#return_type, String> {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok(#param_name * 2 + 1)
        }
        
        pub fn generic_function<T>(value: T) -> T 
        where 
            T: Clone + std::fmt::Debug,
        {
            println!("Processing: {:?}", value);
            value.clone()
        }
    };
    
    println!("Functions:\n{}", tokens);
}
```

## 变量插值

### 基本插值

```rust
use quote::quote;
use syn::{Ident, LitStr, LitInt};
use proc_macro2::Span;

fn basic_interpolation() {
    // 插入标识符
    let name = Ident::new("my_function", Span::call_site());
    let tokens = quote! {
        fn #name() {
            println!("Function name: {}", stringify!(#name));
        }
    };
    
    // 插入字面量
    let message = LitStr::new("Hello, World!", Span::call_site());
    let number = LitInt::new("42", Span::call_site());
    let tokens = quote! {
        fn greet() {
            println!(#message);
            let answer = #number;
        }
    };
    
    // 插入表达式
    let expr = quote! { 1 + 2 * 3 };
    let tokens = quote! {
        let result = #expr;
    };
    
    println!("Interpolation: {}", tokens);
}
```

### 复杂插值

```rust
use quote::quote;
use syn::{Expr, Type, Path};

fn complex_interpolation() {
    // 插入路径
    let path: Path = syn::parse_quote!(std::collections::HashMap);
    let tokens = quote! {
        use #path;
        let map = HashMap::new();
    };
    
    // 插入类型
    let key_type: Type = syn::parse_quote!(String);
    let value_type: Type = syn::parse_quote!(i32);
    let tokens = quote! {
        let map: std::collections::HashMap<#key_type, #value_type> = HashMap::new();
    };
    
    // 插入代码块
    let block = quote! {
        println!("Starting calculation");
        let result = 42 * 2;
        println!("Result: {}", result);
        result
    };
    
    let tokens = quote! {
        fn calculate() -> i32 {
            #block
        }
    };
    
    // 条件插值
    let debug_mode = true;
    let debug_code = if debug_mode {
        quote! { println!("Debug: Processing item"); }
    } else {
        quote! {}
    };
    
    let tokens = quote! {
        fn process_item() {
            #debug_code
            // 处理逻辑
        }
    };
    
    println!("Complex interpolation: {}", tokens);
}
```

### 嵌套插值

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn nested_interpolation() {
    let struct_name = Ident::new("Config", Span::call_site());
    let field_prefix = "setting";
    
    // 生成多个字段
    let fields = (1..=3).map(|i| {
        let field_name = Ident::new(&format!("{}_{}", field_prefix, i), Span::call_site());
        quote! {
            pub #field_name: String,
        }
    });
    
    let tokens = quote! {
        pub struct #struct_name {
            #(#fields)*
        }
        
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(
                        #(Ident::new(&format!("{}_{}", field_prefix, (1..=3)), Span::call_site())): String::new(),
                    )*
                }
            }
        }
    };
    
    // 更简洁的方式
    let field_names: Vec<_> = (1..=3)
        .map(|i| Ident::new(&format!("{}_{}", field_prefix, i), Span::call_site()))
        .collect();
    
    let tokens = quote! {
        pub struct #struct_name {
            #(pub #field_names: String,)*
        }
        
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#field_names: String::new(),)*
                }
            }
        }
    };
    
    println!("Nested interpolation: {}", tokens);
}
```

## 重复和循环

### 基本重复语法

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn basic_repetition() {
    let names = vec!["alice", "bob", "charlie"];
    let idents: Vec<_> = names.iter()
        .map(|name| Ident::new(name, Span::call_site()))
        .collect();
    
    // 重复生成变量
    let tokens = quote! {
        #(let #idents = String::new();)*
    };
    
    // 重复生成函数
    let tokens = quote! {
        #(
            fn #idents() {
                println!("Hello from {}", stringify!(#idents));
            }
        )*
    };
    
    // 重复生成结构体字段
    let tokens = quote! {
        struct Person {
            #(#idents: String,)*
        }
    };
    
    println!("Basic repetition: {}", tokens);
}
```

### 复杂重复模式

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

fn complex_repetition() {
    let fields = vec![
        ("name", "String"),
        ("age", "u32"),
        ("email", "Option<String>"),
    ];
    
    let field_defs = fields.iter().map(|(name, ty_str)| {
        let name = Ident::new(name, Span::call_site());
        let ty: Type = syn::parse_str(ty_str).unwrap();
        quote! { pub #name: #ty }
    });
    
    let field_names: Vec<_> = fields.iter()
        .map(|(name, _)| Ident::new(name, Span::call_site()))
        .collect();
    
    let tokens = quote! {
        #[derive(Debug, Clone)]
        pub struct User {
            #(#field_defs,)*
        }
        
        impl User {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names,)*
                }
            }
        }
    };
    
    // 条件重复
    let optional_fields = fields.iter()
        .filter(|(_, ty)| ty.starts_with("Option"))
        .map(|(name, _)| {
            let name = Ident::new(name, Span::call_site());
            quote! {
                pub fn #name(&self) -> Option<&str> {
                    self.#name.as_deref()
                }
            }
        });
    
    let tokens = quote! {
        impl User {
            #(#optional_fields)*
        }
    };
    
    println!("Complex repetition: {}", tokens);
}
```

### 嵌套重复

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn nested_repetition() {
    let modules = vec![
        ("users", vec!["create", "read", "update", "delete"]),
        ("posts", vec!["publish", "draft", "archive"]),
        ("comments", vec!["approve", "reject"]),
    ];
    
    let module_code = modules.iter().map(|(module_name, actions)| {
        let module_ident = Ident::new(module_name, Span::call_site());
        let action_idents: Vec<_> = actions.iter()
            .map(|action| Ident::new(action, Span::call_site()))
            .collect();
        
        quote! {
            pub mod #module_ident {
                #(
                    pub fn #action_idents() {
                        println!("Executing {} in {}", stringify!(#action_idents), stringify!(#module_ident));
                    }
                )*
                
                pub fn list_actions() {
                    println!("Available actions in {}:", stringify!(#module_ident));
                    #(
                        println!("  - {}", stringify!(#action_idents));
                    )*
                }
            }
        }
    });
    
    let tokens = quote! {
        #(#module_code)*
        
        pub fn list_all_modules() {
            #(
                #(Ident::new(&modules.iter().map(|(name, _)| name).collect::<Vec<_>>(), Span::call_site())::list_actions();)*
            )*
        }
    };
    
    println!("Nested repetition: {}", tokens);
}
```

### 分隔符重复

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn separator_repetition() {
    let params = vec!["x", "y", "z"];
    let param_idents: Vec<_> = params.iter()
        .map(|param| Ident::new(param, Span::call_site()))
        .collect();
    
    // 逗号分隔
    let tokens = quote! {
        fn calculate(#(#param_idents: f64),*) -> f64 {
            #(#param_idents)+*
        }
    };
    
    // 分号分隔
    let tokens = quote! {
        fn print_all() {
            #(println!("Value: {}", #param_idents);)*
        }
    };
    
    // 加号分隔（数学运算）
    let tokens = quote! {
        fn sum() -> f64 {
            #(#param_idents)+*
        }
    };
    
    // 自定义分隔符
    let tokens = quote! {
        fn format_values() -> String {
            format!(#("{}"),*)
        }
    };
    
    // 条件分隔符
    let is_last = |i: usize| i == param_idents.len() - 1;
    let formatted_params = param_idents.iter().enumerate().map(|(i, ident)| {
        if is_last(i) {
            quote! { stringify!(#ident) }
        } else {
            quote! { stringify!(#ident), }
        }
    });
    
    let tokens = quote! {
        const PARAM_NAMES: &[&str] = &[#(#formatted_params)*];
    };
    
    println!("Separator repetition: {}", tokens);
}
```

## 条件生成

### 基本条件

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

fn basic_conditionals(debug_mode: bool, async_support: bool) {
    let struct_name = Ident::new("MyStruct", Span::call_site());
    
    let debug_impl = if debug_mode {
        quote! {
            impl std::fmt::Debug for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#struct_name))
                        .field("debug_info", &"enabled")
                        .finish()
                }
            }
        }
    } else {
        quote! {}
    };
    
    let async_methods = if async_support {
        quote! {
            impl #struct_name {
                pub async fn async_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl #struct_name {
                pub fn sync_operation(&self) -> Result<(), Box<dyn std::error::Error>> {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    Ok(())
                }
            }
        }
    };
    
    let tokens = quote! {
        pub struct #struct_name {
            data: String,
        }
        
        #debug_impl
        #async_methods
    };
    
    println!("Conditional generation: {}", tokens);
}
```

### 复杂条件逻辑

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

#[derive(Debug)]
enum FieldType {
    Required(String),
    Optional(String),
    Collection(String),
}

fn complex_conditionals() {
    let fields = vec![
        ("id", FieldType::Required("u32".to_string())),
        ("name", FieldType::Required("String".to_string())),
        ("email", FieldType::Optional("String".to_string())),
        ("tags", FieldType::Collection("Vec<String>".to_string())),
    ];
    
    let struct_name = Ident::new("User", Span::call_site());
    
    // 生成字段定义
    let field_definitions = fields.iter().map(|(name, field_type)| {
        let field_name = Ident::new(name, Span::call_site());
        match field_type {
            FieldType::Required(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! { pub #field_name: #ty }
            }
            FieldType::Optional(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! { pub #field_name: Option<#ty> }
            }
            FieldType::Collection(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! { pub #field_name: #ty }
            }
        }
    });
    
    // 生成构造函数参数
    let constructor_params = fields.iter().map(|(name, field_type)| {
        let field_name = Ident::new(name, Span::call_site());
        match field_type {
            FieldType::Required(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! { #field_name: #ty }
            }
            FieldType::Optional(_) => {
                quote! {} // 可选字段在构造函数中不是必需的
            }
            FieldType::Collection(_) => {
                quote! {} // 集合字段在构造函数中默认为空
            }
        }
    });
    
    // 生成字段初始化
    let field_initializers = fields.iter().map(|(name, field_type)| {
        let field_name = Ident::new(name, Span::call_site());
        match field_type {
            FieldType::Required(_) => {
                quote! { #field_name }
            }
            FieldType::Optional(_) => {
                quote! { #field_name: None }
            }
            FieldType::Collection(_) => {
                quote! { #field_name: Vec::new() }
            }
        }
    });
    
    // 生成访问器方法
    let accessor_methods = fields.iter().map(|(name, field_type)| {
        let field_name = Ident::new(name, Span::call_site());
        let getter_name = Ident::new(&format!("get_{}", name), Span::call_site());
        
        match field_type {
            FieldType::Required(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! {
                    pub fn #getter_name(&self) -> &#ty {
                        &self.#field_name
                    }
                }
            }
            FieldType::Optional(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                quote! {
                    pub fn #getter_name(&self) -> Option<&#ty> {
                        self.#field_name.as_ref()
                    }
                }
            }
            FieldType::Collection(ty_str) => {
                let ty: Type = syn::parse_str(ty_str).unwrap();
                let add_method = Ident::new(&format!("add_{}", name.trim_end_matches('s')), Span::call_site());
                quote! {
                    pub fn #getter_name(&self) -> &#ty {
                        &self.#field_name
                    }
                    
                    pub fn #add_method(&mut self, item: String) {
                        self.#field_name.push(item);
                    }
                }
            }
        }
    });
    
    let tokens = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#field_definitions,)*
        }
        
        impl #struct_name {
            pub fn new(#(#constructor_params,)*) -> Self {
                Self {
                    #(#field_initializers,)*
                }
            }
            
            #(#accessor_methods)*
        }
    };
    
    println!("Complex conditionals: {}", tokens);
}
```

### 特性条件生成

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

struct TraitConfig {
    debug: bool,
    clone: bool,
    serialize: bool,
    default: bool,
}

fn trait_conditional_generation(config: TraitConfig) {
    let struct_name = Ident::new("MyStruct", Span::call_site());
    
    // 构建 derive 属性
    let mut derives = Vec::new();
    if config.debug {
        derives.push(quote! { Debug });
    }
    if config.clone {
        derives.push(quote! { Clone });
    }
    if config.serialize {
        derives.push(quote! { serde::Serialize, serde::Deserialize });
    }
    
    let derive_attr = if !derives.is_empty() {
        quote! { #[derive(#(#derives),*)] }
    } else {
        quote! {}
    };
    
    // 生成 trait 实现
    let default_impl = if config.default {
        quote! {
            impl Default for #struct_name {
                fn default() -> Self {
                    Self {
                        value: 0,
                        name: String::new(),
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    
    // 条件方法
    let debug_methods = if config.debug {
        quote! {
            impl #struct_name {
                pub fn debug_info(&self) -> String {
                    format!("{:?}", self)
                }
            }
        }
    } else {
        quote! {}
    };
    
    let serialize_methods = if config.serialize {
        quote! {
            impl #struct_name {
                pub fn to_json(&self) -> Result<String, serde_json::Error> {
                    serde_json::to_string(self)
                }
                
                pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
                    serde_json::from_str(json)
                }
            }
        }
    } else {
        quote! {}
    };
    
    let tokens = quote! {
        #derive_attr
        pub struct #struct_name {
            pub value: i32,
            pub name: String,
        }
        
        #default_impl
        #debug_methods
        #serialize_methods
        
        impl #struct_name {
            pub fn new(value: i32, name: String) -> Self {
                Self { value, name }
            }
        }
    };
    
    println!("Trait conditional generation: {}", tokens);
}
```

## 类型和标识符

### 标识符操作

```rust
use quote::{quote, format_ident};
use syn::Ident;
use proc_macro2::Span;

fn identifier_operations() {
    // 创建标识符
    let base_name = "user";
    let getter = format_ident!("get_{}", base_name);
    let setter = format_ident!("set_{}", base_name);
    let field = format_ident!("{}_data", base_name);
    
    let tokens = quote! {
        struct UserManager {
            #field: String,
        }
        
        impl UserManager {
            pub fn #getter(&self) -> &str {
                &self.#field
            }
            
            pub fn #setter(&mut self, value: String) {
                self.#field = value;
            }
        }
    };
    
    // 处理关键字冲突
    let raw_ident = format_ident!("r#type");
    let tokens = quote! {
        struct Config {
            #raw_ident: String,
        }
    };
    
    // 复杂标识符生成
    let prefixes = vec!["is", "has", "can"];
    let properties = vec!["active", "admin", "write"];
    
    let methods = prefixes.iter().flat_map(|prefix| {
        properties.iter().map(move |prop| {
            let method_name = format_ident!("{}_{}", prefix, prop);
            quote! {
                pub fn #method_name(&self) -> bool {
                    true // 实际实现
                }
            }
        })
    });
    
    let tokens = quote! {
        impl UserManager {
            #(#methods)*
        }
    };
    
    println!("Identifier operations: {}", tokens);
}
```

### 类型操作

```rust
use quote::quote;
use syn::{Type, GenericParam, TypeParam, Generics};
use proc_macro2::Span;

fn type_operations() {
    // 基本类型
    let int_type: Type = syn::parse_quote!(i32);
    let string_type: Type = syn::parse_quote!(String);
    let option_type: Type = syn::parse_quote!(Option<String>);
    
    let tokens = quote! {
        struct TypedStruct {
            id: #int_type,
            name: #string_type,
            description: #option_type,
        }
    };
    
    // 泛型类型
    let generic_type: Type = syn::parse_quote!(T);
    let bounded_type: Type = syn::parse_quote!(T: Clone + Send);
    
    let tokens = quote! {
        struct GenericStruct<T> {
            data: #generic_type,
        }
        
        impl<T> GenericStruct<T> 
        where 
            #bounded_type
        {
            pub fn new(data: T) -> Self {
                Self { data }
            }
        }
    };
    
    // 复杂类型构造
    let key_type: Type = syn::parse_quote!(String);
    let value_type: Type = syn::parse_quote!(Vec<i32>);
    let map_type: Type = syn::parse_quote!(std::collections::HashMap<#key_type, #value_type>);
    
    let tokens = quote! {
        struct DataStore {
            storage: #map_type,
        }
        
        impl DataStore {
            pub fn new() -> Self {
                Self {
                    storage: std::collections::HashMap::new(),
                }
            }
        }
    };
    
    // 动态类型生成
    let type_configs = vec![
        ("UserId", "u32"),
        ("UserName", "String"),
        ("UserEmail", "Option<String>"),
    ];
    
    let type_aliases = type_configs.iter().map(|(alias, base_type)| {
        let alias_ident = syn::parse_str::<syn::Ident>(alias).unwrap();
        let base_type: Type = syn::parse_str(base_type).unwrap();
        
        quote! {
            pub type #alias_ident = #base_type;
        }
    });
    
    let tokens = quote! {
        #(#type_aliases)*
        
        pub struct User {
            id: UserId,
            name: UserName,
            email: UserEmail,
        }
    };
    
    println!("Type operations: {}", tokens);
}
```

### 路径和导入

```rust
use quote::quote;
use syn::{Path, UseTree};

fn path_and_imports() {
    // 创建路径
    let std_path: Path = syn::parse_quote!(std::collections::HashMap);
    let custom_path: Path = syn::parse_quote!(crate::models::User);
    
    let tokens = quote! {
        use #std_path;
        use #custom_path;
        
        type UserMap = HashMap<String, User>;
    };
    
    // 条件导入
    let features = vec!["serde", "async"];
    let conditional_imports = features.iter().map(|feature| {
        match *feature {
            "serde" => quote! {
                #[cfg(feature = "serde")]
                use serde::{Serialize, Deserialize};
            },
            "async" => quote! {
                #[cfg(feature = "async")]
                use tokio::runtime::Runtime;
            },
            _ => quote! {},
        }
    });
    
    let tokens = quote! {
        #(#conditional_imports)*
        
        #[cfg(feature = "serde")]
        #[derive(Serialize, Deserialize)]
        pub struct SerializableStruct {
            data: String,
        }
    };
    
    // 模块路径生成
    let modules = vec!["users", "posts", "comments"];
    let module_imports = modules.iter().map(|module| {
        let module_path: Path = syn::parse_str(&format!("crate::{}", module)).unwrap();
        quote! {
            pub use #module_path::*;
        }
    });
    
    let tokens = quote! {
        pub mod api {
            #(#module_imports)*
        }
    };
    
    println!("Path and imports: {}", tokens);
}
```

## 过程宏集成

### 派生宏实现

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = quote::format_ident!("{}Builder", name);
    
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("Builder only supports structs with named fields"),
    };
    
    // 生成 Builder 结构体字段
    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            #name: Option<#ty>
        }
    });
    
    // 生成设置方法
    let builder_methods = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    
    // 生成 build 方法
    let build_fields = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: self.#name.ok_or_else(|| format!("Field '{}' is not set", stringify!(#name)))?
        }
    });
    
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
        
        #[derive(Default)]
        pub struct #builder_name {
            #(#builder_fields,)*
        }
        
        impl #builder_name {
            #(#builder_methods)*
            
            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### 属性宏实现

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, Meta, Lit};

#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    // 解析参数
    let mut print_result = false;
    let mut unit = "ms".to_string();
    
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(meta)) if meta.path.is_ident("unit") => {
                if let Lit::Str(lit_str) = meta.lit {
                    unit = lit_str.value();
                }
            }
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("print_result") => {
                print_result = true;
            }
            _ => {}
        }
    }
    
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    
    let time_unit = match unit.as_str() {
        "ns" => quote! { as_nanos() },
        "us" => quote! { as_micros() },
        "ms" => quote! { as_millis() },
        "s" => quote! { as_secs() },
        _ => quote! { as_millis() },
    };
    
    let result_printing = if print_result {
        quote! {
            println!("Function {} returned: {:?}", stringify!(#fn_name), &result);
        }
    } else {
        quote! {}
    };
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed().#time_unit();
            
            println!("Function {} took: {}{}", stringify!(#fn_name), duration, #unit);
            #result_printing
            
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

### 函数式宏实现

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Expr, LitInt};

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let value = input.base10_parse::<i32>().unwrap();
    
    let expanded = quote! {
        {
            const ANSWER: i32 = #value;
            ANSWER
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let sql = input.value();
    
    // 简单的 SQL 解析
    let query_type = if sql.trim().to_lowercase().starts_with("select") {
        "SELECT"
    } else if sql.trim().to_lowercase().starts_with("insert") {
        "INSERT"
    } else if sql.trim().to_lowercase().starts_with("update") {
        "UPDATE"
    } else if sql.trim().to_lowercase().starts_with("delete") {
        "DELETE"
    } else {
        "UNKNOWN"
    };
    
    let expanded = quote! {
        {
            struct Query {
                sql: &'static str,
                query_type: &'static str,
            }
            
            impl Query {
                fn execute(&self) {
                    println!("Executing {} query: {}", self.query_type, self.sql);
                }
            }
            
            Query {
                sql: #sql,
                query_type: #query_type,
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn generate_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprArray);
    
    let variants = input.elems.iter().map(|elem| {
        if let Expr::Lit(expr_lit) = elem {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let variant_name = syn::parse_str::<syn::Ident>(&lit_str.value()).unwrap();
                quote! { #variant_name }
            } else {
                panic!("Expected string literal");
            }
        } else {
            panic!("Expected literal expression");
        }
    });
    
    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum GeneratedEnum {
            #(#variants,)*
        }
        
        impl GeneratedEnum {
            pub fn all_variants() -> Vec<Self> {
                vec![#(Self::#variants,)*]
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

## 高级技巧

### 代码模板系统

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;
use std::collections::HashMap;

struct CodeTemplate {
    template: proc_macro2::TokenStream,
    variables: HashMap<String, proc_macro2::TokenStream>,
}

impl CodeTemplate {
    fn new(template: proc_macro2::TokenStream) -> Self {
        Self {
            template,
            variables: HashMap::new(),
        }
    }
    
    fn set_variable(&mut self, name: &str, value: proc_macro2::TokenStream) {
        self.variables.insert(name.to_string(), value);
    }
    
    fn render(&self) -> proc_macro2::TokenStream {
        // 简化的模板渲染实现
        self.template.clone()
    }
}

fn template_system_example() {
    // 创建 CRUD 操作模板
    let crud_template = quote! {
        pub struct {{STRUCT_NAME}} {
            {{FIELDS}}
        }
        
        impl {{STRUCT_NAME}} {
            pub fn new({{CONSTRUCTOR_PARAMS}}) -> Self {
                Self {
                    {{FIELD_ASSIGNMENTS}}
                }
            }
            
            pub fn create(&self) -> Result<(), Error> {
                // 创建逻辑
                Ok(())
            }
            
            pub fn read(id: {{ID_TYPE}}) -> Result<Option<Self>, Error> {
                // 读取逻辑
                Ok(None)
            }
            
            pub fn update(&mut self) -> Result<(), Error> {
                // 更新逻辑
                Ok(())
            }
            
            pub fn delete(id: {{ID_TYPE}}) -> Result<(), Error> {
                // 删除逻辑
                Ok(())
            }
        }
    };
    
    // 应用模板生成不同的结构体
    let entities = vec![
        ("User", vec![("id", "u32"), ("name", "String"), ("email", "String")]),
        ("Post", vec![("id", "u32"), ("title", "String"), ("content", "String")]),
    ];
    
    let generated_code = entities.iter().map(|(entity_name, fields)| {
        let struct_name = Ident::new(entity_name, Span::call_site());
        let id_type = syn::parse_str::<Type>("u32").unwrap();
        
        let field_definitions = fields.iter().map(|(name, ty_str)| {
            let field_name = Ident::new(name, Span::call_site());
            let field_type: Type = syn::parse_str(ty_str).unwrap();
            quote! { pub #field_name: #field_type }
        });
        
        let constructor_params = fields.iter().map(|(name, ty_str)| {
            let field_name = Ident::new(name, Span::call_site());
            let field_type: Type = syn::parse_str(ty_str).unwrap();
            quote! { #field_name: #field_type }
        });
        
        let field_assignments = fields.iter().map(|(name, _)| {
            let field_name = Ident::new(name, Span::call_site());
            quote! { #field_name }
        });
        
        quote! {
            pub struct #struct_name {
                #(#field_definitions,)*
            }
            
            impl #struct_name {
                pub fn new(#(#constructor_params,)*) -> Self {
                    Self {
                        #(#field_assignments,)*
                    }
                }
                
                pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
                    println!("Creating {}", stringify!(#struct_name));
                    Ok(())
                }
                
                pub fn read(id: #id_type) -> Result<Option<Self>, Box<dyn std::error::Error>> {
                    println!("Reading {} with id: {}", stringify!(#struct_name), id);
                    Ok(None)
                }
                
                pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                    println!("Updating {}", stringify!(#struct_name));
                    Ok(())
                }
                
                pub fn delete(id: #id_type) -> Result<(), Box<dyn std::error::Error>> {
                    println!("Deleting {} with id: {}", stringify!(#struct_name), id);
                    Ok(())
                }
            }
        }
    });
    
    let final_code = quote! {
        #(#generated_code)*
    };
    
    println!("Template system: {}", final_code);
}
```

### 动态特性实现

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

trait CodeGenerator {
    fn generate_struct(&self, name: &str, fields: &[(String, String)]) -> proc_macro2::TokenStream;
    fn generate_impl(&self, name: &str) -> proc_macro2::TokenStream;
}

struct BasicGenerator;
struct SerializableGenerator;
struct AsyncGenerator;

impl CodeGenerator for BasicGenerator {
    fn generate_struct(&self, name: &str, fields: &[(String, String)]) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        let field_defs = fields.iter().map(|(name, ty_str)| {
            let field_name = Ident::new(name, Span::call_site());
            let field_type: Type = syn::parse_str(ty_str).unwrap();
            quote! { pub #field_name: #field_type }
        });
        
        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #(#field_defs,)*
            }
        }
    }
    
    fn generate_impl(&self, name: &str) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        quote! {
            impl #struct_name {
                pub fn new() -> Self {
                    Default::default()
                }
            }
        }
    }
}

impl CodeGenerator for SerializableGenerator {
    fn generate_struct(&self, name: &str, fields: &[(String, String)]) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        let field_defs = fields.iter().map(|(name, ty_str)| {
            let field_name = Ident::new(name, Span::call_site());
            let field_type: Type = syn::parse_str(ty_str).unwrap();
            quote! { pub #field_name: #field_type }
        });
        
        quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct #struct_name {
                #(#field_defs,)*
            }
        }
    }
    
    fn generate_impl(&self, name: &str) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        quote! {
            impl #struct_name {
                pub fn to_json(&self) -> Result<String, serde_json::Error> {
                    serde_json::to_string(self)
                }
                
                pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
                    serde_json::from_str(json)
                }
            }
        }
    }
}

impl CodeGenerator for AsyncGenerator {
    fn generate_struct(&self, name: &str, fields: &[(String, String)]) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        let field_defs = fields.iter().map(|(name, ty_str)| {
            let field_name = Ident::new(name, Span::call_site());
            let field_type: Type = syn::parse_str(ty_str).unwrap();
            quote! { pub #field_name: #field_type }
        });
        
        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #(#field_defs,)*
            }
        }
    }
    
    fn generate_impl(&self, name: &str) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(name, Span::call_site());
        quote! {
            impl #struct_name {
                pub async fn async_new() -> Self {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    Default::default()
                }
                
                pub async fn async_process(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                    println!("Processing {} asynchronously", stringify!(#struct_name));
                    Ok(())
                }
            }
        }
    }
}

fn dynamic_generation_example() {
    let generators: Vec<Box<dyn CodeGenerator>> = vec![
        Box::new(BasicGenerator),
        Box::new(SerializableGenerator),
        Box::new(AsyncGenerator),
    ];
    
    let entity_config = ("User", vec![
        ("id".to_string(), "u32".to_string()),
        ("name".to_string(), "String".to_string()),
    ]);
    
    let generated_variants = generators.iter().enumerate().map(|(i, generator)| {
        let variant_name = format!("{}V{}", entity_config.0, i);
        let struct_def = generator.generate_struct(&variant_name, &entity_config.1);
        let impl_def = generator.generate_impl(&variant_name);
        
        quote! {
            #struct_def
            #impl_def
        }
    });
    
    let final_code = quote! {
        #(#generated_variants)*
    };
    
    println!("Dynamic generation: {}", final_code);
}
```

### 元编程模式

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

// 定义元数据结构
struct EntityMetadata {
    name: String,
    fields: Vec<FieldMetadata>,
    traits: Vec<String>,
    methods: Vec<MethodMetadata>,
}

struct FieldMetadata {
    name: String,
    field_type: String,
    optional: bool,
    default_value: Option<String>,
}

struct MethodMetadata {
    name: String,
    params: Vec<(String, String)>,
    return_type: String,
    is_async: bool,
}

impl EntityMetadata {
    fn generate_code(&self) -> proc_macro2::TokenStream {
        let struct_code = self.generate_struct();
        let impl_code = self.generate_impl();
        let trait_impls = self.generate_trait_impls();
        
        quote! {
            #struct_code
            #impl_code
            #(#trait_impls)*
        }
    }
    
    fn generate_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(&self.name, Span::call_site());
        
        let derives = if self.traits.contains(&"Serialize".to_string()) {
            quote! { #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] }
        } else {
            quote! { #[derive(Debug, Clone)] }
        };
        
        let fields = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type).unwrap();
            
            if field.optional {
                quote! { pub #field_name: Option<#field_type> }
            } else {
                quote! { pub #field_name: #field_type }
            }
        });
        
        quote! {
            #derives
            pub struct #struct_name {
                #(#fields,)*
            }
        }
    }
    
    fn generate_impl(&self) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(&self.name, Span::call_site());
        let constructor = self.generate_constructor();
        let methods = self.generate_methods();
        
        quote! {
            impl #struct_name {
                #constructor
                #(#methods)*
            }
        }
    }
    
    fn generate_constructor(&self) -> proc_macro2::TokenStream {
        let required_fields: Vec<_> = self.fields.iter()
            .filter(|f| !f.optional)
            .collect();
        
        let params = required_fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type).unwrap();
            quote! { #field_name: #field_type }
        });
        
        let field_assignments = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            
            if field.optional {
                if let Some(default) = &field.default_value {
                    let default_expr: syn::Expr = syn::parse_str(default).unwrap();
                    quote! { #field_name: Some(#default_expr) }
                } else {
                    quote! { #field_name: None }
                }
            } else {
                quote! { #field_name }
            }
        });
        
        quote! {
            pub fn new(#(#params,)*) -> Self {
                Self {
                    #(#field_assignments,)*
                }
            }
        }
    }
    
    fn generate_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter().map(|method| {
            let method_name = Ident::new(&method.name, Span::call_site());
            let return_type: Type = syn::parse_str(&method.return_type).unwrap();
            
            let params = method.params.iter().map(|(name, ty)| {
                let param_name = Ident::new(name, Span::call_site());
                let param_type: Type = syn::parse_str(ty).unwrap();
                quote! { #param_name: #param_type }
            });
            
            if method.is_async {
                quote! {
                    pub async fn #method_name(&self, #(#params,)*) -> #return_type {
                        // 异步方法实现
                        Default::default()
                    }
                }
            } else {
                quote! {
                    pub fn #method_name(&self, #(#params,)*) -> #return_type {
                        // 同步方法实现
                        Default::default()
                    }
                }
            }
        }).collect()
    }
    
    fn generate_trait_impls(&self) -> Vec<proc_macro2::TokenStream> {
        let struct_name = Ident::new(&self.name, Span::call_site());
        let mut impls = Vec::new();
        
        if self.traits.contains(&"Default".to_string()) {
            let default_assignments = self.fields.iter().map(|field| {
                let field_name = Ident::new(&field.name, Span::call_site());
                
                if field.optional {
                    quote! { #field_name: None }
                } else if let Some(default) = &field.default_value {
                    let default_expr: syn::Expr = syn::parse_str(default).unwrap();
                    quote! { #field_name: #default_expr }
                } else {
                    quote! { #field_name: Default::default() }
                }
            });
            
            impls.push(quote! {
                impl Default for #struct_name {
                    fn default() -> Self {
                        Self {
                            #(#default_assignments,)*
                        }
                    }
                }
            });
        }
        
        impls
    }
}

fn meta_programming_example() {
    let user_metadata = EntityMetadata {
        name: "User".to_string(),
        fields: vec![
            FieldMetadata {
                name: "id".to_string(),
                field_type: "u32".to_string(),
                optional: false,
                default_value: None,
            },
            FieldMetadata {
                name: "name".to_string(),
                field_type: "String".to_string(),
                optional: false,
                default_value: None,
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: "String".to_string(),
                optional: true,
                default_value: None,
            },
        ],
        traits: vec!["Default".to_string(), "Serialize".to_string()],
        methods: vec![
            MethodMetadata {
                name: "get_display_name".to_string(),
                params: vec![],
                return_type: "String".to_string(),
                is_async: false,
            },
            MethodMetadata {
                name: "save".to_string(),
                params: vec![],
                return_type: "Result<(), String>".to_string(),
                is_async: true,
            },
        ],
    };
    
    let generated_code = user_metadata.generate_code();
    println!("Meta programming: {}", generated_code);
}
```

## 错误处理

### Quote 错误类型

```rust
use quote::quote;
use syn::{Error, Result, Ident, spanned::Spanned};
use proc_macro2::Span;

fn error_handling_examples() {
    // 创建语法错误
    let invalid_ident = "123invalid";
    match syn::parse_str::<Ident>(invalid_ident) {
        Ok(ident) => {
            let tokens = quote! { let #ident = 42; };
            println!("Valid: {}", tokens);
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
    
    // 创建带位置信息的错误
    let span = Span::call_site();
    let error = Error::new(span, "自定义错误消息");
    println!("Error: {}", error);
    
    // 组合多个错误
    let mut combined_error = Error::new(span, "主要错误");
    combined_error.combine(Error::new(span, "次要错误1"));
    combined_error.combine(Error::new(span, "次要错误2"));
    println!("Combined error: {}", combined_error);
}

fn safe_quote_generation(name: &str) -> Result<proc_macro2::TokenStream> {
    // 验证输入
    if name.is_empty() {
        return Err(Error::new(Span::call_site(), "名称不能为空"));
    }
    
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::new(Span::call_site(), "名称包含无效字符"));
    }
    
    if name.chars().next().unwrap().is_numeric() {
        return Err(Error::new(Span::call_site(), "名称不能以数字开头"));
    }
    
    let ident = Ident::new(name, Span::call_site());
    Ok(quote! {
        pub struct #ident {
            data: String,
        }
        
        impl #ident {
            pub fn new(data: String) -> Self {
                Self { data }
            }
        }
    })
}
```

### 编译时验证

```rust
use quote::quote;
use syn::{Ident, Type, Error, Result, parse_str};
use proc_macro2::Span;

fn compile_time_validation() -> Result<proc_macro2::TokenStream> {
    let fields = vec![
        ("id", "u32"),
        ("name", "String"),
        ("invalid_type", "NonExistentType"),
    ];
    
    // 验证字段类型
    let mut validated_fields = Vec::new();
    for (field_name, type_str) in fields {
        let field_ident = Ident::new(field_name, Span::call_site());
        
        // 尝试解析类型
        match parse_str::<Type>(type_str) {
            Ok(field_type) => {
                validated_fields.push((field_ident, field_type));
            }
            Err(e) => {
                return Err(Error::new(
                    Span::call_site(),
                    format!("字段 '{}' 的类型 '{}' 无效: {}", field_name, type_str, e)
                ));
            }
        }
    }
    
    let field_definitions = validated_fields.iter().map(|(name, ty)| {
        quote! { pub #name: #ty }
    });
    
    Ok(quote! {
        pub struct ValidatedStruct {
            #(#field_definitions,)*
        }
    })
}

fn validate_identifier_conflicts(names: &[&str]) -> Result<proc_macro2::TokenStream> {
    let mut seen = std::collections::HashSet::new();
    let mut validated_idents = Vec::new();
    
    for name in names {
        if seen.contains(name) {
            return Err(Error::new(
                Span::call_site(),
                format!("重复的标识符: '{}'", name)
            ));
        }
        
        // 检查是否为保留关键字
        if matches!(*name, "type" | "impl" | "fn" | "struct" | "enum") {
            return Err(Error::new(
                Span::call_site(),
                format!("'{}' 是保留关键字", name)
            ));
        }
        
        seen.insert(name);
        validated_idents.push(Ident::new(name, Span::call_site()));
    }
    
    let method_definitions = validated_idents.iter().map(|ident| {
        quote! {
            pub fn #ident(&self) -> String {
                stringify!(#ident).to_string()
            }
        }
    });
    
    Ok(quote! {
        impl GeneratedMethods {
            #(#method_definitions)*
        }
    })
}
```

### 错误恢复策略

```rust
use quote::quote;
use syn::{Error, Result, Ident, Type};
use proc_macro2::Span;

fn error_recovery_example(fields: Vec<(&str, &str)>) -> proc_macro2::TokenStream {
    let mut valid_fields = Vec::new();
    let mut errors = Vec::new();
    
    // 收集所有错误而不是立即失败
    for (field_name, type_str) in fields {
        match Ident::new(field_name, Span::call_site()) {
            ident => {
                match syn::parse_str::<Type>(type_str) {
                    Ok(field_type) => {
                        valid_fields.push((ident, field_type));
                    }
                    Err(e) => {
                        errors.push(format!("字段 '{}': {}", field_name, e));
                        // 使用默认类型作为回退
                        let default_type: Type = syn::parse_quote!(String);
                        valid_fields.push((ident, default_type));
                    }
                }
            }
        }
    }
    
    // 生成警告注释
    let error_comments = if !errors.is_empty() {
        let error_list = errors.join(", ");
        quote! {
            // 警告: 以下错误已使用默认类型修复:
            // #error_list
        }
    } else {
        quote! {}
    };
    
    let field_definitions = valid_fields.iter().map(|(name, ty)| {
        quote! { pub #name: #ty }
    });
    
    quote! {
        #error_comments
        pub struct RecoveredStruct {
            #(#field_definitions,)*
        }
    }
}

fn graceful_degradation_example(
    features: Vec<&str>
) -> proc_macro2::TokenStream {
    let mut available_features = Vec::new();
    let mut feature_impls = Vec::new();
    
    for feature in features {
        match feature {
            "serialize" => {
                available_features.push("serialize");
                feature_impls.push(quote! {
                    #[cfg(feature = "serde")]
                    impl serde::Serialize for MyStruct {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            // 序列化实现
                            serializer.serialize_str("placeholder")
                        }
                    }
                });
            }
            "async" => {
                available_features.push("async");
                feature_impls.push(quote! {
                    #[cfg(feature = "async")]
                    impl MyStruct {
                        pub async fn async_method(&self) -> Result<(), Box<dyn std::error::Error>> {
                            Ok(())
                        }
                    }
                });
            }
            unknown => {
                // 未知特性 - 生成注释但不失败
                feature_impls.push(quote! {
                    // 跳过未知特性: #unknown
                });
            }
        }
    }
    
    let feature_list = available_features.join(", ");
    
    quote! {
        // 支持的特性: #feature_list
        pub struct MyStruct {
            data: String,
        }
        
        #(#feature_impls)*
    }
}
```

## 性能优化

### 代码生成优化

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;
use std::collections::HashMap;

// 缓存常用的标识符
lazy_static::lazy_static! {
    static ref COMMON_IDENTS: HashMap<&'static str, Ident> = {
        let mut map = HashMap::new();
        map.insert("new", Ident::new("new", Span::call_site()));
        map.insert("default", Ident::new("default", Span::call_site()));
        map.insert("clone", Ident::new("clone", Span::call_site()));
        map.insert("debug", Ident::new("debug", Span::call_site()));
        map
    };
}

fn get_cached_ident(name: &str) -> Ident {
    COMMON_IDENTS.get(name)
        .cloned()
        .unwrap_or_else(|| Ident::new(name, Span::call_site()))
}

fn optimized_generation(count: usize) -> proc_macro2::TokenStream {
    // 预分配容量
    let mut method_definitions = Vec::with_capacity(count);
    
    // 批量生成
    for i in 0..count {
        let method_name = Ident::new(&format!("method_{}", i), Span::call_site());
        method_definitions.push(quote! {
            pub fn #method_name(&self) -> i32 {
                #i
            }
        });
    }
    
    quote! {
        impl GeneratedMethods {
            #(#method_definitions)*
        }
    }
}

// 重用 TokenStream 模式
fn reusable_patterns() -> proc_macro2::TokenStream {
    let common_pattern = quote! {
        pub fn common_method(&self) -> Result<(), Box<dyn std::error::Error>>
    };
    
    let methods = (0..10).map(|i| {
        let method_name = Ident::new(&format!("method_{}", i), Span::call_site());
        quote! {
            #common_pattern {
                println!("Method {}", #i);
                Ok(())
            }
        }
    });
    
    quote! {
        impl ReusablePatterns {
            #(#methods)*
        }
    }
}
```

### 内存优化

```rust
use quote::quote;
use proc_macro2::TokenStream;

// 避免不必要的克隆
fn memory_efficient_generation(items: &[&str]) -> TokenStream {
    let definitions = items.iter().map(|&item| {
        let ident = syn::parse_str::<syn::Ident>(item).unwrap();
        quote! {
            pub fn #ident(&self) -> &str {
                #item
            }
        }
    });
    
    quote! {
        impl MemoryEfficient {
            #(#definitions)*
        }
    }
}

// 流式处理大量数据
fn streaming_generation<I>(items: I) -> TokenStream 
where 
    I: Iterator<Item = String>
{
    let mut result = quote! {
        impl StreamingGeneration {
    };
    
    for item in items {
        let ident = syn::parse_str::<syn::Ident>(&item).unwrap();
        let method = quote! {
            pub fn #ident(&self) -> String {
                #item.to_string()
            }
        };
        result.extend(method);
    }
    
    result.extend(quote! { } });
    result
}
```

### 编译时优化

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;

// 编译时计算
const fn calculate_method_count(base: usize) -> usize {
    base * 2 + 1
}

fn compile_time_optimized() -> proc_macro2::TokenStream {
    const METHOD_COUNT: usize = calculate_method_count(10);
    
    let methods = (0..METHOD_COUNT).map(|i| {
        let method_name = Ident::new(&format!("computed_method_{}", i), Span::call_site());
        let computed_value = i * i; // 编译时计算
        
        quote! {
            pub const fn #method_name() -> usize {
                #computed_value
            }
        }
    });
    
    quote! {
        impl ComputedMethods {
            #(#methods)*
            
            pub const METHOD_COUNT: usize = #METHOD_COUNT;
        }
    }
}

// 避免重复解析
fn parse_once_use_many() -> proc_macro2::TokenStream {
    let base_types = [
        ("String", quote!(String)),
        ("i32", quote!(i32)),
        ("Vec<String>", quote!(Vec<String>)),
    ];
    
    let field_definitions = base_types.iter().enumerate().map(|(i, (type_name, type_tokens))| {
        let field_name = Ident::new(&format!("field_{}", i), Span::call_site());
        quote! {
            pub #field_name: #type_tokens,
        }
    });
    
    let getter_methods = base_types.iter().enumerate().map(|(i, (type_name, type_tokens))| {
        let field_name = Ident::new(&format!("field_{}", i), Span::call_site());
        let getter_name = Ident::new(&format!("get_field_{}", i), Span::call_site());
        
        quote! {
            pub fn #getter_name(&self) -> &#type_tokens {
                &self.#field_name
            }
        }
    });
    
    quote! {
        pub struct OptimizedStruct {
            #(#field_definitions)*
        }
        
        impl OptimizedStruct {
            #(#getter_methods)*
        }
    }
}
```

## 实战案例

### 配置管理器生成

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;
use std::collections::HashMap;

#[derive(Debug)]
struct ConfigField {
    name: String,
    field_type: String,
    default_value: Option<String>,
    env_var: Option<String>,
    required: bool,
}

struct ConfigGenerator {
    struct_name: String,
    fields: Vec<ConfigField>,
}

impl ConfigGenerator {
    fn new(struct_name: String) -> Self {
        Self {
            struct_name,
            fields: Vec::new(),
        }
    }
    
    fn add_field(mut self, field: ConfigField) -> Self {
        self.fields.push(field);
        self
    }
    
    fn generate(&self) -> proc_macro2::TokenStream {
        let struct_def = self.generate_struct();
        let impl_def = self.generate_impl();
        let builder_def = self.generate_builder();
        
        quote! {
            #struct_def
            #impl_def
            #builder_def
        }
    }
    
    fn generate_struct(&self) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(&self.struct_name, Span::call_site());
        
        let field_definitions = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type).unwrap();
            
            if field.required {
                quote! { pub #field_name: #field_type }
            } else {
                quote! { pub #field_name: Option<#field_type> }
            }
        });
        
        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #(#field_definitions,)*
            }
        }
    }
    
    fn generate_impl(&self) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(&self.struct_name, Span::call_site());
        let from_env_method = self.generate_from_env_method();
        let validate_method = self.generate_validate_method();
        
        quote! {
            impl #struct_name {
                #from_env_method
                #validate_method
            }
        }
    }
    
    fn generate_from_env_method(&self) -> proc_macro2::TokenStream {
        let field_assignments = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            
            if let Some(env_var) = &field.env_var {
                if field.required {
                    quote! {
                        #field_name: std::env::var(#env_var)
                            .map_err(|_| format!("Missing required environment variable: {}", #env_var))?
                            .parse()
                            .map_err(|_| format!("Invalid value for {}", #env_var))?
                    }
                } else {
                    quote! {
                        #field_name: std::env::var(#env_var)
                            .ok()
                            .and_then(|s| s.parse().ok())
                    }
                }
            } else if let Some(default) = &field.default_value {
                let default_expr: syn::Expr = syn::parse_str(default).unwrap();
                quote! { #field_name: #default_expr }
            } else if field.required {
                quote! {
                    #field_name: return Err(format!("No default value for required field: {}", stringify!(#field_name)))
                }
            } else {
                quote! { #field_name: None }
            }
        });
        
        quote! {
            pub fn from_env() -> Result<Self, String> {
                Ok(Self {
                    #(#field_assignments,)*
                })
            }
        }
    }
    
    fn generate_validate_method(&self) -> proc_macro2::TokenStream {
        let validations = self.fields.iter().filter_map(|field| {
            if field.required {
                let field_name = Ident::new(&field.name, Span::call_site());
                Some(quote! {
                    if self.#field_name.is_none() {
                        errors.push(format!("Required field '{}' is missing", stringify!(#field_name)));
                    }
                })
            } else {
                None
            }
        });
        
        quote! {
            pub fn validate(&self) -> Result<(), Vec<String>> {
                let mut errors = Vec::new();
                #(#validations)*
                
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    }
    
    fn generate_builder(&self) -> proc_macro2::TokenStream {
        let struct_name = Ident::new(&self.struct_name, Span::call_site());
        let builder_name = Ident::new(&format!("{}Builder", self.struct_name), Span::call_site());
        
        let builder_fields = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type).unwrap();
            quote! { #field_name: Option<#field_type> }
        });
        
        let builder_methods = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type).unwrap();
            
            quote! {
                pub fn #field_name(mut self, value: #field_type) -> Self {
                    self.#field_name = Some(value);
                    self
                }
            }
        });
        
        let build_assignments = self.fields.iter().map(|field| {
            let field_name = Ident::new(&field.name, Span::call_site());
            
            if field.required {
                quote! {
                    #field_name: self.#field_name.ok_or_else(|| format!("Required field '{}' not set", stringify!(#field_name)))?
                }
            } else {
                quote! { #field_name: self.#field_name }
            }
        });
        
        quote! {
            #[derive(Default)]
            pub struct #builder_name {
                #(#builder_fields,)*
            }
            
            impl #builder_name {
                #(#builder_methods)*
                
                pub fn build(self) -> Result<#struct_name, String> {
                    Ok(#struct_name {
                        #(#build_assignments,)*
                    })
                }
            }
            
            impl #struct_name {
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }
        }
    }
}

fn config_generator_example() {
    let config = ConfigGenerator::new("DatabaseConfig".to_string())
        .add_field(ConfigField {
            name: "host".to_string(),
            field_type: "String".to_string(),
            default_value: Some("\"localhost\".to_string()".to_string()),
            env_var: Some("DB_HOST".to_string()),
            required: true,
        })
        .add_field(ConfigField {
            name: "port".to_string(),
            field_type: "u16".to_string(),
            default_value: Some("5432".to_string()),
            env_var: Some("DB_PORT".to_string()),
            required: true,
        })
        .add_field(ConfigField {
            name: "username".to_string(),
            field_type: "String".to_string(),
            default_value: None,
            env_var: Some("DB_USERNAME".to_string()),
            required: true,
        })
        .add_field(ConfigField {
            name: "password".to_string(),
            field_type: "String".to_string(),
            default_value: None,
            env_var: Some("DB_PASSWORD".to_string()),
            required: false,
        });
    
    let generated_code = config.generate();
    println!("Config generator: {}", generated_code);
}
```

### API 客户端生成器

```rust
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::Span;

#[derive(Debug)]
struct ApiEndpoint {
    name: String,
    method: String,
    path: String,
    request_type: Option<String>,
    response_type: String,
    auth_required: bool,
}

struct ApiClientGenerator {
    client_name: String,
    base_url: String,
    endpoints: Vec<ApiEndpoint>,
}

impl ApiClientGenerator {
    fn new(client_name: String, base_url: String) -> Self {
        Self {
            client_name,
            base_url,
            endpoints: Vec::new(),
        }
    }
    
    fn add_endpoint(mut self, endpoint: ApiEndpoint) -> Self {
        self.endpoints.push(endpoint);
        self
    }
    
    fn generate(&self) -> proc_macro2::TokenStream {
        let struct_def = self.generate_struct();
        let impl_def = self.generate_impl();
        
        quote! {
            #struct_def
            #impl_def
        }
    }
    
    fn generate_struct(&self) -> proc_macro2::TokenStream {
        let client_name = Ident::new(&self.client_name, Span::call_site());
        
        quote! {
            #[derive(Debug, Clone)]
            pub struct #client_name {
                base_url: String,
                client: reqwest::Client,
                auth_token: Option<String>,
            }
        }
    }
    
    fn generate_impl(&self) -> proc_macro2::TokenStream {
        let client_name = Ident::new(&self.client_name, Span::call_site());
        let base_url = &self.base_url;
        let constructor = self.generate_constructor();
        let endpoint_methods = self.generate_endpoint_methods();
        
        quote! {
            impl #client_name {
                #constructor
                
                pub fn with_auth(mut self, token: String) -> Self {
                    self.auth_token = Some(token);
                    self
                }
                
                #(#endpoint_methods)*
            }
        }
    }
    
    fn generate_constructor(&self) -> proc_macro2::TokenStream {
        let base_url = &self.base_url;
        
        quote! {
            pub fn new() -> Self {
                Self {
                    base_url: #base_url.to_string(),
                    client: reqwest::Client::new(),
                    auth_token: None,
                }
            }
        }
    }
    
    fn generate_endpoint_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.endpoints.iter().map(|endpoint| {
            let method_name = Ident::new(&endpoint.name, Span::call_site());
            let path = &endpoint.path;
            let response_type: Type = syn::parse_str(&endpoint.response_type).unwrap();
            
            let request_param = if let Some(req_type) = &endpoint.request_type {
                let request_type: Type = syn::parse_str(req_type).unwrap();
                quote! { request: #request_type, }
            } else {
                quote! {}
            };
            
            let auth_header = if endpoint.auth_required {
                quote! {
                    if let Some(token) = &self.auth_token {
                        request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
                    } else {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::PermissionDenied,
                            "Authentication required but no token provided"
                        )));
                    }
                }
            } else {
                quote! {}
            };
            
            let request_body = if endpoint.request_type.is_some() {
                match endpoint.method.as_str() {
                    "GET" | "DELETE" => quote! {},
                    _ => quote! {
                        request_builder = request_builder.json(&request);
                    }
                }
            } else {
                quote! {}
            };
            
            let http_method = match endpoint.method.as_str() {
                "GET" => quote! { get },
                "POST" => quote! { post },
                "PUT" => quote! { put },
                "DELETE" => quote! { delete },
                "PATCH" => quote! { patch },
                _ => quote! { get },
            };
            
            quote! {
                pub async fn #method_name(
                    &self,
                    #request_param
                ) -> Result<#response_type, Box<dyn std::error::Error + Send + Sync>> {
                    let url = format!("{}{}", self.base_url, #path);
                    let mut request_builder = self.client.#http_method(&url);
                    
                    #auth_header
                    #request_body
                    
                    let response = request_builder.send().await?;
                    
                    if response.status().is_success() {
                        let result = response.json::<#response_type>().await?;
                        Ok(result)
                    } else {
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("HTTP error: {}", response.status())
                        )))
                    }
                }
            }
        }).collect()
    }
}

fn api_client_example() {
    let client = ApiClientGenerator::new(
        "UserApiClient".to_string(),
        "https://api.example.com".to_string()
    )
    .add_endpoint(ApiEndpoint {
        name: "get_user".to_string(),
        method: "GET".to_string(),
        path: "/users/{id}".to_string(),
        request_type: None,
        response_type: "User".to_string(),
        auth_required: true,
    })
    .add_endpoint(ApiEndpoint {
        name: "create_user".to_string(),
        method: "POST".to_string(),
        path: "/users".to_string(),
        request_type: Some("CreateUserRequest".to_string()),
        response_type: "User".to_string(),
        auth_required: true,
    })
    .add_endpoint(ApiEndpoint {
        name: "list_users".to_string(),
        method: "GET".to_string(),
        path: "/users".to_string(),
        request_type: None,
        response_type: "Vec<User>".to_string(),
        auth_required: false,
    });
    
    let generated_code = client.generate();
    println!("API client: {}", generated_code);
}
```

### 状态机生成器

```rust
use quote::quote;
use syn::Ident;
use proc_macro2::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct State {
    name: String,
    data: Option<String>,
}

#[derive(Debug, Clone)]
struct Transition {
    from: String,
    to: String,
    trigger: String,
    guard: Option<String>,
}

struct StateMachineGenerator {
    name: String,
    states: Vec<State>,
    transitions: Vec<Transition>,
    initial_state: String,
}

impl StateMachineGenerator {
    fn new(name: String, initial_state: String) -> Self {
        Self {
            name,
            states: Vec::new(),
            transitions: Vec::new(),
            initial_state,
        }
    }
    
    fn add_state(mut self, state: State) -> Self {
        self.states.push(state);
        self
    }
    
    fn add_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }
    
    fn generate(&self) -> proc_macro2::TokenStream {
        let state_enum = self.generate_state_enum();
        let event_enum = self.generate_event_enum();
        let machine_struct = self.generate_machine_struct();
        let machine_impl = self.generate_machine_impl();
        
        quote! {
            #state_enum
            #event_enum
            #machine_struct
            #machine_impl
        }
    }
    
    fn generate_state_enum(&self) -> proc_macro2::TokenStream {
        let state_variants = self.states.iter().map(|state| {
            let state_name = Ident::new(&state.name, Span::call_site());
            
            if let Some(data_type) = &state.data {
                let data_type: syn::Type = syn::parse_str(data_type).unwrap();
                quote! { #state_name(#data_type) }
            } else {
                quote! { #state_name }
            }
        });
        
        quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum State {
                #(#state_variants,)*
            }
        }
    }
    
    fn generate_event_enum(&self) -> proc_macro2::TokenStream {
        let events: std::collections::HashSet<_> = self.transitions.iter()
            .map(|t| &t.trigger)
            .collect();
        
        let event_variants = events.iter().map(|event| {
            let event_name = Ident::new(event, Span::call_site());
            quote! { #event_name }
        });
        
        quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum Event {
                #(#event_variants,)*
            }
        }
    }
    
    fn generate_machine_struct(&self) -> proc_macro2::TokenStream {
        let machine_name = Ident::new(&self.name, Span::call_site());
        
        quote! {
            #[derive(Debug, Clone)]
            pub struct #machine_name {
                current_state: State,
            }
        }
    }
    
    fn generate_machine_impl(&self) -> proc_macro2::TokenStream {
        let machine_name = Ident::new(&self.name, Span::call_site());
        let constructor = self.generate_constructor();
        let transition_method = self.generate_transition_method();
        let state_predicates = self.generate_state_predicates();
        
        quote! {
            impl #machine_name {
                #constructor
                #transition_method
                #(#state_predicates)*
                
                pub fn current_state(&self) -> &State {
                    &self.current_state
                }
            }
        }
    }
    
    fn generate_constructor(&self) -> proc_macro2::TokenStream {
        let initial_state = Ident::new(&self.initial_state, Span::call_site());
        
        quote! {
            pub fn new() -> Self {
                Self {
                    current_state: State::#initial_state,
                }
            }
        }
    }
    
    fn generate_transition_method(&self) -> proc_macro2::TokenStream {
        // 按状态分组转换
        let mut transitions_by_state: HashMap<String, Vec<&Transition>> = HashMap::new();
        for transition in &self.transitions {
            transitions_by_state
                .entry(transition.from.clone())
                .or_insert_with(Vec::new)
                .push(transition);
        }
        
        let state_matches = transitions_by_state.iter().map(|(from_state, transitions)| {
            let from_state_ident = Ident::new(from_state, Span::call_site());
            
            let event_matches = transitions.iter().map(|transition| {
                let event_ident = Ident::new(&transition.trigger, Span::call_site());
                let to_state_ident = Ident::new(&transition.to, Span::call_site());
                
                if let Some(guard) = &transition.guard {
                    let guard_expr: syn::Expr = syn::parse_str(guard).unwrap();
                    quote! {
                        Event::#event_ident if #guard_expr => {
                            self.current_state = State::#to_state_ident;
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        Event::#event_ident => {
                            self.current_state = State::#to_state_ident;
                            Ok(())
                        }
                    }
                }
            });
            
            quote! {
                State::#from_state_ident => {
                    match event {
                        #(#event_matches)*
                        _ => Err(format!("Invalid transition from {:?} with event {:?}", self.current_state, event))
                    }
                }
            }
        });
        
        quote! {
            pub fn transition(&mut self, event: Event) -> Result<(), String> {
                match &self.current_state {
                    #(#state_matches)*
                    _ => Err(format!("Unknown state: {:?}", self.current_state))
                }
            }
        }
    }
    
    fn generate_state_predicates(&self) -> Vec<proc_macro2::TokenStream> {
        self.states.iter().map(|state| {
            let state_name = Ident::new(&state.name, Span::call_site());
            let predicate_name = Ident::new(&format!("is_{}", state.name.to_lowercase()), Span::call_site());
            
            quote! {
                pub fn #predicate_name(&self) -> bool {
                    matches!(self.current_state, State::#state_name)
                }
            }
        }).collect()
    }
}

fn state_machine_example() {
    let machine = StateMachineGenerator::new(
        "OrderStateMachine".to_string(),
        "Pending".to_string()
    )
    .add_state(State {
        name: "Pending".to_string(),
        data: None,
    })
    .add_state(State {
        name: "Processing".to_string(),
        data: Some("String".to_string()),
    })
    .add_state(State {
        name: "Completed".to_string(),
        data: None,
    })
    .add_state(State {
        name: "Cancelled".to_string(),
        data: Some("String".to_string()),
    })
    .add_transition(Transition {
        from: "Pending".to_string(),
        to: "Processing".to_string(),
        trigger: "Process".to_string(),
        guard: None,
    })
    .add_transition(Transition {
        from: "Processing".to_string(),
        to: "Completed".to_string(),
        trigger: "Complete".to_string(),
        guard: None,
    })
    .add_transition(Transition {
        from: "Pending".to_string(),
        to: "Cancelled".to_string(),
        trigger: "Cancel".to_string(),
        guard: None,
    })
    .add_transition(Transition {
        from: "Processing".to_string(),
        to: "Cancelled".to_string(),
        trigger: "Cancel".to_string(),
        guard: Some("self.can_cancel()".to_string()),
    });
    
    let generated_code = machine.generate();
    println!("State machine: {}", generated_code);
}
```

## 最佳实践

### 1. 代码组织

```rust
// 模块化组织
mod generators {
    pub mod struct_generator;
    pub mod trait_generator;
    pub mod impl_generator;
}

mod templates {
    pub mod crud_template;
    pub mod api_template;
}

mod utils {
    pub mod identifier_utils;
    pub mod type_utils;
    pub mod validation_utils;
}

use quote::quote;

// 使用类型别名提高可读性
type TokenStream = proc_macro2::TokenStream;
type Result<T> = std::result::Result<T, syn::Error>;

// 定义常用的代码片段
macro_rules! generate_getter {
    ($field_name:ident, $field_type:ty) => {
        quote! {
            pub fn $field_name(&self) -> &$field_type {
                &self.$field_name
            }
        }
    };
}
```

### 2. 错误处理策略

```rust
use quote::quote;
use syn::{Error, Result};
use proc_macro2::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("语法错误: {0}")]
    Syntax(#[from] syn::Error),
    #[error("验证错误: {0}")]
    Validation(String),
    #[error("配置错误: {0}")]
    Configuration(String),
}

pub type CodeGenResult<T> = std::result::Result<T, CodeGenError>;

fn safe_code_generation(
    name: &str,
    fields: &[(String, String)]
) -> CodeGenResult<proc_macro2::TokenStream> {
    // 输入验证
    validate_struct_name(name)?;
    validate_fields(fields)?;
    
    // 生成代码
    let struct_name = syn::parse_str::<syn::Ident>(name)
        .map_err(CodeGenError::Syntax)?;
    
    let field_definitions = fields.iter()
        .map(|(field_name, field_type)| -> CodeGenResult<_> {
            let field_ident = syn::parse_str::<syn::Ident>(field_name)
                .map_err(CodeGenError::Syntax)?;
            let field_type = syn::parse_str::<syn::Type>(field_type)
                .map_err(CodeGenError::Syntax)?;
            
            Ok(quote! { pub #field_ident: #field_type })
        })
        .collect::<CodeGenResult<Vec<_>>>()?;
    
    Ok(quote! {
        pub struct #struct_name {
            #(#field_definitions,)*
        }
    })
}

fn validate_struct_name(name: &str) -> CodeGenResult<()> {
    if name.is_empty() {
        return Err(CodeGenError::Validation("结构体名称不能为空".to_string()));
    }
    
    if !name.chars().next().unwrap().is_uppercase() {
        return Err(CodeGenError::Validation("结构体名称应以大写字母开头".to_string()));
    }
    
    Ok(())
}

fn validate_fields(fields: &[(String, String)]) -> CodeGenResult<()> {
    if fields.is_empty() {
        return Err(CodeGenError::Validation("结构体必须至少有一个字段".to_string()));
    }
    
    for (field_name, _) in fields {
        if field_name.is_empty() {
            return Err(CodeGenError::Validation("字段名称不能为空".to_string()));
        }
        
        if field_name.chars().next().unwrap().is_uppercase() {
            return Err(CodeGenError::Validation("字段名称应以小写字母开头".to_string()));
        }
    }
    
    Ok(())
}
```

### 3. 性能优化

```rust
use quote::quote;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// 缓存常用的类型和标识符
static COMMON_TYPES: Lazy<HashMap<&'static str, proc_macro2::TokenStream>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("String", quote!(String));
    map.insert("i32", quote!(i32));
    map.insert("u32", quote!(u32));
    map.insert("bool", quote!(bool));
    map.insert("Vec<String>", quote!(Vec<String>));
    map
});

fn get_cached_type(type_name: &str) -> proc_macro2::TokenStream {
    COMMON_TYPES.get(type_name)
        .cloned()
        .unwrap_or_else(|| {
            syn::parse_str::<syn::Type>(type_name)
                .map(|ty| quote!(#ty))
                .unwrap_or_else(|_| quote!(String))
        })
}

// 批量处理避免重复计算
fn batch_generate_structs(
    struct_configs: &[(&str, &[(String, String)])]
) -> Vec<proc_macro2::TokenStream> {
    struct_configs.par_iter()  // 使用 rayon 并行处理
        .map(|(name, fields)| {
            let struct_name = syn::parse_str::<syn::Ident>(name).unwrap();
            let field_defs = fields.iter().map(|(field_name, field_type)| {
                let field_ident = syn::parse_str::<syn::Ident>(field_name).unwrap();
                let field_type = get_cached_type(field_type);
                quote! { pub #field_ident: #field_type }
            });
            
            quote! {
                pub struct #struct_name {
                    #(#field_defs,)*
                }
            }
        })
        .collect()
}
```

### 4. 测试策略

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;
    
    #[test]
    fn test_basic_struct_generation() {
        let fields = vec![
            ("name".to_string(), "String".to_string()),
            ("age".to_string(), "u32".to_string()),
        ];
        
        let result = safe_code_generation("User", &fields).unwrap();
        let expected = quote! {
            pub struct User {
                pub name: String,
                pub age: u32,
            }
        };
        
        assert_eq!(result.to_string(), expected.to_string());
    }
    
    #[test]
    fn test_error_handling() {
        let result = safe_code_generation("", &[]);
        assert!(result.is_err());
        
        let fields = vec![("Name".to_string(), "String".to_string())]; // 大写字段名
        let result = safe_code_generation("User", &fields);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_generated_code_compiles() {
        let fields = vec![
            ("id".to_string(), "u32".to_string()),
            ("data".to_string(), "Vec<String>".to_string()),
        ];
        
        let tokens = safe_code_generation("TestStruct", &fields).unwrap();
        
        // 验证生成的代码可以解析
        let _: syn::ItemStruct = syn::parse2(tokens).unwrap();
    }
}

// 集成测试
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_complex_generation_scenario() {
        let config = ConfigGenerator::new("AppConfig".to_string())
            .add_field(ConfigField {
                name: "database_url".to_string(),
                field_type: "String".to_string(),
                default_value: None,
                env_var: Some("DATABASE_URL".to_string()),
                required: true,
            });
        
        let generated = config.generate();
        
        // 验证生成的代码包含预期的结构
        let code_str = generated.to_string();
        assert!(code_str.contains("pub struct AppConfig"));
        assert!(code_str.contains("pub fn from_env()"));
        assert!(code_str.contains("DATABASE_URL"));
    }
}
```

## 总结

Quote 是 Rust 生态系统中代码生成的基础工具，提供了强大而灵活的准引用功能。通过本教程，您应该能够：

1. 理解 Quote 的核心概念和语法
2. 掌握变量插值和重复机制
3. 实现条件代码生成
4. 处理类型和标识符操作
5. 与过程宏系统集成
6. 应用高级代码生成技巧
7. 实现错误处理和性能优化

关键要点：
- 简洁的准引用语法
- 强大的插值和重复功能
- 类型安全的代码生成
- 与 syn 完美配合
- 高度可扩展的设计

Quote 的设计理念是提供一个简单而强大的工具来生成 Rust 代码，它在保持语法直观性的同时提供了足够的灵活性来处理复杂的代码生成需求。与 syn 配合使用，Quote 构成了 Rust 元编程的核心基础设施。