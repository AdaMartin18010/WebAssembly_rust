# WebAssembly 2.0 + Rust 1.90 故障排除和调试指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 开发中常见问题的诊断和解决方案，包括编译错误、运行时错误、性能问题和调试技巧。

## 🔍 常见问题分类

### 1. 编译错误

- 依赖问题
- 特性配置错误
- 目标平台不兼容
- 链接错误

### 2. 运行时错误

- 内存访问错误
- 类型转换错误
- 函数调用错误
- 资源泄漏

### 3. 性能问题

- 启动时间过长
- 内存使用过高
- 执行速度慢
- 并发性能差

### 4. 集成问题

- JavaScript 绑定错误
- DOM 操作失败
- 异步处理问题
- 跨平台兼容性

## 🛠️ 编译错误诊断

### 1. 依赖问题

#### 问题：依赖版本冲突

```bash
error: failed to select a version for `wasm-bindgen`.
    ... required by package `my-project v0.1.0`
    ... which satisfies dependency `my-project` of `my-project`
versions that meet the requirements `^0.2.100` are: 0.2.100, 0.2.101, 0.2.102
all possible versions conflict with previously selected packages.
```

**解决方案**：

```toml
# Cargo.toml
[dependencies]
wasm-bindgen = "0.2.103"  # 使用最新版本
web-sys = "0.3.64"
js-sys = "0.3.64"

# 或者使用兼容版本
[dependencies]
wasm-bindgen = "0.2.100"
web-sys = "0.3.60"
js-sys = "0.3.60"
```

#### 问题：特性配置错误

```bash
error: the feature `simd128` is not available for the target `wasm32-unknown-unknown`
```

**解决方案**：

```toml
# Cargo.toml
[features]
default = ["std"]
simd = []

# 在代码中使用条件编译
#[cfg(feature = "simd")]
use std::arch::wasm32::*;

#[cfg(not(feature = "simd"))]
// 回退实现
```

### 2. 目标平台问题

#### 问题：目标平台不兼容

```bash
error: target `wasm32-unknown-unknown` is not supported
```

**解决方案**：

```bash
# 添加 WebAssembly 目标
rustup target add wasm32-unknown-unknown

# 或者使用 wasm32-wasi
rustup target add wasm32-wasi
```

#### 问题：链接器错误

```bash
error: linking with `wasm-ld` failed: exit code: 1
```

**解决方案**：

```bash
# 安装 WebAssembly 工具链
cargo install wasm-pack
wasm-pack build --target web

# 或者使用特定的链接器
export CC_wasm32_unknown_unknown=clang
export AR_wasm32_unknown_unknown=llvm-ar
```

## 🐛 运行时错误诊断

### 1. 内存访问错误

#### 问题：内存越界访问

```rust
// 错误的代码
pub fn unsafe_memory_access(data: &[u8], index: usize) -> u8 {
    data[index]  // 可能导致越界访问
}
```

**解决方案**：

```rust
// 安全的代码
pub fn safe_memory_access(data: &[u8], index: usize) -> Result<u8, MemoryError> {
    if index < data.len() {
        Ok(data[index])
    } else {
        Err(MemoryError::OutOfBounds)
    }
}

// 或者使用 get 方法
pub fn safe_memory_access_v2(data: &[u8], index: usize) -> Option<u8> {
    data.get(index).copied()
}
```

#### 问题：内存泄漏

```rust
// 可能导致内存泄漏的代码
pub fn create_large_data() -> Vec<u8> {
    let mut data = Vec::new();
    for i in 0..1_000_000 {
        data.push(i as u8);
    }
    data  // 如果没有正确释放，可能导致内存泄漏
}
```

**解决方案**：

```rust
// 使用 RAII 和智能指针
use std::sync::Arc;

pub struct DataManager {
    data: Arc<Vec<u8>>,
}

impl DataManager {
    pub fn new(size: usize) -> Self {
        let data = (0..size).map(|i| i as u8).collect();
        Self {
            data: Arc::new(data),
        }
    }
    
    pub fn get_data(&self) -> Arc<Vec<u8>> {
        self.data.clone()
    }
}

// 自动释放资源
impl Drop for DataManager {
    fn drop(&mut self) {
        // 清理资源
        println!("DataManager dropped");
    }
}
```

### 2. 类型转换错误

#### 问题：类型转换失败

```rust
// 错误的类型转换
pub fn unsafe_type_conversion(value: &str) -> i32 {
    value.parse::<i32>().unwrap()  // 可能导致 panic
}
```

**解决方案**：

