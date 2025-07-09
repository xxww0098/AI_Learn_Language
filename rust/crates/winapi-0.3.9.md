# winapi 0.3.9 中文使用教程

## 概述

winapi 是 Windows API 的原始 FFI（Foreign Function Interface）绑定库，为 Rust 提供了直接访问 Windows 系统 API 的能力。它包含了几乎所有的 Windows API 函数、结构体、常量和类型定义。

**版本**: 0.3.9
**许可证**: MIT/Apache-2.0
**仓库**: https://github.com/retep998/winapi-rs
**文档**: https://docs.rs/winapi/

## 主要特性

- 🔗 **完整绑定**: 覆盖几乎所有 Windows API
- 🚀 **零成本**: 直接 FFI 绑定，无性能损失
- 🎯 **类型安全**: 提供类型安全的 Windows API 访问
- 📦 **模块化**: 按功能模块组织，可选择性导入
- 🛠️ **工具集成**: 与 Rust 工具链良好集成

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
winapi = { version = "0.3.9", features = ["winuser", "wincon", "winbase", "fileapi", "processthreadsapi"] }

# 或者使用特定功能
[dependencies]
winapi = { version = "0.3.9", features = ["everything"] }
```

## 常用功能模块

### 1. 核心模块

```rust
use winapi::um::{
    winuser::*,    // 用户界面 API
    wincon::*,     // 控制台 API
    winbase::*,    // 基础 API
    fileapi::*,    // 文件 API
    processthreadsapi::*, // 进程线程 API
    libloaderapi::*, // 库加载 API
    errhandlingapi::*, // 错误处理 API
    handleapi::*,  // 句柄 API
    synchapi::*,   // 同步 API
    memoryapi::*,  // 内存 API
};
```

### 2. 类型定义

```rust
use winapi::ctypes::*;
use winapi::shared::{
    windef::*,     // 基本类型定义
    winerror::*,   // 错误码
    minwindef::*,  // 最小定义
    ntdef::*,      // NT 定义
    basetsd::*,    // 基础大小类型
};
```

## 基本用法

### 1. 文件操作

```rust
use winapi::um::{
    fileapi::*,
    handleapi::*,
    errhandlingapi::*,
    winbase::*,
};
use winapi::shared::winerror::*;
use winapi::ctypes::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

fn create_file_example() -> Result<(), Box<dyn std::error::Error>> {
    // 将路径转换为 Windows 宽字符
    let path = OsStr::new("test.txt");
    let wide_path: Vec<u16> = path.encode_wide().chain(Some(0)).collect();
    
    unsafe {
        // 创建文件
        let handle = CreateFileW(
            wide_path.as_ptr(),
            GENERIC_WRITE,
            FILE_SHARE_READ,
            ptr::null_mut(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        );
        
        if handle == INVALID_HANDLE_VALUE {
            let error = GetLastError();
            println!("创建文件失败，错误码: {}", error);
            return Err(format!("创建文件失败: {}", error).into());
        }
        
        // 写入数据
        let data = b"Hello, Windows API!";
        let mut bytes_written = 0;
        
        let result = WriteFile(
            handle,
            data.as_ptr() as *const c_void,
            data.len() as u32,
            &mut bytes_written,
            ptr::null_mut(),
        );
        
        if result == 0 {
            let error = GetLastError();
            println!("写入文件失败，错误码: {}", error);
        } else {
            println!("成功写入 {} 字节", bytes_written);
        }
        
        // 关闭句柄
        CloseHandle(handle);
    }
    
    Ok(())
}
```

### 2. 进程管理

```rust
use winapi::um::{
    processthreadsapi::*,
    winbase::*,
    handleapi::*,
    tlhelp32::*,
    errhandlingapi::*,
};
use winapi::shared::minwindef::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

fn create_process_example() -> Result<(), Box<dyn std::error::Error>> {
    let command = CString::new("notepad.exe").unwrap();
    
    unsafe {
        let mut si: STARTUPINFOA = mem::zeroed();
        si.cb = mem::size_of::<STARTUPINFOA>() as u32;
        
        let mut pi: PROCESS_INFORMATION = mem::zeroed();
        
        let result = CreateProcessA(
            ptr::null(),
            command.as_ptr() as *mut i8,
            ptr::null_mut(),
            ptr::null_mut(),
            FALSE,
            0,
            ptr::null_mut(),
            ptr::null(),
            &mut si,
            &mut pi,
        );
        
        if result == 0 {
            let error = GetLastError();
            return Err(format!("创建进程失败: {}", error).into());
        }
        
        println!("进程创建成功，PID: {}", pi.dwProcessId);
        
        // 等待进程结束
        WaitForSingleObject(pi.hProcess, INFINITE);
        
        // 关闭句柄
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
    
    Ok(())
}

fn enum_processes() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut processes = Vec::new();
    
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err("创建进程快照失败".into());
        }
        
        let mut pe32: PROCESSENTRY32 = mem::zeroed();
        pe32.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;
        
        if Process32First(snapshot, &mut pe32) != 0 {
            loop {
                processes.push(pe32.th32ProcessID);
                
                if Process32Next(snapshot, &mut pe32) == 0 {
                    break;
                }
            }
        }
        
        CloseHandle(snapshot);
    }
    
    Ok(processes)
}
```

### 3. 窗口操作

```rust
use winapi::um::winuser::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use std::ffi::CString;
use std::ptr;

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows: &mut Vec<HWND> = &mut *(lparam as *mut Vec<HWND>);
    windows.push(hwnd);
    TRUE
}

