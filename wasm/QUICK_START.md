# WebAssembly 2.0 + Rust 1.90 快速启动指南

-Quick Start Guide for WebAssembly 2.0 + Rust 1.90

## 🚀 快速开始

### 1. 环境要求

- **Rust**: 1.90+ (最新稳定版)
- **Cargo**: 最新版本
- **Docker**: 20.10+ (可选，用于容器化部署)
- **Kubernetes**: 1.25+ (可选，用于云原生部署)

### 2. 克隆和构建

```bash
# 克隆项目
git clone <repository-url>
cd WebAssembly_rust

# 构建项目
cargo build --release

# 运行测试
cargo test

# 运行基准测试
cargo bench
```

### 3. 运行示例

```bash
# 运行基础示例
cargo run --example rust_190_demo

# 运行 WebAssembly 2.0 高级示例
cargo run --example webassembly_2_0_advanced_demo

# 运行生产部署示例
cargo run --example production_deployment_demo
```

### 4. 开发工具使用

```bash
# 初始化开发环境
cargo run --example developer_tools_demo

# 代码生成
# 使用 CodeGenerator 生成 WebAssembly 模块代码

# 性能分析
cargo bench --bench advanced_performance_benchmarks

# 安全扫描
# 使用 AdvancedSecurityManager 进行安全检测
```

## 🐳 容器化部署

### Docker 部署

```bash
# 构建镜像
docker build -t webassembly-2.0-service:latest .

# 运行容器
docker run -p 8080:8080 \
  -e WASM_SECURITY_LEVEL=Maximum \
  -e WASM_MONITORING_ENABLED=true \
  webassembly-2.0-service:latest
```

### Docker Compose 部署

```bash
# 启动完整服务栈
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f wasm-service
```

## ☸️ Kubernetes 部署

### 部署到 Kubernetes

```bash
# 创建命名空间
kubectl apply -f k8s/webassembly-deployment.yaml

# 查看部署状态
kubectl get pods -n webassembly-ecosystem

# 查看服务
kubectl get services -n webassembly-ecosystem

# 查看 Ingress
kubectl get ingress -n webassembly-ecosystem
```

### 监控和日志

```bash
# 访问 Prometheus
kubectl port-forward svc/prometheus-service 9090:9090 -n webassembly-ecosystem

# 访问 Grafana
kubectl port-forward svc/grafana 3000:3000 -n webassembly-ecosystem

# 访问 Kibana
kubectl port-forward svc/kibana 5601:5601 -n webassembly-ecosystem
```

## 🔧 配置说明

### 环境变量

```bash
# 基础配置
RUST_LOG=info                          # 日志级别
WASM_CONFIG_PATH=/app/config/ecosystem_config.toml  # 配置文件路径

# 安全配置
WASM_SECURITY_LEVEL=Maximum            # 安全级别 (Low/Medium/High/Maximum)
WASM_MONITORING_ENABLED=true           # 启用监控

# 性能配置
WASM_MEMORY_LIMIT=256MB                # 内存限制
WASM_CPU_LIMIT=2                       # CPU 限制

# 服务配置
WASM_LISTEN_PORT=8080                  # 监听端口
WASM_MAX_CONNECTIONS=1000              # 最大连接数
```

### 配置文件

主要配置文件位于 `wasm/ecosystem/ecosystem_config.toml`，包含：

- 生态系统配置
- 性能基准设置
- 安全策略配置
- 部署环境设置
- 监控配置
- 测试配置

## 📊 性能监控

### 指标端点

- **健康检查**: `GET /health`
- **就绪检查**: `GET /ready`
- **性能指标**: `GET /metrics`
- **安全报告**: `GET /security/report`

### 关键指标

- **性能指标**: CPU 使用率、内存使用量、请求延迟、吞吐量
- **安全指标**: 威胁检测数、安全事件数、策略违规数
- **业务指标**: 请求计数、错误率、成功率

## 🔒 安全配置

### 安全级别

1. **Low**: 基本安全检查，适用于开发环境
2. **Medium**: 标准安全检查，适用于测试环境
3. **High**: 严格安全检查，适用于预生产环境
4. **Maximum**: 最高安全级别，适用于生产环境

### 威胁检测

系统支持以下威胁类型检测：

- 缓冲区溢出 (Buffer Overflow)
- 代码注入 (Code Injection)
- 内存泄漏 (Memory Leak)
- 越界访问 (Out of Bounds Access)
- 释放后使用 (Use After Free)
- 更多威胁类型...

## 🛠️ 开发工具

### 代码生成

```rust
use wasm::developer_tools::*;

let mut code_generator = CodeGenerator::new();
let spec = ModuleSpecification {
    name: "my_module".to_string(),
    // ... 其他配置
};
let generated_code = code_generator.generate_wasm_module(spec)?;
```

### 调试支持

```rust
use wasm::developer_tools::*;

let mut debugger = WasmDebugger::new();
debugger.set_breakpoint(Breakpoint {
    module_id: module_id,
    function_index: 0,
    instruction_index: 10,
    // ... 其他配置
});
```

### 性能分析

```rust
use wasm::developer_tools::*;

let mut profiler = WasmProfiler::new();
profiler.start_profiling(module_id)?;
// ... 执行代码
let report = profiler.generate_performance_report(&module_id)?;
```

## 🧪 测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_webassembly_2_0

# 运行性能测试
cargo bench

# 运行安全测试
cargo test security_tests
```

### 测试覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

## 📚 文档

### 文档生成

```bash
# 生成 API 文档
cargo doc --open

# 生成项目文档
cargo doc --document-private-items
```

### 文档位置

- **API 文档**: `target/doc/`
- **项目文档**: `docs/`
- **示例文档**: `examples/`
- **生态系统文档**: `ecosystem/`

## 🆘 故障排除

### 常见问题

1. **编译错误**: 确保使用 Rust 1.90+ 版本
2. **运行时错误**: 检查环境变量和配置文件
3. **性能问题**: 查看性能监控指标
4. **安全警报**: 检查安全报告和日志

### 日志查看

```bash
# Docker 容器日志
docker logs webassembly-2.0-service

# Kubernetes Pod 日志
kubectl logs -f deployment/wasm-service -n webassembly-ecosystem

# 本地运行日志
RUST_LOG=debug cargo run
```

### 性能调优

1. **内存优化**: 调整 `WASM_MEMORY_LIMIT`
2. **CPU 优化**: 调整 `WASM_CPU_LIMIT`
3. **并发优化**: 调整 `WASM_MAX_CONNECTIONS`
4. **缓存优化**: 配置 Redis 缓存策略

## 🔗 相关链接

- **项目仓库**: [GitHub Repository]
- **文档网站**: [Documentation Site]
- **社区论坛**: [Community Forum]
- **问题报告**: [Issue Tracker]
- **贡献指南**: [Contributing Guide]

## 📞 支持

- **GitHub Issues**: 技术问题和功能请求
- **社区论坛**: 一般讨论和支持
- **邮件支持**: <support@webassembly-ecosystem.org>
- **文档 Wiki**: 详细的文档和教程

---

**快速启动指南版本**: 2.0.0  
**最后更新**: 2025年9月27日  
**适用版本**: WebAssembly 2.0 + Rust 1.90 Integration v2.0.0
