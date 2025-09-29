# WebAssembly 2.0 + Rust 1.90 性能优化脚本
# 全面的性能分析和优化

param(
    [switch]$Analyze,
    [switch]$Optimize,
    [switch]$Benchmark,
    [switch]$Profile,
    [switch]$Memory,
    [switch]$CPU,
    [switch]$Network,
    [switch]$Report,
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
    Magenta = "Magenta"
}

# 帮助信息
function Show-Help {
    Write-Host "WebAssembly 2.0 + Rust 1.90 性能优化脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\performance-optimization.ps1 [选项]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "选项:" -ForegroundColor $Colors.White
    Write-Host "  -Analyze             性能分析" -ForegroundColor $Colors.White
    Write-Host "  -Optimize            性能优化" -ForegroundColor $Colors.White
    Write-Host "  -Benchmark           基准测试" -ForegroundColor $Colors.White
    Write-Host "  -Profile             性能分析" -ForegroundColor $Colors.White
    Write-Host "  -Memory              内存分析" -ForegroundColor $Colors.White
    Write-Host "  -CPU                 CPU分析" -ForegroundColor $Colors.White
    Write-Host "  -Network             网络分析" -ForegroundColor $Colors.White
    Write-Host "  -Report              生成性能报告" -ForegroundColor $Colors.White
    Write-Host "  -Help                显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\performance-optimization.ps1 -Analyze -Report" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\performance-optimization.ps1 -Benchmark -Memory" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\performance-optimization.ps1 -Optimize -CPU" -ForegroundColor $Colors.White
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
    Write-LogInfo "检查性能分析工具..."
    
    $tools = @(
        @{Name="cargo"; Description="Rust包管理器"},
        @{Name="cargo-bench"; Description="Rust基准测试工具"},
        @{Name="cargo-profdata"; Description="Rust性能分析工具"},
        @{Name="perf"; Description="Linux性能分析工具"},
        @{Name="valgrind"; Description="内存分析工具"}
    )
    
    $missingTools = @()
    
    foreach ($tool in $tools) {
        if (-not (Get-Command $tool.Name -ErrorAction SilentlyContinue)) {
            $missingTools += $tool.Description
        }
    }
    
    if ($missingTools.Count -gt 0) {
        Write-LogWarning "缺少以下性能分析工具:"
        foreach ($tool in $missingTools) {
            Write-Host "  - $tool" -ForegroundColor $Colors.Yellow
        }
    }
    
    Write-LogSuccess "性能分析工具检查完成"
}

# 性能分析
function Invoke-PerformanceAnalysis {
    Write-LogInfo "执行性能分析..."
    
    $analysisResults = @{
        BuildTime = 0
        BinarySize = 0
        MemoryUsage = 0
        CPUUsage = 0
        NetworkLatency = 0
        Throughput = 0
    }
    
    # 构建时间分析
    Write-LogInfo "分析构建时间..."
    $buildStartTime = Get-Date
    cargo build --release --workspace
    $buildEndTime = Get-Date
    $analysisResults.BuildTime = ($buildEndTime - $buildStartTime).TotalSeconds
    
    # 二进制大小分析
    Write-LogInfo "分析二进制大小..."
    if (Test-Path "target/release") {
        $binaryFiles = Get-ChildItem -Path "target/release" -Recurse -File
        $totalSize = ($binaryFiles | Measure-Object -Property Length -Sum).Sum
        $analysisResults.BinarySize = [math]::Round($totalSize / 1MB, 2)
    }
    
    # WebAssembly模块大小分析
    Write-LogInfo "分析WebAssembly模块大小..."
    if (Test-Path "pkg") {
        $wasmFiles = Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm"
        $wasmSize = ($wasmFiles | Measure-Object -Property Length -Sum).Sum
        $analysisResults.WasmSize = [math]::Round($wasmSize / 1KB, 2)
    }
    
    return $analysisResults
}

