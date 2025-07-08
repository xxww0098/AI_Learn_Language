// 09_内存管理.rs
// Rust标准库内存管理详解

/*
Rust的内存管理是其最重要的特性之一，通过所有权系统实现内存安全：

核心概念：
- 所有权(Ownership)：每个值都有一个所有者
- 借用(Borrowing)：通过引用访问数据而不获取所有权
- 生命周期(Lifetimes)：确保引用的有效性

智能指针：
- Box<T>：堆上分配单个值
- Rc<T>：引用计数智能指针，允许多个所有者
- Arc<T>：原子引用计数，线程安全版本的Rc
- RefCell<T>：内部可变性，运行时借用检查
- Mutex<T>：互斥锁，提供线程安全的内部可变性

内存分配：
- Vec<T>：动态数组，在堆上分配连续内存
- HashMap<K,V>：哈希表，动态内存管理
- String：动态字符串，在堆上分配

RAII原则：
- 资源获取即初始化
- 自动内存释放
- 确定性析构

内存布局：
- 栈内存：快速但容量有限
- 堆内存：灵活但需要管理
- 静态内存：程序运行期间一直存在
*/

use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use std::mem;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    println!("=== Rust标准库内存管理 ===");
    
    // 1. 所有权基础
    println!("\n1. 所有权基础：");
    ownership_basics();
    
    // 2. 借用和引用
    println!("\n2. 借用和引用：");
    borrowing_and_references();
    
    // 3. 智能指针
    println!("\n3. 智能指针：");
    smart_pointers();
    
    // 4. 内部可变性
    println!("\n4. 内部可变性：");
    interior_mutability();
    
    // 5. 引用计数
    println!("\n5. 引用计数：");
    reference_counting();
    
    // 6. 内存布局分析
    println!("\n6. 内存布局分析：");
    memory_layout_analysis();
    
    // 7. 自定义内存分配
    println!("\n7. 自定义内存分配：");
    custom_allocation();
    
    // 8. 内存泄漏预防
    println!("\n8. 内存泄漏预防：");
    memory_leak_prevention();
    
    // 9. 性能优化
    println!("\n9. 性能优化：");
    performance_optimization();
    
    // 10. 最佳实践
    println!("\n10. 最佳实践：");
    best_practices();
    
    println!("\n=== 内存管理学习完成 ===");
}

// 所有权基础
fn ownership_basics() {
    // 所有权转移
    let s1 = String::from("Hello");
    let s2 = s1; // s1的所有权转移给s2
    // println!("{}", s1); // 编译错误：s1已被移动
    println!("s2: {}", s2);
    
    // Clone 深拷贝
    let s3 = String::from("World");
    let s4 = s3.clone(); // 深拷贝
    println!("s3: {}, s4: {}", s3, s4); // 两者都可以使用
    
    // Copy trait
    let x = 42;
    let y = x; // Copy语义，x仍然有效
    println!("x: {}, y: {}", x, y);
    
    // 函数调用中的所有权
    let s = String::from("hello");
    takes_ownership(s); // s的所有权被移动
    // println!("{}", s); // 编译错误
    
    let x = 5;
    makes_copy(x); // x被复制
    println!("x is still: {}", x); // x仍然有效
    
    // 返回值和所有权
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);
    
    // 移动语义的好处
    demonstrate_move_semantics();
}

// 借用和引用
fn borrowing_and_references() {
    // 不可变引用
    let s = String::from("hello");
    let len = calculate_length(&s); // 借用s
    println!("'{}' 的长度是 {}", s, len); // s仍然可用
    
    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("改变后: {}", s);
    
    // 借用规则演示
    borrowing_rules_demo();
    
    // 悬挂引用预防
    // let reference_to_nothing = dangle(); // 编译错误
    let valid_reference = no_dangle();
    println!("有效引用: {}", valid_reference);
    
    // 切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("切片: '{}' 和 '{}'", hello, world);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    println!("数组切片: {:?}", slice);
}

