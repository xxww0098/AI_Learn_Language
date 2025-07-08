// 10_文件系统操作.rs
// Rust标准库文件系统操作详解

/*
std::fs 模块提供了文件系统操作的核心功能：

主要类型和函数：
- File：文件句柄
- OpenOptions：文件打开选项配置
- DirEntry：目录条目
- Metadata：文件元数据
- Permissions：文件权限

核心操作：
- 文件读写：read(), write(), create()
- 目录操作：create_dir(), read_dir(), remove_dir()
- 文件管理：copy(), rename(), remove_file()
- 权限管理：set_permissions()
- 符号链接：symlink(), read_link()

std::path 模块：
- Path：路径引用
- PathBuf：拥有所有权的路径
- 路径操作：join(), parent(), file_name(), extension()

特点：
- 跨平台：自动处理不同操作系统的路径分隔符
- 安全性：防止路径遍历攻击
- 性能：高效的文件系统操作
- 错误处理：完善的错误信息
*/

use std::fs::{self, File, OpenOptions, DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::os::unix::fs::PermissionsExt; // Unix系统特定
use std::time::SystemTime;

fn main() {
    println!("=== Rust标准库文件系统操作 ===");
    
    // 1. 基本文件操作
    println!("\n1. 基本文件操作：");
    basic_file_operations();
    
    // 2. 目录操作
    println!("\n2. 目录操作：");
    directory_operations();
    
    // 3. 路径处理
    println!("\n3. 路径处理：");
    path_operations();
    
    // 4. 文件元数据
    println!("\n4. 文件元数据：");
    metadata_operations();
    
    // 5. 文件权限
    println!("\n5. 文件权限：");
    permission_operations();
    
    // 6. 高级文件操作
    println!("\n6. 高级文件操作：");
    advanced_file_operations();
    
    // 7. 文件监控
    println!("\n7. 文件监控：");
    file_monitoring();
    
    // 8. 临时文件处理
    println!("\n8. 临时文件处理：");
    temporary_file_handling();
    
    // 9. 文件搜索和过滤
    println!("\n9. 文件搜索和过滤：");
    file_search_and_filter();
    
    // 10. 最佳实践
    println!("\n10. 最佳实践：");
    best_practices();
    
    println!("\n=== 文件系统操作学习完成 ===");
}

// 基本文件操作
fn basic_file_operations() {
    let test_file = "test_basic.txt";
    
    // 创建并写入文件
    match File::create(test_file) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "Hello, Rust!") {
                println!("写入失败: {}", e);
            } else {
                println!("文件创建并写入成功");
            }
        }
        Err(e) => println!("文件创建失败: {}", e),
    }
    
    // 读取文件内容
    match fs::read_to_string(test_file) {
        Ok(content) => println!("文件内容: {}", content.trim()),
        Err(e) => println!("读取失败: {}", e),
    }
    
    // 追加内容
    match OpenOptions::new().append(true).open(test_file) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "追加的内容") {
                println!("追加失败: {}", e);
            } else {
                println!("内容追加成功");
            }
        }
        Err(e) => println!("打开文件失败: {}", e),
    }
    
    // 再次读取验证
    if let Ok(content) = fs::read_to_string(test_file) {
        println!("追加后内容:\n{}", content);
    }
    
    // 文件复制
    let copy_file = "test_copy.txt";
    match fs::copy(test_file, copy_file) {
        Ok(bytes) => println!("复制了 {} 字节", bytes),
        Err(e) => println!("复制失败: {}", e),
    }
    
    // 文件重命名
    let new_name = "test_renamed.txt";
    match fs::rename(copy_file, new_name) {
        Ok(_) => println!("文件重命名成功"),
        Err(e) => println!("重命名失败: {}", e),
    }
    
    // 清理测试文件
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(new_name);
}

