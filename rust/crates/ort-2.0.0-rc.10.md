# ort 2.0.0-rc.10 中文使用教程

## 概述

ort 是 ONNX Runtime 的安全 Rust 封装库，用于优化和加速机器学习模型的推理和训练。它提供了高性能的机器学习推理能力，支持多种硬件平台。

**版本**: 2.0.0-rc.10
**许可证**: MIT OR Apache-2.0
**仓库**: https://github.com/pykeio/ort
**主页**: https://ort.pyke.io/

## 主要特性

- 🚀 **高性能**: 基于 ONNX Runtime 1.22，优化推理速度
- 🔒 **类型安全**: 提供完全类型安全的 Rust API
- 🎯 **多平台**: 支持 CPU、GPU、各种加速器
- 🛠️ **易用性**: 简洁的 API 设计，易于集成
- 📦 **轻量级**: 最小化依赖，快速构建

## 安装

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
ort = "2.0.0-rc.10"

# 可选：添加特定后端支持
[dependencies.ort]
version = "2.0.0-rc.10"
features = ["cuda", "tensorrt", "openvino"]
```

### 系统要求

- ONNX Runtime 库（自动下载或手动安装）
- 支持的操作系统：Windows、Linux、macOS
- 可选：CUDA、TensorRT、OpenVINO 等加速库

## 基本用法

### 1. 环境初始化

```rust
use ort::{Environment, ExecutionProvider, GraphOptimizationLevel};

fn main() -> ort::Result<()> {
    // 初始化 ONNX Runtime 环境
    let environment = Environment::builder()
        .with_name("MyApp")
        .with_log_level(ort::LoggingLevel::Warning)
        .build()?;

    // 设置执行提供程序
    let execution_providers = vec![
        ExecutionProvider::CUDA(Default::default()),
        ExecutionProvider::CPU(Default::default()),
    ];

    Ok(())
}
```

### 2. 加载模型

```rust
use ort::{Session, SessionBuilder};

fn load_model() -> ort::Result<Session> {
    let session = SessionBuilder::new()?
        .with_optimization_level(GraphOptimizationLevel::All)?
        .with_intra_threads(4)?
        .commit_from_file("model.onnx")?;

    Ok(session)
}
```

### 3. 基本推理

```rust
use ort::{Session, Value, inputs, ArrayExtensions};
use ndarray::Array;

fn run_inference(session: &Session) -> ort::Result<()> {
    // 准备输入数据
    let input_data = Array::from_shape_vec(
        (1, 3, 224, 224),
        vec![0.0f32; 1 * 3 * 224 * 224]
    ).unwrap();

    // 创建输入张量
    let input_tensor = Value::from_array(input_data)?;

    // 执行推理
    let outputs = session.run(inputs![input_tensor]?)?;

    // 获取输出
    let output: &Value = &outputs[0];
    let output_array = output.try_extract_tensor::<f32>()?;

    println!("输出形状: {:?}", output_array.shape());
    println!("输出数据: {:?}", output_array.slice(s![0, 0..5]));

    Ok(())
}
```

## 高级功能

### 1. 图像分类示例

```rust
use ort::{Session, Value, inputs};
use ndarray::{Array4, s};
use image::ImageBuffer;

struct ImageClassifier {
    session: Session,
    input_shape: (usize, usize, usize, usize), // (batch, channels, height, width)
}

impl ImageClassifier {
    fn new(model_path: &str) -> ort::Result<Self> {
        let session = SessionBuilder::new()?
            .with_optimization_level(GraphOptimizationLevel::All)?
            .commit_from_file(model_path)?;

        Ok(Self {
            session,
            input_shape: (1, 3, 224, 224),
        })
    }

    fn preprocess_image(&self, image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Array4<f32> {
        let (width, height) = image.dimensions();
        let mut input = Array4::zeros(self.input_shape);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;

                input[[0, 0, y as usize, x as usize]] = (r - 0.485) / 0.229;
                input[[0, 1, y as usize, x as usize]] = (g - 0.456) / 0.224;
                input[[0, 2, y as usize, x as usize]] = (b - 0.406) / 0.225;
            }
        }

        input
    }

    fn predict(&self, image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> ort::Result<Vec<f32>> {
        let input_data = self.preprocess_image(image);
        let input_tensor = Value::from_array(input_data)?;

        let outputs = self.session.run(inputs![input_tensor]?)?;
        let output = outputs[0].try_extract_tensor::<f32>()?;

        Ok(output.slice(s![0, ..]).to_vec())
    }
}

