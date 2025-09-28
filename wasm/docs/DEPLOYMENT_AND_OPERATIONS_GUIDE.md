# WebAssembly 2.0 + Rust 1.90 部署和运维指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 项目的完整部署和运维解决方案，包括多环境部署、容器化、监控、日志管理、性能优化等全方位的运维最佳实践。

## 🎯 部署架构

### 1. 整体架构

#### 部署层次结构

```text
用户请求 → 负载均衡 → CDN → 应用服务器 → WebAssembly运行时 → 数据库/缓存
    ↓         ↓        ↓        ↓             ↓              ↓
  用户层    网络层   缓存层   应用层        运行时层        数据层
```

#### 技术栈选择

- **负载均衡**: Nginx, HAProxy, AWS ALB
- **CDN**: CloudFlare, AWS CloudFront, 阿里云CDN
- **容器化**: Docker, Kubernetes, Docker Compose
- **Web服务器**: Nginx, Apache, Caddy
- **监控**: Prometheus, Grafana, Jaeger
- **日志**: ELK Stack, Fluentd, Loki
- **数据库**: PostgreSQL, Redis, MongoDB

### 2. 环境配置

#### 开发环境

- **目标**: 快速开发和调试
- **特点**: 热重载、详细日志、调试工具
- **资源**: 最小化资源配置

#### 测试环境

- **目标**: 功能测试和集成测试
- **特点**: 接近生产环境、自动化测试
- **资源**: 中等资源配置

#### 预生产环境

- **目标**: 性能测试和压力测试
- **特点**: 与生产环境完全一致
- **资源**: 生产级资源配置

#### 生产环境

- **目标**: 高可用性和高性能
- **特点**: 负载均衡、监控告警、自动扩缩容
- **资源**: 高可用资源配置

## 🐳 容器化部署

### 1. Docker 配置

#### 基础 Dockerfile

```dockerfile
# 多阶段构建 Dockerfile
FROM rust:1.90-slim as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./
COPY wasm/Cargo.toml ./wasm/

# 构建依赖（利用Docker缓存）
RUN mkdir wasm/src && echo "fn main() {}" > wasm/src/main.rs
RUN cargo build --release
RUN rm -rf wasm/src

# 复制源代码
COPY . .

# 安装 wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 构建 WebAssembly
RUN wasm-pack build wasm --target web --out-dir pkg

# 构建前端
RUN npm ci && npm run build

# 生产镜像
FROM nginx:alpine

# 复制构建产物
COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/pkg /usr/share/nginx/html/wasm

# 复制 Nginx 配置
COPY nginx.conf /etc/nginx/nginx.conf

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/health || exit 1

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

#### Nginx 配置

```nginx
# nginx.conf
user nginx;
worker_processes auto;
error_log /var/log/nginx/error.log warn;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use epoll;
    multi_accept on;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # 日志格式
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    # 性能优化
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    client_max_body_size 16M;

    # Gzip 压缩
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types
        text/plain
        text/css
        text/xml
        text/javascript
        application/json
        application/javascript
        application/xml+rss
        application/atom+xml
        image/svg+xml;

    # 安全头
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Referrer-Policy "strict-origin-when-cross-origin";

    # 上游服务器
    upstream wasm_backend {
        server wasm-app:3000;
        keepalive 32;
    }

    server {
        listen 80;
        server_name _;
        root /usr/share/nginx/html;
        index index.html;

        # 健康检查端点
        location /health {
            access_log off;
            return 200 "healthy\n";
            add_header Content-Type text/plain;
        }

        # WebAssembly 文件
        location ~* \.(wasm)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
            add_header Cross-Origin-Embedder-Policy require-corp;
            add_header Cross-Origin-Opener-Policy same-origin;
        }

        # 静态资源
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }

        # API 代理
        location /api/ {
            proxy_pass http://wasm_backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_cache_bypass $http_upgrade;
        }

        # SPA 路由
        location / {
            try_files $uri $uri/ /index.html;
        }
    }
}
```

### 2. Docker Compose 配置

#### 开发环境配置

```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  # WebAssembly 应用
  wasm-app:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "3000:3000"
    volumes:
      - .:/app
      - /app/node_modules
    environment:
      - NODE_ENV=development
      - RUST_LOG=debug
    depends_on:
      - redis
      - postgres
    networks:
      - wasm-network

  # 前端开发服务器
  frontend:
    build:
      context: .
      dockerfile: Dockerfile.frontend
    ports:
      - "3001:3001"
    volumes:
      - ./frontend:/app
      - /app/node_modules
    environment:
      - NODE_ENV=development
    networks:
      - wasm-network

  # Redis 缓存
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes
    networks:
      - wasm-network

  # PostgreSQL 数据库
  postgres:
    image: postgres:15-alpine
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_DB=wasm_app
      - POSTGRES_USER=wasm_user
      - POSTGRES_PASSWORD=wasm_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - wasm-network

  # 监控
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
    networks:
      - wasm-network

  # Grafana 可视化
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3002:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana
    networks:
      - wasm-network

