# libc 0.2.174 详细中文使用教程

## 简介

`libc` 是 Rust 的标准 C 库绑定，提供了对平台原生 C 库函数的直接访问。它是系统编程、FFI (Foreign Function Interface) 和低级操作的基础库。

## 基本信息

- **版本**: 0.2.174
- **许可证**: MIT OR Apache-2.0
- **仓库**: https://github.com/rust-lang/libc
- **下载量**: 637,751,756 (极高人气)

## 快速开始

### 1. 添加依赖

在您的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
libc = "0.2.174"
```

### 2. 基本使用

```rust
extern crate libc;

use libc::{c_char, c_int, size_t};
use std::ffi::CString;

fn main() {
    // 使用 libc 的 strlen 函数
    let hello = CString::new("Hello, World!").unwrap();
    let len = unsafe {
        libc::strlen(hello.as_ptr())
    };
    println!("字符串长度: {}", len);
    
    // 使用 libc 的 getpid 函数
    let pid = unsafe { libc::getpid() };
    println!("进程 ID: {}", pid);
}
```

## 核心概念

### 1. 数据类型

`libc` 提供了 C 语言的所有基本数据类型：

```rust
use libc::{
    c_char, c_int, c_uint, c_long, c_ulong, c_short, c_ushort,
    c_float, c_double, c_void, size_t, ssize_t
};

fn type_examples() {
    // 基本类型
    let char_val: c_char = 65;      // 'A'
    let int_val: c_int = 42;
    let uint_val: c_uint = 42;
    let long_val: c_long = 1234567890;
    let ulong_val: c_ulong = 1234567890;
    let short_val: c_short = 100;
    let ushort_val: c_ushort = 100;
    let float_val: c_float = 3.14;
    let double_val: c_double = 3.14159;
    
    // 大小类型
    let size_val: size_t = 1024;
    let ssize_val: ssize_t = 1024;
    
    println!("C 类型示例完成");
}
```

### 2. 字符串处理

```rust
use libc::{c_char, strlen, strcmp, strcpy, strcat};
use std::ffi::{CString, CStr};

fn string_examples() {
    // 创建 C 字符串
    let hello = CString::new("Hello").unwrap();
    let world = CString::new("World").unwrap();
    
    // 使用 strlen
    let hello_len = unsafe { strlen(hello.as_ptr()) };
    println!("Hello 长度: {}", hello_len);
    
    // 使用 strcmp
    let cmp_result = unsafe { strcmp(hello.as_ptr(), world.as_ptr()) };
    println!("比较结果: {}", cmp_result);
    
    // 字符串拷贝和连接
    let mut buffer = vec![0i8; 100];
    unsafe {
        strcpy(buffer.as_mut_ptr(), hello.as_ptr());
        strcat(buffer.as_mut_ptr(), b" \0".as_ptr() as *const c_char);
        strcat(buffer.as_mut_ptr(), world.as_ptr());
    }
    
    let result = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    println!("连接结果: {:?}", result.to_str().unwrap());
}
```

### 3. 内存管理

```rust
use libc::{malloc, free, calloc, realloc, size_t, c_void};

fn memory_examples() {
    // 分配内存
    let size = 1024;
    let ptr = unsafe { malloc(size) };
    
    if ptr.is_null() {
        panic!("内存分配失败");
    }
    
    // 使用内存
    unsafe {
        *(ptr as *mut u8) = 42;
        println!("内存中的值: {}", *(ptr as *mut u8));
    }
    
    // 重新分配
    let new_size = 2048;
    let new_ptr = unsafe { realloc(ptr, new_size) };
    
    if new_ptr.is_null() {
        panic!("内存重新分配失败");
    }
    
    // 释放内存
    unsafe { free(new_ptr) };
    
    // 使用 calloc 分配清零内存
    let count = 10;
    let elem_size = std::mem::size_of::<i32>();
    let zero_ptr = unsafe { calloc(count, elem_size) };
    
    if !zero_ptr.is_null() {
        unsafe { free(zero_ptr) };
    }
}
```

## 文件操作

### 1. 文件 I/O

```rust
use libc::{FILE, fopen, fclose, fread, fwrite, fseek, ftell, feof, ferror};
use std::ffi::CString;

