// 05_输入输出.rs
// Rust标准库输入输出(I/O)详解

/*
Rust标准库的I/O系统提供了丰富的功能：

核心trait：
- Read：从输入源读取数据
- Write：向输出目标写入数据
- Seek：在流中定位
- BufRead：缓冲读取

标准输入输出：
- stdin()：标准输入
- stdout()：标准输出
- stderr()：标准错误输出

文件操作：
- File：文件操作
- OpenOptions：文件打开选项
- fs模块：文件系统操作

缓冲I/O：
- BufReader：缓冲读取器
- BufWriter：缓冲写入器
- LineWriter：行缓冲写入器

网络I/O：
- TcpStream：TCP连接
- TcpListener：TCP监听器
- UdpSocket：UDP套接字

错误处理：
- io::Error：I/O错误类型
- io::Result<T>：I/O操作结果类型
*/

use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Seek, SeekFrom};
use std::fs::{File, OpenOptions, create_dir_all, remove_file, remove_dir_all};
use std::path::Path;
use std::env;
use std::process::Command;

fn main() {
    println!("=== Rust标准库输入输出 ===");
    
    // 1. 标准输入输出
    println!("\n1. 标准输入输出：");
    standard_io();
    
    // 2. 文件读写操作
    println!("\n2. 文件读写操作：");
    file_operations();
    
    // 3. 缓冲I/O操作
    println!("\n3. 缓冲I/O操作：");
    buffered_io();
    
    // 4. 文件系统操作
    println!("\n4. 文件系统操作：");
    filesystem_operations();
    
    // 5. 二进制数据处理
    println!("\n5. 二进制数据处理：");
    binary_data();
    
    // 6. 高级I/O操作
    println!("\n6. 高级I/O操作：");
    advanced_io();
    
    // 7. 错误处理
    println!("\n7. I/O错误处理：");
    io_error_handling();
    
    // 8. 性能优化
    println!("\n8. I/O性能优化：");
    io_performance();
    
    // 9. 实际应用示例
    println!("\n9. 实际应用示例：");
    practical_examples();
    
    println!("\n=== 输入输出学习完成 ===");
}

// 标准输入输出
fn standard_io() {
    // 标准输出
    println!("这是标准输出");
    print!("这是不换行的输出");
    println!(" - 继续在同一行");
    
    // 标准错误输出
    eprintln!("这是标准错误输出");
    
    // 格式化输出
    let name = "Rust";
    let version = "1.70";
    println!("语言: {}, 版本: {}", name, version);
    
    // 带样式的输出
    println!("普通文本");
    println!("\x1b[31m红色文本\x1b[0m");
    println!("\x1b[32m绿色文本\x1b[0m");
    println!("\x1b[33m黄色文本\x1b[0m");
    println!("\x1b[34m蓝色文本\x1b[0m");
    
    // 刷新输出缓冲区
    print!("正在处理...");
    io::stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!(" 完成!");
    
    // 从标准输入读取 (示例代码，实际运行时可取消注释)
    /*
    println!("请输入您的姓名:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("您好, {}!", input.trim()),
        Err(e) => println!("读取输入失败: {}", e),
    }
    */
}

// 文件读写操作
fn file_operations() {
    let filename = "test_file.txt";
    
    // 写入文件
    match write_to_file(filename, "Hello, Rust I/O!\n这是第二行\n") {
        Ok(_) => println!("文件写入成功"),
        Err(e) => println!("文件写入失败: {}", e),
    }
    
    // 读取文件
    match read_from_file(filename) {
        Ok(content) => println!("文件内容:\n{}", content),
        Err(e) => println!("文件读取失败: {}", e),
    }
    
    // 追加到文件
    match append_to_file(filename, "追加的内容\n") {
        Ok(_) => println!("内容追加成功"),
        Err(e) => println!("内容追加失败: {}", e),
    }
    
    // 再次读取查看追加结果
    match read_from_file(filename) {
        Ok(content) => println!("追加后的文件内容:\n{}", content),
        Err(e) => println!("文件读取失败: {}", e),
    }
    
    // 清理测试文件
    let _ = remove_file(filename);
}

