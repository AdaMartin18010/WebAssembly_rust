# WebAssembly 2.0 + Rust 1.90 安全开发指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 项目的完整安全开发解决方案，包括安全编码实践、漏洞防护、安全测试、威胁建模等全方位的安全最佳实践。

## 🎯 安全架构

### 1. 安全模型

#### 安全层次结构

```text
应用安全 → 运行时安全 → 系统安全 → 网络安全 → 物理安全
    ↓         ↓          ↓         ↓         ↓
  代码层    容器层     主机层     网络层     基础设施层
```

#### 安全原则

- **最小权限原则**: 只授予必要的权限
- **深度防御**: 多层安全防护
- **安全设计**: 从设计阶段考虑安全
- **持续监控**: 持续安全监控和评估

### 2. 威胁模型

#### 常见威胁类型

- **代码注入**: SQL注入、XSS、命令注入
- **内存安全**: 缓冲区溢出、内存泄漏
- **身份认证**: 会话劫持、权限提升
- **数据泄露**: 敏感信息暴露
- **拒绝服务**: 资源耗尽攻击

## 🔒 安全编码实践

### 1. 输入验证

#### 数据验证框架

```rust
// src/security/validation.rs
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::collections::HashSet;

/// 输入验证器
pub struct InputValidator {
    patterns: ValidationPatterns,
    blacklist: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationPatterns {
    pub email: Regex,
    pub phone: Regex,
    pub url: Regex,
    pub alphanumeric: Regex,
    pub safe_string: Regex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub sanitized_value: Option<String>,
}

impl InputValidator {
    pub fn new() -> Result<Self, regex::Error> {
        let patterns = ValidationPatterns {
            email: Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")?,
            phone: Regex::new(r"^\+?[1-9]\d{1,14}$")?,
            url: Regex::new(r"^https?://[^\s/$.?#].[^\s]*$")?,
            alphanumeric: Regex::new(r"^[a-zA-Z0-9]+$")?,
            safe_string: Regex::new(r"^[a-zA-Z0-9\s\-_.,!?]+$")?,
        };

        let blacklist = HashSet::from([
            "script".to_string(),
            "javascript".to_string(),
            "vbscript".to_string(),
            "onload".to_string(),
            "onerror".to_string(),
            "onclick".to_string(),
            "eval".to_string(),
            "expression".to_string(),
        ]);

        Ok(Self { patterns, blacklist })
    }

    /// 验证邮箱地址
    pub fn validate_email(&self, email: &str) -> ValidationResult {
        let mut errors = Vec::new();
        
        if email.is_empty() {
            errors.push("Email cannot be empty".to_string());
        } else if !self.patterns.email.is_match(email) {
            errors.push("Invalid email format".to_string());
        } else if email.len() > 254 {
            errors.push("Email too long".to_string());
        }

        // 检查危险字符
        if self.contains_dangerous_content(email) {
            errors.push("Email contains dangerous content".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            sanitized_value: if errors.is_empty() {
                Some(email.to_lowercase().trim().to_string())
            } else {
                None
            },
        }
    }

    /// 验证用户输入字符串
    pub fn validate_user_input(&self, input: &str, max_length: usize) -> ValidationResult {
        let mut errors = Vec::new();
        
        if input.is_empty() {
            errors.push("Input cannot be empty".to_string());
        } else if input.len() > max_length {
            errors.push(format!("Input too long (max {} characters)", max_length));
        } else if !self.patterns.safe_string.is_match(input) {
            errors.push("Input contains invalid characters".to_string());
        }

        // 检查SQL注入模式
        if self.contains_sql_injection(input) {
            errors.push("Input contains SQL injection patterns".to_string());
        }

        // 检查XSS模式
        if self.contains_xss_patterns(input) {
            errors.push("Input contains XSS patterns".to_string());
        }

        // 检查黑名单内容
        if self.contains_blacklisted_content(input) {
            errors.push("Input contains blacklisted content".to_string());
        }

        let sanitized = if errors.is_empty() {
            Some(self.sanitize_input(input))
        } else {
            None
        };

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            sanitized_value: sanitized,
        }
    }

    /// 验证文件路径
    pub fn validate_file_path(&self, path: &str) -> ValidationResult {
        let mut errors = Vec::new();
        
        if path.is_empty() {
            errors.push("Path cannot be empty".to_string());
        } else if path.contains("..") {
            errors.push("Path traversal detected".to_string());
        } else if path.starts_with('/') || path.starts_with('\\') {
            errors.push("Absolute paths not allowed".to_string());
        } else if path.contains("~") {
            errors.push("Home directory references not allowed".to_string());
        }

        // 检查危险文件扩展名
        let dangerous_extensions = ["exe", "bat", "cmd", "com", "scr", "pif", "vbs", "js"];
        if let Some(extension) = path.split('.').last() {
            if dangerous_extensions.contains(&extension.to_lowercase().as_str()) {
                errors.push("Dangerous file extension detected".to_string());
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            sanitized_value: if errors.is_empty() {
                Some(path.replace('\\', "/").to_string())
            } else {
                None
            },
        }
    }

    /// 检查是否包含危险内容
    fn contains_dangerous_content(&self, input: &str) -> bool {
        let dangerous_patterns = [
            "<script", "javascript:", "vbscript:", "data:",
            "expression(", "url(", "behavior:", "binding:",
        ];
        
        dangerous_patterns.iter().any(|&pattern| {
            input.to_lowercase().contains(pattern)
        })
    }

    /// 检查SQL注入模式
    fn contains_sql_injection(&self, input: &str) -> bool {
        let sql_patterns = [
            "'", "\"", ";", "--", "/*", "*/", "xp_", "sp_",
            "union", "select", "insert", "update", "delete",
            "drop", "create", "alter", "exec", "execute",
        ];
        
        let lower_input = input.to_lowercase();
        sql_patterns.iter().any(|&pattern| {
            lower_input.contains(pattern)
        })
    }

    /// 检查XSS模式
    fn contains_xss_patterns(&self, input: &str) -> bool {
        let xss_patterns = [
            "<script", "</script", "<iframe", "</iframe",
            "<object", "</object", "<embed", "<link",
            "onload=", "onerror=", "onclick=", "onmouseover=",
            "javascript:", "vbscript:", "data:text/html",
        ];
        
        let lower_input = input.to_lowercase();
        xss_patterns.iter().any(|&pattern| {
            lower_input.contains(pattern)
        })
    }

    /// 检查黑名单内容
    fn contains_blacklisted_content(&self, input: &str) -> bool {
        let lower_input = input.to_lowercase();
        self.blacklist.iter().any(|word| {
            lower_input.contains(word)
        })
    }

    /// 清理输入
    fn sanitize_input(&self, input: &str) -> String {
        input
            .trim()
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
            .replace("&", "&amp;")
            .replace("\r\n", "\n")
            .replace("\r", "\n")
    }
}
```

