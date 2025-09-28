//! # WebAssembly 2.0 + Rust 1.90 集成库
//!
//! 本库提供了 Rust 1.90 新特性与 WebAssembly 2.0 的完整集成实现。
//! This library provides complete integration of Rust 1.90 new features with WebAssembly 2.0.

pub mod common;
pub mod types;
pub mod rust_189_features;
pub mod error_handling;
pub mod webassembly_2_0;
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
// Re-export common components
pub use common::{
    WasmError, WasmResult, ErrorSeverity,
    PerformanceStats, PerformanceMonitor, PerformanceTimer,
    Timestamp, TimeRange, TimeWindow, TimeSeries,
    ConfigManager, AppConfig, ConfigBuilder,
    StructuredLogger, LogLevel, LogEntry,
    Serializer, SerializationFormat, SerializationCache
};

// 重新导出主要类型和功能
// Re-export main types and functions
pub use types::{
    Value, ValueType, Module, Function, FunctionType, Memory, Table, 
    Instruction, BulkMemoryOperations, TailCall, HostBinding, 
    HostBindingType, InterfaceType, RecordField, ValidationError
};

pub use rust_189_features::{
    WasmArrayBuilder, BulkMemoryManager, TailCallOptimizer, 
    HostBindingManager, InterfaceTypeHandler, SimdProcessor, 
    SimdInstruction, Rust190Wasm2Integration, TestResult
};

pub use error_handling::{
    WebAssemblyError, RecoveryStrategy, ErrorHandler, 
    ErrorStatistics, ErrorLogEntry
};

pub use webassembly_2_0::{
    WebAssembly2Module, WebAssembly2Function, WebAssembly2Runtime,
    WebAssembly2Features, WebAssembly2Instruction, StringEncoding,
    ExceptionHandler, ExceptionType, ReferenceType, Component,
    WebAssembly2Error
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
    HttpMethod, Request, Response
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
