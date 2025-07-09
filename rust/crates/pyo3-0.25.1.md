# PyO3 0.25.1 中文使用教程

## 概述

PyO3 是一个用于 Rust 和 Python 之间互操作的库，允许你在 Rust 中编写 Python 扩展模块，或者在 Python 中调用 Rust 代码。它提供了安全、高效的方式来连接这两种语言。

**版本**: 0.25.1
**许可证**: MIT OR Apache-2.0
**仓库**: https://github.com/pyo3/pyo3
**文档**: https://docs.rs/crate/pyo3/

## 主要特性

- 🔗 **双向绑定**: 在 Rust 中调用 Python，在 Python 中调用 Rust
- 🛡️ **内存安全**: 自动管理 Python 对象的生命周期
- ⚡ **高性能**: 零成本抽象，接近原生性能
- 🔧 **易用性**: 简单的宏和 API 设计
- 🐍 **Python 兼容**: 支持 Python 3.8+
- 📦 **打包集成**: 与 maturin 等工具完美集成

## 安装

### 在 Rust 项目中添加依赖

```toml
[dependencies]
pyo3 = { version = "0.25.1", features = ["auto-initialize"] }

# 如果要创建 Python 扩展模块
[lib]
name = "my_module"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.25.1"
features = ["extension-module"]
```

### 安装 maturin（用于构建和分发）

```bash
pip install maturin
```

## 基本概念

### 1. Python 对象在 Rust 中的表示

```rust
use pyo3::prelude::*;
use pyo3::types::{PyString, PyList, PyDict};

#[pyfunction]
fn work_with_python_objects(py: Python) -> PyResult<()> {
    // 创建 Python 字符串
    let py_string = PyString::new(py, "Hello, Python!");
    println!("Python string: {}", py_string.to_string());
    
    // 创建 Python 列表
    let py_list = PyList::new(py, &[1, 2, 3, 4, 5]);
    println!("Python list length: {}", py_list.len());
    
    // 创建 Python 字典
    let py_dict = PyDict::new(py);
    py_dict.set_item("key", "value")?;
    py_dict.set_item("number", 42)?;
    println!("Python dict: {:?}", py_dict);
    
    Ok(())
}
```

### 2. 类型转换

```rust
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyfunction]
fn type_conversion_examples(py: Python) -> PyResult<()> {
    // Rust 值转 Python 对象
    let rust_vec = vec![1, 2, 3, 4, 5];
    let py_list = PyList::new(py, &rust_vec);
    
    // Python 对象转 Rust 值
    let back_to_rust: Vec<i32> = py_list.extract()?;
    println!("Back to Rust: {:?}", back_to_rust);
    
    // 字符串转换
    let rust_string = "Hello, World!";
    let py_string = rust_string.to_object(py);
    let back_to_string: String = py_string.extract(py)?;
    println!("String conversion: {}", back_to_string);
    
    Ok(())
}
```

## 创建 Python 扩展模块

### 1. 简单的函数导出

```rust
use pyo3::prelude::*;

/// 计算两个数的和
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 计算阶乘
#[pyfunction]
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// 字符串处理函数
#[pyfunction]
fn process_string(input: &str) -> PyResult<String> {
    Ok(input.to_uppercase().replace(" ", "_"))
}

/// 列表处理函数
#[pyfunction]
fn sum_list(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

/// 使用可选参数
#[pyfunction]
fn greet(name: &str, greeting: Option<&str>) -> String {
    let greeting = greeting.unwrap_or("Hello");
    format!("{}, {}!", greeting, name)
}

/// 模块定义
#[pymodule]
fn my_rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(process_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_list, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
```

### 2. 创建 Python 类

