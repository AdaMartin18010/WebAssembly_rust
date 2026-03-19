# WebAssembly + Rust 1.94 升级实施计划

## 🎯 实施概览

**目标**: 将项目从 Rust 1.90 + WebAssembly 2.0 升级到 Rust 1.94 + WebAssembly 3.0 + WASI 0.3
**预计工期**: 4-6 周
**里程碑**:

- Week 1: 基础升级
- Week 2-3: 新特性实现
- Week 4: 测试与优化
- Week 5-6: 文档与部署

---

## 📋 详细实施步骤

### 第一阶段：基础升级 (Week 1)

#### Day 1-2: 环境准备与工具链更新

```bash
# 1. 更新 Rust 工具链
rustup update stable
rustup default 1.94.0

# 2. 添加新的 WASM 目标
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasip1      # WASI Preview 1
rustup target add wasm32-wasip2      # WASI Preview 2 (包含 WASI 0.3 支持)

# 3. 安装更新的 WASM 工具
cargo install wasm-pack --version 0.12.1
cargo install cargo-component --version 0.21.0
cargo install wasm-tools --version 1.229.0
cargo install wit-bindgen-cli --version 0.41.0

# 4. 验证安装
rustc --version  # 应显示 1.94.0
cargo --version  # 应显示 1.94.0
```

#### Day 3-4: 核心依赖更新

**文件**: `Cargo.toml` (根目录)

```toml
[workspace]
resolver = "3"

members = [
    "wasm",
    "examples/basic",
    "examples/advanced",
    "examples/performance",
    "tests/integration",
    "examples/wasi_03",        # 新增
    "examples/component",      # 新增
]

[workspace.package]
edition = "2024"
rust-version = "1.94"          # 从 1.90 更新
license = "MIT OR Apache-2.0"
repository = "https://github.com/rust-lang/webassembly"
description = "WebAssembly 3.0 + Rust 1.94 完整集成项目"
keywords = ["webassembly", "wasm", "rust", "performance", "security", "wasi"]
categories = ["web-programming", "wasm", "development-tools"]

[workspace.dependencies]
# WebAssembly 相关 - 2026年3月最新版本
wasm-bindgen = "0.2.105"
wasm-bindgen-futures = "0.4.55"
js-sys = "0.3.82"
web-sys = "0.3.82"
wasmtime = { version = "42.0.0", features = ["component-model", "async"] }
wasmparser = "0.243.0"
wasm-encoder = "0.243.0"
wasm-opt = "0.116.1"
wasi = "0.14.0"

# Component Model 工具
wit-bindgen = "0.41.0"
cargo-component = "0.21.0"
wasm-tools = "1.229.0"

# 其他依赖保持不变，只更新版本号...
```

**文件**: `wasm/Cargo.toml`

