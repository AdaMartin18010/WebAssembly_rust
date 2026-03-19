//! # WebAssembly 3.0 + Rust 1.94 完整集成库
//!
//! 本库提供了 Rust 1.94 新特性与 WebAssembly 3.0 的完整集成实现。
//! 包含 WASI 0.3、Component Model、WasmGC、Memory64 等最新特性。
//!
//! This library provides complete integration of:
//! - Rust 1.94 new features (array_windows, SIMD FP16, enhanced LazyLock)
//! - WebAssembly 3.0 (WasmGC, Memory64, Exception Handling with exnref)
//! - WASI 0.3 (native async/await, `stream<T>`, `future<T>`)
//! - Component Model (WIT definitions, polyglot composition)

pub mod common;
pub mod types;
pub mod rust_189_features;
pub mod rust_194_features;      // 新增: Rust 1.94 特性
pub mod error_handling;
pub mod webassembly_2_0;
pub mod webassembly_3_0;        // 新增: WebAssembly 3.0 支持
pub mod wasi_03;                // 新增: WASI 0.3 支持
pub mod component_model;        // 新增: Component Model 支持
pub mod security_advanced;
pub mod developer_tools;
pub mod monitoring_advanced;
pub mod api_gateway;
pub mod intelligent_caching;
pub mod module_marketplace;
pub mod ai_optimization;
pub mod edge_computing;
pub mod blockchain_web3;
pub mod quantum_computing;
pub mod global_cdn;

// 重新导出公共组件
pub use common::{
    WasmError, WasmResult, ErrorSeverity,
    PerformanceStats, PerformanceMonitor, PerformanceTimer,
    Timestamp, TimeRange, TimeWindow, TimeSeries,
    ConfigManager, AppConfig, ConfigBuilder,
    StructuredLogger, LogLevel, LogEntry,
    Serializer, SerializationFormat, SerializationCache
};

// 重新导出主要类型和功能
pub use types::{
    Value, ValueType, Module, Function, FunctionType, Memory, Table, 
    Instruction as TypesInstruction, BulkMemoryOperations, TailCall, HostBinding, 
    HostBindingType, InterfaceType, RecordField, ValidationError as TypesValidationError
};

// 重新导出 Rust 1.90 特性 (向后兼容)
pub use rust_189_features::{
    WasmArrayBuilder, BulkMemoryManager, TailCallOptimizer, 
    HostBindingManager, InterfaceTypeHandler, SimdProcessor, 
    SimdInstruction, Rust190Wasm2Integration, TestResult
};

// 重新导出 Rust 1.94 新特性
pub use rust_194_features::{
    MemoryOptimizer,
    ModuleCache, ModuleMetadata,
    GlobalConfig,
    WASM_MODULE_CACHE, GLOBAL_CONFIG,
    DemoResults, Rust194Demo,
    math_constants,
    enhanced_iterators,
    string_utils,
    char_to_usize,
    Rust194Error,
};

// 重新导出 WebAssembly 2.0 特性 (向后兼容)
pub use webassembly_2_0::{
    WebAssembly2Module, WebAssembly2Function, WebAssembly2Runtime,
    WebAssembly2Features, WebAssembly2Instruction, StringEncoding,
    ExceptionHandler, ExceptionType, ReferenceType as W2ReferenceType, 
    Component as W2Component, WebAssembly2Error
};

// 重新导出 WebAssembly 3.0 新特性
pub use webassembly_3_0::{
    WebAssembly3Module, WebAssembly3Function, WebAssembly3Runtime,
    WebAssembly3Feature,
    Memory64, Memory64Error, WASM_PAGE_SIZE_64,
    GcType, GcStruct, GcArray, GcField,
    ExnrefHandler, ExnrefTag, CatchBlock,
    Instruction as W3Instruction, BlockType, CatchClause,
    ModuleId, Export, ExportKind, Import, ImportKind,
    ValidationResult, ValidationError as W3ValidationError,
};

// 重新导出 WASI 0.3 特性
pub use wasi_03::{
    Wasi03Runtime, RuntimeConfig,
    StreamWriter, StreamReader, StreamError,
    FutureCompleter, WasiFuture,
    CancellationToken,
    Wasi03Error,
    http::{self, HttpClient, HttpRequest, HttpResponse, HttpMethod},
    filesystem::{self, File},
    timer::{self, Timer, Interval, Timeout, TimeoutError},
    network::{self, TcpListener, TcpStream, UdpSocket},
    demo,
};

// 重新导出 Component Model 特性
pub use component_model::{
    Component, ComponentId,
    ComponentImport, ComponentExport,
    ComponentComposer, ComposedComponent, CompositionGraph,
    CompositionError,
    WitInterface, WitFunction, WitParam, WitResults,
    WitTypeDef, WitType, ResourceType,
    WitParser, WitParseError,
    examples as component_examples,
};

pub use error_handling::{
    WebAssemblyError, RecoveryStrategy, ErrorHandler, 
    ErrorStatistics, ErrorLogEntry
};