// 智能指针
fn smart_pointers() {
    // Box<T> - 堆分配
    let b = Box::new(5);
    println!("Box中的值: {}", b);
    
    // 递归类型
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("递归列表: {:?}", list);
    
    // 大型数据的堆分配
    let large_array = Box::new([0; 1_000_000]);
    println!("大数组在堆上分配，第一个元素: {}", large_array[0]);
    
    // Deref trait演示
    deref_trait_demo();
    
    // Drop trait演示
    drop_trait_demo();
    
    // 智能指针的优势
    smart_pointer_advantages();
}

// 内部可变性
fn interior_mutability() {
    // RefCell<T> - 运行时借用检查
    let data = RefCell::new(5);
    
    // 可变借用
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 1;
    } // 可变借用在这里结束
    
    // 不可变借用
    let borrowed = data.borrow();
    println!("RefCell中的值: {}", *borrowed);
    
    // Cell<T> - Copy类型的内部可变性
    let c = Cell::new(5);
    c.set(10);
    println!("Cell中的值: {}", c.get());
    
    // RefCell运行时借用检查
    runtime_borrow_checking();
    
    // 内部可变性的模式
    interior_mutability_patterns();
}

// 引用计数
fn reference_counting() {
    // Rc<T> - 单线程引用计数
    let rc_example = Rc::new(String::from("hello"));
    println!("初始引用计数: {}", Rc::strong_count(&rc_example));
    
    {
        let _rc_clone1 = Rc::clone(&rc_example);
        let _rc_clone2 = Rc::clone(&rc_example);
        println!("克隆后引用计数: {}", Rc::strong_count(&rc_example));
    }
    
    println!("离开作用域后引用计数: {}", Rc::strong_count(&rc_example));
    
    // Weak<T> - 弱引用，避免循环引用
    weak_reference_demo();
    
    // Arc<T> - 多线程引用计数
    arc_demo();
    
    // 循环引用问题
    circular_reference_problem();
}

// 内存布局分析
fn memory_layout_analysis() {
    // 基本类型的大小
    println!("基本类型大小:");
    println!("  bool: {} 字节", mem::size_of::<bool>());
    println!("  i32: {} 字节", mem::size_of::<i32>());
    println!("  f64: {} 字节", mem::size_of::<f64>());
    println!("  char: {} 字节", mem::size_of::<char>());
    println!("  &str: {} 字节", mem::size_of::<&str>());
    println!("  String: {} 字节", mem::size_of::<String>());
    
    // 复合类型的大小
    println!("\n复合类型大小:");
    println!("  Option<i32>: {} 字节", mem::size_of::<Option<i32>>());
    println!("  Result<i32, String>: {} 字节", mem::size_of::<Result<i32, String>>());
    println!("  Vec<i32>: {} 字节", mem::size_of::<Vec<i32>>());
    println!("  HashMap<String, i32>: {} 字节", mem::size_of::<HashMap<String, i32>>());
    
    // 智能指针的大小
    println!("\n智能指针大小:");
    println!("  Box<i32>: {} 字节", mem::size_of::<Box<i32>>());
    println!("  Rc<i32>: {} 字节", mem::size_of::<Rc<i32>>());
    println!("  Arc<i32>: {} 字节", mem::size_of::<Arc<i32>>());
    println!("  RefCell<i32>: {} 字节", mem::size_of::<RefCell<i32>>());
    
    // 内存对齐
    memory_alignment_demo();
    
    // 零大小类型
    zero_sized_types();
}

// 自定义内存分配
fn custom_allocation() {
    // 使用全局分配器
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            *ptr = 42;
            println!("手动分配的内存中的值: {}", *ptr);
            
            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        }
    }
    
    // Vec的内存管理
    vec_memory_management();
    
    // 自定义分配器概念
    custom_allocator_concept();
}

// 内存泄漏预防
fn memory_leak_prevention() {
    // RAII原则
    raii_principle();
    
    // 避免循环引用
    avoid_circular_references();
    
    // 及时释放资源
    timely_resource_release();
    
    // 内存泄漏检测工具
    memory_leak_detection_tools();
}