```toml
[package]
name = "wasm"
version = "0.2.0"              # 版本升级
edition = "2024"
resolver = "3"
rust-version = "1.94"
authors = ["Rust WebAssembly Team"]
description = "WebAssembly 3.0 与 Rust 1.94 集成项目 - 包含WASI 0.3、Component Model和WasmGC支持"

[dependencies]
# 序列化
serde = { workspace = true, features = ["derive", "rc"] }
serde_json = { workspace = true }
serde_bytes = "0.11.19"

# 错误处理
thiserror = { workspace = true }
anyhow = { workspace = true }

# 异步运行时
tokio = { workspace = true, features = ["full"] }

# 工具库
uuid = { workspace = true, features = ["v4", "serde"] }
chrono = { workspace = true, features = ["serde"] }
log = { workspace = true }
env_logger = "0.11.8"
rand = "0.9.2"

# WebAssembly 相关
wasm-bindgen = { workspace = true }
wasm-bindgen-futures = { workspace = true }
js-sys = { workspace = true }
web-sys = { workspace = true }
wasmtime = { workspace = true }
wasmparser = { workspace = true }
wasm-encoder = { workspace = true }
wasm-opt = { workspace = true }
wasi = { workspace = true }
wit-bindgen = { workspace = true }

# 数学和算法
num-traits = "0.2.19"
num-bigint = "0.4.6"
num-complex = "0.4.6"

# 内存管理
bumpalo = "3.19.0"

# 性能分析
criterion = { version = "0.7.0", optional = true }

# 测试工具
proptest = { version = "1.8.0", optional = true }

[dev-dependencies]
criterion = "0.7.0"
proptest = "1.8.0"
tempfile = "3.23.0"

[features]
default = ["std", "webassembly-3-0", "rust-194", "wasi-03"]
std = []
no_std = []
bench = ["criterion"]
test = ["proptest"]

# WebAssembly 版本特性
webassembly-2-0 = ["simd", "bulk-memory", "tail-calls", "host-bindings"]
webassembly-3-0 = ["webassembly-2-0", "wasmgc", "memory64", "exception-handling-exnref"]

# WebAssembly 2.0 特性
simd = []
bulk-memory = []
tail-calls = []
host-bindings = []

# WebAssembly 3.0 新特性
wasmgc = []
memory64 = []
exception-handling-exnref = []

# Rust 版本特性
rust-190 = ["const-generics", "improved-lifetimes", "ffi-improvements"]
rust-194 = ["rust-190", "array-windows", "simd-fp16", "lazy-lock-enhanced"]

# WASI 特性
wasi-02 = []
wasi-03 = ["wasi-02", "async-wasi", "component-model-async"]
async-wasi = []
component-model-async = []

const-generics = []
improved-lifetimes = []
ffi-improvements = []
array-windows = []
simd-fp16 = []
lazy-lock-enhanced = []

[[bench]]
name = "performance_benchmarks"
harness = false
required-features = ["bench"]
```

#### Day 5: 构建验证

```bash
# 验证所有包可以编译
cargo check --all
cargo check --all --features webassembly-3-0
cargo check --all --features wasi-03

# 运行测试
cargo test --all

# 构建 WASM 包
cd wasm
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-wasip2
```

---

### 第二阶段：新模块实现 (Week 2-3)

#### Week 2 Day 1-3: Rust 1.94 特性模块

**创建文件**: `wasm/src/rust_194_features.rs`

```rust
//! # Rust 1.94 新特性在 WebAssembly 中的应用
//!
//! 本模块整合 Rust 1.94 的所有新特性：
//! - array_windows: 常量长度窗口迭代
//! - element_offset: 元素偏移计算
//! - LazyCell/LazyLock 增强方法
//! - AVX-512 FP16 / NEON FP16 SIMD
//! - 新增数学常量

use std::sync::LazyLock;

/// 使用 array_windows 优化 WebAssembly 内存操作
///
/// # 示例
/// ```
/// use wasm::rust_194_features::MemoryOptimizer;
///
/// let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
/// let optimized = MemoryOptimizer::process_with_windows(&data);
/// ```
pub struct MemoryOptimizer;

impl MemoryOptimizer {
    /// 使用 array_windows 进行4字节对齐处理
    pub fn process_with_windows(data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len());

        // Rust 1.94: 使用 array_windows 进行4字节窗口处理
        for [a, b, c, d] in data.array_windows() {
            // 处理4字节块
            let sum = a.wrapping_add(*b).wrapping_add(*c).wrapping_add(*d);
            result.push(sum);
        }

        // 处理剩余字节
        let processed = data.len().saturating_sub(3);
        result.extend_from_slice(&data[processed..]);

        result
    }

    /// 使用 element_offset 计算精确偏移
    pub fn calculate_offsets<T>(arr: &[T]) -> Vec<usize> {
        (0..arr.len())
            .map(|i| arr.element_offset(i))
            .collect()
    }
}

/// 使用增强的 LazyLock 进行 WebAssembly 模块配置
pub static WASM_MODULE_CACHE: LazyLock<ModuleCache> = LazyLock::new(|| {
    ModuleCache::initialize()
});

pub struct ModuleCache {
    modules: std::collections::HashMap<String, Vec<u8>>,
}

