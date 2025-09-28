# WebAssembly 2.0 + Rust 1.90 高级教程

## 📚 概述

本教程提供了 WebAssembly 2.0 + Rust 1.90 的高级学习内容，包括深度技术解析、实战项目构建、性能调优和最佳实践。

## 🎯 教程结构

### 第一部分：深度技术解析

- 内存模型深度解析
- 类型系统高级应用
- 并发编程模式
- 安全编程实践

### 第二部分：实战项目构建

- 图像处理引擎
- 数据可视化系统
- 游戏引擎开发
- 微服务架构

### 第三部分：性能调优

- 编译器优化技巧
- 运行时性能调优
- 内存使用优化
- 并发性能优化

## 🧠 第一部分：深度技术解析

### 1. 内存模型深度解析

#### 线性内存管理

```rust
/// 高级内存管理器
pub struct AdvancedMemoryManager {
    memory: Vec<u8>,
    allocations: HashMap<usize, AllocationInfo>,
    free_blocks: Vec<FreeBlock>,
    next_id: usize,
}

#[derive(Debug, Clone)]
struct AllocationInfo {
    size: usize,
    alignment: usize,
    is_active: bool,
    allocation_time: std::time::Instant,
}

#[derive(Debug, Clone)]
struct FreeBlock {
    start: usize,
    size: usize,
    next: Option<usize>,
}

impl AdvancedMemoryManager {
    /// 创建高级内存管理器
    pub fn new(initial_size: usize) -> Self {
        Self {
            memory: vec![0; initial_size],
            allocations: HashMap::new(),
            free_blocks: vec![FreeBlock {
                start: 0,
                size: initial_size,
                next: None,
            }],
            next_id: 0,
        }
    }
    
    /// 智能内存分配
    pub fn smart_allocate(&mut self, size: usize, alignment: usize) -> Result<usize, MemoryError> {
        // 寻找最佳匹配的可用块
        let best_fit = self.find_best_fit(size, alignment)?;
        
        // 分配内存
        let allocation_id = self.allocate_at(best_fit, size, alignment)?;
        
        // 更新空闲块列表
        self.update_free_blocks(best_fit, size);
        
        Ok(allocation_id)
    }
    
    /// 内存压缩
    pub fn compact_memory(&mut self) -> Result<(), MemoryError> {
        let mut compacted_memory = Vec::new();
        let mut offset = 0;
        
        // 收集所有活跃分配
        let mut active_allocations: Vec<_> = self.allocations.iter()
            .filter(|(_, info)| info.is_active)
            .collect();
        
        // 按分配时间排序
        active_allocations.sort_by_key(|(_, info)| info.allocation_time);
        
        // 重新排列内存
        for (allocation_id, info) in active_allocations {
            let old_start = *allocation_id;
            let new_start = offset;
            
            // 复制数据
            let data = &self.memory[old_start..old_start + info.size];
            compacted_memory.extend_from_slice(data);
            
            // 更新分配信息
            if let Some(allocation_info) = self.allocations.get_mut(allocation_id) {
                allocation_info.allocation_time = std::time::Instant::now();
            }
            
            offset += info.size;
        }
        
        // 更新内存
        self.memory = compacted_memory;
        self.memory.resize(offset, 0);
        
        // 重建空闲块列表
        self.rebuild_free_blocks();
        
        Ok(())
    }
    
    /// 内存使用统计
    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_size = self.memory.len();
        let allocated_size: usize = self.allocations.values()
            .filter(|info| info.is_active)
            .map(|info| info.size)
            .sum();
        
        let fragmentation = if total_size > 0 {
            1.0 - (allocated_size as f64 / total_size as f64)
        } else {
            0.0
        };
        
        MemoryStats {
            total_size,
            allocated_size,
            free_size: total_size - allocated_size,
            fragmentation,
            allocation_count: self.allocations.len(),
        }
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub total_size: usize,
    pub allocated_size: usize,
    pub free_size: usize,
    pub fragmentation: f64,
    pub allocation_count: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("内存不足")]
    OutOfMemory,
    #[error("对齐错误")]
    AlignmentError,
    #[error("无效分配")]
    InvalidAllocation,
}
```

