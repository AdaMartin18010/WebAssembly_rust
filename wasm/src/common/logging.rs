//! # 日志记录模块 / Logging Module
//!
//! 本模块提供了统一的日志记录功能，支持结构化日志和多种日志级别。
//! This module provides unified logging functionality with support for structured logging and multiple log levels.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use std::time::SystemTime; // 暂时注释掉未使用的导入
use chrono::{DateTime, Utc};

/// 日志级别 / Log Level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// 跟踪 / Trace
    Trace,
    /// 调试 / Debug
    Debug,
    /// 信息 / Info
    Info,
    /// 警告 / Warn
    Warn,
    /// 错误 / Error
    Error,
    /// 致命 / Fatal
    Fatal,
}

impl LogLevel {
    /// 转换为字符串 / Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
    
    /// 从字符串解析 / Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            "FATAL" => Some(LogLevel::Fatal),
            _ => None,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 日志条目 / Log Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// 时间戳 / Timestamp
    pub timestamp: DateTime<Utc>,
    /// 日志级别 / Log level
    pub level: LogLevel,
    /// 消息 / Message
    pub message: String,
    /// 模块 / Module
    pub module: Option<String>,
    /// 文件 / File
    pub file: Option<String>,
    /// 行号 / Line number
    pub line: Option<u32>,
    /// 额外字段 / Additional fields
    pub fields: HashMap<String, serde_json::Value>,
    /// 线程ID / Thread ID
    pub thread_id: Option<String>,
    /// 进程ID / Process ID
    pub process_id: Option<u32>,
}

impl LogEntry {
    /// 创建新的日志条目 / Create new log entry
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message,
            module: None,
            file: None,
            line: None,
            fields: HashMap::new(),
            thread_id: None,
            process_id: None,
        }
    }
    
    /// 设置模块 / Set module
    pub fn module(mut self, module: String) -> Self {
        self.module = Some(module);
        self
    }
    
    /// 设置位置信息 / Set location information
    pub fn location(mut self, file: String, line: u32) -> Self {
        self.file = Some(file);
        self.line = Some(line);
        self
    }
    
    /// 添加字段 / Add field
    pub fn field(mut self, key: String, value: serde_json::Value) -> Self {
        self.fields.insert(key, value);
        self
    }
    
    /// 添加字符串字段 / Add string field
    pub fn field_str(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, serde_json::Value::String(value));
        self
    }
    
    /// 添加数字字段 / Add number field
    pub fn field_number(mut self, key: String, value: f64) -> Self {
        self.fields.insert(key, serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap_or(serde_json::Number::from(0))));
        self
    }
    
    /// 添加布尔字段 / Add boolean field
    pub fn field_bool(mut self, key: String, value: bool) -> Self {
        self.fields.insert(key, serde_json::Value::Bool(value));
        self
    }
}

/// 日志处理器 / Log Handler
pub trait LogHandler: Send + Sync {
    /// 处理日志条目 / Handle log entry
    fn handle(&self, entry: &LogEntry);
    
    /// 刷新日志 / Flush logs
    fn flush(&self) {}
    
    /// 关闭处理器 / Close handler
    fn close(&self) {}
}

/// 控制台日志处理器 / Console Log Handler
pub struct ConsoleLogHandler {
    /// 最小日志级别 / Minimum log level
    min_level: LogLevel,
    /// 是否使用颜色 / Use colors
    use_colors: bool,
}

impl ConsoleLogHandler {
    /// 创建新的控制台日志处理器 / Create new console log handler
    pub fn new(min_level: LogLevel) -> Self {
        Self {
            min_level,
            use_colors: true,
        }
    }
    
    /// 设置是否使用颜色 / Set whether to use colors
    pub fn use_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }
    
    /// 获取级别颜色 / Get level color
    fn get_level_color(&self, level: LogLevel) -> &'static str {
        if !self.use_colors {
            return "";
        }
        
        match level {
            LogLevel::Trace => "\x1b[37m", // White
            LogLevel::Debug => "\x1b[36m", // Cyan
            LogLevel::Info => "\x1b[32m",  // Green
            LogLevel::Warn => "\x1b[33m",  // Yellow
            LogLevel::Error => "\x1b[31m", // Red
            LogLevel::Fatal => "\x1b[35m", // Magenta
        }
    }
    
    /// 重置颜色 / Reset color
    fn reset_color(&self) -> &'static str {
        if self.use_colors {
            "\x1b[0m"
        } else {
            ""
        }
    }
}