```rust
// 安全的类型转换
pub fn safe_type_conversion(value: &str) -> Result<i32, ConversionError> {
    value.parse::<i32>().map_err(|e| ConversionError::ParseError(e.to_string()))
}

// 或者使用默认值
pub fn safe_type_conversion_with_default(value: &str) -> i32 {
    value.parse::<i32>().unwrap_or(0)
}
```

### 3. 函数调用错误

#### 问题：函数调用失败

```rust
// 错误的函数调用
pub fn call_wasm_function(instance: &Instance, name: &str, args: &[Value]) -> Value {
    instance.call_function(name, args).unwrap()  // 可能导致 panic
}
```

**解决方案**：

```rust
// 安全的函数调用
pub fn safe_call_wasm_function(instance: &Instance, name: &str, args: &[Value]) -> Result<Value, CallError> {
    instance.call_function(name, args)
        .map_err(|e| CallError::FunctionCallFailed(e.to_string()))
}

// 带重试的函数调用
pub fn call_with_retry(instance: &Instance, name: &str, args: &[Value], max_retries: usize) -> Result<Value, CallError> {
    for attempt in 0..max_retries {
        match instance.call_function(name, args) {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_retries - 1 => {
                return Err(CallError::MaxRetriesExceeded(e.to_string()));
            }
            Err(_) => {
                // 等待一段时间后重试
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
    Err(CallError::Unknown)
}
```

## ⚡ 性能问题诊断

### 1. 启动时间过长

#### 问题：模块加载缓慢

```rust
// 同步加载可能导致阻塞
pub fn load_module_sync(path: &str) -> Module {
    let data = std::fs::read(path).unwrap();
    Module::new(&data).unwrap()
}
```

**解决方案**：

```rust
// 异步加载
pub async fn load_module_async(path: &str) -> Result<Module, LoadError> {
    let data = tokio::fs::read(path).await?;
    Module::new(&data).map_err(LoadError::ModuleCreationFailed)
}

// 预加载和缓存
pub struct ModuleCache {
    cache: HashMap<String, Arc<Module>>,
}

impl ModuleCache {
    pub async fn get_or_load(&mut self, path: &str) -> Result<Arc<Module>, LoadError> {
        if let Some(module) = self.cache.get(path) {
            return Ok(module.clone());
        }
        
        let module = Arc::new(load_module_async(path).await?);
        self.cache.insert(path.to_string(), module.clone());
        Ok(module)
    }
}
```

### 2. 内存使用过高

#### 问题：内存使用过高

```rust
// 可能导致内存使用过高的代码
pub fn process_large_dataset(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for chunk in data.chunks(1024) {
        let processed = expensive_operation(chunk);
        result.extend(processed);  // 可能导致内存使用过高
    }
    result
}
```

**解决方案**：

```rust
// 使用流式处理
pub fn process_large_dataset_streaming<F>(data: &[u8], mut processor: F) -> Result<(), ProcessingError>
where
    F: FnMut(&[u8]) -> Result<(), ProcessingError>,
{
    for chunk in data.chunks(1024) {
        let processed = expensive_operation(chunk);
        processor(&processed)?;
    }
    Ok(())
}

// 使用内存池
pub struct MemoryPool {
    pool: Vec<Vec<u8>>,
    max_size: usize,
}

impl MemoryPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::new(),
            max_size,
        }
    }
    
    pub fn get_buffer(&mut self, size: usize) -> Vec<u8> {
        if let Some(mut buffer) = self.pool.pop() {
            buffer.resize(size, 0);
            buffer
        } else {
            vec![0; size]
        }
    }
    
    pub fn return_buffer(&mut self, mut buffer: Vec<u8>) {
        if self.pool.len() < self.max_size {
            buffer.clear();
            self.pool.push(buffer);
        }
    }
}
```

### 3. 执行速度慢

#### 问题：算法效率低

```rust
// 低效的算法
pub fn inefficient_search(data: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in data.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}
```

**解决方案**：

```rust
// 使用 SIMD 优化
#[cfg(target_arch = "wasm32")]
pub fn simd_search(data: &[i32], target: i32) -> Option<usize> {
    use std::arch::wasm32::*;
    
    let target_vec = i32x4_splat(target);
    
    for (i, chunk) in data.chunks(4).enumerate() {
        if chunk.len() == 4 {
            let data_vec = i32x4_load(chunk.as_ptr() as *const i32);
            let mask = i32x4_eq(data_vec, target_vec);
            
            if i32x4_any_true(mask) {
                // 找到匹配，返回具体位置
                for (j, &value) in chunk.iter().enumerate() {
                    if value == target {
                        return Some(i * 4 + j);
                    }
                }
            }
        }
    }
    
    None
}

// 使用二分搜索
pub fn binary_search(data: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = data.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match data[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    None
}
```