### 2. 类型系统高级应用

#### 高级类型转换

```rust
/// 高级类型转换器
pub struct AdvancedTypeConverter {
    conversion_cache: HashMap<(TypeId, TypeId), ConversionFunction>,
    type_registry: HashMap<String, TypeInfo>,
}

#[derive(Debug, Clone)]
struct TypeInfo {
    size: usize,
    alignment: usize,
    is_copy: bool,
    is_send: bool,
    is_sync: bool,
}

type ConversionFunction = Box<dyn Fn(&[u8]) -> Result<Vec<u8>, ConversionError> + Send + Sync>;

impl AdvancedTypeConverter {
    /// 创建高级类型转换器
    pub fn new() -> Self {
        let mut converter = Self {
            conversion_cache: HashMap::new(),
            type_registry: HashMap::new(),
        };
        
        // 注册基本类型
        converter.register_basic_types();
        converter
    }
    
    /// 注册基本类型
    fn register_basic_types(&mut self) {
        self.type_registry.insert("i32".to_string(), TypeInfo {
            size: 4,
            alignment: 4,
            is_copy: true,
            is_send: true,
            is_sync: true,
        });
        
        self.type_registry.insert("i64".to_string(), TypeInfo {
            size: 8,
            alignment: 8,
            is_copy: true,
            is_send: true,
            is_sync: true,
        });
        
        self.type_registry.insert("f32".to_string(), TypeInfo {
            size: 4,
            alignment: 4,
            is_copy: true,
            is_send: true,
            is_sync: true,
        });
        
        self.type_registry.insert("f64".to_string(), TypeInfo {
            size: 8,
            alignment: 8,
            is_copy: true,
            is_send: true,
            is_sync: true,
        });
    }
    
    /// 智能类型转换
    pub fn smart_convert(&mut self, data: &[u8], from_type: &str, to_type: &str) -> Result<Vec<u8>, ConversionError> {
        let cache_key = (TypeId::of::<String>(), TypeId::of::<String>());
        
        if let Some(converter) = self.conversion_cache.get(&cache_key) {
            return converter(data);
        }
        
        // 创建新的转换函数
        let converter = self.create_conversion_function(from_type, to_type)?;
        let result = converter(data);
        
        // 缓存转换函数
        self.conversion_cache.insert(cache_key, converter);
        
        result
    }
    
    /// 创建转换函数
    fn create_conversion_function(&self, from_type: &str, to_type: &str) -> Result<ConversionFunction, ConversionError> {
        match (from_type, to_type) {
            ("i32", "f32") => Ok(Box::new(|data| {
                if data.len() >= 4 {
                    let value = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                    Ok((value as f32).to_le_bytes().to_vec())
                } else {
                    Err(ConversionError::InvalidData)
                }
            })),
            ("f32", "i32") => Ok(Box::new(|data| {
                if data.len() >= 4 {
                    let value = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                    Ok((value as i32).to_le_bytes().to_vec())
                } else {
                    Err(ConversionError::InvalidData)
                }
            })),
            ("i32", "i64") => Ok(Box::new(|data| {
                if data.len() >= 4 {
                    let value = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                    Ok((value as i64).to_le_bytes().to_vec())
                } else {
                    Err(ConversionError::InvalidData)
                }
            })),
            _ => Err(ConversionError::UnsupportedConversion),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("不支持的类型转换")]
    UnsupportedConversion,
    #[error("无效的数据")]
    InvalidData,
    #[error("类型不匹配")]
    TypeMismatch,
}
```

## 🏗️ 第二部分：实战项目构建

### 1. 图像处理引擎

#### 核心引擎架构

