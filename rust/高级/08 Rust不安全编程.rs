// 08 Rust不安全编程 - unsafe关键字和原始指针
// 本章介绍Rust的不安全编程：unsafe块、原始指针、FFI和内存操作

use std::slice;
use std::ffi::{CStr, CString};

fn main() {
    // 原始指针示例
    raw_pointer_examples();
    
    // 不安全函数示例
    unsafe_function_examples();
    
    // 内存操作示例
    memory_operation_examples();
    
    // FFI示例
    ffi_examples();
}

// 案例1：原始指针操作
fn raw_pointer_examples() {
    println!("=== 原始指针示例 ===");
    
    let mut num = 5;
    
    // 创建原始指针
    let r1 = &num as *const i32;           // 不可变原始指针
    let r2 = &mut num as *mut i32;         // 可变原始指针
    
    println!("原始指针地址: {:p}, {:p}", r1, r2);
    
    // 解引用原始指针（需要unsafe）
    unsafe {
        println!("r1指向的值: {}", *r1);
        println!("r2指向的值: {}", *r2);
        
        // 通过可变原始指针修改值
        *r2 = 10;
        println!("修改后的值: {}", *r2);
    }
    
    println!("变量num的值: {}", num);
    
    // 从任意地址创建原始指针
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("任意地址的原始指针: {:p}", r);
    // 注意：解引用任意地址的指针是非常危险的！
    
    // 原始指针运算
    unsafe {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr();
        
        println!("数组元素:");
        for i in 0..arr.len() {
            let element_ptr = ptr.add(i);
            println!("  索引{}: {}", i, *element_ptr);
        }
    }
}

// 不安全函数
unsafe fn dangerous() -> String {
    "这是一个不安全函数".to_string()
}

// 包装不安全操作的安全函数
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 案例2：不安全函数和安全抽象
fn unsafe_function_examples() {
    println!("\n=== 不安全函数示例 ===");
    
    // 调用不安全函数
    unsafe {
        let message = dangerous();
        println!("不安全函数返回: {}", message);
    }
    
    // 使用安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    
    println!("分割后的切片:");
    println!("  左侧: {:?}", left);
    println!("  右侧: {:?}", right);
    
    // 修改切片内容
    left[0] = 10;
    right[0] = 20;
    
    println!("修改后的向量: {:?}", v);
    
    // 创建自定义智能指针
    let mut value = 42;
    let smart_ptr = SmartPointer::new(&mut value);
    
    unsafe {
        println!("智能指针指向的值: {}", smart_ptr.get());
        smart_ptr.set(100);
        println!("修改后的值: {}", smart_ptr.get());
    }
    
    println!("原始值: {}", value);
}

// 自定义智能指针
struct SmartPointer<T> {
    ptr: *mut T,
}

impl<T> SmartPointer<T> {
    fn new(reference: &mut T) -> Self {
        SmartPointer {
            ptr: reference as *mut T,
        }
    }
    
    unsafe fn get(&self) -> &T {
        &*self.ptr
    }
    
    unsafe fn set(&self, value: T) {
        *self.ptr = value;
    }
}

// 案例3：内存操作
fn memory_operation_examples() {
    println!("\n=== 内存操作示例 ===");
    
    // 手动内存分配
    unsafe {
        use std::alloc::{alloc, dealloc, Layout};
        
        let layout = Layout::new::<u32>();
        let ptr = alloc(layout) as *mut u32;
        
        if ptr.is_null() {
            panic!("内存分配失败");
        }
        
        // 写入数据
        *ptr = 42;
        println!("分配的内存中的值: {}", *ptr);
        
        // 释放内存
        dealloc(ptr as *mut u8, layout);
        println!("内存已释放");
    }
    
    // 内存复制
    let src = [1, 2, 3, 4, 5];
    let mut dst = [0; 5];
    
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    
    println!("复制的数组: {:?}", dst);
    
    // 联合体（Union）示例
    #[repr(C)]
    union IntOrFloat {
        int_val: i32,
        float_val: f32,
    }
    
    let mut union_val = IntOrFloat { int_val: 42 };
    
    unsafe {
        println!("联合体作为整数: {}", union_val.int_val);
        
        union_val.float_val = 3.14;
        println!("联合体作为浮点数: {}", union_val.float_val);
        
        // 注意：这可能会产生无效的整数值
        println!("同一内存作为整数: {}", union_val.int_val);
    }
    
    // 内存操作工具函数
    memory_utilities();
}