impl LogHandler for ConsoleLogHandler {
    fn handle(&self, entry: &LogEntry) {
        if entry.level < self.min_level {
            return;
        }
        
        let color = self.get_level_color(entry.level);
        let reset = self.reset_color();
        
        let timestamp = entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f");
        let level_str = entry.level.as_str();
        
        let location = if let (Some(file), Some(line)) = (&entry.file, &entry.line) {
            format!(" {}:{}", file, line)
        } else {
            String::new()
        };
        
        let module = if let Some(module) = &entry.module {
            format!(" [{}]", module)
        } else {
            String::new()
        };
        
        let fields = if !entry.fields.is_empty() {
            format!(" {}", serde_json::to_string(&entry.fields).unwrap_or_default())
        } else {
            String::new()
        };
        
        println!(
            "{}{} {} {}{}{}{} {}",
            color,
            timestamp,
            level_str,
            module,
            location,
            fields,
            entry.message,
            reset
        );
    }
}

/// 文件日志处理器 / File Log Handler
pub struct FileLogHandler {
    /// 文件路径 / File path
    file_path: String,
    /// 最小日志级别 / Minimum log level
    min_level: LogLevel,
    /// 日志条目缓冲区 / Log entry buffer
    buffer: Arc<Mutex<Vec<LogEntry>>>,
    /// 缓冲区大小 / Buffer size
    buffer_size: usize,
}

impl FileLogHandler {
    /// 创建新的文件日志处理器 / Create new file log handler
    pub fn new(file_path: String, min_level: LogLevel) -> Self {
        Self {
            file_path,
            min_level,
            buffer: Arc::new(Mutex::new(Vec::new())),
            buffer_size: 100,
        }
    }
    
    /// 设置缓冲区大小 / Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// 写入日志到文件 / Write logs to file
    fn write_to_file(&self, entries: &[LogEntry]) -> Result<(), std::io::Error> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        
        for entry in entries {
            let json = serde_json::to_string(entry)?;
            writeln!(file, "{}", json)?;
        }
        
        file.flush()?;
        Ok(())
    }
}

impl LogHandler for FileLogHandler {
    fn handle(&self, entry: &LogEntry) {
        if entry.level < self.min_level {
            return;
        }
        
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(entry.clone());
        
        if buffer.len() >= self.buffer_size {
            let entries = buffer.clone();
            buffer.clear();
            drop(buffer);
            
            if let Err(e) = self.write_to_file(&entries) {
                eprintln!("Failed to write logs to file: {}", e);
            }
        }
    }
    
    fn flush(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        if !buffer.is_empty() {
            let entries = buffer.clone();
            buffer.clear();
            drop(buffer);
            
            if let Err(e) = self.write_to_file(&entries) {
                eprintln!("Failed to flush logs to file: {}", e);
            }
        }
    }
}

/// 结构化日志记录器 / Structured Logger
pub struct StructuredLogger {
    /// 日志处理器 / Log handlers
    handlers: Arc<Mutex<Vec<Box<dyn LogHandler>>>>,
    /// 最小日志级别 / Minimum log level
    min_level: LogLevel,
    /// 默认字段 / Default fields
    default_fields: Arc<Mutex<HashMap<String, serde_json::Value>>>,
}

