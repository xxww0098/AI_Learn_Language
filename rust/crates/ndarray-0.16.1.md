# ndarray 0.16.1 中文使用教程

## 概述

ndarray 是 Rust 中功能强大的 N 维数组库，为通用元素和数值计算提供了高效的数组实现。它提供了轻量级的数组视图和切片功能，支持分块和分割操作，是 Rust 生态系统中科学计算的基础库。

**版本**: 0.16.1
**许可证**: MIT OR Apache-2.0
**仓库**: https://github.com/rust-ndarray/ndarray
**文档**: https://docs.rs/ndarray/

## 主要特性

- 🔢 **N 维数组**: 支持任意维度的数组
- ⚡ **高性能**: 零成本抽象，接近 C 语言性能
- 🔍 **灵活视图**: 轻量级数组视图和切片
- 🧮 **数值计算**: 丰富的数学运算和算法
- 🔗 **生态集成**: 与 Rust 科学计算生态系统深度集成

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ndarray = "0.16.1"

# 可选特性
[dependencies.ndarray]
version = "0.16.1"
features = ["serde", "blas", "approx"]
```

## 基本概念

### 1. 数组类型

```rust
use ndarray::{Array, Array1, Array2, Array3, ArrayD, ArrayView, Dim};

// 1D 数组
let arr1: Array1<f64> = Array1::zeros(5);

// 2D 数组
let arr2: Array2<f64> = Array2::zeros((3, 4));

// 3D 数组
let arr3: Array3<f64> = Array3::zeros((2, 3, 4));

// 动态维度数组
let arr_dyn: ArrayD<f64> = ArrayD::zeros(vec![2, 3, 4]);
```

### 2. 数组创建

```rust
use ndarray::{Array, Array1, Array2, array, arr1, arr2};

// 从向量创建
let arr1 = Array1::from_vec(vec![1, 2, 3, 4, 5]);

// 使用宏创建
let arr2 = array![1, 2, 3, 4, 5];
let arr2d = array![[1, 2, 3], [4, 5, 6]];

// 零数组
let zeros = Array2::<f64>::zeros((3, 4));

// 单位数组
let ones = Array2::<f64>::ones((3, 4));

// 单位矩阵
let eye = Array2::<f64>::eye(3);

// 使用函数创建
let arr = Array2::from_shape_fn((3, 4), |(i, j)| i * 4 + j);

// 线性空间
let linspace = Array1::linspace(0.0, 1.0, 11);

// 随机数组（需要 rand 特性）
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
let random = Array2::random((3, 4), Uniform::new(0.0, 1.0));
```

## 基本操作

### 1. 索引和切片

```rust
use ndarray::{Array2, s};

let mut arr = Array2::from_shape_vec((3, 4), (0..12).collect()).unwrap();

// 单个元素访问
let element = arr[[1, 2]];
arr[[1, 2]] = 100;

// 切片
let slice = arr.slice(s![1..3, 1..3]);
let mut slice_mut = arr.slice_mut(s![1..3, 1..3]);

// 行和列
let row = arr.row(1);
let col = arr.column(2);
let mut row_mut = arr.row_mut(1);
let mut col_mut = arr.column_mut(2);

// 多维切片
let subarray = arr.slice(s![.., 1..3]);
let subarray2 = arr.slice(s![1, ..]);
```

### 2. 形状操作

```rust
use ndarray::Array2;

let arr = Array2::from_shape_vec((2, 6), (0..12).collect()).unwrap();

// 重塑
let reshaped = arr.into_shape((3, 4)).unwrap();

// 转置
let transposed = arr.t();

// 广播
let arr1 = Array2::ones((3, 1));
let arr2 = Array2::ones((1, 4));
let broadcast = &arr1 + &arr2; // 结果是 (3, 4)

// 维度交换
let mut arr = Array2::from_shape_vec((2, 3), (0..6).collect()).unwrap();
arr.swap_axes(0, 1);
```

### 3. 迭代

```rust
use ndarray::Array2;

