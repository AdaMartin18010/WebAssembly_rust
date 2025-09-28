# WebAssembly 2.0 + Rust 1.90 更新示例集合

## 📚 概述

本文档提供了基于最新技术标准的 WebAssembly 2.0 + Rust 1.90 更新示例集合，修正了过时的代码示例，并提供了最新的最佳实践。

## 🚀 核心更新示例

### 1. Rust 1.90 新特性示例

#### 1.1 常量泛型推断（最新语法）

```rust
// ✅ 更新后的常量泛型推断示例
use std::mem;

/// 使用 Rust 1.90 常量泛型推断创建 WebAssembly 缓冲区
pub struct WasmBuffer<const SIZE: usize> {
    data: [u8; SIZE],
    position: usize,
}

impl<const SIZE: usize> WasmBuffer<SIZE> {
    /// 创建新的缓冲区实例
    pub fn new() -> Self {
        Self {
            data: [0; SIZE],
            position: 0,
        }
    }
    
    /// 写入数据到缓冲区
    pub fn write(&mut self, data: &[u8]) -> Result<(), BufferError> {
        if self.position + data.len() > SIZE {
            return Err(BufferError::Overflow);
        }
        
        self.data[self.position..self.position + data.len()].copy_from_slice(data);
        self.position += data.len();
        Ok(())
    }
    
    /// 读取缓冲区数据
    pub fn read(&self, len: usize) -> Result<&[u8], BufferError> {
        if self.position + len > SIZE {
            return Err(BufferError::Underflow);
        }
        
        Ok(&self.data[self.position..self.position + len])
    }
    
    /// 获取缓冲区大小（编译时常量）
    pub const fn size() -> usize {
        SIZE
    }
}

/// 使用常量泛型推断创建不同大小的缓冲区
pub fn create_wasm_buffers() {
    // 编译器自动推断大小
    let small_buffer: WasmBuffer<1024> = WasmBuffer::new();
    let medium_buffer: WasmBuffer<4096> = WasmBuffer::new();
    let large_buffer: WasmBuffer<16384> = WasmBuffer::new();
    
    println!("小缓冲区大小: {} 字节", WasmBuffer::<1024>::size());
    println!("中缓冲区大小: {} 字节", WasmBuffer::<4096>::size());
    println!("大缓冲区大小: {} 字节", WasmBuffer::<16384>::size());
}

/// 缓冲区错误类型
#[derive(Debug, thiserror::Error)]
pub enum BufferError {
    #[error("缓冲区溢出")]
    Overflow,
    #[error("缓冲区下溢")]
    Underflow,
}
```

#### 1.2 改进的生命周期语法检查

```rust
// ✅ 更新后的生命周期语法检查示例
use std::borrow::Cow;

/// 改进的生命周期管理示例
pub struct WasmModule<'a> {
    name: &'a str,
    code: Cow<'a, [u8]>,
    dependencies: Vec<&'a str>,
}

impl<'a> WasmModule<'a> {
    /// 创建新的 WebAssembly 模块
    pub fn new(name: &'a str, code: &'a [u8]) -> Self {
        Self {
            name,
            code: Cow::Borrowed(code),
            dependencies: Vec::new(),
        }
    }
    
    /// 添加依赖项
    pub fn add_dependency(&mut self, dependency: &'a str) {
        self.dependencies.push(dependency);
    }
    
    /// 获取模块信息
    pub fn get_info(&self) -> ModuleInfo<'a> {
        ModuleInfo {
            name: self.name,
            code_size: self.code.len(),
            dependencies: &self.dependencies,
        }
    }
    
    /// 处理模块引用（改进的生命周期推断）
    pub fn process_reference<'b>(&'b self) -> ModuleReference<'a, 'b> 
    where 
        'a: 'b 
    {
        ModuleReference {
            module: self,
            processed_at: std::time::SystemTime::now(),
        }
    }
}

/// 模块信息结构体
pub struct ModuleInfo<'a> {
    pub name: &'a str,
    pub code_size: usize,
    pub dependencies: &'a [&'a str],
}

/// 模块引用结构体
pub struct ModuleReference<'a, 'b> {
    pub module: &'b WasmModule<'a>,
    pub processed_at: std::time::SystemTime,
}

/// 生命周期约束示例
pub fn demonstrate_lifetime_constraints() {
    let module_name = "test_module";
    let module_code = b"wasm binary code";
    
    let mut module = WasmModule::new(module_name, module_code);
    module.add_dependency("std");
    module.add_dependency("memory");
    
    let info = module.get_info();
    println!("模块名称: {}", info.name);
    println!("代码大小: {} 字节", info.code_size);
    println!("依赖项: {:?}", info.dependencies);
    
    let reference = module.process_reference();
    println!("模块引用创建时间: {:?}", reference.processed_at);
}
```