### 2. 内存安全

#### 安全内存管理

```rust
// src/security/memory.rs
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// 安全内存分配器
pub struct SecureAllocator {
    allocations: Arc<Mutex<HashMap<*mut u8, AllocationInfo>>>,
    max_allocations: usize,
    max_allocation_size: usize,
}

#[derive(Debug, Clone)]
struct AllocationInfo {
    size: usize,
    layout: Layout,
    timestamp: std::time::Instant,
}

impl SecureAllocator {
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(Mutex::new(HashMap::new())),
            max_allocations: 10000,
            max_allocation_size: 100 * 1024 * 1024, // 100MB
        }
    }

    /// 安全分配内存
    pub fn secure_alloc(&self, layout: Layout) -> Result<*mut u8, std::alloc::AllocError> {
        // 检查分配大小限制
        if layout.size() > self.max_allocation_size {
            return Err(std::alloc::AllocError);
        }

        // 检查分配数量限制
        {
            let allocations = self.allocations.lock().unwrap();
            if allocations.len() >= self.max_allocations {
                return Err(std::alloc::AllocError);
            }
        }

        // 执行分配
        unsafe {
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                return Err(std::alloc::AllocError);
            }

            // 记录分配信息
            {
                let mut allocations = self.allocations.lock().unwrap();
                allocations.insert(ptr, AllocationInfo {
                    size: layout.size(),
                    layout,
                    timestamp: std::time::Instant::now(),
                });
            }

            // 清零内存
            std::ptr::write_bytes(ptr, 0, layout.size());

            Ok(ptr)
        }
    }

    /// 安全释放内存
    pub fn secure_dealloc(&self, ptr: *mut u8, layout: Layout) {
        // 移除分配记录
        {
            let mut allocations = self.allocations.lock().unwrap();
            allocations.remove(&ptr);
        }

        // 清零内存
        unsafe {
            std::ptr::write_bytes(ptr, 0, layout.size());
        }

        // 释放内存
        unsafe {
            System.dealloc(ptr, layout);
        }
    }

    /// 获取内存使用统计
    pub fn get_memory_stats(&self) -> MemoryStats {
        let allocations = self.allocations.lock().unwrap();
        let total_size: usize = allocations.values().map(|info| info.size).sum();
        let allocation_count = allocations.len();

        MemoryStats {
            total_allocations: allocation_count,
            total_bytes_allocated: total_size,
            average_allocation_size: if allocation_count > 0 {
                total_size / allocation_count
            } else {
                0
            },
        }
    }

    /// 清理过期分配
    pub fn cleanup_expired_allocations(&self, max_age: std::time::Duration) {
        let now = std::time::Instant::now();
        let mut allocations = self.allocations.lock().unwrap();
        
        allocations.retain(|&ptr, info| {
            if now.duration_since(info.timestamp) > max_age {
                // 这里应该记录泄漏的内存
                unsafe {
                    std::ptr::write_bytes(ptr, 0, info.size);
                    System.dealloc(ptr, info.layout);
                }
                false
            } else {
                true
            }
        });
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub total_allocations: usize,
    pub total_bytes_allocated: usize,
    pub average_allocation_size: usize,
}

/// 安全缓冲区
pub struct SecureBuffer {
    data: Vec<u8>,
    max_size: usize,
}

impl SecureBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_size.min(1024)),
            max_size,
        }
    }

    /// 安全追加数据
    pub fn secure_append(&mut self, new_data: &[u8]) -> Result<(), SecurityError> {
        if self.data.len() + new_data.len() > self.max_size {
            return Err(SecurityError::BufferOverflow);
        }

        self.data.extend_from_slice(new_data);
        Ok(())
    }

    /// 安全获取数据
    pub fn secure_get(&self, offset: usize, length: usize) -> Result<&[u8], SecurityError> {
        if offset + length > self.data.len() {
            return Err(SecurityError::OutOfBounds);
        }

        Ok(&self.data[offset..offset + length])
    }

    /// 安全设置数据
    pub fn secure_set(&mut self, offset: usize, data: &[u8]) -> Result<(), SecurityError> {
        if offset + data.len() > self.data.len() {
            return Err(SecurityError::OutOfBounds);
        }

        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// 清零缓冲区
    pub fn secure_clear(&mut self) {
        self.data.fill(0);
        self.data.clear();
    }
}

#[derive(Debug)]
pub enum SecurityError {
    BufferOverflow,
    OutOfBounds,
    InvalidInput,
}
```