// 缓冲I/O操作
fn buffered_io() {
    let filename = "buffered_test.txt";
    
    // 使用缓冲写入器
    match buffered_write(filename) {
        Ok(_) => println!("缓冲写入成功"),
        Err(e) => println!("缓冲写入失败: {}", e),
    }
    
    // 使用缓冲读取器
    match buffered_read(filename) {
        Ok(lines) => {
            println!("缓冲读取结果:");
            for (i, line) in lines.iter().enumerate() {
                println!("  行 {}: {}", i + 1, line);
            }
        }
        Err(e) => println!("缓冲读取失败: {}", e),
    }
    
    // 逐行读取
    match read_lines(filename) {
        Ok(lines) => {
            println!("逐行读取结果:");
            for (i, line) in lines.iter().enumerate() {
                println!("  行 {}: {}", i + 1, line);
            }
        }
        Err(e) => println!("逐行读取失败: {}", e),
    }
    
    // 清理测试文件
    let _ = remove_file(filename);
}

// 文件系统操作
fn filesystem_operations() {
    let test_dir = "test_directory";
    let test_file = format!("{}/test.txt", test_dir);
    
    // 创建目录
    match create_dir_all(test_dir) {
        Ok(_) => println!("目录创建成功: {}", test_dir),
        Err(e) => println!("目录创建失败: {}", e),
    }
    
    // 检查路径是否存在
    if Path::new(test_dir).exists() {
        println!("目录存在: {}", test_dir);
    }
    
    // 创建文件
    match File::create(&test_file) {
        Ok(mut file) => {
            let _ = writeln!(file, "测试文件内容");
            println!("文件创建成功: {}", test_file);
        }
        Err(e) => println!("文件创建失败: {}", e),
    }
    
    // 获取文件信息
    match std::fs::metadata(&test_file) {
        Ok(metadata) => {
            println!("文件信息:");
            println!("  大小: {} 字节", metadata.len());
            println!("  是否为文件: {}", metadata.is_file());
            println!("  是否为目录: {}", metadata.is_dir());
            println!("  只读: {}", metadata.permissions().readonly());
        }
        Err(e) => println!("获取文件信息失败: {}", e),
    }
    
    // 列出目录内容
    match std::fs::read_dir(test_dir) {
        Ok(entries) => {
            println!("目录内容:");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        println!("  {}", entry.path().display());
                    }
                    Err(e) => println!("  读取条目失败: {}", e),
                }
            }
        }
        Err(e) => println!("读取目录失败: {}", e),
    }
    
    // 获取当前工作目录
    match env::current_dir() {
        Ok(path) => println!("当前工作目录: {}", path.display()),
        Err(e) => println!("获取当前目录失败: {}", e),
    }
    
    // 清理测试文件和目录
    let _ = remove_file(&test_file);
    let _ = remove_dir_all(test_dir);
}

// 二进制数据处理
fn binary_data() {
    let filename = "binary_test.bin";
    
    // 写入二进制数据
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" in ASCII
    match write_binary_data(filename, &data) {
        Ok(_) => println!("二进制数据写入成功"),
        Err(e) => println!("二进制数据写入失败: {}", e),
    }
    
    // 读取二进制数据
    match read_binary_data(filename) {
        Ok(data) => {
            println!("二进制数据读取成功:");
            println!("  十六进制: {:?}", data);
            println!("  ASCII: {}", String::from_utf8_lossy(&data));
        }
        Err(e) => println!("二进制数据读取失败: {}", e),
    }
    
    // 处理数字数据
    let numbers = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    match write_numbers(filename, &numbers) {
        Ok(_) => println!("数字数据写入成功"),
        Err(e) => println!("数字数据写入失败: {}", e),
    }
    
    match read_numbers(filename) {
        Ok(numbers) => println!("数字数据读取成功: {:?}", numbers),
        Err(e) => println!("数字数据读取失败: {}", e),
    }
    
    // 清理测试文件
    let _ = remove_file(filename);
}

