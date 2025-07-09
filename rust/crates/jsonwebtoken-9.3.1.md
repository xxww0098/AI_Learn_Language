# JsonWebToken 9.3.1 中文教程

## 简介

JsonWebToken (JWT) 是一个用于在 Rust 中创建和解码 JSON Web Tokens 的库。JWT 是一种紧凑的、URL 安全的方式，用于在各方之间传输信息。该库提供了强类型的 API，确保 JWT 的创建和验证的安全性。

## 核心概念

### JWT 结构

JWT 由三部分组成，用点（.）分隔：
- **Header（头部）**：包含算法和令牌类型
- **Payload（负载）**：包含声明（claims）
- **Signature（签名）**：用于验证令牌的完整性

### 声明（Claims）

JWT 的有效负载包含声明，分为三类：
- **Registered Claims**：预定义的声明（如 `exp`、`iat`、`sub`）
- **Public Claims**：公开定义的声明
- **Private Claims**：自定义声明

## 基本用法

### 1. 创建简单的 JWT

```rust
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// 定义自定义声明
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,    // Subject
    exp: usize,     // Expiration time
    iat: usize,     // Issued at
    name: String,   // 自定义字段
    admin: bool,    // 自定义字段
}

fn create_jwt() -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    
    let claims = Claims {
        sub: "user123".to_string(),
        exp: now + 3600, // 1小时后过期
        iat: now,
        name: "John Doe".to_string(),
        admin: true,
    };
    
    let header = Header::default();
    let secret = "my-secret-key";
    
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}

fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = "my-secret-key";
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

fn main() {
    // 创建 JWT
    let token = create_jwt().unwrap();
    println!("Generated JWT: {}", token);
    
    // 验证 JWT
    let claims = verify_jwt(&token).unwrap();
    println!("Verified claims: {:?}", claims);
}
```

### 2. 使用不同的算法

```rust
use jsonwebtoken::{Header, Algorithm, EncodingKey, DecodingKey};

// HS256 (HMAC SHA256) - 对称加密
let header = Header::new(Algorithm::HS256);
let secret = "my-secret-key";
let encoding_key = EncodingKey::from_secret(secret.as_ref());
let decoding_key = DecodingKey::from_secret(secret.as_ref());

// HS384 (HMAC SHA384)
let header = Header::new(Algorithm::HS384);

// HS512 (HMAC SHA512)
let header = Header::new(Algorithm::HS512);

// RS256 (RSA SHA256) - 非对称加密
let rsa_private_key = include_bytes!("private_key.pem");
let rsa_public_key = include_bytes!("public_key.pem");

let header = Header::new(Algorithm::RS256);
let encoding_key = EncodingKey::from_rsa_pem(rsa_private_key)?;
let decoding_key = DecodingKey::from_rsa_pem(rsa_public_key)?;

// ES256 (ECDSA SHA256)
let ecdsa_private_key = include_bytes!("ec_private_key.pem");
let ecdsa_public_key = include_bytes!("ec_public_key.pem");

let header = Header::new(Algorithm::ES256);
let encoding_key = EncodingKey::from_ec_pem(ecdsa_private_key)?;
let decoding_key = DecodingKey::from_ec_pem(ecdsa_public_key)?;
```

### 3. 高级验证配置

```rust
use jsonwebtoken::{Validation, Algorithm};

// 自定义验证参数
let mut validation = Validation::new(Algorithm::HS256);

// 设置受众（audience）
validation.set_audience(&["my-app", "my-api"]);

// 设置发行者（issuer）
validation.set_issuer(&["my-auth-server"]);

// 设置主题（subject）
validation.sub = Some("user123".to_string());

// 设置过期时间容差（秒）
validation.leeway = 60;

// 禁用过期时间验证
validation.validate_exp = false;

// 禁用 "not before" 验证
validation.validate_nbf = false;

// 必需的声明
validation.required_spec_claims = std::collections::HashSet::from([
    "exp".to_string(),
    "iat".to_string(),
    "sub".to_string(),
]);

// 使用自定义验证
let token_data = decode::<Claims>(
    &token,
    &decoding_key,
    &validation,
)?;
```

## 实际应用示例

### 1. 用户认证系统