volumes:
  redis_data:
  postgres_data:
  prometheus_data:
  grafana_data:

networks:
  wasm-network:
    driver: bridge
```

#### 生产环境配置

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  # Nginx 负载均衡
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
      - nginx_logs:/var/log/nginx
    depends_on:
      - wasm-app
    networks:
      - wasm-network
    restart: unless-stopped

  # WebAssembly 应用（多实例）
  wasm-app:
    build:
      context: .
      dockerfile: Dockerfile
    environment:
      - NODE_ENV=production
      - RUST_LOG=info
      - DATABASE_URL=postgresql://wasm_user:wasm_password@postgres:5432/wasm_app
      - REDIS_URL=redis://redis:6379
    depends_on:
      - redis
      - postgres
    networks:
      - wasm-network
    restart: unless-stopped
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M

  # Redis 集群
  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes --replica-read-only yes
    volumes:
      - redis_data:/data
    networks:
      - wasm-network
    restart: unless-stopped
    deploy:
      replicas: 2

  # PostgreSQL 主从
  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=wasm_app
      - POSTGRES_USER=wasm_user
      - POSTGRES_PASSWORD=wasm_password
      - POSTGRES_REPLICATION_USER=replicator
      - POSTGRES_REPLICATION_PASSWORD=replicator_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./postgres/postgresql.conf:/etc/postgresql/postgresql.conf
    networks:
      - wasm-network
    restart: unless-stopped

  # 监控栈
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
    networks:
      - wasm-network
    restart: unless-stopped

  grafana:
    image: grafana/grafana:latest
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana_data:/var/lib/grafana
    networks:
      - wasm-network
    restart: unless-stopped

  # 日志收集
  fluentd:
    image: fluent/fluentd:latest
    volumes:
      - ./fluentd/fluent.conf:/fluentd/etc/fluent.conf
      - nginx_logs:/var/log/nginx
      - fluentd_data:/var/log/fluentd
    networks:
      - wasm-network
    restart: unless-stopped

volumes:
  redis_data:
  postgres_data:
  prometheus_data:
  grafana_data:
  nginx_logs:
  fluentd_data:

networks:
  wasm-network:
    driver: bridge
```

## ☸️ Kubernetes 部署

### 1. 基础配置

#### Namespace

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: wasm-app
  labels:
    name: wasm-app
    environment: production
```

#### ConfigMap

```yaml
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: wasm-app-config
  namespace: wasm-app
data:
  NODE_ENV: "production"
  RUST_LOG: "info"
  APP_PORT: "3000"
  REDIS_URL: "redis://redis-service:6379"
  DATABASE_URL: "postgresql://wasm_user:wasm_password@postgres-service:5432/wasm_app"
```

#### Secret

```yaml
# k8s/secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: wasm-app-secrets
  namespace: wasm-app
type: Opaque
data:
  DATABASE_PASSWORD: d2FzbV9wYXNzd29yZA==  # base64 encoded
  REDIS_PASSWORD: cmVkaXNfcGFzc3dvcmQ=      # base64 encoded
  JWT_SECRET: and0X3NlY3JldA==              # base64 encoded
