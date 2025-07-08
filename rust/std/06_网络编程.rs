// 06_网络编程.rs
// Rust标准库网络编程详解

/*
Rust标准库的std::net模块提供了网络编程的基础功能：

核心类型：
- TcpListener：TCP服务器监听器
- TcpStream：TCP连接流
- UdpSocket：UDP套接字
- SocketAddr：网络地址
- IpAddr：IP地址（IPv4/IPv6）

网络编程特点：
- 阻塞I/O：标准库提供同步网络API
- 跨平台：在不同操作系统上提供一致的接口
- 线程安全：可以在多线程环境中安全使用
- 错误处理：完善的错误处理机制

常用场景：
- TCP服务器/客户端
- UDP通信
- HTTP客户端（基础）
- 文件传输
- 实时通信
- 网络工具开发

注意事项：
- 标准库只提供基础网络功能
- 复杂应用建议使用tokio等异步框架
- 需要处理网络异常和重连
- 考虑防火墙和网络安全
*/

use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::collections::HashMap;

fn main() {
    println!("=== Rust标准库网络编程 ===");
    
    // 1. 网络地址处理
    println!("\n1. 网络地址处理：");
    address_handling();
    
    // 2. TCP客户端
    println!("\n2. TCP客户端：");
    tcp_client_example();
    
    // 3. TCP服务器
    println!("\n3. TCP服务器：");
    tcp_server_example();
    
    // 4. UDP通信
    println!("\n4. UDP通信：");
    udp_communication();
    
    // 5. 多线程网络服务器
    println!("\n5. 多线程网络服务器：");
    multithreaded_server();
    
    // 6. 网络工具函数
    println!("\n6. 网络工具函数：");
    network_utilities();
    
    // 7. 错误处理和重连
    println!("\n7. 错误处理和重连：");
    error_handling_and_retry();
    
    // 8. 简单的HTTP客户端
    println!("\n8. 简单的HTTP客户端：");
    simple_http_client();
    
    // 9. 网络性能测试
    println!("\n9. 网络性能测试：");
    network_performance_test();
    
    // 10. 实际应用示例
    println!("\n10. 实际应用示例：");
    practical_examples();
    
    println!("\n=== 网络编程学习完成 ===");
}

// 网络地址处理
fn address_handling() {
    // IPv4 地址
    let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    println!("IPv4地址: {}", ipv4);
    println!("是否为本地回环: {}", ipv4.is_loopback());
    println!("是否为私有地址: {}", ipv4.is_private());
    
    // IPv6 地址
    let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    println!("IPv6地址: {}", ipv6);
    println!("是否为本地回环: {}", ipv6.is_loopback());
    
    // IP地址枚举
    let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    println!("IP地址: {}", ip_addr);
    
    match ip_addr {
        IpAddr::V4(ipv4) => println!("这是IPv4地址: {}", ipv4),
        IpAddr::V6(ipv6) => println!("这是IPv6地址: {}", ipv6),
    }
    
    // Socket地址
    let socket_addr = SocketAddr::new(ip_addr, 8080);
    println!("Socket地址: {}", socket_addr);
    println!("IP: {}, 端口: {}", socket_addr.ip(), socket_addr.port());
    
    // 地址解析
    let addresses: Vec<SocketAddr> = "google.com:80"
        .to_socket_addrs()
        .unwrap_or_else(|_| Vec::new())
        .collect();
    
    if !addresses.is_empty() {
        println!("google.com:80 解析的地址:");
        for addr in addresses.iter().take(3) {
            println!("  {}", addr);
        }
    }
    
    // 本地地址
    println!("常用本地地址:");
    println!("  localhost:8080 -> 127.0.0.1:8080");
    println!("  0.0.0.0:8080 -> 监听所有接口");
    println!("  [::1]:8080 -> IPv6本地回环");
}

// TCP客户端示例
fn tcp_client_example() {
    println!("TCP客户端连接示例:");
    
    // 尝试连接到一个不存在的服务器（演示错误处理）
    match TcpStream::connect_timeout(
        &"127.0.0.1:12345".parse().unwrap(),
        Duration::from_secs(1)
    ) {
        Ok(mut stream) => {
            println!("连接成功!");
            
            // 发送数据
            let message = "Hello from TCP client!";
            if let Err(e) = stream.write_all(message.as_bytes()) {
                println!("发送数据失败: {}", e);
            }
            
            // 读取响应
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    let response = String::from_utf8_lossy(&buffer[..size]);
                    println!("服务器响应: {}", response);
                }
                Err(e) => println!("读取响应失败: {}", e),
            }
        }
        Err(e) => {
            println!("连接失败 (这是预期的): {}", e);
            println!("错误类型: {:?}", e.kind());
        }
    }
    
    // 演示HTTP客户端请求
    http_get_request("httpbin.org", 80, "/ip");
}

