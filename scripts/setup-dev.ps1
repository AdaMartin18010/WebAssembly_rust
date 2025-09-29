# WebAssembly 2.0 + Rust 1.90 开发环境设置脚本
# 自动安装和配置开发环境

param(
    [switch]$Full,
    [switch]$Minimal,
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
    Write-Host "WebAssembly 2.0 + Rust 1.90 开发环境设置脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\setup-dev.ps1 [参数]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "参数:" -ForegroundColor $Colors.White
    Write-Host "  -Full              完整安装（包含所有工具和依赖）" -ForegroundColor $Colors.White
    Write-Host "  -Minimal           最小安装（仅核心工具）" -ForegroundColor $Colors.White
    Write-Host "  -Help              显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\setup-dev.ps1 -Full     # 完整安装" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\setup-dev.ps1 -Minimal  # 最小安装" -ForegroundColor $Colors.White
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

# 检查管理员权限
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# 安装Rust工具链
function Install-Rust {
    Write-LogInfo "安装Rust工具链..."
    
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        Write-LogInfo "Rust已安装，版本: $(rustc --version)"
        return
    }
    
    try {
        # 下载并运行rustup安装程序
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupInstaller = "$env:TEMP\rustup-init.exe"
        
        Write-LogInfo "下载Rust安装程序..."
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupInstaller
        
        Write-LogInfo "运行Rust安装程序..."
        Start-Process -FilePath $rustupInstaller -ArgumentList "-y" -Wait
        
        # 刷新环境变量
        $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
        
        Write-LogSuccess "Rust安装完成"
    }
    catch {
        Write-LogError "Rust安装失败: $($_.Exception.Message)"
        exit 1
    }
}

# 安装WebAssembly工具
function Install-WebAssemblyTools {
    Write-LogInfo "安装WebAssembly工具..."
    
    $tools = @(
        @{Name="wasm-pack"; Description="WebAssembly包管理器"},
        @{Name="wasm-opt"; Description="WebAssembly优化工具"},
        @{Name="wasm-bindgen-cli"; Description="WebAssembly绑定生成器"},
        @{Name="wasm-bindgen-test-runner"; Description="WebAssembly测试运行器"}
    )
    
    foreach ($tool in $tools) {
        if (-not (Get-Command $tool.Name -ErrorAction SilentlyContinue)) {
            Write-LogInfo "安装 $($tool.Description)..."
            cargo install $tool.Name
        } else {
            Write-LogInfo "$($tool.Description) 已安装"
        }
    }
    
    Write-LogSuccess "WebAssembly工具安装完成"
}

# 安装开发工具
function Install-DevTools {
    Write-LogInfo "安装开发工具..."
    
    $devTools = @(
        @{Name="cargo-watch"; Description="文件监控工具"},
        @{Name="cargo-expand"; Description="宏展开工具"},
        @{Name="cargo-audit"; Description="安全审计工具"},
        @{Name="cargo-deny"; Description="依赖检查工具"},
        @{Name="cargo-tarpaulin"; Description="代码覆盖率工具"},
        @{Name="cargo-criterion"; Description="基准测试工具"}
    )
    
    foreach ($tool in $devTools) {
        if (-not (Get-Command $tool.Name -ErrorAction SilentlyContinue)) {
            Write-LogInfo "安装 $($tool.Description)..."
            cargo install $tool.Name
        } else {
            Write-LogInfo "$($tool.Description) 已安装"
        }
    }
    
    Write-LogSuccess "开发工具安装完成"
}

# 安装Node.js和npm
function Install-NodeJS {
    Write-LogInfo "安装Node.js和npm..."
    
    if (Get-Command node -ErrorAction SilentlyContinue) {
        Write-LogInfo "Node.js已安装，版本: $(node --version)"
        return
    }
    
    try {
        # 使用Chocolatey安装Node.js
        if (Get-Command choco -ErrorAction SilentlyContinue) {
            choco install nodejs -y
        } else {
            Write-LogWarning "Chocolatey未安装，请手动安装Node.js"
            Write-LogInfo "下载地址: https://nodejs.org/"
        }
        
        Write-LogSuccess "Node.js安装完成"
    }
    catch {
        Write-LogError "Node.js安装失败: $($_.Exception.Message)"
    }
}

# 安装前端工具
function Install-FrontendTools {
    Write-LogInfo "安装前端工具..."
    
    if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
        Write-LogError "npm未安装，请先安装Node.js"
        return
    }
    
    $frontendTools = @(
        @{Name="webpack"; Description="模块打包器"},
        @{Name="webpack-cli"; Description="Webpack命令行工具"},
        @{Name="webpack-dev-server"; Description="开发服务器"},
        @{Name="typescript"; Description="TypeScript编译器"},
        @{Name="@types/node"; Description="Node.js类型定义"},
        @{Name="http-server"; Description="静态文件服务器"}
    )
    
    foreach ($tool in $frontendTools) {
        Write-LogInfo "安装 $($tool.Description)..."
        npm install -g $tool.Name
    }
    
    Write-LogSuccess "前端工具安装完成"
}

# 安装Docker
function Install-Docker {
    Write-LogInfo "安装Docker..."
    
    if (Get-Command docker -ErrorAction SilentlyContinue) {
        Write-LogInfo "Docker已安装，版本: $(docker --version)"
        return
    }
    
    try {
        # 使用Chocolatey安装Docker Desktop
        if (Get-Command choco -ErrorAction SilentlyContinue) {
            choco install docker-desktop -y
        } else {
            Write-LogWarning "Chocolatey未安装，请手动安装Docker Desktop"
            Write-LogInfo "下载地址: https://www.docker.com/products/docker-desktop"
        }
        
        Write-LogSuccess "Docker安装完成"
    }
    catch {
        Write-LogError "Docker安装失败: $($_.Exception.Message)"
    }
}

