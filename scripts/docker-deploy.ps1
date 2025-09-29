# WebAssembly 2.0 + Rust 1.90 Docker部署脚本
# 支持开发、测试和生产环境部署

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("dev", "test", "prod", "monitoring", "logging", "all")]
    [string]$Environment,
    
    [switch]$Build,
    [switch]$Clean,
    [switch]$Logs,
    [switch]$Status,
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
    Write-Host "WebAssembly 2.0 + Rust 1.90 Docker部署脚本" -ForegroundColor $Colors.Blue
    Write-Host ""
    Write-Host "用法: .\scripts\docker-deploy.ps1 -Environment <环境> [选项]" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "参数:" -ForegroundColor $Colors.White
    Write-Host "  -Environment <环境>   部署环境 (dev|test|prod|monitoring|logging|all)" -ForegroundColor $Colors.White
    Write-Host "  -Build                重新构建镜像" -ForegroundColor $Colors.White
    Write-Host "  -Clean                清理容器和镜像" -ForegroundColor $Colors.White
    Write-Host "  -Logs                 查看日志" -ForegroundColor $Colors.White
    Write-Host "  -Status               查看状态" -ForegroundColor $Colors.White
    Write-Host "  -Help                 显示此帮助信息" -ForegroundColor $Colors.White
    Write-Host ""
    Write-Host "示例:" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\docker-deploy.ps1 -Environment dev -Build" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\docker-deploy.ps1 -Environment prod -Clean" -ForegroundColor $Colors.White
    Write-Host "  .\scripts\docker-deploy.ps1 -Environment all -Status" -ForegroundColor $Colors.White
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

# 检查Docker
function Test-Docker {
    if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
        Write-LogError "Docker未安装或未启动"
        exit 1
    }
    
    if (-not (Get-Command docker-compose -ErrorAction SilentlyContinue)) {
        Write-LogError "Docker Compose未安装"
        exit 1
    }
    
    Write-LogSuccess "Docker环境检查通过"
}

# 清理Docker资源
function Clear-DockerResources {
    Write-LogInfo "清理Docker资源..."
    
    # 停止所有容器
    docker-compose down --remove-orphans
    
    # 清理未使用的镜像
    docker image prune -f
    
    # 清理未使用的卷
    docker volume prune -f
    
    # 清理未使用的网络
    docker network prune -f
    
    Write-LogSuccess "Docker资源清理完成"
}

# 构建镜像
function Build-Images {
    Write-LogInfo "构建Docker镜像..."
    
    $buildArgs = @("--build-arg", "BUILDKIT_INLINE_CACHE=1")
    
    switch ($Environment) {
        "dev" {
            docker-compose build wasm-dev
        }
        "test" {
            docker-compose build wasm-test
        }
        "prod" {
            docker-compose build wasm-prod wasm-microservice wasm-edge
        }
        "monitoring" {
            docker-compose build prometheus grafana
        }
        "logging" {
            docker-compose build elasticsearch kibana
        }
        "all" {
            docker-compose build
        }
    }
    
    Write-LogSuccess "Docker镜像构建完成"
}

# 部署服务
function Deploy-Services {
    Write-LogInfo "部署服务到 $Environment 环境..."
    
    switch ($Environment) {
        "dev" {
            docker-compose up -d wasm-dev postgres redis
        }
        "test" {
            docker-compose --profile testing up -d wasm-test postgres redis
        }
        "prod" {
            docker-compose --profile production up -d wasm-prod wasm-microservice wasm-edge nginx-lb postgres redis
        }
        "monitoring" {
            docker-compose --profile monitoring up -d prometheus grafana
        }
        "logging" {
            docker-compose --profile logging up -d elasticsearch kibana
        }
        "all" {
            docker-compose --profile production --profile monitoring --profile logging up -d
        }
    }
    
    Write-LogSuccess "服务部署完成"
}

# 查看服务状态
function Show-ServiceStatus {
    Write-LogInfo "查看服务状态..."
    
    Write-Host ""
    Write-Host "🐳 Docker容器状态:" -ForegroundColor $Colors.Cyan
    docker-compose ps
    
    Write-Host ""
    Write-Host "📊 资源使用情况:" -ForegroundColor $Colors.Cyan
    docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}"
    
    Write-Host ""
    Write-Host "💾 磁盘使用情况:" -ForegroundColor $Colors.Cyan
    docker system df
}

# 查看服务日志
function Show-ServiceLogs {
    Write-LogInfo "查看服务日志..."
    
    switch ($Environment) {
        "dev" {
            docker-compose logs -f wasm-dev
        }
        "test" {
            docker-compose logs -f wasm-test
        }
        "prod" {
            docker-compose logs -f wasm-prod wasm-microservice wasm-edge
        }
        "monitoring" {
            docker-compose logs -f prometheus grafana
        }
        "logging" {
            docker-compose logs -f elasticsearch kibana
        }
        "all" {
            docker-compose logs -f
        }
    }
}