```rust
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    sub: String,        // 用户ID
    email: String,      // 邮箱
    role: String,       // 角色
    permissions: Vec<String>, // 权限列表
    exp: usize,         // 过期时间
    iat: usize,         // 签发时间
    iss: String,        // 发行者
    aud: String,        // 受众
}

struct AuthService {
    secret: String,
    issuer: String,
    audience: String,
}

impl AuthService {
    fn new(secret: String, issuer: String, audience: String) -> Self {
        Self { secret, issuer, audience }
    }
    
    fn generate_token(&self, user_id: &str, email: &str, role: &str, permissions: Vec<String>) -> Result<String, Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let claims = UserClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            permissions,
            exp: now + 3600, // 1小时后过期
            iat: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };
        
        let header = Header::default();
        encode(&header, &claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }
    
    fn verify_token(&self, token: &str) -> Result<UserClaims, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        
        let token_data = decode::<UserClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?;
        
        Ok(token_data.claims)
    }
    
    fn refresh_token(&self, token: &str) -> Result<String, Error> {
        let claims = self.verify_token(token)?;
        
        // 创建新的令牌
        self.generate_token(&claims.sub, &claims.email, &claims.role, claims.permissions)
    }
    
    fn has_permission(&self, token: &str, required_permission: &str) -> bool {
        match self.verify_token(token) {
            Ok(claims) => claims.permissions.contains(&required_permission.to_string()),
            Err(_) => false,
        }
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_service = AuthService::new(
        "super-secret-key".to_string(),
        "my-auth-server".to_string(),
        "my-app".to_string(),
    );
    
    // 生成令牌
    let token = auth_service.generate_token(
        "user123",
        "user@example.com",
        "admin",
        vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    )?;
    
    println!("Generated token: {}", token);
    
    // 验证令牌
    let claims = auth_service.verify_token(&token)?;
    println!("User: {} ({})", claims.email, claims.role);
    
    // 检查权限
    if auth_service.has_permission(&token, "delete") {
        println!("User has delete permission");
    }
    
    // 刷新令牌
    let new_token = auth_service.refresh_token(&token)?;
    println!("New token: {}", new_token);
    
    Ok(())
}
```

### 2. API 中间件

```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Serialize, Deserialize)]
struct ApiClaims {
    sub: String,
    role: String,
    exp: usize,
    iat: usize,
}

// JWT 中间件
pub struct JwtMiddleware<S> {
    inner: S,
    secret: String,
}

impl<S> JwtMiddleware<S> {
    pub fn new(inner: S, secret: String) -> Self {
        Self { inner, secret }
    }
}

impl<S, B> tower::Service<hyper::Request<B>> for JwtMiddleware<S>
where
    S: tower::Service<hyper::Request<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, mut req: hyper::Request<B>) -> Self::Future {
        let secret = self.secret.clone();
        let mut inner = self.inner.clone();
        
        Box::pin(async move {
            // 从请求头中提取 JWT
            let token = req.headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer "));
            
            if let Some(token) = token {
                // 验证 JWT
                let validation = Validation::new(Algorithm::HS256);
                if let Ok(token_data) = decode::<ApiClaims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &validation,
                ) {
                    // 将用户信息添加到请求扩展中
                    req.extensions_mut().insert(token_data.claims);
                }
            }
            
            inner.call(req).await
        })
    }
}
```

### 3. 多租户系统