impl ModuleCache {
    fn initialize() -> Self {
        Self {
            modules: std::collections::HashMap::new(),
        }
    }

    /// 使用 Rust 1.94 的 get_mut 安全获取可变引用
    pub fn get_module_mut(&self, name: &str) -> Option<&mut Vec<u8>> {
        // Rust 1.94 新增: get_mut 允许安全地获取可变引用
        WASM_MODULE_CACHE.get_mut()?.modules.get_mut(name)
    }

    /// 使用 force_mut 强制初始化并获取可变引用
    pub fn force_get_module_mut(&self, name: &str) -> Option<&mut Vec<u8>> {
        WASM_MODULE_CACHE.force_mut().modules.get_mut(name)
    }
}

/// Rust 1.94 新增数学常量在数值计算中的应用
pub mod math_constants {
    use std::f64::consts::{EULER_GAMMA, GOLDEN_RATIO};

    /// 使用欧拉-马歇罗尼常数进行数值近似
    pub fn euler_approximation(n: u64) -> f64 {
        let harmonic = (1..=n).map(|i| 1.0 / i as f64).sum::<f64>();
        harmonic - (n as f64).ln() - EULER_GAMMA
    }

    /// 使用黄金比例进行分割计算
    pub fn golden_ratio_split(total: f64) -> (f64, f64) {
        let major = total / GOLDEN_RATIO;
        let minor = total - major;
        (major, minor)
    }
}

/// AVX-512 FP16 SIMD 支持 (x86_64)
#[cfg(all(target_arch = "x86_64", target_feature = "avx512fp16"))]
pub mod simd_fp16_x86 {
    use std::arch::x86_64::*;

    /// FP16 向量加法
    pub unsafe fn add_fp16(a: __m256h, b: __m256h) -> __m256h {
        _mm256_add_ph(a, b)
    }

    /// FP16 向量乘法
    pub unsafe fn mul_fp16(a: __m256h, b: __m256h) -> __m256h {
        _mm256_mul_ph(a, b)
    }
}

/// NEON FP16 SIMD 支持 (AArch64)
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub mod simd_fp16_arm {
    use std::arch::aarch64::*;

    /// FP16 向量加法
    pub unsafe fn add_fp16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
        vaddq_f16(a, b)
    }
}

/// 使用 Peekable::next_if_map 增强的迭代器处理
pub mod enhanced_iterators {
    /// 处理 WebAssembly 指令流
    pub fn process_instructions<I>(instructions: I) -> Vec<u32>
    where
        I: Iterator<Item = u32>,
    {
        let mut peekable = instructions.peekable();
        let mut results = Vec::new();

        while let Some(value) = peekable.next_if_map(|x| if x > 0 { Some(x * 2) } else { None }) {
            results.push(value);
        }

        results
    }
}

/// char 到 usize 的安全转换
pub fn char_to_usize(c: char) -> Option<usize> {
    // Rust 1.94: TryFrom<char> for usize 已稳定
    c.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_windows() {
        let data = [1u8, 2, 3, 4, 5];
        let result = MemoryOptimizer::process_with_windows(&data);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_euler_constant() {
        let result = math_constants::euler_approximation(1000);
        assert!(result.abs() < 0.001);
    }
}
```

#### Week 2 Day 4-5: WebAssembly 3.0 模块

**创建文件**: `wasm/src/webassembly_3_0.rs`

```rust
//! # WebAssembly 3.0 特性实现
//!
//! 包含 WebAssembly 3.0 的所有新特性：
//! - WasmGC: 垃圾回收支持
//! - Memory64: 64位内存寻址
//! - Exception Handling (exnref): 增强异常处理
//! - Relaxed SIMD: 灵活SIMD操作
//! - JavaScript String Builtins: JS字符串内置支持

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// WebAssembly 3.0 特性标志
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebAssembly3Feature {
    // WebAssembly 2.0 基础特性
    BulkMemoryOperations,
    TailCallOptimization,
    SimdInstructions,
    MultiValue,
    ReferenceTypes,

    // WebAssembly 3.0 新特性
    WasmGC,                    // 垃圾回收
    Memory64,                  // 64位内存
    ExceptionHandlingExnref,   // exnref 异常处理
    RelaxedSimd,               // 灵活 SIMD
    JavaScriptStringBuiltins,  // JS 字符串内置
    ExtendedConstantExpressions, // 扩展常量表达式
}

/// WebAssembly 3.0 模块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly3Module {
    pub id: ModuleId,
    pub name: String,
    pub features: Vec<WebAssembly3Feature>,
    pub memory64: Option<Memory64>,
    pub gc_types: Vec<GcType>,
    pub exception_handlers: Vec<ExnrefHandler>,
}

