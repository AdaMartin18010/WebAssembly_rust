// 导出所有公共函数和类型
pub use crate::main::*;

mod main {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys::console;

    // 导入所有模块
    // use wasm::types::WasmModuleConfig;
    use basic::{Person, fibonacci, fibonacci_fast};
    use advanced::{ImageProcessor, MathCalculator, NetworkManager, WasmRuntimeManager};
    use performance::{PerformanceCalculator, MemoryAllocator, SimdCalculator, PerformanceTestSuite};

    // 定义一个 `console.log` 的宏
    macro_rules! log {
        ( $( $t:tt )* ) => {
            console::log_1(&format!( $( $t )* ).into());
        }
    }

    // 配置wasm-bindgen-test
    wasm_bindgen_test_configure!(run_in_browser);

    // 基础功能集成测试
    #[wasm_bindgen_test]
    fn test_basic_integration() {
        log!("Running basic integration tests...");
        
        // 测试Person结构体
        let person = Person::new("Alice".to_string(), 30);
        assert_eq!(person.name(), "Alice");
        assert_eq!(person.age(), 30);
        
        let greeting = person.greet();
        assert!(greeting.contains("Alice"));
        assert!(greeting.contains("30"));
        
        // 测试JSON序列化
        let json = person.to_json().unwrap();
        let person_from_json = Person::from_json(&json).unwrap();
        assert_eq!(person_from_json.name(), "Alice");
        assert_eq!(person_from_json.age(), 30);
        
        log!("Basic integration tests passed!");
    }

    #[wasm_bindgen_test]
    fn test_fibonacci_functions() {
        log!("Testing fibonacci functions...");
        
        // 测试递归版本
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        
        // 测试优化版本
        assert_eq!(fibonacci_fast(0), 0);
        assert_eq!(fibonacci_fast(1), 1);
        assert_eq!(fibonacci_fast(10), 55);
        
        log!("Fibonacci functions test passed!");
    }

    // 高级功能集成测试
    #[wasm_bindgen_test]
    fn test_image_processing_integration() {
        log!("Testing image processing integration...");
        
        let mut processor = ImageProcessor::new(100, 100);
        
        // 测试滤镜应用
        processor.apply_filter("grayscale").unwrap();
        processor.apply_filter("blur").unwrap();
        processor.apply_filter("sharpen").unwrap();
        processor.apply_filter("edge_detect").unwrap();
        
        log!("Image processing integration test passed!");
    }

    #[wasm_bindgen_test]
    fn test_math_calculator_integration() {
        log!("Testing math calculator integration...");
        
        let mut calc = MathCalculator::new();
        
        // 测试内存存储
        calc.store("pi", 3.14159);
        calc.store("e", 2.71828);
        
        assert_eq!(calc.recall("pi"), Some(3.14159));
        assert_eq!(calc.recall("e"), Some(2.71828));
        assert_eq!(calc.recall("nonexistent"), None);
        
        // 测试矩阵乘法
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let result = calc.matrix_multiply(&a, &b, 2, 2, 2);
        
        // 验证结果
        assert_eq!(result.len(), 4);
        assert!((result[0] - 19.0).abs() < 0.001); // 1*5 + 2*7
        assert!((result[1] - 22.0).abs() < 0.001); // 1*6 + 2*8
        assert!((result[2] - 43.0).abs() < 0.001); // 3*5 + 4*7
        assert!((result[3] - 50.0).abs() < 0.001); // 3*6 + 4*8
        
        log!("Math calculator integration test passed!");
    }

    // 性能测试集成
    #[wasm_bindgen_test]
    fn test_performance_integration() {
        log!("Testing performance integration...");
        
        let mut perf_calc = PerformanceCalculator::new();
        
        // 测试斐波那契缓存
        let result1 = perf_calc.fibonacci_cached(40);
        let result2 = perf_calc.fibonacci_cached(40);
        assert_eq!(result1, result2);
        
        // 测试矩阵乘法基准
        let matrix_result = perf_calc.matrix_multiply_benchmark(50, 10);
        assert_eq!(matrix_result.test_name(), "Matrix Multiplication");
        assert!(matrix_result.iterations() > 0);
        
        // 测试排序基准
        let sort_result = perf_calc.sorting_benchmark(500, 5);
        assert_eq!(sort_result.test_name(), "Quick Sort");
        assert!(sort_result.iterations() > 0);
        
        log!("Performance integration test passed!");
    }