```rust
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct TenantClaims {
    sub: String,        // 用户ID
    tenant_id: String,  // 租户ID
    role: String,       // 角色
    permissions: Vec<String>, // 权限
    exp: usize,
    iat: usize,
}

struct MultiTenantAuth {
    tenant_secrets: HashMap<String, String>,
}

impl MultiTenantAuth {
    fn new() -> Self {
        Self {
            tenant_secrets: HashMap::new(),
        }
    }
    
    fn add_tenant(&mut self, tenant_id: String, secret: String) {
        self.tenant_secrets.insert(tenant_id, secret);
    }
    
    fn generate_token(&self, tenant_id: &str, user_id: &str, role: &str, permissions: Vec<String>) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = self.tenant_secrets.get(tenant_id)
            .ok_or_else(|| jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let claims = TenantClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            role: role.to_string(),
            permissions,
            exp: now + 3600,
            iat: now,
        };
        
        let header = Header::default();
        encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
    }
    
    fn verify_token(&self, token: &str) -> Result<TenantClaims, jsonwebtoken::errors::Error> {
        // 首先解码不验证签名，获取租户信息
        let header = jsonwebtoken::decode_header(token)?;
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.validate_nbf = false;
        
        // 临时解码获取租户ID
        let temp_claims: TenantClaims = decode(
            token,
            &DecodingKey::from_secret("temp".as_ref()),
            &validation,
        ).map_err(|_| {
            // 如果失败，尝试逐个租户验证
            for (tenant_id, secret) in &self.tenant_secrets {
                let mut validation = Validation::new(Algorithm::HS256);
                if let Ok(token_data) = decode::<TenantClaims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &validation,
                ) {
                    return Ok(token_data.claims);
                }
            }
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
        })?;
        
        // 使用正确的租户密钥验证
        let secret = self.tenant_secrets.get(&temp_claims.tenant_id)
            .ok_or_else(|| jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?;
        
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<TenantClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;
        
        Ok(token_data.claims)
    }
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth = MultiTenantAuth::new();
    
    // 添加租户
    auth.add_tenant("tenant1".to_string(), "secret1".to_string());
    auth.add_tenant("tenant2".to_string(), "secret2".to_string());
    
    // 为不同租户生成令牌
    let token1 = auth.generate_token("tenant1", "user1", "admin", vec!["read".to_string(), "write".to_string()])?;
    let token2 = auth.generate_token("tenant2", "user2", "user", vec!["read".to_string()])?;
    
    // 验证令牌
    let claims1 = auth.verify_token(&token1)?;
    let claims2 = auth.verify_token(&token2)?;
    
    println!("Tenant 1 user: {} ({})", claims1.sub, claims1.tenant_id);
    println!("Tenant 2 user: {} ({})", claims2.sub, claims2.tenant_id);
    
    Ok(())
}
```

## 安全最佳实践

### 1. 密钥管理

```rust
use jsonwebtoken::{EncodingKey, DecodingKey};
use std::env;

// 从环境变量读取密钥
let secret = env::var("JWT_SECRET")
    .expect("JWT_SECRET must be set");

// 使用 RSA 密钥对
let private_key = std::fs::read("private_key.pem")
    .expect("Failed to read private key");
let public_key = std::fs::read("public_key.pem")
    .expect("Failed to read public key");

let encoding_key = EncodingKey::from_rsa_pem(&private_key)?;
let decoding_key = DecodingKey::from_rsa_pem(&public_key)?;

// 密钥轮换
struct KeyManager {
    current_key: String,
    previous_key: Option<String>,
}

impl KeyManager {
    fn verify_with_fallback(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // 首先尝试当前密钥
        let validation = Validation::default();
        if let Ok(token_data) = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.current_key.as_ref()),
            &validation,
        ) {
            return Ok(token_data.claims);
        }
        
        // 如果失败，尝试旧密钥
        if let Some(previous_key) = &self.previous_key {
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(previous_key.as_ref()),
                &validation,
            )?;
            return Ok(token_data.claims);
        }
        
        Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))
    }
}
```

### 2. 令牌黑名单

```rust
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

struct TokenBlacklist {
    blacklisted_tokens: Arc<Mutex<HashSet<String>>>,
}

impl TokenBlacklist {
    fn new() -> Self {
        Self {
            blacklisted_tokens: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    
    fn blacklist_token(&self, token: &str) {
        let mut blacklist = self.blacklisted_tokens.lock().unwrap();
        blacklist.insert(token.to_string());
    }
    
    fn is_blacklisted(&self, token: &str) -> bool {
        let blacklist = self.blacklisted_tokens.lock().unwrap();
        blacklist.contains(token)
    }
    
    fn verify_token(&self, token: &str, auth_service: &AuthService) -> Result<UserClaims, jsonwebtoken::errors::Error> {
        if self.is_blacklisted(token) {
            return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
        }
        
        auth_service.verify_token(token)
    }
}
```

### 3. 令牌刷新机制