// 性能优化
fn performance_optimization() {
    // 预分配容量
    capacity_pre_allocation();
    
    // 避免不必要的克隆
    avoid_unnecessary_cloning();
    
    // 使用引用而不是所有权
    use_references_over_ownership();
    
    // 内存池模式
    memory_pool_pattern();
    
    // 缓存友好的数据结构
    cache_friendly_structures();
}

// 最佳实践
fn best_practices() {
    println!("内存管理最佳实践:");
    println!("1. 优先使用栈分配，只有在必要时才使用堆分配");
    println!("2. 使用Vec::with_capacity预分配空间");
    println!("3. 避免不必要的String::clone()，使用&str");
    println!("4. 使用Rc<T>和Arc<T>共享数据，避免深拷贝");
    println!("5. 使用Weak<T>打破循环引用");
    println!("6. 遵循RAII原则，让编译器管理内存");
    println!("7. 使用工具检测内存泄漏");
    println!("8. 理解移动语义，避免意外的所有权转移");
    println!("9. 合理使用内部可变性模式");
    println!("10. 注意内存对齐对性能的影响");
    
    // 常见陷阱
    common_pitfalls();
    
    // 内存调试技巧
    memory_debugging_tips();
}

// 辅助函数和类型定义

// 递归链表类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 所有权转移函数
fn takes_ownership(some_string: String) {
    println!("接收所有权: {}", some_string);
} // some_string在这里被销毁

fn makes_copy(some_integer: i32) {
    println!("复制的值: {}", some_integer);
} // some_integer在这里离开作用域，但没有特殊处理

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回并转移所有权
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回并转移所有权
}

// 借用函数
fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域，但没有删除引用的数据

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 悬挂引用示例（注释掉因为会编译错误）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回对s的引用，但s即将被销毁
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 直接返回String，转移所有权
}

// 移动语义演示
fn demonstrate_move_semantics() {
    println!("移动语义演示:");
    
    // 大型数据结构的移动
    let large_vec = vec![0; 1_000_000];
    println!("原始vec长度: {}", large_vec.len());
    
    let moved_vec = large_vec; // 移动，不是复制
    println!("移动后vec长度: {}", moved_vec.len());
    // large_vec已不可用
    
    // 函数参数中的移动
    process_large_data(moved_vec);
}

fn process_large_data(data: Vec<i32>) {
    println!("处理数据，长度: {}", data.len());
    // data在函数结束时被销毁
}

// 借用规则演示
fn borrowing_rules_demo() {
    println!("借用规则演示:");
    
    let mut s = String::from("hello");
    
    // 规则1: 可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    
    // 规则2: 不可变引用和可变引用不能同时存在
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3);
    
    // 引用的作用域
    {
        let r4 = &s;
        println!("临时引用: {}", r4);
    } // r4在这里离开作用域
    
    let r5 = &mut s;
    r5.push('!');
    println!("最终结果: {}", r5);
}

// Deref trait演示
fn deref_trait_demo() {
    println!("Deref trait演示:");
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // 自动解引用
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 强制解引用
    hello(&(*m)[..]); // 手动解引用
}

// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Drop trait演示
fn drop_trait_demo() {
    println!("Drop trait演示:");
    
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    
    println!("CustomSmartPointers created.");
    
    // 可以手动调用drop
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 智能指针优势
fn smart_pointer_advantages() {
    println!("智能指针优势:");
    println!("1. 自动内存管理");
    println!("2. 防止内存泄漏");
    println!("3. 防止悬挂指针");
    println!("4. 线程安全（Arc<T>）");
    println!("5. 引用计数（Rc<T>）");
}

// 运行时借用检查
fn runtime_borrow_checking() {
    println!("运行时借用检查:");
    
    let data = RefCell::new(vec![1, 2, 3]);
    
    // 同时借用两个不可变引用
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("数据长度: {}, {}", borrow1.len(), borrow2.len());
    
    drop(borrow1);
    drop(borrow2);
    
    // 可变借用
    data.borrow_mut().push(4);
    println!("添加元素后: {:?}", data.borrow());
    
    // 注意：同时进行可变和不可变借用会panic
    // let borrow3 = data.borrow();
    // let mut borrow4 = data.borrow_mut(); // 这会panic
}

// 内部可变性模式
fn interior_mutability_patterns() {
    println!("内部可变性模式:");
    
    // 模式1: 配置对象
    #[derive(Debug)]
    struct Config {
        cache: RefCell<HashMap<String, String>>,
    }
    
    impl Config {
        fn new() -> Self {
            Config {
                cache: RefCell::new(HashMap::new()),
            }
        }
        
        fn get_value(&self, key: &str) -> Option<String> {
            self.cache.borrow().get(key).cloned()
        }
        
        fn set_value(&self, key: String, value: String) {
            self.cache.borrow_mut().insert(key, value);
        }
    }
    
    let config = Config::new();
    config.set_value("debug".to_string(), "true".to_string());
    
    if let Some(value) = config.get_value("debug") {
        println!("  配置值: debug = {}", value);
    }
    
    // 模式2: 计数器
    struct Counter {
        value: Cell<usize>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter { value: Cell::new(0) }
        }
        
        fn increment(&self) {
            let current = self.value.get();
            self.value.set(current + 1);
        }
        
        fn get(&self) -> usize {
            self.value.get()
        }
    }
    
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("  计数器值: {}", counter.get());
}

// 弱引用演示
fn weak_reference_demo() {
    println!("弱引用演示:");
    
    let strong_rc = Rc::new(String::from("hello"));
    let weak_ref = Rc::downgrade(&strong_rc);
    
    println!("强引用计数: {}", Rc::strong_count(&strong_rc));
    println!("弱引用计数: {}", Rc::weak_count(&strong_rc));
    
    // 通过弱引用访问数据
    if let Some(strong_ref) = weak_ref.upgrade() {
        println!("通过弱引用访问: {}", strong_ref);
    }
    
    drop(strong_rc);
    
    // 强引用被销毁后，弱引用无法升级
    if weak_ref.upgrade().is_none() {
        println!("强引用已销毁，弱引用无法升级");
    }
}

// Arc演示
fn arc_demo() {
    use std::thread;
    
    println!("Arc多线程演示:");
    
    let data = Arc::new(String::from("shared data"));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程{}: {}", i, data);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("主线程: {}", data);
}

// 循环引用问题
fn circular_reference_problem() {
    println!("循环引用问题演示:");
    
    // 定义树节点
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                children: RefCell::new(vec![]),
                parent: RefCell::new(Weak::new()),
            })
        }
        
        fn add_child(self: &Rc<Self>, child: Rc<Node>) {
            child.parent.borrow_mut().replace(Rc::downgrade(self));
            self.children.borrow_mut().push(child);
        }
    }
    
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    root.add_child(child1.clone());
    root.add_child(child2.clone());
    
    println!("根节点引用计数: {}", Rc::strong_count(&root));
    println!("子节点1引用计数: {}", Rc::strong_count(&child1));
    
    // 通过弱引用访问父节点
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("子节点1的父节点值: {}", parent.value);
    }
}

// 内存对齐演示
fn memory_alignment_demo() {
    println!("内存对齐演示:");
    
    #[repr(C)]
    struct AlignedStruct {
        a: u8,  // 1字节
        b: u32, // 4字节
        c: u8,  // 1字节
    }
    
    #[repr(C, packed)]
    struct PackedStruct {
        a: u8,  // 1字节
        b: u32, // 4字节
        c: u8,  // 1字节
    }
    
    println!("  对齐结构体大小: {} 字节", mem::size_of::<AlignedStruct>());
    println!("  紧密结构体大小: {} 字节", mem::size_of::<PackedStruct>());
    
    println!("  u8对齐: {} 字节", mem::align_of::<u8>());
    println!("  u32对齐: {} 字节", mem::align_of::<u32>());
    println!("  u64对齐: {} 字节", mem::align_of::<u64>());
}

