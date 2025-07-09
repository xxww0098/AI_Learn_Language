# libloading 0.8.8 详细中文使用教程

## 简介

`libloading` 是一个用于在运行时动态加载共享库的 Rust 库。它提供了跨平台的动态库加载功能，支持 Windows (DLL)、Linux (SO) 和 macOS (dylib) 等平台，并提供了内存安全的接口。

## 基本信息

- **版本**: 0.8.8
- **许可证**: ISC
- **文档**: https://docs.rs/libloading/
- **仓库**: https://github.com/nagisa/rust_libloading/
- **下载量**: 203,802,035 (极高人气)

## 快速开始

### 1. 添加依赖

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
libloading = "0.8.8"
```

### 2. 基本使用

```rust
use libloading::{Library, Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载共享库
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 获取函数符号
    let cos: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"cos") }?;
    
    // 调用函数
    let result = unsafe { cos(0.0) };
    println!("cos(0.0) = {}", result);
    
    Ok(())
}
```

## 核心概念

### 1. Library 结构

`Library` 是动态库的主要接口：

```rust
use libloading::Library;

fn library_examples() -> Result<(), Box<dyn std::error::Error>> {
    // 加载库的不同方式
    
    // 通过文件名加载
    let lib1 = unsafe { Library::new("libm.so.6") }?;
    
    // 通过绝对路径加载
    let lib2 = unsafe { Library::new("/usr/lib/x86_64-linux-gnu/libm.so.6") }?;
    
    // 在 Windows 上
    #[cfg(windows)]
    let lib3 = unsafe { Library::new("kernel32.dll") }?;
    
    // 在 macOS 上
    #[cfg(target_os = "macos")]
    let lib4 = unsafe { Library::new("libSystem.dylib") }?;
    
    println!("库加载成功");
    Ok(())
}
```

### 2. Symbol 获取

```rust
use libloading::{Library, Symbol};

fn symbol_examples() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 获取不同类型的符号
    
    // 函数符号
    let sin: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"sin") }?;
    
    // 变量符号
    let errno: Symbol<*mut i32> = unsafe { lib.get(b"errno") }?;
    
    // 使用符号
    let result = unsafe { sin(3.14159265359 / 2.0) };
    println!("sin(π/2) = {}", result);
    
    // 检查 errno
    let errno_value = unsafe { **errno };
    println!("errno = {}", errno_value);
    
    Ok(())
}
```

### 3. 生命周期管理

```rust
use libloading::{Library, Symbol};

fn lifetime_management() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // Symbol 的生命周期绑定到 Library
    let calculate = |x: f64| -> Result<f64, Box<dyn std::error::Error>> {
        let cos: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"cos") }?;
        let result = unsafe { cos(x) };
        Ok(result)
    };
    
    let result = calculate(0.0)?;
    println!("计算结果: {}", result);
    
    // 库在作用域结束时自动卸载
    Ok(())
}
```

## 错误处理

### 1. 加载错误

```rust
use libloading::{Library, Error};

fn error_handling_examples() {
    // 处理库加载错误
    match unsafe { Library::new("nonexistent_library.so") } {
        Ok(lib) => {
            println!("库加载成功");
        }
        Err(e) => {
            println!("库加载失败: {}", e);
            match e {
                Error::DlOpen { desc } => {
                    println!("dlopen 错误: {}", desc);
                }
                Error::DlSym { desc } => {
                    println!("dlsym 错误: {}", desc);
                }
                Error::DlClose { desc } => {
                    println!("dlclose 错误: {}", desc);
                }
                Error::GetProcAddress { desc } => {
                    println!("GetProcAddress 错误: {}", desc);
                }
                Error::LoadLibrary { desc } => {
                    println!("LoadLibrary 错误: {}", desc);
                }
                Error::FreeLibrary { desc } => {
                    println!("FreeLibrary 错误: {}", desc);
                }
                _ => {
                    println!("其他错误: {}", e);
                }
            }
        }
    }
    
    // 处理符号获取错误
    if let Ok(lib) = unsafe { Library::new("libm.so.6") } {
        match unsafe { lib.get::<unsafe extern fn(f64) -> f64>(b"nonexistent_function") } {
            Ok(func) => {
                println!("符号获取成功");
            }
            Err(e) => {
                println!("符号获取失败: {}", e);
            }
        }
    }
}
```

### 2. 安全的符号获取

```rust
use libloading::{Library, Symbol};