# 健康检查
function Test-ServiceHealth {
    Write-LogInfo "执行健康检查..."
    
    $services = @()
    
    switch ($Environment) {
        "dev" {
            $services = @("wasm-dev:8080", "postgres:5432", "redis:6379")
        }
        "test" {
            $services = @("wasm-test:8080", "postgres:5432", "redis:6379")
        }
        "prod" {
            $services = @("wasm-prod:8080", "wasm-microservice:8080", "wasm-edge:8080", "nginx-lb:80")
        }
        "monitoring" {
            $services = @("prometheus:9090", "grafana:3000")
        }
        "logging" {
            $services = @("elasticsearch:9200", "kibana:5601")
        }
        "all" {
            $services = @("wasm-prod:8080", "wasm-microservice:8080", "wasm-edge:8080", "nginx-lb:80", "prometheus:9090", "grafana:3000")
        }
    }
    
    foreach ($service in $services) {
        $parts = $service.Split(':')
        $container = $parts[0]
        $port = $parts[1]
        
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$port/health" -TimeoutSec 5 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-LogSuccess "$container 健康检查通过"
            } else {
                Write-LogWarning "$container 健康检查失败 (状态码: $($response.StatusCode))"
            }
        } catch {
            Write-LogWarning "$container 健康检查失败: $($_.Exception.Message)"
        }
    }
}

# 显示部署信息
function Show-DeploymentInfo {
    Write-Host ""
    Write-Host "🎉 部署完成！" -ForegroundColor $Colors.Green
    Write-Host ""
    Write-Host "📋 服务信息:" -ForegroundColor $Colors.Cyan
    
    switch ($Environment) {
        "dev" {
            Write-Host "  开发环境: http://localhost:8080" -ForegroundColor $Colors.White
            Write-Host "  数据库: localhost:5432" -ForegroundColor $Colors.White
            Write-Host "  缓存: localhost:6379" -ForegroundColor $Colors.White
        }
        "test" {
            Write-Host "  测试环境: http://localhost:8080" -ForegroundColor $Colors.White
            Write-Host "  数据库: localhost:5432" -ForegroundColor $Colors.White
            Write-Host "  缓存: localhost:6379" -ForegroundColor $Colors.White
        }
        "prod" {
            Write-Host "  生产环境: http://localhost:80" -ForegroundColor $Colors.White
            Write-Host "  负载均衡: http://localhost:80" -ForegroundColor $Colors.White
            Write-Host "  数据库: localhost:5432" -ForegroundColor $Colors.White
            Write-Host "  缓存: localhost:6379" -ForegroundColor $Colors.White
        }
        "monitoring" {
            Write-Host "  Prometheus: http://localhost:9090" -ForegroundColor $Colors.White
            Write-Host "  Grafana: http://localhost:3001" -ForegroundColor $Colors.White
        }
        "logging" {
            Write-Host "  Elasticsearch: http://localhost:9200" -ForegroundColor $Colors.White
            Write-Host "  Kibana: http://localhost:5601" -ForegroundColor $Colors.White
        }
        "all" {
            Write-Host "  生产环境: http://localhost:80" -ForegroundColor $Colors.White
            Write-Host "  Prometheus: http://localhost:9090" -ForegroundColor $Colors.White
            Write-Host "  Grafana: http://localhost:3001" -ForegroundColor $Colors.White
            Write-Host "  Elasticsearch: http://localhost:9200" -ForegroundColor $Colors.White
            Write-Host "  Kibana: http://localhost:5601" -ForegroundColor $Colors.White
        }
    }
    
    Write-Host ""
    Write-Host "🛠️  管理命令:" -ForegroundColor $Colors.Cyan
    Write-Host "  查看状态: docker-compose ps" -ForegroundColor $Colors.White
    Write-Host "  查看日志: docker-compose logs -f" -ForegroundColor $Colors.White
    Write-Host "  停止服务: docker-compose down" -ForegroundColor $Colors.White
    Write-Host "  重启服务: docker-compose restart" -ForegroundColor $Colors.White
    Write-Host ""
}

# 主执行流程
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "🚀 WebAssembly 2.0 + Rust 1.90 Docker部署" -ForegroundColor $Colors.Blue
    Write-Host "=========================================" -ForegroundColor $Colors.Blue
    
    Test-Docker
    
    if ($Clean) {
        Clear-DockerResources
    }
    
    if ($Build) {
        Build-Images
    }
    
    if ($Status) {
        Show-ServiceStatus
        return
    }
    
    if ($Logs) {
        Show-ServiceLogs
        return
    }
    
    Deploy-Services
    
    Start-Sleep -Seconds 10
    
    Test-ServiceHealth
    Show-DeploymentInfo
}

# 执行主函数
Main
