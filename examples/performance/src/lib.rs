// 导出所有公共函数和类型
pub use crate::main::*;

mod main {
    use wasm_bindgen::prelude::*;
    use web_sys::console;
    use serde::{Deserialize, Serialize};

    // 简化的WebAssembly模块配置
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WasmModuleConfig {
        pub name: String,
        pub version: String,
    }

    impl WasmModuleConfig {
        pub fn new(name: &str, version: &str) -> Self {
            Self {
                name: name.to_string(),
                version: version.to_string(),
            }
        }
        
        pub fn validate(&self) -> Result<(), String> {
            if self.name.is_empty() {
                return Err("Module name cannot be empty".to_string());
            }
            if self.version.is_empty() {
                return Err("Module version cannot be empty".to_string());
            }
            Ok(())
        }
    }

    // 当 `wee_alloc` 特性被启用时，使用 `wee_alloc` 作为全局分配器
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    // 定义一个 `console.log` 的宏
    macro_rules! log {
        ( $( $t:tt )* ) => {
            console::log_1(&format!( $( $t )* ).into());
        }
    }

    // 性能测试结果结构体
    #[wasm_bindgen]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PerformanceResult {
        test_name: String,
        duration_ms: f64,
        iterations: u32,
        throughput: f64,
        memory_usage: usize,
    }

    #[wasm_bindgen]
    impl PerformanceResult {
        #[wasm_bindgen(getter)]
        pub fn test_name(&self) -> String {
            self.test_name.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn duration_ms(&self) -> f64 {
            self.duration_ms
        }

        #[wasm_bindgen(getter)]
        pub fn iterations(&self) -> u32 {
            self.iterations
        }

        #[wasm_bindgen(getter)]
        pub fn throughput(&self) -> f64 {
            self.throughput
        }

        #[wasm_bindgen(getter)]
        pub fn memory_usage(&self) -> usize {
            self.memory_usage
        }
    }

    // 高性能数学计算器
    #[wasm_bindgen]
    pub struct PerformanceCalculator {
        cache: std::collections::HashMap<u32, u64>,
    }

    #[wasm_bindgen]
    impl PerformanceCalculator {
        #[wasm_bindgen(constructor)]
        pub fn new() -> PerformanceCalculator {
            PerformanceCalculator {
                cache: std::collections::HashMap::new(),
            }
        }

        // 高性能斐波那契计算（带缓存）
        #[wasm_bindgen]
        pub fn fibonacci_cached(&mut self, n: u32) -> u64 {
            if let Some(&result) = self.cache.get(&n) {
                return result;
            }

            let result = match n {
                0 => 0,
                1 => 1,
                _ => self.fibonacci_cached(n - 1) + self.fibonacci_cached(n - 2),
            };

            self.cache.insert(n, result);
            result
        }

        // 矩阵乘法性能测试
        #[wasm_bindgen]
        pub fn matrix_multiply_benchmark(&self, size: usize, iterations: u32) -> PerformanceResult {
            let start = js_sys::Date::now();
            
            // 创建测试矩阵
            let a = vec![1.0; size * size];
            let b = vec![2.0; size * size];
            
            for _ in 0..iterations {
                let _result = self.matrix_multiply(&a, &b, size, size, size);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Matrix Multiplication".to_string(),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: size * size * 8 * 3, // 3 matrices * 8 bytes per f64
            }
        }

        fn matrix_multiply(&self, a: &[f64], b: &[f64], rows_a: usize, cols_a: usize, cols_b: usize) -> Vec<f64> {
            let mut result = vec![0.0; rows_a * cols_b];
            
            for i in 0..rows_a {
                for j in 0..cols_b {
                    let mut sum = 0.0;
                    for k in 0..cols_a {
                        sum += a[i * cols_a + k] * b[k * cols_b + j];
                    }
                    result[i * cols_b + j] = sum;
                }
            }
            
            result
        }

        // 排序算法性能测试
        #[wasm_bindgen]
        pub fn sorting_benchmark(&self, size: usize, iterations: u32) -> PerformanceResult {
            let start = js_sys::Date::now();
            
            for _ in 0..iterations {
                let mut data: Vec<f64> = (0..size).map(|i| (size - i) as f64).collect();
                self.quick_sort(&mut data);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Quick Sort".to_string(),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: size * 8, // 8 bytes per f64
            }
        }

        fn quick_sort(&self, arr: &mut [f64]) {
            if arr.len() <= 1 {
                return;
            }
            
            let pivot_index = self.partition(arr);
            self.quick_sort(&mut arr[..pivot_index]);
            self.quick_sort(&mut arr[pivot_index + 1..]);
        }

        fn partition(&self, arr: &mut [f64]) -> usize {
            let pivot = arr[arr.len() - 1];
            let mut i = 0;
            
            for j in 0..arr.len() - 1 {
                if arr[j] <= pivot {
                    arr.swap(i, j);
                    i += 1;
                }
            }
            
            arr.swap(i, arr.len() - 1);
            i
        }

        // 字符串处理性能测试
        #[wasm_bindgen]
        pub fn string_processing_benchmark(&self, iterations: u32) -> PerformanceResult {
            let test_string = "Hello, WebAssembly 2.0 + Rust 1.90 Performance Test!";
            let start = js_sys::Date::now();
            
            for _ in 0..iterations {
                let _result = self.process_string(test_string);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "String Processing".to_string(),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: test_string.len() * iterations as usize,
            }
        }

        fn process_string(&self, input: &str) -> String {
            input
                .chars()
                .map(|c| c.to_uppercase().next().unwrap_or(c))
                .collect::<String>()
                .replace("HELLO", "HI")
                .replace("WORLD", "WASM")
        }
    }

    // 内存分配性能测试器
    #[wasm_bindgen]
    pub struct MemoryAllocator {
        allocations: Vec<Vec<u8>>,
    }

    #[wasm_bindgen]
    impl MemoryAllocator {
        #[wasm_bindgen(constructor)]
        pub fn new() -> MemoryAllocator {
            MemoryAllocator {
                allocations: Vec::new(),
            }
        }

        #[wasm_bindgen]
        pub fn allocation_benchmark(&mut self, size: usize, count: u32) -> PerformanceResult {
            let start = js_sys::Date::now();
            
            for _ in 0..count {
                let allocation = vec![0u8; size];
                self.allocations.push(allocation);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Memory Allocation".to_string(),
                duration_ms: duration,
                iterations: count,
                throughput: (count as f64) / (duration / 1000.0),
                memory_usage: size * count as usize,
            }
        }

        #[wasm_bindgen]
        pub fn deallocation_benchmark(&mut self) -> PerformanceResult {
            let count = self.allocations.len() as u32;
            let start = js_sys::Date::now();
            
            self.allocations.clear();
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Memory Deallocation".to_string(),
                duration_ms: duration,
                iterations: count,
                throughput: (count as f64) / (duration / 1000.0),
                memory_usage: 0,
            }
        }

        #[wasm_bindgen]
        pub fn clear(&mut self) {
            self.allocations.clear();
        }
    }

    // SIMD性能测试器
    #[wasm_bindgen]
    pub struct SimdCalculator {
        _simd_enabled: bool,
    }

    #[wasm_bindgen]
    impl SimdCalculator {
        #[wasm_bindgen(constructor)]
        pub fn new() -> SimdCalculator {
            SimdCalculator {
                _simd_enabled: true, // 假设SIMD可用
            }
        }

        #[wasm_bindgen]
        pub fn vector_add_benchmark(&self, size: usize, iterations: u32) -> PerformanceResult {
            let a: Vec<f64> = (0..size).map(|i| i as f64).collect();
            let b: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();
            
            let start = js_sys::Date::now();
            
            for _ in 0..iterations {
                let _result = self.vector_add(&a, &b);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Vector Addition".to_string(),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: size * 8 * 3, // 3 vectors * 8 bytes per f64
            }
        }

        fn vector_add(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
            a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect()
        }

        #[wasm_bindgen]
        pub fn dot_product_benchmark(&self, size: usize, iterations: u32) -> PerformanceResult {
            let a: Vec<f64> = (0..size).map(|i| i as f64).collect();
            let b: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();
            
            let start = js_sys::Date::now();
            
            for _ in 0..iterations {
                let _result = self.dot_product(&a, &b);
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: "Dot Product".to_string(),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: size * 8 * 2, // 2 vectors * 8 bytes per f64
            }
        }

        fn dot_product(&self, a: &[f64], b: &[f64]) -> f64 {
            a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
        }
    }

    // 综合性能测试套件
    #[wasm_bindgen]
    pub struct PerformanceTestSuite {
        calculator: PerformanceCalculator,
        allocator: MemoryAllocator,
        simd_calc: SimdCalculator,
    }

    #[wasm_bindgen]
    impl PerformanceTestSuite {
        #[wasm_bindgen(constructor)]
        pub fn new() -> PerformanceTestSuite {
            PerformanceTestSuite {
                calculator: PerformanceCalculator::new(),
                allocator: MemoryAllocator::new(),
                simd_calc: SimdCalculator::new(),
            }
        }

        #[wasm_bindgen]
        pub fn run_all_tests(&mut self) -> Result<JsValue, JsValue> {
            let mut results = Vec::new();
            
            // 数学计算测试
            let fib_result = PerformanceResult {
                test_name: "Fibonacci Cached".to_string(),
                duration_ms: {
                    let start = js_sys::Date::now();
                    let _ = self.calculator.fibonacci_cached(40);
                    js_sys::Date::now() - start
                },
                iterations: 1,
                throughput: 1.0,
                memory_usage: 0,
            };
            results.push(fib_result);
            
            // 矩阵乘法测试
            let matrix_result = self.calculator.matrix_multiply_benchmark(100, 100);
            results.push(matrix_result);
            
            // 排序测试
            let sort_result = self.calculator.sorting_benchmark(1000, 10);
            results.push(sort_result);
            
            // 字符串处理测试
            let string_result = self.calculator.string_processing_benchmark(1000);
            results.push(string_result);
            
            // 内存分配测试
            let alloc_result = self.allocator.allocation_benchmark(1024, 100);
            results.push(alloc_result);
            
            // 向量计算测试
            let vector_result = self.simd_calc.vector_add_benchmark(1000, 100);
            results.push(vector_result);
            
            // 点积测试
            let dot_result = self.simd_calc.dot_product_benchmark(1000, 100);
            results.push(dot_result);
            
            serde_wasm_bindgen::to_value(&results)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
        }

        #[wasm_bindgen]
        pub fn benchmark_wasm_module(&self, module_name: &str, _module_version: &str) -> PerformanceResult {
            let start = js_sys::Date::now();
            
            // 模拟WebAssembly模块执行
            let iterations = 1000;
            for _ in 0..iterations {
                // 这里应该实际执行WebAssembly模块
                std::hint::black_box(module_name.len());
            }
            
            let end = js_sys::Date::now();
            let duration = end - start;
            
            PerformanceResult {
                test_name: format!("Wasm Module: {}", module_name),
                duration_ms: duration,
                iterations,
                throughput: (iterations as f64) / (duration / 1000.0),
                memory_usage: module_name.len() * iterations as usize,
            }
        }
    }

    // 初始化函数
    #[wasm_bindgen(start)]
    pub fn main() {
        console_error_panic_hook::set_once();
        log!("WebAssembly 2.0 + Rust 1.90 Performance Example loaded!");
    }

    // 导出一个清理函数
    #[wasm_bindgen]
    pub fn cleanup() {
        log!("Cleaning up performance WebAssembly module...");
    }
}
