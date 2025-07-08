// 11_进程与环境.rs
// Rust标准库进程与环境操作详解

/*
std::process 和 std::env 模块提供了进程管理和环境操作功能：

std::process 主要类型：
- Command：命令构建器
- Child：子进程句柄
- Output：进程输出
- Stdio：标准I/O配置
- ExitStatus：退出状态

std::env 主要功能：
- 环境变量操作：var(), set_var(), remove_var()
- 命令行参数：args(), args_os()
- 工作目录：current_dir(), set_current_dir()
- 系统信息：consts模块

进程管理特点：
- 跨平台：Windows和Unix系统兼容
- 安全性：防止命令注入
- 灵活性：支持管道、重定向
- 异步支持：非阻塞进程执行

应用场景：
- 系统工具开发
- 构建系统
- 命令行工具
- 系统监控
- 自动化脚本
*/

use std::process::{Command, Stdio, Child};
use std::env;
use std::io::{Write, BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    println!("=== Rust标准库进程与环境操作 ===");
    
    // 1. 环境变量操作
    println!("\n1. 环境变量操作：");
    environment_variables();
    
    // 2. 命令行参数处理
    println!("\n2. 命令行参数处理：");
    command_line_arguments();
    
    // 3. 基本进程执行
    println!("\n3. 基本进程执行：");
    basic_process_execution();
    
    // 4. 进程输入输出控制
    println!("\n4. 进程输入输出控制：");
    process_io_control();
    
    // 5. 进程管道和重定向
    println!("\n5. 进程管道和重定向：");
    process_pipes_and_redirection();
    
    // 6. 异步进程管理
    println!("\n6. 异步进程管理：");
    async_process_management();
    
    // 7. 系统信息获取
    println!("\n7. 系统信息获取：");
    system_information();
    
    // 8. 工作目录操作
    println!("\n8. 工作目录操作：");
    working_directory_operations();
    
    // 9. 进程监控和控制
    println!("\n9. 进程监控和控制：");
    process_monitoring();
    
    // 10. 实际应用示例
    println!("\n10. 实际应用示例：");
    practical_examples();
    
    println!("\n=== 进程与环境操作学习完成 ===");
}

// 环境变量操作
fn environment_variables() {
    // 读取环境变量
    match env::var("PATH") {
        Ok(path) => {
            println!("PATH环境变量长度: {} 字符", path.len());
            // 只显示前100个字符
            let preview = if path.len() > 100 {
                format!("{}...", &path[..100])
            } else {
                path
            };
            println!("PATH预览: {}", preview);
        }
        Err(e) => println!("读取PATH失败: {}", e),
    }
    
    // 读取特定环境变量
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
    match home {
        Ok(home_dir) => println!("用户主目录: {}", home_dir),
        Err(_) => println!("无法获取用户主目录"),
    }
    
    // 设置环境变量
    env::set_var("RUST_TEST_VAR", "Hello from Rust!");
    match env::var("RUST_TEST_VAR") {
        Ok(value) => println!("设置的环境变量: {}", value),
        Err(_) => println!("环境变量设置失败"),
    }
    
    // 删除环境变量
    env::remove_var("RUST_TEST_VAR");
    match env::var("RUST_TEST_VAR") {
        Ok(_) => println!("环境变量仍然存在"),
        Err(_) => println!("环境变量已删除"),
    }
    
    // 遍历所有环境变量
    println!("环境变量总数: {}", env::vars().count());
    
    // 显示部分环境变量
    println!("部分环境变量:");
    for (key, value) in env::vars().take(5) {
        // 截断长值以便显示
        let display_value = if value.len() > 50 {
            format!("{}...", &value[..50])
        } else {
            value
        };
        println!("  {} = {}", key, display_value);
    }
    
    // 环境变量的OS字符串版本
    println!("OS字符串环境变量示例:");
    for (key, value) in env::vars_os().take(3) {
        println!("  {:?} = {:?}", key, value);
    }
}

// 命令行参数处理
fn command_line_arguments() {
    // 获取所有参数
    let args: Vec<String> = env::args().collect();
    println!("命令行参数数量: {}", args.len());
    
    for (i, arg) in args.iter().enumerate() {
        println!("  参数[{}]: {}", i, arg);
    }
    
    // 跳过程序名称
    let program_args: Vec<String> = env::args().skip(1).collect();
    if program_args.is_empty() {
        println!("没有提供额外的命令行参数");
    } else {
        println!("程序参数: {:?}", program_args);
    }
    
    // OS字符串版本的参数
    let os_args: Vec<_> = env::args_os().collect();
    println!("OS字符串参数数量: {}", os_args.len());
    
    // 简单的参数解析示例
    simple_argument_parsing();
}

