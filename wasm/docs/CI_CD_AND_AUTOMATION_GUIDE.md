# WebAssembly 2.0 + Rust 1.90 CI/CD 和自动化指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 项目的完整 CI/CD 和自动化解决方案，包括持续集成、持续部署、自动化测试、代码质量检查等全流程的自动化配置。

## 🎯 CI/CD 架构

### 1. 整体架构

#### 流水线阶段

```text
代码提交 → 代码检查 → 构建 → 测试 → 部署 → 监控
    ↓         ↓        ↓      ↓      ↓      ↓
   Git      Lint    Build   Test  Deploy Monitor
```

#### 工具链选择

- **版本控制**: Git + GitHub
- **CI/CD**: GitHub Actions
- **构建工具**: Cargo + wasm-pack
- **测试框架**: Cargo Test + wasm-bindgen-test
- **代码质量**: Clippy + Rustfmt + Cargo Audit
- **部署平台**: GitHub Pages + Docker Hub
- **监控工具**: Prometheus + Grafana

### 2. 环境配置

#### 开发环境

- **分支**: develop
- **触发**: push, pull_request
- **目标**: 快速反馈

#### 测试环境

- **分支**: release/*
- **触发**: 自动部署
- **目标**: 集成测试

#### 生产环境

- **分支**: main
- **触发**: 手动审批
- **目标**: 稳定发布

## 🔧 GitHub Actions 配置

### 1. 基础 CI 流水线

#### 主配置文件

```yaml
# .github/workflows/ci.yml
name: Continuous Integration

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  # 代码质量检查
  quality:
    name: Code Quality
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
        target: wasm32-unknown-unknown
        override: true
        
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Install wasm-pack
      run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy --all-targets -- -D warnings
      
    - name: Security audit
      run: cargo audit
      
    - name: Check for outdated dependencies
      run: cargo outdated --exit-code 1

  # 单元测试
  test:
    name: Unit Tests
    runs-on: ubuntu-latest
    needs: quality
    
    strategy:
      matrix:
        rust: [stable, beta, nightly]
        
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust ${{ matrix.rust }}
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
        target: wasm32-unknown-unknown
        override: true
        
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run tests
      run: cargo test --verbose
      
    - name: Run WebAssembly tests
      run: wasm-pack test --headless --firefox

  # 集成测试
  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: test
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Build WebAssembly
      run: wasm-pack build --target web --out-dir pkg
      
    - name: Build frontend
      run: npm run build
      
    - name: Run integration tests
      run: npm run test:integration

  # 性能测试
  performance:
    name: Performance Tests
    runs-on: ubuntu-latest
    needs: integration
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run benchmarks
      run: cargo bench
      
    - name: Upload benchmark results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: target/criterion/

  # 构建
  build:
    name: Build
    runs-on: ubuntu-latest
    needs: performance
    
    strategy:
      matrix:
        target: [web, nodejs, bundler]
        
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Install wasm-pack
      run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
    - name: Build WebAssembly for ${{ matrix.target }}
      run: wasm-pack build --target ${{ matrix.target }} --out-dir pkg-${{ matrix.target }}
      
    - name: Upload build artifacts
      uses: actions/upload-artifact@v3
      with:
        name: wasm-pkg-${{ matrix.target }}
        path: pkg-${{ matrix.target }}/
```

### 2. 高级 CI 配置

#### 多平台构建

```yaml
# .github/workflows/build-matrix.yml
name: Multi-Platform Build

on:
  push:
    branches: [ main ]
  release:
    types: [ published ]

jobs:
  build:
    name: Build for ${{ matrix.os }} - ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        target: [web, nodejs, bundler]
        include:
          - os: ubuntu-latest
            target: web
            cache-key: ubuntu-web
          - os: windows-latest
            target: nodejs
            cache-key: windows-nodejs
          - os: macos-latest
            target: bundler
            cache-key: macos-bundler
            
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ matrix.cache-key }}-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Install wasm-pack
      run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
    - name: Build WebAssembly
      run: wasm-pack build --target ${{ matrix.target }} --out-dir pkg
      
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: wasm-${{ matrix.os }}-${{ matrix.target }}
        path: pkg/
```

#### 安全扫描

```yaml
# .github/workflows/security.yml
name: Security Scan

on:
  schedule:
    - cron: '0 2 * * 1'  # 每周一凌晨2点
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  security:
    name: Security Scan
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Run cargo audit
      run: cargo audit
      
    - name: Run cargo deny
      run: |
        cargo install cargo-deny
        cargo deny check
        
    - name: Run trivy vulnerability scanner
      uses: aquasecurity/trivy-action@master
      with:
        scan-type: 'fs'
        scan-ref: '.'
        format: 'sarif'
        output: 'trivy-results.sarif'
        
    - name: Upload Trivy scan results
      uses: github/codeql-action/upload-sarif@v2
      with:
        sarif_file: 'trivy-results.sarif'
```

### 3. CD 部署配置

#### 自动部署到 GitHub Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pages: write
      id-token: write
      
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Build WebAssembly
      run: wasm-pack build --target web --out-dir pkg
      
    - name: Build frontend
      run: npm run build
      
    - name: Setup Pages
      uses: actions/configure-pages@v3
      
    - name: Upload artifact
      uses: actions/upload-pages-artifact@v2
      with:
        path: './dist'
        
    - name: Deploy to GitHub Pages
      id: deployment
      uses: actions/deploy-pages@v2
```

#### Docker 镜像构建和推送

```yaml
# .github/workflows/docker.yml
name: Build and Push Docker Image

on:
  push:
    branches: [ main ]
    tags: [ 'v*' ]
  pull_request:
    branches: [ main ]

jobs:
  docker:
    name: Docker Build
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2
      
    - name: Login to Docker Hub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_USERNAME }}
        password: ${{ secrets.DOCKER_PASSWORD }}
        
    - name: Extract metadata
      id: meta
      uses: docker/metadata-action@v4
      with:
        images: ${{ secrets.DOCKER_USERNAME }}/webassembly-rust
        tags: |
          type=ref,event=branch
          type=ref,event=pr
          type=semver,pattern={{version}}
          type=semver,pattern={{major}}.{{minor}}
          type=raw,value=latest,enable={{is_default_branch}}
          
    - name: Build and push Docker image
      uses: docker/build-push-action@v4
      with:
        context: .
        platforms: linux/amd64,linux/arm64
        push: true
        tags: ${{ steps.meta.outputs.tags }}
        labels: ${{ steps.meta.outputs.labels }}
        cache-from: type=gha
        cache-to: type=gha,mode=max
```

## 🧪 自动化测试

### 1. 测试策略

#### 测试金字塔

```text
    /\
   /  \     E2E Tests (5%)
  /____\    
 /      \   Integration Tests (15%)
/________\  
/          \ Unit Tests (80%)
/____________\
```

#### 测试类型配置

```yaml
# .github/workflows/test-matrix.yml
name: Test Matrix

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    name: Test ${{ matrix.test-type }} on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        test-type: [unit, integration, e2e]
        rust-version: [stable, beta]
        exclude:
          - os: windows-latest
            test-type: e2e
          - os: macos-latest
            test-type: e2e
            
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust ${{ matrix.rust-version }}
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust-version }}
        target: wasm32-unknown-unknown
        override: true
        
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Run unit tests
      if: matrix.test-type == 'unit'
      run: cargo test --lib
      
    - name: Run integration tests
      if: matrix.test-type == 'integration'
      run: cargo test --test '*'
      
    - name: Run E2E tests
      if: matrix.test-type == 'e2e'
      run: |
        wasm-pack build --target web --out-dir pkg
        npm run test:e2e
```

### 2. 性能测试自动化

#### 基准测试配置

```yaml
# .github/workflows/benchmark.yml
name: Benchmark

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 0'  # 每周日运行

jobs:
  benchmark:
    name: Benchmark
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Run benchmarks
      run: cargo bench
      
    - name: Install cargo-criterion
      run: cargo install cargo-criterion
      
    - name: Generate benchmark report
      run: cargo criterion --message-format=json > benchmark-results.json
      
    - name: Upload benchmark results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: |
          target/criterion/
          benchmark-results.json
          
    - name: Comment benchmark results
      if: github.event_name == 'pull_request'
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const results = JSON.parse(fs.readFileSync('benchmark-results.json', 'utf8'));
          
          const comment = `## 📊 Benchmark Results
          
          | Test | Time | Change |
          |------|------|--------|
          ${results.map(r => `| ${r.name} | ${r.time} | ${r.change} |`).join('\n')}
          `;
          
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: comment
          });
```

## 📊 代码质量检查

### 1. 代码质量工具

#### 综合质量检查

```yaml
# .github/workflows/quality.yml
name: Code Quality

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  quality:
    name: Code Quality Check
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
        override: true
        
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
      
    - name: Run cargo audit
      run: cargo audit
      
    - name: Run cargo deny
      run: |
        cargo install cargo-deny
        cargo deny check
        
    - name: Check documentation
      run: cargo doc --no-deps --document-private-items
      
    - name: Run cargo-machete
      run: |
        cargo install cargo-machete
        cargo machete
        
    - name: Generate code coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml --output-dir coverage/
        
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./coverage/cobertura.xml
        flags: unittests
        name: codecov-umbrella
```

### 2. 依赖管理

#### 依赖更新自动化

```yaml
# .github/workflows/dependabot.yml
name: Dependabot

on:
  schedule:
    - cron: '0 2 * * 1'  # 每周一凌晨2点
  workflow_dispatch:

jobs:
  update-dependencies:
    name: Update Dependencies
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Check for outdated dependencies
      run: |
        cargo install cargo-outdated
        cargo outdated --exit-code 1
        
    - name: Update dependencies
      run: |
        cargo update
        cargo audit fix
        
    - name: Create Pull Request
      uses: peter-evans/create-pull-request@v5
      with:
        token: ${{ secrets.GITHUB_TOKEN }}
        commit-message: 'chore: update dependencies'
        title: 'chore: update dependencies'
        body: |
          This PR updates dependencies to their latest versions.
          
          - Updated Rust dependencies
          - Fixed security vulnerabilities
          - Updated development dependencies
        branch: update-dependencies
        delete-branch: true
```

## 🚀 部署自动化

### 1. 多环境部署

#### 环境配置

```yaml
# .github/workflows/deploy-multi-env.yml
name: Multi-Environment Deployment

on:
  push:
    branches: [ main, develop ]
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to deploy to'
        required: true
        default: 'staging'
        type: choice
        options:
        - staging
        - production

jobs:
  deploy:
    name: Deploy to ${{ github.event.inputs.environment || (github.ref == 'refs/heads/main' && 'production') || 'staging' }}
    runs-on: ubuntu-latest
    environment: ${{ github.event.inputs.environment || (github.ref == 'refs/heads/main' && 'production') || 'staging' }}
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Build WebAssembly
      run: wasm-pack build --target web --out-dir pkg
      
    - name: Build frontend
      run: npm run build
      
    - name: Deploy to staging
      if: github.event.inputs.environment == 'staging' || github.ref == 'refs/heads/develop'
      run: |
        echo "Deploying to staging environment..."
        # 部署到测试环境的脚本
        
    - name: Deploy to production
      if: github.event.inputs.environment == 'production' || github.ref == 'refs/heads/main'
      run: |
        echo "Deploying to production environment..."
        # 部署到生产环境的脚本
        
    - name: Notify deployment
      uses: 8398a7/action-slack@v3
      with:
        status: ${{ job.status }}
        channel: '#deployments'
        webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

### 2. 回滚机制

#### 自动回滚配置

```yaml
# .github/workflows/rollback.yml
name: Rollback

on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to rollback'
        required: true
        type: choice
        options:
        - staging
        - production
      version:
        description: 'Version to rollback to'
        required: true
        type: string

jobs:
  rollback:
    name: Rollback ${{ github.event.inputs.environment }} to ${{ github.event.inputs.version }}
    runs-on: ubuntu-latest
    environment: ${{ github.event.inputs.environment }}
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        ref: ${{ github.event.inputs.version }}
        
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
        
    - name: Build and deploy
      run: |
        echo "Rolling back to version ${{ github.event.inputs.version }}"
        # 回滚脚本
        
    - name: Notify rollback
      uses: 8398a7/action-slack@v3
      with:
        status: ${{ job.status }}
        channel: '#deployments'
        webhook_url: ${{ secrets.SLACK_WEBHOOK }}
        fields: repo,message,commit,author,action,eventName,ref,workflow
```

## 📈 监控和告警

### 1. 监控配置

#### 健康检查

```yaml
# .github/workflows/health-check.yml
name: Health Check

on:
  schedule:
    - cron: '*/5 * * * *'  # 每5分钟检查一次
  workflow_dispatch:

