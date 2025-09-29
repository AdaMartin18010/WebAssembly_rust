# WebAssembly 2.0 + Rust 1.90 开源库集合

## 📚 核心运行时库

### 1. Wasmtime

- **描述**: 高性能、可嵌入的WebAssembly运行时
- **版本**: 最新稳定版
- **特性**:
  - 支持WebAssembly 2.0所有特性
  - 高性能JIT编译
  - 完整的WASI支持
  - 多语言绑定（Rust、C、Python、Go等）
- **使用场景**: 服务器端WebAssembly执行、边缘计算、插件系统
- **文档**: [https://docs.wasmtime.dev/](https://docs.wasmtime.dev/)

### 2. Wasmer

- **描述**: 通用WebAssembly运行时
- **版本**: 最新稳定版
- **特性**:
  - 多后端支持（LLVM、Cranelift、Singlepass）
  - 跨平台兼容
  - 丰富的语言绑定
  - 企业级功能
- **使用场景**: 云原生应用、微服务、容器化
- **文档**: [https://docs.wasmer.io/](https://docs.wasmer.io/)

### 3. WasmEdge

- **描述**: 轻量级、高性能的WebAssembly运行时
- **版本**: 最新稳定版
- **特性**:
  - 专为边缘计算优化
  - 支持TensorFlow、PyTorch
  - 容器化支持
  - 云原生集成
- **使用场景**: AI推理、边缘计算、IoT应用
- **文档**: [https://wasmedge.org/docs/](https://wasmedge.org/docs/)

## 🛠️ 开发工具库

### 1. wasm-bindgen

- **描述**: Rust和JavaScript之间的绑定生成器
- **版本**: 0.2.104
- **特性**:
  - 自动生成绑定代码
  - 类型安全的互操作
  - 支持复杂数据类型
  - 异步函数支持
- **使用场景**: 前端WebAssembly开发、浏览器集成
- **文档**: [https://rustwasm.github.io/wasm-bindgen/](https://rustwasm.github.io/wasm-bindgen/)

### 2. wasm-pack

- **描述**: 构建、测试和发布WebAssembly包的工具
- **版本**: 最新稳定版
- **特性**:
  - 自动化构建流程
  - 多目标支持（浏览器、Node.js、Bundler）
  - 包管理集成
  - 测试支持
- **使用场景**: WebAssembly包开发、发布和分发
- **文档**: [https://rustwasm.github.io/wasm-pack/](https://rustwasm.github.io/wasm-pack/)

### 3. wasm-opt (Binaryen)

- **描述**: WebAssembly优化工具
- **版本**: 最新稳定版
- **特性**:
  - 代码大小优化
  - 性能优化
  - 死代码消除
  - 常量折叠
- **使用场景**: 生产环境优化、性能调优
- **文档**: [https://github.com/WebAssembly/binaryen](https://github.com/WebAssembly/binaryen/)

## 🔒 安全工具库

### 1. Wasmati

- **描述**: WebAssembly安全分析工具
- **版本**: 最新稳定版
- **特性**:
  - 静态分析
  - 漏洞检测
  - 安全策略验证
  - 威胁建模
- **使用场景**: 安全审计、漏洞扫描
- **文档**: [https://github.com/wasmati/wasmati](https://github.com/wasmati/wasmati)

### 2. Wasabi

- **描述**: WebAssembly动态分析框架
- **版本**: 最新稳定版
- **特性**:
  - 动态分析
  - 污点分析
  - 行为监控
  - 恶意代码检测
- **使用场景**: 恶意软件分析、行为监控
- **文档**: [https://github.com/danielkunen/wasabi](https://github.com/danielkunen/wasabi)

### 3. Twine

- **描述**: WebAssembly模糊测试工具
- **版本**: 最新稳定版
- **特性**:
  - 智能模糊测试
  - 覆盖率引导
  - 崩溃检测
  - 变异策略
- **使用场景**: 漏洞发现、质量保证
- **文档**: [https://github.com/MozillaSecurity/twine](https://github.com/MozillaSecurity/twine)

## 🎨 前端框架库

### 1. Yew

- **描述**: 基于Rust的现代Web框架
- **版本**: 0.21
- **特性**:
  - 组件化架构
  - 虚拟DOM
  - 状态管理
  - 路由支持
- **使用场景**: 单页应用、Web界面开发
- **文档**: [https://yew.rs/](https://yew.rs/)

### 2. Seed

- **描述**: 轻量级Web应用框架
- **版本**: 0.10
- **特性**:
  - 函数式编程
  - 简洁的API
  - 高性能
  - 类型安全
- **使用场景**: 简单Web应用、原型开发
- **文档**: [https://seed-rs.org/](https://seed-rs.org/)

### 3. Dioxus

- **描述**: 跨平台UI框架
- **版本**: 0.4
- **特性**:
  - 跨平台支持
  - 响应式设计
  - 热重载
  - 组件复用
- **使用场景**: 跨平台应用、桌面应用
- **文档**: [https://dioxuslabs.com/](https://dioxuslabs.com/)

### 4. Leptos

- **描述**: 全栈Web框架
- **版本**: 0.5
- **特性**:
  - 服务端渲染
  - 客户端水合
  - 响应式系统
  - 类型安全
- **使用场景**: 全栈Web应用、SSR应用
- **文档**: [https://leptos.dev/](https://leptos.dev/)

## 🖥️ 桌面应用库

### 1. Tauri

- **描述**: 构建桌面应用的框架
- **版本**: 1.5
- **特性**:
  - 小体积应用
  - 安全性优先
  - 跨平台支持
  - 现代Web技术
- **使用场景**: 桌面应用、跨平台工具
- **文档**: [https://tauri.app/](https://tauri.app/)

### 2. Egui

- **描述**: 即时模式GUI框架
- **版本**: 0.23
- **特性**:
  - 即时模式
  - 跨平台
  - 高性能
  - 简单易用
- **使用场景**: 游戏UI、工具界面
- **文档**: [https://www.egui.rs/](https://www.egui.rs/)

### 3. Iced

- **描述**: 受Elm启发的GUI框架
- **版本**: 0.10
- **特性**:
  - 函数式编程
  - 响应式架构
  - 跨平台
  - 类型安全
- **使用场景**: 复杂桌面应用、数据可视化
- **文档**: [https://iced.rs/](https://iced.rs/)

## 🧮 数学和科学计算库

### 1. ndarray

- **描述**: N维数组库
- **版本**: 0.15
- **特性**:
  - 多维数组
  - 线性代数
  - 数值计算
  - 并行计算
- **使用场景**: 科学计算、机器学习
- **文档**: [https://docs.rs/ndarray/](https://docs.rs/ndarray/)

### 2. nalgebra

- **描述**: 线性代数库
- **版本**: 0.32
- **特性**:
  - 矩阵运算
  - 向量计算
  - 几何变换
  - 优化算法
- **使用场景**: 3D图形、物理仿真
- **文档**: [https://nalgebra.org/](https://nalgebra.org/)

### 3. petgraph

- **描述**: 图数据结构库
- **版本**: 0.6
- **特性**:
  - 图算法
  - 网络分析
  - 路径查找
  - 拓扑排序
- **使用场景**: 网络分析、算法实现
- **文档**: [https://docs.rs/petgraph/](https://docs.rs/petgraph/)

## 🔗 网络和异步库

### 1. Tokio

- **描述**: 异步运行时
- **版本**: 1.32
- **特性**:
  - 异步I/O
  - 任务调度
  - 网络编程
  - 定时器
- **使用场景**: 网络服务、异步应用
- **文档**: [https://tokio.rs/](https://tokio.rs/)

### 2. Reqwest

- **描述**: HTTP客户端
- **版本**: 0.11
- **特性**:
  - 异步HTTP请求
  - JSON支持
  - 代理支持
  - 重试机制
- **使用场景**: API调用、网络请求
- **文档**: [https://docs.rs/reqwest/](https://docs.rs/reqwest/)

### 3. Axum

- **描述**: Web应用框架
- **版本**: 0.7
- **特性**:
  - 模块化设计
  - 类型安全
  - 中间件支持
  - 异步支持
- **使用场景**: Web服务、API开发
- **文档**: [https://docs.rs/axum/](https://docs.rs/axum/)

## 🗄️ 数据存储库

### 1. Serde

- **描述**: 序列化/反序列化框架
- **版本**: 1.0
- **特性**:
  - 多格式支持
  - 零拷贝
  - 类型安全
  - 高性能
- **使用场景**: 数据序列化、API通信
- **文档**: [https://serde.rs/](https://serde.rs/)

### 2. SQLx

- **描述**: 异步SQL工具包
- **版本**: 0.7
- **特性**:
  - 编译时检查
  - 多数据库支持
  - 异步操作
  - 连接池
- **使用场景**: 数据库操作、数据持久化
- **文档**: [https://docs.rs/sqlx/](https://docs.rs/sqlx/)

### 3. Redis

- **描述**: Redis客户端
- **版本**: 0.23
- **特性**:
  - 异步操作
  - 连接池
  - 集群支持
  - 发布/订阅
- **使用场景**: 缓存、消息队列
- **文档**: [https://docs.rs/redis/](https://docs.rs/redis/)

## 🧪 测试和基准测试库

### 1. Criterion

- **描述**: 统计驱动的基准测试
- **版本**: 0.5
- **特性**:
  - 统计分析
  - 回归检测
  - 图表生成
  - 噪声过滤
- **使用场景**: 性能测试、基准测试
- **文档**: [https://docs.rs/criterion/](https://docs.rs/criterion/)

### 2. rstest

- **描述**: 参数化测试框架
- **版本**: 0.18
- **特性**:
  - 参数化测试
  - 夹具支持
  - 异步测试
  - 并行执行
- **使用场景**: 单元测试、集成测试
- **文档**: [https://docs.rs/rstest/](https://docs.rs/rstest/)

### 3. proptest

- **描述**: 属性测试框架
- **版本**: 1.4
- **特性**:
  - 随机测试
  - 收缩算法
  - 自定义策略
  - 回归测试
- **使用场景**: 属性测试、模糊测试
- **文档**: [https://docs.rs/proptest/](https://docs.rs/proptest/)

## 📊 监控和日志库

### 1. Tracing

- **描述**: 结构化日志和追踪
- **版本**: 0.1
- **特性**:
  - 结构化日志
  - 分布式追踪
  - 性能监控
  - 异步支持
- **使用场景**: 应用监控、性能分析
- **文档**: [https://docs.rs/tracing/](https://docs.rs/tracing/)

### 2. Log

- **描述**: 日志门面
- **版本**: 0.4
- **特性**:
  - 统一接口
  - 多后端支持
  - 性能优化
  - 配置灵活
- **使用场景**: 应用日志、调试信息
- **文档**: [https://docs.rs/log/](https://docs.rs/log/)

### 3. Metrics

- **描述**: 指标收集库
- **版本**: 0.22
- **特性**:
  - 多种指标类型
  - 标签支持
  - 导出器
  - 异步收集
- **使用场景**: 性能监控、业务指标
- **文档**: [https://docs.rs/metrics/](https://docs.rs/metrics/)

## 🚀 使用建议

### 1. 选择合适的运行时

- **服务器端**: Wasmtime（高性能）、Wasmer（通用性）
- **边缘计算**: WasmEdge（轻量级）
- **浏览器**: 原生WebAssembly API

### 2. 开发工具选择

- **绑定生成**: wasm-bindgen
- **构建工具**: wasm-pack
- **优化工具**: wasm-opt

### 3. 前端框架选择

- **复杂应用**: Yew、Leptos
- **简单应用**: Seed
- **跨平台**: Dioxus

### 4. 桌面应用选择

- **现代Web技术**: Tauri
- **游戏UI**: Egui
- **复杂界面**: Iced

### 5. 性能优化

- 使用wasm-opt进行代码优化
- 选择合适的编译目标
- 利用WebAssembly 2.0新特性
- 进行基准测试和性能分析

## 📚 学习资源

### 官方文档

- [WebAssembly官方文档](https://webassembly.org/)
- [Rust WebAssembly工作组](https://rustwasm.github.io/)
- [WASI规范](https://wasi.dev/)

### 社区资源

- [WebAssembly中文社区](https://wasm-cn.org/)
- [Rust中文社区](https://rustcc.cn/)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/webassembly+rust)

### 教程和指南

- [WebAssembly入门教程](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Rust WebAssembly教程](https://rustwasm.github.io/book/)
- [Wasmtime教程](https://docs.wasmtime.dev/tutorial.html)

## 🔄 版本更新

本库集合会定期更新，确保包含最新的稳定版本。建议：

1. 定期检查库的更新
2. 关注安全公告
3. 测试新版本兼容性
4. 更新依赖关系

## 🤝 贡献

欢迎为这个库集合贡献：

1. 添加新的库
2. 更新库信息
3. 改进文档
4. 报告问题

请通过GitHub Issues或Pull Request参与贡献。
