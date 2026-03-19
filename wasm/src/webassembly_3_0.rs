//! # WebAssembly 3.0 特性实现
//!
//! 包含 WebAssembly 3.0 的所有新特性：
//! - WasmGC: 垃圾回收支持，允许Java/Kotlin/Dart等语言直接编译
//! - Memory64: 64位内存寻址，突破4GB限制
//! - Exception Handling (exnref): 增强异常处理
//! - Relaxed SIMD: 灵活SIMD操作
//! - JavaScript String Builtins: JS字符串内置支持

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;

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
    pub version: String,
    pub features: Vec<WebAssembly3Feature>,
    pub memory64: Option<Memory64>,
    pub gc_types: Vec<GcType>,
    pub exception_handlers: Vec<ExnrefHandler>,
    pub functions: Vec<WebAssembly3Function>,
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
}

impl WebAssembly3Module {
    /// 创建新的 WebAssembly 3.0 模块
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ModuleId::new(),
            name: name.into(),
            version: "3.0".to_string(),
            features: vec![],
            memory64: None,
            gc_types: vec![],
            exception_handlers: vec![],
            functions: vec![],
            exports: vec![],
            imports: vec![],
        }
    }
    
    /// 启用特性
    pub fn enable_feature(&mut self, feature: WebAssembly3Feature) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }
    
    /// 检查是否支持某特性
    pub fn supports_feature(&self, feature: &WebAssembly3Feature) -> bool {
        self.features.contains(feature)
    }
    
    /// 初始化64位内存
    pub fn init_memory64(&mut self, initial_pages: u64, maximum_pages: Option<u64>) {
        self.memory64 = Some(Memory64::new(initial_pages, maximum_pages));
        self.enable_feature(WebAssembly3Feature::Memory64);
    }
    
    /// 添加GC类型
    pub fn add_gc_type(&mut self, gc_type: GcType) {
        self.gc_types.push(gc_type);
        self.enable_feature(WebAssembly3Feature::WasmGC);
    }
    
    /// 验证模块
    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        
        // 验证特性依赖
        if self.supports_feature(&WebAssembly3Feature::ExceptionHandlingExnref)
            && !self.supports_feature(&WebAssembly3Feature::ReferenceTypes)
        {
            errors.push(ValidationError::FeatureDependency {
                feature: "ExceptionHandlingExnref".to_string(),
                requires: "ReferenceTypes".to_string(),
            });
        }
        
        // 验证内存
        if let Some(ref mem) = self.memory64 && let Err(e) = mem.validate() {
            errors.push(ValidationError::MemoryError(e.to_string()));
        }
        
        ValidationResult {
            valid: errors.is_empty(),
            errors,
        }
    }
}

/// 64位内存管理 (Memory64)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory64 {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    initial_pages: u64,
    maximum_pages: Option<u64>,
}

pub const WASM_PAGE_SIZE_64: u64 = 65536; // 64KB per page

impl Memory64 {
    /// 创建新的64位内存
    pub fn new(initial_pages: u64, maximum_pages: Option<u64>) -> Self {
        // 限制最大内存大小以避免溢出
        let size = initial_pages.saturating_mul(WASM_PAGE_SIZE_64);
        let size = size.min(1024 * 1024 * 1024 * 16); // 最大16GB
        
        Self {
            data: vec![0; size as usize],
            initial_pages,
            maximum_pages,
        }
    }
    
    /// 获取内存大小（字节）
    pub fn size_bytes(&self) -> u64 {
        self.data.len() as u64
    }
    
    /// 获取页面数
    pub fn size_pages(&self) -> u64 {
        self.data.len() as u64 / WASM_PAGE_SIZE_64
    }
    