// TCP服务器示例
fn tcp_server_example() {
    println!("TCP服务器示例:");
    
    // 启动一个简单的回声服务器
    let server_handle = thread::spawn(|| {
        start_echo_server("127.0.0.1:8081")
    });
    
    // 等待服务器启动
    thread::sleep(Duration::from_millis(100));
    
    // 测试客户端连接
    match TcpStream::connect("127.0.0.1:8081") {
        Ok(mut stream) => {
            println!("客户端连接成功");
            
            // 发送测试消息
            let messages = vec!["Hello", "World", "Rust", "Network"];
            
            for message in messages {
                // 发送消息
                if let Err(e) = writeln!(stream, "{}", message) {
                    println!("发送失败: {}", e);
                    break;
                }
                
                // 读取回声
                let mut reader = BufReader::new(&stream);
                let mut response = String::new();
                
                match reader.read_line(&mut response) {
                    Ok(_) => println!("回声: {}", response.trim()),
                    Err(e) => {
                        println!("读取失败: {}", e);
                        break;
                    }
                }
                
                thread::sleep(Duration::from_millis(100));
            }
            
            // 发送退出信号
            let _ = writeln!(stream, "quit");
        }
        Err(e) => println!("客户端连接失败: {}", e),
    }
    
    // 等待服务器线程结束
    let _ = server_handle.join();
}

// UDP通信示例
fn udp_communication() {
    println!("UDP通信示例:");
    
    // 启动UDP服务器
    let server_handle = thread::spawn(|| {
        start_udp_server("127.0.0.1:8082")
    });
    
    // 等待服务器启动
    thread::sleep(Duration::from_millis(100));
    
    // UDP客户端
    match UdpSocket::bind("127.0.0.1:0") {
        Ok(socket) => {
            println!("UDP客户端启动成功");
            
            let server_addr = "127.0.0.1:8082";
            let messages = vec!["UDP Hello", "UDP World", "UDP Test"];
            
            for message in messages {
                // 发送消息
                match socket.send_to(message.as_bytes(), server_addr) {
                    Ok(sent) => println!("发送了 {} 字节: {}", sent, message),
                    Err(e) => {
                        println!("发送失败: {}", e);
                        continue;
                    }
                }
                
                // 接收响应
                let mut buffer = [0; 1024];
                match socket.recv_from(&mut buffer) {
                    Ok((received, from)) => {
                        let response = String::from_utf8_lossy(&buffer[..received]);
                        println!("从 {} 接收到 {} 字节: {}", from, received, response);
                    }
                    Err(e) => println!("接收失败: {}", e),
                }
                
                thread::sleep(Duration::from_millis(100));
            }
            
            // 发送退出信号
            let _ = socket.send_to(b"quit", server_addr);
        }
        Err(e) => println!("UDP客户端启动失败: {}", e),
    }
    
    // 等待服务器线程结束
    let _ = server_handle.join();
}

// 多线程网络服务器
fn multithreaded_server() {
    println!("多线程TCP服务器示例:");
    
    let (tx, rx) = mpsc::channel();
    
    // 启动多线程服务器
    let server_handle = thread::spawn(move || {
        start_multithreaded_server("127.0.0.1:8083", rx)
    });
    
    // 等待服务器启动
    thread::sleep(Duration::from_millis(100));
    
    // 创建多个客户端连接
    let mut client_handles = Vec::new();
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            match TcpStream::connect("127.0.0.1:8083") {
                Ok(mut stream) => {
                    println!("客户端 {} 连接成功", i);
                    
                    // 发送消息
                    let message = format!("客户端 {} 的消息", i);
                    if let Err(e) = writeln!(stream, "{}", message) {
                        println!("客户端 {} 发送失败: {}", i, e);
                        return;
                    }
                    
                    // 读取响应
                    let mut reader = BufReader::new(&stream);
                    let mut response = String::new();
                    
                    match reader.read_line(&mut response) {
                        Ok(_) => println!("客户端 {} 收到响应: {}", i, response.trim()),
                        Err(e) => println!("客户端 {} 读取失败: {}", i, e),
                    }
                }
                Err(e) => println!("客户端 {} 连接失败: {}", i, e),
            }
        });
        
        client_handles.push(handle);
    }
    
    // 等待所有客户端完成
    for handle in client_handles {
        let _ = handle.join();
    }
    
    // 停止服务器
    let _ = tx.send(());
    let _ = server_handle.join();
}