fn enum_windows_example() -> Result<Vec<HWND>, Box<dyn std::error::Error>> {
    let mut windows = Vec::new();
    
    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            &mut windows as *mut _ as LPARAM,
        );
    }
    
    Ok(windows)
}

fn get_window_text(hwnd: HWND) -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        let length = GetWindowTextLengthA(hwnd);
        if length == 0 {
            return Ok(String::new());
        }
        
        let mut buffer = vec![0u8; (length + 1) as usize];
        let result = GetWindowTextA(hwnd, buffer.as_mut_ptr() as *mut i8, length + 1);
        
        if result == 0 {
            return Err("获取窗口文本失败".into());
        }
        
        buffer.truncate(result as usize);
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}

fn find_window_by_title(title: &str) -> Option<HWND> {
    let title_cstring = CString::new(title).ok()?;
    
    unsafe {
        let hwnd = FindWindowA(ptr::null(), title_cstring.as_ptr());
        if hwnd.is_null() {
            None
        } else {
            Some(hwnd)
        }
    }
}
```

### 4. 注册表操作

```rust
use winapi::um::winreg::*;
use winapi::shared::winerror::*;
use winapi::shared::minwindef::*;
use std::ffi::CString;
use std::ptr;
use std::mem;

fn registry_example() -> Result<(), Box<dyn std::error::Error>> {
    let subkey = CString::new("SOFTWARE\\MyApp").unwrap();
    let mut hkey: HKEY = ptr::null_mut();
    
    unsafe {
        // 创建或打开注册表键
        let result = RegCreateKeyExA(
            HKEY_CURRENT_USER,
            subkey.as_ptr(),
            0,
            ptr::null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_ALL_ACCESS,
            ptr::null_mut(),
            &mut hkey,
            ptr::null_mut(),
        );
        
        if result != ERROR_SUCCESS {
            return Err(format!("创建注册表键失败: {}", result).into());
        }
        
        // 写入字符串值
        let value_name = CString::new("Version").unwrap();
        let value_data = CString::new("1.0.0").unwrap();
        
        let result = RegSetValueExA(
            hkey,
            value_name.as_ptr(),
            0,
            REG_SZ,
            value_data.as_ptr() as *const u8,
            value_data.as_bytes().len() as u32 + 1,
        );
        
        if result != ERROR_SUCCESS {
            RegCloseKey(hkey);
            return Err(format!("设置注册表值失败: {}", result).into());
        }
        
        // 读取字符串值
        let mut buffer = vec![0u8; 256];
        let mut buffer_size = buffer.len() as u32;
        let mut value_type: u32 = 0;
        
        let result = RegQueryValueExA(
            hkey,
            value_name.as_ptr(),
            ptr::null_mut(),
            &mut value_type,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        );
        
        if result == ERROR_SUCCESS && value_type == REG_SZ {
            buffer.truncate(buffer_size as usize - 1); // 移除空终止符
            let value = String::from_utf8_lossy(&buffer);
            println!("读取到的值: {}", value);
        }
        
        RegCloseKey(hkey);
    }
    
    Ok(())
}
```

### 5. 系统信息

```rust
use winapi::um::sysinfoapi::*;
use winapi::um::winbase::*;
use winapi::shared::minwindef::*;
use std::mem;

