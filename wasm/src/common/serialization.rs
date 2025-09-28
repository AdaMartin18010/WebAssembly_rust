//! # 序列化模块 / Serialization Module
//!
//! 本模块提供了统一的序列化和反序列化功能，支持多种格式和优化。
//! This module provides unified serialization and deserialization functionality with support for multiple formats and optimizations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::io::{Read, Write}; // 暂时注释掉未使用的导入

/// 序列化格式 / Serialization Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializationFormat {
    /// JSON格式 / JSON format
    Json,
    /// MessagePack格式 / MessagePack format (需要额外依赖)
    MessagePack,
    /// Bincode格式 / Bincode format (需要额外依赖)
    Bincode,
    /// CBOR格式 / CBOR format (需要额外依赖)
    Cbor,
    /// YAML格式 / YAML format (需要额外依赖)
    Yaml,
    /// TOML格式 / TOML format (需要额外依赖)
    Toml,
}

impl SerializationFormat {
    /// 从文件扩展名获取格式 / Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(SerializationFormat::Json),
            "msgpack" | "mp" => Some(SerializationFormat::MessagePack),
            "bin" => Some(SerializationFormat::Bincode),
            "cbor" => Some(SerializationFormat::Cbor),
            "yaml" | "yml" => Some(SerializationFormat::Yaml),
            "toml" => Some(SerializationFormat::Toml),
            _ => None,
        }
    }
    
    /// 获取文件扩展名 / Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            SerializationFormat::Json => "json",
            SerializationFormat::MessagePack => "msgpack",
            SerializationFormat::Bincode => "bin",
            SerializationFormat::Cbor => "cbor",
            SerializationFormat::Yaml => "yaml",
            SerializationFormat::Toml => "toml",
        }
    }
}

/// 序列化器 / Serializer
pub struct Serializer {
    /// 默认格式 / Default format
    default_format: SerializationFormat,
    /// 优化选项 / Optimization options
    optimization: OptimizationOptions,
}

/// 优化选项 / Optimization Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOptions {
    /// 跳过空字段 / Skip empty fields
    pub skip_empty_fields: bool,
    /// 跳过默认值 / Skip default values
    pub skip_default_values: bool,
    /// 使用紧凑格式 / Use compact format
    pub use_compact_format: bool,
    /// 启用二进制优化 / Enable binary optimization
    pub enable_binary_optimization: bool,
}

impl Default for OptimizationOptions {
    fn default() -> Self {
        Self {
            skip_empty_fields: true,
            skip_default_values: false,
            use_compact_format: false,
            enable_binary_optimization: true,
        }
    }
}

impl Serializer {
    /// 创建新的序列化器 / Create new serializer
    pub fn new(default_format: SerializationFormat) -> Self {
        Self {
            default_format,
            optimization: OptimizationOptions::default(),
        }
    }
    
    /// 设置优化选项 / Set optimization options
    pub fn optimization(mut self, optimization: OptimizationOptions) -> Self {
        self.optimization = optimization;
        self
    }
    
    /// 序列化数据 / Serialize data
    pub fn serialize<T: Serialize>(&self, data: &T, format: Option<SerializationFormat>) -> Result<Vec<u8>, SerializationError> {
        let format = format.unwrap_or(self.default_format);
        let bytes = match format {
            SerializationFormat::Json => {
                if self.optimization.use_compact_format {
                    serde_json::to_vec(data)?
                } else {
                    serde_json::to_vec_pretty(data)?
                }
            },
            // 其他格式需要额外依赖，暂时返回错误
            _ => return Err(SerializationError::UnsupportedFormat(format!("格式 {:?} 需要额外依赖", format))),
        };
        
        Ok(bytes)
    }
    
    /// 反序列化数据 / Deserialize data
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self, bytes: &[u8], format: Option<SerializationFormat>) -> Result<T, SerializationError> {
        let format = format.unwrap_or(self.default_format);
        
        let data = match format {
            SerializationFormat::Json => {
                serde_json::from_slice(bytes)?
            },
            // 其他格式需要额外依赖，暂时返回错误
            _ => return Err(SerializationError::UnsupportedFormat(format!("格式 {:?} 需要额外依赖", format))),
        };
        
        Ok(data)
    }
    
    /// 序列化到文件 / Serialize to file
    pub fn serialize_to_file<T: Serialize>(&self, data: &T, path: &str, format: Option<SerializationFormat>) -> Result<(), SerializationError> {
        let bytes = self.serialize(data, format)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
    
    /// 从文件反序列化 / Deserialize from file
    pub fn deserialize_from_file<T: for<'de> Deserialize<'de>>(&self, path: &str, format: Option<SerializationFormat>) -> Result<T, SerializationError> {
        let bytes = std::fs::read(path)?;
        self.deserialize(&bytes, format)
    }
}

