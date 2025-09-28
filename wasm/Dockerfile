# WebAssembly 2.0 + Rust 1.90 生产环境 Docker 配置
# WebAssembly 2.0 + Rust 1.90 Production Docker Configuration

# 多阶段构建 - 构建阶段
FROM rust:1.90-slim as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./
COPY wasm/Cargo.toml wasm/

# 构建依赖（利用 Docker 缓存）
RUN mkdir -p wasm/src && \
    echo "fn main() {}" > wasm/src/main.rs && \
    cargo build --release --package wasm && \
    rm -rf wasm/src

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release --package wasm

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r wasmuser && useradd -r -g wasmuser wasmuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/wasm /app/wasm
COPY --from=builder /app/target/release/examples/ /app/examples/

# 复制配置文件
COPY wasm/ecosystem/ecosystem_config.toml /app/config/
COPY wasm/docs/ /app/docs/

# 设置权限
RUN chown -R wasmuser:wasmuser /app

# 切换到非 root 用户
USER wasmuser

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 设置环境变量
ENV RUST_LOG=info
ENV WASM_CONFIG_PATH=/app/config/ecosystem_config.toml
ENV WASM_SECURITY_LEVEL=Maximum
ENV WASM_MONITORING_ENABLED=true

# 启动命令
CMD ["./wasm"]

# 标签
LABEL maintainer="WebAssembly Development Team"
LABEL version="2.0.0"
LABEL description="WebAssembly 2.0 + Rust 1.90 Production Service"
LABEL org.opencontainers.image.title="WebAssembly 2.0 Service"
LABEL org.opencontainers.image.description="High-performance WebAssembly 2.0 service with advanced security"
LABEL org.opencontainers.image.version="2.0.0"
LABEL org.opencontainers.image.authors="WebAssembly Development Team"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