    /// 读取8位
    pub fn read_u8(&self, addr: u64) -> Result<u8, Memory64Error> {
        if addr >= self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }
        Ok(self.data[addr as usize])
    }
    
    /// 写入8位
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<(), Memory64Error> {
        if addr >= self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }
        self.data[addr as usize] = value;
        Ok(())
    }
    
    /// 读取64位
    pub fn read_u64(&self, addr: u64) -> Result<u64, Memory64Error> {
        if addr + 8 > self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[addr as usize..addr as usize + 8]);
        Ok(u64::from_le_bytes(bytes))
    }
    
    /// 写入64位
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
        let current_pages = self.size_pages();
        let new_pages = current_pages + delta_pages;
        
        if let Some(max) = self.maximum_pages && new_pages > max {
            return Err(Memory64Error::MemoryLimitExceeded);
        }

        let new_size = new_pages * WASM_PAGE_SIZE_64;
        if new_size > (1024 * 1024 * 1024 * 16) {
            // 16GB 限制
            return Err(Memory64Error::MemoryLimitExceeded);
        }

        self.data.resize(new_size as usize, 0);
        Ok(current_pages)
    }

    /// 验证内存
    pub fn validate(&self) -> Result<(), Memory64Error> {
        if self.initial_pages == 0 {
            return Err(Memory64Error::InvalidSize);
        }

        if let Some(max) = self.maximum_pages && max < self.initial_pages {
            return Err(Memory64Error::InvalidSize);
        }

        Ok(())
    }

    /// 批量复制
    pub fn bulk_copy(&mut self, src: u64, dst: u64, size: u64) -> Result<(), Memory64Error> {
        if src + size > self.data.len() as u64 || dst + size > self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }
        
        // 处理重叠情况
        if src < dst && src + size > dst {
            // 反向复制
            for i in (0..size).rev() {
                self.data[(dst + i) as usize] = self.data[(src + i) as usize];
            }
        } else {
            // 正向复制
            for i in 0..size {
                self.data[(dst + i) as usize] = self.data[(src + i) as usize];
            }
        }
        
        Ok(())
    }
    
    /// 批量填充
    pub fn bulk_fill(&mut self, addr: u64, value: u8, size: u64) -> Result<(), Memory64Error> {
        if addr + size > self.data.len() as u64 {
            return Err(Memory64Error::OutOfBounds);
        }
        
        for i in 0..size {
            self.data[(addr + i) as usize] = value;
        }
        
        Ok(())
    }
}