// 目录操作
fn directory_operations() {
    let test_dir = "test_directory";
    let nested_dir = "test_directory/nested";
    
    // 创建目录
    match fs::create_dir(test_dir) {
        Ok(_) => println!("目录创建成功: {}", test_dir),
        Err(e) => println!("目录创建失败: {}", e),
    }
    
    // 创建嵌套目录
    match fs::create_dir_all(nested_dir) {
        Ok(_) => println!("嵌套目录创建成功: {}", nested_dir),
        Err(e) => println!("嵌套目录创建失败: {}", e),
    }
    
    // 在目录中创建文件
    let file_in_dir = format!("{}/test.txt", test_dir);
    let nested_file = format!("{}/nested.txt", nested_dir);
    
    if let Ok(mut file) = File::create(&file_in_dir) {
        let _ = writeln!(file, "目录中的文件");
    }
    
    if let Ok(mut file) = File::create(&nested_file) {
        let _ = writeln!(file, "嵌套目录中的文件");
    }
    
    // 读取目录内容
    match fs::read_dir(test_dir) {
        Ok(entries) => {
            println!("目录 {} 的内容:", test_dir);
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let file_type = if path.is_dir() { "目录" } else { "文件" };
                        println!("  {}: {}", file_type, path.display());
                    }
                    Err(e) => println!("  读取条目错误: {}", e),
                }
            }
        }
        Err(e) => println!("读取目录失败: {}", e),
    }
    
    // 递归遍历目录
    println!("递归遍历目录:");
    recursive_dir_walk(Path::new(test_dir), 0);
    
    // 清理目录
    let _ = fs::remove_dir_all(test_dir);
    println!("清理完成");
}

// 递归遍历目录
fn recursive_dir_walk(dir: &Path, depth: usize) {
    let indent = "  ".repeat(depth);
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    println!("{}📁 {}", indent, path.file_name().unwrap().to_string_lossy());
                    recursive_dir_walk(&path, depth + 1);
                } else {
                    println!("{}📄 {}", indent, path.file_name().unwrap().to_string_lossy());
                }
            }
        }
    }
}

// 路径处理
fn path_operations() {
    let path = Path::new("/home/user/documents/file.txt");
    
    // 路径组件
    println!("路径分析:");
    println!("  完整路径: {}", path.display());
    println!("  文件名: {:?}", path.file_name());
    println!("  文件stem: {:?}", path.file_stem());
    println!("  扩展名: {:?}", path.extension());
    println!("  父目录: {:?}", path.parent());
    println!("  是否绝对路径: {}", path.is_absolute());
    
    // 路径构建
    let mut path_buf = PathBuf::new();
    path_buf.push("home");
    path_buf.push("user");
    path_buf.push("documents");
    path_buf.set_file_name("new_file");
    path_buf.set_extension("rs");
    
    println!("构建的路径: {}", path_buf.display());
    
    // 路径连接
    let base = Path::new("/usr/local");
    let full_path = base.join("bin").join("rust");
    println!("连接后路径: {}", full_path.display());
    
    // 路径比较
    let path1 = Path::new("./file.txt");
    let path2 = Path::new("file.txt");
    println!("路径比较: {} == {} : {}", 
             path1.display(), path2.display(), path1 == path2);
    
    // 标准化路径
    let messy_path = Path::new("./dir/../file.txt");
    if let Ok(canonical) = messy_path.canonicalize() {
        println!("标准化路径: {} -> {}", messy_path.display(), canonical.display());
    }
    
    // 相对路径
    let current_dir = std::env::current_dir().unwrap();
    println!("当前目录: {}", current_dir.display());
    
    // 跨平台路径处理
    cross_platform_paths();
}

// 跨平台路径处理
fn cross_platform_paths() {
    println!("跨平台路径处理:");
    
    #[cfg(windows)]
    {
        let windows_path = Path::new(r"C:\Users\username\Documents\file.txt");
        println!("  Windows路径: {}", windows_path.display());
    }
    
    #[cfg(unix)]
    {
        let unix_path = Path::new("/home/username/Documents/file.txt");
        println!("  Unix路径: {}", unix_path.display());
    }
    
    // 使用Path::join自动处理分隔符
    let path = Path::new("home").join("user").join("file.txt");
    println!("  自动分隔符: {}", path.display());
    
    // 路径分隔符
    println!("  主要分隔符: {:?}", std::path::MAIN_SEPARATOR);
}

