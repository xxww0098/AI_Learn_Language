# Tauri 2.6.2 中文使用教程

## 概述

Tauri 是一个用于构建现代桌面应用程序的框架，允许开发者使用 Web 技术（HTML、CSS、JavaScript）创建原生桌面应用。Tauri 提供了小巧、快速且安全的桌面应用解决方案。

**版本**: 2.6.2
**许可证**: Apache-2.0 OR MIT
**仓库**: https://github.com/tauri-apps/tauri
**主页**: https://tauri.app/

## 主要特性

- 🚀 **轻量级**: 应用包体积极小
- 🔒 **安全**: 内置安全机制，防止恶意代码执行
- 🎨 **现代化**: 支持现代 Web 技术栈
- 🌐 **跨平台**: 支持 Windows、macOS、Linux
- ⚡ **高性能**: 基于 Rust 构建，性能优异

## 安装

### 前置要求

1. 安装 Rust 开发环境
2. 安装 Node.js 和 npm/yarn
3. 安装系统依赖

### 快速开始

```bash
# 创建新的 Tauri 项目
npm create tauri-app@latest

# 或者使用 Cargo
cargo install create-tauri-app
cargo create-tauri-app
```

## 基本用法

### 1. 项目结构

```
my-tauri-app/
├── src/           # Rust 后端代码
│   ├── main.rs    # 主程序入口
│   └── lib.rs     # 库文件
├── src-tauri/     # Tauri 配置
│   ├── Cargo.toml
│   └── tauri.conf.json
├── dist/          # 前端构建输出
└── package.json
```

### 2. 基本配置

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tauri = { version = "2.6.2", features = ["shell-open"] }
```

### 3. 主程序设置

```rust
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

## 核心功能

### 1. 窗口管理

```rust
use tauri::{Manager, WindowBuilder, WindowUrl};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建新窗口
            let window = WindowBuilder::new(
                app,
                "main",
                WindowUrl::App("index.html".into())
            )
            .title("我的应用")
            .inner_size(800.0, 600.0)
            .min_inner_size(400.0, 300.0)
            .resizable(true)
            .build()?;

            // 窗口事件处理
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        // 自定义关闭逻辑
                        println!("窗口即将关闭");
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. 系统托盘

```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "显示"))
        .add_item(CustomMenuItem::new("hide", "隐藏"))
        .add_separator()
        .add_item(CustomMenuItem::new("quit", "退出"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "show" => {
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                        }
                        "hide" => {
                            let window = app.get_window("main").unwrap();
                            window.hide().unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. 文件系统访问

```rust
use tauri::api::file::{read_string, write_file};
use std::path::Path;

#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    read_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_file(path: String, content: String) -> Result<(), String> {
    write_file(Path::new(&path), content).map_err(|e| e.to_string())
}
```

### 4. 数据库集成

```rust
use tauri::State;
use std::sync::Mutex;

struct AppState {
    db: Mutex<Option<rusqlite::Connection>>,
}

#[tauri::command]
fn initialize_database(state: State<AppState>) -> Result<(), String> {
    let conn = rusqlite::Connection::open("app.db")
        .map_err(|e| e.to_string())?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    ).map_err(|e| e.to_string())?;

    *state.db.lock().unwrap() = Some(conn);
    Ok(())
}

#[tauri::command]
fn add_user(name: String, email: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        [&name, &email],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}
```

## 前端集成

### JavaScript 调用

```javascript
// 前端调用 Rust 函数
import { invoke } from '@tauri-apps/api/tauri';

async function greet() {
    const name = "世界";
    const result = await invoke('greet', { name });
    console.log(result);
}

// 文件操作
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

async function saveFile() {
    try {
        await writeTextFile('config.json', JSON.stringify({ theme: 'dark' }));
        console.log('文件已保存');
    } catch (error) {
        console.error('保存失败:', error);
    }
}

async function loadFile() {
    try {
        const content = await readTextFile('config.json');
        const config = JSON.parse(content);
        console.log('配置:', config);
    } catch (error) {
        console.error('读取失败:', error);
    }
}
```

### 窗口控制

```javascript
import { appWindow } from '@tauri-apps/api/window';

// 最小化窗口
await appWindow.minimize();

// 最大化窗口
await appWindow.maximize();

// 关闭窗口
await appWindow.close();

// 监听窗口事件
appWindow.listen('tauri://close-requested', () => {
    console.log('窗口即将关闭');
});
```

## 构建和打包

### 开发模式

```bash
# 启动开发服务器
npm run tauri dev

# 或者
cargo tauri dev
```

### 生产构建

```bash
# 构建应用
npm run tauri build

# 或者
cargo tauri build
```

### 构建配置

在 `tauri.conf.json` 中配置：

```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "distDir": "../dist"
  },
  "package": {
    "productName": "我的应用",
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["deb", "msi", "app", "dmg"],
      "identifier": "com.example.myapp",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.ico"
      ]
    }
  }
}
```

## 插件系统

### 使用官方插件

```rust
use tauri_plugin_store::StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 创建自定义插件

```rust
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("my-plugin")
        .invoke_handler(tauri::generate_handler![my_command])
        .build()
}

#[tauri::command]
fn my_command() -> String {
    "Hello from plugin!".to_string()
}
```

## 安全配置

### CSP 设置

```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
    }
  }
}
```

### 权限控制

```json
{
  "tauri": {
    "allowlist": {
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "scope": ["$APPDATA/myapp/*"]
      },
      "shell": {
        "all": false,
        "open": true
      }
    }
  }
}
```

## 最佳实践

1. **错误处理**: 始终使用适当的错误处理机制
2. **性能优化**: 合理使用异步操作，避免阻塞主线程
3. **安全性**: 严格配置 CSP 和权限控制
4. **用户体验**: 提供加载状态和错误反馈
5. **资源管理**: 正确管理文件句柄和系统资源

## 常见问题

### 1. 构建失败

- 检查 Rust 版本兼容性
- 确认系统依赖已安装
- 验证配置文件格式

### 2. 窗口显示问题

- 检查 HTML 文件路径
- 验证 CSP 设置
- 确认资源加载正确

### 3. 性能问题

- 优化前端资源加载
- 减少不必要的 API 调用
- 使用合适的数据结构

## 总结

Tauri 提供了一个强大而灵活的桌面应用开发框架，结合了 Web 技术的易用性和 Rust 的性能优势。通过合理的架构设计和配置，可以创建出高质量的桌面应用程序。

更多详细信息请参考：[Tauri 官方文档](https://tauri.app/)