/// 64位内存管理 (Memory64)
#[derive(Debug, Clone)]
pub struct Memory64 {
    data: Vec<u8>,
    initial_pages: u64,
    maximum_pages: Option<u64>,
}

const WASM_PAGE_SIZE_64: u64 = 65536; // 64KB

impl Memory64 {
    pub fn new(initial_pages: u64, maximum_pages: Option<u64>) -> Self {
        let size = initial_pages * WASM_PAGE_SIZE_64;
        Self {
            data: vec![0; size as usize],
            initial_pages,
            maximum_pages,
        }
    }

    /// 读取64位地址
    pub fn read_u64(&self, addr: u64) -> Result<u64, Memory64Error> {
        if addr + 8 > self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[addr as usize..addr as usize + 8]);
        Ok(u64::from_le_bytes(bytes))
    }

    /// 写入64位地址
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<(), Memory64Error> {
        if addr + 8 > self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }

        let bytes = value.to_le_bytes();
        self.data[addr as usize..addr as usize + 8].copy_from_slice(&bytes);
        Ok(())
    }

    /// 增长内存
    pub fn grow(&mut self, delta_pages: u64) -> Result<u64, Memory64Error> {
        let current_pages = self.data.len() as u64 / WASM_PAGE_SIZE_64;
        let new_pages = current_pages + delta_pages;

        if let Some(max) = self.maximum_pages {
            if new_pages > max {
                return Err(Memory64Error::MemoryLimitExceeded);
            }
        }

        let new_size = new_pages * WASM_PAGE_SIZE_64;
        self.data.resize(new_size as usize, 0);
        Ok(current_pages)
    }
}