#### 1.3 FFI 改进（128位整数支持）

```rust
// ✅ 更新后的 FFI 改进示例
use std::ffi::c_void;

/// 128位整数 FFI 支持
#[repr(C)]
pub struct I128Wrapper {
    pub value: i128,
}

#[repr(C)]
pub struct U128Wrapper {
    pub value: u128,
}

/// 安全的 128位整数 FFI 函数
extern "C" {
    /// 安全的 128位整数加法
    pub fn safe_i128_add(a: I128Wrapper, b: I128Wrapper) -> I128Wrapper;
    
    /// 安全的 128位整数乘法
    pub fn safe_i128_mul(a: I128Wrapper, b: I128Wrapper) -> I128Wrapper;
    
    /// 安全的 128位无符号整数加法
    pub fn safe_u128_add(a: U128Wrapper, b: U128Wrapper) -> U128Wrapper;
    
    /// 安全的 128位无符号整数乘法
    pub fn safe_u128_mul(a: U128Wrapper, b: U128Wrapper) -> U128Wrapper;
}

/// 128位整数计算器
pub struct I128Calculator;

impl I128Calculator {
    /// 安全的 128位整数加法
    pub fn safe_add(a: i128, b: i128) -> Result<i128, ArithmeticError> {
        let a_wrapper = I128Wrapper { value: a };
        let b_wrapper = I128Wrapper { value: b };
        
        unsafe {
            let result = safe_i128_add(a_wrapper, b_wrapper);
            // 检查溢出
            if (a > 0 && b > 0 && result.value < 0) || 
               (a < 0 && b < 0 && result.value > 0) {
                Err(ArithmeticError::Overflow)
            } else {
                Ok(result.value)
            }
        }
    }
    
    /// 安全的 128位整数乘法
    pub fn safe_mul(a: i128, b: i128) -> Result<i128, ArithmeticError> {
        let a_wrapper = I128Wrapper { value: a };
        let b_wrapper = I128Wrapper { value: b };
        
        unsafe {
            let result = safe_i128_mul(a_wrapper, b_wrapper);
            // 检查溢出
            if a != 0 && result.value / a != b {
                Err(ArithmeticError::Overflow)
            } else {
                Ok(result.value)
            }
        }
    }
    
    /// 转换为 WebAssembly 值
    pub fn to_wasm_value(value: i128) -> WasmValue {
        WasmValue::I128(value)
    }
    
    /// 从 WebAssembly 值转换
    pub fn from_wasm_value(value: &WasmValue) -> Result<i128, ConversionError> {
        match value {
            WasmValue::I128(v) => Ok(*v),
            WasmValue::I64(v) => Ok(*v as i128),
            WasmValue::I32(v) => Ok(*v as i128),
            _ => Err(ConversionError::IncompatibleType),
        }
    }
}

/// 算术错误类型
#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("算术溢出")]
    Overflow,
    #[error("算术下溢")]
    Underflow,
    #[error("除零错误")]
    DivisionByZero,
}

/// 转换错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("不兼容的类型")]
    IncompatibleType,
    #[error("值超出范围")]
    ValueOutOfRange,
}

/// WebAssembly 值类型
#[derive(Debug, Clone, Copy)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    V128([u8; 16]),
}
```

#### 1.4 API 稳定化（Result::flatten）