// 文件元数据
fn metadata_operations() {
    let test_file = "metadata_test.txt";
    
    // 创建测试文件
    if let Ok(mut file) = File::create(test_file) {
        let _ = writeln!(file, "测试元数据的文件内容");
    }
    
    // 获取元数据
    match fs::metadata(test_file) {
        Ok(metadata) => {
            println!("文件元数据:");
            println!("  文件类型: {}", if metadata.is_file() { "文件" } 
                     else if metadata.is_dir() { "目录" } 
                     else { "其他" });
            println!("  文件大小: {} 字节", metadata.len());
            println!("  只读: {}", metadata.permissions().readonly());
            
            // 时间信息
            if let Ok(created) = metadata.created() {
                if let Ok(duration) = created.duration_since(SystemTime::UNIX_EPOCH) {
                    println!("  创建时间: {} 秒", duration.as_secs());
                }
            }
            
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                    println!("  修改时间: {} 秒", duration.as_secs());
                }
            }
            
            if let Ok(accessed) = metadata.accessed() {
                if let Ok(duration) = accessed.duration_since(SystemTime::UNIX_EPOCH) {
                    println!("  访问时间: {} 秒", duration.as_secs());
                }
            }
            
            // Unix特定信息
            #[cfg(unix)]
            {
                println!("  Unix权限: {:o}", metadata.permissions().mode());
            }
        }
        Err(e) => println!("获取元数据失败: {}", e),
    }
    
    // 检查路径是否存在
    println!("路径存在性检查:");
    println!("  {} 存在: {}", test_file, Path::new(test_file).exists());
    println!("  不存在的文件 存在: {}", Path::new("nonexistent.txt").exists());
    
    // 清理
    let _ = fs::remove_file(test_file);
}

// 文件权限
fn permission_operations() {
    let test_file = "permission_test.txt";
    
    // 创建测试文件
    if let Ok(mut file) = File::create(test_file) {
        let _ = writeln!(file, "权限测试文件");
    }
    
    // 获取当前权限
    if let Ok(metadata) = fs::metadata(test_file) {
        let permissions = metadata.permissions();
        println!("当前权限:");
        println!("  只读: {}", permissions.readonly());
        
        #[cfg(unix)]
        {
            println!("  Unix模式: {:o}", permissions.mode());
        }
    }
    
    // 设置为只读
    if let Ok(metadata) = fs::metadata(test_file) {
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        
        match fs::set_permissions(test_file, permissions) {
            Ok(_) => println!("设置为只读成功"),
            Err(e) => println!("设置权限失败: {}", e),
        }
    }
    
    // 验证权限变更
    if let Ok(metadata) = fs::metadata(test_file) {
        println!("更新后权限:");
        println!("  只读: {}", metadata.permissions().readonly());
    }
    
    // 尝试写入只读文件
    match OpenOptions::new().write(true).append(true).open(test_file) {
        Ok(_) => println!("意外：只读文件可以写入"),
        Err(e) => println!("预期：只读文件写入失败 - {}", e.kind()),
    }
    
    // 恢复权限
    if let Ok(metadata) = fs::metadata(test_file) {
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        let _ = fs::set_permissions(test_file, permissions);
    }
    
    // 清理
    let _ = fs::remove_file(test_file);
}

// 高级文件操作
fn advanced_file_operations() {
    // 符号链接（Unix系统）
    symbolic_link_operations();
    
    // 硬链接
    hard_link_operations();
    
    // 文件锁定
    file_locking_demo();
    
    // 大文件处理
    large_file_handling();
    
    // 文件比较
    file_comparison();
}