fn file_operations() {
    let filename = CString::new("test.txt").unwrap();
    let mode = CString::new("w+").unwrap();
    
    // 打开文件
    let file = unsafe { fopen(filename.as_ptr(), mode.as_ptr()) };
    if file.is_null() {
        panic!("无法打开文件");
    }
    
    // 写入数据
    let data = b"Hello, File!\n";
    let written = unsafe {
        fwrite(data.as_ptr() as *const libc::c_void, 1, data.len(), file)
    };
    println!("写入字节数: {}", written);
    
    // 移动文件指针到开始
    unsafe { fseek(file, 0, libc::SEEK_SET) };
    
    // 读取数据
    let mut buffer = vec![0u8; 100];
    let read = unsafe {
        fread(buffer.as_mut_ptr() as *mut libc::c_void, 1, buffer.len(), file)
    };
    println!("读取字节数: {}", read);
    
    buffer.truncate(read);
    println!("读取内容: {}", String::from_utf8_lossy(&buffer));
    
    // 关闭文件
    unsafe { fclose(file) };
}
```

### 2. 文件描述符操作

```rust
use libc::{open, close, read, write, lseek, O_RDWR, O_CREAT, S_IRUSR, S_IWUSR};
use std::ffi::CString;

fn file_descriptor_operations() {
    let filename = CString::new("test_fd.txt").unwrap();
    
    // 打开文件
    let fd = unsafe {
        open(filename.as_ptr(), O_RDWR | O_CREAT, S_IRUSR | S_IWUSR)
    };
    
    if fd == -1 {
        panic!("无法打开文件");
    }
    
    // 写入数据
    let data = b"Hello, File Descriptor!\n";
    let written = unsafe {
        write(fd, data.as_ptr() as *const libc::c_void, data.len())
    };
    println!("写入字节数: {}", written);
    
    // 移动文件指针
    unsafe { lseek(fd, 0, libc::SEEK_SET) };
    
    // 读取数据
    let mut buffer = vec![0u8; 100];
    let read_bytes = unsafe {
        read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len())
    };
    
    if read_bytes > 0 {
        buffer.truncate(read_bytes as usize);
        println!("读取内容: {}", String::from_utf8_lossy(&buffer));
    }
    
    // 关闭文件
    unsafe { close(fd) };
}
```

## 进程和系统调用

### 1. 进程管理

```rust
use libc::{getpid, getppid, fork, wait, exit, WIFEXITED, WEXITSTATUS};

fn process_examples() {
    // 获取进程信息
    let pid = unsafe { getpid() };
    let ppid = unsafe { getppid() };
    println!("当前进程 ID: {}, 父进程 ID: {}", pid, ppid);
    
    // 创建子进程
    let child_pid = unsafe { fork() };
    
    match child_pid {
        -1 => {
            panic!("fork 失败");
        }
        0 => {
            // 子进程
            println!("子进程运行中");
            unsafe { exit(42) };
        }
        _ => {
            // 父进程
            println!("父进程等待子进程，子进程 PID: {}", child_pid);
            let mut status = 0;
            unsafe { wait(&mut status) };
            
            if unsafe { WIFEXITED(status) } {
                let exit_code = unsafe { WEXITSTATUS(status) };
                println!("子进程退出码: {}", exit_code);
            }
        }
    }
}
```

### 2. 环境变量

```rust
use libc::{getenv, setenv, unsetenv};
use std::ffi::{CString, CStr};