// 简单的参数解析
fn simple_argument_parsing() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("参数解析示例 (当前无额外参数):");
        println!("  --help: 显示帮助");
        println!("  --version: 显示版本");
        println!("  --config <file>: 指定配置文件");
        return;
    }
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                println!("显示帮助信息");
            }
            "--version" | "-v" => {
                println!("版本: 1.0.0");
            }
            "--config" | "-c" => {
                if i + 1 < args.len() {
                    println!("配置文件: {}", args[i + 1]);
                    i += 1; // 跳过配置文件参数
                } else {
                    println!("错误: --config 需要一个参数");
                }
            }
            _ => {
                println!("未知参数: {}", args[i]);
            }
        }
        i += 1;
    }
}

// 基本进程执行
fn basic_process_execution() {
    // 执行简单命令
    match Command::new("echo").arg("Hello from subprocess!").output() {
        Ok(output) => {
            println!("命令执行成功:");
            println!("  退出状态: {}", output.status);
            println!("  标准输出: {}", String::from_utf8_lossy(&output.stdout));
            println!("  标准错误: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => println!("命令执行失败: {}", e),
    }
    
    // 执行带多个参数的命令
    let output = Command::new("ls")
        .arg("-la")
        .arg(".")
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("目录列表 (前5行):");
                for (i, line) in stdout.lines().take(5).enumerate() {
                    println!("  {}: {}", i + 1, line);
                }
            } else {
                println!("ls命令失败: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => println!("ls命令执行失败: {}", e),
    }
    
    // 检查命令是否存在
    check_command_availability();
}

// 检查命令可用性
fn check_command_availability() {
    let commands = ["git", "python3", "node", "cargo", "rustc"];
    
    println!("检查命令可用性:");
    for cmd in &commands {
        let result = Command::new(cmd)
            .arg("--version")
            .output();
        
        match result {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    let first_line = version.lines().next().unwrap_or("未知版本");
                    println!("  ✓ {}: {}", cmd, first_line);
                } else {
                    println!("  ✗ {}: 命令存在但版本检查失败", cmd);
                }
            }
            Err(_) => {
                println!("  ✗ {}: 命令不存在或不可执行", cmd);
            }
        }
    }
}

// 进程输入输出控制
fn process_io_control() {
    // 使用管道进行输入输出
    pipe_communication_example();
    
    // 重定向标准输出
    output_redirection_example();
    
    // 捕获错误输出
    error_capture_example();
}

// 管道通信示例
fn pipe_communication_example() {
    println!("管道通信示例:");
    
    let mut child = match Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            println!("  启动cat进程失败: {}", e);
            return;
        }
    };
    
    // 向子进程写入数据
    if let Some(stdin) = child.stdin.take() {
        let mut stdin = stdin;
        thread::spawn(move || {
            let _ = writeln!(stdin, "Hello from parent process!");
            let _ = writeln!(stdin, "This is line 2");
            let _ = writeln!(stdin, "This is line 3");
            // stdin在这里被drop，关闭管道
        });
    }
    
    // 读取子进程输出
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("  从子进程接收: {}", line),
                Err(e) => {
                    println!("  读取子进程输出失败: {}", e);
                    break;
                }
            }
        }
    }
    
    // 等待子进程结束
    match child.wait() {
        Ok(status) => println!("  子进程退出状态: {}", status),
        Err(e) => println!("  等待子进程失败: {}", e),
    }
}

// 输出重定向示例
fn output_redirection_example() {
    println!("输出重定向示例:");
    
    // 重定向到文件
    use std::fs::File;
    
    let output_file = "process_output.txt";
    match File::create(output_file) {
        Ok(file) => {
            let result = Command::new("echo")
                .arg("This output goes to file")
                .stdout(Stdio::from(file))
                .status();
            
            match result {
                Ok(status) => {
                    println!("  命令执行: {}", status);
                    
                    // 读取并显示文件内容
                    if let Ok(content) = std::fs::read_to_string(output_file) {
                        println!("  文件内容: {}", content.trim());
                    }
                }
                Err(e) => println!("  重定向命令失败: {}", e),
            }
            
            // 清理文件
            let _ = std::fs::remove_file(output_file);
        }
        Err(e) => println!("  创建输出文件失败: {}", e),
    }
}