fn safe_symbol_access() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 安全的符号获取函数
    fn get_symbol<T>(lib: &Library, name: &[u8]) -> Result<Symbol<T>, Box<dyn std::error::Error>> {
        unsafe { lib.get(name) }.map_err(|e| e.into())
    }
    
    // 使用安全包装
    let sin = get_symbol::<unsafe extern fn(f64) -> f64>(&lib, b"sin")?;
    let cos = get_symbol::<unsafe extern fn(f64) -> f64>(&lib, b"cos")?;
    
    // 安全的调用包装
    let safe_sin = |x: f64| -> f64 {
        unsafe { sin(x) }
    };
    
    let safe_cos = |x: f64| -> f64 {
        unsafe { cos(x) }
    };
    
    println!("sin(π/2) = {}", safe_sin(std::f64::consts::FRAC_PI_2));
    println!("cos(0) = {}", safe_cos(0.0));
    
    Ok(())
}
```

## 跨平台支持

### 1. 平台特定代码

```rust
use libloading::Library;

fn platform_specific_examples() -> Result<(), Box<dyn std::error::Error>> {
    // 根据平台选择不同的库
    let lib = unsafe {
        #[cfg(target_os = "linux")]
        Library::new("libm.so.6")?;
        
        #[cfg(target_os = "windows")]
        Library::new("msvcrt.dll")?;
        
        #[cfg(target_os = "macos")]
        Library::new("libSystem.dylib")?;
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return Err("不支持的平台".into());
        
        lib
    };
    
    println!("平台特定库加载成功");
    Ok(())
}
```

### 2. 库文件名处理

```rust
use libloading::Library;

fn library_naming() -> Result<(), Box<dyn std::error::Error>> {
    // 跨平台的库名处理
    let library_name = if cfg!(target_os = "windows") {
        "example.dll"
    } else if cfg!(target_os = "macos") {
        "libexample.dylib"
    } else {
        "libexample.so"
    };
    
    // 尝试加载库
    match unsafe { Library::new(library_name) } {
        Ok(lib) => {
            println!("库 {} 加载成功", library_name);
        }
        Err(e) => {
            println!("库 {} 加载失败: {}", library_name, e);
        }
    }
    
    // 带版本号的库名
    let versioned_name = if cfg!(target_os = "linux") {
        "libexample.so.1"
    } else {
        library_name
    };
    
    match unsafe { Library::new(versioned_name) } {
        Ok(lib) => {
            println!("版本化库 {} 加载成功", versioned_name);
        }
        Err(e) => {
            println!("版本化库 {} 加载失败: {}", versioned_name, e);
        }
    }
    
    Ok(())
}
```

## 高级功能

### 1. 函数类型安全

```rust
use libloading::{Library, Symbol};

// 定义函数类型别名
type MathFunction = unsafe extern fn(f64) -> f64;
type StringFunction = unsafe extern fn(*const i8) -> i32;

fn type_safe_loading() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 类型安全的函数获取
    let sin: Symbol<MathFunction> = unsafe { lib.get(b"sin") }?;
    let cos: Symbol<MathFunction> = unsafe { lib.get(b"cos") }?;
    let tan: Symbol<MathFunction> = unsafe { lib.get(b"tan") }?;
    
    // 创建函数表
    let math_functions = [
        ("sin", sin),
        ("cos", cos),
        ("tan", tan),
    ];
    
    let x = std::f64::consts::FRAC_PI_4; // π/4
    
    for (name, func) in math_functions.iter() {
        let result = unsafe { func(x) };
        println!("{}(π/4) = {}", name, result);
    }
    
    Ok(())
}
```

### 2. 符号缓存

```rust
use libloading::{Library, Symbol};
use std::collections::HashMap;

struct SymbolCache {
    library: Library,
    symbol_cache: HashMap<String, *const ()>,
}

impl SymbolCache {
    fn new(library_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let library = unsafe { Library::new(library_path) }?;
        Ok(SymbolCache {
            library,
            symbol_cache: HashMap::new(),
        })
    }
    
    fn get_symbol<T>(&mut self, name: &str) -> Result<Symbol<T>, Box<dyn std::error::Error>> {
        let symbol_ptr = if let Some(&ptr) = self.symbol_cache.get(name) {
            ptr
        } else {
            let symbol: Symbol<T> = unsafe { self.library.get(name.as_bytes()) }?;
            let ptr = symbol.as_ptr() as *const ();
            self.symbol_cache.insert(name.to_string(), ptr);
            ptr
        };
        
        // 这里需要更复杂的类型转换，实际使用中需要更安全的方法
        unsafe {
            let symbol: Symbol<T> = self.library.get(name.as_bytes())?;
            Ok(symbol)
        }
    }
}