// 符号链接操作
fn symbolic_link_operations() {
    println!("符号链接操作:");
    
    let original = "original.txt";
    let link = "link.txt";
    
    // 创建原始文件
    if let Ok(mut file) = File::create(original) {
        let _ = writeln!(file, "原始文件内容");
    }
    
    // 创建符号链接
    #[cfg(unix)]
    {
        use std::os::unix::fs;
        
        match fs::symlink(original, link) {
            Ok(_) => {
                println!("  符号链接创建成功");
                
                // 读取链接目标
                match fs::read_link(link) {
                    Ok(target) => println!("  链接目标: {}", target.display()),
                    Err(e) => println!("  读取链接失败: {}", e),
                }
                
                // 通过链接读取内容
                if let Ok(content) = fs::read_to_string(link) {
                    println!("  通过链接读取: {}", content.trim());
                }
            }
            Err(e) => println!("  符号链接创建失败: {}", e),
        }
    }
    
    #[cfg(windows)]
    {
        println!("  Windows符号链接需要管理员权限");
    }
    
    // 清理
    let _ = fs::remove_file(original);
    let _ = fs::remove_file(link);
}

// 硬链接操作
fn hard_link_operations() {
    println!("硬链接操作:");
    
    let original = "hard_original.txt";
    let link = "hard_link.txt";
    
    // 创建原始文件
    if let Ok(mut file) = File::create(original) {
        let _ = writeln!(file, "硬链接测试");
    }
    
    // 创建硬链接
    match fs::hard_link(original, link) {
        Ok(_) => {
            println!("  硬链接创建成功");
            
            // 验证两个文件指向同一个inode
            if let (Ok(meta1), Ok(meta2)) = (fs::metadata(original), fs::metadata(link)) {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    println!("  原始文件inode: {}", meta1.ino());
                    println!("  链接文件inode: {}", meta2.ino());
                    println!("  inode相同: {}", meta1.ino() == meta2.ino());
                }
                
                #[cfg(not(unix))]
                {
                    println!("  文件大小相同: {}", meta1.len() == meta2.len());
                }
            }
        }
        Err(e) => println!("  硬链接创建失败: {}", e),
    }
    
    // 清理
    let _ = fs::remove_file(original);
    let _ = fs::remove_file(link);
}

// 文件锁定演示
fn file_locking_demo() {
    println!("文件锁定演示:");
    println!("  注意：Rust标准库不直接支持文件锁定");
    println!("  可以使用外部库如 fs2 或 file-lock");
    
    // 基本的文件独占访问模式
    let lock_file = "lock_test.txt";
    
    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(lock_file) 
    {
        Ok(mut file) => {
            println!("  文件独占打开成功");
            let _ = writeln!(file, "锁定期间的内容");
            
            // 在文件关闭前，其他进程难以写入
            println!("  文件使用中...");
        }
        Err(e) => println!("  文件打开失败: {}", e),
    }
    
    // 清理
    let _ = fs::remove_file(lock_file);
}

// 大文件处理
fn large_file_handling() {
    println!("大文件处理:");
    
    let large_file = "large_test.txt";
    
    // 创建较大的文件
    match File::create(large_file) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            
            for i in 0..10000 {
                if let Err(e) = writeln!(writer, "行 {}: 这是一个测试行，包含一些数据", i) {
                    println!("  写入失败: {}", e);
                    break;
                }
            }
            
            if let Err(e) = writer.flush() {
                println!("  刷新失败: {}", e);
            } else {
                println!("  大文件创建成功");
            }
        }
        Err(e) => {
            println!("  大文件创建失败: {}", e);
            return;
        }
    }
    
    // 逐行读取大文件
    match File::open(large_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut line_count = 0;
            
            for line in reader.lines() {
                match line {
                    Ok(_) => line_count += 1,
                    Err(e) => {
                        println!("  读取行失败: {}", e);
                        break;
                    }
                }
                
                // 只统计，不打印所有行
                if line_count % 1000 == 0 {
                    println!("  已读取 {} 行", line_count);
                }
            }
            
            println!("  总共读取 {} 行", line_count);
        }
        Err(e) => println!("  打开大文件失败: {}", e),
    }
    
    // 文件大小检查
    if let Ok(metadata) = fs::metadata(large_file) {
        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
        println!("  文件大小: {:.2} MB", size_mb);
    }
    
    // 清理
    let _ = fs::remove_file(large_file);
}