fn environment_examples() {
    // 获取环境变量
    let var_name = CString::new("PATH").unwrap();
    let path_ptr = unsafe { getenv(var_name.as_ptr()) };
    
    if !path_ptr.is_null() {
        let path = unsafe { CStr::from_ptr(path_ptr) };
        println!("PATH: {}", path.to_str().unwrap());
    }
    
    // 设置环境变量
    let var_name = CString::new("MY_VAR").unwrap();
    let var_value = CString::new("Hello").unwrap();
    let result = unsafe { setenv(var_name.as_ptr(), var_value.as_ptr(), 1) };
    
    if result == 0 {
        println!("环境变量设置成功");
        
        // 验证设置
        let value_ptr = unsafe { getenv(var_name.as_ptr()) };
        if !value_ptr.is_null() {
            let value = unsafe { CStr::from_ptr(value_ptr) };
            println!("MY_VAR: {}", value.to_str().unwrap());
        }
    }
    
    // 删除环境变量
    unsafe { unsetenv(var_name.as_ptr()) };
}
```

## 时间操作

### 1. 时间获取

```rust
use libc::{time, ctime, localtime, strftime, time_t, tm};
use std::ffi::CStr;

fn time_examples() {
    // 获取当前时间
    let current_time = unsafe { time(std::ptr::null_mut()) };
    println!("当前时间戳: {}", current_time);
    
    // 转换为可读格式
    let time_str = unsafe { ctime(&current_time) };
    if !time_str.is_null() {
        let time_cstr = unsafe { CStr::from_ptr(time_str) };
        println!("当前时间: {}", time_cstr.to_str().unwrap().trim());
    }
    
    // 使用 localtime
    let local_time = unsafe { localtime(&current_time) };
    if !local_time.is_null() {
        let tm_struct = unsafe { *local_time };
        println!("年: {}, 月: {}, 日: {}, 时: {}, 分: {}, 秒: {}",
                 tm_struct.tm_year + 1900,
                 tm_struct.tm_mon + 1,
                 tm_struct.tm_mday,
                 tm_struct.tm_hour,
                 tm_struct.tm_min,
                 tm_struct.tm_sec);
    }
    
    // 格式化时间
    let mut buffer = vec![0i8; 100];
    let format = b"%Y-%m-%d %H:%M:%S\0";
    let formatted_len = unsafe {
        strftime(buffer.as_mut_ptr(), 
                buffer.len(), 
                format.as_ptr() as *const libc::c_char, 
                local_time)
    };
    
    if formatted_len > 0 {
        buffer.truncate(formatted_len);
        let formatted = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        println!("格式化时间: {}", formatted.to_str().unwrap());
    }
}
```

### 2. 休眠和延时

```rust
use libc::{sleep, usleep, nanosleep, timespec};

fn sleep_examples() {
    println!("休眠 1 秒...");
    unsafe { sleep(1) };
    
    println!("休眠 500 毫秒...");
    unsafe { usleep(500_000) }; // 微秒
    
    // 高精度休眠
    let sleep_time = timespec {
        tv_sec: 0,
        tv_nsec: 100_000_000, // 100 毫秒
    };
    
    println!("高精度休眠 100 毫秒...");
    unsafe { nanosleep(&sleep_time, std::ptr::null_mut()) };
}
```

## 网络编程

### 1. Socket 操作

```rust
use libc::{
    socket, bind, listen, accept, connect, send, recv, close,
    AF_INET, SOCK_STREAM, sockaddr_in, sockaddr, socklen_t, in_addr
};
use std::mem;

