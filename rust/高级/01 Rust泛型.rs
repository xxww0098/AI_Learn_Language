// 01 Rust泛型 - 泛型编程和类型参数
// 本章介绍Rust的泛型系统：函数泛型、结构体泛型和枚举泛型

use std::fmt::Display;

fn main() {
    // 泛型函数示例
    generic_functions();
    
    // 泛型结构体示例
    generic_structs();
    
    // 泛型枚举示例
    generic_enums();
    
    // 泛型方法示例
    generic_methods();
}

// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

// 案例1：泛型函数和约束
fn generic_functions() {
    println!("=== 泛型函数示例 ===");
    
    // 使用泛型函数处理不同类型
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符: {}", result);
    
    // 交换变量
    let mut x = 5;
    let mut y = 10;
    println!("交换前: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("交换后: x = {}, y = {}", x, y);
    
    let mut a = String::from("hello");
    let mut b = String::from("world");
    println!("交换前: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("交换后: a = {}, b = {}", a, b);
    
    // 使用多个泛型参数
    let pair = create_pair(42, "hello");
    println!("配对: {:?}", pair);
    
    let result = compare_and_display(10, 20);
    println!("比较结果: {}", result);
}

// 多个泛型参数
fn create_pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// 带约束的泛型函数
fn compare_and_display<T>(a: T, b: T) -> String 
where
    T: PartialOrd + Display + Copy,
{
    if a > b {
        format!("{} 大于 {}", a, b)
    } else if a < b {
        format!("{} 小于 {}", a, b)
    } else {
        format!("{} 等于 {}", a, b)
    }
}

// 案例2：泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数的结构体
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn first(&self) -> &T {
        &self.first
    }
    
    fn second(&self) -> &U {
        &self.second
    }
    
    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

fn generic_structs() {
    println!("\n=== 泛型结构体示例 ===");
    
    // 整数点
    let integer_point = Point::new(5, 10);
    println!("整数点: {:?}", integer_point);
    println!("x坐标: {}", integer_point.x());
    
    // 浮点数点
    let float_point = Point::new(1.0, 4.0);
    println!("浮点数点: {:?}", float_point);
    println!("距离原点: {:.2}", float_point.distance_from_origin());
    
    // 字符串点
    let string_point = Point::new("hello".to_string(), "world".to_string());
    println!("字符串点: {:?}", string_point);
    
    // 混合类型配对
    let mixed_pair = Pair::new(42, "answer".to_string());
    println!("混合配对: {:?}", mixed_pair);
    println!("第一个元素: {}", mixed_pair.first());
    println!("第二个元素: {}", mixed_pair.second());
    
    let (num, text) = mixed_pair.into_tuple();
    println!("解构后: 数字 = {}, 文本 = {}", num, text);
}

// 泛型枚举
#[derive(Debug)]
enum Container<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

impl<T> Container<T> {
    fn is_empty(&self) -> bool {
        matches!(self, Container::Empty)
    }
    
    fn count(&self) -> usize {
        match self {
            Container::Empty => 0,
            Container::Single(_) => 1,
            Container::Pair(_, _) => 2,
        }
    }
}

impl<T: Clone> Container<T> {
    fn get_all(&self) -> Vec<T> {
        match self {
            Container::Empty => vec![],
            Container::Single(item) => vec![item.clone()],
            Container::Pair(item1, item2) => vec![item1.clone(), item2.clone()],
        }
    }
}

// 带多个泛型参数的枚举
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    
    fn left(self) -> Option<L> {
        match self {
            Either::Left(value) => Some(value),
            Either::Right(_) => None,
        }
    }
    
    fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(value) => Some(value),
        }
    }
}