/// 序列化错误 / Serialization Error
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("格式不支持: {0}")]
    UnsupportedFormat(String),
}

/// 序列化缓存 / Serialization Cache
pub struct SerializationCache {
    /// 缓存数据 / Cache data
    cache: HashMap<String, Vec<u8>>,
    /// 最大缓存大小 / Maximum cache size
    max_size: usize,
    /// 序列化器 / Serializer
    serializer: Serializer,
}

impl SerializationCache {
    /// 创建新的序列化缓存 / Create new serialization cache
    pub fn new(max_size: usize, serializer: Serializer) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            serializer,
        }
    }
    
    /// 获取序列化数据 / Get serialized data
    pub fn get<T: Serialize + Clone>(&mut self, key: &str, data: &T, format: Option<SerializationFormat>) -> Result<Vec<u8>, SerializationError> {
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached.clone());
        }
        
        let serialized = self.serializer.serialize(data, format)?;
        
        // 检查缓存大小
        if self.cache.len() >= self.max_size {
            // 移除最旧的条目
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        
        self.cache.insert(key.to_string(), serialized.clone());
        Ok(serialized)
    }
    
    /// 清除缓存 / Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// 获取缓存大小 / Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

/// 序列化统计 / Serialization Statistics
#[derive(Debug, Clone, Default)]
pub struct SerializationStats {
    /// 序列化次数 / Serialization count
    pub serialize_count: u64,
    /// 反序列化次数 / Deserialization count
    pub deserialize_count: u64,
    /// 总序列化时间 / Total serialization time
    pub total_serialize_time: std::time::Duration,
    /// 总反序列化时间 / Total deserialization time
    pub total_deserialize_time: std::time::Duration,
    /// 缓存命中次数 / Cache hit count
    pub cache_hits: u64,
    /// 缓存未命中次数 / Cache miss count
    pub cache_misses: u64,
}

impl SerializationStats {
    /// 记录序列化 / Record serialization
    pub fn record_serialize(&mut self, duration: std::time::Duration) {
        self.serialize_count += 1;
        self.total_serialize_time += duration;
    }
    
    /// 记录反序列化 / Record deserialization
    pub fn record_deserialize(&mut self, duration: std::time::Duration) {
        self.deserialize_count += 1;
        self.total_deserialize_time += duration;
    }
    
    /// 记录缓存命中 / Record cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    /// 记录缓存未命中 / Record cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    
    /// 获取平均序列化时间 / Get average serialization time
    pub fn average_serialize_time(&self) -> std::time::Duration {
        if self.serialize_count > 0 {
            self.total_serialize_time / self.serialize_count as u32
        } else {
            std::time::Duration::ZERO
        }
    }
    
    /// 获取平均反序列化时间 / Get average deserialization time
    pub fn average_deserialize_time(&self) -> std::time::Duration {
        if self.deserialize_count > 0 {
            self.total_deserialize_time / self.deserialize_count as u32
        } else {
            std::time::Duration::ZERO
        }
    }
    
    /// 获取缓存命中率 / Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total > 0 {
            self.cache_hits as f64 / total as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
        items: Vec<String>,
    }
    
    #[test]
    fn test_serialization_format() {
        assert_eq!(SerializationFormat::from_extension("json"), Some(SerializationFormat::Json));
        assert_eq!(SerializationFormat::Json.extension(), "json");
    }
    
    #[test]
    fn test_serializer() {
        let serializer = Serializer::new(SerializationFormat::Json);
        let data = TestData {
            name: "test".to_string(),
            value: 42,
            items: vec!["item1".to_string(), "item2".to_string()],
        };
        
        // 序列化
        let serialized = serializer.serialize(&data, None).unwrap();
        assert!(!serialized.is_empty());
        
        // 反序列化
        let deserialized: TestData = serializer.deserialize(&serialized, None).unwrap();
        assert_eq!(data, deserialized);
    }
    
    #[test]
    fn test_serialization_cache() {
        let serializer = Serializer::new(SerializationFormat::Json);
        let mut cache = SerializationCache::new(10, serializer);
        let data = TestData {
            name: "test".to_string(),
            value: 42,
            items: vec!["item1".to_string()],
        };
        
        // 第一次获取（缓存未命中）
        let result1 = cache.get("key1", &data, None).unwrap();
        
        // 第二次获取（缓存命中）
        let result2 = cache.get("key1", &data, None).unwrap();
        
        assert_eq!(result1, result2);
    }
    
    #[test]
    fn test_serialization_stats() {
        let mut stats = SerializationStats::default();
        
        stats.record_serialize(std::time::Duration::from_millis(100));
        stats.record_deserialize(std::time::Duration::from_millis(50));
        stats.record_cache_hit();
        stats.record_cache_miss();
        
        assert_eq!(stats.serialize_count, 1);
        assert_eq!(stats.deserialize_count, 1);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.cache_hit_rate(), 0.5);
    }
}