### 3. 加密和安全通信

#### 加密工具

```rust
// src/security/crypto.rs
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::{Rng, rngs::OsRng};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Algorithm, Version, Params};
use argon2::password_hash::{rand_core::OsRng, SaltString};

/// 加密管理器
pub struct CryptoManager {
    aes_key: Key<Aes256Gcm>,
    hmac_key: [u8; 32],
}

impl CryptoManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut aes_key_bytes = [0u8; 32];
        OsRng.fill(&mut aes_key_bytes);
        let aes_key = Key::from_slice(&aes_key_bytes);

        let mut hmac_key = [0u8; 32];
        OsRng.fill(&mut hmac_key);

        Ok(Self {
            aes_key: *aes_key,
            hmac_key,
        })
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let cipher = Aes256Gcm::new(&self.aes_key);
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext)?;
        
        // 返回格式: nonce + ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if ciphertext.len() < 12 {
            return Err("Invalid ciphertext length".into());
        }

        let cipher = Aes256Gcm::new(&self.aes_key);
        
        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let encrypted_data = &ciphertext[12..];
        
        let plaintext = cipher.decrypt(nonce, encrypted_data)?;
        Ok(plaintext)
    }

    /// 生成HMAC
    pub fn generate_hmac(&self, data: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.hmac_key)?;
        mac.update(data);
        let result = mac.finalize().into_bytes();
        
        let mut hmac_bytes = [0u8; 32];
        hmac_bytes.copy_from_slice(&result);
        Ok(hmac_bytes)
    }

    /// 验证HMAC
    pub fn verify_hmac(&self, data: &[u8], hmac: &[u8; 32]) -> Result<bool, Box<dyn std::error::Error>> {
        let computed_hmac = self.generate_hmac(data)?;
        Ok(computed_hmac == *hmac)
    }

    /// 哈希密码
    pub fn hash_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(65536, 3, 4, None)?,
        );
        
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    /// 验证密码
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(65536, 3, 4, None)?,
        );
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// 生成安全随机数
    pub fn generate_random_bytes(&self, length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        OsRng.fill(&mut bytes);
        bytes
    }

    /// 生成安全随机字符串
    pub fn generate_random_string(&self, length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = OsRng;
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
```