// 网络工具函数
fn network_utilities() {
    // 端口扫描器
    println!("端口扫描示例 (localhost):");
    let common_ports = vec![22, 80, 443, 3306, 5432, 6379, 8080];
    
    for port in common_ports {
        let addr = format!("127.0.0.1:{}", port);
        match TcpStream::connect_timeout(
            &addr.parse().unwrap(),
            Duration::from_millis(100)
        ) {
            Ok(_) => println!("  端口 {} 开放", port),
            Err(_) => println!("  端口 {} 关闭", port),
        }
    }
    
    // 网络延迟测试
    println!("\n网络延迟测试:");
    let test_addresses = vec!["8.8.8.8:53", "1.1.1.1:53"];
    
    for addr in test_addresses {
        let start = std::time::Instant::now();
        match TcpStream::connect_timeout(
            &addr.parse().unwrap(),
            Duration::from_secs(2)
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                println!("  {} 连接延迟: {:?}", addr, duration);
            }
            Err(e) => println!("  {} 连接失败: {}", addr, e),
        }
    }
}

// 错误处理和重连
fn error_handling_and_retry() {
    println!("网络错误处理和重连示例:");
    
    // 重连机制
    let max_retries = 3;
    let retry_delay = Duration::from_millis(500);
    
    for attempt in 1..=max_retries {
        println!("尝试连接... (第{}/{}次)", attempt, max_retries);
        
        match TcpStream::connect_timeout(
            &"127.0.0.1:99999".parse().unwrap(),
            Duration::from_millis(500)
        ) {
            Ok(_) => {
                println!("连接成功!");
                break;
            }
            Err(e) => {
                println!("连接失败: {}", e);
                
                if attempt < max_retries {
                    println!("等待 {:?} 后重试...", retry_delay);
                    thread::sleep(retry_delay);
                } else {
                    println!("达到最大重试次数，放弃连接");
                }
            }
        }
    }
    
    // 超时处理
    println!("\n超时处理示例:");
    match TcpStream::connect_timeout(
        &"1.2.3.4:80".parse().unwrap(),
        Duration::from_millis(1000)
    ) {
        Ok(_) => println!("连接成功"),
        Err(e) => {
            println!("连接超时或失败: {}", e);
            
            use std::io::ErrorKind;
            match e.kind() {
                ErrorKind::TimedOut => println!("这是超时错误"),
                ErrorKind::ConnectionRefused => println!("连接被拒绝"),
                ErrorKind::NotFound => println!("地址未找到"),
                _ => println!("其他网络错误"),
            }
        }
    }
}

// 简单的HTTP客户端
fn simple_http_client() {
    println!("简单HTTP客户端示例:");
    
    // 发起HTTP GET请求
    http_get_request("httpbin.org", 80, "/user-agent");
    
    // 发起HTTP POST请求
    http_post_request("httpbin.org", 80, "/post", "test=data&name=rust");
}

// 网络性能测试
fn network_performance_test() {
    println!("网络性能测试:");
    
    // 带宽测试
    let data_size = 1024 * 1024; // 1MB
    let test_data = vec![0u8; data_size];
    
    println!("准备发送 {} 字节数据进行性能测试", data_size);
    
    // 启动测试服务器
    let server_handle = thread::spawn(|| {
        start_performance_test_server("127.0.0.1:8084")
    });
    
    thread::sleep(Duration::from_millis(100));
    
    // 性能测试客户端
    match TcpStream::connect("127.0.0.1:8084") {
        Ok(mut stream) => {
            let start = std::time::Instant::now();
            
            match stream.write_all(&test_data) {
                Ok(_) => {
                    let duration = start.elapsed();
                    let throughput = data_size as f64 / duration.as_secs_f64() / 1024.0 / 1024.0;
                    println!("发送完成，耗时: {:?}", duration);
                    println!("吞吐量: {:.2} MB/s", throughput);
                }
                Err(e) => println!("发送失败: {}", e),
            }
        }
        Err(e) => println!("连接失败: {}", e),
    }
    
    let _ = server_handle.join();
}