// 高级I/O操作
fn advanced_io() {
    let filename = "advanced_test.txt";
    
    // 创建测试文件
    let test_content = "第一行\n第二行\n第三行\n第四行\n第五行\n";
    let _ = write_to_file(filename, test_content);
    
    // 随机访问文件
    match random_access_file(filename) {
        Ok(_) => println!("随机访问文件成功"),
        Err(e) => println!("随机访问文件失败: {}", e),
    }
    
    // 文件锁定 (在实际应用中很重要)
    match file_locking_demo(filename) {
        Ok(_) => println!("文件锁定演示成功"),
        Err(e) => println!("文件锁定演示失败: {}", e),
    }
    
    // 内存映射文件 (使用标准库的简化版本)
    match memory_mapped_file(filename) {
        Ok(_) => println!("内存映射文件演示成功"),
        Err(e) => println!("内存映射文件演示失败: {}", e),
    }
    
    // 清理测试文件
    let _ = remove_file(filename);
}

// I/O错误处理
fn io_error_handling() {
    // 尝试读取不存在的文件
    match read_from_file("nonexistent.txt") {
        Ok(_) => println!("读取成功"),
        Err(e) => {
            println!("读取失败: {}", e);
            println!("错误类型: {:?}", e.kind());
            
            // 根据错误类型处理
            match e.kind() {
                io::ErrorKind::NotFound => println!("文件不存在"),
                io::ErrorKind::PermissionDenied => println!("权限不足"),
                io::ErrorKind::InvalidInput => println!("无效输入"),
                _ => println!("其他错误"),
            }
        }
    }
    
    // 尝试写入只读文件
    match write_to_readonly_file() {
        Ok(_) => println!("写入只读文件成功"),
        Err(e) => println!("写入只读文件失败: {}", e),
    }
    
    // 链式错误处理
    match chain_io_operations() {
        Ok(result) => println!("链式操作成功: {}", result),
        Err(e) => println!("链式操作失败: {}", e),
    }
}

// I/O性能优化
fn io_performance() {
    let filename = "performance_test.txt";
    
    // 创建大文件用于测试
    let large_content = "这是一个用于性能测试的较大文件内容。\n".repeat(1000);
    let _ = write_to_file(filename, &large_content);
    
    // 测量读取性能
    let start = std::time::Instant::now();
    match read_from_file(filename) {
        Ok(_) => {
            let duration = start.elapsed();
            println!("普通读取耗时: {:?}", duration);
        }
        Err(e) => println!("读取失败: {}", e),
    }
    
    // 测量缓冲读取性能
    let start = std::time::Instant::now();
    match buffered_read(filename) {
        Ok(_) => {
            let duration = start.elapsed();
            println!("缓冲读取耗时: {:?}", duration);
        }
        Err(e) => println!("缓冲读取失败: {}", e),
    }
    
    // 性能优化建议
    println!("\nI/O性能优化建议:");
    println!("1. 使用缓冲I/O减少系统调用");
    println!("2. 适当调整缓冲区大小");
    println!("3. 批量操作代替单个操作");
    println!("4. 使用异步I/O处理大量并发");
    println!("5. 考虑使用内存映射文件");
    println!("6. 避免频繁的小文件操作");
    
    // 清理测试文件
    let _ = remove_file(filename);
}

// 实际应用示例
fn practical_examples() {
    // 配置文件处理
    config_file_example();
    
    // 日志文件处理
    log_file_example();
    
    // CSV文件处理
    csv_file_example();
    
    // 命令行工具示例
    command_line_tool_example();
}

// 辅助函数实现

// 写入文件
fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?; // 确保数据写入磁盘
    Ok(())
}

// 读取文件
fn read_from_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 追加到文件
fn append_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

// 缓冲写入
fn buffered_write(filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    
    writeln!(writer, "第一行缓冲内容")?;
    writeln!(writer, "第二行缓冲内容")?;
    writeln!(writer, "第三行缓冲内容")?;
    
    // 显式刷新缓冲区
    writer.flush()?;
    Ok(())
}

// 缓冲读取
fn buffered_read(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

// 逐行读取
fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

// 写入二进制数据
fn write_binary_data(filename: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data)?;
    file.sync_all()?;
    Ok(())
}

