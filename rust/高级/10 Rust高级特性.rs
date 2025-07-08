// 10 Rust高级特性 - 高级类型、函数式编程和元编程技巧
// 本章介绍Rust的高级特性：高级类型、闭包、迭代器、函数式编程等

use std::ops::{Add, Deref, Index};
use std::fmt::{self, Display};

fn main() {
    // 高级类型示例
    advanced_types();
    
    // 闭包和函数式编程
    closures_and_functional();
    
    // 迭代器和适配器
    iterators_and_adapters();
    
    // 高级trait和关联类型
    advanced_traits();
    
    // 类型级编程
    type_level_programming();
}

// 案例1：高级类型系统
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

// 新类型模式
struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrapper({})", self.0.join(", "))
    }
}

// 类型别名简化复杂类型
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// 动态大小类型
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("画一个半径为 {:.2} 的圆", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("画一个 {:.2} x {:.2} 的矩形", self.width, self.height);
    }
}

fn advanced_types() {
    println!("=== 高级类型示例 ===");
    
    // 类型别名
    let distance: Kilometers = 100;
    println!("距离: {} 千米", distance);
    
    // 新类型模式
    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("包装器: {}", w);
    println!("长度: {}", w.len());  // 通过Deref可以调用Vec的方法
    
    // 函数指针类型
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    let f: fn(i32) -> i32 = add_one;
    println!("函数指针结果: {}", f(5));
    
    // 闭包作为函数参数
    let closure = |x: i32| x + 1;
    println!("闭包结果: {}", closure(5));
    
    // Thunk示例
    let thunk: Thunk = Box::new(|| {
        println!("这是一个thunk");
    });
    thunk();
    
    // trait对象
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
    
    // 函数式编程组合器
    fn_composition_example();
}

fn fn_composition_example() {
    println!("\n--- 函数组合示例 ---");
    
    // 高阶函数
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }
    
    let double = |x| x * 2;
    let add_one = |x| x + 1;
    
    let result1 = apply_twice(double, 5);
    let result2 = apply_twice(add_one, 5);
    
    println!("应用两次double(5): {}", result1);
    println!("应用两次add_one(5): {}", result2);
    
    // 函数组合
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
    {
        move |x| f(g(x))
    }
    
    let add_ten = |x: i32| x + 10;
    let multiply_by_two = |x: i32| x * 2;
    
    let composed = compose(multiply_by_two, add_ten);
    println!("组合函数 (x+10)*2 with x=5: {}", composed(5));
}

// 案例2：闭包和函数式编程
fn closures_and_functional() {
    println!("\n=== 闭包和函数式编程示例 ===");
    
    // 闭包捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;  // 捕获x
    let y = 4;
    println!("y等于x吗? {}", equal_to_x(y));
    
    // 闭包的三种形式
    let list = vec![1, 2, 3, 4, 5];
    
    // FnOnce: 获取所有权
    let consume_closure = || {
        println!("从闭包中: {:?}", list);
    };
    consume_closure();
    // consume_closure();  // 编译错误：只能调用一次
    
    // Fn: 不可变借用
    let list2 = vec![1, 2, 3, 4, 5];
    let immutable_closure = || {
        println!("从不可变闭包中: {:?}", list2);
    };
    immutable_closure();
    immutable_closure();  // 可以多次调用
    
    // FnMut: 可变借用
    let mut list3 = vec![1, 2, 3, 4, 5];
    let mut mutable_closure = || {
        list3.push(6);
        println!("从可变闭包中: {:?}", list3);
    };
    mutable_closure();
    mutable_closure();
    
    // 缓存计算结果的闭包
    caching_closure_example();
    
    // 函数式数据处理
    functional_data_processing();
}