impl StructuredLogger {
    /// 创建新的结构化日志记录器 / Create new structured logger
    pub fn new(min_level: LogLevel) -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
            min_level,
            default_fields: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 添加日志处理器 / Add log handler
    pub fn add_handler(&self, handler: Box<dyn LogHandler>) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler);
    }
    
    /// 设置默认字段 / Set default field
    pub fn set_default_field(&self, key: String, value: serde_json::Value) {
        let mut default_fields = self.default_fields.lock().unwrap();
        default_fields.insert(key, value);
    }
    
    /// 记录日志 / Log entry
    fn log(&self, level: LogLevel, message: String) {
        if level < self.min_level {
            return;
        }
        
        let mut entry = LogEntry::new(level, message);
        
        // 添加默认字段
        let default_fields = self.default_fields.lock().unwrap();
        for (key, value) in default_fields.iter() {
            entry.fields.insert(key.clone(), value.clone());
        }
        drop(default_fields);
        
        // 发送到所有处理器
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.handle(&entry);
        }
    }
    
    /// 记录跟踪日志 / Log trace
    pub fn trace(&self, message: String) {
        self.log(LogLevel::Trace, message);
    }
    
    /// 记录调试日志 / Log debug
    pub fn debug(&self, message: String) {
        self.log(LogLevel::Debug, message);
    }
    
    /// 记录信息日志 / Log info
    pub fn info(&self, message: String) {
        self.log(LogLevel::Info, message);
    }
    
    /// 记录警告日志 / Log warn
    pub fn warn(&self, message: String) {
        self.log(LogLevel::Warn, message);
    }
    
    /// 记录错误日志 / Log error
    pub fn error(&self, message: String) {
        self.log(LogLevel::Error, message);
    }
    
    /// 记录致命日志 / Log fatal
    pub fn fatal(&self, message: String) {
        self.log(LogLevel::Fatal, message);
    }
    
    /// 刷新所有处理器 / Flush all handlers
    pub fn flush(&self) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.flush();
        }
    }
    
    /// 关闭所有处理器 / Close all handlers
    pub fn close(&self) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.close();
        }
    }
}

/// 全局日志记录器 / Global Logger
pub struct GlobalLogger {
    logger: Arc<StructuredLogger>,
}

impl GlobalLogger {
    /// 创建新的全局日志记录器 / Create new global logger
    pub fn new(min_level: LogLevel) -> Self {
        Self {
            logger: Arc::new(StructuredLogger::new(min_level)),
        }
    }
    
    /// 获取日志记录器实例 / Get logger instance
    pub fn logger(&self) -> Arc<StructuredLogger> {
        self.logger.clone()
    }
    
    /// 添加控制台处理器 / Add console handler
    pub fn add_console_handler(&self, min_level: LogLevel) {
        let handler = Box::new(ConsoleLogHandler::new(min_level));
        self.logger.add_handler(handler);
    }
    
    /// 添加文件处理器 / Add file handler
    pub fn add_file_handler(&self, file_path: String, min_level: LogLevel) {
        let handler = Box::new(FileLogHandler::new(file_path, min_level));
        self.logger.add_handler(handler);
    }
    
    /// 设置默认字段 / Set default field
    pub fn set_default_field(&self, key: String, value: serde_json::Value) {
        self.logger.set_default_field(key, value);
    }
}

impl Default for GlobalLogger {
    fn default() -> Self {
        Self::new(LogLevel::Info)
    }
}

/// 日志宏 / Logging Macros
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.trace(format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.debug(format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.info(format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.warn(format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.error(format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        if let Some(logger) = crate::common::logging::get_global_logger() {
            logger.fatal(format!($($arg)*));
        }
    };
}

/// 全局日志记录器实例 / Global logger instance
use std::sync::OnceLock;

static GLOBAL_LOGGER: OnceLock<GlobalLogger> = OnceLock::new();

/// 初始化全局日志记录器 / Initialize global logger
pub fn init_global_logger(min_level: LogLevel) {
    let _ = GLOBAL_LOGGER.set(GlobalLogger::new(min_level));
}

/// 获取全局日志记录器 / Get global logger
pub fn get_global_logger() -> Option<Arc<StructuredLogger>> {
    GLOBAL_LOGGER.get().map(|logger| logger.logger())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert!(LogLevel::Error > LogLevel::Info);
    }
    
    #[test]
    fn test_log_entry() {
        let entry = LogEntry::new(LogLevel::Info, "Test message".to_string())
            .module("test_module".to_string())
            .location("test.rs".to_string(), 42)
            .field_str("key".to_string(), "value".to_string());
        
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.module, Some("test_module".to_string()));
        assert_eq!(entry.file, Some("test.rs".to_string()));
        assert_eq!(entry.line, Some(42));
    }
    
    #[test]
    fn test_structured_logger() {
        let logger = StructuredLogger::new(LogLevel::Debug);
        logger.add_handler(Box::new(ConsoleLogHandler::new(LogLevel::Debug)));
        
        // 这些调用不应该panic
        logger.info("Test info message".to_string());
        logger.warn("Test warning message".to_string());
        logger.error("Test error message".to_string());
    }
}
