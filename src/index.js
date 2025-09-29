// 导入WebAssembly模块
import init, { 
    greet, 
    fibonacci, 
    fibonacci_fast, 
    process_array, 
    process_string,
    performance_test 
} from '../pkg/wasm.js';

// 导入高级功能模块
import { 
    ImageProcessor, 
    MathCalculator, 
    NetworkManager, 
    WasmRuntimeManager 
} from '../pkg/wasm.js';

// 导入性能测试模块
import { 
    PerformanceCalculator, 
    MemoryAllocator, 
    SimdCalculator, 
    PerformanceTestSuite 
} from '../pkg/wasm.js';

// 全局变量
let wasmModule = null;
let performanceSuite = null;

// 初始化函数
async function initializeWasm() {
    try {
        const statusEl = document.getElementById('status');
        statusEl.innerHTML = '<div class="loading"></div> 正在初始化 WebAssembly 模块...';
        
        // 初始化WebAssembly模块
        await init();
        wasmModule = await init();
        
        // 初始化性能测试套件
        performanceSuite = new PerformanceTestSuite();
        
        statusEl.innerHTML = '<div class="status success">✅ WebAssembly 模块加载成功！</div>';
        
        // 显示欢迎信息
        greet('WebAssembly 2.0 + Rust 1.90');
        
        // 初始化图像画布
        initializeImageCanvas();
        
    } catch (error) {
        console.error('初始化失败:', error);
        document.getElementById('status').innerHTML = 
            '<div class="status error">❌ 初始化失败: ' + error.message + '</div>';
    }
}

// 初始化图像画布
function initializeImageCanvas() {
    const canvas = document.getElementById('imageCanvas');
    const ctx = canvas.getContext('2d');
    
    // 绘制一个简单的测试图像
    const gradient = ctx.createLinearGradient(0, 0, 200, 200);
    gradient.addColorStop(0, '#ff6b6b');
    gradient.addColorStop(0.5, '#4ecdc4');
    gradient.addColorStop(1, '#45b7d1');
    
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, 200, 200);
    
    // 添加一些形状
    ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
    ctx.beginPath();
    ctx.arc(100, 100, 50, 0, 2 * Math.PI);
    ctx.fill();
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
    ctx.font = '16px Arial';
    ctx.textAlign = 'center';
    ctx.fillText('Test Image', 100, 105);
}

// 斐波那契测试函数
async function testFibonacci() {
    const input = parseInt(document.getElementById('fibInput').value);
    const resultEl = document.getElementById('fibResult');
    
    if (isNaN(input) || input < 0) {
        resultEl.innerHTML = '请输入有效的非负整数';
        return;
    }
    
    resultEl.innerHTML = '<div class="loading"></div> 计算中...';
    
    try {
        const start = performance.now();
        const result = fibonacci(input);
        const end = performance.now();
        
        resultEl.innerHTML = `
            <strong>斐波那契(${input}) = ${result}</strong><br>
            计算时间: ${(end - start).toFixed(2)}ms
        `;
    } catch (error) {
        resultEl.innerHTML = '计算失败: ' + error.message;
    }
}

// 快速斐波那契测试函数
async function testFibonacciFast() {
    const input = parseInt(document.getElementById('fibInput').value);
    const resultEl = document.getElementById('fibResult');
    
    if (isNaN(input) || input < 0) {
        resultEl.innerHTML = '请输入有效的非负整数';
        return;
    }
    
    resultEl.innerHTML = '<div class="loading"></div> 计算中...';
    
    try {
        const start = performance.now();
        const result = fibonacci_fast(input);
        const end = performance.now();
        
        resultEl.innerHTML = `
            <strong>快速斐波那契(${input}) = ${result}</strong><br>
            计算时间: ${(end - start).toFixed(2)}ms
        `;
    } catch (error) {
        resultEl.innerHTML = '计算失败: ' + error.message;
    }
}

