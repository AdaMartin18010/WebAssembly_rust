# 🚀 WebAssembly 2.0 + Rust 1.90 部署指南

## 📋 部署概览

本指南提供了WebAssembly 2.0 + Rust 1.90项目的完整部署方案，支持开发、测试和生产环境。

## 🎯 部署选项

### 1. 本地开发部署

- **用途**: 开发和调试
- **环境**: 本地开发环境
- **特点**: 热重载、调试支持、快速迭代

### 2. 容器化部署

- **用途**: 测试和生产环境
- **环境**: Docker容器
- **特点**: 环境一致性、可扩展性、易于管理

### 3. 云原生部署

- **用途**: 生产环境
- **环境**: Kubernetes集群
- **特点**: 高可用性、自动扩缩容、服务发现

## 🛠️ 部署前准备

### 系统要求

- **操作系统**: Windows 10+, macOS 10.15+, Ubuntu 18.04+
- **内存**: 最少4GB，推荐8GB+
- **存储**: 最少10GB可用空间
- **网络**: 稳定的互联网连接

### 软件依赖

- **Rust**: 1.90+
- **Node.js**: 18+
- **Docker**: 20.10+
- **Docker Compose**: 2.0+

## 🚀 快速部署

### 1. 克隆项目

```bash
git clone https://github.com/your-org/webassembly-rust.git
cd webassembly-rust
```

### 2. 设置开发环境

```bash
# Windows
.\scripts\setup-dev.ps1 -Full

# macOS/Linux
chmod +x scripts/setup-dev.sh
./scripts/setup-dev.sh --full
```

### 3. 构建项目

```bash
# Windows
.\scripts\build.ps1

# macOS/Linux
./scripts/build.sh
```

### 4. 运行测试

```bash
# Windows
.\scripts\build.ps1 -Test

# macOS/Linux
./scripts/build.sh --test
```

## 🐳 容器化部署

### 开发环境

```bash
# 启动开发环境
docker-compose up -d wasm-dev

# 查看日志
docker-compose logs -f wasm-dev

# 停止服务
docker-compose down
```

### 测试环境

```bash
# 启动测试环境
docker-compose --profile testing up -d

# 运行测试
docker-compose exec wasm-test cargo test

# 清理测试环境
docker-compose --profile testing down
```

### 生产环境

```bash
# 启动生产环境
docker-compose --profile production up -d

# 查看服务状态
docker-compose ps

# 健康检查
curl http://localhost/health
```

### 完整环境（包含监控和日志）

```bash
# 启动所有服务
docker-compose --profile production --profile monitoring --profile logging up -d

# 访问服务
# 应用: http://localhost
# Prometheus: http://localhost:9090
# Grafana: http://localhost:3001
# Kibana: http://localhost:5601
```

## ☸️ Kubernetes部署

### 1. 创建命名空间

```bash
kubectl create namespace wasm-rust
```

### 2. 部署应用

```bash
# 部署WebAssembly应用
kubectl apply -f k8s/webassembly-deployment.yaml -n wasm-rust

# 部署服务
kubectl apply -f k8s/webassembly-service.yaml -n wasm-rust

# 部署Ingress
kubectl apply -f k8s/webassembly-ingress.yaml -n wasm-rust
```

### 3. 检查部署状态

```bash
# 查看Pod状态
kubectl get pods -n wasm-rust

# 查看服务状态
kubectl get services -n wasm-rust

# 查看Ingress状态
kubectl get ingress -n wasm-rust
```

### 4. 访问应用

```bash
# 获取访问地址
kubectl get ingress -n wasm-rust

# 端口转发（用于本地访问）
kubectl port-forward service/wasm-service 8080:80 -n wasm-rust
```

## 🔧 配置管理

### 环境变量

```bash
# 开发环境
export NODE_ENV=development
export RUST_LOG=debug
export WASM_CACHE_SIZE=1000

# 生产环境
export NODE_ENV=production
export RUST_LOG=info
export WASM_CACHE_SIZE=10000
```

### 配置文件

```toml
# config.toml
[app]
name = "WebAssembly Rust App"
version = "1.0.0"
environment = "production"

[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
host = "localhost"
port = 5432
name = "wasm_db"
user = "wasm_user"
password = "wasm_password"

[redis]
host = "localhost"
port = 6379
password = ""
db = 0

[monitoring]
enabled = true
prometheus_port = 9090
grafana_port = 3001
```