```

### 2. 应用部署

#### Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasm-app
  namespace: wasm-app
  labels:
    app: wasm-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: wasm-app
  template:
    metadata:
      labels:
        app: wasm-app
    spec:
      containers:
      - name: wasm-app
        image: wasm-app:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_ENV
          valueFrom:
            configMapKeyRef:
              name: wasm-app-config
              key: NODE_ENV
        - name: DATABASE_PASSWORD
          valueFrom:
            secretKeyRef:
              name: wasm-app-secrets
              key: DATABASE_PASSWORD
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
        volumeMounts:
        - name: wasm-cache
          mountPath: /app/cache
      volumes:
      - name: wasm-cache
        emptyDir: {}
```

#### Service

```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: wasm-app-service
  namespace: wasm-app
spec:
  selector:
    app: wasm-app
  ports:
  - protocol: TCP
    port: 80
    targetPort: 3000
  type: ClusterIP
```

#### Ingress

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: wasm-app-ingress
  namespace: wasm-app
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - wasm-app.example.com
    secretName: wasm-app-tls
  rules:
  - host: wasm-app.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: wasm-app-service
            port:
              number: 80
```

### 3. 自动扩缩容

#### HorizontalPodAutoscaler

```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: wasm-app-hpa
  namespace: wasm-app
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: wasm-app
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
      - type: Percent
        value: 100
        periodSeconds: 60
```

#### VerticalPodAutoscaler

```yaml
# k8s/vpa.yaml
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: wasm-app-vpa
  namespace: wasm-app
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: wasm-app
  updatePolicy:
    updateMode: "Auto"
  resourcePolicy:
    containerPolicies:
    - containerName: wasm-app
      minAllowed:
        cpu: 100m
        memory: 128Mi
      maxAllowed:
        cpu: 1
        memory: 2Gi
      controlledResources: ["cpu", "memory"]
```

## 📊 监控和告警

### 1. Prometheus 配置

#### Prometheus 主配置

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "rules/*.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'wasm-app'
    static_configs:
      - targets: ['wasm-app:3000']
    metrics_path: /metrics
    scrape_interval: 10s

  - job_name: 'nginx'
    static_configs:
      - targets: ['nginx:9113']

  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
```

#### 告警规则

```yaml
# monitoring/rules/wasm-app.yml
groups:
- name: wasm-app
  rules:
  - alert: HighCPUUsage
    expr: rate(process_cpu_seconds_total[5m]) * 100 > 80
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High CPU usage detected"
      description: "CPU usage is above 80% for more than 5 minutes"

  - alert: HighMemoryUsage
    expr: (process_resident_memory_bytes / 1024 / 1024) > 1024
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High memory usage detected"
      description: "Memory usage is above 1GB for more than 5 minutes"

  - alert: WebAssemblyModuleLoadFailure
    expr: wasm_module_load_failures_total > 10
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "WebAssembly module load failures"
      description: "More than 10 WebAssembly module load failures in the last minute"

  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m]) > 0.05
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
      description: "Error rate is above 5% for more than 5 minutes"
```

### 2. Grafana 仪表板

#### 应用性能仪表板

```json
{
  "dashboard": {
    "title": "WebAssembly App Performance",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{status}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.50, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "50th percentile"
          }
        ]
      },
      {
        "title": "WebAssembly Module Performance",
        "type": "graph",
        "targets": [
          {
            "expr": "wasm_module_execution_time_seconds",
            "legendFormat": "{{module_name}}"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "process_resident_memory_bytes / 1024 / 1024",
            "legendFormat": "RSS Memory (MB)"
          }
        ]
      }
    ]
  }
}
```

### 3. 日志管理

#### Fluentd 配置