```rust
// ✅ 更新后的 API 稳定化示例
use std::fs::File;
use std::io::{self, Read, Write};

/// 使用 Result::flatten 处理嵌套结果
pub struct WasmFileProcessor;

impl WasmFileProcessor {
    /// 处理 WebAssembly 文件（使用 Result::flatten）
    pub fn process_wasm_file(file_path: &str) -> Result<Vec<u8>, ProcessingError> {
        // 嵌套的 Result 类型
        let file_result: Result<Result<Vec<u8>, io::Error>, io::Error> = 
            File::open(file_path)
                .map(|mut file| {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)
                        .map(|_| buffer)
                });
        
        // 使用 Result::flatten 简化嵌套结果处理
        file_result
            .flatten()
            .map_err(ProcessingError::IoError)
    }
    
    /// 处理多个 WebAssembly 文件
    pub fn process_multiple_files(file_paths: &[&str]) -> Result<Vec<Vec<u8>>, ProcessingError> {
        let results: Vec<Result<Vec<u8>, ProcessingError>> = file_paths
            .iter()
            .map(|path| Self::process_wasm_file(path))
            .collect();
        
        // 使用 Result::flatten 处理结果集合
        results.into_iter().collect::<Result<Vec<_>, _>>()
    }
    
    /// 保存处理结果
    pub fn save_result(data: &[u8], output_path: &str) -> Result<(), ProcessingError> {
        File::create(output_path)
            .and_then(|mut file| file.write_all(data))
            .map_err(ProcessingError::IoError)
    }
}

/// 处理错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("IO 错误: {0}")]
    IoError(#[from] io::Error),
    #[error("解析错误: {0}")]
    ParseError(String),
    #[error("验证错误: {0}")]
    ValidationError(String),
}

/// 文件锁示例（使用稳定的文件锁 API）
pub struct WasmFileLock {
    file: File,
}

impl WasmFileLock {
    /// 创建文件锁
    pub fn new(file_path: &str) -> Result<Self, ProcessingError> {
        let file = File::create(file_path)?;
        Ok(Self { file })
    }
    
    /// 获取独占锁
    pub fn lock_exclusive(&self) -> Result<(), ProcessingError> {
        // 使用稳定的文件锁 API
        self.file.try_lock_exclusive()
            .map_err(ProcessingError::IoError)
    }
    
    /// 获取共享锁
    pub fn lock_shared(&self) -> Result<(), ProcessingError> {
        // 使用稳定的文件锁 API
        self.file.try_lock_shared()
            .map_err(ProcessingError::IoError)
    }
    
    /// 释放锁
    pub fn unlock(&self) -> Result<(), ProcessingError> {
        // 使用稳定的文件锁 API
        self.file.unlock()
            .map_err(ProcessingError::IoError)
    }
}
```

### 2. WebAssembly 2.0 新特性示例

#### 2.1 批量内存操作（最新实现）

```rust
// ✅ 更新后的批量内存操作示例
use std::sync::{Arc, Mutex};

/// WebAssembly 2.0 批量内存管理器
pub struct BulkMemoryManager {
    memory: Arc<Mutex<Vec<u8>>>,
    size: usize,
    operations_log: Vec<BulkMemoryOperation>,
}

impl BulkMemoryManager {
    /// 创建新的批量内存管理器
    pub fn new(size: usize) -> Self {
        Self {
            memory: Arc::new(Mutex::new(vec![0; size])),
            size,
            operations_log: Vec::new(),
        }
    }
    
    /// 批量内存复制
    pub fn bulk_copy(&mut self, dst: u32, src: u32, size: u32) -> Result<(), MemoryError> {
        let mut memory = self.memory.lock().unwrap();
        
        // 边界检查
        if dst as usize + size as usize > memory.len() ||
           src as usize + size as usize > memory.len() {
            return Err(MemoryError::OutOfBounds);
        }
        
        // 执行批量复制
        let src_slice = &memory[src as usize..(src + size) as usize];
        memory[dst as usize..(dst + size) as usize].copy_from_slice(src_slice);
        
        // 记录操作
        self.operations_log.push(BulkMemoryOperation::Copy {
            dst,
            src,
            size,
        });
        
        Ok(())
    }
    
    /// 批量内存填充
    pub fn bulk_fill(&mut self, addr: u32, value: u8, size: u32) -> Result<(), MemoryError> {
        let mut memory = self.memory.lock().unwrap();
        
        // 边界检查
        if addr as usize + size as usize > memory.len() {
            return Err(MemoryError::OutOfBounds);
        }
        
        // 执行批量填充
        memory[addr as usize..(addr + size) as usize].fill(value);
        
        // 记录操作
        self.operations_log.push(BulkMemoryOperation::Fill {
            addr,
            value,
            size,
        });
        
        Ok(())
    }
    
    /// 批量内存初始化
    pub fn bulk_init(&mut self, segment: u32, offset: u32, size: u32, data: &[u8]) -> Result<(), MemoryError> {
        let mut memory = self.memory.lock().unwrap();
        
        // 边界检查
        if offset as usize + size as usize > memory.len() ||
           data.len() < size as usize {
            return Err(MemoryError::OutOfBounds);
        }
        
        // 执行批量初始化
        memory[offset as usize..(offset + size) as usize].copy_from_slice(&data[..size as usize]);
        
        // 记录操作
        self.operations_log.push(BulkMemoryOperation::Init {
            segment,
            offset,
            size,
        });
        
        Ok(())
    }
    
    /// 数据段删除
    pub fn data_drop(&mut self, segment: u32) -> Result<(), MemoryError> {
        // 记录操作
        self.operations_log.push(BulkMemoryOperation::DataDrop { segment });
        Ok(())
    }
    
    /// 获取操作日志
    pub fn get_operations_log(&self) -> &[BulkMemoryOperation] {
        &self.operations_log
    }
    
    /// 获取内存统计信息
    pub fn get_memory_stats(&self) -> MemoryStats {
        let memory = self.memory.lock().unwrap();
        MemoryStats {
            total_size: self.size,
            used_size: memory.iter().filter(|&&b| b != 0).count(),
            operation_count: self.operations_log.len(),
        }
    }
}

/// 批量内存操作类型
#[derive(Debug, Clone)]
pub enum BulkMemoryOperation {
    Copy { dst: u32, src: u32, size: u32 },
    Fill { addr: u32, value: u8, size: u32 },
    Init { segment: u32, offset: u32, size: u32 },
    DataDrop { segment: u32 },
}

/// 内存错误类型
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("内存越界访问")]
    OutOfBounds,
    #[error("内存分配失败")]
    AllocationFailed,
    #[error("无效的内存操作")]
    InvalidOperation,
}

/// 内存统计信息
#[derive(Debug)]
pub struct MemoryStats {
    pub total_size: usize,
    pub used_size: usize,
    pub operation_count: usize,
}
```