## 🔧 调试技巧

### 1. 日志调试

#### 结构化日志

```rust
use log::{info, warn, error, debug};

pub struct DebugLogger {
    level: log::Level,
    context: String,
}

impl DebugLogger {
    pub fn new(level: log::Level, context: String) -> Self {
        Self { level, context }
    }
    
    pub fn log_operation(&self, operation: &str, duration: std::time::Duration) {
        match self.level {
            log::Level::Debug => debug!("{}: {} completed in {:?}", self.context, operation, duration),
            log::Level::Info => info!("{}: {} completed in {:?}", self.context, operation, duration),
            log::Level::Warn => warn!("{}: {} completed in {:?}", self.context, operation, duration),
            log::Level::Error => error!("{}: {} completed in {:?}", self.context, operation, duration),
            _ => {}
        }
    }
    
    pub fn log_error(&self, operation: &str, error: &dyn std::error::Error) {
        error!("{}: {} failed with error: {}", self.context, operation, error);
    }
}
```

### 2. 性能分析

#### 性能分析器

```rust
pub struct PerformanceProfiler {
    measurements: HashMap<String, Vec<Duration>>,
    current_measurements: HashMap<String, Instant>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
            current_measurements: HashMap::new(),
        }
    }
    
    pub fn start_measurement(&mut self, name: &str) {
        self.current_measurements.insert(name.to_string(), Instant::now());
    }
    
    pub fn end_measurement(&mut self, name: &str) {
        if let Some(start_time) = self.current_measurements.remove(name) {
            let duration = start_time.elapsed();
            self.measurements.entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
    }
    
    pub fn get_statistics(&self) -> HashMap<String, MeasurementStats> {
        self.measurements.iter()
            .map(|(name, durations)| {
                let stats = MeasurementStats::from_durations(durations);
                (name.clone(), stats)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct MeasurementStats {
    pub count: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl MeasurementStats {
    fn from_durations(durations: &[Duration]) -> Self {
        let count = durations.len();
        let total_time: Duration = durations.iter().sum();
        let average_time = if count > 0 {
            Duration::from_nanos(total_time.as_nanos() as u64 / count as u64)
        } else {
            Duration::ZERO
        };
        let min_time = durations.iter().min().copied().unwrap_or(Duration::ZERO);
        let max_time = durations.iter().max().copied().unwrap_or(Duration::ZERO);
        
        Self {
            count,
            total_time,
            average_time,
            min_time,
            max_time,
        }
    }
}
```

### 3. 内存调试

#### 内存泄漏检测器

```rust
pub struct MemoryLeakDetector {
    allocations: HashMap<usize, AllocationInfo>,
    next_id: usize,
}

#[derive(Debug, Clone)]
struct AllocationInfo {
    size: usize,
    timestamp: Instant,
    stack_trace: Vec<String>,
}

impl MemoryLeakDetector {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            next_id: 0,
        }
    }
    
    pub fn track_allocation(&mut self, size: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let info = AllocationInfo {
            size,
            timestamp: Instant::now(),
            stack_trace: self.get_stack_trace(),
        };
        
        self.allocations.insert(id, info);
        id
    }
    
    pub fn track_deallocation(&mut self, id: usize) {
        self.allocations.remove(&id);
    }
    
    pub fn detect_leaks(&self) -> Vec<LeakInfo> {
        let mut leaks = Vec::new();
        
        for (id, info) in &self.allocations {
            let age = info.timestamp.elapsed();
            if age > Duration::from_secs(60) {  // 超过60秒的分配视为潜在泄漏
                leaks.push(LeakInfo {
                    id: *id,
                    size: info.size,
                    age,
                    stack_trace: info.stack_trace.clone(),
                });
            }
        }
        
        leaks
    }
    
    fn get_stack_trace(&self) -> Vec<String> {
        // 简化的堆栈跟踪实现
        vec!["function1".to_string(), "function2".to_string()]
    }
}

#[derive(Debug)]
pub struct LeakInfo {
    pub id: usize,
    pub size: usize,
    pub age: Duration,
    pub stack_trace: Vec<String>,
}
```

## 🚨 错误处理最佳实践

### 1. 错误类型设计