fn get_system_info() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut system_info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(&mut system_info);
        
        println!("处理器架构: {}", system_info.wProcessorArchitecture);
        println!("处理器数量: {}", system_info.dwNumberOfProcessors);
        println!("页面大小: {}", system_info.dwPageSize);
        println!("最小应用程序地址: 0x{:x}", system_info.lpMinimumApplicationAddress as usize);
        println!("最大应用程序地址: 0x{:x}", system_info.lpMaximumApplicationAddress as usize);
        
        // 获取内存状态
        let mut mem_status: MEMORYSTATUSEX = mem::zeroed();
        mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            println!("内存使用率: {}%", mem_status.dwMemoryLoad);
            println!("总物理内存: {} MB", mem_status.ullTotalPhys / 1024 / 1024);
            println!("可用物理内存: {} MB", mem_status.ullAvailPhys / 1024 / 1024);
            println!("总虚拟内存: {} MB", mem_status.ullTotalVirtual / 1024 / 1024);
            println!("可用虚拟内存: {} MB", mem_status.ullAvailVirtual / 1024 / 1024);
        }
        
        // 获取计算机名称
        let mut computer_name = vec![0u8; MAX_COMPUTERNAME_LENGTH as usize + 1];
        let mut size = computer_name.len() as u32;
        
        if GetComputerNameA(computer_name.as_mut_ptr() as *mut i8, &mut size) != 0 {
            computer_name.truncate(size as usize);
            println!("计算机名称: {}", String::from_utf8_lossy(&computer_name));
        }
    }
    
    Ok(())
}
```

## 高级用法

### 1. 窗口消息处理

```rust
use winapi::um::winuser::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use std::ffi::CString;
use std::ptr;

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            let text = CString::new("Hello, Windows!").unwrap();
            TextOutA(hdc, 10, 10, text.as_ptr(), text.as_bytes().len() as i32);
            
            EndPaint(hwnd, &ps);
            0
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

fn create_window_example() -> Result<(), Box<dyn std::error::Error>> {
    let class_name = CString::new("MyWindowClass").unwrap();
    let window_name = CString::new("My Window").unwrap();
    
    unsafe {
        let hinstance = GetModuleHandleA(ptr::null());
        
        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconA(ptr::null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorA(ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        
        if RegisterClassA(&wc) == 0 {
            return Err("注册窗口类失败".into());
        }
        
        let hwnd = CreateWindowExA(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            ptr::null_mut(),
            ptr::null_mut(),
            hinstance,
            ptr::null_mut(),
        );
        
        if hwnd.is_null() {
            return Err("创建窗口失败".into());
        }
        
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    
    Ok(())
}
```

### 2. 服务操作

```rust
use winapi::um::winsvc::*;
use winapi::um::handleapi::*;
use winapi::shared::winerror::*;
use std::ffi::CString;
use std::ptr;
use std::mem;

fn service_example() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let sc_manager = OpenSCManagerA(ptr::null(), ptr::null(), SC_MANAGER_ALL_ACCESS);
        if sc_manager.is_null() {
            return Err("打开服务控制管理器失败".into());
        }
        
        let service_name = CString::new("Spooler").unwrap();
        let service = OpenServiceA(sc_manager, service_name.as_ptr(), SERVICE_ALL_ACCESS);
        
        if service.is_null() {
            CloseServiceHandle(sc_manager);
            return Err("打开服务失败".into());
        }
        
        // 查询服务状态
        let mut status: SERVICE_STATUS = mem::zeroed();
        if QueryServiceStatus(service, &mut status) != 0 {
            println!("服务状态: {}", match status.dwCurrentState {
                SERVICE_RUNNING => "运行中",
                SERVICE_STOPPED => "已停止",
                SERVICE_PAUSED => "已暂停",
                _ => "未知状态",
            });
        }
        
        // 获取服务配置
        let mut bytes_needed = 0;
        QueryServiceConfigA(service, ptr::null_mut(), 0, &mut bytes_needed);
        
        if bytes_needed > 0 {
            let mut buffer = vec![0u8; bytes_needed as usize];
            let config = buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGA;
            
            if QueryServiceConfigA(service, config, bytes_needed, &mut bytes_needed) != 0 {
                println!("服务类型: {}", (*config).dwServiceType);
                println!("启动类型: {}", (*config).dwStartType);
            }
        }
        
        CloseServiceHandle(service);
        CloseServiceHandle(sc_manager);
    }
    
    Ok(())
}
```

### 3. 性能监控

```rust
use winapi::um::pdh::*;
use winapi::um::winbase::*;
use winapi::shared::winerror::*;
use std::ffi::CString;
use std::ptr;
use std::mem;

fn performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut query: PDH_HQUERY = ptr::null_mut();
        let mut counter: PDH_HCOUNTER = ptr::null_mut();
        
        // 创建查询
        let result = PdhOpenQueryA(ptr::null(), 0, &mut query);
        if result != ERROR_SUCCESS {
            return Err(format!("创建 PDH 查询失败: {}", result).into());
        }
        
        // 添加计数器（CPU 使用率）
        let counter_path = CString::new("\\Processor(_Total)\\% Processor Time").unwrap();
        let result = PdhAddCounterA(query, counter_path.as_ptr(), 0, &mut counter);
        if result != ERROR_SUCCESS {
            PdhCloseQuery(query);
            return Err(format!("添加计数器失败: {}", result).into());
        }
        
        // 收集初始样本
        PdhCollectQueryData(query);
        
        // 等待一秒钟
        Sleep(1000);
        
        // 收集第二个样本
        PdhCollectQueryData(query);
        
        // 获取格式化值
        let mut counter_value: PDH_FMT_COUNTERVALUE = mem::zeroed();
        let result = PdhGetFormattedCounterValue(
            counter,
            PDH_FMT_DOUBLE,
            ptr::null_mut(),
            &mut counter_value,
        );
        
        if result == ERROR_SUCCESS {
            println!("CPU 使用率: {:.2}%", counter_value.u.doubleValue());
        }
        
        PdhCloseQuery(query);
    }
    
    Ok(())
}
```

## 错误处理

### 1. 系统错误

```rust
use winapi::um::errhandlingapi::*;
use winapi::um::winbase::*;
use winapi::shared::minwindef::*;
use std::ptr;
use std::ffi::CStr;