jobs:
  health-check:
    name: Health Check
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Health check staging
      run: |
        curl -f https://staging.webassembly-rust.dev/health || exit 1
        
    - name: Health check production
      run: |
        curl -f https://webassembly-rust.dev/health || exit 1
        
    - name: Notify on failure
      if: failure()
      uses: 8398a7/action-slack@v3
      with:
        status: failure
        channel: '#alerts'
        webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

### 2. 性能监控

#### 性能指标收集

```yaml
# .github/workflows/performance-monitor.yml
name: Performance Monitor

on:
  schedule:
    - cron: '0 */6 * * *'  # 每6小时运行一次
  workflow_dispatch:

jobs:
  performance:
    name: Performance Monitor
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        
    - name: Install Lighthouse CI
      run: npm install -g @lhci/cli@0.12.x
      
    - name: Run Lighthouse CI
      run: |
        lhci autorun --upload.target=temporary-public-storage --collect.url=https://webassembly-rust.dev
        
    - name: Upload performance results
      uses: actions/upload-artifact@v3
      with:
        name: lighthouse-results
        path: .lighthouseci/
```

## 📋 最佳实践总结

### 1. CI/CD 最佳实践

- **快速反馈**: 保持 CI 流水线快速执行
- **并行执行**: 使用矩阵策略并行执行任务
- **缓存优化**: 合理使用缓存减少构建时间
- **环境隔离**: 开发、测试、生产环境完全隔离

### 2. 自动化最佳实践

- **测试覆盖**: 保持高测试覆盖率
- **代码质量**: 使用多种工具检查代码质量
- **安全扫描**: 定期进行安全漏洞扫描
- **依赖管理**: 及时更新依赖并修复漏洞

### 3. 部署最佳实践

- **蓝绿部署**: 使用蓝绿部署减少停机时间
- **回滚机制**: 建立快速回滚机制
- **监控告警**: 建立完善的监控告警系统
- **文档更新**: 保持部署文档的及时更新

### 4. 监控最佳实践

- **健康检查**: 定期进行健康检查
- **性能监控**: 持续监控性能指标
- **日志收集**: 收集和分析应用日志
- **告警机制**: 建立有效的告警机制

---

**注意**: 本指南提供了完整的 CI/CD 和自动化解决方案，建议在实际项目中根据具体需求进行调整和优化。