```ruby
# fluentd/fluent.conf
<source>
  @type tail
  path /var/log/nginx/access.log
  pos_file /var/log/fluentd/nginx.access.log.pos
  tag nginx.access
  format nginx
</source>

<source>
  @type tail
  path /var/log/nginx/error.log
  pos_file /var/log/fluentd/nginx.error.log.pos
  tag nginx.error
  format /^(?<time>\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2}) \[(?<log_level>\w+)\] (?<pid>\d+)#(?<tid>\d+): (?<message>.*)$/
</source>

<source>
  @type tail
  path /app/logs/wasm-app.log
  pos_file /var/log/fluentd/wasm-app.log.pos
  tag wasm-app
  format json
</source>

<filter nginx.access>
  @type record_transformer
  <record>
    service nginx
    environment ${ENV["ENVIRONMENT"] || "development"}
  </record>
</filter>

<filter wasm-app>
  @type record_transformer
  <record>
    service wasm-app
    environment ${ENV["ENVIRONMENT"] || "development"}
  </record>
</filter>

<match **>
  @type elasticsearch
  host elasticsearch
  port 9200
  index_name fluentd
  type_name _doc
  include_tag_key true
  tag_key @log_name
  flush_interval 1s
</match>
```

## 🚀 性能优化

### 1. WebAssembly 优化

#### 编译优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.release.package."*"]
opt-level = 3
lto = true

[profile.release.package.wasm-bindgen]
opt-level = 2
lto = false
```

#### 运行时优化

```rust
// src/performance.rs
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// WebAssembly 性能优化器
pub struct WasmPerformanceOptimizer {
    module_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    execution_cache: Arc<Mutex<HashMap<String, ExecutionResult>>>,
    stats: Arc<Mutex<PerformanceStats>>,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub result: Vec<u8>,
    pub execution_time: std::time::Duration,
    pub memory_usage: usize,
}

#[derive(Debug)]
pub struct PerformanceStats {
    pub total_executions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_execution_time: std::time::Duration,
    pub total_memory_usage: usize,
}

impl WasmPerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            module_cache: Arc::new(Mutex::new(HashMap::new())),
            execution_cache: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(PerformanceStats {
                total_executions: 0,
                cache_hits: 0,
                cache_misses: 0,
                average_execution_time: std::time::Duration::from_millis(0),
                total_memory_usage: 0,
            })),
        }
    }

    /// 执行 WebAssembly 模块（带缓存）
    pub fn execute_with_cache<F>(
        &self,
        module_id: &str,
        input: &[u8],
        executor: F,
    ) -> Result<ExecutionResult, WasmError>
    where
        F: FnOnce(&[u8]) -> Result<Vec<u8>, WasmError>,
    {
        // 检查执行缓存
        let cache_key = format!("{}:{}", module_id, base64::encode(input));
        {
            let cache = self.execution_cache.lock().unwrap();
            if let Some(cached_result) = cache.get(&cache_key) {
                let mut stats = self.stats.lock().unwrap();
                stats.cache_hits += 1;
                return Ok(cached_result.clone());
            }
        }

        // 执行模块
        let start_time = std::time::Instant::now();
        let result = executor(input)?;
        let execution_time = start_time.elapsed();

        // 计算内存使用
        let memory_usage = result.len();

        let execution_result = ExecutionResult {
            result: result.clone(),
            execution_time,
            memory_usage,
        };

        // 更新缓存
        {
            let mut cache = self.execution_cache.lock().unwrap();
            cache.insert(cache_key, execution_result.clone());
            
            // 限制缓存大小
            if cache.len() > 1000 {
                let keys_to_remove: Vec<String> = cache.keys().take(100).cloned().collect();
                for key in keys_to_remove {
                    cache.remove(&key);
                }
            }
        }

        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_executions += 1;
            stats.cache_misses += 1;
            stats.total_memory_usage += memory_usage;
            
            // 更新平均执行时间
            let total_time = stats.average_execution_time.as_nanos() as u128
                * (stats.total_executions - 1) as u128
                + execution_time.as_nanos() as u128;
            stats.average_execution_time = std::time::Duration::from_nanos(
                (total_time / stats.total_executions as u128) as u64
            );
        }

        Ok(execution_result)
    }

    /// 预加载模块
    pub fn preload_module(&self, module_id: &str, module_data: Vec<u8>) {
        let mut cache = self.module_cache.lock().unwrap();
        cache.insert(module_id.to_string(), module_data);
    }

    /// 获取性能统计
    pub fn get_stats(&self) -> PerformanceStats {
        self.stats.lock().unwrap().clone()
    }

    /// 清理缓存
    pub fn clear_cache(&self) {
        self.module_cache.lock().unwrap().clear();
        self.execution_cache.lock().unwrap().clear();
    }
}
```

### 2. 内存管理

#### 内存池优化

```rust
// src/memory_pool.rs
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::alloc::{GlobalAlloc, Layout, System};

