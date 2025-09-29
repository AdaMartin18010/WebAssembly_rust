# WebAssembly 2.0 + Rust 1.90 安全审计脚本
# 全面的安全审计和漏洞扫描

param(
    [switch]$Full,
    [switch]$Quick,
    [switch]$Dependencies,
    [switch]$Code,
    [switch]$Docker,
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
    Write-Host "WebAssembly 2.0 + Rust 1.90 安全审计脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\security-audit.ps1 [选项]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "选项:" -ForegroundColor $Colors.White
    Write-Host "  -Full                完整安全审计" -ForegroundColor $Colors.White
    Write-Host "  -Quick               快速安全检查" -ForegroundColor $Colors.White
    Write-Host "  -Dependencies        依赖安全审计" -ForegroundColor $Colors.White
    Write-Host "  -Code                代码安全审计" -ForegroundColor $Colors.White
    Write-Host "  -Docker              Docker安全审计" -ForegroundColor $Colors.White
    Write-Host "  -Network             网络安全审计" -ForegroundColor $Colors.White
    Write-Host "  -Report              生成安全报告" -ForegroundColor $Colors.White
    Write-Host "  -Help                显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\security-audit.ps1 -Full" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\security-audit.ps1 -Quick -Report" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\security-audit.ps1 -Dependencies -Code" -ForegroundColor $Colors.White
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

function Write-LogCritical {
    param([string]$Message)
    Write-Host "[CRITICAL] $Message" -ForegroundColor $Colors.Red
}

# 检查依赖
function Test-Dependencies {
    Write-LogInfo "检查安全审计工具..."
    
    $tools = @(
        @{Name="cargo"; Description="Rust包管理器"},
        @{Name="cargo-audit"; Description="Rust安全审计工具"},
        @{Name="cargo-deny"; Description="Rust依赖检查工具"},
        @{Name="trivy"; Description="容器安全扫描工具"},
        @{Name="npm"; Description="Node.js包管理器"},
        @{Name="npm audit"; Description="npm安全审计"}
    )
    
    $missingTools = @()
    
    foreach ($tool in $tools) {
        if (-not (Get-Command $tool.Name -ErrorAction SilentlyContinue)) {
            $missingTools += $tool.Description
        }
    }
    
    if ($missingTools.Count -gt 0) {
        Write-LogWarning "缺少以下安全工具:"
        foreach ($tool in $missingTools) {
            Write-Host "  - $tool" -ForegroundColor $Colors.Yellow
        }
        Write-LogInfo "正在安装缺失的工具..."
        
        # 安装cargo-audit
        if (-not (Get-Command cargo-audit -ErrorAction SilentlyContinue)) {
            cargo install cargo-audit
        }
        
        # 安装cargo-deny
        if (-not (Get-Command cargo-deny -ErrorAction SilentlyContinue)) {
            cargo install cargo-deny
        }
    }
    
    Write-LogSuccess "安全审计工具检查完成"
}

# Rust依赖安全审计
function Invoke-RustSecurityAudit {
    Write-LogInfo "执行Rust依赖安全审计..."
    
    $auditResults = @()
    
    try {
        # Cargo Audit
        Write-LogInfo "运行cargo audit..."
        $auditOutput = cargo audit 2>&1
        $auditResults += @{
            Tool = "cargo-audit"
            Status = if ($LASTEXITCODE -eq 0) { "PASSED" } else { "FAILED" }
            Output = $auditOutput
            Severity = if ($LASTEXITCODE -eq 0) { "INFO" } else { "HIGH" }
        }
        
        # Cargo Deny
        Write-LogInfo "运行cargo deny..."
        $denyOutput = cargo deny check 2>&1
        $auditResults += @{
            Tool = "cargo-deny"
            Status = if ($LASTEXITCODE -eq 0) { "PASSED" } else { "FAILED" }
            Output = $denyOutput
            Severity = if ($LASTEXITCODE -eq 0) { "INFO" } else { "MEDIUM" }
        }
        
        # 检查已知漏洞
        Write-LogInfo "检查已知漏洞..."
        $vulnerabilityCheck = cargo audit --deny warnings 2>&1
        $auditResults += @{
            Tool = "vulnerability-check"
            Status = if ($LASTEXITCODE -eq 0) { "PASSED" } else { "FAILED" }
            Output = $vulnerabilityCheck
            Severity = if ($LASTEXITCODE -eq 0) { "INFO" } else { "CRITICAL" }
        }
        
    } catch {
        Write-LogError "Rust安全审计失败: $($_.Exception.Message)"
        $auditResults += @{
            Tool = "rust-audit"
            Status = "ERROR"
            Output = $_.Exception.Message
            Severity = "HIGH"
        }
    }
    
    return $auditResults
}

# 代码安全审计
function Invoke-CodeSecurityAudit {
    Write-LogInfo "执行代码安全审计..."
    
    $codeResults = @()
    
    # 检查敏感信息泄露
    Write-LogInfo "检查敏感信息泄露..."
    $sensitivePatterns = @(
        "password\s*=\s*['`"][^'`"]+['`"]",
        "api_key\s*=\s*['`"][^'`"]+['`"]",
        "secret\s*=\s*['`"][^'`"]+['`"]",
        "token\s*=\s*['`"][^'`"]+['`"]",
        "private_key\s*=\s*['`"][^'`"]+['`"]"
    )
    
    foreach ($pattern in $sensitivePatterns) {
        $patternMatches = Get-ChildItem -Path . -Recurse -Include "*.rs", "*.toml", "*.js", "*.json" | 
                         Select-String -Pattern $pattern -AllMatches
        
        if ($patternMatches) {
            $codeResults += @{
                Type = "sensitive-info"
                Severity = "HIGH"
                Pattern = $pattern
                Matches = $patternMatches.Count
                Files = $patternMatches.Filename | Sort-Object -Unique
            }
        }
    }
    
    # 检查不安全的函数使用
    Write-LogInfo "检查不安全的函数使用..."
    $unsafePatterns = @(
        "unsafe\s+\{",
        "std::ptr::",
        "std::mem::transmute",
        "std::mem::forget",
        "std::mem::uninitialized"
    )
    
    foreach ($pattern in $unsafePatterns) {
        $unsafeMatches = Get-ChildItem -Path . -Recurse -Include "*.rs" | 
                        Select-String -Pattern $pattern -AllMatches
        
        if ($unsafeMatches) {
            $codeResults += @{
                Type = "unsafe-code"
                Severity = "MEDIUM"
                Pattern = $pattern
                Matches = $unsafeMatches.Count
                Files = $unsafeMatches.Filename | Sort-Object -Unique
            }
        }
    }
    
    # 检查SQL注入风险
    Write-LogInfo "检查SQL注入风险..."
    $sqlPatterns = @(
        "format!\s*\(\s*['`"].*\{.*\}.*['`"]",
        "concat!\s*\(\s*['`"].*\{.*\}.*['`"]"
    )
    
    foreach ($pattern in $sqlPatterns) {
        $sqlMatches = Get-ChildItem -Path . -Recurse -Include "*.rs" | 
                     Select-String -Pattern $pattern -AllMatches
        
        if ($sqlMatches) {
            $codeResults += @{
                Type = "sql-injection"
                Severity = "HIGH"
                Pattern = $pattern
                Matches = $sqlMatches.Count
                Files = $sqlMatches.Filename | Sort-Object -Unique
            }
        }
    }
    
    return $codeResults
}