## 📊 监控和日志

### Prometheus监控

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'wasm-app'
    static_configs:
      - targets: ['wasm-app:8080']
    metrics_path: '/metrics'
    scrape_interval: 10s
```

### Grafana仪表板

```json
{
  "dashboard": {
    "title": "WebAssembly Rust App",
    "panels": [
      {
        "title": "请求率",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      }
    ]
  }
}
```

### 日志配置

```yaml
# logback.xml
<configuration>
  <appender name="STDOUT" class="ch.qos.logback.core.ConsoleAppender">
    <encoder>
      <pattern>%d{HH:mm:ss.SSS} [%thread] %-5level %logger{36} - %msg%n</pattern>
    </encoder>
  </appender>

  <root level="INFO">
    <appender-ref ref="STDOUT" />
  </root>
</configuration>
```

## 🔒 安全配置

### SSL/TLS配置

```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
}
```

### 防火墙配置

```bash
# UFW配置
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

### 安全头配置

```nginx
# 安全头
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header X-Content-Type-Options "nosniff" always;
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
```

## 🚀 性能优化

### 构建优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 运行时优化

```bash
# 环境变量
export RUST_LOG=warn
export WASM_CACHE_SIZE=10000
export WORKER_THREADS=4
export MAX_CONNECTIONS=1000
```

### 缓存配置

```nginx
# 缓存配置
location ~* \.(wasm|js|css|png|jpg|jpeg|gif|ico|svg)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

## 🔄 CI/CD部署

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to production
        run: |
          docker-compose -f docker-compose.prod.yml up -d
```

### 自动化部署脚本

```bash
#!/bin/bash
# deploy.sh

set -e

echo "🚀 开始部署..."

# 构建镜像
docker-compose build

# 运行测试
docker-compose run --rm test

# 部署到生产环境
docker-compose -f docker-compose.prod.yml up -d

# 健康检查
curl -f http://localhost/health || exit 1

echo "✅ 部署完成！"
```

## 🐛 故障排除

### 常见问题

#### 1. 容器启动失败

```bash
# 查看容器日志
docker-compose logs wasm-prod

# 检查容器状态
docker-compose ps

# 重启服务
docker-compose restart wasm-prod
```

#### 2. 端口冲突

```bash
# 查看端口使用情况
netstat -tulpn | grep :8080

# 修改端口配置
# 在docker-compose.yml中修改端口映射
```

#### 3. 内存不足

```bash
# 查看内存使用情况
docker stats

# 增加内存限制
# 在docker-compose.yml中设置mem_limit
```

#### 4. 网络连接问题

```bash
# 检查网络连接
docker network ls
docker network inspect wasm-network

# 重建网络
docker-compose down
docker network prune
docker-compose up -d
```

### 日志分析

```bash
# 查看应用日志
docker-compose logs -f wasm-prod

# 查看系统日志
journalctl -u docker

# 查看Nginx日志
docker-compose logs -f nginx-lb
```

## 📈 扩展和升级

### 水平扩展

```bash
# 扩展服务实例
docker-compose up -d --scale wasm-prod=3

# 使用负载均衡器
docker-compose up -d nginx-lb
```

### 版本升级

```bash
# 构建新版本
docker-compose build

# 滚动更新
docker-compose up -d --no-deps wasm-prod

# 回滚
docker-compose down
docker-compose up -d
```

### 数据迁移

```bash
# 备份数据
docker-compose exec postgres pg_dump -U wasm_user wasm_db > backup.sql

# 恢复数据
docker-compose exec -T postgres psql -U wasm_user wasm_db < backup.sql
```

## 📞 支持和帮助

### 获取帮助

- **文档**: 查看项目文档
- **Issues**: 提交GitHub Issues
- **社区**: 参与社区讨论
- **邮件**: 联系技术支持

### 贡献指南

- **代码贡献**: 提交Pull Request
- **文档改进**: 改进文档质量
- **问题报告**: 报告Bug和问题
- **功能建议**: 提出新功能建议

---

*本部署指南最后更新于2025年9月，反映了最新的部署最佳实践。*
