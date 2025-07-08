// 02 Rust特征 - 定义和实现trait
// 本章介绍Rust的trait系统：定义行为、实现trait和trait约束

use std::fmt::{Debug, Display};

fn main() {
    // 基本trait示例
    basic_traits();
    
    // trait约束示例
    trait_bounds();
    
    // 关联类型示例
    associated_types();
    
    // trait对象示例
    trait_objects();
}

// 基本trait定义
trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn author(&self) -> String {
        String::from("Unknown Author")
    }
    
    // 使用其他方法的默认实现
    fn full_summary(&self) -> String {
        format!("Author: {}, Summary: {}", self.author(), self.summarize())
    }
}

// 新闻文章结构体
#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn author(&self) -> String {
        self.author.clone()
    }
}

// 推文结构体
#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 案例1：基本trait使用
fn basic_traits() {
    println!("=== 基本Trait示例 ===");
    
    let article = NewsArticle {
        headline: String::from("Rust 1.70发布"),
        location: String::from("全球"),
        author: String::from("Rust团队"),
        content: String::from("Rust 1.70带来了许多新特性..."),
    };
    
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust让系统编程变得安全而高效！"),
        reply: false,
        retweet: false,
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("文章作者: {}", article.author());
    println!("文章完整摘要: {}", article.full_summary());
    
    println!("推文摘要: {}", tweet.summarize());
    println!("推文作者: {}", tweet.author());
    println!("推文完整摘要: {}", tweet.full_summary());
    
    // 使用trait作为参数
    notify(&article);
    notify(&tweet);
    
    // 使用impl Trait语法
    notify_impl_trait(&article);
    notify_impl_trait(&tweet);
}

// trait作为参数
fn notify(item: &impl Summary) {
    println!("新消息: {}", item.summarize());
}

// impl Trait语法
fn notify_impl_trait(item: &impl Summary) {
    println!("通知: {}", item.full_summary());
}

// trait bound语法
fn notify_trait_bound<T: Summary>(item: &T) {
    println!("推送: {}", item.summarize());
}

// 多个trait约束
fn notify_multiple<T: Summary + Display>(item: &T) {
    println!("显示: {}", item);
    println!("摘要: {}", item.summarize());
}

// where子句简化复杂约束
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42
}

// 案例2：trait约束和泛型
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("画一个半径为 {:.2} 的圆形", self.radius);
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("画一个 {:.2} x {:.2} 的矩形", self.width, self.height);
    }
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 返回实现了trait的类型
fn create_shape(shape_type: &str) -> Box<dyn Drawable> {
    match shape_type {
        "circle" => Box::new(Circle::new(5.0)),
        "rectangle" => Box::new(Rectangle::new(4.0, 6.0)),
        _ => Box::new(Circle::new(1.0)),
    }
}

fn trait_bounds() {
    println!("\n=== Trait约束示例 ===");
    
    let circle = Circle::new(3.0);
    let rectangle = Rectangle::new(4.0, 5.0);
    
    draw_shape(&circle);
    draw_shape(&rectangle);
    
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle::new(2.0)),
        Box::new(Rectangle::new(3.0, 4.0)),
        Box::new(Circle::new(1.5)),
    ];
    
    let total_area = calculate_total_area(&shapes);
    println!("总面积: {:.2}", total_area);
    
    // 动态创建形状
    let shape1 = create_shape("circle");
    let shape2 = create_shape("rectangle");
    
    shape1.draw();
    shape2.draw();
}

fn draw_shape<T: Drawable>(shape: &T) {
    shape.draw();
    println!("面积: {:.2}", shape.area());
}

fn calculate_total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

// 关联类型示例
trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
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

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// 计算trait
trait Calculate<T> {
    type Output;
    
    fn calculate(&self, other: &T) -> Self::Output;
}

struct Calculator;

impl Calculate<i32> for Calculator {
    type Output = i32;
    
    fn calculate(&self, other: &i32) -> Self::Output {
        other * 2
    }
}

impl Calculate<f64> for Calculator {
    type Output = f64;
    
    fn calculate(&self, other: &f64) -> Self::Output {
        other * 3.14
    }
}

fn associated_types() {
    println!("\n=== 关联类型示例 ===");
    
    let mut counter = Counter::new(5);
    print!("计数器输出: ");
    while let Some(value) = counter.next() {
        print!("{} ", value);
    }
    println!();
    
    let calc = Calculator;
    let int_result = calc.calculate(&10);
    let float_result = calc.calculate(&2.0);
    
    println!("整数计算结果: {}", int_result);
    println!("浮点数计算结果: {:.2}", float_result);
}