// 错误捕获示例
fn error_capture_example() {
    println!("错误捕获示例:");
    
    // 执行一个会产生错误的命令
    let result = Command::new("ls")
        .arg("/nonexistent_directory_12345")
        .output();
    
    match result {
        Ok(output) => {
            println!("  退出状态: {}", output.status);
            println!("  标准输出: '{}'", String::from_utf8_lossy(&output.stdout));
            println!("  错误输出: '{}'", String::from_utf8_lossy(&output.stderr));
            
            if !output.status.success() {
                println!("  命令执行失败，但捕获了错误信息");
            }
        }
        Err(e) => println!("  命令执行失败: {}", e),
    }
}

// 进程管道和重定向
fn process_pipes_and_redirection() {
    // 命令链：模拟 ls | grep txt
    command_chain_example();
    
    // 多进程协作
    multi_process_cooperation();
}

// 命令链示例
fn command_chain_example() {
    println!("命令链示例 (ls | grep .rs):");
    
    // 第一个命令：ls
    let ls_child = Command::new("ls")
        .arg(".")
        .stdout(Stdio::piped())
        .spawn();
    
    let ls_child = match ls_child {
        Ok(child) => child,
        Err(e) => {
            println!("  启动ls失败: {}", e);
            return;
        }
    };
    
    // 第二个命令：grep，使用第一个命令的输出作为输入
    let grep_child = Command::new("grep")
        .arg(".rs")
        .stdin(Stdio::from(ls_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn();
    
    let grep_child = match grep_child {
        Ok(child) => child,
        Err(e) => {
            println!("  启动grep失败: {}", e);
            return;
        }
    };
    
    // 读取最终输出
    let output = grep_child.wait_with_output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                println!("  找到的.rs文件:");
                for line in result.lines() {
                    println!("    {}", line);
                }
            } else {
                println!("  grep命令失败");
            }
        }
        Err(e) => println!("  等待grep输出失败: {}", e),
    }
}

// 多进程协作
fn multi_process_cooperation() {
    println!("多进程协作示例:");
    
    // 启动多个worker进程
    let mut workers = Vec::new();
    
    for i in 0..3 {
        match Command::new("echo")
            .arg(&format!("Worker {} completed task", i))
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(child) => workers.push(child),
            Err(e) => println!("  启动worker {}失败: {}", i, e),
        }
    }
    
    // 收集所有worker的输出
    for (i, worker) in workers.into_iter().enumerate() {
        match worker.wait_with_output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                println!("  Worker {}: {}", i, result.trim());
            }
            Err(e) => println!("  Worker {}失败: {}", i, e),
        }
    }
}

// 异步进程管理
fn async_process_management() {
    // 非阻塞进程启动
    non_blocking_process_example();
    
    // 进程超时控制
    process_timeout_example();
    
    // 并发进程执行
    concurrent_process_example();
}

// 非阻塞进程示例
fn non_blocking_process_example() {
    println!("非阻塞进程示例:");
    
    // 启动长时间运行的进程
    let mut child = match Command::new("sleep").arg("2").spawn() {
        Ok(child) => child,
        Err(_) => {
            // 如果sleep命令不可用，使用替代方案
            println!("  sleep命令不可用，跳过此示例");
            return;
        }
    };
    
    println!("  进程已启动，ID: {:?}", child.id());
    
    // 在等待进程时做其他工作
    for i in 1..=5 {
        thread::sleep(Duration::from_millis(500));
        
        // 检查进程是否完成
        match child.try_wait() {
            Ok(Some(status)) => {
                println!("  进程提前完成，状态: {}", status);
                return;
            }
            Ok(None) => {
                println!("  进程仍在运行... ({})", i);
            }
            Err(e) => {
                println!("  检查进程状态失败: {}", e);
                return;
            }
        }
    }
    
    // 等待进程完成
    match child.wait() {
        Ok(status) => println!("  进程最终状态: {}", status),
        Err(e) => println!("  等待进程失败: {}", e),
    }
}