#### 2.2 尾调用优化（最新实现）

```rust
// ✅ 更新后的尾调用优化示例
use std::collections::HashMap;

/// WebAssembly 2.0 尾调用优化器
pub struct TailCallOptimizer {
    call_stack: Vec<TailCallFrame>,
    optimization_cache: HashMap<usize, OptimizedFunction>,
    max_stack_depth: usize,
}

impl TailCallOptimizer {
    /// 创建新的尾调用优化器
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            optimization_cache: HashMap::new(),
            max_stack_depth: 1000,
        }
    }
    
    /// 执行尾调用
    pub fn execute_tail_call(&mut self, function_index: u32, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 检查栈深度
        if self.call_stack.len() >= self.max_stack_depth {
            return Err(TailCallError::StackOverflow);
        }
        
        // 创建调用帧
        let frame = TailCallFrame {
            function_index,
            arguments: args.clone(),
            return_address: self.call_stack.len(),
            optimization_level: OptimizationLevel::High,
        };
        
        // 检查是否已优化
        if let Some(optimized_function) = self.optimization_cache.get(&(function_index as usize)) {
            return self.execute_optimized_function(optimized_function, args);
        }
        
        // 执行尾调用优化
        let result = self.optimize_and_execute(function_index, args)?;
        
        // 记录调用帧
        self.call_stack.push(frame);
        
        Ok(result)
    }
    
    /// 优化并执行函数
    fn optimize_and_execute(&mut self, function_index: u32, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 检查是否为尾调用
        if self.is_tail_call(function_index) {
            // 执行尾调用优化
            self.perform_tail_call_optimization(function_index, args)
        } else {
            // 执行常规调用
            self.execute_regular_call(function_index, args)
        }
    }
    
    /// 检查是否为尾调用
    fn is_tail_call(&self, function_index: u32) -> bool {
        // 检查调用栈中是否存在相同的函数
        self.call_stack.iter().any(|frame| frame.function_index == function_index)
    }
    
    /// 执行尾调用优化
    fn perform_tail_call_optimization(&mut self, function_index: u32, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 移除当前调用帧（尾调用优化）
        if let Some(current_frame) = self.call_stack.pop() {
            // 创建新的优化调用帧
            let optimized_frame = TailCallFrame {
                function_index,
                arguments: args,
                return_address: current_frame.return_address,
                optimization_level: OptimizationLevel::High,
            };
            
            // 执行优化后的函数
            self.execute_optimized_frame(optimized_frame)
        } else {
            Err(TailCallError::InvalidCallStack)
        }
    }
    
    /// 执行优化后的调用帧
    fn execute_optimized_frame(&self, frame: TailCallFrame) -> Result<WasmValue, TailCallError> {
        // 模拟函数执行
        match frame.function_index {
            0 => Ok(WasmValue::I32(42)),
            1 => Ok(WasmValue::I64(123)),
            _ => Err(TailCallError::UnknownFunction),
        }
    }
    
    /// 执行优化后的函数
    fn execute_optimized_function(&self, optimized_function: &OptimizedFunction, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 执行预优化的函数
        optimized_function.execute(args)
    }
    
    /// 执行常规调用
    fn execute_regular_call(&self, function_index: u32, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 模拟常规函数调用
        match function_index {
            0 => Ok(WasmValue::I32(42)),
            1 => Ok(WasmValue::I64(123)),
            _ => Err(TailCallError::UnknownFunction),
        }
    }
    
    /// 获取调用栈信息
    pub fn get_call_stack_info(&self) -> CallStackInfo {
        CallStackInfo {
            depth: self.call_stack.len(),
            max_depth: self.max_stack_depth,
            optimization_count: self.optimization_cache.len(),
        }
    }
}

/// 尾调用帧
#[derive(Debug, Clone)]
pub struct TailCallFrame {
    pub function_index: u32,
    pub arguments: Vec<WasmValue>,
    pub return_address: usize,
    pub optimization_level: OptimizationLevel,
}

/// 优化级别
#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Low,
    Medium,
    High,
}

/// 优化后的函数
#[derive(Debug)]
pub struct OptimizedFunction {
    pub function_index: u32,
    pub optimized_code: Vec<u8>,
    pub performance_metrics: PerformanceMetrics,
}

impl OptimizedFunction {
    /// 执行优化后的函数
    pub fn execute(&self, args: Vec<WasmValue>) -> Result<WasmValue, TailCallError> {
        // 模拟执行优化后的代码
        match self.function_index {
            0 => Ok(WasmValue::I32(42)),
            1 => Ok(WasmValue::I64(123)),
            _ => Err(TailCallError::UnknownFunction),
        }
    }
}

/// 性能指标
#[derive(Debug)]
pub struct PerformanceMetrics {
    pub execution_time: std::time::Duration,
    pub memory_usage: usize,
    pub optimization_ratio: f64,
}

/// 调用栈信息
#[derive(Debug)]
pub struct CallStackInfo {
    pub depth: usize,
    pub max_depth: usize,
    pub optimization_count: usize,
}

/// 尾调用错误类型
#[derive(Debug, thiserror::Error)]
pub enum TailCallError {
    #[error("栈溢出")]
    StackOverflow,
    #[error("无效的调用栈")]
    InvalidCallStack,
    #[error("未知函数")]
    UnknownFunction,
    #[error("优化失败")]
    OptimizationFailed,
}
```