## 🧪 安全测试

### 1. 静态代码分析

#### 安全扫描配置

```toml
# Cargo.toml
[dependencies]
# 安全相关依赖
ring = "0.17"
aes-gcm = "0.10"
argon2 = "0.5"
hmac = "0.12"
sha2 = "0.10"
rand = "0.8"

[dev-dependencies]
# 安全测试工具
cargo-audit = "0.17"
cargo-deny = "0.14"
```

#### 安全检查脚本

```bash
#!/bin/bash
# scripts/security-check.sh

set -e

echo "🔒 开始安全检查..."

# 安装安全工具
echo "📦 安装安全工具..."
cargo install cargo-audit cargo-deny

# 依赖安全检查
echo "🔍 检查依赖安全漏洞..."
cargo audit

# 许可证检查
echo "📄 检查许可证..."
cargo deny check

# 代码安全检查
echo "🔬 运行代码安全检查..."
cargo clippy -- -D warnings

# 格式化检查
echo "🎨 检查代码格式..."
cargo fmt -- --check

# 运行测试
echo "🧪 运行安全测试..."
cargo test --features security

echo "✅ 安全检查完成"
```

### 2. 动态安全测试

#### 模糊测试

```rust
// tests/fuzz_tests.rs
use libfuzzer_sys::fuzz_target;
use wasm_app::security::InputValidator;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let validator = InputValidator::new().unwrap();
        let _result = validator.validate_user_input(input, 1000);
    }
});

#[cfg(test)]
mod security_tests {
    use super::*;
    use wasm_app::security::{InputValidator, CryptoManager};

    #[test]
    fn test_sql_injection_prevention() {
        let validator = InputValidator::new().unwrap();
        
        let malicious_inputs = [
            "'; DROP TABLE users; --",
            "' OR '1'='1",
            "admin'--",
            "' UNION SELECT * FROM users--",
        ];

        for input in &malicious_inputs {
            let result = validator.validate_user_input(input, 100);
            assert!(!result.is_valid, "Should reject SQL injection: {}", input);
        }
    }

    #[test]
    fn test_xss_prevention() {
        let validator = InputValidator::new().unwrap();
        
        let malicious_inputs = [
            "<script>alert('XSS')</script>",
            "javascript:alert('XSS')",
            "<img src=x onerror=alert('XSS')>",
            "<iframe src=javascript:alert('XSS')></iframe>",
        ];

        for input in &malicious_inputs {
            let result = validator.validate_user_input(input, 100);
            assert!(!result.is_valid, "Should reject XSS: {}", input);
        }
    }

    #[test]
    fn test_path_traversal_prevention() {
        let validator = InputValidator::new().unwrap();
        
        let malicious_paths = [
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\drivers\\etc\\hosts",
            "/etc/passwd",
            "C:\\Windows\\System32\\config\\SAM",
        ];

        for path in &malicious_paths {
            let result = validator.validate_file_path(path);
            assert!(!result.is_valid, "Should reject path traversal: {}", path);
        }
    }

    #[test]
    fn test_encryption_decryption() {
        let crypto = CryptoManager::new().unwrap();
        let plaintext = b"Hello, World!";
        
        let ciphertext = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_password_hashing() {
        let crypto = CryptoManager::new().unwrap();
        let password = "test_password_123";
        
        let hash = crypto.hash_password(password).unwrap();
        assert!(crypto.verify_password(password, &hash).unwrap());
        assert!(!crypto.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_hmac_verification() {
        let crypto = CryptoManager::new().unwrap();
        let data = b"test_data";
        
        let hmac = crypto.generate_hmac(data).unwrap();
        assert!(crypto.verify_hmac(data, &hmac).unwrap());
        
        let wrong_data = b"wrong_data";
        assert!(!crypto.verify_hmac(wrong_data, &hmac).unwrap());
    }
}
```