fn memory_utilities() {
    println!("\n--- 内存操作工具 ---");
    
    // 手动实现Vec的简化版本
    struct SimpleVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SimpleVec<T> {
        fn new() -> Self {
            SimpleVec {
                ptr: std::ptr::NonNull::dangling().as_ptr(),
                len: 0,
                capacity: 0,
            }
        }
        
        fn push(&mut self, element: T) {
            if self.len == self.capacity {
                self.grow();
            }
            
            unsafe {
                std::ptr::write(self.ptr.add(self.len), element);
            }
            self.len += 1;
        }
        
        fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                unsafe {
                    Some(std::ptr::read(self.ptr.add(self.len)))
                }
            }
        }
        
        fn grow(&mut self) {
            let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
            
            unsafe {
                use std::alloc::{alloc, realloc, Layout};
                
                let new_layout = Layout::array::<T>(new_capacity).unwrap();
                
                let new_ptr = if self.capacity == 0 {
                    alloc(new_layout)
                } else {
                    let old_layout = Layout::array::<T>(self.capacity).unwrap();
                    realloc(self.ptr as *mut u8, old_layout, new_layout.size())
                };
                
                if new_ptr.is_null() {
                    panic!("内存分配失败");
                }
                
                self.ptr = new_ptr as *mut T;
                self.capacity = new_capacity;
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe {
                    Some(&*self.ptr.add(index))
                }
            } else {
                None
            }
        }
    }
    
    impl<T> Drop for SimpleVec<T> {
        fn drop(&mut self) {
            // 析构所有元素
            while let Some(_) = self.pop() {}
            
            // 释放内存
            if self.capacity != 0 {
                unsafe {
                    use std::alloc::{dealloc, Layout};
                    let layout = Layout::array::<T>(self.capacity).unwrap();
                    dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
    
    // 测试自定义Vec
    let mut vec = SimpleVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("自定义Vec长度: {}", vec.len());
    
    for i in 0..vec.len() {
        if let Some(value) = vec.get(i) {
            println!("  元素{}: {}", i, value);
        }
    }
    
    while let Some(value) = vec.pop() {
        println!("弹出元素: {}", value);
    }
}

// 案例4：外部函数接口（FFI）
// 声明外部C函数
extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

// 导出给C使用的Rust函数
#[no_mangle]
pub extern "C" fn call_from_c(x: i32) -> i32 {
    println!("从C调用的Rust函数，参数: {}", x);
    x * 2
}

// Rust函数供其他语言调用
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn ffi_examples() {
    println!("\n=== FFI示例 ===");
    
    // 调用C标准库函数
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
    }
    
    // 使用C字符串
    let c_string = CString::new("Hello, C!").expect("CString::new failed");
    
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("C字符串长度: {}", len);
    }
    
    // 从C字符串创建Rust字符串
    let c_str = "Hello from C\0";
    let c_str_ptr = c_str.as_ptr() as *const std::os::raw::c_char;
    
    unsafe {
        let rust_str = CStr::from_ptr(c_str_ptr).to_str().unwrap();
        println!("从C字符串转换: {}", rust_str);
    }
    
    // 示例：调用导出的函数（模拟）
    let result = call_from_c(21);
    println!("导出函数结果: {}", result);
    
    let sum = add_numbers(10, 20);
    println!("加法函数结果: {}", sum);
    
    // 与C库集成的示例
    c_integration_example();
}

fn c_integration_example() {
    println!("\n--- C集成示例 ---");
    
    // 模拟一个需要与C库交互的场景
    
    // 创建一个包含C结构体的Rust结构体
    #[repr(C)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
        
        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }
    
    // 模拟C库函数（实际上是Rust实现）
    #[no_mangle]
    pub extern "C" fn point_distance(p1: *const Point, p2: *const Point) -> f64 {
        unsafe {
            if p1.is_null() || p2.is_null() {
                return -1.0;  // 错误代码
            }
            
            let point1 = &*p1;
            let point2 = &*p2;
            point1.distance(point2)
        }
    }
    
    // 使用"C库"函数
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    
    let distance = point_distance(&p1 as *const Point, &p2 as *const Point);
    println!("两点间距离: {}", distance);
    
    // 回调函数示例
    type CallbackFn = extern "C" fn(i32) -> i32;
    
    extern "C" fn square(x: i32) -> i32 {
        x * x
    }
    
    extern "C" fn cube(x: i32) -> i32 {
        x * x * x
    }
    
    fn apply_callback(f: CallbackFn, value: i32) -> i32 {
        f(value)
    }
    
    let result1 = apply_callback(square, 5);
    let result2 = apply_callback(cube, 3);
    
    println!("回调结果: square(5) = {}, cube(3) = {}", result1, result2);
}

// 不安全trait
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

// 实现不安全trait
struct UnsafeStruct {
    data: *mut i32,
}

unsafe impl UnsafeTrait for UnsafeStruct {
    fn unsafe_method(&self) {
        unsafe {
            if !self.data.is_null() {
                println!("不安全方法访问数据: {}", *self.data);
            }
        }
    }
}