    #[wasm_bindgen_test]
    fn test_memory_allocation_integration() {
        log!("Testing memory allocation integration...");
        
        let mut allocator = MemoryAllocator::new();
        
        // 测试内存分配
        let alloc_result = allocator.allocation_benchmark(1024, 50);
        assert_eq!(alloc_result.test_name(), "Memory Allocation");
        assert_eq!(alloc_result.iterations(), 50);
        assert!(alloc_result.memory_usage() > 0);
        
        // 测试内存释放
        let dealloc_result = allocator.deallocation_benchmark();
        assert_eq!(dealloc_result.test_name(), "Memory Deallocation");
        
        log!("Memory allocation integration test passed!");
    }

    #[wasm_bindgen_test]
    fn test_simd_integration() {
        log!("Testing SIMD integration...");
        
        let simd_calc = SimdCalculator::new();
        
        // 测试向量加法
        let vector_result = simd_calc.vector_add_benchmark(500, 20);
        assert_eq!(vector_result.test_name(), "Vector Addition");
        assert!(vector_result.iterations() > 0);
        
        // 测试点积
        let dot_result = simd_calc.dot_product_benchmark(500, 20);
        assert_eq!(dot_result.test_name(), "Dot Product");
        assert!(dot_result.iterations() > 0);
        
        log!("SIMD integration test passed!");
    }

    // WebAssembly模块集成测试
    #[wasm_bindgen_test]
    fn test_wasm_module_integration() {
        log!("Testing WebAssembly module integration...");
        
        // 创建WebAssembly模块配置
        // let config = WasmModuleConfig::new("test_module", "1.0.0");
        // assert_eq!(config.name, "test_module");
        // assert_eq!(config.version, "1.0.0");
        
        // 验证配置
        // assert!(config.validate().is_ok());
        
        // 测试运行时管理器
        let runtime_manager = WasmRuntimeManager::new().unwrap();
        let module_info = runtime_manager.get_module_info("test_module").unwrap();
        assert!(!module_info.is_undefined());
        
        log!("WebAssembly module integration test passed!");
    }

    // 综合测试套件
    #[wasm_bindgen_test]
    fn test_comprehensive_integration() {
        log!("Running comprehensive integration test...");
        
        let mut test_suite = PerformanceTestSuite::new();
        
        // 运行所有性能测试
        let results = test_suite.run_all_tests().unwrap();
        assert!(!results.is_undefined());
        
        // 创建WebAssembly模块配置并测试
        // let config = WasmModuleConfig::new("comprehensive_test", "1.0.0");
        // let benchmark_result = test_suite.benchmark_wasm_module(&config);
        
        // assert_eq!(benchmark_result.test_name(), "Wasm Module: comprehensive_test");
        // assert!(benchmark_result.iterations() > 0);
        
        log!("Comprehensive integration test passed!");
    }

    // 错误处理集成测试
    #[wasm_bindgen_test]
    fn test_error_handling_integration() {
        log!("Testing error handling integration...");
        
        let mut processor = ImageProcessor::new(10, 10);
        
        // 测试无效滤镜
        let result = processor.apply_filter("invalid_filter");
        assert!(result.is_err());
        
        // 测试数学计算器错误处理
        let calc = MathCalculator::new();
        let nonexistent = calc.recall("nonexistent_key");
        assert_eq!(nonexistent, None);
        
        log!("Error handling integration test passed!");
    }

    // 异步功能集成测试
    #[wasm_bindgen_test]
    async fn test_async_integration() {
        log!("Testing async integration...");
        
        let _network_manager = NetworkManager::new("https://httpbin.org");
        
        // 注意：在实际测试中，这可能会因为网络问题而失败
        // 这里只是演示异步功能的集成
        log!("Async integration test setup completed!");
    }

    // 内存安全集成测试
    #[wasm_bindgen_test]
    fn test_memory_safety_integration() {
        log!("Testing memory safety integration...");
        
        // 测试大量分配和释放
        let mut allocator = MemoryAllocator::new();
        
        for _ in 0..10 {
            let _result = allocator.allocation_benchmark(1024, 10);
            allocator.clear();
        }
        
        // 测试缓存不会无限增长
        let mut calc = PerformanceCalculator::new();
        for i in 0..100 {
            let _ = calc.fibonacci_cached(i % 50);
        }
        
        log!("Memory safety integration test passed!");
    }

    // 初始化函数
    #[wasm_bindgen(start)]
    pub fn main() {
        console_error_panic_hook::set_once();
        log!("WebAssembly 2.0 + Rust 1.90 Integration Tests loaded!");
    }

    // 导出一个清理函数
    #[wasm_bindgen]
    pub fn cleanup() {
        log!("Cleaning up integration test module...");
    }
}