```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
struct Counter {
    value: i32,
}

#[pymethods]
impl Counter {
    /// 构造函数
    #[new]
    fn new(initial_value: Option<i32>) -> Self {
        Counter {
            value: initial_value.unwrap_or(0),
        }
    }
    
    /// 增加计数器
    fn increment(&mut self) {
        self.value += 1;
    }
    
    /// 减少计数器
    fn decrement(&mut self) {
        self.value -= 1;
    }
    
    /// 获取当前值
    fn get_value(&self) -> i32 {
        self.value
    }
    
    /// 重置计数器
    fn reset(&mut self) {
        self.value = 0;
    }
    
    /// 属性访问器
    #[getter]
    fn value(&self) -> i32 {
        self.value
    }
    
    /// 属性设置器
    #[setter]
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
    
    /// 字符串表示
    fn __str__(&self) -> String {
        format!("Counter(value={})", self.value)
    }
    
    /// 调试表示
    fn __repr__(&self) -> String {
        format!("Counter(value={})", self.value)
    }
}

#[pyclass]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

#[pymethods]
impl Person {
    #[new]
    fn new(name: String, age: u32, email: Option<String>) -> Self {
        Person { name, age, email }
    }
    
    /// 获取全名
    fn get_full_info(&self) -> String {
        match &self.email {
            Some(email) => format!("{} ({}), email: {}", self.name, self.age, email),
            None => format!("{} ({})", self.name, self.age),
        }
    }
    
    /// 生日方法
    fn have_birthday(&mut self) {
        self.age += 1;
    }
    
    /// 更新邮箱
    fn update_email(&mut self, new_email: Option<String>) {
        self.email = new_email;
    }
    
    /// 属性访问器
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }
    
    #[getter]
    fn age(&self) -> u32 {
        self.age
    }
    
    #[getter]
    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    
    /// 静态方法
    #[staticmethod]
    fn create_anonymous() -> Self {
        Person {
            name: "Anonymous".to_string(),
            age: 0,
            email: None,
        }
    }
    
    /// 类方法
    #[classmethod]
    fn from_string(_cls: &PyType, person_str: &str) -> PyResult<Self> {
        let parts: Vec<&str> = person_str.split(',').collect();
        if parts.len() >= 2 {
            let name = parts[0].trim().to_string();
            let age = parts[1].trim().parse::<u32>()
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid age"))?;
            let email = if parts.len() > 2 {
                Some(parts[2].trim().to_string())
            } else {
                None
            };
            Ok(Person { name, age, email })
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid format"))
        }
    }
}

#[pymodule]
fn my_classes_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<Person>()?;
    Ok(())
}
```

## 错误处理

### 1. 自定义异常

```rust
use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;

// 创建自定义异常
create_exception!(my_module, CustomError, PyException);

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(CustomError::new_err("Division by zero"))
    } else {
        Ok(a / b)
    }
}

#[pyfunction]
fn validate_age(age: i32) -> PyResult<String> {
    if age < 0 {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Age cannot be negative"
        ))
    } else if age > 150 {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Age seems unrealistic"
        ))
    } else {
        Ok(format!("Age {} is valid", age))
    }
}

#[pyfunction]
fn parse_number(s: &str) -> PyResult<i32> {
    s.parse::<i32>()
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Cannot parse '{}' as number", s)
        ))
}

#[pymodule]
fn error_handling_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("CustomError", py.get_type::<CustomError>())?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(validate_age, m)?)?;
    m.add_function(wrap_pyfunction!(parse_number, m)?)?;
    Ok(())
}
```

### 2. 结果处理

```rust
use pyo3::prelude::*;
use std::fs;

#[pyfunction]
fn read_file(path: &str) -> PyResult<String> {
    fs::read_to_string(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Failed to read file '{}': {}", path, e)
        ))
}

#[pyfunction]
fn write_file(path: &str, content: &str) -> PyResult<()> {
    fs::write(path, content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Failed to write file '{}': {}", path, e)
        ))
}

#[pyfunction]
fn safe_divide(a: f64, b: f64) -> PyResult<Option<f64>> {
    if b == 0.0 {
        Ok(None)
    } else {
        Ok(Some(a / b))
    }
}
```

## 高级功能

### 1. 使用 Python 解释器

```rust
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};

#[pyfunction]
fn execute_python_code(code: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        py.run(code, None, Some(locals))?;
        
        // 获取变量
        if let Some(result) = locals.get_item("result")? {
            Ok(result.to_string())
        } else {
            Ok("No result variable found".to_string())
        }
    })
}

#[pyfunction]
fn call_python_function(py: Python) -> PyResult<i32> {
    let sys = py.import("sys")?;
    let version = sys.getattr("version_info")?;
    let major: i32 = version.getattr("major")?.extract()?;
    Ok(major)
}

#[pyfunction]
fn use_python_modules(py: Python) -> PyResult<()> {
    // 使用标准库模块
    let datetime = py.import("datetime")?;
    let now = datetime.getattr("datetime")?.call_method0("now")?;
    println!("Current time: {}", now);
    
    // 使用数学模块
    let math = py.import("math")?;
    let pi: f64 = math.getattr("pi")?.extract()?;
    println!("Pi value: {}", pi);
    
    let sqrt_result: f64 = math.call_method1("sqrt", (16.0,))?.extract()?;
    println!("Square root of 16: {}", sqrt_result);
    
    Ok(())
}
```

### 2. 回调函数