fn get_last_error_message() -> String {
    unsafe {
        let error_code = GetLastError();
        let mut buffer: *mut u8 = ptr::null_mut();
        
        let length = FormatMessageA(
            FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
            ptr::null(),
            error_code,
            0,
            &mut buffer as *mut _ as *mut i8,
            0,
            ptr::null_mut(),
        );
        
        if length == 0 {
            return format!("未知错误: {}", error_code);
        }
        
        let message = CStr::from_ptr(buffer as *const i8)
            .to_string_lossy()
            .to_string();
        
        LocalFree(buffer as *mut _);
        message.trim().to_string()
    }
}

fn handle_error(operation: &str) -> Result<(), Box<dyn std::error::Error>> {
    let error_message = get_last_error_message();
    Err(format!("{} 失败: {}", operation, error_message).into())
}
```

### 2. 安全包装

```rust
use winapi::um::handleapi::*;
use winapi::shared::minwindef::*;
use std::ops::Drop;

struct SafeHandle {
    handle: HANDLE,
}

impl SafeHandle {
    fn new(handle: HANDLE) -> Result<Self, Box<dyn std::error::Error>> {
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return Err("无效句柄".into());
        }
        Ok(SafeHandle { handle })
    }
    
    fn get(&self) -> HANDLE {
        self.handle
    }
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() && self.handle != INVALID_HANDLE_VALUE {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}
```

## 最佳实践

1. **安全性**: 始终检查返回值和错误码
2. **资源管理**: 使用 RAII 模式管理 Windows 资源
3. **字符串处理**: 正确处理 Unicode 和 ANSI 字符串
4. **错误处理**: 实现完善的错误处理机制
5. **内存管理**: 注意内存分配和释放

## 常见问题

### 1. 字符串转换

```rust
use std::ffi::{OsString, OsStr};
use std::os::windows::ffi::{OsStringExt, OsStrExt};

fn string_conversion_examples() {
    // Rust String 到 Windows 宽字符
    let rust_string = "Hello, 世界!";
    let wide_string: Vec<u16> = OsStr::new(rust_string)
        .encode_wide()
        .chain(Some(0))
        .collect();
    
    // Windows 宽字符到 Rust String
    let os_string = OsString::from_wide(&wide_string[..wide_string.len() - 1]);
    let back_to_rust = os_string.to_string_lossy();
    
    println!("原始: {}", rust_string);
    println!("转换后: {}", back_to_rust);
}
```

### 2. 句柄管理

```rust
use winapi::um::handleapi::*;
use winapi::shared::minwindef::*;

struct HandleWrapper(HANDLE);

impl HandleWrapper {
    fn new(handle: HANDLE) -> Option<Self> {
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            None
        } else {
            Some(HandleWrapper(handle))
        }
    }
}

impl Drop for HandleWrapper {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
```

### 3. 异步操作

```rust
use winapi::um::ioapiset::*;
use winapi::um::minwinbase::*;
use std::mem;
use std::ptr;

fn async_file_operation() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut overlapped: OVERLAPPED = mem::zeroed();
        let event = CreateEventA(ptr::null_mut(), TRUE, FALSE, ptr::null());
        overlapped.hEvent = event;
        
        // 异步操作示例
        // 实际实现需要更多的错误处理和同步
        
        CloseHandle(event);
    }
    
    Ok(())
}
```

## 总结

winapi 是访问 Windows API 的底层库，提供了完整的 Windows API 绑定。虽然使用起来需要更多的注意事项，但它提供了最直接和高效的 Windows 系统编程能力。

使用 winapi 时需要特别注意：
- 内存安全和资源管理
- 错误处理和异常情况
- 字符串编码转换
- 并发和线程安全

更多详细信息请参考：
- [winapi 官方文档](https://docs.rs/winapi/)
- [Windows API 文档](https://docs.microsoft.com/en-us/windows/win32/api/)
- [GitHub 仓库](https://github.com/retep998/winapi-rs)