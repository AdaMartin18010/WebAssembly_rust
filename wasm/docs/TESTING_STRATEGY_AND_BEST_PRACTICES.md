# WebAssembly 2.0 + Rust 1.90 测试策略和最佳实践指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 项目的完整测试策略和最佳实践，包括测试金字塔、测试类型、测试工具、测试自动化等全方位的测试解决方案。

## 🎯 测试策略

### 1. 测试金字塔

#### 测试层次结构

```text
    /\
   /  \     E2E Tests (5%)
  /____\    端到端测试
 /      \   
/________\  Integration Tests (15%)
/          \ 集成测试
/____________\
Unit Tests (80%)
单元测试
```

#### 测试分布原则

- **单元测试 (80%)**: 快速、独立、可重复
- **集成测试 (15%)**: 模块间交互、API测试
- **端到端测试 (5%)**: 完整用户流程、关键路径

### 2. 测试类型分类

#### 按测试目的分类

- **功能测试**: 验证功能正确性
- **性能测试**: 验证性能指标
- **安全测试**: 验证安全性
- **兼容性测试**: 验证跨平台兼容性
- **可用性测试**: 验证用户体验

#### 按测试阶段分类

- **开发测试**: 开发过程中的测试
- **集成测试**: 模块集成后的测试
- **系统测试**: 完整系统测试
- **验收测试**: 用户验收测试

## 🧪 单元测试

### 1. 基础单元测试

#### 测试结构

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    /// 测试模块配置创建
    #[test]
    fn test_wasm_module_config_creation() {
        // Arrange - 准备测试数据
        let name = "test-module";
        let version = "1.0.0";
        
        // Act - 执行被测试的操作
        let config = WasmModuleConfig::new(name, version);
        
        // Assert - 验证结果
        assert_eq!(config.name, "test-module");
        assert_eq!(config.version, "1.0.0");
        assert!(config.enable_simd);
        assert!(config.enable_bulk_memory);
    }
    
    /// 测试配置验证
    #[test]
    fn test_wasm_module_config_validation() {
        // 测试有效配置
        let valid_config = WasmModuleConfig::new("valid", "1.0.0");
        assert!(valid_config.validate().is_ok());
        
        // 测试无效名称
        let invalid_name = WasmModuleConfig::new("", "1.0.0");
        assert!(invalid_name.validate().is_err());
        
        // 测试无效版本
        let invalid_version = WasmModuleConfig::new("valid", "");
        assert!(invalid_version.validate().is_err());
    }
    
    /// 测试错误处理
    #[test]
    fn test_error_handling() {
        let result = process_wasm_module(&[]);
        assert!(result.is_err());
        
        if let Err(WasmError::CompilationError(msg)) = result {
            assert_eq!(msg, "Empty module data");
        } else {
            panic!("Expected CompilationError");
        }
    }
}
```

#### 参数化测试

```rust
#[cfg(test)]
mod parameterized_tests {
    use super::*;
    
    /// 测试不同大小的内存分配
    #[test]
    fn test_memory_pool_different_sizes() {
        let pool = HighPerformanceMemoryPool::new();
        let test_sizes = vec![64, 256, 1024, 4096, 16384];
        
        for size in test_sizes {
            let buffer = pool.allocate(size);
            assert!(buffer.is_some(), "Failed to allocate size: {}", size);
            assert_eq!(buffer.unwrap().len(), size);
        }
    }
    
    /// 使用 rstest 进行参数化测试
    use rstest::*;
    
    #[rstest]
    #[case("module1", "1.0.0", true)]
    #[case("module2", "2.1.0", true)]
    #[case("", "1.0.0", false)]
    #[case("valid", "", false)]
    fn test_wasm_module_config_validation_cases(
        #[case] name: &str,
        #[case] version: &str,
        #[case] should_be_valid: bool,
    ) {
        let config = WasmModuleConfig::new(name, version);
        let is_valid = config.validate().is_ok();
        assert_eq!(is_valid, should_be_valid);
    }
}
```

### 2. 高级单元测试

#### 模拟和存根

```rust
use mockall::*;

/// 定义可模拟的 trait
#[automock]
pub trait WasmModuleLoader {
    fn load_module(&self, path: &str) -> Result<Vec<u8>, WasmError>;
    fn validate_module(&self, data: &[u8]) -> Result<(), WasmError>;
}

/// 使用模拟对象进行测试
#[cfg(test)]
mod mock_tests {
    use super::*;
    use mockall::predicate::*;
    