// 进程超时控制
fn process_timeout_example() {
    println!("进程超时控制示例:");
    
    // 启动可能长时间运行的进程
    let mut child = match Command::new("sleep").arg("10").spawn() {
        Ok(child) => child,
        Err(_) => {
            println!("  sleep命令不可用，跳过此示例");
            return;
        }
    };
    
    let timeout = Duration::from_secs(1);
    let start = std::time::Instant::now();
    
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                println!("  进程正常完成: {}", status);
                return;
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    println!("  进程超时，强制终止");
                    if let Err(e) = child.kill() {
                        println!("  终止进程失败: {}", e);
                    } else {
                        let _ = child.wait(); // 清理僵尸进程
                        println!("  进程已终止");
                    }
                    return;
                }
            }
            Err(e) => {
                println!("  检查进程状态失败: {}", e);
                return;
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
}

// 并发进程执行
fn concurrent_process_example() {
    println!("并发进程执行示例:");
    
    let commands = vec![
        ("echo", vec!["Task 1 completed"]),
        ("echo", vec!["Task 2 completed"]),
        ("echo", vec!["Task 3 completed"]),
    ];
    
    let mut handles = Vec::new();
    
    // 启动所有进程
    for (i, (cmd, args)) in commands.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            let start = std::time::Instant::now();
            let result = Command::new(cmd).args(&args).output();
            let duration = start.elapsed();
            
            match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    (i, stdout.trim().to_string(), duration)
                }
                Err(e) => (i, format!("错误: {}", e), duration),
            }
        });
        handles.push(handle);
    }
    
    // 收集结果
    for handle in handles {
        match handle.join() {
            Ok((i, result, duration)) => {
                println!("  任务{}: {} (耗时: {:?})", i + 1, result, duration);
            }
            Err(_) => println!("  某个任务执行失败"),
        }
    }
}

// 系统信息获取
fn system_information() {
    // 程序信息
    println!("程序信息:");
    println!("  当前可执行文件: {:?}", env::current_exe());
    
    // 系统常量
    println!("系统信息:");
    println!("  操作系统: {}", env::consts::OS);
    println!("  架构: {}", env::consts::ARCH);
    println!("  系列: {}", env::consts::FAMILY);
    println!("  DLL前缀: {}", env::consts::DLL_PREFIX);
    println!("  DLL后缀: {}", env::consts::DLL_SUFFIX);
    println!("  可执行文件后缀: {}", env::consts::EXE_SUFFIX);
    
    // 特殊目录
    println!("特殊目录:");
    if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        println!("  用户主目录: {}", home);
    }
    
    let temp_dir = env::temp_dir();
    println!("  临时目录: {}", temp_dir.display());
    
    // 系统配置
    system_configuration();
}

// 系统配置信息
fn system_configuration() {
    println!("系统配置:");
    
    // CPU相关
    if let Ok(cpu_count) = env::var("NUMBER_OF_PROCESSORS") {
        println!("  CPU数量: {}", cpu_count);
    }
    
    // 内存相关
    if let Ok(memory) = env::var("MEMORY") {
        println!("  内存信息: {}", memory);
    }
    
    // 路径分隔符
    println!("  路径分隔符: {:?}", std::path::MAIN_SEPARATOR);
    
    // 换行符
    #[cfg(windows)]
    println!("  换行符: CRLF");
    
    #[cfg(unix)]
    println!("  换行符: LF");
    
    // 字节序
    if cfg!(target_endian = "little") {
        println!("  字节序: 小端");
    } else {
        println!("  字节序: 大端");
    }
    
    // 指针大小
    println!("  指针大小: {} 位", std::mem::size_of::<usize>() * 8);
}

// 工作目录操作
fn working_directory_operations() {
    // 获取当前工作目录
    match env::current_dir() {
        Ok(current) => println!("当前工作目录: {}", current.display()),
        Err(e) => println!("获取工作目录失败: {}", e),
    }
    
    // 改变工作目录
    let original_dir = env::current_dir().unwrap();
    
    // 尝试切换到上级目录
    if let Some(parent) = original_dir.parent() {
        match env::set_current_dir(parent) {
            Ok(_) => {
                println!("切换到上级目录: {}", parent.display());
                
                // 验证切换
                if let Ok(new_dir) = env::current_dir() {
                    println!("验证新工作目录: {}", new_dir.display());
                }
                
                // 切换回原目录
                if let Err(e) = env::set_current_dir(&original_dir) {
                    println!("切换回原目录失败: {}", e);
                } else {
                    println!("已切换回原目录");
                }
            }
            Err(e) => println!("切换目录失败: {}", e),
        }
    }
    
    // 工作目录对子进程的影响
    working_directory_inheritance();
}

