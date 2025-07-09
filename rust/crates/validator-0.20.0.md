# Validator 0.20.0 - Rust 验证库使用教程

## 概述

Validator 是一个功能强大的 Rust 验证库，提供了常见的验证功能（如邮箱、URL、长度等）。它通常与 `validator_derive` 宏一起使用，可以轻松地为结构体字段添加验证规则。

**基本信息：**
- 版本：0.20.0
- 许可证：MIT
- 仓库：https://github.com/Keats/validator
- 下载量：24,015,267+

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
validator = { version = "0.20", features = ["derive"] }
```

## 基本使用

### 1. 导入必要的宏和特征

```rust
use validator::{Validate, ValidationError};
use validator::derive::Validate;
```

### 2. 定义带验证的结构体

```rust
#[derive(Debug, Validate)]
struct SignupData {
    #[validate(
        length(min = 1, message = "用户名不能为空"),
        length(max = 20, message = "用户名不能超过20个字符")
    )]
    username: String,
    
    #[validate(email(message = "请输入有效的邮箱地址"))]
    email: String,
    
    #[validate(length(min = 8, message = "密码长度至少8个字符"))]
    password: String,
    
    #[validate(range(min = 18, max = 120, message = "年龄必须在18到120之间"))]
    age: u32,
}
```

### 3. 执行验证

```rust
fn main() {
    let signup = SignupData {
        username: "".to_string(),
        email: "invalid-email".to_string(),
        password: "123".to_string(),
        age: 16,
    };
    
    match signup.validate() {
        Ok(_) => println!("验证通过"),
        Err(e) => {
            println!("验证失败：");
            for (field, errors) in e.field_errors() {
                for error in errors {
                    println!("  {}: {}", field, error.message.as_ref().unwrap_or(&"验证失败".into()));
                }
            }
        }
    }
}
```

## 常用验证规则

### 1. 字符串验证

```rust
#[derive(Debug, Validate)]
struct StringValidation {
    // 长度验证
    #[validate(length(min = 5, max = 50))]
    name: String,
    
    // 邮箱验证
    #[validate(email)]
    email: String,
    
    // URL验证
    #[validate(url)]
    website: String,
    
    // 正则表达式验证
    #[validate(regex = "PHONE_REGEX")]
    phone: String,
    
    // 自定义验证函数
    #[validate(custom = "validate_username")]
    username: String,
}

// 正则表达式常量
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^\d{11}$").unwrap();
}

// 自定义验证函数
fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.contains("admin") {
        return Err(ValidationError::new("用户名不能包含 'admin'"));
    }
    Ok(())
}
```

### 2. 数字验证

```rust
#[derive(Debug, Validate)]
struct NumberValidation {
    // 范围验证
    #[validate(range(min = 0, max = 100))]
    score: f64,
    
    // 最小值验证
    #[validate(range(min = 18))]
    age: u32,
    
    // 最大值验证
    #[validate(range(max = 1000))]
    price: u32,
}
```

### 3. 集合验证

```rust
#[derive(Debug, Validate)]
struct CollectionValidation {
    // 数组长度验证
    #[validate(length(min = 1, max = 10))]
    tags: Vec<String>,
    
    // 嵌套验证
    #[validate]
    profile: UserProfile,
    
    // 可选字段验证
    #[validate(email)]
    backup_email: Option<String>,
}

#[derive(Debug, Validate)]
struct UserProfile {
    #[validate(length(min = 1))]
    first_name: String,
    
    #[validate(length(min = 1))]
    last_name: String,
}
```

## 高级功能

### 1. 自定义错误消息

```rust
#[derive(Debug, Validate)]
struct CustomMessages {
    #[validate(
        length(min = 1, message = "姓名不能为空"),
        length(max = 50, message = "姓名不能超过50个字符")
    )]
    name: String,
    
    #[validate(
        email(message = "请输入有效的邮箱地址"),
        length(min = 5, message = "邮箱长度至少5个字符")
    )]
    email: String,
}
```

### 2. 条件验证

```rust
#[derive(Debug, Validate)]
struct ConditionalValidation {
    user_type: String,
    
    #[validate(email)]
    #[validate(custom = "validate_admin_email")]
    email: String,
}

fn validate_admin_email(email: &str) -> Result<(), ValidationError> {
    // 仅对管理员用户进行额外验证
    if email.ends_with("@admin.com") {
        Ok(())
    } else {
        Err(ValidationError::new("管理员邮箱必须以 @admin.com 结尾"))
    }
}
```

### 3. 批量验证

```rust
use validator::{ValidationErrors, ValidationErrorsKind};