    #[test]
    fn test_wasm_module_processing_with_mock() {
        // 创建模拟对象
        let mut mock_loader = MockWasmModuleLoader::new();
        
        // 设置期望行为
        mock_loader
            .expect_load_module()
            .with(eq("test.wasm"))
            .times(1)
            .returning(|_| Ok(vec![0x00, 0x61, 0x73, 0x6d]));
            
        mock_loader
            .expect_validate_module()
            .with(eq(vec![0x00, 0x61, 0x73, 0x6d]))
            .times(1)
            .returning(|_| Ok(()));
        
        // 执行测试
        let result = process_wasm_module_with_loader(&mock_loader, "test.wasm");
        assert!(result.is_ok());
    }
}
```

#### 属性测试

```rust
use proptest::prelude::*;

#[cfg(test)]
mod property_tests {
    use super::*;
    
    /// 属性测试：内存分配和释放应该保持平衡
    proptest! {
        #[test]
        fn test_memory_pool_allocation_deallocation(
            sizes in prop::collection::vec(1usize..10000, 1..100)
        ) {
            let pool = HighPerformanceMemoryPool::new();
            let mut allocated_buffers = Vec::new();
            
            // 分配内存
            for size in &sizes {
                if let Some(buffer) = pool.allocate(*size) {
                    allocated_buffers.push(buffer);
                }
            }
            
            // 释放内存
            for buffer in allocated_buffers {
                pool.deallocate(buffer);
            }
            
            // 验证统计信息
            let stats = pool.get_statistics();
            assert!(stats.reuse_ratio() >= 0.0);
            assert!(stats.reuse_ratio() <= 1.0);
        }
    }
    
    /// 属性测试：配置验证的幂等性
    proptest! {
        #[test]
        fn test_config_validation_idempotent(
            name in "[a-zA-Z0-9_-]+",
            version in "[0-9]+\\.[0-9]+\\.[0-9]+"
        ) {
            let config = WasmModuleConfig::new(&name, &version);
            
            // 多次验证应该得到相同结果
            let result1 = config.validate();
            let result2 = config.validate();
            
            assert_eq!(result1.is_ok(), result2.is_ok());
        }
    }
}
```

## 🔗 集成测试

### 1. 模块集成测试

#### 基础集成测试

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    /// 测试完整的模块处理流程
    #[test]
    fn test_wasm_module_processing_workflow() {
        // 创建测试配置
        let config = WasmModuleConfig::new("integration-test", "1.0.0");
        assert!(config.validate().is_ok());
        
        // 序列化配置
        let config_data = serde_json::to_vec(&config).unwrap();
        
        // 处理模块
        let processed_config = process_wasm_module(&config_data).unwrap();
        
        // 验证结果
        assert_eq!(processed_config.name, "integration-test");
        assert_eq!(processed_config.version, "1.0.0");
    }
    
    /// 测试内存池与模块处理的集成
    #[test]
    fn test_memory_pool_with_wasm_processing() {
        let pool = HighPerformanceMemoryPool::new();
        
        // 分配内存用于处理
        let mut buffer = pool.allocate(1024).unwrap();
        
        // 模拟数据处理
        buffer.fill(0x42);
        
        // 验证数据
        assert!(buffer.iter().all(|&b| b == 0x42));
        
        // 释放内存
        pool.deallocate(buffer);
        
        // 验证统计信息
        let stats = pool.get_statistics();
        assert!(stats.reuse_ratio() > 0.0);
    }
}
```

#### API 集成测试

```rust
#[cfg(test)]
mod api_integration_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    /// 测试 WebAssembly 模块加载
    #[wasm_bindgen_test]
    fn test_wasm_module_loading() {
        let module_data = include_bytes!("../test_data/simple.wasm");
        let result = load_wasm_module(module_data);
        assert!(result.is_ok());
    }
    
    /// 测试 JavaScript 互操作
    #[wasm_bindgen_test]
    fn test_js_interop() {
        let result = call_js_function("testFunction", &[]);
        assert!(result.is_ok());
    }
}
```

### 2. 数据库集成测试

#### 测试数据库