let arr = Array2::from_shape_vec((2, 3), (0..6).collect()).unwrap();

// 元素迭代
for &elem in arr.iter() {
    println!("{}", elem);
}

// 带索引迭代
for ((i, j), &elem) in arr.indexed_iter() {
    println!("arr[{}, {}] = {}", i, j, elem);
}

// 行迭代
for row in arr.rows() {
    println!("行: {:?}", row);
}

// 列迭代
for col in arr.columns() {
    println!("列: {:?}", col);
}

// 可变迭代
let mut arr = Array2::from_shape_vec((2, 3), (0..6).collect()).unwrap();
for elem in arr.iter_mut() {
    *elem *= 2;
}
```

## 数学运算

### 1. 基本运算

```rust
use ndarray::Array2;

let a = Array2::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
let b = Array2::from_shape_vec((2, 3), vec![6, 5, 4, 3, 2, 1]).unwrap();

// 加法
let c = &a + &b;

// 减法
let d = &a - &b;

// 元素乘法
let e = &a * &b;

// 元素除法
let f = &a / &b;

// 标量运算
let g = &a + 10;
let h = &a * 2.0;
```

### 2. 线性代数

```rust
use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

// 矩阵乘法
let a = Array2::from_shape_vec((2, 3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
let b = Array2::from_shape_vec((3, 2), vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
let c = a.dot(&b);

// 向量点积
let v1 = Array1::from_vec(vec![1.0, 2.0, 3.0]);
let v2 = Array1::from_vec(vec![4.0, 5.0, 6.0]);
let dot_product = v1.dot(&v2);

// 矩阵分解（需要 ndarray-linalg）
use ndarray_linalg::{Eig, SVD};

let matrix = Array2::from_shape_vec((3, 3), vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0,
]).unwrap();

// 特征值分解
let (eigenvalues, eigenvectors) = matrix.eig().unwrap();

// 奇异值分解
let (u, s, vt) = matrix.svd(true, true).unwrap();
```

### 3. 统计函数

```rust
use ndarray::Array2;

let arr = Array2::from_shape_vec((3, 4), (1..=12).collect()).unwrap();

// 求和
let sum = arr.sum();
let sum_axis0 = arr.sum_axis(ndarray::Axis(0));
let sum_axis1 = arr.sum_axis(ndarray::Axis(1));

// 均值
let mean = arr.mean().unwrap();
let mean_axis0 = arr.mean_axis(ndarray::Axis(0)).unwrap();

// 最大值和最小值
let max = arr.fold(std::i32::MIN, |acc, &x| acc.max(x));
let min = arr.fold(std::i32::MAX, |acc, &x| acc.min(x));

// 标准差和方差
let std_dev = arr.std_axis(ndarray::Axis(0), 0.0);
let variance = arr.var_axis(ndarray::Axis(0), 0.0);
```

## 高级功能

### 1. 并行计算

```rust
use ndarray::Array2;
use ndarray::parallel::prelude::*;

let mut arr = Array2::from_shape_vec((1000, 1000), (0..1000000).collect()).unwrap();

// 并行映射
arr.par_map_inplace(|x| *x = x.pow(2));

// 并行迭代
let sum: i32 = arr.par_iter().sum();

// 并行逐行处理
arr.axis_iter_mut(ndarray::Axis(0))
    .into_par_iter()
    .for_each(|mut row| {
        row.mapv_inplace(|x| x * 2);
    });
```

### 2. 自定义数据类型

```rust
use ndarray::Array2;

#[derive(Clone, Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

// 使用自定义类型
let mut arr = Array2::from_elem((3, 3), Complex::new(1.0, 0.0));
arr[[1, 1]] = Complex::new(2.0, 3.0);
```

### 3. 窗口操作

```rust
use ndarray::{Array2, s};

let arr = Array2::from_shape_vec((5, 5), (0..25).collect()).unwrap();

// 滑动窗口
fn sliding_window_sum(arr: &Array2<i32>, window_size: usize) -> Array2<i32> {
    let (rows, cols) = arr.dim();
    let mut result = Array2::zeros((rows - window_size + 1, cols - window_size + 1));
    
    for i in 0..result.nrows() {
        for j in 0..result.ncols() {
            let window = arr.slice(s![i..i+window_size, j..j+window_size]);
            result[[i, j]] = window.sum();
        }
    }
    
    result
}

let windowed = sliding_window_sum(&arr, 3);
```

## 性能优化

### 1. 内存布局

```rust
use ndarray::{Array2, Order};

// 行优先（C 风格）
let row_major = Array2::zeros((1000, 1000));

// 列优先（Fortran 风格）
let col_major = Array2::zeros((1000, 1000).f());

// 手动指定内存布局
let arr = Array2::from_shape_vec((100, 100), (0..10000).collect()).unwrap();
let fortran_arr = arr.reversed_axes(); // 转换为 Fortran 布局
```

### 2. 就地操作

```rust
use ndarray::Array2;

let mut arr1 = Array2::from_shape_vec((100, 100), (0..10000).collect()).unwrap();
let arr2 = Array2::from_shape_vec((100, 100), (0..10000).collect()).unwrap();

// 就地加法
arr1 += &arr2;

// 就地映射
arr1.mapv_inplace(|x| x * 2);

// 就地应用函数
arr1.par_map_inplace(|x| *x = x.pow(2));
```

### 3. 视图优化

```rust
use ndarray::{Array2, ArrayView2};

fn process_subarray(view: ArrayView2<f64>) -> f64 {
    view.sum()
}

let arr = Array2::from_shape_vec((1000, 1000), (0..1000000).map(|x| x as f64).collect()).unwrap();

// 使用视图避免复制
let result = process_subarray(arr.view());
```

## 实际应用示例

### 1. 图像处理

```rust
use ndarray::{Array3, Array2, s};

struct Image {
    data: Array3<u8>, // (height, width, channels)
}

impl Image {
    fn new(height: usize, width: usize, channels: usize) -> Self {
        Image {
            data: Array3::zeros((height, width, channels)),
        }
    }

    fn grayscale(&self) -> Array2<u8> {
        let (height, width, _) = self.data.dim();
        let mut gray = Array2::zeros((height, width));
        
        for i in 0..height {
            for j in 0..width {
                let r = self.data[[i, j, 0]] as f32;
                let g = self.data[[i, j, 1]] as f32;
                let b = self.data[[i, j, 2]] as f32;
                gray[[i, j]] = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            }
        }
        
        gray
    }

    fn blur(&self, kernel_size: usize) -> Array3<u8> {
        let (height, width, channels) = self.data.dim();
        let mut blurred = self.data.clone();
        let half_kernel = kernel_size / 2;
        
        for c in 0..channels {
            for i in half_kernel..height - half_kernel {
                for j in half_kernel..width - half_kernel {
                    let mut sum = 0.0;
                    let mut count = 0;
                    
                    for ki in 0..kernel_size {
                        for kj in 0..kernel_size {
                            sum += self.data[[i + ki - half_kernel, j + kj - half_kernel, c]] as f32;
                            count += 1;
                        }
                    }
                    
                    blurred[[i, j, c]] = (sum / count as f32) as u8;
                }
            }
        }
        
        blurred
    }
}
```

### 2. 数据分析

```rust
use ndarray::{Array1, Array2};

struct Dataset {
    data: Array2<f64>,
    labels: Array1<usize>,
}

impl Dataset {
    fn new(data: Array2<f64>, labels: Array1<usize>) -> Self {
        Dataset { data, labels }
    }

    fn normalize(&mut self) {
        let (n_samples, n_features) = self.data.dim();
        
        for feature in 0..n_features {
            let mut col = self.data.column_mut(feature);
            let mean = col.mean().unwrap();
            let std = col.std(0.0);
            
            col.mapv_inplace(|x| (x - mean) / std);
        }
    }

    fn train_test_split(&self, test_ratio: f64) -> (Dataset, Dataset) {
        let n_samples = self.data.nrows();
        let test_size = (n_samples as f64 * test_ratio) as usize;
        let train_size = n_samples - test_size;
        
        let train_data = self.data.slice(s![..train_size, ..]).to_owned();
        let train_labels = self.labels.slice(s![..train_size]).to_owned();
        
        let test_data = self.data.slice(s![train_size.., ..]).to_owned();
        let test_labels = self.labels.slice(s![train_size..]).to_owned();
        
        (
            Dataset::new(train_data, train_labels),
            Dataset::new(test_data, test_labels),
        )
    }
}
```

### 3. 科学计算

```rust
use ndarray::{Array1, Array2};
use std::f64::consts::PI;

// 数值积分
fn integrate_simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let x = Array1::linspace(a, b, n + 1);
    let mut y = Array1::zeros(n + 1);
    
    for i in 0..=n {
        y[i] = f(x[i]);
    }
    
    let mut integral = y[0] + y[n];
    for i in 1..n {
        integral += if i % 2 == 0 { 2.0 } else { 4.0 } * y[i];
    }
    
    integral * h / 3.0
}

// 数值微分
fn differentiate_central(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

// 解线性方程组
fn solve_linear_system(a: &Array2<f64>, b: &Array1<f64>) -> Array1<f64> {
    // 使用 Gaussian elimination 或其他方法
    // 这里是简化示例
    a.dot(b) // 实际需要更复杂的实现
}
```

## 与其他库的集成

### 1. 与 serde 集成

```rust
use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(with = "ndarray_serde")]
    matrix: Array2<f64>,
}