// 工作目录继承
fn working_directory_inheritance() {
    println!("工作目录继承测试:");
    
    // 子进程继承父进程的工作目录
    let output = Command::new("pwd")
        .output()
        .or_else(|_| Command::new("cd").output()); // Windows fallback
    
    match output {
        Ok(output) => {
            let pwd = String::from_utf8_lossy(&output.stdout);
            println!("  子进程工作目录: {}", pwd.trim());
        }
        Err(e) => println!("  获取子进程工作目录失败: {}", e),
    }
    
    // 为子进程指定不同的工作目录
    if let Some(parent) = env::current_dir().unwrap().parent() {
        let output = Command::new("pwd")
            .current_dir(parent)
            .output()
            .or_else(|_| {
                Command::new("cd")
                    .current_dir(parent)
                    .output()
            });
        
        match output {
            Ok(output) => {
                let pwd = String::from_utf8_lossy(&output.stdout);
                println!("  指定工作目录的子进程: {}", pwd.trim());
            }
            Err(e) => println!("  指定工作目录失败: {}", e),
        }
    }
}

// 进程监控
fn process_monitoring() {
    // 进程资源使用监控
    process_resource_monitoring();
    
    // 进程状态跟踪
    process_status_tracking();
    
    // 进程组管理
    process_group_management();
}

// 进程资源监控
fn process_resource_monitoring() {
    println!("进程资源监控:");
    
    // 启动一个简单的进程
    let mut child = match Command::new("echo")
        .arg("monitoring test")
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            println!("  启动进程失败: {}", e);
            return;
        }
    };
    
    println!("  进程ID: {:?}", child.id());
    
    // 监控进程状态
    let start_time = std::time::Instant::now();
    
    match child.wait() {
        Ok(status) => {
            let duration = start_time.elapsed();
            println!("  进程完成: {} (耗时: {:?})", status, duration);
            
            if status.success() {
                println!("  进程正常退出");
            } else {
                if let Some(code) = status.code() {
                    println!("  进程退出码: {}", code);
                } else {
                    println!("  进程被信号终止");
                }
            }
        }
        Err(e) => println!("  等待进程失败: {}", e),
    }
}

// 进程状态跟踪
fn process_status_tracking() {
    println!("进程状态跟踪:");
    
    // 创建一个稍微长一点的进程
    let mut child = match Command::new("sleep").arg("1").spawn() {
        Ok(child) => child,
        Err(_) => {
            println!("  sleep命令不可用，跳过状态跟踪");
            return;
        }
    };
    
    println!("  开始跟踪进程ID: {:?}", child.id());
    
    // 定期检查进程状态
    for i in 1..=10 {
        thread::sleep(Duration::from_millis(200));
        
        match child.try_wait() {
            Ok(Some(status)) => {
                println!("  第{}次检查: 进程已完成, 状态: {}", i, status);
                return;
            }
            Ok(None) => {
                println!("  第{}次检查: 进程仍在运行", i);
            }
            Err(e) => {
                println!("  第{}次检查失败: {}", i, e);
                return;
            }
        }
    }
    
    // 如果进程还在运行，等待它完成
    let _ = child.wait();
}

// 进程组管理
fn process_group_management() {
    println!("进程组管理:");
    println!("  当前进程PID: {}", std::process::id());
    
    // 注意：Rust标准库对进程组的支持有限
    // 这里主要演示概念
    
    println!("  进程组功能需要平台特定的扩展");
    println!("  Unix系统可使用 nix 库");
    println!("  Windows系统可使用 winapi 库");
    
    // 启动多个相关进程
    let mut children = Vec::new();
    
    for i in 0..3 {
        match Command::new("echo")
            .arg(&format!("Process group member {}", i))
            .spawn()
        {
            Ok(child) => {
                println!("  启动子进程{}: PID {:?}", i, child.id());
                children.push(child);
            }
            Err(e) => println!("  启动子进程{}失败: {}", i, e),
        }
    }
    
    // 等待所有子进程
    for (i, child) in children.into_iter().enumerate() {
        match child.wait_with_output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("  子进程{}: {}", i, stdout.trim());
            }
            Err(e) => println!("  子进程{}失败: {}", i, e),
        }
    }
}