# Docker安全审计
function Invoke-DockerSecurityAudit {
    Write-LogInfo "执行Docker安全审计..."
    
    $dockerResults = @()
    
    # 检查Dockerfile安全配置
    Write-LogInfo "检查Dockerfile安全配置..."
    if (Test-Path "Dockerfile") {
        $dockerfileContent = Get-Content "Dockerfile" -Raw
        
        # 检查是否以root用户运行
        if ($dockerfileContent -notmatch "USER\s+\w+") {
            $dockerResults += @{
                Type = "root-user"
                Severity = "MEDIUM"
                Description = "容器可能以root用户运行"
                Recommendation = "添加USER指令指定非root用户"
            }
        }
        
        # 检查是否使用最新基础镜像
        $dockerMatch = $dockerfileContent -match "FROM\s+(\w+):(\w+)"
        if ($dockerMatch) {
            $tag = $matches[2]
            if ($tag -eq "latest") {
                $dockerResults += @{
                    Type = "latest-tag"
                    Severity = "LOW"
                    Description = "使用了latest标签，可能导致不可预测的构建"
                    Recommendation = "使用具体的版本标签"
                }
            }
        }
        
        # 检查是否包含敏感信息
        if ($dockerfileContent -match "ENV\s+.*PASSWORD|ENV\s+.*SECRET|ENV\s+.*KEY") {
            $dockerResults += @{
                Type = "sensitive-env"
                Severity = "HIGH"
                Description = "Dockerfile中可能包含敏感环境变量"
                Recommendation = "使用Docker secrets或外部配置管理"
            }
        }
    }
    
    # 检查docker-compose.yml安全配置
    Write-LogInfo "检查docker-compose.yml安全配置..."
    if (Test-Path "docker-compose.yml") {
        $composeContent = Get-Content "docker-compose.yml" -Raw
        
        # 检查是否暴露敏感端口
        if ($composeContent -match "ports:\s*-\s*['`"]?(\d+):") {
            $exposedPorts = [regex]::Matches($composeContent, "ports:\s*-\s*['`"]?(\d+):") | 
                           ForEach-Object { $_.Groups[1].Value }
            
            $sensitivePorts = @("22", "3306", "5432", "6379", "27017")
            foreach ($port in $exposedPorts) {
                if ($sensitivePorts -contains $port) {
                    $dockerResults += @{
                        Type = "sensitive-port"
                        Severity = "MEDIUM"
                        Description = "暴露了敏感端口: $port"
                        Recommendation = "使用内部网络或限制访问"
                    }
                }
            }
        }
    }
    
    return $dockerResults
}