fn symbol_caching_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = SymbolCache::new("libm.so.6")?;
    
    // 第一次获取符号
    let sin1: Symbol<unsafe extern fn(f64) -> f64> = cache.get_symbol("sin")?;
    
    // 第二次获取符号（从缓存）
    let sin2: Symbol<unsafe extern fn(f64) -> f64> = cache.get_symbol("sin")?;
    
    println!("符号缓存示例完成");
    Ok(())
}
```

### 3. 动态插件系统

```rust
use libloading::{Library, Symbol};
use std::collections::HashMap;

// 定义插件接口
trait Plugin {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn execute(&self, input: &str) -> String;
}

// 插件创建函数类型
type CreatePlugin = unsafe extern fn() -> *mut dyn Plugin;
type DestroyPlugin = unsafe extern fn(*mut dyn Plugin);

struct PluginManager {
    plugins: HashMap<String, (Library, Box<dyn Plugin>)>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }
    
    fn load_plugin(&mut self, name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(path) }?;
        
        // 获取插件创建函数
        let create_plugin: Symbol<CreatePlugin> = unsafe { lib.get(b"create_plugin") }?;
        
        // 创建插件实例
        let plugin_ptr = unsafe { create_plugin() };
        let plugin = unsafe { Box::from_raw(plugin_ptr) };
        
        self.plugins.insert(name.to_string(), (lib, plugin));
        
        Ok(())
    }
    
    fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|(_, plugin)| plugin.as_ref())
    }
    
    fn unload_plugin(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((lib, plugin)) = self.plugins.remove(name) {
            // 获取销毁函数
            let destroy_plugin: Symbol<DestroyPlugin> = unsafe { lib.get(b"destroy_plugin") }?;
            
            // 销毁插件
            let plugin_ptr = Box::into_raw(plugin);
            unsafe { destroy_plugin(plugin_ptr) };
        }
        
        Ok(())
    }
}

fn plugin_system_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();
    
    // 加载插件（假设插件存在）
    match manager.load_plugin("example", "libexample_plugin.so") {
        Ok(()) => {
            println!("插件加载成功");
            
            if let Some(plugin) = manager.get_plugin("example") {
                println!("插件名称: {}", plugin.name());
                println!("插件版本: {}", plugin.version());
                
                let result = plugin.execute("test input");
                println!("插件执行结果: {}", result);
            }
            
            // 卸载插件
            manager.unload_plugin("example")?;
            println!("插件卸载成功");
        }
        Err(e) => {
            println!("插件加载失败: {}", e);
        }
    }
    
    Ok(())
}
```

## 实际应用示例

### 1. 数据库驱动加载

```rust
use libloading::{Library, Symbol};

struct DatabaseDriver {
    library: Library,
    connect: Symbol<'static, unsafe extern fn(*const i8) -> *mut ()>,
    execute: Symbol<'static, unsafe extern fn(*mut (), *const i8) -> i32>,
    disconnect: Symbol<'static, unsafe extern fn(*mut ())>,
}

impl DatabaseDriver {
    fn new(driver_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let library = unsafe { Library::new(driver_path) }?;
        
        let connect = unsafe { library.get(b"db_connect") }?;
        let execute = unsafe { library.get(b"db_execute") }?;
        let disconnect = unsafe { library.get(b"db_disconnect") }?;
        
        Ok(DatabaseDriver {
            library,
            connect: unsafe { std::mem::transmute(connect) },
            execute: unsafe { std::mem::transmute(execute) },
            disconnect: unsafe { std::mem::transmute(disconnect) },
        })
    }
    
    fn connect(&self, connection_string: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
        let c_string = std::ffi::CString::new(connection_string)?;
        let handle = unsafe { (self.connect)(c_string.as_ptr()) };
        
        if handle.is_null() {
            Err("数据库连接失败".into())
        } else {
            Ok(DatabaseConnection {
                handle,
                driver: self,
            })
        }
    }
}

struct DatabaseConnection<'a> {
    handle: *mut (),
    driver: &'a DatabaseDriver,
}