```rust
#[cfg(test)]
mod database_integration_tests {
    use super::*;
    use tempfile::TempDir;
    
    /// 测试数据库操作
    #[test]
    fn test_database_operations() {
        // 创建临时数据库
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        // 初始化数据库
        let db = Database::new(&db_path).unwrap();
        
        // 测试插入操作
        let config = WasmModuleConfig::new("db-test", "1.0.0");
        db.insert_config(&config).unwrap();
        
        // 测试查询操作
        let retrieved_config = db.get_config("db-test").unwrap();
        assert_eq!(retrieved_config.name, "db-test");
        
        // 测试更新操作
        let mut updated_config = config.clone();
        updated_config.version = "2.0.0".to_string();
        db.update_config(&updated_config).unwrap();
        
        // 验证更新
        let final_config = db.get_config("db-test").unwrap();
        assert_eq!(final_config.version, "2.0.0");
    }
}
```

## 🌐 端到端测试

### 1. Web 端到端测试

#### 使用 Playwright 进行 E2E 测试

```rust
// tests/e2e/web_tests.rs
use wasm_bindgen_test::*;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_web_application_workflow() {
    // 测试页面加载
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // 验证页面元素
    let canvas = document.get_element_by_id("canvas").unwrap();
    assert!(canvas.is_some());
    
    // 测试 WebAssembly 模块加载
    let module_data = include_bytes!("../../test_data/app.wasm");
    let result = load_wasm_module(module_data);
    assert!(result.is_ok());
    
    // 测试用户交互
    let button = document.get_element_by_id("start-button").unwrap();
    let click_event = Event::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();
    
    // 验证结果
    let result_element = document.get_element_by_id("result").unwrap();
    let result_text = result_element.text_content().unwrap();
    assert!(!result_text.is_empty());
}
```

#### 使用 Selenium 进行 E2E 测试

```javascript
// tests/e2e/selenium_tests.js
const { Builder, By, until } = require('selenium-webdriver');

describe('WebAssembly Application E2E Tests', () => {
    let driver;
    
    beforeAll(async () => {
        driver = await new Builder().forBrowser('chrome').build();
    });
    
    afterAll(async () => {
        await driver.quit();
    });
    
    test('should load and run WebAssembly module', async () => {
        // 导航到应用页面
        await driver.get('http://localhost:3000');
        
        // 等待页面加载
        await driver.wait(until.elementLocated(By.id('canvas')), 10000);
        
        // 点击开始按钮
        const startButton = await driver.findElement(By.id('start-button'));
        await startButton.click();
        
        // 等待结果
        await driver.wait(until.elementLocated(By.id('result')), 10000);
        
        // 验证结果
        const resultElement = await driver.findElement(By.id('result'));
        const resultText = await resultElement.getText();
        expect(resultText).not.toBe('');
    });
    
    test('should handle error cases gracefully', async () => {
        // 导航到错误测试页面
        await driver.get('http://localhost:3000/error-test');
        
        // 触发错误
        const errorButton = await driver.findElement(By.id('error-button'));
        await errorButton.click();
        
        // 验证错误处理
        const errorMessage = await driver.findElement(By.id('error-message'));
        await driver.wait(until.elementTextContains(errorMessage, 'Error'), 5000);
    });
});
```

### 2. 移动端测试

#### 使用 Appium 进行移动端测试

```javascript
// tests/e2e/mobile_tests.js
const { Builder, By, until } = require('selenium-webdriver');
const { Options } = require('selenium-webdriver/chrome');

describe('Mobile WebAssembly Tests', () => {
    let driver;
    
    beforeAll(async () => {
        const options = new Options()
            .addArguments('--user-agent=Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)')
            .addArguments('--window-size=375,667');
            
        driver = await new Builder()
            .forBrowser('chrome')
            .setChromeOptions(options)
            .build();
    });
    
    afterAll(async () => {
        await driver.quit();
    });
    
    test('should work on mobile devices', async () => {
        await driver.get('http://localhost:3000');
        
        // 等待移动端适配
        await driver.wait(until.elementLocated(By.className('mobile-layout')), 10000);
        
        // 测试触摸交互
        const touchElement = await driver.findElement(By.id('touch-area'));
        await driver.actions()
            .move({ origin: touchElement })
            .press()
            .release()
            .perform();
        
        // 验证响应
        const response = await driver.findElement(By.id('touch-response'));
        await driver.wait(until.elementTextContains(response, 'Touch detected'), 5000);
    });
});
```

## ⚡ 性能测试

### 1. 基准测试

#### 使用 Criterion 进行基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_memory_allocation(c: &mut Criterion) {
    let pool = HighPerformanceMemoryPool::new();
    
    c.bench_function("memory_allocation_1kb", |b| {
        b.iter(|| {
            let buffer = pool.allocate(black_box(1024));
            black_box(buffer)
        })
    });
    
    c.bench_function("memory_allocation_64kb", |b| {
        b.iter(|| {
            let buffer = pool.allocate(black_box(65536));
            black_box(buffer)
        })
    });
    
    c.bench_function("memory_allocation_1mb", |b| {
        b.iter(|| {
            let buffer = pool.allocate(black_box(1048576));
            black_box(buffer)
        })
    });
}

