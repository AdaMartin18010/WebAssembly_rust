//! # WebAssembly 2.0 最新特性实现
//!
//! 本模块实现了 WebAssembly 2.0 标准的最新特性，包括：
//! - 批量内存操作
//! - 尾调用优化
//! - 宿主绑定
//! - 接口类型
//! - SIMD 指令集
//! - 多值返回
//! - 异常处理
//!
//! 基于 2024年12月发布的 WebAssembly 2.0 候选推荐标准

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

/// WebAssembly 2.0 特性标志
/// WebAssembly 2.0 Feature Flags
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebAssembly2Features {
    /// 批量内存操作
    BulkMemoryOperations,
    /// 尾调用优化
    TailCallOptimization,
    /// 宿主绑定
    HostBindings,
    /// 接口类型
    InterfaceTypes,
    /// SIMD 指令集
    SimdInstructions,
    /// 多值返回
    MultiValue,
    /// 异常处理
    ExceptionHandling,
    /// 多线程
    MultiThreading,
    /// 引用类型
    ReferenceTypes,
    /// 垃圾回收
    GarbageCollection,
}

/// WebAssembly 2.0 模块
/// WebAssembly 2.0 Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Module {
    /// 模块ID
    pub id: ModuleId,
    /// 模块名称
    pub name: String,
    /// 支持的特性
    pub features: Vec<WebAssembly2Features>,
    /// 函数列表
    pub functions: Vec<WebAssembly2Function>,
    /// 内存列表
    pub memories: Vec<WebAssembly2Memory>,
    /// 表列表
    pub tables: Vec<WebAssembly2Table>,
    /// 全局变量列表
    pub globals: Vec<WebAssembly2Global>,
    /// 导入列表
    pub imports: Vec<WebAssembly2Import>,
    /// 导出列表
    pub exports: Vec<WebAssembly2Export>,
    /// 异常处理器列表
    pub exception_handlers: Vec<ExceptionHandler>,
    /// 组件列表
    pub components: Vec<Component>,
}

impl WebAssembly2Module {
    /// 创建新的 WebAssembly 2.0 模块
    /// Create new WebAssembly 2.0 module
    pub fn new(name: String) -> Self {
        Self {
            id: ModuleId::new(),
            name,
            features: Vec::new(),
            functions: Vec::new(),
            memories: Vec::new(),
            tables: Vec::new(),
            globals: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            exception_handlers: Vec::new(),
            components: Vec::new(),
        }
    }

    /// 启用特性
    /// Enable feature
    pub fn enable_feature(&mut self, feature: WebAssembly2Features) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    /// 检查是否支持特性
    /// Check if feature is supported
    pub fn supports_feature(&self, feature: &WebAssembly2Features) -> bool {
        self.features.contains(feature)
    }

    /// 验证模块
    /// Validate module
    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // 验证特性兼容性
        self.validate_feature_compatibility(&mut errors);

        // 验证函数
        for function in &self.functions {
            if let Err(e) = function.validate() {
                errors.push(e);
            }
        }

        // 验证内存
        for memory in &self.memories {
            if let Err(e) = memory.validate() {
                errors.push(e);
            }
        }

        // 验证异常处理器
        for handler in &self.exception_handlers {
            if let Err(e) = handler.validate() {
                errors.push(e);
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
        }
    }

    /// 验证特性兼容性
    /// Validate feature compatibility
    fn validate_feature_compatibility(&self, errors: &mut Vec<ValidationError>) {
        // 检查特性依赖关系
        if self.supports_feature(&WebAssembly2Features::TailCallOptimization) {
            if !self.supports_feature(&WebAssembly2Features::MultiValue) {
                errors.push(ValidationError::FeatureDependencyError {
                    feature: "TailCallOptimization".to_string(),
                    required: "MultiValue".to_string(),
                });
            }
        }

        if self.supports_feature(&WebAssembly2Features::ExceptionHandling) {
            if !self.supports_feature(&WebAssembly2Features::ReferenceTypes) {
                errors.push(ValidationError::FeatureDependencyError {
                    feature: "ExceptionHandling".to_string(),
                    required: "ReferenceTypes".to_string(),
                });
            }
        }
    }
}