fn socket_server_example() {
    // 创建 socket
    let server_fd = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if server_fd == -1 {
        panic!("无法创建 socket");
    }
    
    // 设置地址结构
    let mut addr: sockaddr_in = unsafe { mem::zeroed() };
    addr.sin_family = AF_INET as u16;
    addr.sin_port = 8080u16.to_be();
    addr.sin_addr.s_addr = libc::INADDR_ANY;
    
    // 绑定地址
    let bind_result = unsafe {
        bind(server_fd, 
             &addr as *const sockaddr_in as *const sockaddr, 
             mem::size_of::<sockaddr_in>() as socklen_t)
    };
    
    if bind_result == -1 {
        unsafe { close(server_fd) };
        panic!("绑定失败");
    }
    
    // 监听
    let listen_result = unsafe { listen(server_fd, 5) };
    if listen_result == -1 {
        unsafe { close(server_fd) };
        panic!("监听失败");
    }
    
    println!("服务器在端口 8080 上监听...");
    
    // 接受连接
    let mut client_addr: sockaddr_in = unsafe { mem::zeroed() };
    let mut client_len = mem::size_of::<sockaddr_in>() as socklen_t;
    
    let client_fd = unsafe {
        accept(server_fd, 
               &mut client_addr as *mut sockaddr_in as *mut sockaddr, 
               &mut client_len)
    };
    
    if client_fd != -1 {
        println!("客户端连接成功");
        
        // 发送响应
        let response = b"Hello from server!\n";
        unsafe {
            send(client_fd, response.as_ptr() as *const libc::c_void, response.len(), 0);
            close(client_fd);
        }
    }
    
    unsafe { close(server_fd) };
}
```

### 2. 客户端连接

```rust
use libc::{socket, connect, send, recv, close, AF_INET, SOCK_STREAM, sockaddr_in, sockaddr, in_addr};
use std::mem;

fn socket_client_example() {
    // 创建 socket
    let client_fd = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if client_fd == -1 {
        panic!("无法创建 socket");
    }
    
    // 设置服务器地址
    let mut server_addr: sockaddr_in = unsafe { mem::zeroed() };
    server_addr.sin_family = AF_INET as u16;
    server_addr.sin_port = 8080u16.to_be();
    server_addr.sin_addr.s_addr = libc::INADDR_LOOPBACK;
    
    // 连接服务器
    let connect_result = unsafe {
        connect(client_fd,
                &server_addr as *const sockaddr_in as *const sockaddr,
                mem::size_of::<sockaddr_in>() as libc::socklen_t)
    };
    
    if connect_result == -1 {
        unsafe { close(client_fd) };
        panic!("连接失败");
    }
    
    println!("连接到服务器成功");
    
    // 发送数据
    let message = b"Hello from client!";
    unsafe {
        send(client_fd, message.as_ptr() as *const libc::c_void, message.len(), 0);
    }
    
    // 接收响应
    let mut buffer = vec![0u8; 1024];
    let received = unsafe {
        recv(client_fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len(), 0)
    };
    
    if received > 0 {
        buffer.truncate(received as usize);
        println!("收到响应: {}", String::from_utf8_lossy(&buffer));
    }
    
    unsafe { close(client_fd) };
}
```

## 错误处理

### 1. errno 处理

```rust
use libc::{errno, strerror};
use std::ffi::CStr;

fn error_handling() {
    // 模拟一个失败的系统调用
    let result = unsafe { libc::open(b"/nonexistent/file\0".as_ptr() as *const libc::c_char, libc::O_RDONLY) };
    
    if result == -1 {
        let error_code = unsafe { errno() };
        let error_ptr = unsafe { strerror(error_code) };
        
        if !error_ptr.is_null() {
            let error_str = unsafe { CStr::from_ptr(error_ptr) };
            println!("错误 {}: {}", error_code, error_str.to_str().unwrap());
        }
    }
}

fn safe_system_call<F, R>(operation: F) -> Result<R, i32>
where
    F: FnOnce() -> R,
    R: PartialEq<i32>,
{
    let result = operation();
    if result == -1 {
        Err(unsafe { errno() })
    } else {
        Ok(result)
    }
}
```

### 2. 错误码映射

```rust
use libc::{ENOENT, EACCES, EINVAL, ENOMEM};

fn error_code_mapping(error_code: i32) -> &'static str {
    match error_code {
        ENOENT => "文件或目录不存在",
        EACCES => "权限被拒绝",
        EINVAL => "无效参数",
        ENOMEM => "内存不足",
        _ => "未知错误",
    }
}