// 可比较trait
trait Comparable {
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

impl Comparable for Person {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)
    }
}

// 可克隆trait的自定义实现
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, pages: u32) -> Self {
        Book {
            title: title.to_string(),
            pages,
        }
    }
}

impl Clone for Book {
    fn clone(&self) -> Self {
        println!("正在克隆书籍: {}", self.title);
        Book {
            title: self.title.clone(),
            pages: self.pages,
        }
    }
}

// trait对象和动态分发
trait Animal {
    fn make_sound(&self);
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Self {
        Dog { name: name.to_string() }
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Self {
        Cat { name: name.to_string() }
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

fn trait_objects() {
    println!("\n=== Trait对象示例 ===");
    
    // 比较人员
    let person1 = Person::new("Alice", 30);
    let person2 = Person::new("Bob", 25);
    
    match person1.compare(&person2) {
        std::cmp::Ordering::Greater => println!("{} 比 {} 年龄大", person1.name, person2.name),
        std::cmp::Ordering::Less => println!("{} 比 {} 年龄小", person1.name, person2.name),
        std::cmp::Ordering::Equal => println!("{} 和 {} 年龄相同", person1.name, person2.name),
    }
    
    // 克隆书籍
    let book1 = Book::new("Rust编程", 500);
    let book2 = book1.clone();
    println!("原书: {:?}", book1);
    println!("副本: {:?}", book2);
    
    // 动物trait对象
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("旺财")),
        Box::new(Cat::new("小咪")),
        Box::new(Dog::new("大黄")),
    ];
    
    for animal in &animals {
        println!("{}说: ", animal.name());
        animal.make_sound();
    }
    
    // 使用trait对象的函数
    let dog = Dog::new("小白");
    let cat = Cat::new("花花");
    
    make_animal_sound(&dog);
    make_animal_sound(&cat);
}

fn make_animal_sound(animal: &dyn Animal) {
    println!("让{}发出声音:", animal.name());
    animal.make_sound();
}

// 高级trait示例：序列化
trait Serialize {
    fn serialize(&self) -> String;
}

impl Serialize for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Serialize for String {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> String {
        let items: Vec<String> = self.iter().map(|item| item.serialize()).collect();
        format!("[{}]", items.join(", "))
    }
}

fn serialization_example() {
    println!("\n=== 序列化示例 ===");
    
    let number = 42;
    let text = String::from("Hello, Rust!");
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("序列化数字: {}", number.serialize());
    println!("序列化字符串: {}", text.serialize());
    println!("序列化数组: {}", numbers.serialize());
}

// 条件trait实现
struct Wrapper<T>(T);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wrapper({})", self.0)
    }
}

fn conditional_implementation() {
    println!("\n=== 条件实现示例 ===");
    
    let wrapper = Wrapper(42);
    println!("包装器: {}", wrapper);
    
    let string_wrapper = Wrapper("Hello".to_string());
    println!("字符串包装器: {}", string_wrapper);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_trait() {
        let article = NewsArticle {
            headline: "Test".to_string(),
            location: "Test Location".to_string(),
            author: "Test Author".to_string(),
            content: "Test content".to_string(),
        };
        
        assert!(article.summarize().contains("Test"));
        assert_eq!(article.author(), "Test Author");
    }

    #[test]
    fn test_drawable_trait() {
        let circle = Circle::new(5.0);
        let area = circle.area();
        assert!((area - 78.54).abs() < 0.01);
        
        let rectangle = Rectangle::new(4.0, 5.0);
        assert_eq!(rectangle.area(), 20.0);
    }

    #[test]
    fn test_iterator_trait() {
        let mut counter = Counter::new(3);
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_comparable_trait() {
        let person1 = Person::new("Alice", 30);
        let person2 = Person::new("Bob", 25);
        
        assert_eq!(person1.compare(&person2), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_serialize_trait() {
        let number = 42;
        assert_eq!(number.serialize(), "42");
        
        let text = String::from("test");
        assert_eq!(text.serialize(), "\"test\"");
    }

    #[test]
    fn test_examples() {
        basic_traits();
        trait_bounds();
        associated_types();
        trait_objects();
        serialization_example();
        conditional_implementation();
    }
}

// Trait要点总结：
// 1. trait定义共同行为，类似于接口
// 2. 可以提供默认实现
// 3. trait可以用作函数参数和返回值的约束
// 4. 关联类型简化复杂的泛型约束
// 5. trait对象支持动态分发
// 6. 可以为外部类型实现自定义trait（孤儿规则）
// 7. where子句提供清晰的约束语法
// 8. trait是Rust零成本抽象的核心