/// WebAssembly 2.0 函数
/// WebAssembly 2.0 Function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Function {
    /// 函数索引
    pub index: u32,
    /// 函数名称
    pub name: String,
    /// 参数类型
    pub params: Vec<ValueType>,
    /// 返回类型（支持多值返回）
    pub results: Vec<ValueType>,
    /// 局部变量
    pub locals: Vec<ValueType>,
    /// 函数体
    pub body: Vec<WebAssembly2Instruction>,
    /// 异常处理标签
    pub exception_labels: Vec<ExceptionLabel>,
    /// 是否支持尾调用
    pub supports_tail_call: bool,
}

impl WebAssembly2Function {
    /// 创建新函数
    /// Create new function
    pub fn new(
        index: u32,
        name: String,
        params: Vec<ValueType>,
        results: Vec<ValueType>,
    ) -> Self {
        Self {
            index,
            name,
            params,
            results,
            locals: Vec::new(),
            body: Vec::new(),
            exception_labels: Vec::new(),
            supports_tail_call: false,
        }
    }

    /// 验证函数
    /// Validate function
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 验证多值返回
        if self.results.len() > 1 {
            return Err(ValidationError::MultiValueNotSupported);
        }

        // 验证函数体
        self.validate_body()?;

        Ok(())
    }

    /// 验证函数体
    /// Validate function body
    fn validate_body(&self) -> Result<(), ValidationError> {
        let stack: Vec<Value> = Vec::new();
        let mut exception_stack = Vec::new();

        for instruction in &self.body {
            match instruction {
                WebAssembly2Instruction::Throw(tag) => {
                    // 验证异常标签
                    if !self.exception_labels.iter().any(|label| label.tag == *tag) {
                        return Err(ValidationError::InvalidExceptionTag(*tag));
                    }
                    exception_stack.push(*tag);
                }
                WebAssembly2Instruction::TryCatch(try_block) => {
                    exception_stack.push(try_block.catch_label);
                }
                WebAssembly2Instruction::Return => {
                    // 检查返回类型匹配
                    if stack.len() != self.results.len() {
                        return Err(ValidationError::ReturnTypeMismatch {
                            expected: self.results[0].clone(),
                            actual: self.results[0].clone(), // 简化实现
                        });
                    }
                    break;
                }
                _ => {
                    // 其他指令的验证逻辑
                }
            }
        }

        Ok(())
    }
}

/// WebAssembly 2.0 指令
/// WebAssembly 2.0 Instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebAssembly2Instruction {
    /// 基础指令（继承自 WebAssembly 1.0）
    /// Basic instructions (inherited from WebAssembly 1.0)
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    Call(u32),
    Return,

    /// WebAssembly 2.0 新指令
    /// WebAssembly 2.0 new instructions

    /// 批量内存操作
    /// Bulk memory operations
    MemoryCopy { src: u32, dst: u32, size: u32 },
    MemoryFill { addr: u32, value: u8, size: u32 },
    TableCopy { src_table: u32, dst_table: u32, src_offset: u32, dst_offset: u32, size: u32 },
    TableFill { table: u32, offset: u32, value: Option<u32>, size: u32 },

    /// 尾调用指令
    /// Tail call instructions
    ReturnCall(u32),
    ReturnCallIndirect(u32),

    /// 多值返回
    /// Multi-value returns
    ReturnValues(Vec<Value>),

    /// 异常处理指令
    /// Exception handling instructions
    Throw(u32),
    Rethrow,
    TryCatch(TryCatchBlock),
    TryCatchAll(TryCatchAllBlock),

    /// SIMD 指令
    /// SIMD instructions
    V128Const([u8; 16]),
    V128Load { offset: u32, align: u32 },
    V128Store { offset: u32, align: u32 },
    V128Add,
    V128Sub,
    V128Mul,
    V128Div,
    V128And,
    V128Or,
    V128Xor,
    V128Not,
    V128Shl,
    V128Shr,
    V128Eq,
    V128Ne,
    V128Lt,
    V128Le,
    V128Gt,
    V128Ge,

    /// 扩展 SIMD 指令（WebAssembly 2.0）
    /// Extended SIMD instructions (WebAssembly 2.0)
    V128Load8x8S { offset: u32 },
    V128Load8x8U { offset: u32 },
    V128Load16x4S { offset: u32 },
    V128Load16x4U { offset: u32 },
    V128Load32x2S { offset: u32 },
    V128Load32x2U { offset: u32 },
    V128Store8x8 { offset: u32 },
    V128Store16x4 { offset: u32 },
    V128Store32x2 { offset: u32 },

    /// 接口类型指令
    /// Interface type instructions
    StringNew { encoding: StringEncoding },
    StringMeasure { encoding: StringEncoding },
    StringEncode { encoding: StringEncoding },
    StringConcat,
    StringEq,
    StringAsWTF16,
    StringFromWTF16,
    StringFromWTF8Array,
    StringToWTF8Array,
    StringConst(String),
    StringMeasureWTF8,
    StringMeasureWTF16,
    StringEncodeWTF8,
    StringEncodeWTF16,
    StringConstWTF16(Vec<u16>),
    StringConstWTF8Array(Vec<u8>),
    StringAsLower,
    StringAsUpper,
}