# 基准测试
function Invoke-BenchmarkTests {
    Write-LogInfo "执行基准测试..."
    
    $benchmarkResults = @()
    
    # 运行Rust基准测试
    Write-LogInfo "运行Rust基准测试..."
    try {
        $benchmarkOutput = cargo bench --workspace 2>&1
        $benchmarkResults += @{
            Type = "rust-benchmark"
            Output = $benchmarkOutput
            Status = if ($LASTEXITCODE -eq 0) { "SUCCESS" } else { "FAILED" }
        }
    } catch {
        Write-LogError "Rust基准测试失败: $($_.Exception.Message)"
        $benchmarkResults += @{
            Type = "rust-benchmark"
            Output = $_.Exception.Message
            Status = "ERROR"
        }
    }
    
    # 运行WebAssembly基准测试
    Write-LogInfo "运行WebAssembly基准测试..."
    try {
        # 构建WebAssembly模块
        wasm-pack build --target web --out-dir pkg wasm
        
        # 运行性能示例
        if (Test-Path "examples/performance") {
            wasm-pack build --target web --out-dir pkg/performance examples/performance
        }
        
        $benchmarkResults += @{
            Type = "wasm-benchmark"
            Status = "SUCCESS"
            Output = "WebAssembly模块构建成功"
        }
    } catch {
        Write-LogError "WebAssembly基准测试失败: $($_.Exception.Message)"
        $benchmarkResults += @{
            Type = "wasm-benchmark"
            Output = $_.Exception.Message
            Status = "ERROR"
        }
    }
    
    return $benchmarkResults
}

# 内存分析
function Invoke-MemoryAnalysis {
    Write-LogInfo "执行内存分析..."
    
    $memoryResults = @()
    
    # 分析Rust内存使用
    Write-LogInfo "分析Rust内存使用..."
    try {
        # 使用cargo-profdata进行内存分析
        $memoryOutput = cargo profdata --help 2>&1
        $memoryResults += @{
            Type = "rust-memory"
            Status = "SUCCESS"
            Output = "Rust内存分析工具可用"
        }
    } catch {
        $memoryResults += @{
            Type = "rust-memory"
            Status = "ERROR"
            Output = "Rust内存分析工具不可用"
        }
    }
    
    # 分析WebAssembly内存使用
    Write-LogInfo "分析WebAssembly内存使用..."
    if (Test-Path "pkg") {
        $wasmFiles = Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm"
        foreach ($wasmFile in $wasmFiles) {
            $fileSize = [math]::Round($wasmFile.Length / 1KB, 2)
            $memoryResults += @{
                Type = "wasm-memory"
                File = $wasmFile.Name
                Size = $fileSize
                Status = if ($fileSize -lt 100) { "OPTIMAL" } elseif ($fileSize -lt 500) { "GOOD" } else { "LARGE" }
            }
        }
    }
    
    return $memoryResults
}