```rust
use pyo3::prelude::*;
use pyo3::types::PyFunction;

#[pyfunction]
fn apply_callback(numbers: Vec<i32>, callback: &PyFunction) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        let mut results = Vec::new();
        for num in numbers {
            let result = callback.call1((num,))?;
            results.push(result.to_object(py));
        }
        Ok(results)
    })
}

#[pyfunction]
fn filter_with_callback(numbers: Vec<i32>, predicate: &PyFunction) -> PyResult<Vec<i32>> {
    Python::with_gil(|py| {
        let mut filtered = Vec::new();
        for num in numbers {
            let result: bool = predicate.call1((num,))?.extract()?;
            if result {
                filtered.push(num);
            }
        }
        Ok(filtered)
    })
}

#[pyclass]
struct EventHandler {
    callbacks: Vec<PyObject>,
}

#[pymethods]
impl EventHandler {
    #[new]
    fn new() -> Self {
        EventHandler {
            callbacks: Vec::new(),
        }
    }
    
    fn add_callback(&mut self, callback: PyObject) {
        self.callbacks.push(callback);
    }
    
    fn trigger_event(&self, event_data: &str) -> PyResult<()> {
        Python::with_gil(|py| {
            for callback in &self.callbacks {
                callback.call1(py, (event_data,))?;
            }
            Ok(())
        })
    }
}
```

### 3. 多线程和 GIL

```rust
use pyo3::prelude::*;
use std::thread;
use std::time::Duration;

#[pyfunction]
fn cpu_intensive_task(n: usize) -> PyResult<Vec<u64>> {
    // 释放 GIL 进行计算密集型任务
    Python::with_gil(|py| {
        py.allow_threads(|| {
            let mut results = Vec::new();
            for i in 0..n {
                // 模拟计算密集型任务
                let mut sum = 0u64;
                for j in 0..1000 {
                    sum += (i * j) as u64;
                }
                results.push(sum);
                
                // 模拟一些工作
                thread::sleep(Duration::from_millis(1));
            }
            results
        })
    })
}

#[pyfunction]
fn parallel_computation(numbers: Vec<i32>) -> PyResult<Vec<i32>> {
    Python::with_gil(|py| {
        py.allow_threads(|| {
            use std::sync::mpsc;
            use std::sync::Arc;
            
            let numbers = Arc::new(numbers);
            let (tx, rx) = mpsc::channel();
            let thread_count = 4;
            let chunk_size = numbers.len() / thread_count;
            
            for i in 0..thread_count {
                let tx = tx.clone();
                let numbers = Arc::clone(&numbers);
                
                thread::spawn(move || {
                    let start = i * chunk_size;
                    let end = if i == thread_count - 1 {
                        numbers.len()
                    } else {
                        start + chunk_size
                    };
                    
                    let mut results = Vec::new();
                    for j in start..end {
                        // 模拟计算密集型操作
                        results.push(numbers[j] * numbers[j]);
                    }
                    
                    tx.send((start, results)).unwrap();
                });
            }
            
            drop(tx);
            let mut final_results = vec![0; numbers.len()];
            
            for (start, results) in rx {
                for (i, result) in results.into_iter().enumerate() {
                    final_results[start + i] = result;
                }
            }
            
            final_results
        })
    })
}
```

## 实际应用示例

### 1. 数据处理模块

```rust
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;

#[pyclass]
struct DataProcessor {
    data: Vec<f64>,
    cache: HashMap<String, f64>,
}

#[pymethods]
impl DataProcessor {
    #[new]
    fn new(data: Vec<f64>) -> Self {
        DataProcessor {
            data,
            cache: HashMap::new(),
        }
    }
    
    fn mean(&mut self) -> f64 {
        if let Some(&cached) = self.cache.get("mean") {
            return cached;
        }
        
        let sum: f64 = self.data.iter().sum();
        let mean = sum / self.data.len() as f64;
        self.cache.insert("mean".to_string(), mean);
        mean
    }
    
    fn std_dev(&mut self) -> f64 {
        if let Some(&cached) = self.cache.get("std_dev") {
            return cached;
        }
        
        let mean = self.mean();
        let variance: f64 = self.data
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / self.data.len() as f64;
        let std_dev = variance.sqrt();
        self.cache.insert("std_dev".to_string(), std_dev);
        std_dev
    }
    
    fn normalize(&self) -> Vec<f64> {
        let mean = self.data.iter().sum::<f64>() / self.data.len() as f64;
        let std_dev = {
            let variance: f64 = self.data
                .iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / self.data.len() as f64;
            variance.sqrt()
        };
        
        self.data
            .iter()
            .map(|x| (x - mean) / std_dev)
            .collect()
    }
    
    fn percentile(&self, p: f64) -> PyResult<f64> {
        if p < 0.0 || p > 100.0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Percentile must be between 0 and 100"
            ));
        }
        
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = (p / 100.0) * (sorted_data.len() - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        
        if lower == upper {
            Ok(sorted_data[lower])
        } else {
            let weight = index - lower as f64;
            Ok(sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight)
        }
    }
    
    fn filter_outliers(&self, threshold: f64) -> Vec<f64> {
        let mean = self.data.iter().sum::<f64>() / self.data.len() as f64;
        let std_dev = {
            let variance: f64 = self.data
                .iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / self.data.len() as f64;
            variance.sqrt()
        };
        
        self.data
            .iter()
            .filter(|&&x| (x - mean).abs() <= threshold * std_dev)
            .copied()
            .collect()
    }
    
    fn summary(&mut self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();
        summary.insert("count".to_string(), self.data.len() as f64);
        summary.insert("mean".to_string(), self.mean());
        summary.insert("std_dev".to_string(), self.std_dev());
        summary.insert("min".to_string(), self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
        summary.insert("max".to_string(), self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)));
        summary
    }
}
```

