# WebAssembly 2.0 + Rust 1.90 多阶段构建Dockerfile
# 支持开发、测试和生产环境

# 基础镜像阶段
FROM rust:1.90-slim as base

# 设置工作目录
WORKDIR /app

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 安装WebAssembly工具
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install wasm-opt

# 开发阶段
FROM base as development

# 安装开发工具
RUN cargo install cargo-watch cargo-expand cargo-audit

# 复制源代码
COPY . .

# 设置环境变量
ENV RUST_BACKTRACE=1
ENV CARGO_TERM_COLOR=always

# 构建依赖
RUN cargo build --workspace

# 暴露端口
EXPOSE 8080 3000

# 开发命令
CMD ["cargo", "watch", "-x", "run"]

# 测试阶段
FROM base as testing

# 复制源代码
COPY . .

# 运行测试
RUN cargo test --workspace
RUN cargo test --package integration

# 构建阶段
FROM base as builder

# 复制源代码
COPY . .

# 构建WebAssembly模块
RUN wasm-pack build --target web --out-dir pkg wasm
RUN wasm-pack build --target web --out-dir pkg/basic examples/basic
RUN wasm-pack build --target web --out-dir pkg/advanced examples/advanced
RUN wasm-pack build --target web --out-dir pkg/performance examples/performance

# 优化WebAssembly文件
RUN wasm-opt -Os pkg/wasm_bg.wasm -o pkg/wasm_bg.wasm
RUN wasm-opt -Os pkg/basic/basic_bg.wasm -o pkg/basic/basic_bg.wasm
RUN wasm-opt -Os pkg/advanced/advanced_bg.wasm -o pkg/advanced/advanced_bg.wasm
RUN wasm-opt -Os pkg/performance/performance_bg.wasm -o pkg/performance/performance_bg.wasm

# 前端构建阶段
FROM node:18-alpine as frontend-builder

WORKDIR /app/frontend

# 复制前端文件
COPY frontend/package*.json ./
RUN npm ci --only=production

COPY frontend/ ./

# 构建前端
RUN npm run build

# 生产阶段
FROM nginx:alpine as production

# 安装Node.js用于服务端渲染
RUN apk add --no-cache nodejs npm

# 复制nginx配置
COPY docker/nginx.conf /etc/nginx/nginx.conf

# 复制构建产物
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY --from=frontend-builder /app/frontend/dist /usr/share/nginx/html

# 复制静态文件
COPY frontend/index.html /usr/share/nginx/html/

# 设置权限
RUN chown -R nginx:nginx /usr/share/nginx/html
RUN chmod -R 755 /usr/share/nginx/html

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 暴露端口
EXPOSE 8080

# 启动命令
CMD ["nginx", "-g", "daemon off;"]

# 微服务阶段
FROM base as microservice

# 复制源代码
COPY . .

# 构建微服务
RUN cargo build --release --bin wasm-service

# 创建非root用户
RUN useradd -r -s /bin/false wasmuser
USER wasmuser

# 暴露端口
EXPOSE 8080

# 启动微服务
CMD ["./target/release/wasm-service"]

# 边缘计算阶段
FROM alpine:latest as edge

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    curl \
    tzdata

# 复制WebAssembly运行时
COPY --from=builder /app/pkg /app/pkg

# 创建应用用户
RUN adduser -D -s /bin/sh wasmuser
USER wasmuser

WORKDIR /app

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 暴露端口
EXPOSE 8080

# 启动命令
CMD ["node", "server.js"]