// 文件比较
fn file_comparison() {
    println!("文件比较:");
    
    let file1 = "compare1.txt";
    let file2 = "compare2.txt";
    let file3 = "compare3.txt";
    
    // 创建测试文件
    let content1 = "相同的内容\n第二行";
    let content2 = "相同的内容\n第二行";
    let content3 = "不同的内容\n第二行";
    
    let _ = fs::write(file1, content1);
    let _ = fs::write(file2, content2);
    let _ = fs::write(file3, content3);
    
    // 比较文件内容
    match (fs::read(file1), fs::read(file2)) {
        (Ok(data1), Ok(data2)) => {
            println!("  {} 和 {} 内容相同: {}", file1, file2, data1 == data2);
        }
        _ => println!("  文件读取失败"),
    }
    
    match (fs::read(file1), fs::read(file3)) {
        (Ok(data1), Ok(data3)) => {
            println!("  {} 和 {} 内容相同: {}", file1, file3, data1 == data3);
        }
        _ => println!("  文件读取失败"),
    }
    
    // 比较文件大小
    if let (Ok(meta1), Ok(meta3)) = (fs::metadata(file1), fs::metadata(file3)) {
        println!("  {} 大小: {} 字节", file1, meta1.len());
        println!("  {} 大小: {} 字节", file3, meta3.len());
    }
    
    // 清理
    let _ = fs::remove_file(file1);
    let _ = fs::remove_file(file2);
    let _ = fs::remove_file(file3);
}

// 文件监控
fn file_monitoring() {
    println!("文件监控:");
    println!("  标准库不直接支持文件监控");
    println!("  可以使用 notify 库进行文件系统事件监控");
    
    // 基本的轮询监控示例
    basic_file_polling();
}

// 基本的文件轮询监控
fn basic_file_polling() {
    use std::thread;
    use std::time::Duration;
    
    let monitor_file = "monitor_test.txt";
    
    // 创建初始文件
    let _ = fs::write(monitor_file, "初始内容");
    
    let mut last_modified = if let Ok(metadata) = fs::metadata(monitor_file) {
        metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
    } else {
        SystemTime::UNIX_EPOCH
    };
    
    println!("  开始监控文件: {}", monitor_file);
    
    // 模拟监控循环
    for i in 0..3 {
        thread::sleep(Duration::from_millis(100));
        
        if i == 1 {
            // 在第二次检查时修改文件
            let _ = fs::write(monitor_file, "修改后的内容");
        }
        
        if let Ok(metadata) = fs::metadata(monitor_file) {
            if let Ok(modified) = metadata.modified() {
                if modified > last_modified {
                    println!("  检测到文件修改!");
                    last_modified = modified;
                }
            }
        }
    }
    
    // 清理
    let _ = fs::remove_file(monitor_file);
}

// 临时文件处理
fn temporary_file_handling() {
    println!("临时文件处理:");
    
    // 获取临时目录
    let temp_dir = std::env::temp_dir();
    println!("  系统临时目录: {}", temp_dir.display());
    
    // 创建临时文件
    let temp_file = temp_dir.join("rust_temp_test.txt");
    
    match File::create(&temp_file) {
        Ok(mut file) => {
            let _ = writeln!(file, "这是一个临时文件");
            println!("  临时文件创建: {}", temp_file.display());
            
            // 使用临时文件
            if let Ok(content) = fs::read_to_string(&temp_file) {
                println!("  临时文件内容: {}", content.trim());
            }
        }
        Err(e) => println!("  临时文件创建失败: {}", e),
    }
    
    // 清理临时文件
    let _ = fs::remove_file(&temp_file);
    println!("  临时文件已清理");
    
    // 临时文件的最佳实践
    temp_file_best_practices();
}