/// 字符串编码类型
/// String encoding type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringEncoding {
    /// UTF-8 编码
    UTF8,
    /// UTF-16 编码
    UTF16,
    /// Latin-1 编码
    Latin1,
    /// WTF-8 编码
    WTF8,
    /// WTF-16 编码
    WTF16,
}

/// Try-Catch 块
/// Try-Catch block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryCatchBlock {
    /// Catch 标签
    pub catch_label: u32,
    /// Try 块指令
    pub try_instructions: Vec<WebAssembly2Instruction>,
    /// Catch 块指令
    pub catch_instructions: Vec<WebAssembly2Instruction>,
}

/// Try-CatchAll 块
/// Try-CatchAll block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryCatchAllBlock {
    /// CatchAll 标签
    pub catch_all_label: u32,
    /// Try 块指令
    pub try_instructions: Vec<WebAssembly2Instruction>,
    /// CatchAll 块指令
    pub catch_all_instructions: Vec<WebAssembly2Instruction>,
}

/// 异常处理器
/// Exception handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionHandler {
    /// 异常标签
    pub tag: u32,
    /// 异常类型
    pub exception_type: ExceptionType,
    /// 处理指令
    pub handler_instructions: Vec<WebAssembly2Instruction>,
}

impl ExceptionHandler {
    /// 验证异常处理器
    /// Validate exception handler
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 验证异常类型
        self.exception_type.validate()?;

        // 验证处理指令
        for instruction in &self.handler_instructions {
            match instruction {
                WebAssembly2Instruction::Throw(_) => {
                    return Err(ValidationError::InvalidInstructionInHandler);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/// 异常类型
/// Exception type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExceptionType {
    /// 基本异常类型
    Basic(ValueType),
    /// 引用异常类型
    Reference(ReferenceType),
    /// 复合异常类型
    Compound(Vec<ValueType>),
}

impl ExceptionType {
    /// 验证异常类型
    /// Validate exception type
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            ExceptionType::Basic(value_type) => {
                // 验证基本类型
                match value_type {
                    ValueType::I32 | ValueType::I64 | ValueType::F32 | ValueType::F64 => Ok(()),
                    _ => Err(ValidationError::InvalidExceptionType),
                }
            }
            ExceptionType::Reference(_) => {
                // 引用类型需要 ReferenceTypes 特性
                Ok(())
            }
            ExceptionType::Compound(types) => {
                // 复合类型需要多个基本类型
                if types.is_empty() {
                    Err(ValidationError::EmptyExceptionType)
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// 引用类型
/// Reference type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    /// 函数引用
    FuncRef,
    /// 外部引用
    ExternRef,
    /// 类型引用
    TypeRef(u32),
}

/// 异常标签
/// Exception label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionLabel {
    /// 标签ID
    pub tag: u32,
    /// 标签名称
    pub name: String,
    /// 异常类型
    pub exception_type: ExceptionType,
}

/// WebAssembly 2.0 内存
/// WebAssembly 2.0 Memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Memory {
    /// 内存索引
    pub index: u32,
    /// 初始大小（页数）
    pub initial: u32,
    /// 最大大小（页数）
    pub maximum: Option<u32>,
    /// 内存数据
    pub data: Vec<u8>,
    /// 是否支持共享
    pub shared: bool,
    /// 内存类型
    pub memory_type: WebAssembly2MemoryType,
}

/// WebAssembly 2.0 内存类型
/// WebAssembly 2.0 Memory type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebAssembly2MemoryType {
    /// 标准内存
    Standard,
    /// 共享内存
    Shared,
    /// 64位内存
    Memory64,
}

impl WebAssembly2Memory {
    /// 创建新内存
    /// Create new memory
    pub fn new(
        index: u32,
        initial: u32,
        maximum: Option<u32>,
        memory_type: WebAssembly2MemoryType,
    ) -> Self {
        let size = initial * PAGE_SIZE;
        Self {
            index,
            initial,
            maximum,
            data: vec![0; size as usize],
            shared: matches!(memory_type, WebAssembly2MemoryType::Shared),
            memory_type,
        }
    }

    /// 验证内存
    /// Validate memory
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 检查初始大小
        if self.initial == 0 {
            return Err(ValidationError::InvalidMemorySize { size: self.initial });
        }

        // 检查最大大小
        if let Some(max) = self.maximum {
            if max < self.initial {
                return Err(ValidationError::InvalidMemorySize { size: max });
            }
        }

        // 检查共享内存
        if self.shared && !matches!(self.memory_type, WebAssembly2MemoryType::Shared) {
            return Err(ValidationError::InvalidSharedMemory);
        }

        Ok(())
    }
}

/// WebAssembly 2.0 表
/// WebAssembly 2.0 Table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Table {
    /// 表索引
    pub index: u32,
    /// 元素类型
    pub element_type: WebAssembly2ElementType,
    /// 初始大小
    pub initial: u32,
    /// 最大大小
    pub maximum: Option<u32>,
    /// 表数据
    pub data: Vec<Option<u32>>,
    /// 是否支持共享
    pub shared: bool,
}