// 高级不安全操作
fn advanced_unsafe_operations() {
    println!("\n=== 高级不安全操作 ===");
    
    // 类型转换
    let x: i32 = 42;
    let y: u32 = unsafe { std::mem::transmute(x) };
    println!("类型转换: i32 {} -> u32 {}", x, y);
    
    // 内存忘记（阻止析构函数运行）
    let boxed = Box::new(String::from("不会被析构"));
    let ptr = Box::into_raw(boxed);
    
    unsafe {
        println!("原始指针指向的字符串: {}", &*ptr);
        // 手动释放内存
        let _boxed_again = Box::from_raw(ptr);
        // boxed_again在这里会被正常析构
    }
    
    // 不安全trait使用
    let mut value = 100;
    let unsafe_struct = UnsafeStruct {
        data: &mut value as *mut i32,
    };
    
    unsafe_struct.unsafe_method();
    
    // 内存对齐
    #[repr(C, packed)]
    struct PackedStruct {
        a: u8,
        b: u32,
        c: u8,
    }
    
    let packed = PackedStruct { a: 1, b: 2, c: 3 };
    
    println!("打包结构体大小: {}", std::mem::size_of::<PackedStruct>());
    println!("  a: {}, b: {}, c: {}", packed.a, packed.b, packed.c);
    
    // 注意：访问打包结构体的字段可能导致未对齐访问
    let b_ptr: *const u32 = &packed.b;
    unsafe {
        println!("通过指针访问b: {}", std::ptr::read_unaligned(b_ptr));
    }
}

// 内存泄漏检测器
struct LeakDetector {
    allocations: std::collections::HashMap<*mut u8, usize>,
}

impl LeakDetector {
    fn new() -> Self {
        LeakDetector {
            allocations: std::collections::HashMap::new(),
        }
    }
    
    unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
        use std::alloc::{alloc, Layout};
        
        let layout = Layout::from_size_align(size, 1).unwrap();
        let ptr = alloc(layout);
        
        if !ptr.is_null() {
            self.allocations.insert(ptr, size);
            println!("分配内存: {:p} ({}字节)", ptr, size);
        }
        
        ptr
    }
    
    unsafe fn deallocate(&mut self, ptr: *mut u8) {
        if let Some(size) = self.allocations.remove(&ptr) {
            use std::alloc::{dealloc, Layout};
            let layout = Layout::from_size_align(size, 1).unwrap();
            dealloc(ptr, layout);
            println!("释放内存: {:p} ({}字节)", ptr, size);
        } else {
            println!("警告: 尝试释放未知指针: {:p}", ptr);
        }
    }
    
    fn check_leaks(&self) {
        if self.allocations.is_empty() {
            println!("没有内存泄漏");
        } else {
            println!("检测到内存泄漏:");
            for (&ptr, &size) in &self.allocations {
                println!("  {:p}: {}字节", ptr, size);
            }
        }
    }
}

fn memory_leak_detection() {
    println!("\n=== 内存泄漏检测 ===");
    
    let mut detector = LeakDetector::new();
    
    unsafe {
        let ptr1 = detector.allocate(64);
        let ptr2 = detector.allocate(128);
        
        // 只释放一个指针
        detector.deallocate(ptr1);
        
        // 检查泄漏（ptr2应该仍然分配着）
        detector.check_leaks();
        
        // 清理剩余内存
        detector.deallocate(ptr2);
        detector.check_leaks();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raw_pointers() {
        let mut num = 5;
        let ptr = &mut num as *mut i32;
        
        unsafe {
            *ptr = 10;
        }
        
        assert_eq!(num, 10);
    }
    
    #[test]
    fn test_split_at_mut() {
        let mut arr = [1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut arr, 2);
        
        assert_eq!(left, &mut [1, 2]);
        assert_eq!(right, &mut [3, 4, 5]);
    }
    
    #[test]
    fn test_smart_pointer() {
        let mut value = 42;
        let smart_ptr = SmartPointer::new(&mut value);
        
        unsafe {
            assert_eq!(*smart_ptr.get(), 42);
            smart_ptr.set(100);
            assert_eq!(*smart_ptr.get(), 100);
        }
        
        assert_eq!(value, 100);
    }
    
    #[test]
    fn test_c_functions() {
        unsafe {
            assert_eq!(abs(-42), 42);
            assert_eq!(abs(42), 42);
        }
        
        let result = add_numbers(10, 20);
        assert_eq!(result, 30);
    }
    
    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        
        let distance = point_distance(&p1, &p2);
        assert_eq!(distance, 5.0);
    }
    
    #[test]
    fn test_examples() {
        raw_pointer_examples();
        unsafe_function_examples();
        memory_operation_examples();
        ffi_examples();
        advanced_unsafe_operations();
        memory_leak_detection();
    }
}

// 不安全编程要点总结：
// 1. unsafe关键字标记不安全代码块
// 2. 原始指针允许直接内存访问
// 3. 不安全函数必须在unsafe块中调用
// 4. 手动内存管理需要配对分配和释放
// 5. FFI用于与其他语言的互操作
// 6. 联合体允许多种类型共享内存
// 7. 不安全代码应该被安全抽象封装
// 8. 使用不安全代码时要格外小心内存安全和线程安全