fn validate_multiple_users(users: &[SignupData]) -> Result<(), Vec<ValidationErrors>> {
    let mut errors = Vec::new();
    
    for user in users {
        if let Err(e) = user.validate() {
            errors.push(e);
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

## 错误处理

### 1. 错误信息提取

```rust
use validator::{ValidationErrors, ValidationErrorsKind};

fn handle_validation_errors(errors: ValidationErrors) {
    for (field, error_kind) in errors.errors() {
        match error_kind {
            ValidationErrorsKind::Field(field_errors) => {
                for error in field_errors {
                    println!("字段 '{}' 验证失败: {}", 
                        field, 
                        error.message.as_ref().unwrap_or(&"未知错误".into())
                    );
                }
            }
            ValidationErrorsKind::Struct(struct_errors) => {
                // 处理结构体级别的错误
                println!("结构体验证失败: {:?}", struct_errors);
            }
            ValidationErrorsKind::List(list_errors) => {
                // 处理列表验证错误
                println!("列表验证失败: {:?}", list_errors);
            }
        }
    }
}
```

### 2. 国际化错误消息

```rust
use validator::ValidationError;

fn get_localized_message(error: &ValidationError, lang: &str) -> String {
    match error.code.as_ref() {
        "email" => match lang {
            "zh" => "请输入有效的邮箱地址".to_string(),
            "en" => "Please enter a valid email address".to_string(),
            _ => "Invalid email".to_string(),
        },
        "length" => match lang {
            "zh" => format!("长度必须在{}到{}之间", 
                error.params.get("min").unwrap_or(&serde_json::Value::Null),
                error.params.get("max").unwrap_or(&serde_json::Value::Null)
            ),
            _ => "Invalid length".to_string(),
        },
        _ => error.message.as_ref().unwrap_or(&"验证失败".into()).to_string(),
    }
}
```

## 与 Web 框架集成

### 1. 与 Actix-web 集成

```rust
use actix_web::{web, HttpResponse, Result};
use validator::Validate;

#[derive(Debug, Validate, serde::Deserialize)]
struct CreateUserRequest {
    #[validate(length(min = 1, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8))]
    password: String,
}

async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<HttpResponse> {
    if let Err(errors) = user_data.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }
    
    // 处理用户创建逻辑
    Ok(HttpResponse::Created().json("用户创建成功"))
}
```

### 2. 与 Warp 集成

```rust
use warp::{Filter, Rejection, Reply};
use validator::Validate;

#[derive(Debug, Validate, serde::Deserialize)]
struct UserInput {
    #[validate(length(min = 1))]
    name: String,
    
    #[validate(email)]
    email: String,
}

async fn handle_user_input(input: UserInput) -> Result<impl Reply, Rejection> {
    if let Err(errors) = input.validate() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&errors),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    Ok(warp::reply::with_status(
        warp::reply::json(&"处理成功"),
        warp::http::StatusCode::OK,
    ))
}
```

## 性能优化

### 1. 使用 `lazy_static` 优化正则表达式

```rust
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^\d{11}$").unwrap();
}

#[derive(Debug, Validate)]
struct OptimizedValidation {
    #[validate(regex = "EMAIL_REGEX")]
    email: String,
    
    #[validate(regex = "PHONE_REGEX")]
    phone: String,
}
```

### 2. 条件验证优化

```rust
fn validate_conditionally(data: &MyData) -> Result<(), ValidationError> {
    // 只在必要时进行昂贵的验证
    if data.needs_expensive_validation {
        expensive_validation(&data.field)?;
    }
    Ok(())
}
```

## 最佳实践

### 1. 验证规则组织

```rust
// 将验证规则分组到不同的结构体中
#[derive(Debug, Validate)]
struct UserRegistration {
    #[validate]
    personal_info: PersonalInfo,
    
    #[validate]
    account_info: AccountInfo,
    
    #[validate]
    preferences: UserPreferences,
}

#[derive(Debug, Validate)]
struct PersonalInfo {
    #[validate(length(min = 1, max = 50))]
    first_name: String,
    
    #[validate(length(min = 1, max = 50))]
    last_name: String,
    
    #[validate(range(min = 18, max = 120))]
    age: u32,
}

#[derive(Debug, Validate)]
struct AccountInfo {
    #[validate(length(min = 3, max = 30))]
    username: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8))]
    password: String,
}
```

### 2. 错误处理封装

```rust
#[derive(Debug)]
pub struct ValidationResponse {
    pub success: bool,
    pub errors: Vec<FieldError>,
}

#[derive(Debug)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl From<ValidationErrors> for ValidationResponse {
    fn from(errors: ValidationErrors) -> Self {
        let mut field_errors = Vec::new();
        
        for (field, error_kind) in errors.errors() {
            if let ValidationErrorsKind::Field(field_errors_vec) = error_kind {
                for error in field_errors_vec {
                    field_errors.push(FieldError {
                        field: field.to_string(),
                        message: error.message.as_ref().unwrap_or(&"验证失败".into()).to_string(),
                        code: error.code.to_string(),
                    });
                }
            }
        }
        
        ValidationResponse {
            success: false,
            errors: field_errors,
        }
    }
}
```

## 总结

Validator 是一个功能强大且易于使用的 Rust 验证库，提供了：

1. **简单易用**：通过派生宏轻松添加验证规则
2. **功能丰富**：支持各种常见验证场景
3. **可扩展**：支持自定义验证函数和错误消息
4. **高性能**：编译时验证，运行时性能优秀
5. **良好集成**：与各种 Web 框架无缝集成

使用时建议：
- 合理组织验证规则
- 提供清晰的错误消息
- 使用条件验证优化性能
- 考虑国际化需求
- 与业务逻辑适当分离

这个库是 Rust 生态系统中验证数据的标准选择，适合各种规模的项目使用。