```rust
/// 图像处理引擎
pub struct ImageProcessingEngine {
    processors: HashMap<String, Box<dyn ImageProcessor>>,
    memory_pool: Arc<Mutex<MemoryPool>>,
    task_scheduler: Arc<TaskScheduler>,
}

pub trait ImageProcessor: Send + Sync {
    fn process(&self, input: &ImageData, params: &ProcessingParams) -> Result<ImageData, ProcessingError>;
    fn get_name(&self) -> &str;
    fn get_supported_formats(&self) -> Vec<ImageFormat>;
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum ImageFormat {
    RGB,
    RGBA,
    Grayscale,
    YUV,
}

#[derive(Debug, Clone)]
pub struct ProcessingParams {
    pub filter_type: FilterType,
    pub intensity: f32,
    pub kernel_size: u32,
    pub custom_params: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum FilterType {
    Blur,
    Sharpen,
    EdgeDetection,
    ColorAdjustment,
    Custom(String),
}

impl ImageProcessingEngine {
    /// 创建图像处理引擎
    pub fn new() -> Self {
        let mut engine = Self {
            processors: HashMap::new(),
            memory_pool: Arc::new(Mutex::new(MemoryPool::new(1024 * 1024 * 100))), // 100MB
            task_scheduler: Arc::new(TaskScheduler::new(4)), // 4个线程
        };
        
        // 注册默认处理器
        engine.register_default_processors();
        engine
    }
    
    /// 注册处理器
    pub fn register_processor(&mut self, name: String, processor: Box<dyn ImageProcessor>) {
        self.processors.insert(name, processor);
    }
    
    /// 处理图像
    pub async fn process_image(&self, input: ImageData, params: ProcessingParams) -> Result<ImageData, ProcessingError> {
        let processor_name = match params.filter_type {
            FilterType::Blur => "blur_processor",
            FilterType::Sharpen => "sharpen_processor",
            FilterType::EdgeDetection => "edge_detection_processor",
            FilterType::ColorAdjustment => "color_adjustment_processor",
            FilterType::Custom(name) => &name,
        };
        
        if let Some(processor) = self.processors.get(processor_name) {
            processor.process(&input, &params)
        } else {
            Err(ProcessingError::UnknownProcessor)
        }
    }
    
    /// 批量处理图像
    pub async fn batch_process(&self, inputs: Vec<ImageData>, params: ProcessingParams) -> Result<Vec<ImageData>, ProcessingError> {
        let tasks: Vec<_> = inputs.into_iter()
            .map(|input| {
                let engine = self.clone();
                let params = params.clone();
                async move { engine.process_image(input, params).await }
            })
            .collect();
        
        let results = futures::future::join_all(tasks).await;
        
        // 检查是否有错误
        for result in &results {
            if let Err(e) = result {
                return Err(e.clone());
            }
        }
        
        // 提取成功的结果
        let mut processed_images = Vec::new();
        for result in results {
            processed_images.push(result?);
        }
        
        Ok(processed_images)
    }
    
    /// 注册默认处理器
    fn register_default_processors(&mut self) {
        self.processors.insert("blur_processor".to_string(), Box::new(BlurProcessor::new()));
        self.processors.insert("sharpen_processor".to_string(), Box::new(SharpenProcessor::new()));
        self.processors.insert("edge_detection_processor".to_string(), Box::new(EdgeDetectionProcessor::new()));
        self.processors.insert("color_adjustment_processor".to_string(), Box::new(ColorAdjustmentProcessor::new()));
    }
}

/// 模糊处理器
pub struct BlurProcessor {
    kernel_cache: HashMap<u32, Vec<f32>>,
}

impl BlurProcessor {
    pub fn new() -> Self {
        Self {
            kernel_cache: HashMap::new(),
        }
    }
    
    fn generate_gaussian_kernel(&mut self, size: u32) -> Vec<f32> {
        if let Some(kernel) = self.kernel_cache.get(&size) {
            return kernel.clone();
        }
        
        let mut kernel = Vec::new();
        let sigma = size as f32 / 6.0;
        let center = size as f32 / 2.0;
        
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();
                let value = (-(distance * distance) / (2.0 * sigma * sigma)).exp();
                kernel.push(value);
            }
        }
        
        // 归一化
        let sum: f32 = kernel.iter().sum();
        for value in &mut kernel {
            *value /= sum;
        }
        
        self.kernel_cache.insert(size, kernel.clone());
        kernel
    }
}

impl ImageProcessor for BlurProcessor {
    fn process(&self, input: &ImageData, params: &ProcessingParams) -> Result<ImageData, ProcessingError> {
        let kernel_size = params.kernel_size;
        let mut kernel = self.generate_gaussian_kernel(kernel_size);
        
        let mut output_data = input.data.clone();
        let half_kernel = (kernel_size / 2) as i32;
        
        for y in 0..input.height {
            for x in 0..input.width {
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;
                let mut a_sum = 0.0;
                
                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        let px = (x as i32 + kx as i32 - half_kernel).clamp(0, input.width as i32 - 1) as u32;
                        let py = (y as i32 + ky as i32 - half_kernel).clamp(0, input.height as i32 - 1) as u32;
                        
                        let pixel_idx = (py * input.width + px) as usize * 4;
                        let kernel_idx = (ky * kernel_size + kx) as usize;
                        
                        if pixel_idx + 3 < input.data.len() && kernel_idx < kernel.len() {
                            r_sum += input.data[pixel_idx] as f32 * kernel[kernel_idx];
                            g_sum += input.data[pixel_idx + 1] as f32 * kernel[kernel_idx];
                            b_sum += input.data[pixel_idx + 2] as f32 * kernel[kernel_idx];
                            a_sum += input.data[pixel_idx + 3] as f32 * kernel[kernel_idx];
                        }
                    }
                }
                
                let output_idx = (y * input.width + x) as usize * 4;
                if output_idx + 3 < output_data.len() {
                    output_data[output_idx] = r_sum.clamp(0.0, 255.0) as u8;
                    output_data[output_idx + 1] = g_sum.clamp(0.0, 255.0) as u8;
                    output_data[output_idx + 2] = b_sum.clamp(0.0, 255.0) as u8;
                    output_data[output_idx + 3] = a_sum.clamp(0.0, 255.0) as u8;
                }
            }
        }
        
        Ok(ImageData {
            width: input.width,
            height: input.height,
            format: input.format.clone(),
            data: output_data,
        })
    }
    
    fn get_name(&self) -> &str {
        "blur_processor"
    }
    
    fn get_supported_formats(&self) -> Vec<ImageFormat> {
        vec![ImageFormat::RGBA, ImageFormat::RGB]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("未知的处理器")]
    UnknownProcessor,
    #[error("不支持的格式")]
    UnsupportedFormat,
    #[error("处理失败")]
    ProcessingFailed,
    #[error("内存不足")]
    OutOfMemory,
}
```