// 零大小类型
fn zero_sized_types() {
    println!("零大小类型:");
    
    struct ZeroSized;
    
    println!("  单元类型 () 大小: {} 字节", mem::size_of::<()>());
    println!("  空结构体大小: {} 字节", mem::size_of::<ZeroSized>());
    println!("  空数组大小: {} 字节", mem::size_of::<[i32; 0]>());
    
    // 零大小类型的优化
    let zst_vec: Vec<()> = vec![(); 1_000_000];
    println!("  百万个单元类型的Vec: {} 字节", mem::size_of_val(&zst_vec));
}

// Vec内存管理
fn vec_memory_management() {
    println!("Vec内存管理:");
    
    let mut v = Vec::new();
    println!("  初始容量: {}", v.capacity());
    
    v.push(1);
    println!("  添加1个元素后容量: {}", v.capacity());
    
    for i in 2..=10 {
        v.push(i);
        println!("  添加{}个元素后容量: {}", i, v.capacity());
    }
    
    // 预分配容量
    let mut v2 = Vec::with_capacity(100);
    println!("  预分配容量: {}", v2.capacity());
    
    for i in 1..=50 {
        v2.push(i);
    }
    println!("  添加50个元素后容量: {}", v2.capacity());
    
    // 收缩到合适大小
    v2.shrink_to_fit();
    println!("  收缩后容量: {}", v2.capacity());
}

// 自定义分配器概念
fn custom_allocator_concept() {
    println!("自定义分配器概念:");
    println!("1. 全局分配器替换");
    println!("2. 内存池分配器");
    println!("3. 栈分配器");
    println!("4. 区域分配器");
    println!("5. 垃圾收集分配器");
    
    // 简单的内存使用统计
    memory_usage_tracking();
}

// 内存使用跟踪
fn memory_usage_tracking() {
    // 这是概念性演示，实际实现需要更复杂的机制
    struct MemoryTracker {
        allocated: std::cell::Cell<usize>,
    }
    
    impl MemoryTracker {
        fn new() -> Self {
            MemoryTracker {
                allocated: std::cell::Cell::new(0),
            }
        }
        
        fn track_allocation(&self, size: usize) {
            let current = self.allocated.get();
            self.allocated.set(current + size);
        }
        
        fn track_deallocation(&self, size: usize) {
            let current = self.allocated.get();
            self.allocated.set(current.saturating_sub(size));
        }
        
        fn current_usage(&self) -> usize {
            self.allocated.get()
        }
    }
    
    let tracker = MemoryTracker::new();
    
    // 模拟分配
    tracker.track_allocation(1024);
    println!("  分配1KB后: {} 字节", tracker.current_usage());
    
    tracker.track_allocation(2048);
    println!("  再分配2KB后: {} 字节", tracker.current_usage());
    
    tracker.track_deallocation(1024);
    println!("  释放1KB后: {} 字节", tracker.current_usage());
}

// RAII原则
fn raii_principle() {
    println!("RAII原则演示:");
    
    struct Resource {
        name: String,
    }
    
    impl Resource {
        fn new(name: &str) -> Self {
            println!("  获取资源: {}", name);
            Resource { name: name.to_string() }
        }
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  释放资源: {}", self.name);
        }
    }
    
    {
        let _r1 = Resource::new("文件句柄");
        let _r2 = Resource::new("网络连接");
        println!("  资源正在使用中");
    } // 资源在此处自动释放
    
    println!("  所有资源已释放");
}

// 避免循环引用
fn avoid_circular_references() {
    println!("避免循环引用的策略:");
    println!("1. 使用Weak<T>引用");
    println!("2. 重新设计数据结构");
    println!("3. 使用索引而不是引用");
    println!("4. 手动打破循环");
    
    // 使用索引的树结构
    struct IndexedTree {
        nodes: Vec<IndexedNode>,
    }
    
    struct IndexedNode {
        value: i32,
        children: Vec<usize>,
        parent: Option<usize>,
    }
    
    let mut tree = IndexedTree { nodes: vec![] };
    
    // 添加根节点
    tree.nodes.push(IndexedNode {
        value: 1,
        children: vec![],
        parent: None,
    });
    
    // 添加子节点
    tree.nodes.push(IndexedNode {
        value: 2,
        children: vec![],
        parent: Some(0),
    });
    
    // 更新父节点的子节点列表
    tree.nodes[0].children.push(1);
    
    println!("  使用索引的树结构避免了循环引用");
}