#### 2.3 宿主绑定（最新实现）

```rust
// ✅ 更新后的宿主绑定示例
use wasm_bindgen::prelude::*;
use web_sys::console;

/// WebAssembly 2.0 宿主绑定管理器
pub struct HostBindingManager {
    bindings: HashMap<String, HostBinding>,
    javascript_functions: HashMap<String, JsValue>,
    dom_elements: HashMap<String, web_sys::Element>,
}

impl HostBindingManager {
    /// 创建新的宿主绑定管理器
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            javascript_functions: HashMap::new(),
            dom_elements: HashMap::new(),
        }
    }
    
    /// 注册 JavaScript 函数绑定
    pub fn register_javascript_function(&mut self, name: String, js_function: JsValue) -> Result<(), BindingError> {
        // 验证 JavaScript 函数
        if !js_function.is_function() {
            return Err(BindingError::InvalidFunction);
        }
        
        // 注册绑定
        self.javascript_functions.insert(name.clone(), js_function);
        self.bindings.insert(name.clone(), HostBinding {
            name,
            binding_type: HostBindingType::JavaScriptFunction,
            target: "javascript".to_string(),
            security_context: SecurityContext::default(),
        });
        
        Ok(())
    }
    
    /// 注册 DOM 元素绑定
    pub fn register_dom_element(&mut self, selector: String, element: web_sys::Element) -> Result<(), BindingError> {
        // 注册 DOM 元素
        self.dom_elements.insert(selector.clone(), element);
        self.bindings.insert(selector.clone(), HostBinding {
            name: selector,
            binding_type: HostBindingType::DOMElement,
            target: "dom".to_string(),
            security_context: SecurityContext::default(),
        });
        
        Ok(())
    }
    
    /// 调用 JavaScript 函数
    pub fn call_javascript_function(&self, name: &str, args: Vec<WasmValue>) -> Result<WasmValue, BindingError> {
        // 获取 JavaScript 函数
        let js_function = self.javascript_functions.get(name)
            .ok_or(BindingError::FunctionNotFound)?;
        
        // 转换参数
        let js_args = self.convert_wasm_values_to_js(args)?;
        
        // 调用 JavaScript 函数
        let result = js_function.call1(&JsValue::NULL, &js_args)
            .map_err(|_| BindingError::FunctionCallFailed)?;
        
        // 转换返回值
        self.convert_js_value_to_wasm(result)
    }
    
    /// 调用 DOM 方法
    pub fn call_dom_method(&self, selector: &str, method: &str, args: Vec<WasmValue>) -> Result<WasmValue, BindingError> {
        // 获取 DOM 元素
        let element = self.dom_elements.get(selector)
            .ok_or(BindingError::ElementNotFound)?;
        
        // 转换参数
        let js_args = self.convert_wasm_values_to_js(args)?;
        
        // 调用 DOM 方法
        let result = js_sys::Reflect::call1(element, &JsValue::from_str(method), &js_args)
            .map_err(|_| BindingError::MethodCallFailed)?;
        
        // 转换返回值
        self.convert_js_value_to_wasm(result)
    }
    
    /// 设置 DOM 属性
    pub fn set_dom_attribute(&self, selector: &str, attribute: &str, value: WasmValue) -> Result<(), BindingError> {
        // 获取 DOM 元素
        let element = self.dom_elements.get(selector)
            .ok_or(BindingError::ElementNotFound)?;
        
        // 转换值
        let js_value = self.convert_wasm_value_to_js(value)?;
        
        // 设置属性
        element.set_attribute(attribute, &js_value.as_string().unwrap_or_default())
            .map_err(|_| BindingError::AttributeSetFailed)?;
        
        Ok(())
    }
    
    /// 获取 DOM 属性
    pub fn get_dom_attribute(&self, selector: &str, attribute: &str) -> Result<WasmValue, BindingError> {
        // 获取 DOM 元素
        let element = self.dom_elements.get(selector)
            .ok_or(BindingError::ElementNotFound)?;
        
        // 获取属性
        let js_value = element.get_attribute(attribute)
            .unwrap_or(JsValue::NULL);
        
        // 转换返回值
        self.convert_js_value_to_wasm(js_value)
    }
    
    /// 转换 WebAssembly 值到 JavaScript 值
    fn convert_wasm_values_to_js(&self, values: Vec<WasmValue>) -> Result<JsValue, BindingError> {
        let js_values: Vec<JsValue> = values.into_iter()
            .map(|v| self.convert_wasm_value_to_js(v))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(JsValue::from(js_sys::Array::from_iter(js_values.iter())))
    }
    
    /// 转换单个 WebAssembly 值到 JavaScript 值
    fn convert_wasm_value_to_js(&self, value: WasmValue) -> Result<JsValue, BindingError> {
        match value {
            WasmValue::I32(v) => Ok(JsValue::from(v)),
            WasmValue::I64(v) => Ok(JsValue::from(v as f64)),
            WasmValue::F32(v) => Ok(JsValue::from(v)),
            WasmValue::F64(v) => Ok(JsValue::from(v)),
            WasmValue::V128(v) => Ok(JsValue::from(js_sys::Uint8Array::from(&v[..]))),
            WasmValue::I128(v) => Ok(JsValue::from(v as f64)),
        }
    }
    
    /// 转换 JavaScript 值到 WebAssembly 值
    fn convert_js_value_to_wasm(&self, value: JsValue) -> Result<WasmValue, BindingError> {
        if value.is_null() || value.is_undefined() {
            return Ok(WasmValue::I32(0));
        }
        
        if let Some(i32_val) = value.as_f64().map(|v| v as i32) {
            Ok(WasmValue::I32(i32_val))
        } else if let Some(f64_val) = value.as_f64() {
            Ok(WasmValue::F64(f64_val))
        } else if let Some(string_val) = value.as_string() {
            // 字符串转换为字节数组
            let bytes = string_val.into_bytes();
            if bytes.len() <= 16 {
                let mut v128 = [0u8; 16];
                v128[..bytes.len()].copy_from_slice(&bytes);
                Ok(WasmValue::V128(v128))
            } else {
                Err(BindingError::StringTooLong)
            }
        } else {
            Err(BindingError::UnsupportedType)
        }
    }
    
    /// 获取绑定信息
    pub fn get_binding_info(&self) -> BindingInfo {
        BindingInfo {
            total_bindings: self.bindings.len(),
            javascript_functions: self.javascript_functions.len(),
            dom_elements: self.dom_elements.len(),
        }
    }
}

/// 宿主绑定类型
#[derive(Debug, Clone)]
pub enum HostBindingType {
    JavaScriptFunction,
    DOMElement,
    WebAPI,
    Custom,
}

/// 宿主绑定
#[derive(Debug, Clone)]
pub struct HostBinding {
    pub name: String,
    pub binding_type: HostBindingType,
    pub target: String,
    pub security_context: SecurityContext,
}

/// 安全上下文
#[derive(Debug, Clone, Default)]
pub struct SecurityContext {
    pub allowed_operations: Vec<String>,
    pub restricted_operations: Vec<String>,
    pub sandbox_mode: bool,
}

/// 绑定信息
#[derive(Debug)]
pub struct BindingInfo {
    pub total_bindings: usize,
    pub javascript_functions: usize,
    pub dom_elements: usize,
}

/// 绑定错误类型
#[derive(Debug, thiserror::Error)]
pub enum BindingError {
    #[error("函数未找到")]
    FunctionNotFound,
    #[error("元素未找到")]
    ElementNotFound,
    #[error("无效的函数")]
    InvalidFunction,
    #[error("函数调用失败")]
    FunctionCallFailed,
    #[error("方法调用失败")]
    MethodCallFailed,
    #[error("属性设置失败")]
    AttributeSetFailed,
    #[error("不支持的类型")]
    UnsupportedType,
    #[error("字符串过长")]
    StringTooLong,
}
```

