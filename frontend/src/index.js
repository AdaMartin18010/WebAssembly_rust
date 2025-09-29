// WebAssembly 2.0 + Rust 1.90 前端集成示例
// 主要的JavaScript入口文件

import './styles/main.css';

// 导入WebAssembly模块
import init, { 
    fibonacci, 
    fibonacci_fast,
    run_matrix_multiply_benchmark,
    run_sorting_benchmark,
    run_simd_benchmark,
    apply_image_filter
} from '../pkg/wasm.js';

// 全局状态
let wasmModule = null;
let performanceStats = {
    loadTime: 0,
    memoryUsage: 0,
    executionTime: 0,
    throughput: 0
};

// 性能监控类
class PerformanceMonitor {
    constructor() {
        this.metrics = new Map();
        this.startTime = performance.now();
    }

    start(label) {
        this.metrics.set(label, performance.now());
    }

    end(label) {
        const startTime = this.metrics.get(label);
        if (startTime) {
            const duration = performance.now() - startTime;
            this.metrics.set(label, duration);
            return duration;
        }
        return 0;
    }

    getMetrics() {
        return Object.fromEntries(this.metrics);
    }
}

// 应用主类
class WasmApp {
    constructor() {
        this.monitor = new PerformanceMonitor();
        this.isInitialized = false;
    }

    async init() {
        try {
            this.monitor.start('init');
            
            // 初始化WebAssembly模块
            await init();
            wasmModule = true;
            this.isInitialized = true;
            
            const loadTime = this.monitor.end('init');
            this.updatePerformanceStats('loadTime', Math.round(loadTime));
            
            console.log('WebAssembly模块初始化成功');
            this.setupEventListeners();
            this.updateMemoryStats();
            
            // 定期更新内存统计
            setInterval(() => this.updateMemoryStats(), 1000);
            
        } catch (error) {
            console.error('WebAssembly模块初始化失败:', error);
            this.showError('WebAssembly模块初始化失败，请检查浏览器兼容性');
        }
    }

    setupEventListeners() {
        // 基础示例事件监听
        const basicBtn = document.getElementById('basicBtn');
        if (basicBtn) {
            basicBtn.addEventListener('click', () => this.runBasicDemo());
        }

        // 高级示例事件监听
        const advancedBtn = document.getElementById('advancedBtn');
        if (advancedBtn) {
            advancedBtn.addEventListener('click', () => this.runAdvancedDemo());
        }

        // 性能示例事件监听
        const performanceBtn = document.getElementById('performanceBtn');
        if (performanceBtn) {
            performanceBtn.addEventListener('click', () => this.runPerformanceDemo());
        }

        // 图像处理示例事件监听
        const imageBtn = document.getElementById('imageBtn');
        if (imageBtn) {
            imageBtn.addEventListener('click', () => this.runImageDemo());
        }
    }

    async runBasicDemo() {
        if (!this.isInitialized) {
            this.showError('WebAssembly模块未初始化');
            return;
        }

        const input = parseInt(document.getElementById('fibInput')?.value || 40);
        this.showLoading('basicLoading');

        try {
            this.monitor.start('basicDemo');
            const result = fibonacci(input);
            const executionTime = this.monitor.end('basicDemo');
            
            this.updatePerformanceStats('executionTime', Math.round(executionTime));
            this.showResult('basicResult', `斐波那契数列第${input}项: ${result}\n执行时间: ${Math.round(executionTime)}ms`);
            
        } catch (error) {
            this.showResult('basicResult', `错误: ${error.message}`);
        } finally {
            this.hideLoading('basicLoading');
        }
    }

    async runAdvancedDemo() {
        if (!this.isInitialized) {
            this.showError('WebAssembly模块未初始化');
            return;
        }

        const matrixSize = parseInt(document.getElementById('matrixSize')?.value || 100);
        const iterations = parseInt(document.getElementById('iterations')?.value || 100);
        this.showLoading('advancedLoading');

        try {
            this.monitor.start('advancedDemo');
            const result = run_matrix_multiply_benchmark(matrixSize, iterations);
            const executionTime = this.monitor.end('advancedDemo');
            
            this.updatePerformanceStats('executionTime', Math.round(executionTime));
            this.showResult('advancedResult', 
                `矩阵乘法基准测试完成\n` +
                `矩阵大小: ${matrixSize}x${matrixSize}\n` +
                `迭代次数: ${iterations}\n` +
                `执行时间: ${Math.round(executionTime)}ms\n` +
                `结果: ${JSON.stringify(result, null, 2)}`
            );
            
        } catch (error) {
            this.showResult('advancedResult', `错误: ${error.message}`);
        } finally {
            this.hideLoading('advancedLoading');
        }
    }