### 2. 文件处理模块

```rust
use pyo3::prelude::*;
use std::fs;
use std::path::Path;
use std::io::Read;

#[pyclass]
struct FileProcessor {
    base_path: String,
}

#[pymethods]
impl FileProcessor {
    #[new]
    fn new(base_path: String) -> Self {
        FileProcessor { base_path }
    }
    
    fn read_text_file(&self, filename: &str) -> PyResult<String> {
        let path = Path::new(&self.base_path).join(filename);
        fs::read_to_string(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to read file: {}", e)
            ))
    }
    
    fn write_text_file(&self, filename: &str, content: &str) -> PyResult<()> {
        let path = Path::new(&self.base_path).join(filename);
        fs::write(&path, content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to write file: {}", e)
            ))
    }
    
    fn list_files(&self) -> PyResult<Vec<String>> {
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to read directory: {}", e)
            ))?;
        
        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to read entry: {}", e)
            ))?;
            
            if entry.file_type().map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to get file type: {}", e)
            ))?.is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }
        
        Ok(files)
    }
    
    fn file_size(&self, filename: &str) -> PyResult<u64> {
        let path = Path::new(&self.base_path).join(filename);
        let metadata = fs::metadata(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Failed to get file metadata: {}", e)
            ))?;
        Ok(metadata.len())
    }
    
    fn file_exists(&self, filename: &str) -> bool {
        let path = Path::new(&self.base_path).join(filename);
        path.exists()
    }
}
```

### 3. 网络请求模块

```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass]
struct HttpClient {
    base_url: String,
    headers: HashMap<String, String>,
}

#[pymethods]
impl HttpClient {
    #[new]
    fn new(base_url: String) -> Self {
        HttpClient {
            base_url,
            headers: HashMap::new(),
        }
    }
    
    fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
    
    fn set_auth_token(&mut self, token: String) {
        self.headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    }
    
    // 注意：这是一个简化的示例，实际应用中需要使用 reqwest 等库
    fn get(&self, endpoint: &str) -> PyResult<String> {
        // 这里应该实现实际的 HTTP 请求
        // 为了示例简化，返回模拟响应
        Ok(format!("GET request to {}/{}", self.base_url, endpoint))
    }
    
    fn post(&self, endpoint: &str, data: &str) -> PyResult<String> {
        // 这里应该实现实际的 HTTP POST 请求
        Ok(format!("POST request to {}/{} with data: {}", self.base_url, endpoint, data))
    }
}
```

## 构建和分发

### 1. 使用 maturin 构建

```bash
# 初始化项目
maturin new my_python_extension

# 开发构建
maturin develop

# 构建 wheel
maturin build

# 发布到 PyPI
maturin publish
```

### 2. pyproject.toml 配置

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "my_python_extension"
version = "0.1.0"
description = "My Python extension written in Rust"
authors = [{name = "Your Name", email = "your@email.com"}]
requires-python = ">=3.8"
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
]

[tool.maturin]
features = ["pyo3/extension-module"]
```

### 3. 测试模块

```python
# test_module.py
import my_rust_module

def test_basic_functions():
    # 测试基本函数
    assert my_rust_module.add(2, 3) == 5
    assert my_rust_module.factorial(5) == 120
    
    # 测试字符串处理
    result = my_rust_module.process_string("hello world")
    assert result == "HELLO_WORLD"
    
    # 测试列表处理
    numbers = [1, 2, 3, 4, 5]
    assert my_rust_module.sum_list(numbers) == 15
    
    # 测试可选参数
    assert my_rust_module.greet("Alice") == "Hello, Alice!"
    assert my_rust_module.greet("Bob", "Hi") == "Hi, Bob!"