// 及时释放资源
fn timely_resource_release() {
    println!("及时释放资源:");
    
    // 使用作用域控制生命周期
    {
        let large_vec = vec![0; 1_000_000];
        println!("  大向量创建，长度: {}", large_vec.len());
    } // large_vec在此处被释放
    
    // 手动释放
    let mut optional_data = Some(vec![0; 1_000_000]);
    if let Some(data) = optional_data.take() {
        println!("  处理数据，长度: {}", data.len());
        // data在此处被销毁
    }
    
    // 使用mem::drop显式释放
    let temp_data = vec![0; 100];
    mem::drop(temp_data);
    println!("  临时数据已显式释放");
}

// 内存泄漏检测工具
fn memory_leak_detection_tools() {
    println!("内存泄漏检测工具:");
    println!("1. Valgrind (Linux)");
    println!("2. AddressSanitizer");
    println!("3. Miri (Rust专用)");
    println!("4. cargo-leak");
    println!("5. Heaptrack");
    println!("6. 自定义分配器跟踪");
}

// 预分配容量
fn capacity_pre_allocation() {
    println!("预分配容量优化:");
    
    // 低效方式
    let start = std::time::Instant::now();
    let mut v1 = Vec::new();
    for i in 0..10000 {
        v1.push(i);
    }
    let time1 = start.elapsed();
    
    // 高效方式
    let start = std::time::Instant::now();
    let mut v2 = Vec::with_capacity(10000);
    for i in 0..10000 {
        v2.push(i);
    }
    let time2 = start.elapsed();
    
    println!("  无预分配: {:?}", time1);
    println!("  预分配: {:?}", time2);
    
    if time2 < time1 {
        println!("  预分配更快!");
    }
}

// 避免不必要的克隆
fn avoid_unnecessary_cloning() {
    println!("避免不必要的克隆:");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // 低效：克隆整个向量
    fn process_data_clone(data: Vec<i32>) -> i32 {
        data.iter().sum()
    }
    
    // 高效：使用引用
    fn process_data_ref(data: &[i32]) -> i32 {
        data.iter().sum()
    }
    
    let sum1 = process_data_clone(data.clone());
    let sum2 = process_data_ref(&data);
    
    println!("  克隆方式结果: {}", sum1);
    println!("  引用方式结果: {}", sum2);
    println!("  原始数据仍可用: {:?}", data);
}

// 使用引用而不是所有权
fn use_references_over_ownership() {
    println!("使用引用优化:");
    
    struct DataProcessor;
    
    impl DataProcessor {
        // 低效：获取所有权
        fn process_owned(&self, data: String) -> usize {
            data.len()
        }
        
        // 高效：使用引用
        fn process_ref(&self, data: &str) -> usize {
            data.len()
        }
    }
    
    let processor = DataProcessor;
    let text = String::from("Hello, World!");
    
    // 使用引用，保持所有权
    let len = processor.process_ref(&text);
    println!("  文本长度: {}", len);
    println!("  原始文本仍可用: {}", text);
}

// 内存池模式
fn memory_pool_pattern() {
    println!("内存池模式概念:");
    
    // 简化的内存池
    struct SimplePool<T> {
        items: Vec<Option<T>>,
        free_list: Vec<usize>,
    }
    
    impl<T> SimplePool<T> {
        fn new(capacity: usize) -> Self {
            SimplePool {
                items: vec![None; capacity],
                free_list: (0..capacity).collect(),
            }
        }
        
        fn allocate(&mut self, item: T) -> Option<usize> {
            if let Some(index) = self.free_list.pop() {
                self.items[index] = Some(item);
                Some(index)
            } else {
                None
            }
        }
        
        fn deallocate(&mut self, index: usize) {
            if index < self.items.len() && self.items[index].is_some() {
                self.items[index] = None;
                self.free_list.push(index);
            }
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)?.as_ref()
        }
    }
    
    let mut pool = SimplePool::new(10);
    
    let id1 = pool.allocate("Hello".to_string()).unwrap();
    let id2 = pool.allocate("World".to_string()).unwrap();
    
    println!("  分配ID {}: {:?}", id1, pool.get(id1));
    println!("  分配ID {}: {:?}", id2, pool.get(id2));
    
    pool.deallocate(id1);
    println!("  释放ID {}", id1);
    
    let id3 = pool.allocate("Rust".to_string()).unwrap();
    println!("  重新分配ID {}: {:?}", id3, pool.get(id3));
}