## ⚡ 第三部分：性能调优

### 1. 编译器优化技巧

#### 高级编译配置

```toml
# Cargo.toml 高级优化配置
[profile.release]
# 最高优化级别
opt-level = 3
# 链接时优化
lto = "fat"
# 代码生成单元
codegen-units = 1
# 恐慌处理
panic = "abort"
# 符号剥离
strip = true
# 调试信息
debug = false
# 溢出检查
overflow-checks = false
# 增量编译
incremental = false

# 目标特定优化
[target.wasm32-unknown-unknown]
rustflags = [
    # WebAssembly 特性
    "-C", "target-feature=+bulk-memory,+simd128,+tail-calls,+sign-ext,+nontrapping-fptoint",
    # 优化级别
    "-C", "opt-level=3",
    # 链接时优化
    "-C", "lto=fat",
    # 恐慌处理
    "-C", "panic=abort",
    # 代码生成单元
    "-C", "codegen-units=1",
    # 符号剥离
    "-C", "strip=symbols",
    # 内联阈值
    "-C", "inline-threshold=275",
    # 向量化
    "-C", "vectorize-loops=true",
    # 向量化SLP
    "-C", "vectorize-slp=true",
]

# 特性配置
[features]
default = ["std", "simd", "bulk-memory", "tail-calls"]
std = []
no_std = []
simd = []
bulk-memory = []
tail-calls = []
host-bindings = []
interface-types = []
```