// 序列化
let data = Data {
    matrix: Array2::from_shape_vec((2, 3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap(),
};
let json = serde_json::to_string(&data).unwrap();

// 反序列化
let deserialized: Data = serde_json::from_str(&json).unwrap();
```

### 2. 与 BLAS 集成

```rust
use ndarray::Array2;
use ndarray_linalg::*;

// 使用 BLAS 进行矩阵乘法
let a = Array2::from_shape_vec((100, 100), (0..10000).map(|x| x as f64).collect()).unwrap();
let b = Array2::from_shape_vec((100, 100), (0..10000).map(|x| x as f64).collect()).unwrap();

// 高性能矩阵乘法
let c = a.dot(&b);
```

## 最佳实践

1. **选择合适的数据类型**: 根据精度需求选择 f32 或 f64
2. **避免不必要的复制**: 使用视图和引用
3. **就地操作**: 尽可能使用就地操作减少内存分配
4. **并行计算**: 对于大型数组使用并行操作
5. **内存布局**: 根据访问模式选择合适的内存布局

## 常见问题

### 1. 形状不匹配

```rust
use ndarray::Array2;

let a = Array2::ones((3, 4));
let b = Array2::ones((4, 3));

// 错误：形状不匹配
// let c = &a + &b;

// 正确：使用广播或重塑
let c = &a + &b.t();
```

### 2. 性能问题

- 使用就地操作
- 选择合适的内存布局
- 避免频繁的内存分配
- 使用并行计算

### 3. 内存使用

- 及时释放不需要的数组
- 使用视图避免复制
- 考虑使用稀疏数组对于大型稀疏数据

## 总结

ndarray 是 Rust 生态系统中最重要的数值计算库之一，提供了强大的 N 维数组功能。通过合理的设计和优化，可以实现高性能的科学计算和数据分析应用。

更多详细信息请参考：
- [ndarray 官方文档](https://docs.rs/ndarray/)
- [ndarray-linalg 线性代数扩展](https://docs.rs/ndarray-linalg/)
- [GitHub 仓库](https://github.com/rust-ndarray/ndarray)