fn error_mapping_example() {
    let error_codes = vec![ENOENT, EACCES, EINVAL, ENOMEM];
    
    for error_code in error_codes {
        println!("错误 {}: {}", error_code, error_code_mapping(error_code));
    }
}
```

## 平台特定功能

### 1. Unix 特定

```rust
#[cfg(unix)]
use libc::{getuid, getgid, getpwuid, getgrgid, passwd, group};

#[cfg(unix)]
fn unix_specific_examples() {
    // 获取用户和组信息
    let uid = unsafe { getuid() };
    let gid = unsafe { getgid() };
    
    println!("UID: {}, GID: {}", uid, gid);
    
    // 获取用户信息
    let pwd = unsafe { getpwuid(uid) };
    if !pwd.is_null() {
        let passwd_struct = unsafe { *pwd };
        if !passwd_struct.pw_name.is_null() {
            let username = unsafe { CStr::from_ptr(passwd_struct.pw_name) };
            println!("用户名: {}", username.to_str().unwrap());
        }
    }
    
    // 获取组信息
    let grp = unsafe { getgrgid(gid) };
    if !grp.is_null() {
        let group_struct = unsafe { *grp };
        if !group_struct.gr_name.is_null() {
            let groupname = unsafe { CStr::from_ptr(group_struct.gr_name) };
            println!("组名: {}", groupname.to_str().unwrap());
        }
    }
}
```

### 2. 信号处理

```rust
#[cfg(unix)]
use libc::{signal, kill, getpid, SIGTERM, SIGINT, SIG_DFL, SIG_IGN};

#[cfg(unix)]
extern "C" fn signal_handler(sig: libc::c_int) {
    println!("收到信号: {}", sig);
}

#[cfg(unix)]
fn signal_examples() {
    // 设置信号处理器
    unsafe {
        signal(SIGINT, signal_handler as *const fn(libc::c_int) as usize);
    }
    
    // 忽略信号
    unsafe {
        signal(SIGTERM, SIG_IGN);
    }
    
    // 发送信号给自己
    let pid = unsafe { getpid() };
    unsafe {
        kill(pid, SIGTERM);
    }
    
    println!("信号处理设置完成");
}
```

## 性能优化

### 1. 内存对齐

```rust
use libc::{posix_memalign, free, c_void};

fn aligned_memory_example() {
    let alignment = 16; // 16 字节对齐
    let size = 1024;
    let mut ptr: *mut c_void = std::ptr::null_mut();
    
    let result = unsafe {
        posix_memalign(&mut ptr, alignment, size)
    };
    
    if result == 0 {
        println!("对齐内存分配成功，地址: {:p}", ptr);
        
        // 验证对齐
        let addr = ptr as usize;
        println!("地址是否对齐: {}", addr % alignment == 0);
        
        unsafe { free(ptr) };
    } else {
        println!("对齐内存分配失败");
    }
}
```

### 2. 零拷贝操作

```rust
use libc::{sendfile, open, close, stat, off_t, O_RDONLY};

#[cfg(target_os = "linux")]
fn zero_copy_example() {
    let src_fd = unsafe { open(b"input.txt\0".as_ptr() as *const libc::c_char, O_RDONLY) };
    let dst_fd = unsafe { open(b"output.txt\0".as_ptr() as *const libc::c_char, 
                              libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC, 
                              0o644) };
    
    if src_fd != -1 && dst_fd != -1 {
        // 获取文件大小
        let mut file_stat: libc::stat = unsafe { std::mem::zeroed() };
        let stat_result = unsafe { stat(b"input.txt\0".as_ptr() as *const libc::c_char, &mut file_stat) };
        
        if stat_result == 0 {
            let mut offset: off_t = 0;
            let bytes_sent = unsafe {
                sendfile(dst_fd, src_fd, &mut offset, file_stat.st_size as usize)
            };
            
            println!("零拷贝传输字节数: {}", bytes_sent);
        }
        
        unsafe {
            close(src_fd);
            close(dst_fd);
        }
    }
}
```

## 安全编程

### 1. 边界检查

```rust
use libc::{c_char, c_void};

