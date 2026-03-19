# WebAssembly 3.0 + Rust 1.94 完整集成项目

[![Rust Version](https://img.shields.io/badge/rust-1.94-blue.svg)](https://releases.rs/docs/1.94.0/)
[![WebAssembly](https://img.shields.io/badge/webassembly-3.0-brightgreen.svg)](https://webassembly.org/)
[![WASI](https://img.shields.io/badge/wasi-0.3-orange.svg)](https://wasi.dev/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

> 完整的 WebAssembly 3.0、Rust 1.94 新特性、WASI 0.3 和 Component Model 集成项目

[English](#english) | [中文](#中文)

---

## 中文

### 🚀 项目简介

本项目提供了 Rust 1.94 与 WebAssembly 3.0 的完整集成实现，包含最新的 WASI 0.3 原生异步支持和 Component Model 多语言组合能力。

### ✨ 主要特性

- **🦀 Rust 1.94 新特性**
  - `array_windows` - 常量长度窗口迭代
  - 数学常量 (`EULER_GAMMA`, `GOLDEN_RATIO`)
  - AVX-512 FP16 / NEON FP16 SIMD
  - 增强的 `LazyLock` 支持

- **🌐 WebAssembly 3.0**
  - **WasmGC** - 垃圾回收支持
  - **Memory64** - 64位内存寻址（突破4GB限制）
  - **Exception Handling (exnref)** - 增强异常处理
  - 引用类型和多值返回

- **⚡ WASI 0.3**
  - 原生 `async`/`await` 支持
  - `stream<T>` 和 `future<T>` 类型
  - 取消令牌集成
  - HTTP 客户端原生异步

- **🧩 Component Model**
  - WIT 接口定义
  - 组件创建与组合
  - 多语言互操作

### 📦 快速开始

```bash
# 克隆项目
git clone https://github.com/your-org/webassembly-rust.git
cd webassembly-rust

# 构建项目
cargo build --all --release

# 运行测试
cargo test --all

# 运行示例
cargo run --example rust_194_demo
cargo run --example webassembly_3_0_demo
cargo run --example wasi_03_demo --features wasi-03
cargo run --example component_model_demo --features webassembly-3-0
```

### 📚 文档

- [升级完成报告](PROJECT_UPDATE_COMPLETE.md)
- [Rust 1.94 更新报告](RUST_194_WEBASSEMBLY_UPDATE_REPORT.md)
- [实施计划](IMPLEMENTATION_PLAN.md)

### 🏗️ 项目结构

```
├── wasm/                   # 核心库
│   ├── src/
│   │   ├── rust_194_features.rs    # Rust 1.94 特性
│   │   ├── webassembly_3_0.rs      # WebAssembly 3.0
│   │   ├── wasi_03.rs              # WASI 0.3
│   │   ├── component_model.rs      # Component Model
│   │   └── ...
│   └── examples/           # 示例代码
├── examples/               # 示例项目
└── frontend/              # 前端集成
```

### 🧪 测试

```bash
# 运行所有测试
cargo test --all

# 运行基准测试
cargo bench --features bench

# 检查代码
cargo clippy --all
```

### 📄 许可证

本项目采用 MIT 或 Apache-2.0 双许可证。

---

## English

### 🚀 Introduction

This project provides complete integration of Rust 1.94 with WebAssembly 3.0, including the latest WASI 0.3 native async support and Component Model polyglot composition capabilities.

### ✨ Key Features

- **🦀 Rust 1.94 Features**
  - `array_windows` - Constant-length window iteration
  - Math constants (`EULER_GAMMA`, `GOLDEN_RATIO`)
  - AVX-512 FP16 / NEON FP16 SIMD
  - Enhanced `LazyLock` support

- **🌐 WebAssembly 3.0**
  - **WasmGC** - Garbage collection support
  - **Memory64** - 64-bit memory addressing (>4GB)
  - **Exception Handling (exnref)** - Enhanced exception handling
  - Reference types and multi-value returns

- **⚡ WASI 0.3**
  - Native `async`/`await` support
  - `stream<T>` and `future<T>` types
  - Cancellation token integration
  - Native async HTTP client

- **🧩 Component Model**
  - WIT interface definitions
  - Component creation and composition
  - Polyglot interoperability

### 📦 Quick Start

```bash
# Clone the project
git clone https://github.com/your-org/webassembly-rust.git
cd webassembly-rust

# Build the project
cargo build --all --release

# Run tests
cargo test --all

# Run examples
cargo run --example rust_194_demo
cargo run --example webassembly_3_0_demo
cargo run --example wasi_03_demo --features wasi-03
cargo run --example component_model_demo --features webassembly-3-0
```

### 📚 Documentation

- [Project Update Report](PROJECT_UPDATE_COMPLETE.md)
- [Rust 1.94 Update Report](RUST_194_WEBASSEMBLY_UPDATE_REPORT.md)
- [Implementation Plan](IMPLEMENTATION_PLAN.md)

### 🏗️ Project Structure

```
├── wasm/                   # Core library
│   ├── src/
│   │   ├── rust_194_features.rs    # Rust 1.94 features
│   │   ├── webassembly_3_0.rs      # WebAssembly 3.0
│   │   ├── wasi_03.rs              # WASI 0.3
│   │   ├── component_model.rs      # Component Model
│   │   └── ...
│   └── examples/           # Example code
├── examples/               # Example projects
└── frontend/              # Frontend integration
```

### 🧪 Testing

```bash
# Run all tests
cargo test --all

# Run benchmarks
cargo bench --features bench

# Check code
cargo clippy --all
```

### 📄 License

This project is dual-licensed under MIT or Apache-2.0.

---

## 🔗 Resources

- [Rust 1.94 Release Notes](https://releases.rs/docs/1.94.0/)
- [WebAssembly 3.0 Announcement](https://webassembly.org/)
- [WASI Roadmap](https://wasi.dev/roadmap)
- [Component Model](https://component-model.bytecodealliance.org/)

---

**Version**: 0.2.0  
**Last Updated**: 2026-03-19