pub use security_advanced::{
    AdvancedSecurityManager, SecurityPolicy, SecurityLevel, ThreatType,
    SecurityEvent, SecuritySeverity, ThreatDetector, SecurityContext,
    MemoryMonitor, ExecutionMonitor, SecurityStatistics
};

pub use developer_tools::{
    DeveloperToolsManager, CodeGenerator, WasmDebugger, WasmProfiler,
    WasmTestFramework, DocGenerator, ProjectManager, ModuleSpecification,
    GeneratedCode, PerformanceReport, TestSuiteResult
};

pub use monitoring_advanced::{
    AdvancedMonitoringManager, MetricsCollector, DistributedTracer,
    AlertManager, PerformanceAnalyzer, HealthChecker
};

pub use api_gateway::{
    ApiGatewayManager, Route, LoadBalancer, RateLimiter, Cache,
    HttpMethod as ApiHttpMethod, Request, Response
};

pub use intelligent_caching::{
    IntelligentCacheManager, PerformanceOptimizer, CachePolicy,
    EvictionPolicy, CompressionPolicy, OptimizationStrategy
};

pub use module_marketplace::{
    ModuleMarketplaceManager, ModuleEntry, ModuleCategory,
    UserManager, RatingSystem, SearchQuery, SortBy
};

pub use ai_optimization::{
    AiOptimizationEngine, MachineLearningModel, NeuralNetworkModel,
    OptimizationContext, OptimizationResult, TrainingDataPoint
};

pub use edge_computing::{
    EdgeComputingManager, EdgeNode, EdgeTask, TaskScheduler,
    ResourceManager, NetworkManager, GeographicLocation
};

pub use blockchain_web3::{
    BlockchainManager, BlockchainNetwork, SmartContract,
    WalletManager, TransactionManager, NetworkType
};

pub use quantum_computing::{
    QuantumComputingManager, QuantumProcessor, QuantumAlgorithm,
    QuantumCircuit, QuantumSimulator, QuantumResult
};

pub use global_cdn::{
    GlobalCdnManager, CdnNode, ContentDistributor, CdnCacheManager,
    CdnLoadBalancer, CdnMonitoringSystem
};

/// 库版本信息
pub const VERSION: &str = "0.2.0";
pub const RUST_VERSION: &str = "1.94";
pub const WEBASSEMBLY_VERSION: &str = "3.0";
pub const WASI_VERSION: &str = "0.3";

/// 特性标志检查
pub mod features {
    /// 检查是否启用了 WebAssembly 3.0 特性
    #[cfg(feature = "webassembly-3-0")]
    pub const WEBASSEMBLY_3_0: bool = true;
    
    #[cfg(not(feature = "webassembly-3-0"))]
    pub const WEBASSEMBLY_3_0: bool = false;
    
    /// 检查是否启用了 WASI 0.3 特性
    #[cfg(feature = "wasi-03")]
    pub const WASI_03: bool = true;
    
    #[cfg(not(feature = "wasi-03"))]
    pub const WASI_03: bool = false;
    
    /// 检查是否启用了 Rust 1.94 特性
    #[cfg(feature = "rust-194")]
    pub const RUST_194: bool = true;
    
    #[cfg(not(feature = "rust-194"))]
    pub const RUST_194: bool = false;
}

/// 快速启动函数 - 初始化运行时
pub fn initialize() -> Result<(), WasmError> {
    // 初始化日志
    let _ = env_logger::try_init();
    
    log::info!(
        "WebAssembly 3.0 + Rust 1.94 Runtime initialized (v{})",
        VERSION
    );
    
    #[cfg(feature = "webassembly-3-0")]
    log::info!("WebAssembly 3.0 features enabled");
    
    #[cfg(feature = "wasi-03")]
    log::info!("WASI 0.3 features enabled");
    
    #[cfg(feature = "rust-194")]
    log::info!("Rust 1.94 features enabled");
    
    Ok(())
}

/// 运行所有演示
#[cfg(all(feature = "rust-194", feature = "webassembly-3-0"))]
pub fn run_all_demos() -> Result<(), Box<dyn std::error::Error>> {
    // Rust 1.94 演示
    let demo = rust_194_features::Rust194Demo::new();
    let results = demo.run_all_demos();
    
    if results.all_success() {
        println!("✅ All Rust 1.94 demos passed!");
    } else {
        eprintln!("❌ Some demos failed: {:?}", results.failures);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_info() {
        assert_eq!(VERSION, "0.2.0");
        assert_eq!(RUST_VERSION, "1.94");
        assert_eq!(WEBASSEMBLY_VERSION, "3.0");
        assert_eq!(WASI_VERSION, "0.3");
    }
    
    #[test]
    fn test_feature_flags() {
        // 这些测试根据编译特性标志而变化
        println!("WebAssembly 3.0: {}", features::WEBASSEMBLY_3_0);
        println!("WASI 0.3: {}", features::WASI_03);
        println!("Rust 1.94: {}", features::RUST_194);
    }
    
    #[test]
    fn test_initialize() {
        let result = initialize();
        assert!(result.is_ok());
    }
}
