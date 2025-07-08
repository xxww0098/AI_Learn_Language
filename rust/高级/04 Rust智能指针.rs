// 04 Rust智能指针 - Box、Rc、RefCell和智能指针模式
// 本章介绍Rust的智能指针：Box<T>、Rc<T>、RefCell<T>和自定义智能指针

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

fn main() {
    // Box智能指针示例
    box_examples();
    
    // Rc智能指针示例
    rc_examples();
    
    // RefCell智能指针示例
    refcell_examples();
    
    // 自定义智能指针示例
    custom_smart_pointers();
}

// 案例1：Box<T> - 在堆上存储数据
fn box_examples() {
    println!("=== Box智能指针示例 ===");
    
    // 在堆上存储单个值
    let b = Box::new(5);
    println!("堆上的值: {}", b);
    
    // 递归类型：链表
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {:?}", list);
    
    // 大型数据结构
    let large_array = Box::new([0; 1000]);
    println!("大数组的第一个元素: {}", large_array[0]);
    
    // trait对象
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
    ];
    
    for shape in &shapes {
        println!("面积: {:.2}", shape.area());
    }
    
    // 移动语义
    let boxed_value = Box::new(String::from("Hello, Box!"));
    let moved_value = boxed_value;  // Box被移动
    // println!("{}", boxed_value);  // 编译错误：boxed_value已被移动
    println!("移动后的值: {}", moved_value);
}

// 递归数据结构：链表
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// trait对象示例
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 案例2：Rc<T> - 引用计数智能指针
fn rc_examples() {
    println!("\n=== Rc智能指针示例 ===");
    
    // 多个所有者的场景
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a创建后的引用计数: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("b创建后的引用计数: {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c创建后的引用计数: {}", Rc::strong_count(&a));
        println!("共享的链表节点: {:?}", c);
    }
    
    println!("c离开作用域后的引用计数: {}", Rc::strong_count(&a));
    
    // 图数据结构
    let shared_node = Rc::new(Node::new(42));
    println!("共享节点值: {}", shared_node.value);
    println!("共享节点引用计数: {}", Rc::strong_count(&shared_node));
    
    let node1 = Node::with_connection(1, Rc::clone(&shared_node));
    let node2 = Node::with_connection(2, Rc::clone(&shared_node));
    
    println!("连接后引用计数: {}", Rc::strong_count(&shared_node));
    println!("节点1连接到: {:?}", node1.connection.as_ref().map(|n| n.value));
    println!("节点2连接到: {:?}", node2.connection.as_ref().map(|n| n.value));
}

// 使用Rc的链表
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// 图节点结构
#[derive(Debug)]
struct Node {
    value: i32,
    connection: Option<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            connection: None,
        }
    }
    
    fn with_connection(value: i32, connection: Rc<Node>) -> Self {
        Node {
            value,
            connection: Some(connection),
        }
    }
}

// 案例3：RefCell<T> - 内部可变性
fn refcell_examples() {
    println!("\n=== RefCell智能指针示例 ===");
    
    // 基本RefCell使用
    let value = RefCell::new(5);
    
    println!("初始值: {}", *value.borrow());
    
    // 运行时借用检查
    {
        let mut mutable_borrow = value.borrow_mut();
        *mutable_borrow = 10;
        println!("修改后的值: {}", *mutable_borrow);
    }  // 可变借用在此处结束
    
    println!("最终值: {}", *value.borrow());
    
    // Mock对象示例
    let mock = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock, 100);
    
    limit_tracker.set_value(80);
    
    let sent_messages = mock.sent_messages.borrow();
    println!("发送的消息数量: {}", sent_messages.len());
    for (i, message) in sent_messages.iter().enumerate() {
        println!("消息 {}: {}", i + 1, message);
    }
    
    // Rc<RefCell<T>>组合使用
    rc_refcell_combination();
}

// 消息发送trait
trait Messenger {
    fn send(&self, msg: &str);
}

// Mock实现
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

// 限制跟踪器
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send("错误：已超过配额！");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("紧急警告：已使用超过90%的配额！");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("警告：已使用超过75%的配额");
        }
    }
}

// Rc<RefCell<T>>组合
fn rc_refcell_combination() {
    println!("\n--- Rc<RefCell<T>>组合示例 ---");
    
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;
    
    println!("最终值: {}", *value.borrow());
    println!("引用计数: {}", Rc::strong_count(&value));
}

// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("释放MyBox!");
    }
}