// 临时文件最佳实践
fn temp_file_best_practices() {
    println!("  临时文件最佳实践:");
    println!("    1. 使用唯一的文件名避免冲突");
    println!("    2. 及时清理临时文件");
    println!("    3. 使用 RAII 确保清理");
    println!("    4. 考虑使用 tempfile 库");
    
    // RAII 临时文件示例
    struct TempFile {
        path: PathBuf,
    }
    
    impl TempFile {
        fn new(name: &str) -> std::io::Result<Self> {
            let path = std::env::temp_dir().join(name);
            File::create(&path)?;
            Ok(TempFile { path })
        }
        
        fn path(&self) -> &Path {
            &self.path
        }
    }
    
    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }
    
    // 使用 RAII 临时文件
    {
        if let Ok(temp) = TempFile::new("raii_temp.txt") {
            println!("    RAII临时文件: {}", temp.path().display());
        } // 文件在此处自动清理
    }
    
    println!("    RAII临时文件已自动清理");
}

// 文件搜索和过滤
fn file_search_and_filter() {
    println!("文件搜索和过滤:");
    
    // 创建测试目录结构
    let test_root = "search_test";
    setup_search_test_structure(test_root);
    
    // 按扩展名搜索
    search_by_extension(test_root, "txt");
    search_by_extension(test_root, "rs");
    
    // 按文件大小过滤
    filter_by_size(test_root, 50);
    
    // 按修改时间过滤
    filter_by_time(test_root);
    
    // 递归搜索特定内容
    search_content(test_root, "测试");
    
    // 清理
    let _ = fs::remove_dir_all(test_root);
}

// 设置搜索测试结构
fn setup_search_test_structure(root: &str) {
    let _ = fs::create_dir_all(format!("{}/subdir", root));
    
    // 创建不同类型的文件
    let _ = fs::write(format!("{}/file1.txt", root), "这是一个测试文件");
    let _ = fs::write(format!("{}/file2.rs", root), "// Rust源代码\nfn main() {}");
    let _ = fs::write(format!("{}/large.txt", root), "很长的内容".repeat(100));
    let _ = fs::write(format!("{}/subdir/nested.txt", root), "嵌套目录中的测试文件");
}

// 按扩展名搜索
fn search_by_extension(root: &str, ext: &str) {
    println!("  搜索 .{} 文件:", ext);
    search_files_recursive(Path::new(root), |path| {
        path.extension().map_or(false, |e| e == ext)
    });
}

// 按大小过滤
fn filter_by_size(root: &str, min_size: u64) {
    println!("  搜索大于 {} 字节的文件:", min_size);
    search_files_recursive(Path::new(root), |path| {
        fs::metadata(path).map_or(false, |meta| meta.len() > min_size)
    });
}

// 按时间过滤
fn filter_by_time(root: &str) {
    println!("  搜索最近创建的文件:");
    let now = SystemTime::now();
    let five_minutes_ago = now - Duration::from_secs(300);
    
    search_files_recursive(Path::new(root), |path| {
        fs::metadata(path)
            .and_then(|meta| meta.created())
            .map_or(false, |created| created > five_minutes_ago)
    });
}

// 搜索文件内容
fn search_content(root: &str, pattern: &str) {
    println!("  搜索包含 '{}' 的文件:", pattern);
    search_files_recursive(Path::new(root), |path| {
        if let Ok(content) = fs::read_to_string(path) {
            content.contains(pattern)
        } else {
            false
        }
    });
}

// 递归搜索文件
fn search_files_recursive<F>(dir: &Path, predicate: F) 
where
    F: Fn(&Path) -> bool + Copy,
{
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    search_files_recursive(&path, predicate);
                } else if predicate(&path) {
                    println!("    找到: {}", path.display());
                }
            }
        }
    }
}

