//! # 统一错误处理模块 / Unified Error Handling Module
//!
//! 本模块提供了统一的错误处理框架，确保整个项目使用一致的错误处理模式。
//! This module provides a unified error handling framework to ensure consistent error handling patterns throughout the project.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// WebAssembly 统一错误类型 / WebAssembly Unified Error Type
///
/// 这是整个项目的统一错误类型，所有模块都应该使用这个错误类型。
/// This is the unified error type for the entire project. All modules should use this error type.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum WasmError {
    /// 模块错误 / Module Error
    #[error("模块错误: {0}")]
    Module(#[from] ModuleError),
    
    /// 运行时错误 / Runtime Error
    #[error("运行时错误: {0}")]
    Runtime(#[from] RuntimeError),
    
    /// 验证错误 / Validation Error
    #[error("验证错误: {0}")]
    Validation(#[from] ValidationError),
    
    /// 安全错误 / Security Error
    #[error("安全错误: {0}")]
    Security(#[from] SecurityError),
    
    /// AI优化错误 / AI Optimization Error
    #[error("AI优化错误: {0}")]
    AiOptimization(#[from] AiError),
    
    /// 区块链错误 / Blockchain Error
    #[error("区块链错误: {0}")]
    Blockchain(#[from] BlockchainError),
    
    /// 量子计算错误 / Quantum Computing Error
    #[error("量子计算错误: {0}")]
    Quantum(#[from] QuantumError),
    
    /// CDN错误 / CDN Error
    #[error("CDN错误: {0}")]
    Cdn(#[from] CdnError),
    
    /// 开发者工具错误 / Developer Tools Error
    #[error("开发者工具错误: {0}")]
    DeveloperTools(#[from] DeveloperToolsError),
    
    /// 监控错误 / Monitoring Error
    #[error("监控错误: {0}")]
    Monitoring(#[from] MonitoringError),
    
    /// API网关错误 / API Gateway Error
    #[error("API网关错误: {0}")]
    ApiGateway(#[from] ApiGatewayError),
    
    /// 缓存错误 / Cache Error
    #[error("缓存错误: {0}")]
    Cache(#[from] CacheError),
    
    /// 市场错误 / Marketplace Error
    #[error("市场错误: {0}")]
    Marketplace(#[from] MarketplaceError),
    
    /// 边缘计算错误 / Edge Computing Error
    #[error("边缘计算错误: {0}")]
    EdgeComputing(#[from] EdgeComputingError),
    
    /// 内部错误 / Internal Error
    #[error("内部错误: {message} (组件: {component})")]
    Internal { message: String, component: String },
    
    /// IO错误 / IO Error
    #[error("IO错误: {0}")]
    Io(String),
    
    /// 序列化错误 / Serialization Error
    #[error("序列化错误: {0}")]
    Serialization(String),
    
    /// 配置错误 / Configuration Error
    #[error("配置错误: {key} - {message}")]
    Configuration { key: String, message: String },
}

/// 模块错误 / Module Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ModuleError {
    #[error("模块未找到: {0}")]
    NotFound(String),
    
    #[error("模块加载失败: {0}")]
    LoadFailed(String),
    
    #[error("模块验证失败: {0}")]
    ValidationFailed(String),
    
    #[error("模块执行失败: {0}")]
    ExecutionFailed(String),
}

/// 运行时错误 / Runtime Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeError {
    #[error("内存错误: {0}")]
    Memory(String),
    
    #[error("类型错误: 期望 {expected}, 实际 {actual}")]
    Type { expected: String, actual: String },
    
    #[error("执行错误: {0}")]
    Execution(String),
    
    #[error("函数未找到: {0}")]
    FunctionNotFound(String),
    
    #[error("栈溢出")]
    StackOverflow,
    
    #[error("内存不足")]
    OutOfMemory,
}

/// 验证错误 / Validation Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("无效指令: {0}")]
    InvalidInstruction(String),
    
    #[error("类型不匹配: {0}")]
    TypeMismatch(String),
    
    #[error("内存访问越界")]
    MemoryOutOfBounds,
    
    #[error("函数签名不匹配")]
    FunctionSignatureMismatch,
}

/// 安全错误 / Security Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    #[error("访问被拒绝: {0}")]
    AccessDenied(String),
    
    #[error("权限不足: {0}")]
    InsufficientPermissions(String),
    
    #[error("安全策略违规: {0}")]
    PolicyViolation(String),
    
    #[error("威胁检测: {0}")]
    ThreatDetected(String),
}

/// AI优化错误 / AI Optimization Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AiError {
    #[error("模型加载失败: {0}")]
    ModelLoadFailed(String),
    
    #[error("训练失败: {0}")]
    TrainingFailed(String),
    
    #[error("预测失败: {0}")]
    PredictionFailed(String),
    
    #[error("数据不足: {0}")]
    InsufficientData(String),
}

/// 区块链错误 / Blockchain Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum BlockchainError {
    #[error("网络连接失败: {0}")]
    NetworkConnectionFailed(String),
    
    #[error("交易失败: {0}")]
    TransactionFailed(String),
    
    #[error("智能合约错误: {0}")]
    SmartContractError(String),
    
    #[error("钱包错误: {0}")]
    WalletError(String),
}

/// 量子计算错误 / Quantum Computing Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum QuantumError {
    #[error("量子处理器错误: {0}")]
    ProcessorError(String),
    
    #[error("量子电路错误: {0}")]
    CircuitError(String),
    
    #[error("量子算法错误: {0}")]
    AlgorithmError(String),
    
    #[error("量子模拟器错误: {0}")]
    SimulatorError(String),
}

/// CDN错误 / CDN Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CdnError {
    #[error("CDN节点错误: {0}")]
    NodeError(String),
    
    #[error("内容分发失败: {0}")]
    ContentDistributionFailed(String),
    
    #[error("缓存错误: {0}")]
    CacheError(String),
    
    #[error("负载均衡错误: {0}")]
    LoadBalancingError(String),
}

/// 开发者工具错误 / Developer Tools Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DeveloperToolsError {
    #[error("代码生成失败: {0}")]
    CodeGenerationFailed(String),
    
    #[error("调试器错误: {0}")]
    DebuggerError(String),
    
    #[error("性能分析错误: {0}")]
    ProfilerError(String),
    
    #[error("测试框架错误: {0}")]
    TestFrameworkError(String),
}

/// 监控错误 / Monitoring Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringError {
    #[error("指标收集失败: {0}")]
    MetricsCollectionFailed(String),
    
    #[error("日志记录失败: {0}")]
    LoggingFailed(String),
    
    #[error("告警系统错误: {0}")]
    AlertSystemError(String),
    
    #[error("健康检查失败: {0}")]
    HealthCheckFailed(String),
}

/// API网关错误 / API Gateway Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ApiGatewayError {
    #[error("路由错误: {0}")]
    RoutingError(String),
    
    #[error("负载均衡错误: {0}")]
    LoadBalancingError(String),
    
    #[error("限流错误: {0}")]
    RateLimitingError(String),
    
    #[error("缓存错误: {0}")]
    CacheError(String),
}

/// 缓存错误 / Cache Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CacheError {
    #[error("缓存未命中: {0}")]
    CacheMiss(String),
    
    #[error("缓存过期: {0}")]
    CacheExpired(String),
    
    #[error("缓存驱逐失败: {0}")]
    EvictionFailed(String),
    
    #[error("缓存配置错误: {0}")]
    ConfigurationError(String),
}

/// 市场错误 / Marketplace Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MarketplaceError {
    #[error("模块未找到: {0}")]
    ModuleNotFound(String),
    
    #[error("用户认证失败: {0}")]
    AuthenticationFailed(String),
    
    #[error("权限不足: {0}")]
    InsufficientPermissions(String),
    
    #[error("支付失败: {0}")]
    PaymentFailed(String),
}

