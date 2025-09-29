# WebAssembly 2.0 + Rust 1.90 API文档生成脚本
# 自动生成API文档和交互式文档

param(
    [switch]$Rust,
    [switch]$WebAssembly,
    [switch]$Frontend,
    [switch]$All,
    [switch]$Serve,
    [switch]$Open,
    [switch]$Help
)

# 颜色定义
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    White = "White"
    Cyan = "Cyan"
}

# 帮助信息
function Show-Help {
    Write-Host "WebAssembly 2.0 + Rust 1.90 API文档生成脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\generate-api-docs.ps1 [选项]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "选项:" -ForegroundColor $Colors.White
    Write-Host "  -Rust                生成Rust API文档" -ForegroundColor $Colors.White
    Write-Host "  -WebAssembly         生成WebAssembly API文档" -ForegroundColor $Colors.White
    Write-Host "  -Frontend            生成前端API文档" -ForegroundColor $Colors.White
    Write-Host "  -All                 生成所有API文档" -ForegroundColor $Colors.White
    Write-Host "  -Serve               启动文档服务器" -ForegroundColor $Colors.White
    Write-Host "  -Open                在浏览器中打开文档" -ForegroundColor $Colors.White
    Write-Host "  -Help                显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\generate-api-docs.ps1 -All" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\generate-api-docs.ps1 -Rust -Serve" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\generate-api-docs.ps1 -WebAssembly -Open" -ForegroundColor $Colors.White
}

# 日志函数
function Write-LogInfo {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $Colors.Blue
}

function Write-LogSuccess {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $Colors.Green
}

function Write-LogWarning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor $Colors.Yellow
}

function Write-LogError {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $Colors.Red
}

# 检查依赖
function Test-Dependencies {
    Write-LogInfo "检查依赖..."
    
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-LogError "Cargo未安装"
        exit 1
    }
    
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-LogError "Node.js未安装"
        exit 1
    }
    
    if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
        Write-LogError "npm未安装"
        exit 1
    }
    
    Write-LogSuccess "依赖检查通过"
}

# 生成Rust API文档
function New-RustApiDocs {
    Write-LogInfo "生成Rust API文档..."
    
    # 创建文档目录
    if (-not (Test-Path "docs/api/rust")) {
        New-Item -ItemType Directory -Path "docs/api/rust" -Force
    }
    
    # 生成Rust文档
    cargo doc --workspace --no-deps --document-private-items --open false
    
    # 复制文档到目标目录
    if (Test-Path "target/doc") {
        Copy-Item -Path "target/doc/*" -Destination "docs/api/rust/" -Recurse -Force
        Write-LogSuccess "Rust API文档生成完成: docs/api/rust/"
    } else {
        Write-LogWarning "Rust文档生成失败"
    }
}