// 案例4：自定义智能指针
fn custom_smart_pointers() {
    println!("\n=== 自定义智能指针示例 ===");
    
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // 解引用MyBox
    
    println!("MyBox中的值: {}", *y);
    
    // 强制解引用转换
    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // &MyBox<String> -> &String -> &str
    
    // Drop trait示例
    {
        let _c = MyBox::new(String::from("临时值"));
        println!("MyBox创建完成");
    }  // _c在此处被drop
    
    println!("作用域结束后");
    
    // 提前drop
    let to_drop = MyBox::new(String::from("提前释放"));
    println!("准备提前释放");
    drop(to_drop);  // 手动调用drop
    println!("已提前释放");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// 弱引用示例
use std::rc::Weak;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Weak<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }
    
    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
}

fn weak_reference_example() {
    println!("\n=== 弱引用示例 ===");
    
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    
    println!("根节点强引用计数: {}", Rc::strong_count(&root));
    println!("根节点弱引用计数: {}", Rc::weak_count(&root));
    
    println!("子节点1强引用计数: {}", Rc::strong_count(&child1));
    println!("子节点1弱引用计数: {}", Rc::weak_count(&child1));
    
    // 通过弱引用访问父节点
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("子节点1的父节点值: {}", parent.value);
    }
    
    println!("根节点的子节点数量: {}", root.children.borrow().len());
}

// 循环引用问题演示
fn cycle_reference_problem() {
    println!("\n=== 循环引用问题演示 ===");
    
    // 这会创建循环引用（注释掉避免内存泄漏）
    /*
    let a = Rc::new(RefCell::new(5));
    let b = Rc::new(RefCell::new(10));
    
    // 创建循环引用
    *a.borrow_mut() = Rc::clone(&b);  // 类型不匹配，仅为演示
    *b.borrow_mut() = Rc::clone(&a);  // 类型不匹配，仅为演示
    
    // a和b形成循环引用，永远不会被释放
    */
    
    println!("循环引用问题需要使用Weak<T>来解决");
}

// 智能指针性能测试
fn performance_comparison() {
    println!("\n=== 智能指针性能比较 ===");
    
    use std::time::Instant;
    
    let iterations = 1_000_000;
    
    // Box性能测试
    let start = Instant::now();
    for i in 0..iterations {
        let _boxed = Box::new(i);
    }
    let box_duration = start.elapsed();
    println!("Box创建 {} 次耗时: {:?}", iterations, box_duration);
    
    // Rc性能测试
    let start = Instant::now();
    for i in 0..iterations {
        let _rc = Rc::new(i);
    }
    let rc_duration = start.elapsed();
    println!("Rc创建 {} 次耗时: {:?}", iterations, rc_duration);
    
    // RefCell性能测试
    let start = Instant::now();
    let refcell = RefCell::new(0);
    for i in 0..iterations {
        *refcell.borrow_mut() = i;
    }
    let refcell_duration = start.elapsed();
    println!("RefCell修改 {} 次耗时: {:?}", iterations, refcell_duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let boxed = Box::new(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_rc() {
        let rc = Rc::new(42);
        let rc2 = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(*rc, 42);
        assert_eq!(*rc2, 42);
    }

    #[test]
    fn test_refcell() {
        let refcell = RefCell::new(42);
        
        {
            let value = refcell.borrow();
            assert_eq!(*value, 42);
        }
        
        {
            let mut value = refcell.borrow_mut();
            *value = 84;
        }
        
        assert_eq!(*refcell.borrow(), 84);
    }

    #[test]
    fn test_mybox() {
        let mybox = MyBox::new(42);
        assert_eq!(*mybox, 42);
    }

    #[test]
    fn test_mock_messenger() {
        let mock = MockMessenger::new();
        mock.send("test message");
        
        let messages = mock.sent_messages.borrow();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test message");
    }

    #[test]
    fn test_tree_node() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);
        
        TreeNode::add_child(&root, child.clone());
        
        assert_eq!(root.children.borrow().len(), 1);
        assert_eq!(Rc::strong_count(&root), 1);
        assert_eq!(Rc::weak_count(&root), 1);
    }

    #[test]
    fn test_examples() {
        box_examples();
        rc_examples();
        refcell_examples();
        custom_smart_pointers();
        weak_reference_example();
        cycle_reference_problem();
        performance_comparison();
    }
}

// 智能指针要点总结：
// 1. Box<T>在堆上存储数据，拥有数据的所有权
// 2. Rc<T>允许多个所有者，使用引用计数
// 3. RefCell<T>提供内部可变性，运行时借用检查
// 4. Weak<T>解决循环引用问题
// 5. Deref trait允许智能指针像引用一样使用
// 6. Drop trait定义智能指针离开作用域时的清理逻辑
// 7. 智能指针是零成本抽象的体现
// 8. 合理选择智能指针类型是内存安全的关键