// 读取二进制数据
fn read_binary_data(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

// 写入数字数据
fn write_numbers(filename: &str, numbers: &[u32]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for &number in numbers {
        file.write_all(&number.to_le_bytes())?;
    }
    file.sync_all()?;
    Ok(())
}

// 读取数字数据
fn read_numbers(filename: &str) -> io::Result<Vec<u32>> {
    let mut file = File::open(filename)?;
    let mut numbers = Vec::new();
    let mut buffer = [0u8; 4];
    
    while file.read_exact(&mut buffer).is_ok() {
        numbers.push(u32::from_le_bytes(buffer));
    }
    
    Ok(numbers)
}

// 随机访问文件
fn random_access_file(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    
    // 移动到文件开头
    file.seek(SeekFrom::Start(0))?;
    println!("文件位置: {}", file.stream_position()?);
    
    // 读取前5个字节
    let mut buffer = vec![0u8; 5];
    file.read_exact(&mut buffer)?;
    println!("前5个字节: {}", String::from_utf8_lossy(&buffer));
    
    // 移动到文件末尾
    let end_pos = file.seek(SeekFrom::End(0))?;
    println!("文件大小: {} 字节", end_pos);
    
    // 移动到文件中间
    file.seek(SeekFrom::Start(end_pos / 2))?;
    println!("移动到中间位置: {}", file.stream_position()?);
    
    Ok(())
}

// 文件锁定演示
fn file_locking_demo(filename: &str) -> io::Result<()> {
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    
    // 注意：标准库不直接支持文件锁定
    // 在实际应用中，您可能需要使用第三方库如 fs2
    println!("文件锁定演示 (需要外部库支持)");
    
    Ok(())
}

// 内存映射文件演示
fn memory_mapped_file(filename: &str) -> io::Result<()> {
    // 标准库不直接支持内存映射
    // 这里只是演示概念
    let content = read_from_file(filename)?;
    println!("内存映射文件内容长度: {} 字节", content.len());
    println!("内容预览: {}", &content[..content.len().min(50)]);
    Ok(())
}

// 尝试写入只读文件
fn write_to_readonly_file() -> io::Result<()> {
    let filename = "readonly_test.txt";
    
    // 创建文件
    let mut file = File::create(filename)?;
    writeln!(file, "只读文件内容")?;
    drop(file);
    
    // 设置为只读
    let mut permissions = std::fs::metadata(filename)?.permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(filename, permissions)?;
    
    // 尝试写入只读文件
    let result = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename);
    
    // 清理
    let _ = remove_file(filename);
    
    result.map(|_| ())
}

// 链式I/O操作
fn chain_io_operations() -> io::Result<String> {
    let filename = "chain_test.txt";
    
    // 写入文件
    write_to_file(filename, "链式操作测试")?;
    
    // 读取文件
    let content = read_from_file(filename)?;
    
    // 清理
    remove_file(filename)?;
    
    Ok(content)
}