# 网络安全审计
function Invoke-NetworkSecurityAudit {
    Write-LogInfo "执行网络安全审计..."
    
    $networkResults = @()
    
    # 检查HTTP vs HTTPS
    Write-LogInfo "检查HTTP vs HTTPS配置..."
    $httpFiles = Get-ChildItem -Path . -Recurse -Include "*.js", "*.html", "*.json" | 
                 Select-String -Pattern "http://" -AllMatches
    
    if ($httpFiles) {
        $networkResults += @{
            Type = "http-usage"
            Severity = "MEDIUM"
            Description = "发现HTTP链接使用"
            Count = $httpFiles.Count
            Recommendation = "使用HTTPS确保安全传输"
        }
    }
    
    # 检查CORS配置
    Write-LogInfo "检查CORS配置..."
    $corsFiles = Get-ChildItem -Path . -Recurse -Include "*.js", "*.rs" | 
                 Select-String -Pattern "Access-Control-Allow-Origin.*\*" -AllMatches
    
    if ($corsFiles) {
        $networkResults += @{
            Type = "cors-wildcard"
            Severity = "MEDIUM"
            Description = "发现CORS通配符配置"
            Count = $corsFiles.Count
            Recommendation = "限制CORS到特定域名"
        }
    }
    
    # 检查API端点安全
    Write-LogInfo "检查API端点安全..."
    $apiFiles = Get-ChildItem -Path . -Recurse -Include "*.rs", "*.js" | 
                Select-String -Pattern "app\.(get|post|put|delete)\s*\(\s*['\"][^'\"]*['\"]" -AllMatches
    
    if ($apiFiles) {
        $unprotectedEndpoints = $apiFiles | Where-Object { 
            $_.Line -notmatch "auth|login|token|session" 
        }
        
        if ($unprotectedEndpoints) {
            $networkResults += @{
                Type = "unprotected-api"
                Severity = "HIGH"
                Description = "发现未保护的API端点"
                Count = $unprotectedEndpoints.Count
                Recommendation = "添加身份验证和授权"
            }
        }
    }
    
    return $networkResults
}