// 字符串处理测试函数
async function testStringProcessing() {
    const input = document.getElementById('stringInput').value;
    const resultEl = document.getElementById('stringResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 处理中...';
    
    try {
        const start = performance.now();
        const result = process_string(input);
        const end = performance.now();
        
        resultEl.innerHTML = `
            <strong>原始:</strong> ${input}<br>
            <strong>处理后:</strong> ${result}<br>
            处理时间: ${(end - start).toFixed(2)}ms
        `;
    } catch (error) {
        resultEl.innerHTML = '处理失败: ' + error.message;
    }
}

// 数组处理测试函数
async function testArrayProcessing() {
    const input = document.getElementById('arrayInput').value;
    const resultEl = document.getElementById('arrayResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 处理中...';
    
    try {
        // 解析输入数组
        const numbers = input.split(',').map(x => parseFloat(x.trim()));
        
        if (numbers.some(isNaN)) {
            resultEl.innerHTML = '请输入有效的数字数组';
            return;
        }
        
        const start = performance.now();
        const result = process_array(numbers);
        const end = performance.now();
        
        resultEl.innerHTML = `
            <strong>原始数组:</strong> [${numbers.join(', ')}]<br>
            <strong>处理后:</strong> [${result.join(', ')}]<br>
            处理时间: ${(end - start).toFixed(2)}ms
        `;
    } catch (error) {
        resultEl.innerHTML = '处理失败: ' + error.message;
    }
}

// 应用图像滤镜
async function applyFilter() {
    const canvas = document.getElementById('imageCanvas');
    const filterType = document.getElementById('filterSelect').value;
    const statusEl = document.getElementById('status');
    
    statusEl.innerHTML = '<div class="loading"></div> 应用滤镜中...';
    
    try {
        const processor = new ImageProcessor(canvas);
        processor.apply_filter(filterType);
        processor.to_canvas(canvas);
        
        statusEl.innerHTML = `<div class="status success">✅ 滤镜 "${filterType}" 应用成功！</div>`;
    } catch (error) {
        statusEl.innerHTML = `<div class="status error">❌ 滤镜应用失败: ${error.message}</div>`;
    }
}

// 重置图像
function resetImage() {
    initializeImageCanvas();
    document.getElementById('status').innerHTML = '<div class="status info">🔄 图像已重置</div>';
}

// 测试矩阵乘法
async function testMatrixMultiply() {
    const input1 = document.getElementById('calcInput1').value;
    const input2 = document.getElementById('calcInput2').value;
    const resultEl = document.getElementById('calcResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 计算中...';
    
    try {
        const matrixA = input1.split(',').map(x => parseFloat(x.trim()));
        const matrixB = input2.split(',').map(x => parseFloat(x.trim()));
        
        if (matrixA.length !== 4 || matrixB.length !== 4) {
            resultEl.innerHTML = '请输入2x2矩阵（4个数字）';
            return;
        }
        
        const calc = new MathCalculator();
        const result = calc.matrix_multiply(matrixA, matrixB, 2, 2, 2);
        
        resultEl.innerHTML = `
            <strong>矩阵A:</strong> [${matrixA.join(', ')}]<br>
            <strong>矩阵B:</strong> [${matrixB.join(', ')}]<br>
            <strong>结果:</strong> [${result.join(', ')}]
        `;
    } catch (error) {
        resultEl.innerHTML = '计算失败: ' + error.message;
    }
}

// 测试内存分配
async function testMemoryAllocation() {
    const size = parseInt(document.getElementById('allocSize').value);
    const count = parseInt(document.getElementById('allocCount').value);
    const resultEl = document.getElementById('allocResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 测试中...';
    
    try {
        const allocator = new MemoryAllocator();
        const result = allocator.allocation_benchmark(size, count);
        
        resultEl.innerHTML = `
            <strong>测试名称:</strong> ${result.test_name()}<br>
            <strong>分配大小:</strong> ${size} bytes<br>
            <strong>分配次数:</strong> ${count}<br>
            <strong>总内存:</strong> ${(result.memory_usage() / 1024).toFixed(2)} KB<br>
            <strong>吞吐量:</strong> ${result.throughput().toFixed(2)} ops/sec
        `;
    } catch (error) {
        resultEl.innerHTML = '测试失败: ' + error.message;
    }
}

// 测试SIMD计算
async function testSimdCalculation() {
    const size = parseInt(document.getElementById('simdSize').value);
    const iterations = parseInt(document.getElementById('simdIterations').value);
    const resultEl = document.getElementById('simdResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 测试中...';
    
    try {
        const simdCalc = new SimdCalculator();
        const result = simdCalc.vector_add_benchmark(size, iterations);
        
        resultEl.innerHTML = `
            <strong>测试名称:</strong> ${result.test_name()}<br>
            <strong>向量大小:</strong> ${size}<br>
            <strong>迭代次数:</strong> ${iterations}<br>
            <strong>内存使用:</strong> ${(result.memory_usage() / 1024).toFixed(2)} KB<br>
            <strong>吞吐量:</strong> ${result.throughput().toFixed(2)} ops/sec
        `;
    } catch (error) {
        resultEl.innerHTML = '测试失败: ' + error.message;
    }
}

// 运行所有性能测试
async function runAllPerformanceTests() {
    const resultEl = document.getElementById('performanceResult');
    
    resultEl.innerHTML = '<div class="loading"></div> 运行所有性能测试...';
    
    try {
        const results = performanceSuite.run_all_tests();
        
        let html = '<h4>性能测试结果:</h4>';
        for (let result of results) {
            html += `
                <div style="margin: 10px 0; padding: 10px; background: rgba(0,0,0,0.2); border-radius: 5px;">
                    <strong>${result.test_name()}</strong><br>
                    耗时: ${result.duration_ms().toFixed(2)}ms<br>
                    迭代: ${result.iterations()}<br>
                    吞吐量: ${result.throughput().toFixed(2)} ops/sec<br>
                    内存: ${(result.memory_usage() / 1024).toFixed(2)} KB
                </div>
            `;
        }
        
        resultEl.innerHTML = html;
    } catch (error) {
        resultEl.innerHTML = '测试失败: ' + error.message;
    }
}

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', initializeWasm);

// 导出函数供HTML使用
window.testFibonacci = testFibonacci;
window.testFibonacciFast = testFibonacciFast;
window.testStringProcessing = testStringProcessing;
window.testArrayProcessing = testArrayProcessing;
window.applyFilter = applyFilter;
window.resetImage = resetImage;
window.testMatrixMultiply = testMatrixMultiply;
window.testMemoryAllocation = testMemoryAllocation;
window.testSimdCalculation = testSimdCalculation;
window.runAllPerformanceTests = runAllPerformanceTests;