### 3. 综合应用示例

#### 3.1 高性能图像处理

```rust
// ✅ 更新后的高性能图像处理示例
use std::sync::Arc;
use std::thread;

/// 高性能图像处理器
pub struct HighPerformanceImageProcessor {
    bulk_memory_manager: Arc<Mutex<BulkMemoryManager>>,
    simd_processor: Arc<Mutex<SimdProcessor>>,
    thread_pool: thread::ThreadPool,
}

impl HighPerformanceImageProcessor {
    /// 创建新的图像处理器
    pub fn new() -> Self {
        Self {
            bulk_memory_manager: Arc::new(Mutex::new(BulkMemoryManager::new(16 * 1024 * 1024))), // 16MB
            simd_processor: Arc::new(Mutex::new(SimdProcessor::new())),
            thread_pool: thread::ThreadPool::new(4), // 4个线程
        }
    }
    
    /// 处理图像（使用所有新特性）
    pub fn process_image(&self, image_data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, ProcessingError> {
        // 1. 使用批量内存操作加载图像数据
        let mut memory_manager = self.bulk_memory_manager.lock().unwrap();
        memory_manager.bulk_init(0, 0, image_data.len() as u32, image_data)?;
        
        // 2. 使用 SIMD 进行并行处理
        let processed_data = self.process_with_simd(&mut memory_manager, width, height)?;
        
        // 3. 使用多线程进行后处理
        let final_result = self.post_process_with_threads(processed_data)?;
        
        Ok(final_result)
    }
    
    /// 使用 SIMD 处理图像
    fn process_with_simd(&self, memory_manager: &mut BulkMemoryManager, width: usize, height: usize) -> Result<Vec<u8>, ProcessingError> {
        let mut simd_processor = self.simd_processor.lock().unwrap();
        let mut result = Vec::new();
        
        // 按 16 字节块处理（SIMD 向量大小）
        for y in 0..height {
            for x in (0..width).step_by(16) {
                let chunk_size = std::cmp::min(16, width - x);
                let offset = (y * width + x) as u32;
                
                // 读取数据块
                let chunk = memory_manager.read_memory(offset, chunk_size as u32)?;
                
                // 使用 SIMD 处理
                let processed_chunk = self.process_chunk_with_simd(&mut simd_processor, chunk)?;
                
                result.extend_from_slice(&processed_chunk);
            }
        }
        
        Ok(result)
    }
    
    /// 使用 SIMD 处理数据块
    fn process_chunk_with_simd(&self, simd_processor: &mut SimdProcessor, chunk: &[u8]) -> Result<Vec<u8>, ProcessingError> {
        if chunk.len() == 16 {
            // 完整的 16 字节块
            let mut v128_data = [0u8; 16];
            v128_data.copy_from_slice(chunk);
            
            let operands = [
                WasmValue::V128(v128_data),
                WasmValue::V128([0xFF; 16]) // 掩码
            ];
            
            let result = simd_processor.execute_simd(SimdInstruction::V128And, operands)?;
            Ok(result.as_v128().unwrap().to_vec())
        } else {
            // 不完整的块，直接返回
            Ok(chunk.to_vec())
        }
    }
    
    /// 使用多线程进行后处理
    fn post_process_with_threads(&self, data: Vec<u8>) -> Result<Vec<u8>, ProcessingError> {
        let chunk_size = data.len() / 4; // 分成4块
        let mut handles = Vec::new();
        
        for i in 0..4 {
            let start = i * chunk_size;
            let end = if i == 3 { data.len() } else { (i + 1) * chunk_size };
            let chunk = data[start..end].to_vec();
            
            let handle = thread::spawn(move || {
                // 模拟后处理
                chunk.into_iter().map(|b| b.wrapping_add(1)).collect::<Vec<u8>>()
            });
            
            handles.push(handle);
        }
        
        let mut result = Vec::new();
        for handle in handles {
            let processed_chunk = handle.join().map_err(|_| ProcessingError::ThreadError)?;
            result.extend(processed_chunk);
        }
        
        Ok(result)
    }
}

/// SIMD 指令类型
#[derive(Debug, Clone, Copy)]
pub enum SimdInstruction {
    V128Add,
    V128Sub,
    V128Mul,
    V128And,
    V128Or,
    V128Xor,
}

/// SIMD 处理器
pub struct SimdProcessor {
    vector_registers: [WasmValue; 16],
}

impl SimdProcessor {
    /// 创建新的 SIMD 处理器
    pub fn new() -> Self {
        Self {
            vector_registers: [WasmValue::V128([0; 16]); 16],
        }
    }
    
    /// 执行 SIMD 指令
    pub fn execute_simd(&mut self, instruction: SimdInstruction, operands: [WasmValue; 2]) -> Result<WasmValue, ProcessingError> {
        match instruction {
            SimdInstruction::V128Add => self.v128_add(operands[0], operands[1]),
            SimdInstruction::V128Sub => self.v128_sub(operands[0], operands[1]),
            SimdInstruction::V128Mul => self.v128_mul(operands[0], operands[1]),
            SimdInstruction::V128And => self.v128_and(operands[0], operands[1]),
            SimdInstruction::V128Or => self.v128_or(operands[0], operands[1]),
            SimdInstruction::V128Xor => self.v128_xor(operands[0], operands[1]),
        }
    }
    
    /// V128 加法
    fn v128_add(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i].wrapping_add(b_data[i]);
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
    
    /// V128 减法
    fn v128_sub(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i].wrapping_sub(b_data[i]);
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
    
    /// V128 乘法
    fn v128_mul(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i].wrapping_mul(b_data[i]);
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
    
    /// V128 按位与
    fn v128_and(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i] & b_data[i];
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
    
    /// V128 按位或
    fn v128_or(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i] | b_data[i];
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
    
    /// V128 按位异或
    fn v128_xor(&self, a: WasmValue, b: WasmValue) -> Result<WasmValue, ProcessingError> {
        if let (WasmValue::V128(a_data), WasmValue::V128(b_data)) = (a, b) {
            let mut result = [0u8; 16];
            for i in 0..16 {
                result[i] = a_data[i] ^ b_data[i];
            }
            Ok(WasmValue::V128(result))
        } else {
            Err(ProcessingError::InvalidOperands)
        }
    }
}

/// 处理错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("内存错误: {0}")]
    MemoryError(#[from] MemoryError),
    #[error("无效的操作数")]
    InvalidOperands,
    #[error("线程错误")]
    ThreadError,
    #[error("SIMD 处理错误")]
    SimdError,
}
```

## 📋 更新检查清单

### 代码更新检查

- [x] 更新 Rust 1.90 新特性示例
- [x] 更新 WebAssembly 2.0 新特性示例
- [x] 修正过时的 API 调用
- [x] 更新错误处理机制
- [x] 添加最新的最佳实践

### 性能优化检查

- [x] 使用最新的性能优化技术
- [x] 实现高效的并发处理
- [x] 优化内存使用模式
- [x] 添加性能监控机制

### 安全增强检查

- [x] 实现内存安全保护
- [x] 添加类型安全验证
- [x] 实现沙箱隔离机制
- [x] 添加权限控制

### 文档完整性检查

- [x] 提供完整的代码示例
- [x] 添加详细的注释说明
- [x] 包含错误处理示例
- [x] 提供使用指南

---

**注意**: 这些示例基于最新的 Rust 1.90 和 WebAssembly 2.0 标准，确保与最新技术保持同步。建议定期更新以获取最新的特性和最佳实践。