```rust
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct RefreshClaims {
    sub: String,
    token_type: String, // "access" or "refresh"
    exp: usize,
    iat: usize,
}

struct TokenPair {
    access_token: String,
    refresh_token: String,
}

impl AuthService {
    fn generate_token_pair(&self, user_id: &str) -> Result<TokenPair, jsonwebtoken::errors::Error> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        // 访问令牌（短期）
        let access_claims = RefreshClaims {
            sub: user_id.to_string(),
            token_type: "access".to_string(),
            exp: now + 900, // 15分钟
            iat: now,
        };
        
        // 刷新令牌（长期）
        let refresh_claims = RefreshClaims {
            sub: user_id.to_string(),
            token_type: "refresh".to_string(),
            exp: now + 2592000, // 30天
            iat: now,
        };
        
        let header = Header::default();
        let access_token = encode(&header, &access_claims, &EncodingKey::from_secret(self.secret.as_ref()))?;
        let refresh_token = encode(&header, &refresh_claims, &EncodingKey::from_secret(self.secret.as_ref()))?;
        
        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
    
    fn refresh_access_token(&self, refresh_token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<RefreshClaims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?;
        
        // 验证是否为刷新令牌
        if token_data.claims.token_type != "refresh" {
            return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
        }
        
        // 生成新的访问令牌
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let new_access_claims = RefreshClaims {
            sub: token_data.claims.sub,
            token_type: "access".to_string(),
            exp: now + 900,
            iat: now,
        };
        
        let header = Header::default();
        encode(&header, &new_access_claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }
}
```

## 错误处理

### 1. 自定义错误类型

```rust
use jsonwebtoken::errors::{Error, ErrorKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Token not found")]
    TokenNotFound,
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    #[error("JWT error: {0}")]
    JwtError(#[from] Error),
}

impl From<ErrorKind> for AuthError {
    fn from(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            ErrorKind::InvalidToken => AuthError::InvalidToken,
            _ => AuthError::JwtError(Error::from(kind)),
        }
    }
}
```

### 2. 错误处理示例

```rust
fn handle_auth_error(error: AuthError) -> http::Response<String> {
    match error {
        AuthError::InvalidToken => {
            http::Response::builder()
                .status(401)
                .body("Invalid token".to_string())
                .unwrap()
        }
        AuthError::TokenExpired => {
            http::Response::builder()
                .status(401)
                .body("Token expired".to_string())
                .unwrap()
        }
        AuthError::InsufficientPermissions => {
            http::Response::builder()
                .status(403)
                .body("Insufficient permissions".to_string())
                .unwrap()
        }
        _ => {
            http::Response::builder()
                .status(500)
                .body("Internal server error".to_string())
                .unwrap()
        }
    }
}
```

## 性能优化

### 1. 令牌缓存

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};

struct TokenCache {
    cache: Arc<Mutex<HashMap<String, (UserClaims, SystemTime)>>>,
    ttl: Duration,
}

impl TokenCache {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }
    
    fn get(&self, token: &str) -> Option<UserClaims> {
        let mut cache = self.cache.lock().unwrap();
        if let Some((claims, timestamp)) = cache.get(token) {
            if timestamp.elapsed().unwrap() < self.ttl {
                return Some(claims.clone());
            } else {
                cache.remove(token);
            }
        }
        None
    }
    
    fn set(&self, token: String, claims: UserClaims) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(token, (claims, SystemTime::now()));
    }
}
```

### 2. 批量验证

```rust
use rayon::prelude::*;

fn verify_tokens_batch(tokens: Vec<String>, auth_service: &AuthService) -> Vec<Result<UserClaims, jsonwebtoken::errors::Error>> {
    tokens.par_iter()
        .map(|token| auth_service.verify_token(token))
        .collect()
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
jsonwebtoken = "9.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

## 总结

JsonWebToken 是一个功能强大、类型安全的 JWT 库，适用于各种认证和授权场景。通过合理的配置和最佳实践，可以构建安全、高效的认证系统。

主要特性：
- 🔐 支持多种加密算法
- 🚀 高性能和内存安全
- 🛡️ 强类型验证
- 🔧 灵活的配置选项
- 📝 详细的错误处理
- 🌐 完整的 JWT 规范支持

JsonWebToken 是 Rust 生态系统中构建认证系统的首选库。