    async runPerformanceDemo() {
        if (!this.isInitialized) {
            this.showError('WebAssembly模块未初始化');
            return;
        }

        const testType = document.getElementById('perfTest')?.value || 'fibonacci';
        const size = parseInt(document.getElementById('perfSize')?.value || 1000);
        this.showLoading('performanceLoading');

        try {
            this.monitor.start('performanceDemo');
            let result;
            
            switch (testType) {
                case 'fibonacci':
                    result = fibonacci_fast(size);
                    break;
                case 'matrix':
                    result = run_matrix_multiply_benchmark(Math.sqrt(size), 10);
                    break;
                case 'sorting':
                    result = run_sorting_benchmark(size, 10);
                    break;
                case 'simd':
                    result = run_simd_benchmark(size, 100);
                    break;
                default:
                    throw new Error('未知的测试类型');
            }
            
            const executionTime = this.monitor.end('performanceDemo');
            const throughput = Math.round(size / (executionTime / 1000));
            
            this.updatePerformanceStats('executionTime', Math.round(executionTime));
            this.updatePerformanceStats('throughput', throughput);
            
            this.showResult('performanceResult', 
                `${testType} 性能测试完成\n` +
                `数据大小: ${size}\n` +
                `执行时间: ${Math.round(executionTime)}ms\n` +
                `吞吐量: ${throughput} ops/s\n` +
                `结果: ${JSON.stringify(result, null, 2)}`
            );
            
        } catch (error) {
            this.showResult('performanceResult', `错误: ${error.message}`);
        } finally {
            this.hideLoading('performanceLoading');
        }
    }

    async runImageDemo() {
        if (!this.isInitialized) {
            this.showError('WebAssembly模块未初始化');
            return;
        }

        const filterType = document.getElementById('imageFilter')?.value || 'grayscale';
        const imageSize = parseInt(document.getElementById('imageSize')?.value || 256);
        this.showLoading('imageLoading');

        try {
            this.monitor.start('imageDemo');
            const result = apply_image_filter(filterType, imageSize, imageSize);
            const executionTime = this.monitor.end('imageDemo');
            
            this.updatePerformanceStats('executionTime', Math.round(executionTime));
            this.showResult('imageResult', 
                `图像处理完成\n` +
                `滤镜类型: ${filterType}\n` +
                `图像大小: ${imageSize}x${imageSize}\n` +
                `执行时间: ${Math.round(executionTime)}ms\n` +
                `结果: ${JSON.stringify(result, null, 2)}`
            );
            
        } catch (error) {
            this.showResult('imageResult', `错误: ${error.message}`);
        } finally {
            this.hideLoading('imageLoading');
        }
    }

    updatePerformanceStats(stat, value) {
        performanceStats[stat] = value;
        const element = document.getElementById(stat);
        if (element) {
            element.textContent = value;
        }
    }

    updateMemoryStats() {
        if (performance.memory) {
            const memoryMB = Math.round(performance.memory.usedJSHeapSize / 1024 / 1024);
            this.updatePerformanceStats('memoryUsage', memoryMB);
        }
    }

    showLoading(loadingId) {
        const element = document.getElementById(loadingId);
        if (element) {
            element.style.display = 'block';
        }
    }

    hideLoading(loadingId) {
        const element = document.getElementById(loadingId);
        if (element) {
            element.style.display = 'none';
        }
    }

    showResult(resultId, content) {
        const element = document.getElementById(resultId);
        if (element) {
            element.textContent = content;
            element.style.display = 'block';
        }
    }

    showError(message) {
        console.error(message);
        // 可以在这里添加更友好的错误显示
        alert(message);
    }
}

// 工具函数
const utils = {
    // 格式化数字
    formatNumber(num) {
        return new Intl.NumberFormat('zh-CN').format(num);
    },

    // 格式化时间
    formatTime(ms) {
        if (ms < 1000) {
            return `${ms}ms`;
        } else if (ms < 60000) {
            return `${(ms / 1000).toFixed(2)}s`;
        } else {
            return `${(ms / 60000).toFixed(2)}min`;
        }
    },

    // 格式化字节
    formatBytes(bytes) {
        const sizes = ['B', 'KB', 'MB', 'GB'];
        if (bytes === 0) return '0 B';
        const i = Math.floor(Math.log(bytes) / Math.log(1024));
        return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
    },

    // 生成随机数据
    generateRandomData(size, type = 'number') {
        const data = [];
        for (let i = 0; i < size; i++) {
            switch (type) {
                case 'number':
                    data.push(Math.random() * 1000);
                    break;
                case 'integer':
                    data.push(Math.floor(Math.random() * 1000));
                    break;
                case 'string':
                    data.push(Math.random().toString(36).substring(7));
                    break;
                default:
                    data.push(Math.random());
            }
        }
        return data;
    }
};

// 全局函数（为了兼容HTML中的onclick）
window.runBasicDemo = () => app.runBasicDemo();
window.runAdvancedDemo = () => app.runAdvancedDemo();
window.runPerformanceDemo = () => app.runPerformanceDemo();
window.runImageDemo = () => app.runImageDemo();

// 创建应用实例
const app = new WasmApp();

// 页面加载完成后初始化应用
document.addEventListener('DOMContentLoaded', () => {
    app.init();
});

// 导出供其他模块使用
export { app, utils, PerformanceMonitor };