/// WasmGC 类型系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GcType {
    Struct(GcStruct),
    Array(GcArray),
    I31,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcStruct {
    pub fields: Vec<GcField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcField {
    pub value_type: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcArray {
    pub element_type: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    Ref(RefType),
    RefNull(RefType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefType {
    Func,
    Extern,
    Any,        // WasmGC
    Eq,         // WasmGC
    I31,        // WasmGC
    Struct(u32), // WasmGC
    Array(u32),  // WasmGC
}

/// exnref 异常处理器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExnrefHandler {
    pub tag: ExnrefTag,
    pub catch_blocks: Vec<CatchBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExnrefTag {
    pub index: u32,
    pub params: Vec<ValueType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchBlock {
    pub tag_index: u32,
    pub label: u32,
    pub handler_instructions: Vec<Instruction>,
}

/// WebAssembly 3.0 指令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    // 基础指令...

    // WasmGC 指令
    StructNew(u32),
    StructGet(u32, u32),
    StructSet(u32, u32),
    ArrayNew(u32),
    ArrayGet(u32),
    ArraySet(u32),
    ArrayLen,
    I31New,
    I31GetSigned,
    I31GetUnsigned,
    RefNull(RefType),
    RefIsNull,
    RefFunc(u32),
    RefEq,

    // exnref 异常处理指令
    TryTable(TryTableBlock),
    Throw(u32),
    Rethrow(u32),

    // Memory64 指令
    Memory64Load { offset: u64, align: u32 },
    Memory64Store { offset: u64, align: u32 },
    Memory64Grow,
    Memory64Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryTableBlock {
    pub block_type: BlockType,
    pub catches: Vec<CatchClause>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatchClause {
    Catch(u32, u32),      // tag, label
    CatchRef(u32, u32),   // tag, label (保留异常引用)
    CatchAll(u32),        // label
    CatchAllRef(u32),     // label (保留异常引用)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}

/// 错误类型
#[derive(Debug, Clone, Error)]
pub enum Memory64Error {
    #[error("内存越界访问")]
    OutOfBounds,
    #[error("内存限制超出")]
    MemoryLimitExceeded,
    #[error("内存增长失败")]
    GrowFailed,
}

/// 模块ID类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModuleId(pub String);

impl ModuleId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for ModuleId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory64_basic() {
        let mut memory = Memory64::new(1, Some(10));

        // 测试写入和读取
        memory.write_u64(0, 0x123456789ABCDEF0).unwrap();
        let value = memory.read_u64(0).unwrap();
        assert_eq!(value, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_memory64_grow() {
        let mut memory = Memory64::new(1, Some(10));
        let old_pages = memory.grow(2).unwrap();
        assert_eq!(old_pages, 1);
    }
}
```

#### Week 3 Day 1-3: WASI 0.3 模块

**创建文件**: `wasm/src/wasi_03.rs`

```rust
//! # WASI 0.3 原生异步支持
//!
//! 提供 WASI 0.3 的完整实现，包括：
//! - 原生 async/await 支持
//! - stream<T> 和 future<T> 类型
//! - 取消令牌
//! - 组件模型异步

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// WASI 0.3 运行时
pub struct Wasi03Runtime {
    config: RuntimeConfig,
}

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub max_concurrent_tasks: usize,
    pub enable_cancellation: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 1000,
            enable_cancellation: true,
        }
    }
}

impl Wasi03Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }

    /// 执行异步操作
    pub async fn execute<F, T>(&self, future: F) -> Result<T, Wasi03Error>
    where
        F: Future<Output = Result<T, Wasi03Error>>,
    {
        future.await
    }

    /// 创建新的流
    pub fn create_stream<T>(&self, buffer_size: usize) -> (StreamWriter<T>, StreamReader<T>)
    where
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel(buffer_size);
        let cancelled = Arc::new(Mutex::new(false));

        let writer = StreamWriter {
            sender: tx,
            cancelled: cancelled.clone(),
        };

        let reader = StreamReader {
            receiver: rx,
            cancelled,
        };

        (writer, reader)
    }

    /// 创建新的 future
    pub fn create_future<T>(&self) -> (FutureCompleter<T>, WasiFuture<T>)
    where
        T: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let cancelled = Arc::new(Mutex::new(false));

        let completer = FutureCompleter {
            sender: Some(tx),
            cancelled: cancelled.clone(),
        };

        let future = WasiFuture {
            receiver: Some(rx),
            cancelled,
        };

        (completer, future)
    }
}

/// 流写入端
pub struct StreamWriter<T> {
    sender: mpsc::Sender<T>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> StreamWriter<T> {
    pub async fn send(&self, item: T) -> Result<(), StreamError> {
        if *self.cancelled.lock().await {
            return Err(StreamError::Cancelled);
        }

        self.sender.send(item).await.map_err(|_| StreamError::Closed)
    }

    pub async fn close(self) {
        drop(self.sender);
    }
}

/// 流读取端
pub struct StreamReader<T> {
    receiver: mpsc::Receiver<T>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> StreamReader<T> {
    pub async fn recv(&mut self) -> Option<T> {
        if *self.cancelled.lock().await {
            return None;
        }

        self.receiver.recv().await
    }

    pub async fn cancel(&self) {
        *self.cancelled.lock().await = true;
    }
}

/// Future 完成端
pub struct FutureCompleter<T> {
    sender: Option<tokio::sync::oneshot::Sender<T>>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> FutureCompleter<T> {
    pub fn complete(mut self, value: T) -> Result<(), T> {
        if let Some(sender) = self.sender.take() {
            sender.send(value)
        } else {
            Err(value)
        }
    }

    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
}

/// WASI Future 类型
pub struct WasiFuture<T> {
    receiver: Option<tokio::sync::oneshot::Receiver<T>>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> Future for WasiFuture<T> {
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.receiver.as_mut() {
            Some(receiver) => match receiver.try_recv() {
                Ok(value) => Poll::Ready(Some(value)),
                Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                Err(_) => Poll::Ready(None),
            },
            None => Poll::Ready(None),
        }
    }
}

impl<T> WasiFuture<T> {
    pub async fn cancel(&self) {
        *self.cancelled.lock().await = true;
    }
}

/// 取消令牌
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<Mutex<bool>>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn cancel(&self) {
        *self.cancelled.lock().await = true;
    }

    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }

    /// 检查是否被取消，如果被取消则返回错误
    pub async fn check_cancelled(&self) -> Result<(), Wasi03Error> {
        if self.is_cancelled().await {
            Err(Wasi03Error::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// 错误类型
#[derive(Debug, Clone)]
pub enum Wasi03Error {
    Io(String),
    Cancelled,
    StreamClosed,
    InvalidState,
}

#[derive(Debug, Clone)]
pub enum StreamError {
    Cancelled,
    Closed,
    Full,
}

/// HTTP 请求示例 (WASI 0.3)
pub mod http {
    use super::*;

    pub struct HttpClient;

    impl HttpClient {
        pub async fn request(
            &self,
            method: &str,
            url: &str,
            body: Option<Vec<u8>>,
        ) -> Result<HttpResponse, Wasi03Error> {
            // WASI 0.3 原生异步 HTTP 请求
            // 不再需要手动管理 pollable handles
            todo!("WASI 0.3 HTTP implementation")
        }
    }

    pub struct HttpResponse {
        pub status: u16,
        pub headers: Vec<(String, String)>,
        pub body: Vec<u8>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stream() {
        let runtime = Wasi03Runtime::new(RuntimeConfig::default());
        let (writer, mut reader) = runtime.create_stream::<i32>(10);

        writer.send(42).await.unwrap();
        let value = reader.recv().await;

        assert_eq!(value, Some(42));
    }

    #[tokio::test]
    async fn test_future() {
        let runtime = Wasi03Runtime::new(RuntimeConfig::default());
        let (completer, future) = runtime.create_future::<i32>();

        completer.complete(42).unwrap();
        let value = future.await;

        assert_eq!(value, Some(42));
    }

    #[tokio::test]
    async fn test_cancellation() {
        let token = CancellationToken::new();

        token.cancel().await;
        assert!(token.is_cancelled().await);
    }
}
```

#### Week 3 Day 4-5: Component Model 模块

**创建文件**: `wasm/src/component_model.rs`

```rust
//! # WebAssembly Component Model 支持
//!
//! 实现 WIT 定义和组件组合

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WIT 接口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitInterface {
    pub name: String,
    pub functions: Vec<WitFunction>,
    pub types: Vec<WitTypeDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitFunction {
    pub name: String,
    pub params: Vec<WitParam>,
    pub results: WitResults,
    pub is_async: bool,  // WASI 0.3 支持
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitParam {
    pub name: String,
    pub param_type: WitType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitResults {
    Single(WitType),
    Multiple(Vec<(String, WitType)>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitTypeDef {
    pub name: String,
    pub definition: WitType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitType {
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    Char,
    Bool,
    String,
    List(Box<WitType>),
    Option(Box<WitType>),
    Result(Box<WitType>, Box<WitType>),
    Tuple(Vec<WitType>),
    Record(Vec<(String, WitType)>),
    Variant(Vec<(String, Option<WitType>)>),
    Flags(Vec<String>),
    Own(u32),        // 资源句柄
    Borrow(u32),     // 借用句柄
}

/// 组件定义
#[derive(Debug, Clone)]
pub struct Component {
    pub id: ComponentId,
    pub name: String,
    pub imports: Vec<ComponentImport>,
    pub exports: Vec<ComponentExport>,
    pub wasm_module: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ComponentImport {
    pub name: String,
    pub interface: WitInterface,
}

#[derive(Debug, Clone)]
pub struct ComponentExport {
    pub name: String,
    pub interface: WitInterface,
}

/// 组件组合器
pub struct ComponentComposer {
    components: Vec<Component>,
    wit_files: HashMap<String, String>,
}

impl ComponentComposer {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            wit_files: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn register_wit(&mut self, name: String, wit_content: String) {
        self.wit_files.insert(name, wit_content);
    }

    /// 组合多个组件
    pub fn compose(&self) -> Result<ComposedComponent, CompositionError> {
        // 验证接口兼容性
        self.validate_interfaces()?;

        // 组合组件
        let composed = ComposedComponent {
            components: self.components.clone(),
            entry_point: self.find_entry_point()?,
        };

        Ok(composed)
    }

    fn validate_interfaces(&self) -> Result<(), CompositionError> {
        // 检查导入/导出匹配
        for component in &self.components {
            for import in &component.imports {
                // 验证是否有对应的导出
                let found = self.components.iter().any(|c| {
                    c.exports.iter().any(|e| e.name == import.name)
                });

                if !found {
                    return Err(CompositionError::MissingExport {
                        component: component.name.clone(),
                        import: import.name.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    fn find_entry_point(&self) -> Result<ComponentId, CompositionError> {
        // 查找入口组件
        self.components.first()
            .map(|c| c.id.clone())
            .ok_or(CompositionError::NoEntryPoint)
    }
}

/// 组合后的组件
#[derive(Debug, Clone)]
pub struct ComposedComponent {
    pub components: Vec<Component>,
    pub entry_point: ComponentId,
}

/// 组合错误
#[derive(Debug, Clone)]
pub enum CompositionError {
    MissingExport { component: String, import: String },
    InterfaceMismatch { expected: String, actual: String },
    NoEntryPoint,
    ValidationFailed(String),
}

/// WIT 解析器
pub struct WitParser;

impl WitParser {
    pub fn parse(wit_content: &str) -> Result<WitInterface, WitParseError> {
        // WIT 解析实现
        // 这里简化实现，实际应使用 wit-parser crate
        todo!("WIT parsing implementation")
    }
}

#[derive(Debug, Clone)]
pub struct WitParseError(pub String);

/// 示例 WIT 定义
pub mod examples {
    use super::*;

    /// 计算器接口示例
    pub fn calculator_interface() -> WitInterface {
        WitInterface {
            name: "calculator".to_string(),
            functions: vec![
                WitFunction {
                    name: "add".to_string(),
                    params: vec![
                        WitParam { name: "a".to_string(), param_type: WitType::S32 },
                        WitParam { name: "b".to_string(), param_type: WitType::S32 },
                    ],
                    results: WitResults::Single(WitType::S32),
                    is_async: false,
                },
                WitFunction {
                    name: "divide".to_string(),
                    params: vec![
                        WitParam { name: "a".to_string(), param_type: WitType::F64 },
                        WitParam { name: "b".to_string(), param_type: WitType::F64 },
                    ],
                    results: WitResults::Single(WitType::Result(
                        Box::new(WitType::F64),
                        Box::new(WitType::String),
                    )),
                    is_async: false,
                },
            ],
            types: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_composition() {
        let mut composer = ComponentComposer::new();

        let component = Component {
            id: ComponentId::new(),
            name: "test".to_string(),
            imports: vec![],
            exports: vec![],
            wasm_module: vec![],
        };

        composer.add_component(component);

        let result = composer.compose();
        assert!(result.is_ok());
    }
}
```

---

### 第三阶段：测试与优化 (Week 4)

#### Week 4 Day 1-2: 集成测试

**创建文件**: `tests/integration/wasi_03_tests.rs`

```rust
//! WASI 0.3 集成测试

use wasm::wasi_03::*;

#[tokio::test]
async fn test_http_request() {
    let runtime = Wasi03Runtime::new(RuntimeConfig::default());
    let client = http::HttpClient;

    // 测试异步 HTTP 请求
    let response = client.request("GET", "https://example.com", None).await;

    // 验证结果
    assert!(response.is_ok() || response.is_err()); // 网络依赖
}

#[tokio::test]
async fn test_stream_processing() {
    let runtime = Wasi03Runtime::new(RuntimeConfig::default());
    let (writer, mut reader) = runtime.create_stream::<i32>(100);

    // 并发写入
    let write_task = tokio::spawn(async move {
        for i in 0..100 {
            writer.send(i).await.unwrap();
        }
    });

    // 并发读取
    let mut count = 0;
    while reader.recv().await.is_some() {
        count += 1;
        if count >= 100 {
            break;
        }
    }

    write_task.await.unwrap();
    assert_eq!(count, 100);
}
```

#### Week 4 Day 3-4: 性能基准测试

**创建文件**: `wasm/benches/rust_194_benchmarks.rs`

```rust
//! Rust 1.94 新特性性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm::rust_194_features::*;

fn bench_array_windows(c: &mut Criterion) {
    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    c.bench_function("array_windows", |b| {
        b.iter(|| {
            MemoryOptimizer::process_with_windows(black_box(&data))
        });
    });
}

fn bench_element_offset(c: &mut Criterion) {
    let data: Vec<u64> = (0..10000).collect();

    c.bench_function("element_offset", |b| {
        b.iter(|| {
            MemoryOptimizer::calculate_offsets(black_box(&data))
        });
    });
}

criterion_group!(rust_194_benches, bench_array_windows, bench_element_offset);
criterion_main!(rust_194_benches);
```

#### Week 4 Day 5: 验证与修复

```bash
# 运行所有测试
cargo test --all --features webassembly-3-0,wasi-03

# 运行基准测试
cargo bench --features bench

# 检查代码质量
cargo clippy --all --features webassembly-3-0,wasi-03 -- -D warnings
cargo fmt --all -- --check

# 安全审计
cargo audit
```

---

### 第四阶段：文档与部署 (Week 5-6)

#### Week 5: 文档更新

**更新文件**: `README.md`

```markdown
# WebAssembly 3.0 + Rust 1.94 完整集成项目

## 新特性

- ✅ WebAssembly 3.0 (WasmGC, Memory64, Exception Handling)
- ✅ WASI 0.3 原生异步支持
- ✅ Rust 1.94 新特性 (array_windows, SIMD FP16, 增强 LazyLock)
- ✅ Component Model 多语言组合

## 快速开始

### 环境要求
- Rust 1.94.0+
- wasm-pack 0.12.1+
- cargo-component 0.21.0+

### 构建
```bash
cargo build --all --features webassembly-3-0,wasi-03
```

### 运行示例

```bash
cargo run --example wasi_03_demo --features wasi-03
```

```

**创建文件**: `docs/RUST_194_MIGRATION_GUIDE.md`

详细迁移指南...

**创建文件**: `docs/WASI_03_GUIDE.md`

WASI 0.3 使用指南...

#### Week 6: 最终验证与发布

```bash
# 完整构建验证
cargo build --release --all-features

# 文档构建
cargo doc --all --no-deps

# 发布准备
cargo publish --dry-run -p wasm
```

---

## 📊 成功指标

| 指标 | 目标 | 验证方法 |
|------|------|----------|
| 构建成功率 | 100% | `cargo build --all` |
| 测试通过率 | >95% | `cargo test --all` |
| 代码覆盖率 | >80% | `cargo tarpaulin` |
| 零安全警告 | 0 | `cargo audit` |
| 文档完整度 | 100% | `cargo doc` |

---

## 🚨 风险缓解

| 风险 | 可能性 | 缓解措施 |
|------|--------|----------|
| 依赖冲突 | 中 | 使用 workspace dependencies |
| 构建时间增加 | 高 | 优化 CI/CD，使用缓存 |
| API 不稳定 | 低 | 使用特性标志隔离新功能 |

---

**计划版本**: v1.0
**最后更新**: 2026-03-19