fn main() -> ort::Result<()> {
    let classifier = ImageClassifier::new("resnet50.onnx")?;
    
    let img = image::open("test_image.jpg")
        .unwrap()
        .to_rgb8();
    
    let predictions = classifier.predict(&img)?;
    
    // 找到最高概率的类别
    let max_idx = predictions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();
    
    println!("预测类别: {}, 概率: {:.4}", max_idx, predictions[max_idx]);
    
    Ok(())
}
```

### 2. 文本处理模型

```rust
use ort::{Session, Value, inputs};
use ndarray::Array2;

struct TextProcessor {
    session: Session,
    vocab_size: usize,
    max_length: usize,
}

impl TextProcessor {
    fn new(model_path: &str, vocab_size: usize, max_length: usize) -> ort::Result<Self> {
        let session = SessionBuilder::new()?
            .with_optimization_level(GraphOptimizationLevel::All)?
            .commit_from_file(model_path)?;

        Ok(Self {
            session,
            vocab_size,
            max_length,
        })
    }

    fn tokenize(&self, text: &str) -> Vec<i64> {
        // 简单的词汇表映射（实际应用中应使用专门的分词器）
        text.chars()
            .map(|c| c as i64)
            .take(self.max_length)
            .collect()
    }

    fn pad_sequence(&self, tokens: Vec<i64>) -> Array2<i64> {
        let mut padded = vec![0i64; self.max_length];
        for (i, &token) in tokens.iter().enumerate() {
            if i < self.max_length {
                padded[i] = token;
            }
        }
        Array2::from_shape_vec((1, self.max_length), padded).unwrap()
    }

    fn process_text(&self, text: &str) -> ort::Result<Vec<f32>> {
        let tokens = self.tokenize(text);
        let input_ids = self.pad_sequence(tokens);
        
        let input_tensor = Value::from_array(input_ids)?;
        let outputs = self.session.run(inputs![input_tensor]?)?;
        
        let output = outputs[0].try_extract_tensor::<f32>()?;
        Ok(output.slice(s![0, ..]).to_vec())
    }
}
```

### 3. 批量推理

```rust
use ort::{Session, Value, inputs};
use ndarray::Array4;

fn batch_inference(session: &Session, batch_data: Vec<Array4<f32>>) -> ort::Result<Vec<Vec<f32>>> {
    let mut results = Vec::new();
    
    for data in batch_data {
        let input_tensor = Value::from_array(data)?;
        let outputs = session.run(inputs![input_tensor]?)?;
        let output = outputs[0].try_extract_tensor::<f32>()?;
        results.push(output.slice(s![0, ..]).to_vec());
    }
    
    Ok(results)
}

// 异步批量推理
use tokio;

async fn async_batch_inference(
    session: &Session,
    batch_data: Vec<Array4<f32>>
) -> ort::Result<Vec<Vec<f32>>> {
    let tasks: Vec<_> = batch_data.into_iter().map(|data| {
        let session = session.clone(); // 注意：Session 需要支持 Clone
        tokio::spawn(async move {
            let input_tensor = Value::from_array(data)?;
            let outputs = session.run(inputs![input_tensor]?)?;
            let output = outputs[0].try_extract_tensor::<f32>()?;
            Ok::<Vec<f32>, ort::Error>(output.slice(s![0, ..]).to_vec())
        })
    }).collect();

    let mut results = Vec::new();
    for task in tasks {
        results.push(task.await.unwrap()?);
    }
    
    Ok(results)
}
```

## 性能优化

### 1. 执行提供程序配置

```rust
use ort::{ExecutionProvider, CUDAExecutionProvider, CPUExecutionProvider};

fn configure_providers() -> Vec<ExecutionProvider> {
    vec![
        // GPU 加速（如果可用）
        ExecutionProvider::CUDA(CUDAExecutionProvider::default()
            .with_device_id(0)
            .with_gpu_mem_limit(2 * 1024 * 1024 * 1024) // 2GB
        ),
        
        // CPU 后备
        ExecutionProvider::CPU(CPUExecutionProvider::default()
            .with_intra_op_num_threads(4)
            .with_inter_op_num_threads(2)
        ),
        
        // 其他加速器
        ExecutionProvider::TensorRT(Default::default()),
        ExecutionProvider::OpenVINO(Default::default()),
    ]
}
```

### 2. 内存管理

```rust
use ort::{AllocatorType, MemType};

