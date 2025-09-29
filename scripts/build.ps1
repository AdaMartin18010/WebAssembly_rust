# WebAssembly 2.0 + Rust 1.90 构建脚本 (PowerShell版本)
# 支持多种构建目标和优化选项

param(
    [string]$Target = "web",
    [string]$Profile = "release", 
    [string]$Optimize = "size",
    [switch]$Clean,
    [switch]$Test,
    [switch]$Bench,
    [switch]$Doc,
    [switch]$Help
)

# 颜色定义
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    White = "White"
}

# 帮助信息
function Show-Help {
    Write-Host "WebAssembly 2.0 + Rust 1.90 构建脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\build.ps1 [参数]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "参数:" -ForegroundColor $Colors.White
    Write-Host "  -Target TARGET     构建目标 (web|nodejs|bundler|no-modules) [默认: web]" -ForegroundColor $Colors.White
    Write-Host "  -Profile PROFILE   构建配置 (dev|release) [默认: release]" -ForegroundColor $Colors.White
    Write-Host "  -Optimize OPT      优化选项 (size|speed|debug) [默认: size]" -ForegroundColor $Colors.White
    Write-Host "  -Clean             清理构建缓存" -ForegroundColor $Colors.White
    Write-Host "  -Test              运行测试" -ForegroundColor $Colors.White
    Write-Host "  -Bench             运行基准测试" -ForegroundColor $Colors.White
    Write-Host "  -Doc               生成文档" -ForegroundColor $Colors.White
    Write-Host "  -Help              显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\build.ps1                      # 默认构建" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\build.ps1 -Target nodejs -Optimize speed   # 为Node.js构建，优化速度" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\build.ps1 -Test -Doc           # 运行测试并生成文档" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\build.ps1 -Clean -Test         # 清理并运行测试" -ForegroundColor $Colors.White
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
        Write-LogError "Cargo 未安装"
        exit 1
    }
    
    if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
        Write-LogWarning "wasm-pack 未安装，正在安装..."
        cargo install wasm-pack
    }
    
    if (-not (Get-Command wasm-opt -ErrorAction SilentlyContinue)) {
        Write-LogWarning "wasm-opt 未安装，正在安装..."
        cargo install wasm-opt
    }
    
    Write-LogSuccess "依赖检查完成"
}

# 清理构建缓存
function Clear-BuildCache {
    Write-LogInfo "清理构建缓存..."
    cargo clean
    if (Test-Path "pkg") { Remove-Item -Recurse -Force "pkg" }
    if (Test-Path "target") { Remove-Item -Recurse -Force "target" }
    Write-LogSuccess "清理完成"
}

# 设置构建配置
function Set-BuildConfig {
    Write-LogInfo "设置构建配置..."
    
    # 根据优化选项设置配置
    switch ($Optimize) {
        "size" {
            $script:OptLevel = "s"
            $script:Lto = "true"
            $script:CodegenUnits = "1"
        }
        "speed" {
            $script:OptLevel = "3"
            $script:Lto = "fat"
            $script:CodegenUnits = "16"
        }
        "debug" {
            $script:OptLevel = "0"
            $script:Lto = "false"
            $script:CodegenUnits = "16"
        }
        default {
            Write-LogError "无效的优化选项: $Optimize"
            exit 1
        }
    }
    
    Write-LogInfo "优化配置: $Optimize (opt-level=$OptLevel, lto=$Lto)"
}

# 构建WebAssembly模块
function Build-Wasm {
    Write-LogInfo "构建WebAssembly模块..."
    Write-LogInfo "目标: $Target, 配置: $Profile, 优化: $Optimize"
    
    # 构建主模块
    wasm-pack build --target $Target --$Profile --out-dir pkg --scope wasm-rust wasm
    
    # 构建示例
    $examples = @("basic", "advanced", "performance")
    foreach ($example in $examples) {
        Write-LogInfo "构建示例: $example"
        wasm-pack build --target $Target --$Profile --out-dir "pkg/$example" --scope wasm-rust "examples/$example"
    }
    
    Write-LogSuccess "WebAssembly模块构建完成"
}