// 最佳实践
fn best_practices() {
    println!("文件系统操作最佳实践:");
    println!("1. 始终处理文件操作错误");
    println!("2. 使用Path和PathBuf进行路径操作");
    println!("3. 及时关闭文件句柄，使用RAII");
    println!("4. 注意文件权限和安全性");
    println!("5. 大文件使用缓冲I/O");
    println!("6. 跨平台兼容性考虑");
    println!("7. 避免路径遍历攻击");
    println!("8. 使用适当的文件锁定机制");
    println!("9. 监控磁盘空间使用");
    println!("10. 定期清理临时文件");
    
    // 安全性示例
    security_examples();
    
    // 性能优化
    performance_tips();
}

// 安全性示例
fn security_examples() {
    println!("\n安全性考虑:");
    
    // 路径验证
    fn validate_path(path: &str) -> bool {
        // 简单的路径遍历检查
        !path.contains("..") && !path.starts_with('/')
    }
    
    let safe_path = "documents/file.txt";
    let unsafe_path = "../../../etc/passwd";
    
    println!("  路径验证:");
    println!("    {} 安全: {}", safe_path, validate_path(safe_path));
    println!("    {} 安全: {}", unsafe_path, validate_path(unsafe_path));
    
    // 权限检查
    println!("  权限检查:");
    println!("    创建文件前检查目录写权限");
    println!("    读取文件前检查读权限");
    println!("    操作前验证文件所有权");
    
    // 临时文件安全
    println!("  临时文件安全:");
    println!("    使用安全的临时目录");
    println!("    设置适当的文件权限");
    println!("    及时清理敏感数据");
}

// 性能优化提示
fn performance_tips() {
    println!("\n性能优化提示:");
    println!("1. 使用BufReader/BufWriter进行大文件I/O");
    println!("2. 批量操作减少系统调用");
    println!("3. 异步I/O用于高并发场景");
    println!("4. 内存映射用于大文件随机访问");
    println!("5. 预分配文件大小避免碎片");
    println!("6. 使用并行处理加速文件操作");
    println!("7. 缓存元数据减少重复查询");
    println!("8. 选择合适的缓冲区大小");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_basic_file_ops() {
        let test_file = "test_ops.txt";
        
        // 创建文件
        fs::write(test_file, "test content").unwrap();
        
        // 读取文件
        let content = fs::read_to_string(test_file).unwrap();
        assert_eq!(content, "test content");
        
        // 检查存在性
        assert!(Path::new(test_file).exists());
        
        // 清理
        fs::remove_file(test_file).unwrap();
        assert!(!Path::new(test_file).exists());
    }
    
    #[test]
    fn test_directory_ops() {
        let test_dir = "test_dir_ops";
        
        // 创建目录
        fs::create_dir(test_dir).unwrap();
        assert!(Path::new(test_dir).is_dir());
        
        // 创建文件
        let file_path = format!("{}/test.txt", test_dir);
        fs::write(&file_path, "content").unwrap();
        
        // 读取目录
        let entries: Vec<_> = fs::read_dir(test_dir).unwrap().collect();
        assert_eq!(entries.len(), 1);
        
        // 清理
        fs::remove_dir_all(test_dir).unwrap();
    }
    
    #[test]
    fn test_path_operations() {
        let path = Path::new("/home/user/file.txt");
        
        assert_eq!(path.file_name().unwrap(), "file.txt");
        assert_eq!(path.extension().unwrap(), "txt");
        assert_eq!(path.parent().unwrap(), Path::new("/home/user"));
        
        let joined = Path::new("/home").join("user").join("file.txt");
        assert_eq!(joined, path);
    }
    
    #[test]
    fn test_metadata() {
        let test_file = "test_metadata.txt";
        fs::write(test_file, "metadata test").unwrap();
        
        let metadata = fs::metadata(test_file).unwrap();
        assert!(metadata.is_file());
        assert_eq!(metadata.len(), 13);
        
        fs::remove_file(test_file).unwrap();
    }
}