impl<'a> DatabaseConnection<'a> {
    fn execute(&self, query: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let c_string = std::ffi::CString::new(query)?;
        let result = unsafe { (self.driver.execute)(self.handle, c_string.as_ptr()) };
        Ok(result)
    }
}

impl<'a> Drop for DatabaseConnection<'a> {
    fn drop(&mut self) {
        unsafe { (self.driver.disconnect)(self.handle) };
    }
}

fn database_driver_example() -> Result<(), Box<dyn std::error::Error>> {
    // 加载数据库驱动（假设存在）
    match DatabaseDriver::new("libdb_driver.so") {
        Ok(driver) => {
            println!("数据库驱动加载成功");
            
            match driver.connect("host=localhost;database=test") {
                Ok(conn) => {
                    println!("数据库连接成功");
                    
                    let result = conn.execute("SELECT * FROM users")?;
                    println!("查询结果: {}", result);
                    
                    // 连接会在 drop 时自动关闭
                }
                Err(e) => {
                    println!("数据库连接失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("数据库驱动加载失败: {}", e);
        }
    }
    
    Ok(())
}
```

### 2. 图形插件系统

```rust
use libloading::{Library, Symbol};

// 图形渲染器接口
trait GraphicsRenderer {
    fn init(&mut self) -> Result<(), String>;
    fn render(&self, data: &[u8]) -> Result<(), String>;
    fn shutdown(&mut self);
}

// 渲染器创建函数
type CreateRenderer = unsafe extern fn() -> *mut dyn GraphicsRenderer;

struct GraphicsSystem {
    renderer_lib: Option<Library>,
    renderer: Option<Box<dyn GraphicsRenderer>>,
}

impl GraphicsSystem {
    fn new() -> Self {
        GraphicsSystem {
            renderer_lib: None,
            renderer: None,
        }
    }
    
    fn load_renderer(&mut self, renderer_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(renderer_path) }?;
        
        let create_renderer: Symbol<CreateRenderer> = unsafe { lib.get(b"create_renderer") }?;
        
        let renderer_ptr = unsafe { create_renderer() };
        let mut renderer = unsafe { Box::from_raw(renderer_ptr) };
        
        renderer.init().map_err(|e| format!("渲染器初始化失败: {}", e))?;
        
        self.renderer_lib = Some(lib);
        self.renderer = Some(renderer);
        
        Ok(())
    }
    
    fn render(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(renderer) = &self.renderer {
            renderer.render(data).map_err(|e| e.into())
        } else {
            Err("渲染器未加载".into())
        }
    }
}

impl Drop for GraphicsSystem {
    fn drop(&mut self) {
        if let Some(mut renderer) = self.renderer.take() {
            renderer.shutdown();
        }
    }
}

fn graphics_system_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut graphics = GraphicsSystem::new();
    
    // 尝试加载不同的渲染器
    let renderers = ["libvulkan_renderer.so", "libopengl_renderer.so", "libdx12_renderer.so"];
    
    for renderer_path in &renderers {
        match graphics.load_renderer(renderer_path) {
            Ok(()) => {
                println!("渲染器 {} 加载成功", renderer_path);
                
                let sample_data = vec![0u8; 1024]; // 示例数据
                match graphics.render(&sample_data) {
                    Ok(()) => {
                        println!("渲染成功");
                    }
                    Err(e) => {
                        println!("渲染失败: {}", e);
                    }
                }
                
                break;
            }
            Err(e) => {
                println!("渲染器 {} 加载失败: {}", renderer_path, e);
            }
        }
    }
    
    Ok(())
}
```

### 3. 脚本语言集成

```rust
use libloading::{Library, Symbol};

// 脚本引擎接口
trait ScriptEngine {
    fn execute(&self, script: &str) -> Result<String, String>;
    fn call_function(&self, name: &str, args: &[&str]) -> Result<String, String>;
}

// 脚本引擎创建函数
type CreateScriptEngine = unsafe extern fn() -> *mut dyn ScriptEngine;

struct ScriptManager {
    engines: std::collections::HashMap<String, (Library, Box<dyn ScriptEngine>)>,
}

impl ScriptManager {
    fn new() -> Self {
        ScriptManager {
            engines: std::collections::HashMap::new(),
        }
    }
    
    fn load_engine(&mut self, name: &str, library_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(library_path) }?;
        
        let create_engine: Symbol<CreateScriptEngine> = unsafe { lib.get(b"create_script_engine") }?;
        
        let engine_ptr = unsafe { create_engine() };
        let engine = unsafe { Box::from_raw(engine_ptr) };
        
        self.engines.insert(name.to_string(), (lib, engine));
        
        Ok(())
    }
    
    fn execute_script(&self, engine_name: &str, script: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some((_, engine)) = self.engines.get(engine_name) {
            engine.execute(script).map_err(|e| e.into())
        } else {
            Err(format!("脚本引擎 {} 未找到", engine_name).into())
        }
    }
    
    fn call_function(&self, engine_name: &str, function_name: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        if let Some((_, engine)) = self.engines.get(engine_name) {
            engine.call_function(function_name, args).map_err(|e| e.into())
        } else {
            Err(format!("脚本引擎 {} 未找到", engine_name).into())
        }
    }
}