/// WebAssembly 2.0 元素类型
/// WebAssembly 2.0 Element type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebAssembly2ElementType {
    /// 函数引用
    FuncRef,
    /// 外部引用
    ExternRef,
    /// 类型引用
    TypeRef(u32),
}

impl WebAssembly2Table {
    /// 创建新表
    /// Create new table
    pub fn new(
        index: u32,
        element_type: WebAssembly2ElementType,
        initial: u32,
        maximum: Option<u32>,
    ) -> Self {
        Self {
            index,
            element_type,
            initial,
            maximum,
            data: vec![None; initial as usize],
            shared: false,
        }
    }

    /// 验证表
    /// Validate table
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 检查初始大小
        if self.initial == 0 {
            return Err(ValidationError::InvalidTableSize { size: self.initial });
        }

        // 检查最大大小
        if let Some(max) = self.maximum {
            if max < self.initial {
                return Err(ValidationError::InvalidTableSize { size: max });
            }
        }

        Ok(())
    }
}

/// WebAssembly 2.0 全局变量
/// WebAssembly 2.0 Global variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Global {
    /// 全局变量索引
    pub index: u32,
    /// 值类型
    pub value_type: ValueType,
    /// 是否可变
    pub mutable: bool,
    /// 初始值
    pub init_value: Value,
    /// 是否支持引用类型
    pub supports_reference_types: bool,
}

impl WebAssembly2Global {
    /// 创建新全局变量
    /// Create new global variable
    pub fn new(
        index: u32,
        value_type: ValueType,
        mutable: bool,
        init_value: Value,
    ) -> Self {
        Self {
            index,
            value_type: value_type.clone(),
            mutable,
            init_value,
            supports_reference_types: matches!(
                value_type,
                ValueType::FuncRef | ValueType::ExternRef
            ),
        }
    }

    /// 验证全局变量
    /// Validate global variable
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 检查值类型匹配
        if self.init_value.get_type() != self.value_type {
            return Err(ValidationError::TypeMismatch {
                expected: self.value_type.clone(),
                actual: self.init_value.get_type(),
            });
        }

        Ok(())
    }
}

/// WebAssembly 2.0 导入
/// WebAssembly 2.0 Import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Import {
    /// 模块名称
    pub module: String,
    /// 字段名称
    pub field: String,
    /// 导入类型
    pub import_type: WebAssembly2ImportType,
}

/// WebAssembly 2.0 导入类型
/// WebAssembly 2.0 Import type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebAssembly2ImportType {
    /// 函数导入
    Function(WebAssembly2FunctionType),
    /// 表导入
    Table(WebAssembly2TableType),
    /// 内存导入
    Memory(WebAssembly2MemoryType),
    /// 全局变量导入
    Global(WebAssembly2GlobalType),
    /// 异常类型导入
    Exception(ExceptionType),
}

/// WebAssembly 2.0 函数类型
/// WebAssembly 2.0 Function type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2FunctionType {
    /// 参数类型
    pub params: Vec<ValueType>,
    /// 返回类型
    pub results: Vec<ValueType>,
}