## 🛡️ 威胁防护

### 1. 速率限制

#### 速率限制器

```rust
// src/security/rate_limiter.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
            burst_size: 10,
        }
    }
}

#[derive(Debug)]
struct RateLimitEntry {
    minute_count: u32,
    hour_count: u32,
    day_count: u32,
    last_minute_reset: Instant,
    last_hour_reset: Instant,
    last_day_reset: Instant,
    burst_tokens: u32,
    last_burst_refill: Instant,
}

impl RateLimitEntry {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            minute_count: 0,
            hour_count: 0,
            day_count: 0,
            last_minute_reset: now,
            last_hour_reset: now,
            last_day_reset: now,
            burst_tokens: 10,
            last_burst_refill: now,
        }
    }
}

pub struct RateLimiter {
    entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub fn check_rate_limit(&self, identifier: &str) -> Result<(), RateLimitError> {
        let mut entries = self.entries.lock().unwrap();
        let entry = entries.entry(identifier.to_string())
            .or_insert_with(RateLimitEntry::new);

        let now = Instant::now();

        // 重置计数器
        self.reset_counters(entry, now);

        // 检查突发限制
        if entry.burst_tokens == 0 {
            return Err(RateLimitError::BurstLimitExceeded);
        }

        // 检查速率限制
        if entry.minute_count >= self.config.requests_per_minute {
            return Err(RateLimitError::MinuteLimitExceeded);
        }

        if entry.hour_count >= self.config.requests_per_hour {
            return Err(RateLimitError::HourLimitExceeded);
        }

        if entry.day_count >= self.config.requests_per_day {
            return Err(RateLimitError::DayLimitExceeded);
        }

        // 更新计数器
        entry.minute_count += 1;
        entry.hour_count += 1;
        entry.day_count += 1;
        entry.burst_tokens -= 1;

        Ok(())
    }

    fn reset_counters(&self, entry: &mut RateLimitEntry, now: Instant) {
        // 重置分钟计数器
        if now.duration_since(entry.last_minute_reset) >= Duration::from_secs(60) {
            entry.minute_count = 0;
            entry.last_minute_reset = now;
        }

        // 重置小时计数器
        if now.duration_since(entry.last_hour_reset) >= Duration::from_secs(3600) {
            entry.hour_count = 0;
            entry.last_hour_reset = now;
        }

        // 重置天计数器
        if now.duration_since(entry.last_day_reset) >= Duration::from_secs(86400) {
            entry.day_count = 0;
            entry.last_day_reset = now;
        }

        // 补充突发令牌
        if now.duration_since(entry.last_burst_refill) >= Duration::from_secs(1) {
            let tokens_to_add = now.duration_since(entry.last_burst_refill).as_secs() as u32;
            entry.burst_tokens = (entry.burst_tokens + tokens_to_add)
                .min(self.config.burst_size);
            entry.last_burst_refill = now;
        }
    }

    pub fn get_remaining_requests(&self, identifier: &str) -> RateLimitStatus {
        let entries = self.entries.lock().unwrap();
        let entry = entries.get(identifier);

        match entry {
            Some(entry) => {
                let now = Instant::now();
                // 这里应该重置计数器以获取准确的状态
                RateLimitStatus {
                    minute_remaining: self.config.requests_per_minute.saturating_sub(entry.minute_count),
                    hour_remaining: self.config.requests_per_hour.saturating_sub(entry.hour_count),
                    day_remaining: self.config.requests_per_day.saturating_sub(entry.day_count),
                    burst_tokens: entry.burst_tokens,
                }
            }
            None => RateLimitStatus {
                minute_remaining: self.config.requests_per_minute,
                hour_remaining: self.config.requests_per_hour,
                day_remaining: self.config.requests_per_day,
                burst_tokens: self.config.burst_size,
            }
        }
    }
}

#[derive(Debug)]
pub enum RateLimitError {
    MinuteLimitExceeded,
    HourLimitExceeded,
    DayLimitExceeded,
    BurstLimitExceeded,
}

#[derive(Debug)]
pub struct RateLimitStatus {
    pub minute_remaining: u32,
    pub hour_remaining: u32,
    pub day_remaining: u32,
    pub burst_tokens: u32,
}
```