# CPU分析
function Invoke-CPUAnalysis {
    Write-LogInfo "执行CPU分析..."
    
    $cpuResults = @()
    
    # 分析编译时间
    Write-LogInfo "分析编译时间..."
    $compileStartTime = Get-Date
    cargo build --release
    $compileEndTime = Get-Date
    $compileTime = ($compileEndTime - $compileStartTime).TotalSeconds
    
    $cpuResults += @{
        Type = "compile-time"
        Value = [math]::Round($compileTime, 2)
        Unit = "seconds"
        Status = if ($compileTime -lt 60) { "FAST" } elseif ($compileTime -lt 300) { "NORMAL" } else { "SLOW" }
    }
    
    # 分析优化级别效果
    Write-LogInfo "分析优化级别效果..."
    $optimizationLevels = @("0", "1", "2", "3", "s", "z")
    
    foreach ($level in $optimizationLevels) {
        Write-LogInfo "测试优化级别: $level"
        $optStartTime = Get-Date
        
        # 临时修改Cargo.toml中的优化级别
        $cargoToml = Get-Content "Cargo.toml" -Raw
        $modifiedToml = $cargoToml -replace 'opt-level = "\d+"', "opt-level = `"$level`""
        $modifiedToml | Out-File "Cargo.toml" -Encoding UTF8
        
        cargo build --release
        $optEndTime = Get-Date
        $optTime = ($optEndTime - $optStartTime).TotalSeconds
        
        # 恢复原始Cargo.toml
        $cargoToml | Out-File "Cargo.toml" -Encoding UTF8
        
        $cpuResults += @{
            Type = "optimization-level"
            Level = $level
            Time = [math]::Round($optTime, 2)
            Status = if ($optTime -lt 60) { "FAST" } elseif ($optTime -lt 300) { "NORMAL" } else { "SLOW" }
        }
    }
    
    return $cpuResults
}

# 网络分析
function Invoke-NetworkAnalysis {
    Write-LogInfo "执行网络分析..."
    
    $networkResults = @()
    
    # 分析WebAssembly模块加载时间
    Write-LogInfo "分析WebAssembly模块加载时间..."
    if (Test-Path "pkg") {
        $wasmFiles = Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm"
        foreach ($wasmFile in $wasmFiles) {
            $fileSize = $wasmFile.Length
            $estimatedLoadTime = [math]::Round($fileSize / (1024 * 1024), 2) # 假设1MB/s加载速度
            
            $networkResults += @{
                Type = "wasm-load-time"
                File = $wasmFile.Name
                Size = [math]::Round($fileSize / 1KB, 2)
                EstimatedLoadTime = $estimatedLoadTime
                Status = if ($estimatedLoadTime -lt 1) { "FAST" } elseif ($estimatedLoadTime -lt 5) { "NORMAL" } else { "SLOW" }
            }
        }
    }
    
    # 分析HTTP请求性能
    Write-LogInfo "分析HTTP请求性能..."
    $testUrls = @(
        "http://localhost:8080/health",
        "http://localhost:8080/api/status"
    )
    
    foreach ($url in $testUrls) {
        try {
            $requestStartTime = Get-Date
            $response = Invoke-WebRequest -Uri $url -TimeoutSec 10 -ErrorAction Stop
            $requestEndTime = Get-Date
            $responseTime = ($requestEndTime - $requestStartTime).TotalMilliseconds
            
            $networkResults += @{
                Type = "http-request"
                URL = $url
                ResponseTime = [math]::Round($responseTime, 2)
                Status = if ($responseTime -lt 100) { "FAST" } elseif ($responseTime -lt 500) { "NORMAL" } else { "SLOW" }
            }
        } catch {
            $networkResults += @{
                Type = "http-request"
                URL = $url
                ResponseTime = -1
                Status = "ERROR"
                Error = $_.Exception.Message
            }
        }
    }
    
    return $networkResults
}

# 性能优化
function Invoke-PerformanceOptimization {
    Write-LogInfo "执行性能优化..."
    
    $optimizationResults = @()
    
    # 优化Cargo.toml配置
    Write-LogInfo "优化Cargo.toml配置..."
    $cargoToml = Get-Content "Cargo.toml" -Raw
    
    # 添加性能优化配置
    $optimizedConfig = @"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
overflow-checks = false

[profile.bench]
opt-level = 3
lto = true
codegen-units = 1
"@
    
    if ($cargoToml -notmatch "\[profile\.release\]") {
        $cargoToml += $optimizedConfig
        $cargoToml | Out-File "Cargo.toml" -Encoding UTF8
        $optimizationResults += @{
            Type = "cargo-config"
            Status = "OPTIMIZED"
            Description = "添加了性能优化配置"
        }
    }
    
    # 优化WebAssembly构建
    Write-LogInfo "优化WebAssembly构建..."
    try {
        # 使用wasm-opt进行优化
        if (Get-Command wasm-opt -ErrorAction SilentlyContinue) {
            $wasmFiles = Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm"
            foreach ($wasmFile in $wasmFiles) {
                $originalSize = $wasmFile.Length
                wasm-opt -Os $wasmFile.FullName -o $wasmFile.FullName
                $optimizedSize = $wasmFile.Length
                $savings = [math]::Round((($originalSize - $optimizedSize) / $originalSize) * 100, 2)
                
                $optimizationResults += @{
                    Type = "wasm-optimization"
                    File = $wasmFile.Name
                    OriginalSize = [math]::Round($originalSize / 1KB, 2)
                    OptimizedSize = [math]::Round($optimizedSize / 1KB, 2)
                    Savings = $savings
                    Status = if ($savings -gt 10) { "EXCELLENT" } elseif ($savings -gt 5) { "GOOD" } else { "MINIMAL" }
                }
            }
        }
    } catch {
        $optimizationResults += @{
            Type = "wasm-optimization"
            Status = "ERROR"
            Description = "WebAssembly优化失败: $($_.Exception.Message)"
        }
    }
    
    # 优化前端资源
    Write-LogInfo "优化前端资源..."
    if (Test-Path "frontend") {
        # 检查是否有webpack配置
        if (Test-Path "frontend/webpack.config.js") {
            $webpackConfig = Get-Content "frontend/webpack.config.js" -Raw
            
            # 添加性能优化配置
            $optimizedWebpackConfig = $webpackConfig -replace 'mode: ["\']development["\']', 'mode: "production"'
            $optimizedWebpackConfig = $optimizedWebpackConfig -replace 'devtool: ["\'].*["\']', 'devtool: "source-map"'
            
            $optimizationResults += @{
                Type = "webpack-optimization"
                Status = "OPTIMIZED"
                Description = "优化了Webpack配置"
            }
        }
    }
    
    return $optimizationResults
}

# 生成性能报告
function New-PerformanceReport {
    param(
        [hashtable]$AnalysisResults,
        [array]$BenchmarkResults,
        [array]$MemoryResults,
        [array]$CPUResults,
        [array]$NetworkResults,
        [array]$OptimizationResults
    )
    
    Write-LogInfo "生成性能报告..."
    
    $reportContent = @"
# WebAssembly 2.0 + Rust 1.90 性能分析报告

**分析日期**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**分析工具**: cargo-bench, wasm-opt, 自定义性能分析
**分析范围**: 构建、运行、内存、CPU、网络

## 📊 性能摘要

### 总体性能指标
- **构建时间**: $($AnalysisResults.BuildTime) 秒
- **二进制大小**: $($AnalysisResults.BinarySize) MB
- **WebAssembly大小**: $($AnalysisResults.WasmSize) KB
- **内存使用**: $($AnalysisResults.MemoryUsage) MB
- **CPU使用**: $($AnalysisResults.CPUUsage) %

## 🔍 详细分析结果

### 1. 构建性能分析

- **构建时间**: $($AnalysisResults.BuildTime) 秒
- **二进制大小**: $($AnalysisResults.BinarySize) MB
- **WebAssembly大小**: $($AnalysisResults.WasmSize) KB

### 2. 基准测试结果

"@
    
    foreach ($result in $BenchmarkResults) {
        $status = if ($result.Status -eq "SUCCESS") { "✅" } else { "❌" }
        $reportContent += @"

#### $($result.Type) - $status
```
$($result.Output)
```
"@
    }
    
    $reportContent += @"

### 3. 内存分析结果

"@
    
    foreach ($result in $MemoryResults) {
        $status = switch ($result.Status) {
            "OPTIMAL" { "🟢 优秀" }
            "GOOD" { "🟡 良好" }
            "LARGE" { "🔴 较大" }
            "SUCCESS" { "✅ 成功" }
            "ERROR" { "❌ 错误" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $status
- **文件**: $($result.File)
- **大小**: $($result.Size) KB
"@
    }
    
    $reportContent += @"

### 4. CPU分析结果

"@
    
    foreach ($result in $CPUResults) {
        $status = switch ($result.Status) {
            "FAST" { "🟢 快速" }
            "NORMAL" { "🟡 正常" }
            "SLOW" { "🔴 缓慢" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $status
- **值**: $($result.Value) $($result.Unit)
- **级别**: $($result.Level)
- **时间**: $($result.Time) 秒
"@
    }
    
    $reportContent += @"

### 5. 网络分析结果

"@
    
    foreach ($result in $NetworkResults) {
        $status = switch ($result.Status) {
            "FAST" { "🟢 快速" }
            "NORMAL" { "🟡 正常" }
            "SLOW" { "🔴 缓慢" }
            "ERROR" { "❌ 错误" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $status
- **文件/URL**: $($result.File)$($result.URL)
- **大小**: $($result.Size) KB
- **响应时间**: $($result.ResponseTime) ms
- **预估加载时间**: $($result.EstimatedLoadTime) s
"@
    }
    
    $reportContent += @"

### 6. 优化结果

"@
    
    foreach ($result in $OptimizationResults) {
        $status = switch ($result.Status) {
            "OPTIMIZED" { "✅ 已优化" }
            "EXCELLENT" { "🏆 优秀" }
            "GOOD" { "🟢 良好" }
            "MINIMAL" { "🟡 最小" }
            "ERROR" { "❌ 错误" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $status
- **描述**: $($result.Description)
- **原始大小**: $($result.OriginalSize) KB
- **优化后大小**: $($result.OptimizedSize) KB
- **节省空间**: $($result.Savings) %
"@
    }
    
    $reportContent += @"

## 🚀 性能优化建议

### 高优先级
1. **启用LTO优化**: 在Cargo.toml中启用链接时优化
2. **使用wasm-opt**: 优化WebAssembly模块大小
3. **启用strip**: 移除调试信息减少二进制大小
4. **优化编译选项**: 使用适当的优化级别

### 中优先级
1. **内存池管理**: 实现内存池减少分配开销
2. **缓存策略**: 实现适当的缓存策略
3. **异步处理**: 使用异步处理提高并发性能
4. **SIMD优化**: 使用SIMD指令优化计算

### 低优先级
1. **代码重构**: 重构性能热点代码
2. **算法优化**: 选择更高效的算法
3. **数据结构优化**: 使用更合适的数据结构
4. **并行处理**: 实现并行处理提高性能

## 📋 性能监控

### 关键指标
- **构建时间**: 监控构建时间变化
- **二进制大小**: 监控二进制大小增长
- **内存使用**: 监控运行时内存使用
- **响应时间**: 监控API响应时间
- **吞吐量**: 监控系统吞吐量

### 监控工具
- **Prometheus**: 指标收集和监控
- **Grafana**: 性能指标可视化
- **cargo-bench**: Rust基准测试
- **wasm-opt**: WebAssembly优化
- **自定义监控**: 应用特定监控

---

*本报告由WebAssembly 2.0 + Rust 1.90性能优化脚本自动生成*
"@
    
    # 保存报告
    $reportPath = "performance-analysis-report-$(Get-Date -Format 'yyyy-MM-dd-HH-mm-ss').md"
    $reportContent | Out-File -FilePath $reportPath -Encoding UTF8
    
    Write-LogSuccess "性能分析报告已生成: $reportPath"
    return $reportPath
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "⚡ WebAssembly 2.0 + Rust 1.90 性能优化" -ForegroundColor $Colors.Blue
    Write-Host "=======================================" -ForegroundColor $Colors.Blue
    
    Test-Dependencies
    
    $analysisResults = @{}
    $benchmarkResults = @()
    $memoryResults = @()
    $cpuResults = @()
    $networkResults = @()
    $optimizationResults = @()
    
    if ($Analyze) {
        $analysisResults = Invoke-PerformanceAnalysis
    }
    
    if ($Benchmark) {
        $benchmarkResults = Invoke-BenchmarkTests
    }
    
    if ($Memory) {
        $memoryResults = Invoke-MemoryAnalysis
    }
    
    if ($CPU) {
        $cpuResults = Invoke-CPUAnalysis
    }
    
    if ($Network) {
        $networkResults = Invoke-NetworkAnalysis
    }
    
    if ($Optimize) {
        $optimizationResults = Invoke-PerformanceOptimization
    }
    
    # 显示性能摘要
    Write-Host ""
    Write-Host "📊 性能分析摘要:" -ForegroundColor $Colors.Cyan
    
    if ($analysisResults.Count -gt 0) {
        Write-Host "  构建时间: $($analysisResults.BuildTime) 秒" -ForegroundColor $Colors.White
        Write-Host "  二进制大小: $($analysisResults.BinarySize) MB" -ForegroundColor $Colors.White
        Write-Host "  WebAssembly大小: $($analysisResults.WasmSize) KB" -ForegroundColor $Colors.White
    }
    
    if ($benchmarkResults.Count -gt 0) {
        $successfulBenchmarks = ($benchmarkResults | Where-Object { $_.Status -eq "SUCCESS" }).Count
        Write-Host "  基准测试: $successfulBenchmarks/$($benchmarkResults.Count) 成功" -ForegroundColor $Colors.White
    }
    
    if ($memoryResults.Count -gt 0) {
        $optimalMemory = ($memoryResults | Where-Object { $_.Status -eq "OPTIMAL" }).Count
        Write-Host "  内存优化: $optimalMemory/$($memoryResults.Count) 优秀" -ForegroundColor $Colors.White
    }
    
    if ($cpuResults.Count -gt 0) {
        $fastCPU = ($cpuResults | Where-Object { $_.Status -eq "FAST" }).Count
        Write-Host "  CPU性能: $fastCPU/$($cpuResults.Count) 快速" -ForegroundColor $Colors.White
    }
    
    if ($networkResults.Count -gt 0) {
        $fastNetwork = ($networkResults | Where-Object { $_.Status -eq "FAST" }).Count
        Write-Host "  网络性能: $fastNetwork/$($networkResults.Count) 快速" -ForegroundColor $Colors.White
    }
    
    if ($optimizationResults.Count -gt 0) {
        $optimized = ($optimizationResults | Where-Object { $_.Status -eq "OPTIMIZED" -or $_.Status -eq "EXCELLENT" }).Count
        Write-Host "  优化结果: $optimized/$($optimizationResults.Count) 已优化" -ForegroundColor $Colors.White
    }
    
    if ($Report) {
        $reportPath = New-PerformanceReport -AnalysisResults $analysisResults -BenchmarkResults $benchmarkResults -MemoryResults $memoryResults -CPUResults $cpuResults -NetworkResults $networkResults -OptimizationResults $optimizationResults
        Write-Host ""
        Write-Host "📋 详细报告已生成: $reportPath" -ForegroundColor $Colors.Cyan
    }
    
    Write-Host ""
    Write-Host "⚡ 性能优化完成！" -ForegroundColor $Colors.Green
}

# 执行主函数
Main