def test_counter_class():
    # 测试计数器类
    counter = my_rust_module.Counter()
    assert counter.get_value() == 0
    
    counter.increment()
    assert counter.get_value() == 1
    
    counter.decrement()
    assert counter.get_value() == 0
    
    counter.value = 10
    assert counter.value == 10

if __name__ == "__main__":
    test_basic_functions()
    test_counter_class()
    print("All tests passed!")
```

## 性能优化

### 1. 避免不必要的转换

```rust
use pyo3::prelude::*;
use pyo3::types::PyList;

// 低效的做法
#[pyfunction]
fn inefficient_sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

// 高效的做法：直接使用 PyList
#[pyfunction]
fn efficient_sum(numbers: &PyList) -> PyResult<i32> {
    let mut sum = 0;
    for item in numbers.iter() {
        sum += item.extract::<i32>()?;
    }
    Ok(sum)
}
```

### 2. 批量处理

```rust
use pyo3::prelude::*;

#[pyfunction]
fn batch_process(data: Vec<f64>, batch_size: usize) -> Vec<f64> {
    Python::with_gil(|py| {
        py.allow_threads(|| {
            data.chunks(batch_size)
                .flat_map(|chunk| {
                    // 处理每个批次
                    chunk.iter().map(|x| x * 2.0).collect::<Vec<_>>()
                })
                .collect()
        })
    })
}
```

### 3. 内存管理

```rust
use pyo3::prelude::*;

#[pyfunction]
fn memory_efficient_processing(data: &PyList) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        // 使用迭代器避免创建中间集合
        let result: PyResult<Vec<_>> = data
            .iter()
            .map(|item| {
                let value: f64 = item.extract()?;
                Ok(value * 2.0)
            })
            .collect();
        
        let processed = result?;
        Ok(PyList::new(py, processed).to_object(py))
    })
}
```

## 最佳实践

1. **错误处理**: 始终使用 `PyResult` 处理可能的错误
2. **GIL 管理**: 对于 CPU 密集型任务，使用 `py.allow_threads()`
3. **类型转换**: 避免不必要的 Python 对象转换
4. **内存安全**: 利用 Rust 的所有权系统确保内存安全
5. **文档编写**: 为导出的函数和类添加文档字符串

## 常见问题

### 1. GIL 相关问题

```rust
use pyo3::prelude::*;

// 错误：在没有 GIL 的情况下使用 Python 对象
fn wrong_approach() {
    let result = Python::with_gil(|py| {
        py.allow_threads(|| {
            // 这里不能使用 Python 对象
            // py.import("sys") // 这会导致错误
        })
    });
}

// 正确：在适当的作用域中使用 Python 对象
fn correct_approach() {
    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();
        
        py.allow_threads(|| {
            // 在这里进行 CPU 密集型计算
            // 不使用 Python 对象
        });
    });
}
```

### 2. 类型转换问题

```rust
use pyo3::prelude::*;

#[pyfunction]
fn handle_optional_types(value: Option<i32>) -> String {
    match value {
        Some(v) => format!("Value: {}", v),
        None => "No value".to_string(),
    }
}

#[pyfunction]
fn handle_complex_types(data: PyObject) -> PyResult<String> {
    Python::with_gil(|py| {
        // 检查类型
        if let Ok(s) = data.extract::<String>(py) {
            Ok(format!("String: {}", s))
        } else if let Ok(i) = data.extract::<i32>(py) {
            Ok(format!("Integer: {}", i))
        } else {
            Ok("Unknown type".to_string())
        }
    })
}
```

## 总结

PyO3 提供了一个强大而安全的方式来连接 Rust 和 Python，允许开发者：

1. **扩展 Python**: 使用 Rust 编写高性能的 Python 扩展模块
2. **嵌入 Python**: 在 Rust 应用中嵌入 Python 解释器
3. **类型安全**: 利用 Rust 的类型系统确保内存安全
4. **高性能**: 通过 Rust 的零成本抽象获得接近原生的性能

主要优势：
- 内存安全和线程安全
- 优秀的性能表现
- 丰富的 Python 生态系统访问
- 简洁的 API 设计
- 良好的文档和社区支持

更多详细信息请参考：
- [PyO3 官方文档](https://pyo3.rs/)
- [PyO3 用户指南](https://pyo3.rs/latest/)
- [GitHub 仓库](https://github.com/pyo3/pyo3)
- [maturin 工具](https://github.com/PyO3/maturin)