### 2. 安全头配置

#### 安全中间件

```rust
// src/security/middleware.rs
use actix_web::{HttpResponse, HttpRequest, Result};
use actix_web::middleware::DefaultHeaders;

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("X-XSS-Protection", "1; mode=block"))
        .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
        .add(("Content-Security-Policy", 
            "default-src 'self'; \
             script-src 'self' 'unsafe-inline' 'unsafe-eval'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: https:; \
             font-src 'self' data:; \
             connect-src 'self' https:; \
             frame-ancestors 'none';"))
        .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
        .add(("Permissions-Policy", 
            "camera=(), microphone=(), geolocation=(), \
             payment=(), usb=(), magnetometer=(), gyroscope=(), \
             accelerometer=(), ambient-light-sensor=()"))
}
```

## 📋 安全最佳实践

### 1. 开发安全

- **安全编码**: 遵循安全编码规范
- **代码审查**: 定期进行安全代码审查
- **依赖管理**: 及时更新依赖并修复漏洞
- **最小权限**: 使用最小权限原则

### 2. 部署安全

- **容器安全**: 使用安全的基础镜像
- **网络隔离**: 实施网络分段和隔离
- **密钥管理**: 安全存储和管理密钥
- **监控告警**: 建立安全监控和告警

### 3. 运维安全

- **访问控制**: 实施严格的访问控制
- **审计日志**: 记录和监控所有操作
- **备份恢复**: 定期备份和测试恢复
- **安全更新**: 及时应用安全补丁

### 4. 应急响应

- **事件响应**: 建立安全事件响应流程
- **漏洞管理**: 建立漏洞管理流程
- **通信计划**: 制定安全事件通信计划
- **恢复程序**: 制定业务恢复程序

---

**注意**: 本指南提供了完整的安全开发解决方案，建议在实际项目中根据具体需求进行调整和优化。安全是一个持续的过程，需要定期评估和改进。