fn script_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ScriptManager::new();
    
    // 加载不同的脚本引擎
    let engines = [
        ("lua", "liblua_engine.so"),
        ("python", "libpython_engine.so"),
        ("javascript", "libjs_engine.so"),
    ];
    
    for (name, path) in &engines {
        match manager.load_engine(name, path) {
            Ok(()) => {
                println!("脚本引擎 {} 加载成功", name);
                
                // 执行脚本
                let script = "function greet(name) { return 'Hello, ' + name + '!'; }";
                match manager.execute_script(name, script) {
                    Ok(result) => {
                        println!("脚本执行结果: {}", result);
                        
                        // 调用函数
                        match manager.call_function(name, "greet", &["World"]) {
                            Ok(result) => {
                                println!("函数调用结果: {}", result);
                            }
                            Err(e) => {
                                println!("函数调用失败: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("脚本执行失败: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("脚本引擎 {} 加载失败: {}", name, e);
            }
        }
    }
    
    Ok(())
}
```

## 安全考虑

### 1. 内存安全

```rust
use libloading::{Library, Symbol};

fn memory_safety_example() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 确保符号的生命周期不超过库的生命周期
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 错误示例：符号离开作用域但库仍然存在
    // let symbol = {
    //     let sin: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"sin") }?;
    //     sin // 这里会发生错误，因为 Symbol 不能超出 Library 的生命周期
    // };
    
    // 正确示例：在同一作用域内使用
    let sin: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"sin") }?;
    let result = unsafe { sin(0.5) };
    println!("sin(0.5) = {}", result);
    
    // 2. 使用 RAII 模式管理资源
    struct SafeLibrary {
        lib: Library,
    }
    
    impl SafeLibrary {
        fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let lib = unsafe { Library::new(path) }?;
            Ok(SafeLibrary { lib })
        }
        
        fn call_function(&self, name: &[u8], arg: f64) -> Result<f64, Box<dyn std::error::Error>> {
            let func: Symbol<unsafe extern fn(f64) -> f64> = unsafe { self.lib.get(name) }?;
            Ok(unsafe { func(arg) })
        }
    }
    
    let safe_lib = SafeLibrary::new("libm.so.6")?;
    let result = safe_lib.call_function(b"cos", 0.0)?;
    println!("cos(0.0) = {}", result);
    
    Ok(())
}
```

### 2. 错误处理和验证

```rust
use libloading::{Library, Symbol};

fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    // 验证库文件是否存在
    let library_path = "libm.so.6";
    
    if !std::path::Path::new(library_path).exists() {
        return Err(format!("库文件 {} 不存在", library_path).into());
    }
    
    // 安全地加载库
    let lib = unsafe { Library::new(library_path) }?;
    
    // 验证必需的符号是否存在
    let required_symbols = ["sin", "cos", "tan"];
    
    for symbol_name in &required_symbols {
        match unsafe { lib.get::<unsafe extern fn(f64) -> f64>(symbol_name.as_bytes()) } {
            Ok(_) => {
                println!("符号 {} 存在", symbol_name);
            }
            Err(e) => {
                return Err(format!("必需的符号 {} 不存在: {}", symbol_name, e).into());
            }
        }
    }
    
    println!("所有必需的符号都存在");
    Ok(())
}
```

## 调试和诊断

### 1. 调试工具

```rust
use libloading::{Library, Symbol};

