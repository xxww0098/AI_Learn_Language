// 09 Rust模块系统 - 模块、包、crate和可见性
// 本章介绍Rust的模块系统：模块定义、包管理、crate组织和可见性控制

fn main() {
    // 模块基础示例
    module_basics();
    
    // 可见性控制示例
    visibility_control();
    
    // 路径和引用示例
    path_and_import();
    
    // 实际项目结构示例
    project_structure_example();
}

// 案例1：基本模块定义
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
        
        fn seat_at_table() {
            println!("安排就座");
        }
        
        pub fn take_order() {
            println!("接受订单");
            seat_at_table();  // 同一模块内可以调用私有函数
        }
    }
    
    mod serving {
        fn take_payment() {
            println!("收取费用");
        }
        
        fn serve_order() {
            println!("上菜");
        }
        
        pub fn complete_order() {
            serve_order();
            take_payment();
        }
    }
}

// 在同一crate的其他模块中使用
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  // 私有字段
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }
        
        pub fn get_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }
    
    // 公有枚举的所有变体都是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // 使用super访问父模块
    }
    
    fn cook_order() {
        println!("准备订单");
    }
}

fn deliver_order() {
    println!("配送订单");
}

fn module_basics() {
    println!("=== 模块基础示例 ===");
    
    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 使用相对路径
    front_of_house::hosting::take_order();
    
    // 访问结构体
    let mut meal = back_of_house::Breakfast::summer("黑麦面包");
    meal.toast = String::from("小麦面包");
    println!("我想要{}的早餐", meal.toast);
    println!("今天的水果是: {}", meal.get_fruit());
    
    // 不能直接访问私有字段
    // meal.seasonal_fruit = String::from("苹果");  // 编译错误
    
    // 使用公有枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    match order1 {
        back_of_house::Appetizer::Soup => println!("点了汤"),
        back_of_house::Appetizer::Salad => println!("点了沙拉"),
    }
}

// 案例2：可见性控制
mod library {
    pub mod books {
        pub struct Book {
            pub title: String,
            pub author: String,
            isbn: String,  // 私有字段
        }
        
        impl Book {
            pub fn new(title: &str, author: &str, isbn: &str) -> Book {
                Book {
                    title: title.to_string(),
                    author: author.to_string(),
                    isbn: isbn.to_string(),
                }
            }
            
            pub fn get_isbn(&self) -> &str {
                &self.isbn
            }
            
            // 私有方法
            fn validate_isbn(&self) -> bool {
                self.isbn.len() == 13
            }
            
            pub fn is_valid(&self) -> bool {
                self.validate_isbn()
            }
        }
        
        // 在模块内可见的函数
        pub(in crate::library) fn internal_function() {
            println!("这个函数只在library模块内可见");
        }
        
        // 在crate内可见的函数
        pub(crate) fn crate_visible_function() {
            println!("这个函数在整个crate内可见");
        }
        
        // 对父模块可见的函数
        pub(super) fn parent_visible_function() {
            println!("这个函数对父模块可见");
        }
    }
    
    pub mod readers {
        pub struct Reader {
            pub name: String,
            member_id: u32,  // 私有字段
        }
        
        impl Reader {
            pub fn new(name: &str, member_id: u32) -> Reader {
                Reader {
                    name: name.to_string(),
                    member_id,
                }
            }
            
            pub fn get_member_id(&self) -> u32 {
                self.member_id
            }
        }
        
        pub fn borrow_book() {
            // 可以调用同一模块层级的函数
            super::books::parent_visible_function();
            super::books::internal_function();
        }
    }
    
    pub fn library_function() {
        books::internal_function();  // 在library模块内可以调用
        books::crate_visible_function();
    }
}

fn visibility_control() {
    println!("\n=== 可见性控制示例 ===");
    
    let book = library::books::Book::new(
        "Rust程序设计语言",
        "Steve Klabnik",
        "9787121327186",
    );
    
    println!("书名: {}", book.title);
    println!("作者: {}", book.author);
    println!("ISBN: {}", book.get_isbn());
    println!("是否有效: {}", book.is_valid());
    
    let reader = library::readers::Reader::new("Alice", 12345);
    println!("读者: {}, 会员号: {}", reader.name, reader.get_member_id());
    
    // 调用模块函数
    library::library_function();
    library::readers::borrow_book();
    
    // 可以调用crate可见的函数
    library::books::crate_visible_function();
    
    // 不能调用模块内部函数
    // library::books::internal_function();  // 编译错误
}

// 案例3：使用use关键字简化路径
use std::collections::HashMap;
use std::fmt::Display;

// 使用use引入模块
use front_of_house::hosting;

// 使用as关键字创建别名
use back_of_house::Appetizer as BackAppetizer;

// 使用pub use重导出
pub use library::books::Book;

// 嵌套导入
use std::{
    collections::{BTreeMap, HashSet},
    io::{self, Write},
};

// 通配符导入（谨慎使用）
use std::collections::*;