```rust
#[derive(Debug, thiserror::Error)]
pub enum WebAssemblyError {
    #[error("编译错误: {0}")]
    CompilationError(#[from] CompilationError),
    
    #[error("运行时错误: {0}")]
    RuntimeError(#[from] RuntimeError),
    
    #[error("内存错误: {0}")]
    MemoryError(#[from] MemoryError),
    
    #[error("类型错误: {0}")]
    TypeError(#[from] TypeError),
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CompilationError {
    #[error("模块验证失败: {0}")]
    ValidationFailed(String),
    
    #[error("链接失败: {0}")]
    LinkingFailed(String),
    
    #[error("优化失败: {0}")]
    OptimizationFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("函数调用失败: {0}")]
    FunctionCallFailed(String),
    
    #[error("内存访问失败: {0}")]
    MemoryAccessFailed(String),
    
    #[error("类型转换失败: {0}")]
    TypeConversionFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("内存不足")]
    OutOfMemory,
    
    #[error("内存越界访问")]
    OutOfBounds,
    
    #[error("内存对齐错误")]
    AlignmentError,
}

#[derive(Debug, thiserror::Error)]
pub enum TypeError {
    #[error("类型不匹配: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    #[error("类型转换失败: {0}")]
    ConversionFailed(String),
    
    #[error("类型验证失败: {0}")]
    ValidationFailed(String),
}
```

### 2. 错误恢复策略

```rust
pub struct ErrorRecoveryManager {
    retry_policies: HashMap<String, RetryPolicy>,
    fallback_handlers: HashMap<String, Box<dyn Fn() -> Result<(), WebAssemblyError> + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl ErrorRecoveryManager {
    pub fn new() -> Self {
        Self {
            retry_policies: HashMap::new(),
            fallback_handlers: HashMap::new(),
        }
    }
    
    pub fn register_retry_policy(&mut self, error_type: &str, policy: RetryPolicy) {
        self.retry_policies.insert(error_type.to_string(), policy);
    }
    
    pub fn register_fallback_handler<F>(&mut self, error_type: &str, handler: F)
    where
        F: Fn() -> Result<(), WebAssemblyError> + Send + Sync + 'static,
    {
        self.fallback_handlers.insert(error_type.to_string(), Box::new(handler));
    }
    
    pub async fn handle_error_with_retry(&self, error: &WebAssemblyError, operation: &str) -> Result<(), WebAssemblyError> {
        let error_type = self.get_error_type(error);
        
        if let Some(policy) = self.retry_policies.get(&error_type) {
            let mut delay = policy.initial_delay;
            
            for attempt in 0..policy.max_retries {
                match self.execute_operation(operation).await {
                    Ok(()) => return Ok(()),
                    Err(e) if attempt == policy.max_retries - 1 => {
                        // 最后一次尝试失败，使用回退处理
                        return self.handle_with_fallback(&e).await;
                    }
                    Err(_) => {
                        // 等待后重试
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(
                            Duration::from_millis((delay.as_millis() as f64 * policy.backoff_multiplier) as u64),
                            policy.max_delay
                        );
                    }
                }
            }
        }
        
        Err(error.clone())
    }
    
    async fn handle_with_fallback(&self, error: &WebAssemblyError) -> Result<(), WebAssemblyError> {
        let error_type = self.get_error_type(error);
        
        if let Some(handler) = self.fallback_handlers.get(&error_type) {
            handler()
        } else {
            Err(error.clone())
        }
    }
    
    async fn execute_operation(&self, operation: &str) -> Result<(), WebAssemblyError> {
        // 执行操作的占位符实现
        Ok(())
    }
    
    fn get_error_type(&self, error: &WebAssemblyError) -> String {
        match error {
            WebAssemblyError::CompilationError(_) => "compilation".to_string(),
            WebAssemblyError::RuntimeError(_) => "runtime".to_string(),
            WebAssemblyError::MemoryError(_) => "memory".to_string(),
            WebAssemblyError::TypeError(_) => "type".to_string(),
            WebAssemblyError::IoError(_) => "io".to_string(),
            WebAssemblyError::SerializationError(_) => "serialization".to_string(),
        }
    }
}
```

## 📋 调试检查清单

### 编译阶段

- [ ] 检查依赖版本兼容性
- [ ] 验证特性配置
- [ ] 确认目标平台支持
- [ ] 检查链接器配置

### 运行时阶段

- [ ] 验证内存访问边界
- [ ] 检查类型转换安全性
- [ ] 监控资源使用情况
- [ ] 验证错误处理逻辑

### 性能优化

- [ ] 分析启动时间
- [ ] 监控内存使用
- [ ] 测量执行性能
- [ ] 优化关键路径

### 集成测试

- [ ] 测试 JavaScript 绑定
- [ ] 验证 DOM 操作
- [ ] 检查异步处理
- [ ] 测试跨平台兼容性

---

**注意**: 本指南提供了常见问题的诊断和解决方案，建议在实际开发中根据具体情况选择合适的调试策略。