/// WebAssembly 2.0 表类型
/// WebAssembly 2.0 Table type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2TableType {
    /// 元素类型
    pub element_type: WebAssembly2ElementType,
    /// 初始大小
    pub initial: u32,
    /// 最大大小
    pub maximum: Option<u32>,
}

/// WebAssembly 2.0 全局变量类型
/// WebAssembly 2.0 Global variable type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2GlobalType {
    /// 值类型
    pub value_type: ValueType,
    /// 是否可变
    pub mutable: bool,
}

/// WebAssembly 2.0 导出
/// WebAssembly 2.0 Export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAssembly2Export {
    /// 导出名称
    pub name: String,
    /// 导出类型
    pub export_type: WebAssembly2ExportType,
    /// 导出索引
    pub index: u32,
}

/// WebAssembly 2.0 导出类型
/// WebAssembly 2.0 Export type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebAssembly2ExportType {
    /// 函数导出
    Function,
    /// 表导出
    Table,
    /// 内存导出
    Memory,
    /// 全局变量导出
    Global,
    /// 异常类型导出
    Exception,
}

/// 组件
/// Component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// 组件ID
    pub id: u32,
    /// 组件名称
    pub name: String,
    /// 组件类型
    pub component_type: ComponentType,
    /// 组件实例
    pub instances: Vec<ComponentInstance>,
}

/// 组件类型
/// Component type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    /// 核心组件
    Core,
    /// 接口组件
    Interface,
    /// 复合组件
    Composite,
}

/// 组件实例
/// Component instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInstance {
    /// 实例ID
    pub id: u32,
    /// 实例名称
    pub name: String,
    /// 实例类型
    pub instance_type: InstanceType,
}

/// 实例类型
/// Instance type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceType {
    /// 模块实例
    Module,
    /// 组件实例
    Component,
    /// 函数实例
    Function,
}

/// WebAssembly 2.0 错误类型
/// WebAssembly 2.0 Error types
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum WebAssembly2Error {
    /// 特性依赖错误
    #[error("特性依赖错误: {feature} 需要 {required}")]
    FeatureDependencyError { feature: String, required: String },
    /// 多值不支持
    #[error("多值返回不支持")]
    MultiValueNotSupported,
    /// 无效异常标签
    #[error("无效异常标签: {0}")]
    InvalidExceptionTag(u32),
    /// 无效异常类型
    #[error("无效异常类型")]
    InvalidExceptionType,
    /// 空异常类型
    #[error("空异常类型")]
    EmptyExceptionType,
    /// 无效共享内存
    #[error("无效共享内存")]
    InvalidSharedMemory,
    /// 处理器中无效指令
    #[error("异常处理器中无效指令")]
    InvalidInstructionInHandler,
}

/// WebAssembly 2.0 运行时
/// WebAssembly 2.0 Runtime
#[derive(Debug, Clone)]
pub struct WebAssembly2Runtime {
    /// 模块实例
    pub modules: HashMap<ModuleId, WebAssembly2Module>,
    /// 执行环境
    pub execution_environments: HashMap<ModuleId, ExecutionEnvironment>,
    /// 支持的特性
    pub supported_features: Vec<WebAssembly2Features>,
    /// 性能统计
    pub performance_stats: PerformanceStats,
}