/// 高性能内存池分配器
pub struct WasmMemoryPool {
    pools: Vec<Arc<Mutex<VecDeque<*mut u8>>>>>,
    pool_sizes: Vec<usize>,
    stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug)]
pub struct PoolStats {
    pub total_allocations: u64,
    pub pool_hits: u64,
    pub system_allocations: u64,
    pub total_bytes_allocated: usize,
    pub total_bytes_freed: usize,
}

impl WasmMemoryPool {
    pub fn new() -> Self {
        let pool_sizes = vec![64, 256, 1024, 4096, 16384, 65536, 262144];
        let pools: Vec<_> = pool_sizes.iter()
            .map(|&size| {
                let mut pool = VecDeque::new();
                // 预分配一些内存块
                for _ in 0..4 {
                    if let Ok(ptr) = Self::allocate_system(size) {
                        pool.push_back(ptr);
                    }
                }
                Arc::new(Mutex::new(pool))
            })
            .collect();

        Self {
            pools,
            pool_sizes,
            stats: Arc::new(Mutex::new(PoolStats {
                total_allocations: 0,
                pool_hits: 0,
                system_allocations: 0,
                total_bytes_allocated: 0,
                total_bytes_freed: 0,
            })),
        }
    }

    pub fn allocate(&self, size: usize) -> Option<*mut u8> {
        if let Some(pool_index) = self.find_best_pool(size) {
            let mut pool = self.pools[pool_index].lock().unwrap();
            
            if let Some(ptr) = pool.pop_front() {
                let mut stats = self.stats.lock().unwrap();
                stats.total_allocations += 1;
                stats.pool_hits += 1;
                stats.total_bytes_allocated += size;
                return Some(ptr);
            }
        }

        // 从系统分配
        if let Ok(ptr) = Self::allocate_system(size) {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocations += 1;
            stats.system_allocations += 1;
            stats.total_bytes_allocated += size;
            Some(ptr)
        } else {
            None
        }
    }

    pub fn deallocate(&self, ptr: *mut u8, size: usize) {
        if let Some(pool_index) = self.find_best_pool(size) {
            let mut pool = self.pools[pool_index].lock().unwrap();
            if pool.len() < 8 { // 限制池大小
                pool.push_back(ptr);
                let mut stats = self.stats.lock().unwrap();
                stats.total_bytes_freed += size;
                return;
            }
        }

        // 释放到系统
        Self::deallocate_system(ptr, size);
        let mut stats = self.stats.lock().unwrap();
        stats.total_bytes_freed += size;
    }

    fn find_best_pool(&self, size: usize) -> Option<usize> {
        self.pool_sizes.iter().position(|&pool_size| pool_size >= size)
    }

    fn allocate_system(size: usize) -> Result<*mut u8, std::alloc::AllocError> {
        let layout = Layout::from_size_align(size, 8)?;
        unsafe {
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                Err(std::alloc::AllocError)
            } else {
                Ok(ptr)
            }
        }
    }

    fn deallocate_system(ptr: *mut u8, size: usize) {
        if let Ok(layout) = Layout::from_size_align(size, 8) {
            unsafe {
                System.dealloc(ptr, layout);
            }
        }
    }

    pub fn get_stats(&self) -> PoolStats {
        self.stats.lock().unwrap().clone()
    }
}
```

## 🔧 运维工具

### 1. 健康检查

#### 应用健康检查

```rust
// src/health.rs
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: SystemTime,
    pub version: String,
    pub uptime: Duration,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
    pub duration: Duration,
}

