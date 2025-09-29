#!/bin/bash

# WebAssembly 2.0 + Rust 1.90 构建脚本
# 支持多种构建目标和优化选项

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 默认配置
TARGET="web"
PROFILE="release"
OPTIMIZE="size"
CLEAN=false
TEST=false
BENCH=false
DOC=false

# 帮助信息
show_help() {
    echo "WebAssembly 2.0 + Rust 1.90 构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -t, --target TARGET     构建目标 (web|nodejs|bundler|no-modules) [默认: web]"
    echo "  -p, --profile PROFILE   构建配置 (dev|release) [默认: release]"
    echo "  -o, --optimize OPT      优化选项 (size|speed|debug) [默认: size]"
    echo "  -c, --clean             清理构建缓存"
    echo "  --test                  运行测试"
    echo "  --bench                 运行基准测试"
    echo "  --doc                   生成文档"
    echo "  -h, --help              显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0                      # 默认构建"
    echo "  $0 -t nodejs -o speed   # 为Node.js构建，优化速度"
    echo "  $0 --test --doc         # 运行测试并生成文档"
    echo "  $0 -c --test            # 清理并运行测试"
}

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装"
        exit 1
    fi
    
    if ! command -v wasm-pack &> /dev/null; then
        log_warning "wasm-pack 未安装，正在安装..."
        cargo install wasm-pack
    fi
    
    if ! command -v wasm-opt &> /dev/null; then
        log_warning "wasm-opt 未安装，正在安装..."
        cargo install wasm-opt
    fi
    
    log_success "依赖检查完成"
}

# 清理构建缓存
clean_build() {
    log_info "清理构建缓存..."
    cargo clean
    rm -rf pkg/
    rm -rf target/
    log_success "清理完成"
}

# 设置构建配置
setup_build_config() {
    log_info "设置构建配置..."
    
    # 根据优化选项设置Cargo.toml
    case $OPTIMIZE in
        "size")
            OPT_LEVEL="s"
            LTO="true"
            CODECEN_UNITS="1"
            ;;
        "speed")
            OPT_LEVEL="3"
            LTO="fat"
            CODECEN_UNITS="16"
            ;;
        "debug")
            OPT_LEVEL="0"
            LTO="false"
            CODECEN_UNITS="16"
            ;;
        *)
            log_error "无效的优化选项: $OPTIMIZE"
            exit 1
            ;;
    esac
    
    log_info "优化配置: $OPTIMIZE (opt-level=$OPT_LEVEL, lto=$LTO)"
}

# 构建WebAssembly模块
build_wasm() {
    log_info "构建WebAssembly模块..."
    log_info "目标: $TARGET, 配置: $PROFILE, 优化: $OPTIMIZE"
    
    # 构建主模块
    wasm-pack build \
        --target $TARGET \
        --$PROFILE \
        --out-dir pkg \
        --scope wasm-rust \
        wasm
    
    # 构建示例
    for example in basic advanced performance; do
        log_info "构建示例: $example"
        wasm-pack build \
            --target $TARGET \
            --$PROFILE \
            --out-dir pkg/$example \
            --scope wasm-rust \
            examples/$example
    done
    
    log_success "WebAssembly模块构建完成"
}

# 优化WebAssembly文件
optimize_wasm() {
    log_info "优化WebAssembly文件..."
    
    # 优化主模块
    if [ -f "pkg/wasm_bg.wasm" ]; then
        wasm-opt -$OPT_LEVEL pkg/wasm_bg.wasm -o pkg/wasm_bg.wasm
        log_info "优化主模块完成"
    fi
    
    # 优化示例模块
    for example in basic advanced performance; do
        if [ -f "pkg/$example/${example}_bg.wasm" ]; then
            wasm-opt -$OPT_LEVEL pkg/$example/${example}_bg.wasm -o pkg/$example/${example}_bg.wasm
            log_info "优化示例 $example 完成"
        fi
    done
    
    log_success "WebAssembly文件优化完成"
}

# 运行测试
run_tests() {
    log_info "运行测试..."
    
    # 运行单元测试
    cargo test --workspace
    
    # 运行集成测试
    cargo test --package integration
    
    # 运行WebAssembly测试
    if command -v wasm-pack &> /dev/null; then
        wasm-pack test --headless --firefox
    fi
    
    log_success "测试完成"
}

# 运行基准测试
run_benchmarks() {
    log_info "运行基准测试..."
    
    cargo bench --workspace
    
    log_success "基准测试完成"
}

# 生成文档
generate_docs() {
    log_info "生成文档..."
    
    # 生成Rust文档
    cargo doc --workspace --no-deps --open
    
    # 生成WebAssembly文档
    if [ -f "pkg/wasm.d.ts" ]; then
        log_info "TypeScript定义文件已生成"
    fi
    
    log_success "文档生成完成"
}

# 显示构建结果
show_results() {
    log_info "构建结果:"
    
    if [ -d "pkg" ]; then
        echo "📦 构建产物:"
        find pkg -name "*.wasm" -exec ls -lh {} \; | while read line; do
            echo "  $line"
        done
        
        echo ""
        echo "📊 文件大小统计:"
        find pkg -name "*.wasm" -exec wc -c {} \; | awk '{sum += $1} END {print "  总大小: " sum " bytes (" sum/1024 " KB)"}'
    fi
    
    echo ""
    echo "🎯 构建目标: $TARGET"
    echo "⚙️  构建配置: $PROFILE"
    echo "🚀 优化选项: $OPTIMIZE"
    
    log_success "构建完成！"
}

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--target)
            TARGET="$2"
            shift 2
            ;;
        -p|--profile)
            PROFILE="$2"
            shift 2
            ;;
        -o|--optimize)
            OPTIMIZE="$2"
            shift 2
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        --test)
            TEST=true
            shift
            ;;
        --bench)
            BENCH=true
            shift
            ;;
        --doc)
            DOC=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            log_error "未知选项: $1"
            show_help
            exit 1
            ;;
    esac
done

# 主执行流程
main() {
    echo "🚀 WebAssembly 2.0 + Rust 1.90 构建脚本"
    echo "========================================"
    
    check_dependencies
    
    if [ "$CLEAN" = true ]; then
        clean_build
    fi
    
    setup_build_config
    build_wasm
    optimize_wasm
    
    if [ "$TEST" = true ]; then
        run_tests
    fi
    
    if [ "$BENCH" = true ]; then
        run_benchmarks
    fi
    
    if [ "$DOC" = true ]; then
        generate_docs
    fi
    
    show_results
}

# 执行主函数
main "$@"