### 2. 运行时性能调优

#### 性能监控器

```rust
/// 高级性能监控器
pub struct AdvancedPerformanceMonitor {
    metrics: Arc<Mutex<PerformanceMetrics>>,
    profiler: Arc<Mutex<Profiler>>,
    alert_system: Arc<Mutex<AlertSystem>>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: usize,
    pub operation_count: u64,
    pub average_latency: Duration,
    pub throughput: f64,
    pub error_rate: f64,
}

pub struct Profiler {
    call_stack: Vec<CallFrame>,
    function_times: HashMap<String, Duration>,
    memory_allocations: Vec<AllocationRecord>,
}

#[derive(Debug, Clone)]
struct CallFrame {
    function_name: String,
    start_time: Instant,
    memory_usage: usize,
}

#[derive(Debug, Clone)]
struct AllocationRecord {
    size: usize,
    timestamp: Instant,
    stack_trace: Vec<String>,
}

impl AdvancedPerformanceMonitor {
    /// 创建高级性能监控器
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            profiler: Arc::new(Mutex::new(Profiler::new())),
            alert_system: Arc::new(Mutex::new(AlertSystem::new())),
        }
    }
    
    /// 开始性能分析
    pub fn start_profiling(&self, function_name: &str) -> ProfilingHandle {
        let mut profiler = self.profiler.lock().unwrap();
        let handle = profiler.start_function(function_name);
        ProfilingHandle {
            handle,
            profiler: self.profiler.clone(),
        }
    }
    
    /// 记录内存分配
    pub fn record_allocation(&self, size: usize) {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.record_allocation(size);
    }
    
    /// 更新性能指标
    pub fn update_metrics(&self, metrics: PerformanceMetrics) {
        let mut current_metrics = self.metrics.lock().unwrap();
        *current_metrics = metrics;
        
        // 检查是否需要发送警报
        let mut alert_system = self.alert_system.lock().unwrap();
        alert_system.check_alerts(&metrics);
    }
    
    /// 获取性能报告
    pub fn get_performance_report(&self) -> PerformanceReport {
        let metrics = self.metrics.lock().unwrap().clone();
        let profiler = self.profiler.lock().unwrap();
        
        PerformanceReport {
            metrics,
            top_functions: profiler.get_top_functions(10),
            memory_analysis: profiler.get_memory_analysis(),
            recommendations: self.generate_recommendations(&metrics),
        }
    }
    
    /// 生成优化建议
    fn generate_recommendations(&self, metrics: &PerformanceMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if metrics.cpu_usage > 80.0 {
            recommendations.push("CPU使用率过高，建议优化算法或增加并发处理".to_string());
        }
        
        if metrics.memory_usage > 100 * 1024 * 1024 {
            recommendations.push("内存使用过高，建议优化内存管理或减少内存分配".to_string());
        }
        
        if metrics.average_latency > Duration::from_millis(100) {
            recommendations.push("平均延迟过高，建议优化关键路径或使用缓存".to_string());
        }
        
        if metrics.error_rate > 0.01 {
            recommendations.push("错误率过高，建议检查错误处理逻辑".to_string());
        }
        
        recommendations
    }
}

pub struct ProfilingHandle {
    handle: usize,
    profiler: Arc<Mutex<Profiler>>,
}

impl Drop for ProfilingHandle {
    fn drop(&mut self) {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.end_function(self.handle);
    }
}

impl Profiler {
    fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            function_times: HashMap::new(),
            memory_allocations: Vec::new(),
        }
    }
    
    fn start_function(&mut self, function_name: &str) -> usize {
        let handle = self.call_stack.len();
        let frame = CallFrame {
            function_name: function_name.to_string(),
            start_time: Instant::now(),
            memory_usage: self.get_current_memory_usage(),
        };
        self.call_stack.push(frame);
        handle
    }
    
    fn end_function(&mut self, handle: usize) {
        if let Some(frame) = self.call_stack.get(handle) {
            let duration = frame.start_time.elapsed();
            let function_name = &frame.function_name;
            
            let total_time = self.function_times.get(function_name)
                .map(|d| *d)
                .unwrap_or(Duration::ZERO);
            
            self.function_times.insert(function_name.clone(), total_time + duration);
        }
    }
    
    fn record_allocation(&mut self, size: usize) {
        let allocation = AllocationRecord {
            size,
            timestamp: Instant::now(),
            stack_trace: self.get_stack_trace(),
        };
        self.memory_allocations.push(allocation);
    }
    
    fn get_top_functions(&self, count: usize) -> Vec<(String, Duration)> {
        let mut functions: Vec<_> = self.function_times.iter().collect();
        functions.sort_by(|a, b| b.1.cmp(a.1));
        functions.into_iter().take(count).map(|(name, duration)| (name.clone(), *duration)).collect()
    }
    
    fn get_memory_analysis(&self) -> MemoryAnalysis {
        let total_allocations = self.memory_allocations.len();
        let total_size: usize = self.memory_allocations.iter().map(|a| a.size).sum();
        let average_size = if total_allocations > 0 {
            total_size / total_allocations
        } else {
            0
        };
        
        MemoryAnalysis {
            total_allocations,
            total_size,
            average_size,
            allocation_rate: self.calculate_allocation_rate(),
        }
    }
    
    fn calculate_allocation_rate(&self) -> f64 {
        if self.memory_allocations.is_empty() {
            return 0.0;
        }
        
        let first_time = self.memory_allocations.first().unwrap().timestamp;
        let last_time = self.memory_allocations.last().unwrap().timestamp;
        let duration = last_time.duration_since(first_time);
        
        if duration.as_secs() > 0 {
            self.memory_allocations.len() as f64 / duration.as_secs() as f64
        } else {
            0.0
        }
    }
    
    fn get_current_memory_usage(&self) -> usize {
        // 简化的内存使用计算
        std::process::id() as usize * 1024
    }
    
    fn get_stack_trace(&self) -> Vec<String> {
        self.call_stack.iter().map(|frame| frame.function_name.clone()).collect()
    }
}

#[derive(Debug)]
pub struct PerformanceReport {
    pub metrics: PerformanceMetrics,
    pub top_functions: Vec<(String, Duration)>,
    pub memory_analysis: MemoryAnalysis,
    pub recommendations: Vec<String>,
}

#[derive(Debug)]
pub struct MemoryAnalysis {
    pub total_allocations: usize,
    pub total_size: usize,
    pub average_size: usize,
    pub allocation_rate: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            operation_count: 0,
            average_latency: Duration::ZERO,
            throughput: 0.0,
            error_rate: 0.0,
        }
    }
}
```

## 📋 最佳实践总结

### 1. 开发实践

- 使用类型安全的设计模式
- 实现完善的错误处理
- 采用模块化架构
- 编写全面的测试

### 2. 性能实践

- 使用性能监控工具
- 优化关键路径
- 实现智能缓存
- 采用并发处理

### 3. 安全实践

- 实现内存安全保护
- 使用安全的类型转换
- 验证输入数据
- 实现访问控制

### 4. 维护实践

- 保持代码文档更新
- 定期进行代码审查
- 监控运行时性能
- 收集用户反馈

---

**注意**: 本教程提供了高级的 WebAssembly 2.0 + Rust 1.90 开发技术，建议在实际项目中根据具体需求选择合适的实现方案。