# 生成安全报告
function New-SecurityReport {
    param(
        [array]$RustResults,
        [array]$CodeResults,
        [array]$DockerResults,
        [array]$NetworkResults
    )
    
    Write-LogInfo "生成安全审计报告..."
    
    $reportContent = @"
# WebAssembly 2.0 + Rust 1.90 安全审计报告

**审计日期**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**审计工具**: cargo-audit, cargo-deny, 自定义安全扫描
**审计范围**: 依赖、代码、Docker、网络

## 📊 审计摘要

### 总体安全状态
- **Rust依赖审计**: $(if ($RustResults | Where-Object { $_.Status -eq "FAILED" }) { "❌ 发现问题" } else { "✅ 通过" })
- **代码安全审计**: $(if ($CodeResults.Count -gt 0) { "❌ 发现问题" } else { "✅ 通过" })
- **Docker安全审计**: $(if ($DockerResults.Count -gt 0) { "❌ 发现问题" } else { "✅ 通过" })
- **网络安全审计**: $(if ($NetworkResults.Count -gt 0) { "❌ 发现问题" } else { "✅ 通过" })

## 🔍 详细审计结果

### 1. Rust依赖安全审计

"@
    
    foreach ($result in $RustResults) {
        $status = if ($result.Status -eq "PASSED") { "✅" } else { "❌" }
        $severity = switch ($result.Severity) {
            "CRITICAL" { "🔴 严重" }
            "HIGH" { "🟠 高" }
            "MEDIUM" { "🟡 中" }
            "LOW" { "🟢 低" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Tool) - $status $severity
```
$($result.Output)
```
"@
    }
    
    $reportContent += @"

### 2. 代码安全审计

"@
    
    foreach ($result in $CodeResults) {
        $severity = switch ($result.Severity) {
            "HIGH" { "🔴 高" }
            "MEDIUM" { "🟡 中" }
            "LOW" { "🟢 低" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $severity
- **匹配数量**: $($result.Matches)
- **影响文件**: $($result.Files -join ", ")
- **模式**: \`$($result.Pattern)\`
"@
    }
    
    $reportContent += @"

### 3. Docker安全审计

"@
    
    foreach ($result in $DockerResults) {
        $severity = switch ($result.Severity) {
            "HIGH" { "🔴 高" }
            "MEDIUM" { "🟡 中" }
            "LOW" { "🟢 低" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $severity
- **描述**: $($result.Description)
- **建议**: $($result.Recommendation)
"@
    }
    
    $reportContent += @"

### 4. 网络安全审计

"@
    
    foreach ($result in $NetworkResults) {
        $severity = switch ($result.Severity) {
            "HIGH" { "🔴 高" }
            "MEDIUM" { "🟡 中" }
            "LOW" { "🟢 低" }
            default { "ℹ️ 信息" }
        }
        
        $reportContent += @"

#### $($result.Type) - $severity
- **描述**: $($result.Description)
- **数量**: $($result.Count)
- **建议**: $($result.Recommendation)
"@
    }
    
    $reportContent += @"

## 🛡️ 安全建议

### 高优先级
1. **修复所有严重和高危漏洞**
2. **移除代码中的敏感信息**
3. **实施适当的身份验证和授权**
4. **使用HTTPS进行所有通信**

### 中优先级
1. **限制Docker容器的权限**
2. **配置适当的CORS策略**
3. **使用具体的镜像标签**
4. **实施输入验证和输出编码**

### 低优先级
1. **定期更新依赖项**
2. **实施安全编码实践**
3. **配置安全头**
4. **实施日志记录和监控**

## 📋 行动计划

1. **立即行动**: 修复所有严重和高危问题
2. **短期计划**: 解决中优先级问题
3. **长期计划**: 实施安全最佳实践
4. **持续改进**: 建立定期安全审计流程

---

*本报告由WebAssembly 2.0 + Rust 1.90安全审计脚本自动生成*
"@
    
    # 保存报告
    $reportPath = "security-audit-report-$(Get-Date -Format 'yyyy-MM-dd-HH-mm-ss').md"
    $reportContent | Out-File -FilePath $reportPath -Encoding UTF8
    
    Write-LogSuccess "安全审计报告已生成: $reportPath"
    return $reportPath
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "🛡️ WebAssembly 2.0 + Rust 1.90 安全审计" -ForegroundColor $Colors.Blue
    Write-Host "=========================================" -ForegroundColor $Colors.Blue
    
    Test-Dependencies
    
    $rustResults = @()
    $codeResults = @()
    $dockerResults = @()
    $networkResults = @()
    
    if ($Full -or $Dependencies) {
        $rustResults = Invoke-RustSecurityAudit
    }
    
    if ($Full -or $Code) {
        $codeResults = Invoke-CodeSecurityAudit
    }
    
    if ($Full -or $Docker) {
        $dockerResults = Invoke-DockerSecurityAudit
    }
    
    if ($Full -or $Network) {
        $networkResults = Invoke-NetworkSecurityAudit
    }
    
    if ($Quick) {
        Write-LogInfo "执行快速安全检查..."
        $rustResults = Invoke-RustSecurityAudit
        $codeResults = Invoke-CodeSecurityAudit
    }
    
    # 显示审计结果摘要
    Write-Host ""
    Write-Host "📊 安全审计结果摘要:" -ForegroundColor $Colors.Cyan
    
    $totalIssues = 0
    $criticalIssues = 0
    $highIssues = 0
    $mediumIssues = 0
    
    # 统计Rust审计结果
    foreach ($result in $rustResults) {
        if ($result.Status -eq "FAILED") {
            $totalIssues++
            switch ($result.Severity) {
                "CRITICAL" { $criticalIssues++ }
                "HIGH" { $highIssues++ }
                "MEDIUM" { $mediumIssues++ }
            }
        }
    }
    
    # 统计代码审计结果
    foreach ($result in $codeResults) {
        $totalIssues++
        switch ($result.Severity) {
            "HIGH" { $highIssues++ }
            "MEDIUM" { $mediumIssues++ }
        }
    }
    
    # 统计Docker审计结果
    foreach ($result in $dockerResults) {
        $totalIssues++
        switch ($result.Severity) {
            "HIGH" { $highIssues++ }
            "MEDIUM" { $mediumIssues++ }
        }
    }
    
    # 统计网络审计结果
    foreach ($result in $networkResults) {
        $totalIssues++
        switch ($result.Severity) {
            "HIGH" { $highIssues++ }
            "MEDIUM" { $mediumIssues++ }
        }
    }
    
    Write-Host "  总问题数: $totalIssues" -ForegroundColor $Colors.White
    Write-Host "  严重问题: $criticalIssues" -ForegroundColor $(if ($criticalIssues -gt 0) { $Colors.Red } else { $Colors.Green })
    Write-Host "  高危问题: $highIssues" -ForegroundColor $(if ($highIssues -gt 0) { $Colors.Red } else { $Colors.Green })
    Write-Host "  中危问题: $mediumIssues" -ForegroundColor $(if ($mediumIssues -gt 0) { $Colors.Yellow } else { $Colors.Green })
    
    if ($totalIssues -eq 0) {
        Write-LogSuccess "🎉 安全审计通过！未发现安全问题。"
    } else {
        if ($criticalIssues -gt 0) {
            Write-LogCritical "🚨 发现严重安全问题，需要立即修复！"
        } elseif ($highIssues -gt 0) {
            Write-LogError "⚠️ 发现高危安全问题，建议尽快修复！"
        } else {
            Write-LogWarning "⚠️ 发现中危安全问题，建议修复！"
        }
    }
    
    if ($Report) {
        $reportPath = New-SecurityReport -RustResults $rustResults -CodeResults $codeResults -DockerResults $dockerResults -NetworkResults $networkResults
        Write-Host ""
        Write-Host "📋 详细报告已生成: $reportPath" -ForegroundColor $Colors.Cyan
    }
    
    Write-Host ""
    Write-Host "🛡️ 安全审计完成！" -ForegroundColor $Colors.Green
}

# 执行主函数
Main