# 生成WebAssembly API文档
function New-WebAssemblyApiDocs {
    Write-LogInfo "生成WebAssembly API文档..."
    
    # 创建文档目录
    if (-not (Test-Path "docs/api/webassembly")) {
        New-Item -ItemType Directory -Path "docs/api/webassembly" -Force
    }
    
    # 构建WebAssembly模块
    wasm-pack build --target web --out-dir pkg wasm
    
    # 生成TypeScript定义文档
    if (Test-Path "pkg/wasm.d.ts") {
        $tsContent = Get-Content "pkg/wasm.d.ts" -Raw
        
        $htmlContent = @"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebAssembly API文档</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 2px solid #007acc; padding-bottom: 10px; }
        pre { background: #f8f8f8; padding: 20px; border-radius: 5px; overflow-x: auto; border-left: 4px solid #007acc; }
        code { font-family: 'Consolas', 'Monaco', monospace; }
        .highlight { background: #fff3cd; padding: 2px 4px; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 WebAssembly 2.0 + Rust 1.90 API文档</h1>
        <p>本文档包含了WebAssembly模块的TypeScript接口定义。</p>
        
        <h2>📋 接口定义</h2>
        <pre><code>$tsContent</code></pre>
        
        <h2>🔧 使用方法</h2>
        <pre><code>
// 导入WebAssembly模块
import init, { fibonacci, run_matrix_multiply_benchmark } from './pkg/wasm.js';

// 初始化模块
await init();

// 使用API
const result = fibonacci(40);
const benchmark = run_matrix_multiply_benchmark(100, 10);
        </code></pre>
        
        <h2>📚 更多信息</h2>
        <p>查看完整的项目文档: <a href="../README.md">项目文档</a></p>
    </div>
</body>
</html>
"@
        
        $htmlContent | Out-File -FilePath "docs/api/webassembly/index.html" -Encoding UTF8
        Write-LogSuccess "WebAssembly API文档生成完成: docs/api/webassembly/"
    } else {
        Write-LogWarning "WebAssembly模块构建失败"
    }
}

# 生成前端API文档
function New-FrontendApiDocs {
    Write-LogInfo "生成前端API文档..."
    
    # 创建文档目录
    if (-not (Test-Path "docs/api/frontend")) {
        New-Item -ItemType Directory -Path "docs/api/frontend" -Force
    }
    
    # 生成前端API文档
    $frontendApiContent = @"
# 前端API文档

## 概述
本文档描述了WebAssembly 2.0 + Rust 1.90项目的前端API接口。

## 核心类

### WasmApp
主要的应用类，负责WebAssembly模块的初始化和交互。

#### 方法

##### init()
初始化WebAssembly模块。
```javascript
await app.init();
```

##### runBasicDemo()
运行基础示例。
```javascript
await app.runBasicDemo();
```

##### runAdvancedDemo()
运行高级示例。
```javascript
await app.runAdvancedDemo();
```

##### runPerformanceDemo()
运行性能测试。
```javascript
await app.runPerformanceDemo();
```

##### runImageDemo()
运行图像处理示例。
```javascript
await app.runImageDemo();
```

## 工具函数

### utils.formatNumber(num)
格式化数字显示。
```javascript
const formatted = utils.formatNumber(1234567); // "1,234,567"
```

### utils.formatTime(ms)
格式化时间显示。
```javascript
const formatted = utils.formatTime(1500); // "1.5s"
```

### utils.formatBytes(bytes)
格式化字节显示。
```javascript
const formatted = utils.formatBytes(1024); // "1 KB"
```

### utils.generateRandomData(size, type)
生成随机数据。
```javascript
const data = utils.generateRandomData(100, 'number');
```

## 性能监控

### PerformanceMonitor
性能监控类，用于测量和记录性能指标。

#### 方法

##### start(label)
开始性能测量。
```javascript
monitor.start('operation');
```

##### end(label)
结束性能测量并返回持续时间。
```javascript
const duration = monitor.end('operation');
```

##### getMetrics()
获取所有性能指标。
```javascript
const metrics = monitor.getMetrics();
```

## 事件处理

### 全局事件
- `DOMContentLoaded`: 页面加载完成后自动初始化应用
- `click`: 按钮点击事件处理

### 自定义事件
- `wasm:initialized`: WebAssembly模块初始化完成
- `wasm:error`: WebAssembly模块错误

## 配置选项

### 环境变量
- `NODE_ENV`: 环境模式 (development|production)
- `RUST_LOG`: Rust日志级别
- `WASM_CACHE_SIZE`: WebAssembly缓存大小

### 性能配置
- `PERFORMANCE_MONITORING`: 是否启用性能监控
- `MEMORY_MONITORING`: 是否启用内存监控
- `AUTO_UPDATE_STATS`: 是否自动更新统计信息

## 错误处理

### 错误类型
- `InitializationError`: 初始化错误
- `RuntimeError`: 运行时错误
- `NetworkError`: 网络错误

### 错误处理示例
```javascript
try {
    await app.runBasicDemo();
} catch (error) {
    console.error('Demo执行失败:', error);
    app.showError('Demo执行失败: ' + error.message);
}
```

## 最佳实践

1. **异步操作**: 所有WebAssembly操作都是异步的，使用await等待完成
2. **错误处理**: 始终使用try-catch处理可能的错误
3. **性能监控**: 使用PerformanceMonitor测量关键操作的性能
4. **内存管理**: 定期检查内存使用情况，避免内存泄漏
5. **用户体验**: 提供加载状态和错误反馈

## 示例代码

### 完整的使用示例
```javascript
import { app, utils, PerformanceMonitor } from './src/index.js';

// 创建性能监控器
const monitor = new PerformanceMonitor();

// 初始化应用
document.addEventListener('DOMContentLoaded', async () => {
    try {
        await app.init();
        console.log('应用初始化成功');
        
        // 运行基础示例
        monitor.start('basicDemo');
        await app.runBasicDemo();
        const duration = monitor.end('basicDemo');
        console.log(\`基础示例执行时间: \${duration}ms\`);
        
    } catch (error) {
        console.error('应用初始化失败:', error);
    }
});
```
"@
    
    $frontendApiContent | Out-File -FilePath "docs/api/frontend/README.md" -Encoding UTF8
    Write-LogSuccess "前端API文档生成完成: docs/api/frontend/"
}

# 生成交互式文档
function New-InteractiveDocs {
    Write-LogInfo "生成交互式文档..."
    
    # 创建交互式文档目录
    if (-not (Test-Path "docs/interactive")) {
        New-Item -ItemType Directory -Path "docs/interactive" -Force
    }
    
    # 生成交互式文档HTML
    $interactiveContent = @"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebAssembly 2.0 + Rust 1.90 交互式文档</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', sans-serif; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 40px; color: #333; }
        .tabs { display: flex; margin-bottom: 20px; border-bottom: 2px solid #ddd; }
        .tab { padding: 10px 20px; cursor: pointer; border: none; background: none; font-size: 16px; }
        .tab.active { background: #007acc; color: white; border-radius: 5px 5px 0 0; }
        .tab-content { display: none; background: white; padding: 30px; border-radius: 0 10px 10px 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .tab-content.active { display: block; }
        .code-block { background: #f8f8f8; padding: 20px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #007acc; }
        .run-button { background: #007acc; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; margin: 10px 0; }
        .run-button:hover { background: #005a9e; }
        .result { background: #e8f5e8; padding: 15px; border-radius: 5px; margin: 10px 0; border-left: 4px solid #4caf50; }
        .error { background: #ffe8e8; padding: 15px; border-radius: 5px; margin: 10px 0; border-left: 4px solid #f44336; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 WebAssembly 2.0 + Rust 1.90 交互式文档</h1>
            <p>在线体验和测试WebAssembly功能</p>
        </div>
        
        <div class="tabs">
            <button class="tab active" onclick="showTab('basic')">基础示例</button>
            <button class="tab" onclick="showTab('advanced')">高级示例</button>
            <button class="tab" onclick="showTab('performance')">性能测试</button>
            <button class="tab" onclick="showTab('api')">API参考</button>
        </div>
        
        <div id="basic" class="tab-content active">
            <h2>🦀 基础示例</h2>
            <p>演示基本的WebAssembly功能，包括斐波那契数列计算。</p>
            
            <div class="code-block">
                <h3>斐波那契数列计算</h3>
                <pre><code>// 计算斐波那契数列第n项
const result = fibonacci(40);
console.log(\`斐波那契数列第40项: \${result}\`);</code></pre>
                <button class="run-button" onclick="runBasicExample()">运行示例</button>
                <div id="basic-result"></div>
            </div>
        </div>
        
        <div id="advanced" class="tab-content">
            <h2>🌐 高级示例</h2>
            <p>演示复杂的WebAssembly功能，包括矩阵运算和网络请求。</p>
            
            <div class="code-block">
                <h3>矩阵乘法基准测试</h3>
                <pre><code>// 运行矩阵乘法基准测试
const result = run_matrix_multiply_benchmark(100, 10);
console.log('矩阵乘法基准测试结果:', result);</code></pre>
                <button class="run-button" onclick="runAdvancedExample()">运行示例</button>
                <div id="advanced-result"></div>
            </div>
        </div>
        
        <div id="performance" class="tab-content">
            <h2>⚡ 性能测试</h2>
            <p>测试WebAssembly性能优化和SIMD操作。</p>
            
            <div class="code-block">
                <h3>性能基准测试</h3>
                <pre><code>// 运行性能基准测试
const result = run_simd_benchmark(1000, 100);
console.log('SIMD基准测试结果:', result);</code></pre>
                <button class="run-button" onclick="runPerformanceExample()">运行测试</button>
                <div id="performance-result"></div>
            </div>
        </div>
        
        <div id="api" class="tab-content">
            <h2>📚 API参考</h2>
            <p>完整的API接口文档和示例。</p>
            
            <div class="code-block">
                <h3>WebAssembly模块API</h3>
                <pre><code>// 导入WebAssembly模块
import init, { 
    fibonacci, 
    fibonacci_fast,
    run_matrix_multiply_benchmark,
    run_sorting_benchmark,
    run_simd_benchmark,
    apply_image_filter
} from './pkg/wasm.js';

// 初始化模块
await init();</code></pre>
            </div>
        </div>
    </div>
    
    <script type="module">
        import init, { 
            fibonacci, 
            run_matrix_multiply_benchmark,
            run_simd_benchmark
        } from '../pkg/wasm.js';
        
        let wasmInitialized = false;
        
        // 初始化WebAssembly模块
        async function initializeWasm() {
            if (!wasmInitialized) {
                try {
                    await init();
                    wasmInitialized = true;
                    console.log('WebAssembly模块初始化成功');
                } catch (error) {
                    console.error('WebAssembly模块初始化失败:', error);
                }
            }
        }
        
        // 显示标签页
        window.showTab = function(tabName) {
            // 隐藏所有标签内容
            document.querySelectorAll('.tab-content').forEach(content => {
                content.classList.remove('active');
            });
            
            // 移除所有标签的active类
            document.querySelectorAll('.tab').forEach(tab => {
                tab.classList.remove('active');
            });
            
            // 显示选中的标签内容
            document.getElementById(tabName).classList.add('active');
            
            // 添加选中标签的active类
            event.target.classList.add('active');
        };
        
        // 运行基础示例
        window.runBasicExample = async function() {
            await initializeWasm();
            
            const resultDiv = document.getElementById('basic-result');
            resultDiv.innerHTML = '<div class="result">计算中...</div>';
            
            try {
                const startTime = performance.now();
                const result = fibonacci(40);
                const endTime = performance.now();
                const duration = Math.round(endTime - startTime);
                
                resultDiv.innerHTML = \`
                    <div class="result">
                        <strong>斐波那契数列第40项:</strong> \${result}<br>
                        <strong>执行时间:</strong> \${duration}ms
                    </div>
                \`;
            } catch (error) {
                resultDiv.innerHTML = \`
                    <div class="error">
                        <strong>错误:</strong> \${error.message}
                    </div>
                \`;
            }
        };
        
        // 运行高级示例
        window.runAdvancedExample = async function() {
            await initializeWasm();
            
            const resultDiv = document.getElementById('advanced-result');
            resultDiv.innerHTML = '<div class="result">计算中...</div>';
            
            try {
                const startTime = performance.now();
                const result = run_matrix_multiply_benchmark(100, 10);
                const endTime = performance.now();
                const duration = Math.round(endTime - startTime);
                
                resultDiv.innerHTML = \`
                    <div class="result">
                        <strong>矩阵乘法基准测试完成</strong><br>
                        <strong>执行时间:</strong> \${duration}ms<br>
                        <strong>结果:</strong> \${JSON.stringify(result, null, 2)}
                    </div>
                \`;
            } catch (error) {
                resultDiv.innerHTML = \`
                    <div class="error">
                        <strong>错误:</strong> \${error.message}
                    </div>
                \`;
            }
        };
        
        // 运行性能示例
        window.runPerformanceExample = async function() {
            await initializeWasm();
            
            const resultDiv = document.getElementById('performance-result');
            resultDiv.innerHTML = '<div class="result">测试中...</div>';
            
            try {
                const startTime = performance.now();
                const result = run_simd_benchmark(1000, 100);
                const endTime = performance.now();
                const duration = Math.round(endTime - startTime);
                
                resultDiv.innerHTML = \`
                    <div class="result">
                        <strong>SIMD基准测试完成</strong><br>
                        <strong>执行时间:</strong> \${duration}ms<br>
                        <strong>结果:</strong> \${JSON.stringify(result, null, 2)}
                    </div>
                \`;
            } catch (error) {
                resultDiv.innerHTML = \`
                    <div class="error">
                        <strong>错误:</strong> \${error.message}
                    </div>
                \`;
            }
        };
        
        // 页面加载完成后初始化
        document.addEventListener('DOMContentLoaded', initializeWasm);
    </script>
</body>
</html>
"@
    
    $interactiveContent | Out-File -FilePath "docs/interactive/index.html" -Encoding UTF8
    Write-LogSuccess "交互式文档生成完成: docs/interactive/"
}

# 启动文档服务器
function Start-DocServer {
    Write-LogInfo "启动文档服务器..."
    
    # 检查是否安装了http-server
    if (-not (Get-Command http-server -ErrorAction SilentlyContinue)) {
        Write-LogInfo "安装http-server..."
        npm install -g http-server
    }
    
    # 启动服务器
    $port = 8080
    Write-LogInfo "文档服务器启动在端口 $port"
    Write-LogInfo "访问地址: http://localhost:$port"
    
    if ($Open) {
        Start-Process "http://localhost:$port"
    }
    
    http-server docs -p $port -o
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "📚 WebAssembly 2.0 + Rust 1.90 API文档生成" -ForegroundColor $Colors.Blue
    Write-Host "===========================================" -ForegroundColor $Colors.Blue
    
    Test-Dependencies
    
    # 创建文档目录
    if (-not (Test-Path "docs")) {
        New-Item -ItemType Directory -Path "docs" -Force
    }
    
    if (-not (Test-Path "docs/api")) {
        New-Item -ItemType Directory -Path "docs/api" -Force
    }
    
    if ($All -or $Rust) {
        New-RustApiDocs
    }
    
    if ($All -or $WebAssembly) {
        New-WebAssemblyApiDocs
    }
    
    if ($All -or $Frontend) {
        New-FrontendApiDocs
    }
    
    if ($All) {
        New-InteractiveDocs
    }
    
    if ($Serve) {
        Start-DocServer
    } else {
        Write-LogSuccess "API文档生成完成！"
        Write-Host ""
        Write-Host "📋 生成的文档:" -ForegroundColor $Colors.Cyan
        if ($All -or $Rust) {
            Write-Host "  Rust API文档: docs/api/rust/" -ForegroundColor $Colors.White
        }
        if ($All -or $WebAssembly) {
            Write-Host "  WebAssembly API文档: docs/api/webassembly/" -ForegroundColor $Colors.White
        }
        if ($All -or $Frontend) {
            Write-Host "  前端API文档: docs/api/frontend/" -ForegroundColor $Colors.White
        }
        if ($All) {
            Write-Host "  交互式文档: docs/interactive/" -ForegroundColor $Colors.White
        }
        Write-Host ""
        Write-Host "🚀 启动文档服务器: .\scripts\generate-api-docs.ps1 -Serve" -ForegroundColor $Colors.Cyan
    }
}

# 执行主函数
Main