fn safe_string_copy(src: &str, dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() >= dst.len() {
        return Err("目标缓冲区太小");
    }
    
    let c_str = std::ffi::CString::new(src).map_err(|_| "字符串包含空字符")?;
    let src_ptr = c_str.as_ptr();
    let dst_ptr = dst.as_mut_ptr() as *mut c_char;
    
    unsafe {
        libc::strncpy(dst_ptr, src_ptr, dst.len() - 1);
        *dst_ptr.add(dst.len() - 1) = 0; // 确保以空字符结尾
    }
    
    Ok(())
}

fn safe_copy_example() {
    let mut buffer = vec![0u8; 100];
    
    match safe_string_copy("Hello, Safe World!", &mut buffer) {
        Ok(()) => {
            let result = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr() as *const c_char) };
            println!("安全拷贝结果: {}", result.to_str().unwrap());
        }
        Err(e) => {
            println!("拷贝失败: {}", e);
        }
    }
}
```

### 2. 资源管理

```rust
use libc::{FILE, fopen, fclose, c_char};

struct SafeFile {
    file: *mut FILE,
}

impl SafeFile {
    fn open(path: &str, mode: &str) -> Result<Self, &'static str> {
        let c_path = std::ffi::CString::new(path).map_err(|_| "路径包含空字符")?;
        let c_mode = std::ffi::CString::new(mode).map_err(|_| "模式包含空字符")?;
        
        let file = unsafe { fopen(c_path.as_ptr(), c_mode.as_ptr()) };
        
        if file.is_null() {
            Err("无法打开文件")
        } else {
            Ok(SafeFile { file })
        }
    }
    
    fn as_ptr(&self) -> *mut FILE {
        self.file
    }
}

impl Drop for SafeFile {
    fn drop(&mut self) {
        if !self.file.is_null() {
            unsafe { fclose(self.file) };
        }
    }
}

fn safe_file_example() {
    match SafeFile::open("test.txt", "w") {
        Ok(file) => {
            println!("文件安全打开成功");
            // 文件会在 SafeFile 销毁时自动关闭
        }
        Err(e) => {
            println!("文件打开失败: {}", e);
        }
    }
}
```

## 最佳实践

1. **安全性优先**: 始终检查 C 函数的返回值
2. **资源管理**: 使用 RAII 模式管理资源
3. **错误处理**: 正确处理 errno 和其他错误码
4. **边界检查**: 防止缓冲区溢出
5. **内存安全**: 避免内存泄漏和双重释放

## 调试技巧

### 1. 调试输出

```rust
use libc::{printf, fprintf, stderr};

fn debug_output() {
    let message = b"Debug message: %d\n\0";
    unsafe {
        printf(message.as_ptr() as *const libc::c_char, 42);
        fprintf(stderr, message.as_ptr() as *const libc::c_char, 42);
    }
}
```

### 2. 系统调用跟踪

```rust
use libc::{strace, ptrace};

fn trace_system_calls() {
    // 在实际调试中，您可以使用 strace 工具
    // 或者通过 ptrace 系统调用实现自定义跟踪
    println!("使用 strace 跟踪系统调用");
}
```

## 总结

`libc` 是 Rust 系统编程的基础库，提供了对底层系统功能的直接访问。主要特点包括：

- **全面性**: 覆盖了 C 标准库的所有功能
- **跨平台**: 支持多种操作系统
- **性能**: 直接映射到系统调用，性能损失最小
- **互操作性**: 与 C 代码的完美集成

通过掌握 `libc`，您可以进行高效的系统编程，处理文件、网络、进程管理等底层任务，同时保持 Rust 的内存安全特性。但请注意，使用 `libc` 需要 `unsafe` 代码，因此需要特别注意安全性和正确性。