/// 边缘计算错误 / Edge Computing Error
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum EdgeComputingError {
    #[error("边缘节点错误: {0}")]
    EdgeNodeError(String),
    
    #[error("任务调度失败: {0}")]
    TaskSchedulingFailed(String),
    
    #[error("资源管理错误: {0}")]
    ResourceManagementError(String),
    
    #[error("网络管理错误: {0}")]
    NetworkManagementError(String),
}

/// WebAssembly 结果类型 / WebAssembly Result Type
///
/// 这是整个项目的统一结果类型，所有函数都应该使用这个类型。
/// This is the unified result type for the entire project. All functions should use this type.
pub type WasmResult<T> = Result<T, WasmError>;

/// 错误严重程度 / Error Severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// 低 / Low
    Low,
    /// 中 / Medium
    Medium,
    /// 高 / High
    High,
    /// 严重 / Critical
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "低"),
            ErrorSeverity::Medium => write!(f, "中"),
            ErrorSeverity::High => write!(f, "高"),
            ErrorSeverity::Critical => write!(f, "严重"),
        }
    }
}

impl WasmError {
    /// 获取错误严重程度 / Get error severity
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            WasmError::Internal { .. } => ErrorSeverity::Critical,
            WasmError::Runtime(RuntimeError::StackOverflow) => ErrorSeverity::Critical,
            WasmError::Runtime(RuntimeError::OutOfMemory) => ErrorSeverity::Critical,
            WasmError::Security(_) => ErrorSeverity::High,
            WasmError::Validation(_) => ErrorSeverity::Medium,
            WasmError::Module(_) => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        }
    }
    
    /// 获取错误恢复建议 / Get error recovery suggestions
    pub fn recovery_suggestions(&self) -> Vec<String> {
        match self {
            WasmError::Module(ModuleError::NotFound(_)) => vec![
                "检查模块路径是否正确".to_string(),
                "确认模块已正确加载".to_string(),
                "验证模块依赖关系".to_string(),
            ],
            WasmError::Runtime(RuntimeError::OutOfMemory) => vec![
                "增加可用内存".to_string(),
                "优化内存使用".to_string(),
                "检查内存泄漏".to_string(),
            ],
            WasmError::Security(_) => vec![
                "检查安全策略".to_string(),
                "验证用户权限".to_string(),
                "审查访问日志".to_string(),
            ],
            _ => vec!["联系技术支持".to_string()],
        }
    }
}

/// 错误上下文宏 / Error Context Macro
#[macro_export]
macro_rules! wasm_error_context {
    ($error:expr, $context:expr) => {
        $error.map_err(|e| {
            log::error!("错误上下文: {} - {}", $context, e);
            e
        })
    };
}

/// 错误恢复宏 / Error Recovery Macro
#[macro_export]
macro_rules! with_wasm_recovery {
    ($operation:expr, $recovery:expr) => {
        match $operation {
            Ok(result) => Ok(result),
            Err(e) => {
                log::warn!("操作失败，尝试恢复: {}", e);
                $recovery
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_severity() {
        let error = WasmError::Internal {
            message: "测试错误".to_string(),
            component: "测试组件".to_string(),
        };
        assert_eq!(error.severity(), ErrorSeverity::Critical);
    }
    
    #[test]
    fn test_recovery_suggestions() {
        let error = WasmError::Module(ModuleError::NotFound("test_module".to_string()));
        let suggestions = error.recovery_suggestions();
        assert!(!suggestions.is_empty());
    }
}