// 缓存闭包示例
fn caching_closure_example() {
    println!("\n--- 缓存闭包示例 ---");
    
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    
    let mut expensive_closure = Cacher::new(|num| {
        println!("计算中...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        num
    });
    
    println!("第一次调用: {}", expensive_closure.value(10));
    println!("第二次调用: {}", expensive_closure.value(10));
}

fn functional_data_processing() {
    println!("\n--- 函数式数据处理 ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)    // 过滤偶数
        .map(|&x| x * x)             // 平方
        .collect();
    
    println!("偶数的平方: {:?}", result);
    
    // 折叠操作
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("数字总和: {}", sum);
    
    // 查找操作
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);
    
    // 分组操作
    let (evens, odds): (Vec<_>, Vec<_>) = numbers
        .iter()
        .partition(|&&x| x % 2 == 0);
    
    println!("偶数: {:?}", evens);
    println!("奇数: {:?}", odds);
}

// 案例3：迭代器和适配器
fn iterators_and_adapters() {
    println!("\n=== 迭代器和适配器示例 ===");
    
    let v1 = vec![1, 2, 3];
    
    // 基本迭代器
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("迭代器值: {}", val);
    }
    
    // 迭代器适配器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("映射后的向量: {:?}", v2);
    
    // 消费适配器
    let sum: i32 = v1.iter().sum();
    println!("向量总和: {}", sum);
    
    // 自定义迭代器
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
    
    let mut counter = Counter::new(5);
    for num in counter {
        println!("计数器: {}", num);
    }
    
    // 链式迭代器操作
    let result: Vec<_> = Counter::new(10)
        .skip(2)                    // 跳过前2个
        .take(5)                    // 取5个
        .filter(|&x| x % 2 == 0)    // 过滤偶数
        .map(|x| x * x)             // 平方
        .collect();
    
    println!("复杂迭代器操作结果: {:?}", result);
    
    // 性能对比
    iterator_performance_comparison();
}

fn iterator_performance_comparison() {
    println!("\n--- 迭代器性能对比 ---");
    
    use std::time::Instant;
    
    let large_vec: Vec<i32> = (0..1_000_000).collect();
    
    // 使用for循环
    let start = Instant::now();
    let mut sum1 = 0;
    for &item in &large_vec {
        if item % 2 == 0 {
            sum1 += item * item;
        }
    }
    let duration1 = start.elapsed();
    
    // 使用迭代器
    let start = Instant::now();
    let sum2: i32 = large_vec
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    let duration2 = start.elapsed();
    
    println!("for循环结果: {}, 耗时: {:?}", sum1, duration1);
    println!("迭代器结果: {}, 耗时: {:?}", sum2, duration2);
    println!("性能比较: 迭代器是零成本抽象");
}

// 案例4：高级trait和关联类型
trait Iterator2 {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // 默认方法实现
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map { iter: self, f }
    }
}

struct Map<I, F> {
    iter: I,
    f: F,
}

impl<B, I: Iterator2, F> Iterator2 for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
}

// 操作符重载
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Index<usize> for Point {
    type Output = f64;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("索引超出范围"),
        }
    }
}

// 泛型特化（模拟）
trait MyTrait<T> {
    fn process(&self, item: T) -> String;
}

struct MyStruct;

impl MyTrait<i32> for MyStruct {
    fn process(&self, item: i32) -> String {
        format!("处理整数: {}", item)
    }
}

impl MyTrait<String> for MyStruct {
    fn process(&self, item: String) -> String {
        format!("处理字符串: {}", item)
    }
}

fn advanced_traits() {
    println!("\n=== 高级Trait示例 ===");
    
    // 操作符重载
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;
    
    println!("点加法: {:?} + {:?} = {:?}", p1, p2, p3);
    println!("点索引: p1[0] = {}, p1[1] = {}", p1[0], p1[1]);
    
    // trait特化
    let processor = MyStruct;
    println!("{}", processor.process(42));
    println!("{}", processor.process("Hello".to_string()));
    
    // 高级trait bound
    advanced_trait_bounds();
    
    // 关联类型vs泛型
    associated_types_vs_generics();
}