fn optimize_memory_usage(session_builder: SessionBuilder) -> ort::Result<Session> {
    session_builder
        .with_memory_pattern(true)?
        .with_allocator(AllocatorType::Arena)?
        .with_memory_type(MemType::CPUInput)?
        .commit_from_file("model.onnx")
}
```

### 3. 图优化

```rust
use ort::GraphOptimizationLevel;

fn optimize_graph(session_builder: SessionBuilder) -> ort::Result<Session> {
    session_builder
        .with_optimization_level(GraphOptimizationLevel::All)?
        .with_optimized_model_file_path("optimized_model.onnx")?
        .commit_from_file("model.onnx")
}
```

## 错误处理

### 1. 自定义错误类型

```rust
use ort::Error as OrtError;

#[derive(Debug)]
pub enum MyError {
    OrtError(OrtError),
    InvalidInput(String),
    ModelNotFound,
}

impl From<OrtError> for MyError {
    fn from(error: OrtError) -> Self {
        MyError::OrtError(error)
    }
}

fn safe_inference(session: &Session, input: Array4<f32>) -> Result<Vec<f32>, MyError> {
    if input.shape()[0] != 1 {
        return Err(MyError::InvalidInput("批量大小必须为1".to_string()));
    }

    let input_tensor = Value::from_array(input)?;
    let outputs = session.run(inputs![input_tensor]?)?;
    let output = outputs[0].try_extract_tensor::<f32>()?;
    
    Ok(output.slice(s![0, ..]).to_vec())
}
```

### 2. 重试机制

```rust
use std::thread::sleep;
use std::time::Duration;

fn retry_inference<F, T>(mut f: F, max_retries: u32) -> Result<T, OrtError>
where
    F: FnMut() -> Result<T, OrtError>,
{
    let mut retries = 0;
    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err(err) => {
                if retries >= max_retries {
                    return Err(err);
                }
                retries += 1;
                sleep(Duration::from_millis(100 * retries as u64));
            }
        }
    }
}
```

## 模型转换和部署

### 1. 模型格式转换

```rust
// 从 PyTorch 模型转换
fn convert_pytorch_model() {
    // 在 Python 中执行转换
    // torch.onnx.export(model, dummy_input, "model.onnx")
}

// 模型量化
fn quantize_model() -> ort::Result<()> {
    // 使用 ONNX Runtime 量化工具
    // 这通常在 Python 中完成，然后在 Rust 中加载量化模型
    let session = SessionBuilder::new()?
        .commit_from_file("quantized_model.onnx")?;
    
    Ok(())
}
```

### 2. 模型验证

```rust
fn validate_model(model_path: &str) -> ort::Result<()> {
    let session = SessionBuilder::new()?
        .commit_from_file(model_path)?;
    
    // 检查输入/输出信息
    let input_info = session.inputs();
    let output_info = session.outputs();
    
    println!("模型输入数量: {}", input_info.len());
    println!("模型输出数量: {}", output_info.len());
    
    for (i, input) in input_info.iter().enumerate() {
        println!("输入 {}: {:?}", i, input);
    }
    
    for (i, output) in output_info.iter().enumerate() {
        println!("输出 {}: {:?}", i, output);
    }
    
    Ok(())
}
```

## 最佳实践

1. **模型预加载**: 在应用启动时预加载模型，避免运行时延迟
2. **批量处理**: 尽可能使用批量推理提高吞吐量
3. **内存管理**: 合理配置内存使用，避免内存泄漏
4. **错误处理**: 实现完善的错误处理和重试机制
5. **性能监控**: 监控推理时间和资源使用情况

## 常见问题

### 1. 模型加载失败

```rust
fn diagnose_model_loading(model_path: &str) {
    match SessionBuilder::new() {
        Ok(builder) => {
            match builder.commit_from_file(model_path) {
                Ok(_) => println!("模型加载成功"),
                Err(e) => println!("模型加载失败: {:?}", e),
            }
        }
        Err(e) => println!("创建会话构建器失败: {:?}", e),
    }
}
```

### 2. 性能优化

- 选择合适的执行提供程序
- 调整线程数量
- 启用图优化
- 使用模型量化

### 3. 内存不足

- 减少批量大小
- 启用内存模式
- 使用流式处理

## 总结

ort 提供了一个强大且高效的 ONNX Runtime Rust 接口，适用于各种机器学习推理场景。通过合理的配置和优化，可以实现高性能的 AI 应用部署。

更多详细信息请参考：
- [ort 官方文档](https://ort.pyke.io/)
- [ONNX Runtime 文档](https://onnxruntime.ai/)
- [GitHub 仓库](https://github.com/pykeio/ort)