/// WasmGC 类型系统
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcType {
    Struct(GcStruct),
    Array(GcArray),
    I31,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcStruct {
    pub name: String,
    pub fields: Vec<GcField>,
    pub super_type: Option<u32>, // 父类型索引（继承）
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcField {
    pub name: String,
    pub value_type: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcArray {
    pub element_type: ValueType,
    pub mutable: bool,
    pub min_size: u32,
    pub max_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    V128,
    I8,
    I16,
    Ref(RefType),
    RefNull(RefType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefType {
    Func,
    Extern,
    Any,        // WasmGC
    Eq,         // WasmGC
    I31,        // WasmGC
    Struct(u32), // WasmGC
    Array(u32),  // WasmGC
    Exn,        // Exception handling
    NoExn,      // Exception handling
}

/// exnref 异常处理器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExnrefHandler {
    pub tag: ExnrefTag,
    pub catch_blocks: Vec<CatchBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExnrefTag {
    pub index: u32,
    pub params: Vec<ValueType>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchBlock {
    pub tag_index: u32,
    pub label: u32,
    pub handler_instructions: Vec<Instruction>,
    pub rethrow: bool,
}

/// WebAssembly 3.0 指令
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Instruction {
    // 基础指令
    Nop,
    Drop,
    Select,
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
    
    // 数值指令
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Add,
    I32Sub,
    I32Mul,
    I64Add,
    I64Sub,
    I64Mul,
    
    // WasmGC 指令
    StructNew(u32),
    StructNewDefault(u32),
    StructGet(u32, u32),  // type, field
    StructGetS(u32, u32),
    StructGetU(u32, u32),
    StructSet(u32, u32),
    ArrayNew(u32),
    ArrayNewDefault(u32),
    ArrayNewFixed(u32, u32),
    ArrayNewData(u32, u32),
    ArrayNewElem(u32, u32),
    ArrayGet(u32),
    ArrayGetS(u32),
    ArrayGetU(u32),
    ArraySet(u32),
    ArrayLen,
    ArrayFill(u32),
    ArrayCopy(u32, u32),
    ArrayInitData(u32, u32),
    ArrayInitElem(u32, u32),
    I31New,
    I31GetS,
    I31GetU,
    RefNull(RefType),
    RefIsNull,
    RefFunc(u32),
    RefAsNonNull,
    RefEq,
    RefTest(RefType),
    RefCast(RefType),
    RefTestNullable(RefType),
    RefCastNullable(RefType),
    ExternConvertAny,
    AnyConvertExtern,
    
    // exnref 异常处理指令
    TryTable(TryTableBlock),
    Throw(u32),
    ThrowRef,
    Rethrow(u32),
    
    // Memory64 指令
    Memory64Load { offset: u64, align: u32, ty: LoadType },
    Memory64Store { offset: u64, align: u32, ty: StoreType },
    Memory64Grow,
    Memory64Size,
    Memory64Fill,
    Memory64Copy,
    
    // 控制流
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>),
    Return,
    Call(u32),
    CallIndirect(u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadType {
    I32,
    I64,
    F32,
    F64,
    I8S, I8U,
    I16S, I16U,
    I32S, I32U,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StoreType {
    I32,
    I64,
    F32,
    F64,
    I8, I16, I32_8, I64_8, I64_16, I64_32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TryTableBlock {
    pub block_type: BlockType,
    pub catches: Vec<CatchClause>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CatchClause {
    Catch(u32, u32),      // tag, label
    CatchRef(u32, u32),   // tag, label (保留异常引用)
    CatchAll(u32),        // label
    CatchAllRef(u32),     // label (保留异常引用)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
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

/// 导出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub kind: ExportKind,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportKind {
    Function,
    Table,
    Memory,
    Global,
    Tag, // Exception tag
}

/// 导入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub kind: ImportKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportKind {
    Function(u32), // type index
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
    Tag(ExnrefTag),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableType {
    pub element_type: RefType,
    pub minimum: u32,
    pub maximum: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryType {
    pub minimum: u64,
    pub maximum: Option<u64>,
    pub memory64: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalType {
    pub value_type: ValueType,
    pub mutable: bool,
}

/// WebAssembly 3.0 函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly3Function {
    pub index: u32,
    pub name: Option<String>,
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
    pub locals: Vec<ValueType>,
    pub body: Vec<Instruction>,
}

/// 验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

/// 错误类型
#[derive(Debug, Clone, Error)]
pub enum Memory64Error {
    #[error("内存越界访问")]
    OutOfBounds,
    #[error("内存限制超出")]
    MemoryLimitExceeded,
    #[error("无效的内存大小")]
    InvalidSize,
    #[error("内存增长失败")]
    GrowFailed,
}

#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    #[error("特性依赖错误: {feature} 需要 {requires}")]
    FeatureDependency { feature: String, requires: String },
    #[error("内存错误: {0}")]
    MemoryError(String),
    #[error("类型错误: {0}")]
    TypeError(String),
}

/// 运行时
pub struct WebAssembly3Runtime {
    modules: HashMap<ModuleId, WebAssembly3Module>,
    gc_heap: GcHeap,
}

/// GC堆
#[derive(Debug)]
pub struct GcHeap {
    objects: HashMap<u32, GcObject>,
    next_id: u32,
}

#[derive(Debug, Clone)]
pub struct GcObject {
    pub id: u32,
    pub ty: GcType,
    pub data: Vec<u8>,
    pub marked: bool,
}

impl Default for WebAssembly3Runtime {
    fn default() -> Self {
        Self {
            modules: HashMap::new(),
            gc_heap: GcHeap {
                objects: HashMap::new(),
                next_id: 1,
            },
        }
    }
}

impl WebAssembly3Runtime {
    /// 创建新的运行时
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load_module(&mut self, module: WebAssembly3Module) -> ModuleId {
        let id = module.id.clone();
        self.modules.insert(id.clone(), module);
        id
    }
    
    pub fn allocate_gc_object(&mut self, ty: GcType, data: Vec<u8>) -> u32 {
        let id = self.gc_heap.next_id;
        self.gc_heap.next_id += 1;
        
        let obj = GcObject {
            id,
            ty,
            data,
            marked: false,
        };
        
        self.gc_heap.objects.insert(id, obj);
        id
    }
    
    /// 获取GC堆中的对象数量
    pub fn gc_object_count(&self) -> usize {
        self.gc_heap.objects.len()
    }
    
    /// 获取GC对象数据（如果存在）
    pub fn get_gc_object_data(&self, id: u32) -> Option<&Vec<u8>> {
        self.gc_heap.objects.get(&id).map(|obj| &obj.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory64_basic() {
        let mut memory = Memory64::new(1, Some(10));
        
        // 测试写入和读取8位
        memory.write_u8(0, 0xAB).unwrap();
        assert_eq!(memory.read_u8(0).unwrap(), 0xAB);
        
        // 测试写入和读取64位
        memory.write_u64(8, 0x123456789ABCDEF0).unwrap();
        let value = memory.read_u64(8).unwrap();
        assert_eq!(value, 0x123456789ABCDEF0);
    }
    
    #[test]
    fn test_memory64_grow() {
        let mut memory = Memory64::new(1, Some(10));
        let old_pages = memory.grow(2).unwrap();
        assert_eq!(old_pages, 1);
        assert_eq!(memory.size_pages(), 3);
    }
    
    #[test]
    fn test_memory64_bulk_operations() {
        let mut memory = Memory64::new(1, None);
        
        // 填充
        memory.bulk_fill(0, 0x42, 100).unwrap();
        assert_eq!(memory.read_u8(50).unwrap(), 0x42);
        
        // 复制
        memory.bulk_copy(0, 100, 50).unwrap();
        assert_eq!(memory.read_u8(100).unwrap(), 0x42);
        assert_eq!(memory.read_u8(149).unwrap(), 0x42);
    }
    
    #[test]
    fn test_module_creation() {
        let mut module = WebAssembly3Module::new("test");
        module.enable_feature(WebAssembly3Feature::Memory64);
        module.init_memory64(1, Some(10));
        
        assert!(module.supports_feature(&WebAssembly3Feature::Memory64));
        assert!(module.memory64.is_some());
    }
    
    #[test]
    fn test_gc_types() {
        let struct_type = GcType::Struct(GcStruct {
            name: "Point".to_string(),
            fields: vec![
                GcField { name: "x".to_string(), value_type: ValueType::F64, mutable: true },
                GcField { name: "y".to_string(), value_type: ValueType::F64, mutable: true },
            ],
            super_type: None,
        });
        
        let array_type = GcType::Array(GcArray {
            element_type: ValueType::I32,
            mutable: true,
            min_size: 0,
            max_size: Some(1000),
        });
        
        let mut module = WebAssembly3Module::new("gc_test");
        module.add_gc_type(struct_type);
        module.add_gc_type(array_type);
        
        assert_eq!(module.gc_types.len(), 2);
    }
    
    #[test]
    fn test_validation() {
        let mut module = WebAssembly3Module::new("validation_test");
        module.enable_feature(WebAssembly3Feature::ExceptionHandlingExnref);
        // 故意不添加 ReferenceTypes，验证应该失败
        
        let result = module.validate();
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }
    
    #[test]
    fn test_runtime_gc() {
        let mut runtime = WebAssembly3Runtime::new();
        
        let struct_type = GcType::Struct(GcStruct {
            name: "Test".to_string(),
            fields: vec![],
            super_type: None,
        });
        
        let id = runtime.allocate_gc_object(struct_type, vec![1, 2, 3]);
        assert_eq!(id, 1);
        
        assert!(runtime.gc_heap.objects.contains_key(&id));
    }
}