fn path_and_import() {
    println!("\n=== 路径和引用示例 ===");
    
    // 使用简化的路径
    hosting::add_to_waitlist();
    
    // 使用别名
    let appetizer = BackAppetizer::Soup;
    
    // 使用导入的类型
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("rust".to_string(), 1);
    map.insert("go".to_string(), 2);
    
    println!("语言映射: {:?}", map);
    
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    println!("数字集合: {:?}", set);
    
    // 使用重导出的类型
    let book = Book::new("Rust Book", "The Rust Team", "1234567890123");
    println!("重导出的书: {}", book.title);
}

// 案例4：实际项目结构模拟
mod web_server {
    pub mod http {
        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
        }
        
        pub struct Request {
            pub method: Method,
            pub path: String,
            pub headers: std::collections::HashMap<String, String>,
        }
        
        impl Request {
            pub fn new(method: Method, path: &str) -> Self {
                Request {
                    method,
                    path: path.to_string(),
                    headers: std::collections::HashMap::new(),
                }
            }
            
            pub fn add_header(&mut self, key: &str, value: &str) {
                self.headers.insert(key.to_string(), value.to_string());
            }
        }
        
        pub struct Response {
            pub status_code: u16,
            pub body: String,
        }
        
        impl Response {
            pub fn new(status_code: u16, body: &str) -> Self {
                Response {
                    status_code,
                    body: body.to_string(),
                }
            }
            
            pub fn ok(body: &str) -> Self {
                Self::new(200, body)
            }
            
            pub fn not_found() -> Self {
                Self::new(404, "Not Found")
            }
        }
    }
    
    pub mod router {
        use super::http::{Request, Response, Method};
        use std::collections::HashMap;
        
        type Handler = Box<dyn Fn(&Request) -> Response>;
        
        pub struct Router {
            routes: HashMap<String, Handler>,
        }
        
        impl Router {
            pub fn new() -> Self {
                Router {
                    routes: HashMap::new(),
                }
            }
            
            pub fn add_route<F>(&mut self, path: &str, handler: F)
            where
                F: Fn(&Request) -> Response + 'static,
            {
                self.routes.insert(path.to_string(), Box::new(handler));
            }
            
            pub fn handle_request(&self, request: &Request) -> Response {
                if let Some(handler) = self.routes.get(&request.path) {
                    handler(request)
                } else {
                    Response::not_found()
                }
            }
        }
    }
    
    pub mod middleware {
        use super::http::{Request, Response};
        
        pub trait Middleware {
            fn process(&self, request: &mut Request) -> Option<Response>;
        }
        
        pub struct Logger;
        
        impl Middleware for Logger {
            fn process(&self, request: &mut Request) -> Option<Response> {
                println!("请求: {} {}", 
                    match request.method {
                        super::http::Method::GET => "GET",
                        super::http::Method::POST => "POST",
                        super::http::Method::PUT => "PUT",
                        super::http::Method::DELETE => "DELETE",
                    },
                    request.path
                );
                None  // 继续处理
            }
        }
        
        pub struct Auth {
            required_token: String,
        }
        
        impl Auth {
            pub fn new(token: &str) -> Self {
                Auth {
                    required_token: token.to_string(),
                }
            }
        }
        
        impl Middleware for Auth {
            fn process(&self, request: &mut Request) -> Option<Response> {
                if let Some(auth_header) = request.headers.get("Authorization") {
                    if auth_header == &self.required_token {
                        None  // 认证通过，继续处理
                    } else {
                        Some(Response::new(401, "Unauthorized"))
                    }
                } else {
                    Some(Response::new(401, "Missing Authorization header"))
                }
            }
        }
    }
    
    pub mod server {
        use super::{http::*, router::Router, middleware::Middleware};
        
        pub struct Server {
            router: Router,
            middleware: Vec<Box<dyn Middleware>>,
        }
        
        impl Server {
            pub fn new() -> Self {
                Server {
                    router: Router::new(),
                    middleware: Vec::new(),
                }
            }
            
            pub fn add_route<F>(&mut self, path: &str, handler: F)
            where
                F: Fn(&Request) -> Response + 'static,
            {
                self.router.add_route(path, handler);
            }
            
            pub fn add_middleware<M>(&mut self, middleware: M)
            where
                M: Middleware + 'static,
            {
                self.middleware.push(Box::new(middleware));
            }
            
            pub fn handle_request(&self, mut request: Request) -> Response {
                // 执行中间件
                for middleware in &self.middleware {
                    if let Some(response) = middleware.process(&mut request) {
                        return response;
                    }
                }
                
                // 路由处理
                self.router.handle_request(&request)
            }
        }
    }
}

// 数据库模块
mod database {
    use std::collections::HashMap;
    
    pub trait Database {
        fn get(&self, key: &str) -> Option<String>;
        fn set(&mut self, key: &str, value: &str);
        fn delete(&mut self, key: &str) -> bool;
    }
    
    pub struct MemoryDatabase {
        data: HashMap<String, String>,
    }
    
    impl MemoryDatabase {
        pub fn new() -> Self {
            MemoryDatabase {
                data: HashMap::new(),
            }
        }
    }
    