fn debugging_example() -> Result<(), Box<dyn std::error::Error>> {
    // 启用调试输出
    println!("开始调试动态库加载");
    
    let library_path = "libm.so.6";
    println!("尝试加载库: {}", library_path);
    
    match unsafe { Library::new(library_path) } {
        Ok(lib) => {
            println!("库加载成功");
            
            // 尝试获取符号
            let symbol_name = b"sin";
            println!("尝试获取符号: {}", String::from_utf8_lossy(symbol_name));
            
            match unsafe { lib.get::<unsafe extern fn(f64) -> f64>(symbol_name) } {
                Ok(symbol) => {
                    println!("符号获取成功");
                    
                    // 测试符号调用
                    let test_value = 1.0;
                    let result = unsafe { symbol(test_value) };
                    println!("符号调用结果: sin({}) = {}", test_value, result);
                }
                Err(e) => {
                    println!("符号获取失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("库加载失败: {}", e);
            
            // 尝试获取更多错误信息
            match e {
                libloading::Error::DlOpen { desc } => {
                    println!("DlOpen 错误详情: {}", desc);
                }
                _ => {
                    println!("其他错误类型");
                }
            }
        }
    }
    
    Ok(())
}
```

### 2. 性能监控

```rust
use libloading::{Library, Symbol};
use std::time::Instant;

fn performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // 监控库加载时间
    let start = Instant::now();
    let lib = unsafe { Library::new("libm.so.6") }?;
    let load_time = start.elapsed();
    println!("库加载时间: {:?}", load_time);
    
    // 监控符号获取时间
    let start = Instant::now();
    let sin: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"sin") }?;
    let symbol_time = start.elapsed();
    println!("符号获取时间: {:?}", symbol_time);
    
    // 监控函数调用时间
    let start = Instant::now();
    let result = unsafe { sin(1.0) };
    let call_time = start.elapsed();
    println!("函数调用时间: {:?}", call_time);
    println!("计算结果: {}", result);
    
    // 批量测试性能
    let iterations = 1000000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = unsafe { sin(1.0) };
    }
    
    let batch_time = start.elapsed();
    println!("批量调用 {} 次，总时间: {:?}", iterations, batch_time);
    println!("平均调用时间: {:?}", batch_time / iterations);
    
    Ok(())
}
```

## 最佳实践

1. **安全第一**: 始终在 `unsafe` 块中处理动态库操作
2. **生命周期管理**: 确保符号的生命周期不超过库的生命周期
3. **错误处理**: 妥善处理库加载和符号获取的错误
4. **跨平台兼容**: 考虑不同平台的库文件命名约定
5. **资源管理**: 使用 RAII 模式管理动态库资源

## 常见问题

### 1. 符号未找到

```rust
use libloading::{Library, Symbol};

fn symbol_not_found_solutions() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libm.so.6") }?;
    
    // 问题：符号名称错误
    // 解决：使用正确的符号名称
    match unsafe { lib.get::<unsafe extern fn(f64) -> f64>(b"sine") } {
        Ok(_) => {},
        Err(e) => {
            println!("符号 'sine' 不存在: {}", e);
            
            // 使用正确的符号名称
            let sin: Symbol<unsafe extern fn(f64) -> f64> = unsafe { lib.get(b"sin") }?;
            println!("使用正确的符号名称 'sin' 成功");
        }
    }
    
    Ok(())
}
```

### 2. 库加载失败

```rust
use libloading::Library;

fn library_load_failure_solutions() {
    // 问题：库路径错误
    // 解决：使用正确的路径或库名
    let library_attempts = [
        "libm.so.6",           // 完整版本号
        "libm.so",             // 不带版本号
        "/lib/x86_64-linux-gnu/libm.so.6",  // 绝对路径
    ];
    
    for library_path in &library_attempts {
        match unsafe { Library::new(library_path) } {
            Ok(lib) => {
                println!("库 {} 加载成功", library_path);
                break;
            }
            Err(e) => {
                println!("库 {} 加载失败: {}", library_path, e);
            }
        }
    }
}
```

## 总结

`libloading` 是一个强大的动态库加载库，提供了：

- **跨平台支持**: 支持 Windows、Linux 和 macOS
- **内存安全**: 提供安全的接口来处理动态库
- **灵活性**: 支持运行时加载和卸载库
- **性能**: 高效的符号查找和调用机制
- **错误处理**: 详细的错误信息和处理机制

通过掌握 `libloading`，您可以构建灵活的插件系统、动态加载驱动程序、集成脚本语言等，为 Rust 应用程序提供强大的运行时扩展能力。但请注意，使用动态库加载需要特别注意内存安全和错误处理。