# 优化WebAssembly文件
function Optimize-Wasm {
    Write-LogInfo "优化WebAssembly文件..."
    
    # 优化主模块
    if (Test-Path "pkg/wasm_bg.wasm") {
        wasm-opt "-$OptLevel" pkg/wasm_bg.wasm -o pkg/wasm_bg.wasm
        Write-LogInfo "优化主模块完成"
    }
    
    # 优化示例模块
    $examples = @("basic", "advanced", "performance")
    foreach ($example in $examples) {
        $wasmFile = "pkg/$example/${example}_bg.wasm"
        if (Test-Path $wasmFile) {
            wasm-opt "-$OptLevel" $wasmFile -o $wasmFile
            Write-LogInfo "优化示例 $example 完成"
        }
    }
    
    Write-LogSuccess "WebAssembly文件优化完成"
}

# 运行测试
function Invoke-Tests {
    Write-LogInfo "运行测试..."
    
    # 运行单元测试
    cargo test --workspace
    
    # 运行集成测试
    cargo test --package integration
    
    # 运行WebAssembly测试
    if (Get-Command wasm-pack -ErrorAction SilentlyContinue) {
        wasm-pack test --headless --firefox
    }
    
    Write-LogSuccess "测试完成"
}

# 运行基准测试
function Invoke-Benchmarks {
    Write-LogInfo "运行基准测试..."
    
    cargo bench --workspace
    
    Write-LogSuccess "基准测试完成"
}

# 生成文档
function New-Documentation {
    Write-LogInfo "生成文档..."
    
    # 生成Rust文档
    cargo doc --workspace --no-deps --open
    
    # 检查TypeScript定义文件
    if (Test-Path "pkg/wasm.d.ts") {
        Write-LogInfo "TypeScript定义文件已生成"
    }
    
    Write-LogSuccess "文档生成完成"
}

# 显示构建结果
function Show-Results {
    Write-LogInfo "构建结果:"
    
    if (Test-Path "pkg") {
        Write-Host "📦 构建产物:" -ForegroundColor $Colors.White
        Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm" | ForEach-Object {
            $size = [math]::Round($_.Length / 1KB, 2)
            Write-Host "  $($_.Name): $size KB" -ForegroundColor $Colors.White
        }
        
        Write-Host ""
        Write-Host "📊 文件大小统计:" -ForegroundColor $Colors.White
        $totalSize = (Get-ChildItem -Path "pkg" -Recurse -Filter "*.wasm" | Measure-Object -Property Length -Sum).Sum
        $totalSizeKB = [math]::Round($totalSize / 1KB, 2)
        Write-Host "  总大小: $totalSize bytes ($totalSizeKB KB)" -ForegroundColor $Colors.White
    }
    
    Write-Host ""
    Write-Host "🎯 构建目标: $Target" -ForegroundColor $Colors.White
    Write-Host "⚙️  构建配置: $Profile" -ForegroundColor $Colors.White
    Write-Host "🚀 优化选项: $Optimize" -ForegroundColor $Colors.White
    
    Write-LogSuccess "构建完成！"
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "🚀 WebAssembly 2.0 + Rust 1.90 构建脚本" -ForegroundColor $Colors.Blue
    Write-Host "========================================" -ForegroundColor $Colors.Blue
    
    Test-Dependencies
    
    if ($Clean) {
        Clear-BuildCache
    }
    
    Set-BuildConfig
    Build-Wasm
    Optimize-Wasm
    
    if ($Test) {
        Invoke-Tests
    }
    
    if ($Bench) {
        Invoke-Benchmarks
    }
    
    if ($Doc) {
        New-Documentation
    }
    
    Show-Results
}

# 执行主函数
Main