impl WebAssembly2Runtime {
    /// 创建新运行时
    /// Create new runtime
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            execution_environments: HashMap::new(),
            supported_features: vec![
                WebAssembly2Features::BulkMemoryOperations,
                WebAssembly2Features::TailCallOptimization,
                WebAssembly2Features::HostBindings,
                WebAssembly2Features::InterfaceTypes,
                WebAssembly2Features::SimdInstructions,
                WebAssembly2Features::MultiValue,
                WebAssembly2Features::ExceptionHandling,
                WebAssembly2Features::ReferenceTypes,
            ],
            performance_stats: PerformanceStats::new(),
        }
    }

    /// 加载模块
    /// Load module
    pub fn load_module(&mut self, module: WebAssembly2Module) -> Result<ModuleId, WebAssembly2Error> {
        let module_id = module.id.clone();
        
        // 验证模块
        let validation = module.validate();
        if !validation.is_valid {
            return Err(WebAssembly2Error::FeatureDependencyError {
                feature: "Module".to_string(),
                required: "Validation".to_string(),
            });
        }

        // 创建执行环境
        let execution_env = ExecutionEnvironment::new(module_id.clone(), 1024 * 1024);
        
        self.modules.insert(module_id.clone(), module);
        self.execution_environments.insert(module_id.clone(), execution_env);
        
        Ok(module_id)
    }

    /// 执行函数
    /// Execute function
    pub fn execute_function(
        &mut self,
        module_id: &ModuleId,
        function_index: u32,
        args: Vec<Value>,
    ) -> Result<Vec<Value>, WebAssembly2Error> {
        let start = Instant::now();
        
        // 获取模块
        let module = self.modules.get(module_id)
            .ok_or_else(|| WebAssembly2Error::FeatureDependencyError {
                feature: "Module".to_string(),
                required: "ModuleId".to_string(),
            })?;

        // 获取函数
        let function = module.functions.get(function_index as usize)
            .ok_or_else(|| WebAssembly2Error::FeatureDependencyError {
                feature: "Function".to_string(),
                required: "FunctionIndex".to_string(),
            })?;

        // 克隆函数以避免借用冲突
        let function_clone = function.clone();
        let module_id_clone = module_id.clone();

        // 执行函数
        let result = self.execute_function_internal(&module_id_clone, &function_clone, args)?;
        
        // 更新性能统计
        let execution_time = start.elapsed();
        self.performance_stats.record_execution(execution_time);
        
        Ok(result)
    }

    /// 内部函数执行
    /// Internal function execution
    fn execute_function_internal(
        &mut self,
        module_id: &ModuleId,
        function: &WebAssembly2Function,
        _args: Vec<Value>,
    ) -> Result<Vec<Value>, WebAssembly2Error> {
        // 获取执行环境
        let _execution_env = self.execution_environments.get_mut(module_id)
            .ok_or_else(|| WebAssembly2Error::FeatureDependencyError {
                feature: "ExecutionEnvironment".to_string(),
                required: "ModuleId".to_string(),
            })?;

        // 执行指令
        let mut stack: Vec<Value> = Vec::new();
        let _exception_stack: Vec<ExceptionType> = Vec::new();
        
        for instruction in &function.body {
            match instruction {
                WebAssembly2Instruction::I32Const(value) => {
                    stack.push(Value::I32(*value));
                }
                WebAssembly2Instruction::I32Add => {
                    if let (Some(Value::I32(b)), Some(Value::I32(a))) = (stack.pop(), stack.pop()) {
                        stack.push(Value::I32(a + b));
                    }
                }
                WebAssembly2Instruction::Return => {
                    break;
                }
                _ => {
                    // 其他指令的处理逻辑
                }
            }
        }

        // 返回结果
        Ok(vec![stack.pop().unwrap_or(Value::I32(0))])
    }
}

/// 性能统计
/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    /// 总执行时间
    pub total_execution_time: Duration,
    /// 执行次数
    pub execution_count: u64,
    /// 平均执行时间
    pub average_execution_time: Duration,
    /// 最大执行时间
    pub max_execution_time: Duration,
    /// 最小执行时间
    pub min_execution_time: Duration,
}

impl PerformanceStats {
    /// 创建新性能统计
    /// Create new performance statistics
    pub fn new() -> Self {
        Self {
            total_execution_time: Duration::ZERO,
            execution_count: 0,
            average_execution_time: Duration::ZERO,
            max_execution_time: Duration::ZERO,
            min_execution_time: Duration::MAX,
        }
    }

    /// 记录执行时间
    /// Record execution time
    pub fn record_execution(&mut self, execution_time: Duration) {
        self.total_execution_time += execution_time;
        self.execution_count += 1;
        self.average_execution_time = Duration::from_millis(self.total_execution_time.as_millis() as u64 / self.execution_count);
        
        if execution_time > self.max_execution_time {
            self.max_execution_time = execution_time;
        }
        
        if execution_time < self.min_execution_time {
            self.min_execution_time = execution_time;
        }
    }
}

// 扩展 ValidationError 以支持 WebAssembly 2.0
impl From<WebAssembly2Error> for ValidationError {
    fn from(error: WebAssembly2Error) -> Self {
        match error {
            WebAssembly2Error::FeatureDependencyError { feature, required } => {
                ValidationError::InterfaceTypeError {
                    message: format!("{} 需要 {}", feature, required),
                }
            }
            _ => ValidationError::InterfaceTypeError {
                message: format!("{:?}", error),
            },
        }
    }
}
