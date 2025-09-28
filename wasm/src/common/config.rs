//! # 配置管理模块 / Configuration Management Module
//!
//! 本模块提供了统一的配置管理功能，支持多种配置格式和动态配置更新。
//! This module provides unified configuration management functionality, supporting multiple configuration formats and dynamic configuration updates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::Path;
use std::fs;

/// 配置值类型 / Configuration Value Type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    /// 字符串值 / String value
    String(String),
    /// 整数值 / Integer value
    Integer(i64),
    /// 浮点数值 / Float value
    Float(f64),
    /// 布尔值 / Boolean value
    Boolean(bool),
    /// 数组值 / Array value
    Array(Vec<ConfigValue>),
    /// 对象值 / Object value
    Object(HashMap<String, ConfigValue>),
}

impl ConfigValue {
    /// 转换为字符串 / Convert to string
    pub fn as_string(&self) -> Option<&String> {
        match self {
            ConfigValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// 转换为整数 / Convert to integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ConfigValue::Integer(i) => Some(*i),
            ConfigValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }
    
    /// 转换为浮点数 / Convert to float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(f) => Some(*f),
            ConfigValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    /// 转换为布尔值 / Convert to boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// 转换为数组 / Convert to array
    pub fn as_array(&self) -> Option<&Vec<ConfigValue>> {
        match self {
            ConfigValue::Array(a) => Some(a),
            _ => None,
        }
    }
    
    /// 转换为对象 / Convert to object
    pub fn as_object(&self) -> Option<&HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Object(o) => Some(o),
            _ => None,
        }
    }
}

/// 配置管理器 / Configuration Manager
pub struct ConfigManager {
    /// 配置数据 / Configuration data
    config: Arc<RwLock<HashMap<String, ConfigValue>>>,
    /// 配置监听器 / Configuration listeners
    listeners: Arc<RwLock<Vec<Box<dyn ConfigListener + Send + Sync>>>>,
}

/// 配置监听器 / Configuration Listener
pub trait ConfigListener {
    /// 配置更新回调 / Configuration update callback
    fn on_config_updated(&self, key: &str, old_value: Option<&ConfigValue>, new_value: &ConfigValue);
}

impl ConfigManager {
    /// 创建新的配置管理器 / Create new configuration manager
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(HashMap::new())),
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 从文件加载配置 / Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: HashMap<String, ConfigValue> = serde_json::from_str(&content)?;
        
        let mut current_config = self.config.write().unwrap();
        *current_config = config;
        
        Ok(())
    }
    
    /// 保存配置到文件 / Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.read().unwrap();
        let content = serde_json::to_string_pretty(&*config)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// 设置配置值 / Set configuration value
    pub fn set(&self, key: &str, value: ConfigValue) {
        let mut config = self.config.write().unwrap();
        let old_value = config.get(key).cloned();
        config.insert(key.to_string(), value.clone());
        drop(config);
        
        // 通知监听器
        self.notify_listeners(key, old_value.as_ref(), &value);
    }
    
    /// 获取配置值 / Get configuration value
    pub fn get(&self, key: &str) -> Option<ConfigValue> {
        let config = self.config.read().unwrap();
        config.get(key).cloned()
    }
    
    /// 获取字符串配置值 / Get string configuration value
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)?.as_string().cloned()
    }
    
    /// 获取整数配置值 / Get integer configuration value
    pub fn get_integer(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_integer()
    }
    
    /// 获取浮点数配置值 / Get float configuration value
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key)?.as_float()
    }
    
    /// 获取布尔配置值 / Get boolean configuration value
    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_boolean()
    }
    
    /// 获取带默认值的配置 / Get configuration with default value
    pub fn get_or_default(&self, key: &str, default: ConfigValue) -> ConfigValue {
        self.get(key).unwrap_or(default)
    }
    
    /// 获取带默认值的字符串配置 / Get string configuration with default value
    pub fn get_string_or_default(&self, key: &str, default: String) -> String {
        self.get_string(key).unwrap_or(default)
    }
    
    /// 获取带默认值的整数配置 / Get integer configuration with default value
    pub fn get_integer_or_default(&self, key: &str, default: i64) -> i64 {
        self.get_integer(key).unwrap_or(default)
    }
    
    /// 获取带默认值的浮点数配置 / Get float configuration with default value
    pub fn get_float_or_default(&self, key: &str, default: f64) -> f64 {
        self.get_float(key).unwrap_or(default)
    }
    
    /// 获取带默认值的布尔配置 / Get boolean configuration with default value
    pub fn get_boolean_or_default(&self, key: &str, default: bool) -> bool {
        self.get_boolean(key).unwrap_or(default)
    }
    
    /// 检查配置是否存在 / Check if configuration exists
    pub fn has(&self, key: &str) -> bool {
        let config = self.config.read().unwrap();
        config.contains_key(key)
    }
    
    /// 删除配置 / Remove configuration
    pub fn remove(&self, key: &str) -> Option<ConfigValue> {
        let mut config = self.config.write().unwrap();
        let old_value = config.remove(key);
        drop(config);
        
        // 通知监听器
        if let Some(ref value) = old_value {
            self.notify_listeners(key, Some(value), &ConfigValue::String("".to_string()));
        }
        
        old_value
    }
    
    /// 获取所有配置键 / Get all configuration keys
    pub fn keys(&self) -> Vec<String> {
        let config = self.config.read().unwrap();
        config.keys().cloned().collect()
    }
    
    /// 获取所有配置 / Get all configuration
    pub fn all(&self) -> HashMap<String, ConfigValue> {
        let config = self.config.read().unwrap();
        config.clone()
    }
    
    /// 添加配置监听器 / Add configuration listener
    pub fn add_listener(&self, listener: Box<dyn ConfigListener + Send + Sync>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }
    
    /// 通知监听器 / Notify listeners
    fn notify_listeners(&self, key: &str, old_value: Option<&ConfigValue>, new_value: &ConfigValue) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_config_updated(key, old_value, new_value);
        }
    }
    
    /// 清空所有配置 / Clear all configuration
    pub fn clear(&self) {
        let mut config = self.config.write().unwrap();
        config.clear();
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 应用配置 / Application Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用名称 / Application name
    pub app_name: String,
    /// 应用版本 / Application version
    pub app_version: String,
    /// 调试模式 / Debug mode
    pub debug: bool,
    /// 日志级别 / Log level
    pub log_level: String,
    /// 数据库配置 / Database configuration
    pub database: DatabaseConfig,
    /// 服务器配置 / Server configuration
    pub server: ServerConfig,
    /// 缓存配置 / Cache configuration
    pub cache: CacheConfig,
    /// 安全配置 / Security configuration
    pub security: SecurityConfig,
}