    impl Database for MemoryDatabase {
        fn get(&self, key: &str) -> Option<String> {
            self.data.get(key).cloned()
        }
        
        fn set(&mut self, key: &str, value: &str) {
            self.data.insert(key.to_string(), value.to_string());
        }
        
        fn delete(&mut self, key: &str) -> bool {
            self.data.remove(key).is_some()
        }
    }
    
    // 数据库连接池
    pub mod pool {
        use super::Database;
        use std::sync::{Arc, Mutex};
        
        pub struct ConnectionPool<T: Database> {
            connections: Vec<Arc<Mutex<T>>>,
            current: usize,
        }
        
        impl<T: Database> ConnectionPool<T> {
            pub fn new(connections: Vec<T>) -> Self {
                ConnectionPool {
                    connections: connections
                        .into_iter()
                        .map(|conn| Arc::new(Mutex::new(conn)))
                        .collect(),
                    current: 0,
                }
            }
            
            pub fn get_connection(&mut self) -> Option<Arc<Mutex<T>>> {
                if self.connections.is_empty() {
                    return None;
                }
                
                let conn = self.connections[self.current].clone();
                self.current = (self.current + 1) % self.connections.len();
                Some(conn)
            }
        }
    }
}

fn project_structure_example() {
    println!("\n=== 实际项目结构示例 ===");
    
    // 创建Web服务器
    let mut server = web_server::server::Server::new();
    
    // 添加中间件
    server.add_middleware(web_server::middleware::Logger);
    server.add_middleware(web_server::middleware::Auth::new("secret-token"));
    
    // 添加路由
    server.add_route("/", |_req| {
        web_server::http::Response::ok("Welcome to Rust Web Server!")
    });
    
    server.add_route("/api/users", |_req| {
        web_server::http::Response::ok(r#"{"users": ["Alice", "Bob"]}"#)
    });
    
    // 模拟请求处理
    let mut request = web_server::http::Request::new(
        web_server::http::Method::GET,
        "/",
    );
    request.add_header("Authorization", "secret-token");
    
    let response = server.handle_request(request);
    println!("响应状态: {}, 内容: {}", response.status_code, response.body);
    
    // 未授权请求
    let unauthorized_request = web_server::http::Request::new(
        web_server::http::Method::GET,
        "/api/users",
    );
    
    let response = server.handle_request(unauthorized_request);
    println!("未授权响应: {}, 内容: {}", response.status_code, response.body);
    
    // 数据库使用示例
    let mut db = database::MemoryDatabase::new();
    db.set("user:1", "Alice");
    db.set("user:2", "Bob");
    
    if let Some(user) = db.get("user:1") {
        println!("数据库查询结果: {}", user);
    }
    
    // 连接池示例
    let databases = vec![
        database::MemoryDatabase::new(),
        database::MemoryDatabase::new(),
    ];
    
    let mut pool = database::pool::ConnectionPool::new(databases);
    
    if let Some(conn) = pool.get_connection() {
        let mut db = conn.lock().unwrap();
        db.set("test", "value");
        println!("连接池数据库操作完成");
    }
}

// 模块重导出示例
pub mod prelude {
    // 重导出常用类型和函数
    pub use crate::library::books::Book;
    pub use crate::web_server::http::{Request, Response, Method};
    pub use crate::database::{Database, MemoryDatabase};
}

// 条件编译模块
#[cfg(feature = "networking")]
mod networking {
    pub fn send_request() {
        println!("发送网络请求");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_visibility() {
        let book = library::books::Book::new("Test", "Author", "1234567890123");
        assert_eq!(book.title, "Test");
        assert!(book.is_valid());
    }
    
    #[test]
    fn test_web_server() {
        let mut server = web_server::server::Server::new();
        
        server.add_route("/test", |_req| {
            web_server::http::Response::ok("Test response")
        });
        
        let request = web_server::http::Request::new(
            web_server::http::Method::GET,
            "/test",
        );
        
        let response = server.handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, "Test response");
    }
    
    #[test]
    fn test_database() {
        let mut db = database::MemoryDatabase::new();
        
        db.set("key1", "value1");
        assert_eq!(db.get("key1"), Some("value1".to_string()));
        
        assert!(db.delete("key1"));
        assert_eq!(db.get("key1"), None);
    }
    
    #[test]
    fn test_prelude() {
        use crate::prelude::*;
        
        let book = Book::new("Test", "Author", "1234567890123");
        assert_eq!(book.title, "Test");
        
        let mut db = MemoryDatabase::new();
        db.set("test", "value");
        assert_eq!(db.get("test"), Some("value".to_string()));
    }
    
    #[test]
    fn test_examples() {
        module_basics();
        visibility_control();
        path_and_import();
        project_structure_example();
    }
}

// 模块系统要点总结：
// 1. mod关键字定义模块
// 2. pub关键字控制可见性
// 3. use关键字简化路径引用
// 4. super和crate关键字用于相对路径
// 5. pub(crate)、pub(super)等提供细粒度可见性控制
// 6. 模块可以嵌套，形成层次结构
// 7. 使用prelude模块重导出常用项目
// 8. 合理的模块组织提高代码可维护性