// 配置文件处理示例
fn config_file_example() {
    let config_content = r#"
# 应用配置文件
name = "MyApp"
version = "1.0.0"
debug = true
port = 8080
"#;
    
    let filename = "config.toml";
    
    // 写入配置文件
    if let Err(e) = write_to_file(filename, config_content) {
        println!("写入配置文件失败: {}", e);
        return;
    }
    
    // 读取配置文件
    match read_from_file(filename) {
        Ok(content) => {
            println!("配置文件内容:");
            for line in content.lines() {
                if !line.trim().is_empty() && !line.starts_with('#') {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => println!("读取配置文件失败: {}", e),
    }
    
    // 清理
    let _ = remove_file(filename);
}

// 日志文件处理示例
fn log_file_example() {
    let log_filename = "app.log";
    
    // 写入日志
    let log_entries = vec![
        "2023-01-01 10:00:00 INFO 应用启动",
        "2023-01-01 10:00:01 DEBUG 初始化配置",
        "2023-01-01 10:00:02 WARN 配置文件不存在，使用默认值",
        "2023-01-01 10:00:03 ERROR 连接数据库失败",
        "2023-01-01 10:00:04 INFO 使用备用数据库",
    ];
    
    for entry in log_entries {
        if let Err(e) = append_to_file(log_filename, &format!("{}\n", entry)) {
            println!("写入日志失败: {}", e);
        }
    }
    
    // 读取日志
    match read_lines(log_filename) {
        Ok(lines) => {
            println!("日志文件内容:");
            for line in lines {
                if line.contains("ERROR") {
                    println!("  \x1b[31m{}\x1b[0m", line); // 红色显示错误
                } else if line.contains("WARN") {
                    println!("  \x1b[33m{}\x1b[0m", line); // 黄色显示警告
                } else {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => println!("读取日志失败: {}", e),
    }
    
    // 清理
    let _ = remove_file(log_filename);
}

// CSV文件处理示例
fn csv_file_example() {
    let csv_content = "姓名,年龄,城市\n张三,25,北京\n李四,30,上海\n王五,28,广州\n";
    let filename = "data.csv";
    
    // 写入CSV文件
    if let Err(e) = write_to_file(filename, csv_content) {
        println!("写入CSV文件失败: {}", e);
        return;
    }
    
    // 读取并解析CSV文件
    match read_lines(filename) {
        Ok(lines) => {
            println!("CSV文件内容:");
            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    println!("  表头: {}", line);
                } else {
                    let fields: Vec<&str> = line.split(',').collect();
                    if fields.len() >= 3 {
                        println!("  数据: 姓名={}, 年龄={}, 城市={}", 
                                fields[0], fields[1], fields[2]);
                    }
                }
            }
        }
        Err(e) => println!("读取CSV文件失败: {}", e),
    }
    
    // 清理
    let _ = remove_file(filename);
}

// 命令行工具示例
fn command_line_tool_example() {
    println!("命令行工具示例:");
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    println!("命令行参数: {:?}", args);
    
    // 获取环境变量
    match env::var("PATH") {
        Ok(path) => println!("PATH环境变量长度: {}", path.len()),
        Err(_) => println!("获取PATH环境变量失败"),
    }
    
    // 执行系统命令
    match Command::new("echo")
        .arg("Hello from system command!")
        .output()
    {
        Ok(output) => {
            println!("命令执行成功:");
            println!("  输出: {}", String::from_utf8_lossy(&output.stdout));
            println!("  错误: {}", String::from_utf8_lossy(&output.stderr));
            println!("  退出码: {}", output.status);
        }
        Err(e) => println!("命令执行失败: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_file_write_read() {
        let filename = "test_write_read.txt";
        let content = "测试内容";
        
        // 写入文件
        assert!(write_to_file(filename, content).is_ok());
        
        // 读取文件
        let read_content = read_from_file(filename).unwrap();
        assert_eq!(read_content, content);
        
        // 清理
        let _ = remove_file(filename);
    }
    
    #[test]
    fn test_buffered_operations() {
        let filename = "test_buffered.txt";
        
        // 缓冲写入
        assert!(buffered_write(filename).is_ok());
        
        // 缓冲读取
        let lines = buffered_read(filename).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "第一行缓冲内容");
        
        // 清理
        let _ = remove_file(filename);
    }
    
    #[test]
    fn test_binary_data() {
        let filename = "test_binary.bin";
        let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
        
        // 写入二进制数据
        assert!(write_binary_data(filename, &data).is_ok());
        
        // 读取二进制数据
        let read_data = read_binary_data(filename).unwrap();
        assert_eq!(read_data, data);
        
        // 清理
        let _ = remove_file(filename);
    }
    
    #[test]
    fn test_number_operations() {
        let filename = "test_numbers.bin";
        let numbers = vec![1u32, 2u32, 3u32, 4u32, 5u32];
        
        // 写入数字
        assert!(write_numbers(filename, &numbers).is_ok());
        
        // 读取数字
        let read_numbers = read_numbers(filename).unwrap();
        assert_eq!(read_numbers, numbers);
        
        // 清理
        let _ = remove_file(filename);
    }
    
    #[test]
    fn test_error_handling() {
        // 测试读取不存在的文件
        let result = read_from_file("nonexistent_file.txt");
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
        }
    }
    
    #[test]
    fn test_directory_operations() {
        let test_dir = "test_dir_ops";
        
        // 创建目录
        assert!(create_dir_all(test_dir).is_ok());
        
        // 检查目录是否存在
        assert!(Path::new(test_dir).exists());
        
        // 清理
        let _ = remove_dir_all(test_dir);
    }
}