// 缓存友好的数据结构
fn cache_friendly_structures() {
    println!("缓存友好的数据结构:");
    
    // 结构体数组 vs 数组结构体
    #[derive(Clone, Copy)]
    struct Point {
        x: f32,
        y: f32,
        z: f32,
    }
    
    // AoS (Array of Structures)
    let aos: Vec<Point> = vec![Point { x: 1.0, y: 2.0, z: 3.0 }; 1000];
    
    // SoA (Structure of Arrays)
    struct Points {
        x: Vec<f32>,
        y: Vec<f32>,
        z: Vec<f32>,
    }
    
    let soa = Points {
        x: vec![1.0; 1000],
        y: vec![2.0; 1000],
        z: vec![3.0; 1000],
    };
    
    println!("  AoS大小: {} 字节", mem::size_of_val(&aos));
    println!("  SoA X向量大小: {} 字节", mem::size_of_val(&soa.x));
    
    // 访问模式影响性能
    // 如果只需要x坐标，SoA更cache-friendly
    let sum_x_aos: f32 = aos.iter().map(|p| p.x).sum();
    let sum_x_soa: f32 = soa.x.iter().sum();
    
    println!("  AoS X坐标和: {}", sum_x_aos);
    println!("  SoA X坐标和: {}", sum_x_soa);
}

// 常见陷阱
fn common_pitfalls() {
    println!("常见内存管理陷阱:");
    println!("1. 循环引用导致内存泄漏");
    println!("2. 过度使用clone()影响性能");
    println!("3. 不必要的Box<T>装箱");
    println!("4. 在循环中重复分配内存");
    println!("5. 忘记释放外部资源（文件句柄等）");
    println!("6. 持有大量不必要的引用");
    println!("7. 错误的生命周期设计");
    println!("8. 内存对齐问题");
}

// 内存调试技巧
fn memory_debugging_tips() {
    println!("内存调试技巧:");
    println!("1. 使用mem::size_of分析内存布局");
    println!("2. 实现自定义Drop trait跟踪析构");
    println!("3. 使用引用计数调试共享所有权");
    println!("4. 利用编译器警告和Clippy");
    println!("5. 使用工具分析内存使用");
    println!("6. 编写单元测试验证内存行为");
    println!("7. 使用基准测试评估性能");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ownership_transfer() {
        let s1 = String::from("hello");
        let s2 = s1;
        assert_eq!(s2, "hello");
        // s1不再可用
    }
    
    #[test]
    fn test_borrowing() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s仍然可用
    }
    
    #[test]
    fn test_box() {
        let b = Box::new(5);
        assert_eq!(*b, 5);
    }
    
    #[test]
    fn test_rc() {
        let rc = Rc::new(String::from("hello"));
        let rc2 = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(*rc, *rc2);
    }
    
    #[test]
    fn test_refcell() {
        let data = RefCell::new(5);
        *data.borrow_mut() = 10;
        assert_eq!(*data.borrow(), 10);
    }
    
    #[test]
    fn test_memory_size() {
        assert_eq!(mem::size_of::<i32>(), 4);
        assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<usize>());
    }
    
    #[test]
    fn test_vec_capacity() {
        let mut v = Vec::with_capacity(10);
        assert_eq!(v.capacity(), 10);
        assert_eq!(v.len(), 0);
        
        v.push(1);
        assert_eq!(v.capacity(), 10);
        assert_eq!(v.len(), 1);
    }
    
    #[test]
    fn test_weak_reference() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        
        assert!(weak.upgrade().is_some());
        drop(strong);
        assert!(weak.upgrade().is_none());
    }
}