# 配置Git
function Set-GitConfig {
    Write-LogInfo "配置Git..."
    
    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        Write-LogWarning "Git未安装，请手动安装Git"
        return
    }
    
    # 设置Git配置
    git config --global init.defaultBranch main
    git config --global pull.rebase false
    git config --global core.autocrlf true
    
    Write-LogSuccess "Git配置完成"
}

# 创建开发环境配置文件
function New-DevConfig {
    Write-LogInfo "创建开发环境配置文件..."
    
    # 创建.vscode目录和配置文件
    if (-not (Test-Path ".vscode")) {
        New-Item -ItemType Directory -Path ".vscode"
    }
    
    # VS Code设置
    $vscodeSettings = @{
        "rust-analyzer.checkOnSave.command" = "clippy"
        "rust-analyzer.cargo.features" = "all"
        "rust-analyzer.procMacro.enable" = $true
        "files.associations" = @{
            "*.wasm" = "wasm"
            "*.wat" = "wat"
        }
        "emmet.includeLanguages" = @{
            "rust" = "html"
        }
    } | ConvertTo-Json -Depth 3
    
    $vscodeSettings | Out-File -FilePath ".vscode/settings.json" -Encoding UTF8
    
    # VS Code扩展推荐
    $vscodeExtensions = @{
        "recommendations" = @(
            "rust-lang.rust-analyzer",
            "vadimcn.vscode-lldb",
            "ms-vscode.vscode-json",
            "bradlc.vscode-tailwindcss",
            "ms-vscode.powershell"
        )
    } | ConvertTo-Json -Depth 2
    
    $vscodeExtensions | Out-File -FilePath ".vscode/extensions.json" -Encoding UTF8
    
    Write-LogSuccess "开发环境配置文件创建完成"
}

# 验证安装
function Test-Installation {
    Write-LogInfo "验证安装..."
    
    $tools = @(
        @{Name="rustc"; Description="Rust编译器"},
        @{Name="cargo"; Description="Rust包管理器"},
        @{Name="wasm-pack"; Description="WebAssembly包管理器"},
        @{Name="wasm-opt"; Description="WebAssembly优化工具"},
        @{Name="node"; Description="Node.js运行时"},
        @{Name="npm"; Description="Node.js包管理器"}
    )
    
    $allInstalled = $true
    
    foreach ($tool in $tools) {
        if (Get-Command $tool.Name -ErrorAction SilentlyContinue) {
            $version = & $tool.Name --version 2>$null
            Write-LogSuccess "$($tool.Description): $version"
        } else {
            Write-LogError "$($tool.Description) 未安装"
            $allInstalled = $false
        }
    }
    
    if ($allInstalled) {
        Write-LogSuccess "所有工具安装验证通过！"
    } else {
        Write-LogWarning "部分工具安装失败，请检查安装过程"
    }
}

# 显示环境信息
function Show-EnvironmentInfo {
    Write-Host ""
    Write-Host "🎉 开发环境设置完成！" -ForegroundColor $Colors.Green
    Write-Host ""
    Write-Host "📋 环境信息:" -ForegroundColor $Colors.Cyan
    Write-Host "  操作系统: $([System.Environment]::OSVersion.VersionString)" -ForegroundColor $Colors.White
    Write-Host "  PowerShell版本: $($PSVersionTable.PSVersion)" -ForegroundColor $Colors.White
    Write-Host "  架构: $([System.Environment]::GetEnvironmentVariable('PROCESSOR_ARCHITECTURE'))" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "🛠️  可用工具:" -ForegroundColor $Colors.Cyan
    Write-Host "  • Rust工具链 (rustc, cargo)" -ForegroundColor $Colors.White
    Write-Host "  • WebAssembly工具 (wasm-pack, wasm-opt)" -ForegroundColor $Colors.White
    Write-Host "  • 开发工具 (cargo-watch, cargo-expand等)" -ForegroundColor $Colors.White
    Write-Host "  • 前端工具 (webpack, typescript等)" -ForegroundColor $Colors.White
    Write-Host "  • 容器工具 (Docker)" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "🚀 下一步:" -ForegroundColor $Colors.Cyan
    Write-Host "  1. 运行 .\scripts\build.ps1 构建项目" -ForegroundColor $Colors.White
    Write-Host "  2. 运行 .\scripts\build.ps1 -Test 运行测试" -ForegroundColor $Colors.White
    Write-Host "  3. 查看 docs/ 目录了解项目文档" -ForegroundColor $Colors.White
    Write-Host ""
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "🚀 WebAssembly 2.0 + Rust 1.90 开发环境设置" -ForegroundColor $Colors.Blue
    Write-Host "=============================================" -ForegroundColor $Colors.Blue
    
    # 检查管理员权限
    if (-not (Test-Administrator)) {
        Write-LogWarning "建议以管理员权限运行此脚本以确保完整安装"
    }
    
    # 安装核心工具
    Install-Rust
    Install-WebAssemblyTools
    
    if ($Full) {
        Install-DevTools
        Install-NodeJS
        Install-FrontendTools
        Install-Docker
    }
    
    Set-GitConfig
    New-DevConfig
    Test-Installation
    Show-EnvironmentInfo
}

# 执行主函数
Main