// 实际应用示例
fn practical_examples() {
    // 构建系统示例
    build_system_example();
    
    // 系统监控示例
    system_monitoring_example();
    
    // 批处理脚本示例
    batch_processing_example();
}

// 构建系统示例
fn build_system_example() {
    println!("构建系统示例:");
    
    // 模拟构建步骤
    let build_steps = vec![
        ("检查依赖", "echo", vec!["Checking dependencies..."]),
        ("编译代码", "echo", vec!["Compiling source code..."]),
        ("运行测试", "echo", vec!["Running tests..."]),
        ("打包程序", "echo", vec!["Packaging application..."]),
    ];
    
    let mut all_success = true;
    
    for (step_name, command, args) in build_steps {
        println!("  执行步骤: {}", step_name);
        
        let start = std::time::Instant::now();
        let result = Command::new(command).args(&args).status();
        let duration = start.elapsed();
        
        match result {
            Ok(status) => {
                if status.success() {
                    println!("    ✓ 成功 (耗时: {:?})", duration);
                } else {
                    println!("    ✗ 失败 (退出码: {:?})", status.code());
                    all_success = false;
                    break;
                }
            }
            Err(e) => {
                println!("    ✗ 执行失败: {}", e);
                all_success = false;
                break;
            }
        }
    }
    
    if all_success {
        println!("  构建完成!");
    } else {
        println!("  构建失败!");
    }
}

// 系统监控示例
fn system_monitoring_example() {
    println!("系统监控示例:");
    
    // 监控系统信息
    let monitors = vec![
        ("磁盘使用", "df", vec!["-h"]),
        ("内存使用", "free", vec!["-h"]),
        ("系统负载", "uptime", vec![]),
    ];
    
    for (name, command, args) in monitors {
        match Command::new(command).args(&args).output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("  {}:", name);
                    for line in stdout.lines().take(3) {
                        println!("    {}", line);
                    }
                } else {
                    println!("  {}: 命令执行失败", name);
                }
            }
            Err(_) => {
                println!("  {}: 命令不可用", name);
            }
        }
    }
}

// 批处理示例
fn batch_processing_example() {
    println!("批处理示例:");
    
    // 模拟批量文件处理
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    
    for file in &files {
        // 创建临时文件
        let content = format!("Content of {}", file);
        if std::fs::write(file, &content).is_ok() {
            println!("  创建文件: {}", file);
        }
    }
    
    // 批量处理文件
    let mut processed = 0;
    let mut failed = 0;
    
    for file in &files {
        let result = Command::new("wc")
            .arg("-c")
            .arg(file)
            .output()
            .or_else(|_| {
                // Windows fallback
                Command::new("cmd")
                    .args(&["/C", "dir", file])
                    .output()
            });
        
        match result {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("  处理 {}: {}", file, stdout.trim());
                    processed += 1;
                } else {
                    println!("  处理 {} 失败", file);
                    failed += 1;
                }
            }
            Err(e) => {
                println!("  处理 {} 出错: {}", file, e);
                failed += 1;
            }
        }
    }
    
    println!("  批处理完成: {} 成功, {} 失败", processed, failed);
    
    // 清理临时文件
    for file in &files {
        let _ = std::fs::remove_file(file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_environment_variables() {
        // 设置测试环境变量
        env::set_var("TEST_VAR", "test_value");
        assert_eq!(env::var("TEST_VAR").unwrap(), "test_value");
        
        // 删除环境变量
        env::remove_var("TEST_VAR");
        assert!(env::var("TEST_VAR").is_err());
    }
    
    #[test]
    fn test_command_execution() {
        let output = Command::new("echo")
            .arg("test")
            .output()
            .unwrap();
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("test"));
    }
    
    #[test]
    fn test_working_directory() {
        let original = env::current_dir().unwrap();
        
        // 测试工作目录获取
        assert!(original.exists());
        assert!(original.is_dir());
    }
    
    #[test]
    fn test_command_args() {
        let args: Vec<String> = env::args().collect();
        assert!(!args.is_empty());
        
        // 第一个参数应该是程序名
        assert!(args[0].contains("test") || args[0].contains("cargo"));
    }
    
    #[test]
    fn test_system_info() {
        // 测试系统常量
        assert!(!env::consts::OS.is_empty());
        assert!(!env::consts::ARCH.is_empty());
        assert!(!env::consts::FAMILY.is_empty());
    }
}