fn generic_enums() {
    println!("\n=== 泛型枚举示例 ===");
    
    // 容器枚举
    let empty: Container<i32> = Container::Empty;
    let single = Container::Single(42);
    let pair = Container::Pair("hello", "world");
    
    println!("空容器: {:?}, 计数: {}", empty, empty.count());
    println!("单元素容器: {:?}, 计数: {}", single, single.count());
    println!("双元素容器: {:?}, 计数: {}", pair, pair.count());
    
    let string_container = Container::Single("Rust".to_string());
    let all_items = string_container.get_all();
    println!("所有元素: {:?}", all_items);
    
    // Either枚举
    let left_value: Either<i32, String> = Either::Left(42);
    let right_value: Either<i32, String> = Either::Right("hello".to_string());
    
    println!("左值: {:?}, 是否为左: {}", left_value, left_value.is_left());
    println!("右值: {:?}, 是否为右: {}", right_value, right_value.is_right());
    
    if let Some(value) = Either::Left(100).left() {
        println!("提取的左值: {}", value);
    }
}

// 泛型方法和关联类型
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator<usize> for Counter {
    fn next(&mut self) -> Option<usize> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// 泛型集合
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Display> Stack<T> {
    fn display_all(&self) {
        println!("栈内容 (从底到顶):");
        for (i, item) in self.items.iter().enumerate() {
            println!("  {}: {}", i, item);
        }
    }
}

fn generic_methods() {
    println!("\n=== 泛型方法示例 ===");
    
    // 计数器
    let mut counter = Counter::new(5);
    print!("计数序列: ");
    while let Some(value) = counter.next() {
        print!("{} ", value);
    }
    println!();
    
    // 泛型栈
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    println!("整数栈长度: {}", int_stack.len());
    int_stack.display_all();
    
    while let Some(item) = int_stack.pop() {
        println!("弹出: {}", item);
    }
    
    // 字符串栈
    let mut string_stack = Stack::new();
    string_stack.push("first".to_string());
    string_stack.push("second".to_string());
    string_stack.push("third".to_string());
    
    string_stack.display_all();
    
    if let Some(top) = string_stack.peek() {
        println!("栈顶元素: {}", top);
    }
}

// 复杂泛型示例：通用缓存
#[derive(Debug)]
struct Cache<K, V> 
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    data: std::collections::HashMap<K, V>,
    capacity: usize,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Cache {
            data: std::collections::HashMap::new(),
            capacity,
        }
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
            // 简单的LRU策略：删除第一个键
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value)
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
}

fn cache_example() {
    println!("\n=== 缓存示例 ===");
    
    let mut cache: Cache<String, i32> = Cache::new(3);
    
    cache.insert("key1".to_string(), 100);
    cache.insert("key2".to_string(), 200);
    cache.insert("key3".to_string(), 300);
    
    println!("缓存长度: {}", cache.len());
    
    if let Some(value) = cache.get(&"key2".to_string()) {
        println!("key2的值: {}", value);
    }
    
    // 插入第四个元素，将超出容量
    cache.insert("key4".to_string(), 400);
    println!("插入key4后，缓存长度: {}", cache.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(*largest(&numbers), 100);
        
        let chars = vec!['y', 'm', 'a', 'q'];
        assert_eq!(*largest(&chars), 'y');
    }

    #[test]
    fn test_point() {
        let point = Point::new(5, 10);
        assert_eq!(*point.x(), 5);
        assert_eq!(*point.y(), 10);
        
        let float_point = Point::new(3.0, 4.0);
        assert_eq!(float_point.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_container() {
        let empty: Container<i32> = Container::Empty;
        assert!(empty.is_empty());
        assert_eq!(empty.count(), 0);
        
        let single = Container::Single(42);
        assert_eq!(single.count(), 1);
        
        let pair = Container::Pair(1, 2);
        assert_eq!(pair.count(), 2);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&2));
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(2);
        cache.insert("key1", 100);
        cache.insert("key2", 200);
        
        assert_eq!(cache.get(&"key1"), Some(&100));
        assert_eq!(cache.len(), 2);
        
        cache.insert("key3", 300);  // 超出容量
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_examples() {
        generic_functions();
        generic_structs();
        generic_enums();
        generic_methods();
        cache_example();
    }
}

// 泛型要点总结：
// 1. 泛型允许编写适用于多种类型的代码
// 2. 使用<T>语法定义类型参数
// 3. 可以为泛型添加trait约束
// 4. 泛型在编译时进行单态化，没有运行时开销
// 5. where子句提供更清晰的约束语法
// 6. 可以为特定的泛型类型实现特定方法
// 7. 泛型与Rust的所有权系统完美结合