/// 数据库配置 / Database Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库URL / Database URL
    pub url: String,
    /// 最大连接数 / Maximum connections
    pub max_connections: u32,
    /// 连接超时 / Connection timeout
    pub connection_timeout: u64,
    /// 查询超时 / Query timeout
    pub query_timeout: u64,
}

/// 服务器配置 / Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 主机地址 / Host address
    pub host: String,
    /// 端口号 / Port number
    pub port: u16,
    /// 工作线程数 / Worker threads
    pub worker_threads: usize,
    /// 请求超时 / Request timeout
    pub request_timeout: u64,
    /// 最大请求大小 / Maximum request size
    pub max_request_size: usize,
}

/// 缓存配置 / Cache Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存类型 / Cache type
    pub cache_type: String,
    /// 最大缓存大小 / Maximum cache size
    pub max_size: usize,
    /// 缓存过期时间 / Cache expiration time
    pub expiration_time: u64,
    /// 清理间隔 / Cleanup interval
    pub cleanup_interval: u64,
}

/// 安全配置 / Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 启用HTTPS / Enable HTTPS
    pub enable_https: bool,
    /// 证书文件路径 / Certificate file path
    pub cert_file: Option<String>,
    /// 私钥文件路径 / Private key file path
    pub key_file: Option<String>,
    /// 会话超时 / Session timeout
    pub session_timeout: u64,
    /// 最大登录尝试次数 / Maximum login attempts
    pub max_login_attempts: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "WebAssembly 2.0 + Rust 1.90".to_string(),
            app_version: "1.0.0".to_string(),
            debug: false,
            log_level: "info".to_string(),
            database: DatabaseConfig {
                url: "sqlite://:memory:".to_string(),
                max_connections: 10,
                connection_timeout: 30,
                query_timeout: 60,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                worker_threads: 4,
                request_timeout: 30,
                max_request_size: 1024 * 1024, // 1MB
            },
            cache: CacheConfig {
                cache_type: "memory".to_string(),
                max_size: 1000,
                expiration_time: 3600, // 1 hour
                cleanup_interval: 300, // 5 minutes
            },
            security: SecurityConfig {
                enable_https: false,
                cert_file: None,
                key_file: None,
                session_timeout: 1800, // 30 minutes
                max_login_attempts: 5,
            },
        }
    }
}

/// 配置构建器 / Configuration Builder
pub struct ConfigBuilder {
    config: AppConfig,
}

impl ConfigBuilder {
    /// 创建新的配置构建器 / Create new configuration builder
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }
    
    /// 设置应用名称 / Set application name
    pub fn app_name(mut self, name: String) -> Self {
        self.config.app_name = name;
        self
    }
    
    /// 设置应用版本 / Set application version
    pub fn app_version(mut self, version: String) -> Self {
        self.config.app_version = version;
        self
    }
    
    /// 设置调试模式 / Set debug mode
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }
    
    /// 设置日志级别 / Set log level
    pub fn log_level(mut self, level: String) -> Self {
        self.config.log_level = level;
        self
    }
    
    /// 设置数据库配置 / Set database configuration
    pub fn database(mut self, database: DatabaseConfig) -> Self {
        self.config.database = database;
        self
    }
    
    /// 设置服务器配置 / Set server configuration
    pub fn server(mut self, server: ServerConfig) -> Self {
        self.config.server = server;
        self
    }
    
    /// 设置缓存配置 / Set cache configuration
    pub fn cache(mut self, cache: CacheConfig) -> Self {
        self.config.cache = cache;
        self
    }
    
    /// 设置安全配置 / Set security configuration
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }
    
    /// 构建配置 / Build configuration
    pub fn build(self) -> AppConfig {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_value() {
        let string_val = ConfigValue::String("test".to_string());
        assert_eq!(string_val.as_string(), Some(&"test".to_string()));
        
        let int_val = ConfigValue::Integer(42);
        assert_eq!(int_val.as_integer(), Some(42));
        
        let bool_val = ConfigValue::Boolean(true);
        assert_eq!(bool_val.as_boolean(), Some(true));
    }
    
    #[test]
    fn test_config_manager() {
        let manager = ConfigManager::new();
        
        // 设置配置
        manager.set("test_key", ConfigValue::String("test_value".to_string()));
        
        // 获取配置
        let value = manager.get("test_key").unwrap();
        assert_eq!(value.as_string(), Some(&"test_value".to_string()));
        
        // 检查配置是否存在
        assert!(manager.has("test_key"));
        
        // 删除配置
        let removed = manager.remove("test_key");
        assert!(removed.is_some());
        assert!(!manager.has("test_key"));
    }
    
    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .app_name("Test App".to_string())
            .debug(true)
            .build();
        
        assert_eq!(config.app_name, "Test App");
        assert!(config.debug);
    }
}