fn benchmark_wasm_config_validation(c: &mut Criterion) {
    let config = WasmModuleConfig::new("benchmark-module", "1.0.0");
    
    c.bench_function("config_validation", |b| {
        b.iter(|| {
            black_box(config.validate())
        })
    });
}

fn benchmark_wasm_module_processing(c: &mut Criterion) {
    let test_data = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    
    c.bench_function("module_processing", |b| {
        b.iter(|| {
            black_box(process_wasm_module(black_box(&test_data)))
        })
    });
}

criterion_group!(
    benches,
    benchmark_memory_allocation,
    benchmark_wasm_config_validation,
    benchmark_wasm_module_processing
);
criterion_main!(benches);
```

#### 使用 Iai 进行指令级基准测试

```rust
use iai::black_box;

fn iai_memory_allocation() {
    let pool = HighPerformanceMemoryPool::new();
    black_box(pool.allocate(black_box(1024)));
}

fn iai_config_validation() {
    let config = WasmModuleConfig::new("iai-test", "1.0.0");
    black_box(config.validate());
}

iai::main!(iai_memory_allocation, iai_config_validation);
```

### 2. 负载测试

#### 使用 K6 进行负载测试

```javascript
// tests/performance/load_test.js
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    stages: [
        { duration: '2m', target: 100 }, // 2分钟内达到100用户
        { duration: '5m', target: 100 }, // 保持100用户5分钟
        { duration: '2m', target: 200 }, // 2分钟内达到200用户
        { duration: '5m', target: 200 }, // 保持200用户5分钟
        { duration: '2m', target: 0 },   // 2分钟内降到0用户
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95%的请求在500ms内完成
        http_req_failed: ['rate<0.1'],    // 错误率低于10%
    },
};

export default function () {
    // 测试 WebAssembly 模块加载
    let response = http.get('http://localhost:3000/api/load-module');
    check(response, {
        'status is 200': (r) => r.status === 200,
        'response time < 500ms': (r) => r.timings.duration < 500,
    });
    
    // 测试数据处理
    let payload = JSON.stringify({
        data: new Array(1000).fill(0).map((_, i) => i),
    });
    
    response = http.post('http://localhost:3000/api/process-data', payload, {
        headers: { 'Content-Type': 'application/json' },
    });
    
    check(response, {
        'status is 200': (r) => r.status === 200,
        'response time < 1000ms': (r) => r.timings.duration < 1000,
    });
    
    sleep(1);
}
```

#### 使用 Artillery 进行压力测试

```yaml
# tests/performance/stress_test.yml
config:
  target: 'http://localhost:3000'
  phases:
    - duration: 60
      arrivalRate: 10
    - duration: 120
      arrivalRate: 50
    - duration: 60
      arrivalRate: 100
  defaults:
    headers:
      Content-Type: 'application/json'

scenarios:
  - name: "WebAssembly Module Processing"
    weight: 70
    flow:
      - post:
          url: "/api/load-module"
          json:
            moduleName: "test-module"
      - think: 1
      - post:
          url: "/api/process-data"
          json:
            data: "{{ $randomString() }}"
            
  - name: "Configuration Management"
    weight: 30
    flow:
      - get:
          url: "/api/config"
      - think: 0.5
      - post:
          url: "/api/config"
          json:
            name: "{{ $randomString() }}"
            version: "1.0.0"
```

## 🔒 安全测试

### 1. 安全漏洞测试

#### 使用 Cargo Audit 进行依赖安全检查

```bash
# 安装 cargo-audit
cargo install cargo-audit

# 运行安全检查
cargo audit