fn advanced_trait_bounds() {
    println!("\n--- 高级Trait约束 ---");
    
    // 多个trait bound
    fn print_and_compare<T>(item1: &T, item2: &T)
    where
        T: Display + PartialEq,
    {
        println!("item1: {}, item2: {}", item1, item2);
        if item1 == item2 {
            println!("它们相等");
        } else {
            println!("它们不相等");
        }
    }
    
    print_and_compare(&5, &5);
    print_and_compare(&"hello", &"world");
    
    // 条件trait实现
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Pair { x, y }
        }
    }
    
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大的成员是 x = {}", self.x);
            } else {
                println!("最大的成员是 y = {}", self.y);
            }
        }
    }
    
    let pair = Pair::new(5, 10);
    pair.cmp_display();
}

fn associated_types_vs_generics() {
    println!("\n--- 关联类型vs泛型 ---");
    
    // 使用关联类型的trait
    trait AssociatedIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    // 使用泛型的trait
    trait GenericIterator<T> {
        fn next(&mut self) -> Option<T>;
    }
    
    struct NumberIterator {
        current: i32,
        max: i32,
    }
    
    impl AssociatedIterator for NumberIterator {
        type Item = i32;
        
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
    
    let mut iter = NumberIterator { current: 0, max: 3 };
    while let Some(num) = iter.next() {
        println!("关联类型迭代器: {}", num);
    }
}

// 案例5：类型级编程
fn type_level_programming() {
    println!("\n=== 类型级编程示例 ===");
    
    // 幻影类型
    use std::marker::PhantomData;
    
    struct Measurement<Unit> {
        value: f64,
        _unit: PhantomData<Unit>,
    }
    
    struct Meter;
    struct Kilometer;
    
    impl<Unit> Measurement<Unit> {
        fn new(value: f64) -> Self {
            Measurement {
                value,
                _unit: PhantomData,
            }
        }
        
        fn value(&self) -> f64 {
            self.value
        }
    }
    
    impl Measurement<Meter> {
        fn to_kilometers(self) -> Measurement<Kilometer> {
            Measurement::new(self.value / 1000.0)
        }
    }
    
    impl Measurement<Kilometer> {
        fn to_meters(self) -> Measurement<Meter> {
            Measurement::new(self.value * 1000.0)
        }
    }
    
    let distance_m = Measurement::<Meter>::new(1500.0);
    let distance_km = distance_m.to_kilometers();
    
    println!("距离: {} 米 = {} 千米", 1500.0, distance_km.value());
    
    // 类型状态模式
    type_state_pattern();
    
    // 编译时计算
    compile_time_computation();
}

fn type_state_pattern() {
    println!("\n--- 类型状态模式 ---");
    
    struct Open;
    struct Closed;
    
    struct Door<State> {
        _state: std::marker::PhantomData<State>,
    }
    
    impl Door<Closed> {
        fn new() -> Door<Closed> {
            Door {
                _state: std::marker::PhantomData,
            }
        }
        
        fn open(self) -> Door<Open> {
            println!("门被打开了");
            Door {
                _state: std::marker::PhantomData,
            }
        }
    }
    
    impl Door<Open> {
        fn close(self) -> Door<Closed> {
            println!("门被关闭了");
            Door {
                _state: std::marker::PhantomData,
            }
        }
        
        fn walk_through(&self) {
            println!("穿过门");
        }
    }
    
    let door = Door::new();        // 初始状态：关闭
    let door = door.open();        // 打开门
    door.walk_through();           // 可以穿过
    let _door = door.close();      // 关闭门
    
    // door.walk_through();        // 编译错误：门已关闭
}

fn compile_time_computation() {
    println!("\n--- 编译时计算 ---");
    
    // 常量泛型
    struct Array<T, const N: usize> {
        data: [T; N],
    }
    
    impl<T, const N: usize> Array<T, N> {
        fn new(data: [T; N]) -> Self {
            Array { data }
        }
        
        fn len(&self) -> usize {
            N
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }
    }
    
    let arr = Array::new([1, 2, 3, 4, 5]);
    println!("数组长度: {}", arr.len());
    println!("第一个元素: {:?}", arr.get(0));
    
    // 类型级数字
    trait TypeNum {
        const VALUE: usize;
    }
    
    struct Zero;
    struct Succ<N>(std::marker::PhantomData<N>);
    
    impl TypeNum for Zero {
        const VALUE: usize = 0;
    }
    
    impl<N: TypeNum> TypeNum for Succ<N> {
        const VALUE: usize = N::VALUE + 1;
    }
    
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type Three = Succ<Two>;
    
    println!("类型级数字:");
    println!("Zero: {}", Zero::VALUE);
    println!("One: {}", One::VALUE);
    println!("Two: {}", Two::VALUE);
    println!("Three: {}", Three::VALUE);
}

// 高级模式匹配
fn advanced_pattern_matching() {
    println!("\n=== 高级模式匹配 ===");
    
    // 守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于五: {}", x),
        Some(x) => println!("大于等于五: {}", x),
        None => println!("没有值"),
    }
    
    // @绑定
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("找到范围内的id: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("找到另一个范围内的id");
        }
        Message::Hello { id } => {
            println!("找到其他id: {}", id);
        }
    }
    
    // 解构复杂数据
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3.0, y: -10.0 });
    println!("解构结果: {} 英尺 {} 英寸, 点 ({}, {})", feet, inches, x, y);
}