pub struct HealthChecker {
    start_time: Instant,
    version: String,
    checks: Vec<Box<dyn HealthCheckTrait + Send + Sync>>,
}

pub trait HealthCheckTrait {
    fn name(&self) -> &str;
    fn check(&self) -> HealthCheckResult;
}

pub struct HealthCheckResult {
    pub status: String,
    pub message: Option<String>,
    pub duration: Duration,
}

impl HealthChecker {
    pub fn new(version: String) -> Self {
        Self {
            start_time: Instant::now(),
            version,
            checks: Vec::new(),
        }
    }

    pub fn add_check(&mut self, check: Box<dyn HealthCheckTrait + Send + Sync>) {
        self.checks.push(check);
    }

    pub fn check_health(&self) -> HealthStatus {
        let mut health_checks = Vec::new();

        for check in &self.checks {
            let start = Instant::now();
            let result = check.check();
            let duration = start.elapsed();

            health_checks.push(HealthCheck {
                name: check.name().to_string(),
                status: result.status,
                message: result.message,
                duration,
            });
        }

        // 确定整体状态
        let overall_status = if health_checks.iter().any(|c| c.status == "DOWN") {
            "DOWN"
        } else if health_checks.iter().any(|c| c.status == "DEGRADED") {
            "DEGRADED"
        } else {
            "UP"
        };

        HealthStatus {
            status: overall_status.to_string(),
            timestamp: SystemTime::now(),
            version: self.version.clone(),
            uptime: self.start_time.elapsed(),
            checks: health_checks,
        }
    }
}

// 数据库健康检查
pub struct DatabaseHealthCheck {
    connection_string: String,
}

impl DatabaseHealthCheck {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl HealthCheckTrait for DatabaseHealthCheck {
    fn name(&self) -> &str {
        "database"
    }

    fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        // 这里应该实现实际的数据库连接检查
        // 示例实现
        match self.connection_string.len() > 0 {
            true => HealthCheckResult {
                status: "UP".to_string(),
                message: Some("Database connection successful".to_string()),
                duration: start.elapsed(),
            },
            false => HealthCheckResult {
                status: "DOWN".to_string(),
                message: Some("Database connection failed".to_string()),
                duration: start.elapsed(),
            },
        }
    }
}

// Redis 健康检查
pub struct RedisHealthCheck {
    redis_url: String,
}

impl RedisHealthCheck {
    pub fn new(redis_url: String) -> Self {
        Self { redis_url }
    }
}

impl HealthCheckTrait for RedisHealthCheck {
    fn name(&self) -> &str {
        "redis"
    }

    fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        // 这里应该实现实际的 Redis 连接检查
        match self.redis_url.len() > 0 {
            true => HealthCheckResult {
                status: "UP".to_string(),
                message: Some("Redis connection successful".to_string()),
                duration: start.elapsed(),
            },
            false => HealthCheckResult {
                status: "DOWN".to_string(),
                message: Some("Redis connection failed".to_string()),
                duration: start.elapsed(),
            },
        }
    }
}
```

### 2. 部署脚本

#### 自动化部署脚本

```bash
#!/bin/bash
# scripts/deploy.sh

set -e

# 配置变量
ENVIRONMENT=${1:-staging}
VERSION=${2:-latest}
NAMESPACE="wasm-app"
REGISTRY="your-registry.com"
IMAGE_NAME="wasm-app"

echo "🚀 开始部署 WebAssembly 应用到 $ENVIRONMENT 环境"
echo "📦 版本: $VERSION"

# 函数：检查命令是否存在
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ 错误: $1 命令未找到"
        exit 1
    fi
}