# 自动修复安全问题
cargo audit fix
```

#### 使用 Cargo Deny 进行许可证和依赖检查

```toml
# deny.toml
[advisories]
# 检查安全漏洞
vulnerability = "deny"
# 检查未维护的包
unmaintained = "warn"
# 检查许可证
license = "deny"
# 检查许可证兼容性
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
]
```

### 2. 输入验证测试

#### 测试恶意输入处理

```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_malicious_input_handling() {
        // 测试 SQL 注入
        let sql_injection = "'; DROP TABLE users; --";
        let result = process_user_input(sql_injection);
        assert!(result.is_err());
        
        // 测试 XSS 攻击
        let xss_payload = "<script>alert('XSS')</script>";
        let result = process_user_input(xss_payload);
        assert!(result.is_err());
        
        // 测试路径遍历
        let path_traversal = "../../../etc/passwd";
        let result = process_file_path(path_traversal);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_memory_exhaustion_attack() {
        let pool = HighPerformanceMemoryPool::new();
        
        // 尝试分配大量内存
        let large_size = usize::MAX;
        let result = pool.allocate(large_size);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_integer_overflow_attack() {
        // 测试整数溢出
        let large_number = i32::MAX;
        let result = safe_add(large_number, 1);
        assert!(result.is_err());
    }
}
```

## 🧪 测试工具和框架

### 1. Rust 测试工具

#### 测试框架选择

```toml
# Cargo.toml
[dev-dependencies]
# 基础测试
tokio-test = "0.4"
# 属性测试
proptest = "1.0"
# 模拟测试
mockall = "0.11"
# 基准测试
criterion = "0.5"
iai = "0.1"
# WebAssembly 测试
wasm-bindgen-test = "0.3"
# 参数化测试
rstest = "0.18"
# 测试覆盖率
cargo-tarpaulin = "0.25"
```

#### 测试配置

```toml
# Cargo.toml
[profile.test]
opt-level = 0
debug = true
overflow-checks = true

[profile.bench]
opt-level = 3
debug = false
lto = true
```

### 2. WebAssembly 测试工具

#### wasm-bindgen-test 配置

```rust
// tests/wasm_tests.rs
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_function() {
    let result = wasm_function(42);
    assert_eq!(result, 84);
}

#[wasm_bindgen_test]
async fn test_async_wasm_function() {
    let result = async_wasm_function().await;
    assert!(result.is_ok());
}
```

#### 测试运行配置

```bash
# 在浏览器中运行测试
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
wasm-pack test --headless --safari

# 在 Node.js 中运行测试
wasm-pack test --node
```

## 📊 测试报告和覆盖率

### 1. 测试覆盖率

#### 使用 Tarpaulin 生成覆盖率报告

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage/

# 生成 XML 报告（用于 CI/CD）
cargo tarpaulin --out Xml --output-dir coverage/
```

#### 覆盖率配置

```toml
# tarpaulin.toml
[tool.tarpaulin]
# 排除测试文件
exclude_files = [
    "*/tests/*",
    "*/benches/*",
    "*/examples/*",
]

# 排除特定行
exclude_lines = [
    "panic!",
    "unreachable!",
    "unimplemented!",
]

# 覆盖率阈值
fail_under = 80
```

### 2. 测试报告

#### 生成测试报告

```bash
# 生成详细的测试报告
cargo test -- --nocapture --test-threads=1

# 生成 JSON 格式的测试结果
cargo test -- --format json

# 生成 JUnit 格式的测试结果
cargo test -- --format junit
```

#### 测试报告分析

```rust
// tests/report_analysis.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct TestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub execution_time: std::time::Duration,
    pub coverage_percentage: f64,
}

impl TestReport {
    pub fn analyze(&self) -> TestAnalysis {
        let pass_rate = if self.total_tests > 0 {
            self.passed_tests as f64 / self.total_tests as f64
        } else {
            0.0
        };
        
        TestAnalysis {
            pass_rate,
            coverage_adequate: self.coverage_percentage >= 80.0,
            performance_acceptable: self.execution_time.as_secs() < 300,
            recommendations: self.generate_recommendations(),
        }
    }
    
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.coverage_percentage < 80.0 {
            recommendations.push("增加测试覆盖率".to_string());
        }
        
        if self.failed_tests > 0 {
            recommendations.push("修复失败的测试".to_string());
        }
        
        if self.execution_time.as_secs() > 300 {
            recommendations.push("优化测试执行时间".to_string());
        }
        
        recommendations
    }
}

#[derive(Debug)]
pub struct TestAnalysis {
    pub pass_rate: f64,
    pub coverage_adequate: bool,
    pub performance_acceptable: bool,
    pub recommendations: Vec<String>,
}
```

## 📋 测试最佳实践

### 1. 测试编写最佳实践

#### 测试命名规范

```rust
// 好的测试命名
#[test]
fn test_wasm_module_config_creation_with_valid_input() {
    // 测试内容
}

#[test]
fn test_memory_pool_allocation_should_fail_when_size_is_zero() {
    // 测试内容
}

// 不好的测试命名
#[test]
fn test1() {
    // 测试内容
}

#[test]
fn test_config() {
    // 测试内容
}
```

#### 测试结构规范

```rust
#[test]
fn test_example() {
    // Arrange - 准备测试数据和环境
    let input = create_test_input();
    let expected_output = create_expected_output();
    
    // Act - 执行被测试的操作
    let actual_output = function_under_test(input);
    
    // Assert - 验证结果
    assert_eq!(actual_output, expected_output);
}
```

### 2. 测试维护最佳实践

#### 测试数据管理

```rust
// 使用工厂模式创建测试数据
pub struct TestDataFactory;

impl TestDataFactory {
    pub fn create_wasm_config() -> WasmModuleConfig {
        WasmModuleConfig::new("test-module", "1.0.0")
    }
    
    pub fn create_wasm_config_with_name(name: &str) -> WasmModuleConfig {
        WasmModuleConfig::new(name, "1.0.0")
    }
    
    pub fn create_large_memory_pool() -> HighPerformanceMemoryPool {
        HighPerformanceMemoryPool::new()
    }
}
```

#### 测试环境隔离

```rust
// 使用测试专用的数据库
#[cfg(test)]
mod test_utils {
    use tempfile::TempDir;
    
    pub fn create_test_database() -> (Database, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(&db_path).unwrap();
        (db, temp_dir)
    }
}
```

### 3. 测试性能最佳实践

#### 并行测试执行

```toml
# Cargo.toml
[profile.test]
# 启用并行测试
test-threads = 4
```

#### 测试优化

```rust
// 使用 lazy_static 避免重复初始化
lazy_static::lazy_static! {
    static ref TEST_POOL: HighPerformanceMemoryPool = {
        HighPerformanceMemoryPool::new()
    };
}

#[test]
fn test_with_shared_pool() {
    // 使用共享的测试池
    let buffer = TEST_POOL.allocate(1024);
    assert!(buffer.is_some());
}
```

## 📈 测试指标和监控

### 1. 测试指标

#### 关键指标定义

```rust
#[derive(Debug)]
pub struct TestMetrics {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub execution_time: std::time::Duration,
    pub coverage_percentage: f64,
    pub flaky_tests: usize,
    pub slow_tests: usize,
}

impl TestMetrics {
    pub fn pass_rate(&self) -> f64 {
        if self.total_tests > 0 {
            self.passed_tests as f64 / self.total_tests as f64
        } else {
            0.0
        }
    }
    
    pub fn failure_rate(&self) -> f64 {
        if self.total_tests > 0 {
            self.failed_tests as f64 / self.total_tests as f64
        } else {
            0.0
        }
    }
    
    pub fn is_healthy(&self) -> bool {
        self.pass_rate() >= 0.95 && 
        self.coverage_percentage >= 80.0 &&
        self.flaky_tests == 0
    }
}
```

### 2. 测试监控

#### 测试结果监控

```rust
pub struct TestMonitor {
    metrics_history: Vec<TestMetrics>,
    alert_thresholds: AlertThresholds,
}

#[derive(Debug)]
pub struct AlertThresholds {
    pub min_pass_rate: f64,
    pub min_coverage: f64,
    pub max_execution_time: std::time::Duration,
    pub max_flaky_tests: usize,
}

impl TestMonitor {
    pub fn check_health(&self) -> Vec<Alert> {
        let mut alerts = Vec::new();
        let current_metrics = self.metrics_history.last().unwrap();
        
        if current_metrics.pass_rate() < self.alert_thresholds.min_pass_rate {
            alerts.push(Alert::LowPassRate(current_metrics.pass_rate()));
        }
        
        if current_metrics.coverage_percentage < self.alert_thresholds.min_coverage {
            alerts.push(Alert::LowCoverage(current_metrics.coverage_percentage));
        }
        
        if current_metrics.execution_time > self.alert_thresholds.max_execution_time {
            alerts.push(Alert::SlowExecution(current_metrics.execution_time));
        }
        
        if current_metrics.flaky_tests > self.alert_thresholds.max_flaky_tests {
            alerts.push(Alert::TooManyFlakyTests(current_metrics.flaky_tests));
        }
        
        alerts
    }
}

#[derive(Debug)]
pub enum Alert {
    LowPassRate(f64),
    LowCoverage(f64),
    SlowExecution(std::time::Duration),
    TooManyFlakyTests(usize),
}
```

---

**注意**: 本指南提供了完整的测试策略和最佳实践，建议在实际项目中根据具体需求选择合适的测试方法和工具。