// 性能优化技巧
fn performance_optimization() {
    println!("\n=== 性能优化技巧 ===");
    
    // 零成本抽象演示
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用迭代器（零成本）
    let sum: i32 = numbers.iter().map(|&x| x * 2).sum();
    println!("迭代器计算结果: {}", sum);
    
    // 内联函数
    #[inline(always)]
    fn fast_add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    let result = fast_add(5, 3);
    println!("内联函数结果: {}", result);
    
    // 避免不必要的分配
    let text = "hello,world,rust";
    let words: Vec<&str> = text.split(',').collect();  // 借用，不分配新字符串
    println!("分割结果: {:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_types() {
        let w = Wrapper(vec!["test".to_string()]);
        assert_eq!(w.len(), 1);
    }
    
    #[test]
    fn test_point_operations() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        let p3 = p1 + p2;
        
        assert_eq!(p3, Point { x: 4.0, y: 6.0 });
        assert_eq!(p1[0], 1.0);
        assert_eq!(p1[1], 2.0);
    }
    
    #[test]
    fn test_counter_iterator() {
        let sum: usize = Counter::new(5).sum();
        assert_eq!(sum, 10);  // 0+1+2+3+4 = 10
    }
    
    #[test]
    fn test_measurement_conversion() {
        let distance = Measurement::<Meter>::new(1000.0);
        let km_distance = distance.to_kilometers();
        assert_eq!(km_distance.value(), 1.0);
    }
    
    #[test]
    fn test_array_const_generic() {
        let arr = Array::new([1, 2, 3]);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), Some(&1));
    }
    
    #[test]
    fn test_examples() {
        advanced_types();
        closures_and_functional();
        iterators_and_adapters();
        advanced_traits();
        type_level_programming();
        advanced_pattern_matching();
        performance_optimization();
    }
}

// 高级特性要点总结：
// 1. 类型别名和新类型模式提供更好的抽象
// 2. 闭包捕获环境，支持函数式编程
// 3. 迭代器是零成本抽象，性能优异
// 4. 关联类型简化复杂的泛型约束
// 5. 幻影类型实现编译时状态检查
// 6. 常量泛型支持编译时计算
// 7. 高级模式匹配提供强大的解构能力
// 8. 合理使用高级特性可以提高代码质量和性能