// 实际应用示例
fn practical_examples() {
    // 聊天服务器示例
    println!("聊天服务器概念演示:");
    println!("- 多客户端连接管理");
    println!("- 消息广播机制");
    println!("- 用户认证和会话管理");
    
    // 文件传输示例
    println!("\n文件传输概念演示:");
    println!("- 文件分块传输");
    println!("- 传输进度跟踪");
    println!("- 断点续传支持");
    
    // 代理服务器示例
    println!("\n代理服务器概念演示:");
    println!("- 请求转发");
    println!("- 负载均衡");
    println!("- 连接池管理");
}

// 辅助函数实现

// 启动回声服务器
fn start_echo_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("回声服务器启动在: {}", addr);
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                let mut line = String::new();
                
                loop {
                    line.clear();
                    match reader.read_line(&mut line) {
                        Ok(0) => break, // 连接关闭
                        Ok(_) => {
                            let trimmed = line.trim();
                            if trimmed == "quit" {
                                println!("客户端请求退出");
                                return Ok(());
                            }
                            
                            // 回声
                            if let Err(e) = writeln!(writer, "回声: {}", trimmed) {
                                println!("写入失败: {}", e);
                                break;
                            }
                            
                            if let Err(e) = writer.flush() {
                                println!("刷新失败: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("读取失败: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("连接失败: {}", e),
        }
    }
    
    Ok(())
}

// 启动UDP服务器
fn start_udp_server(addr: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind(addr)?;
    println!("UDP服务器启动在: {}", addr);
    
    let mut buffer = [0; 1024];
    
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((received, from)) => {
                let message = String::from_utf8_lossy(&buffer[..received]);
                
                if message.trim() == "quit" {
                    println!("UDP服务器收到退出信号");
                    break;
                }
                
                println!("UDP服务器收到: {} 从 {}", message.trim(), from);
                
                // 发送回声
                let response = format!("UDP回声: {}", message.trim());
                if let Err(e) = socket.send_to(response.as_bytes(), from) {
                    println!("UDP发送失败: {}", e);
                }
            }
            Err(e) => {
                println!("UDP接收失败: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

// 启动多线程服务器
fn start_multithreaded_server(addr: &str, shutdown_rx: mpsc::Receiver<()>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;
    println!("多线程服务器启动在: {}", addr);
    
    let mut client_count = 0;
    
    loop {
        // 检查是否收到停止信号
        if shutdown_rx.try_recv().is_ok() {
            println!("多线程服务器收到停止信号");
            break;
        }
        
        match listener.accept() {
            Ok((stream, addr)) => {
                client_count += 1;
                let client_id = client_count;
                println!("客户端 {} 连接: {}", client_id, addr);
                
                thread::spawn(move || {
                    handle_client(stream, client_id);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // 非阻塞模式下没有连接，继续循环
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                println!("接受连接失败: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

// 处理客户端连接
fn handle_client(mut stream: TcpStream, client_id: usize) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut line = String::new();
    
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("客户端 {} 断开连接", client_id);
                break;
            }
            Ok(_) => {
                let message = line.trim();
                println!("客户端 {} 发送: {}", client_id, message);
                
                // 发送响应
                let response = format!("服务器收到客户端 {} 的消息: {}", client_id, message);
                if let Err(e) = writeln!(writer, "{}", response) {
                    println!("发送响应失败: {}", e);
                    break;
                }
                
                if let Err(e) = writer.flush() {
                    println!("刷新缓冲区失败: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("读取客户端 {} 消息失败: {}", client_id, e);
                break;
            }
        }
    }
}

// HTTP GET请求
fn http_get_request(host: &str, port: u16, path: &str) {
    match TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        Duration::from_secs(5)
    ) {
        Ok(mut stream) => {
            // 构造HTTP请求
            let request = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                path, host
            );
            
            // 发送请求
            if let Err(e) = stream.write_all(request.as_bytes()) {
                println!("发送HTTP请求失败: {}", e);
                return;
            }
            
            // 读取响应
            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => {
                    let lines: Vec<&str> = response.lines().collect();
                    if !lines.is_empty() {
                        println!("HTTP响应状态: {}", lines[0]);
                        
                        // 查找响应体
                        if let Some(body_start) = response.find("\r\n\r\n") {
                            let body = &response[body_start + 4..];
                            if !body.is_empty() {
                                println!("响应体预览: {}...", 
                                    &body[..body.len().min(100)]);
                            }
                        }
                    }
                }
                Err(e) => println!("读取HTTP响应失败: {}", e),
            }
        }
        Err(e) => println!("HTTP连接失败: {}", e),
    }
}

// HTTP POST请求
fn http_post_request(host: &str, port: u16, path: &str, data: &str) {
    match TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        Duration::from_secs(5)
    ) {
        Ok(mut stream) => {
            // 构造HTTP POST请求
            let request = format!(
                "POST {} HTTP/1.1\r\n\
                 Host: {}\r\n\
                 Content-Type: application/x-www-form-urlencoded\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\r\n\
                 {}",
                path, host, data.len(), data
            );
            
            // 发送请求
            if let Err(e) = stream.write_all(request.as_bytes()) {
                println!("发送HTTP POST请求失败: {}", e);
                return;
            }
            
            // 读取响应
            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => {
                    let lines: Vec<&str> = response.lines().collect();
                    if !lines.is_empty() {
                        println!("HTTP POST响应状态: {}", lines[0]);
                    }
                }
                Err(e) => println!("读取HTTP POST响应失败: {}", e),
            }
        }
        Err(e) => println!("HTTP POST连接失败: {}", e),
    }
}

// 性能测试服务器
fn start_performance_test_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("性能测试服务器启动在: {}", addr);
    
    if let Ok((mut stream, _)) = listener.accept() {
        let mut total_received = 0;
        let mut buffer = [0; 8192];
        
        let start = std::time::Instant::now();
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break, // 连接关闭
                Ok(size) => {
                    total_received += size;
                }
                Err(e) => {
                    println!("性能测试读取失败: {}", e);
                    break;
                }
            }
        }
        
        let duration = start.elapsed();
        println!("性能测试服务器接收了 {} 字节，耗时: {:?}", total_received, duration);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    #[test]
    fn test_address_parsing() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(addr.port(), 8080);
    }
    
    #[test]
    fn test_ipv4_properties() {
        let localhost = Ipv4Addr::new(127, 0, 0, 1);
        assert!(localhost.is_loopback());
        
        let private_ip = Ipv4Addr::new(192, 168, 1, 1);
        assert!(private_ip.is_private());
    }
    
    #[test]
    fn test_tcp_connection_error() {
        // 尝试连接到不存在的端口
        let result = TcpStream::connect_timeout(
            &"127.0.0.1:99999".parse().unwrap(),
            Duration::from_millis(100)
        );
        assert!(result.is_err());
    }
    
    #[test]
    fn test_udp_socket_binding() {
        // 绑定到任意可用端口
        let socket = UdpSocket::bind("127.0.0.1:0");
        assert!(socket.is_ok());
        
        if let Ok(socket) = socket {
            let local_addr = socket.local_addr().unwrap();
            assert_eq!(local_addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            assert!(local_addr.port() > 0);
        }
    }
    
    #[test]
    fn test_echo_server_basic() {
        let server_running = Arc::new(AtomicBool::new(true));
        let server_flag = server_running.clone();
        
        // 启动测试服务器
        let server_handle = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            println!("测试服务器地址: {}", addr);
            
            listener.set_nonblocking(true).unwrap();
            
            while server_flag.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024];
                        if let Ok(size) = stream.read(&mut buffer) {
                            let _ = stream.write_all(&buffer[..size]);
                        }
                        break;
                    }
                    Err(_) => {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });
        
        thread::sleep(Duration::from_millis(100));
        server_running.store(false, Ordering::Relaxed);
        let _ = server_handle.join();
    }
    
    #[test]
    fn test_socket_addresses_resolution() {
        // 测试localhost地址解析
        let addresses: Result<Vec<SocketAddr>, _> = "localhost:80"
            .to_socket_addrs()
            .map(|iter| iter.collect());
        
        assert!(addresses.is_ok());
        let addresses = addresses.unwrap();
        assert!(!addresses.is_empty());
        
        // 检查是否包含预期的地址
        let has_ipv4 = addresses.iter().any(|addr| {
            matches!(addr.ip(), IpAddr::V4(ip) if ip == Ipv4Addr::new(127, 0, 0, 1))
        });
        
        assert!(has_ipv4 || addresses.iter().any(|addr| addr.ip().is_loopback()));
    }
}