# 函数：构建 Docker 镜像
build_image() {
    echo "🔨 构建 Docker 镜像..."
    docker build -t $REGISTRY/$IMAGE_NAME:$VERSION .
    docker tag $REGISTRY/$IMAGE_NAME:$VERSION $REGISTRY/$IMAGE_NAME:latest
    
    echo "📤 推送镜像到注册表..."
    docker push $REGISTRY/$IMAGE_NAME:$VERSION
    docker push $REGISTRY/$IMAGE_NAME:latest
}

# 函数：更新 Kubernetes 部署
update_k8s_deployment() {
    echo "☸️ 更新 Kubernetes 部署..."
    
    # 更新镜像标签
    kubectl set image deployment/wasm-app wasm-app=$REGISTRY/$IMAGE_NAME:$VERSION -n $NAMESPACE
    
    # 等待部署完成
    echo "⏳ 等待部署完成..."
    kubectl rollout status deployment/wasm-app -n $NAMESPACE --timeout=300s
    
    # 验证部署
    echo "✅ 验证部署..."
    kubectl get pods -n $NAMESPACE -l app=wasm-app
    
    # 检查服务状态
    echo "🔍 检查服务状态..."
    kubectl get svc -n $NAMESPACE
}

# 函数：运行健康检查
health_check() {
    echo "🏥 运行健康检查..."
    
    # 获取服务端点
    SERVICE_IP=$(kubectl get svc wasm-app-service -n $NAMESPACE -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    
    if [ -z "$SERVICE_IP" ]; then
        echo "⚠️ 警告: 无法获取服务 IP，跳过健康检查"
        return
    fi
    
    # 等待服务启动
    sleep 30
    
    # 健康检查
    for i in {1..10}; do
        if curl -f http://$SERVICE_IP/health; then
            echo "✅ 健康检查通过"
            return
        fi
        echo "⏳ 等待服务启动... ($i/10)"
        sleep 10
    done
    
    echo "❌ 健康检查失败"
    exit 1
}

# 函数：回滚部署
rollback() {
    echo "🔄 回滚部署..."
    kubectl rollout undo deployment/wasm-app -n $NAMESPACE
    kubectl rollout status deployment/wasm-app -n $NAMESPACE --timeout=300s
}

# 主执行流程
main() {
    # 检查必要命令
    check_command docker
    check_command kubectl
    check_command curl
    
    # 检查环境
    if ! kubectl get namespace $NAMESPACE &> /dev/null; then
        echo "❌ 错误: 命名空间 $NAMESPACE 不存在"
        exit 1
    fi
    
    # 设置错误处理
    trap rollback ERR
    
    # 执行部署步骤
    case $ENVIRONMENT in
        "staging"|"production")
            build_image
            update_k8s_deployment
            health_check
            ;;
        *)
            echo "❌ 错误: 不支持的环境 $ENVIRONMENT"
            echo "支持的环境: staging, production"
            exit 1
            ;;
    esac
    
    echo "🎉 部署完成！"
    echo "🌐 应用访问地址: http://$SERVICE_IP"
}

# 执行主函数
main "$@"
```

## 📋 最佳实践总结

### 1. 部署最佳实践

- **蓝绿部署**: 使用蓝绿部署减少停机时间
- **金丝雀发布**: 逐步发布新版本，降低风险
- **自动回滚**: 建立自动回滚机制
- **版本管理**: 使用语义化版本控制

### 2. 监控最佳实践

- **全链路监控**: 监控整个请求链路
- **关键指标**: 监控响应时间、错误率、吞吐量
- **告警机制**: 建立有效的告警机制
- **日志聚合**: 集中管理和分析日志

### 3. 性能最佳实践

- **缓存策略**: 合理使用多级缓存
- **资源优化**: 优化内存和CPU使用
- **负载均衡**: 使用合适的负载均衡策略
- **数据库优化**: 优化数据库查询和连接

### 4. 安全最佳实践

- **网络安全**: 使用防火墙和网络安全组
- **数据加密**: 传输和存储数据加密
- **访问控制**: 实施最小权限原则
- **安全扫描**: 定期进行安全漏洞扫描

---

**注意**: 本指南提供了完整的部署和运维解决方案，建议在实际项目中根据具体需求进行调整和优化。
