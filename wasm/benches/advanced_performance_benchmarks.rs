//! # 高级性能基准测试套件
//!
//! 本模块提供了全面的性能基准测试，用于验证 WebAssembly 2.0 + Rust 1.90 的性能优化效果。
//! 包括 SIMD 操作、批量内存操作、尾调用优化等核心功能的性能测试。

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use wasm::webassembly_2_0::*;
use wasm::types::*;
use wasm::rust_189_features::*;
use std::hint::black_box;

/// SIMD 性能基准测试
/// SIMD Performance Benchmarks
fn bench_simd_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("SIMD Operations");
    
    // 测试不同大小的向量
    for size in [16, 64, 256, 1024, 4096].iter() {
        group.throughput(criterion::Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("V128Add", size), size, |b, &size| {
            let mut simd_processor = SimdProcessor::new();
            let vector_a = Value::V128([1; 16]);
            let vector_b = Value::V128([2; 16]);
            
            b.iter(|| {
                for _ in 0..size / 16 {
                    black_box(simd_processor.execute_simd(
                        SimdInstruction::V128Add,
                        [vector_a, vector_b]
                    ).unwrap());
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("V128Mul", size), size, |b, &size| {
            let mut simd_processor = SimdProcessor::new();
            let vector_a = Value::V128([1; 16]);
            let vector_b = Value::V128([2; 16]);
            
            b.iter(|| {
                for _ in 0..size / 16 {
                    black_box(simd_processor.execute_simd(
                        SimdInstruction::V128Mul,
                        [vector_a, vector_b]
                    ).unwrap());
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("V128And", size), size, |b, &size| {
            let mut simd_processor = SimdProcessor::new();
            let vector_a = Value::V128([0xFF; 16]);
            let vector_b = Value::V128([0xAA; 16]);
            
            b.iter(|| {
                for _ in 0..size / 16 {
                    black_box(simd_processor.execute_simd(
                        SimdInstruction::V128And,
                        [vector_a, vector_b]
                    ).unwrap());
                }
            });
        });
    }
    
    group.finish();
}

/// 批量内存操作性能基准测试
/// Bulk Memory Operations Performance Benchmarks
fn bench_bulk_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bulk Memory Operations");
    
    // 测试不同大小的内存操作
    for size in [1024, 4096, 16384, 65536, 262144].iter() {
        group.throughput(criterion::Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("BulkCopy", size), size, |b, &size| {
            let mut memory_manager = BulkMemoryManager::new(size * 2);
            
            // 初始化测试数据
            let test_data = vec![0xAA; size as usize];
            memory_manager.write_memory(0, &test_data).unwrap();
            
            b.iter(|| {
                black_box(memory_manager.bulk_copy(0, size as u32, size as u32).unwrap());
            });
        });
        
        group.bench_with_input(BenchmarkId::new("BulkFill", size), size, |b, &size| {
            let mut memory_manager = BulkMemoryManager::new(size);
            
            b.iter(|| {
                black_box(memory_manager.bulk_fill(0, 0xFF, size as u32).unwrap());
            });
        });
        
        group.bench_with_input(BenchmarkId::new("TraditionalCopy", size), size, |b, &size| {
            let src = vec![0xAA; size as usize];
            let mut dst = vec![0x00; size as usize];
            
            b.iter(|| {
                dst.copy_from_slice(&src);
                black_box(&dst);
            });
        });
    }
    
    group.finish();
}

/// 尾调用优化性能基准测试
/// Tail Call Optimization Performance Benchmarks
fn bench_tail_call_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tail Call Optimization");
    
    // 测试不同深度的递归调用
    for depth in [100, 500, 1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("TailCall", depth), depth, |b, &depth| {
            let mut optimizer = TailCallOptimizer::new();
            let args = vec![Value::I32(42), Value::I64(123)];
            
            b.iter(|| {
                for i in 0..depth {
                    black_box(optimizer.execute_tail_call(i % 10, args.clone()).unwrap());
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("RegularCall", depth), depth, |b, &depth| {
            b.iter(|| {
                for i in 0..depth {
                    // 模拟常规函数调用（不使用尾调用优化）
                    black_box(regular_function_call(i as i32, 123i64));
                }
            });
        });
    }
    
    group.finish();
}

/// 接口类型处理性能基准测试
/// Interface Type Processing Performance Benchmarks
fn bench_interface_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interface Types");
    
    // 测试不同数量的类型验证
    for count in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("TypeValidation", count), count, |b, &count| {
            let mut type_handler = InterfaceTypeHandler::new();
            
            // 注册测试类型
            for i in 0..count {
                type_handler.register_type(
                    format!("type_{}", i),
                    InterfaceType::Basic(ValueType::I32)
                );
            }
            
            let test_value = Value::I32(42);
            
            b.iter(|| {
                for i in 0..count {
                    black_box(type_handler.validate_interface_type(
                        &format!("type_{}", i),
                        &test_value
                    ).unwrap());
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("StringProcessing", count), count, |b, &count| {
            let test_strings: Vec<String> = (0..count)
                .map(|i| format!("test_string_{}_with_some_content", i))
                .collect();
            
            b.iter(|| {
                for string in &test_strings {
                    let wasm_value = Value::from_string(string);
                    black_box(wasm_value.as_v128());
                }
            });
        });
    }
    
    group.finish();
}

/// 宿主绑定性能基准测试
/// Host Binding Performance Benchmarks
fn bench_host_bindings(c: &mut Criterion) {
    let mut group = c.benchmark_group("Host Bindings");
    
    // 测试不同数量的绑定调用
    for count in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("JavaScriptCall", count), count, |b, &count| {
            let mut binding_manager = HostBindingManager::new();
            
            // 注册测试绑定
            for i in 0..count {
                binding_manager.register_binding(
                    format!("func_{}", i),
                    HostBindingType::JavaScriptFunction,
                    "test".to_string()
                );
            }
            
            let args = vec![Value::I32(42)];
            
            b.iter(|| {
                for i in 0..count {
                    black_box(binding_manager.call_javascript_function(
                        &format!("func_{}", i),
                        args.clone()
                    ).unwrap());
                }
            });
        });
    }
    
    group.finish();
}

/// WebAssembly 2.0 运行时性能基准测试
/// WebAssembly 2.0 Runtime Performance Benchmarks
fn bench_wasm2_runtime(c: &mut Criterion) {
    let mut group = c.benchmark_group("WebAssembly 2.0 Runtime");
    
    // 测试不同复杂度的模块加载
    for complexity in [1, 5, 10, 20, 50].iter() {
        group.bench_with_input(BenchmarkId::new("ModuleLoading", complexity), complexity, |b, &complexity| {
            b.iter(|| {
                let mut runtime = WebAssembly2Runtime::new();
                
                for i in 0..complexity {
                    let mut module = WebAssembly2Module::new(format!("module_{}", i));
                    module.enable_feature(WebAssembly2Features::SimdInstructions);
                    module.enable_feature(WebAssembly2Features::BulkMemoryOperations);
                    
                    // 添加测试函数
                    for j in 0..5 {
                        let function = WebAssembly2Function::new(
                            j,
                            format!("func_{}", j),
                            vec![ValueType::I32],
                            vec![ValueType::I32]
                        );
                        module.functions.push(function);
                    }
                    
                    black_box(runtime.load_module(module).unwrap());
                }
            });
        });
    }
    
    group.finish();
}

/// 内存使用效率基准测试
/// Memory Usage Efficiency Benchmarks
fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Efficiency");
    
    // 测试不同大小的内存分配
    for size in [1024, 4096, 16384, 65536, 262144].iter() {
        group.bench_with_input(BenchmarkId::new("Allocation", size), size, |b, &size| {
            b.iter(|| {
                let mut memory_manager = BulkMemoryManager::new(size);
                let data = vec![0xAA; size as usize];
                black_box(memory_manager.write_memory(0, &data).unwrap());
            });
        });
        
        group.bench_with_input(BenchmarkId::new("TraditionalAllocation", size), size, |b, &size| {
            b.iter(|| {
                let mut data = vec![0u8; size as usize];
                data.fill(0xAA);
                black_box(&data);
            });
        });
    }
    
    group.finish();
}

/// 并发性能基准测试
/// Concurrent Performance Benchmarks
fn bench_concurrent_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Concurrent Performance");
    
    // 测试不同线程数的并发执行
    for thread_count in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(BenchmarkId::new("MultiThreaded", thread_count), thread_count, |b, &thread_count| {
            b.iter(|| {
                use std::sync::Arc;
                use std::thread;
                
                let runtime = Arc::new(WebAssembly2Runtime::new());
                let mut handles = vec![];
                
                for _ in 0..thread_count {
                    let _runtime_clone = Arc::clone(&runtime);
                    let handle = thread::spawn(move || {
                        let mut local_runtime = WebAssembly2Runtime::new();
                        let mut module = WebAssembly2Module::new("test".to_string());
                        module.enable_feature(WebAssembly2Features::SimdInstructions);
                        
                        let function = WebAssembly2Function::new(0, "test".to_string(), vec![], vec![]);
                        module.functions.push(function);
                        
                        black_box(local_runtime.load_module(module).unwrap());
                    });
                    handles.push(handle);
                }
                
                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    }
    
    group.finish();
}

/// 错误处理性能基准测试
/// Error Handling Performance Benchmarks
fn bench_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Error Handling");
    
    // 测试不同错误率的处理性能
    for error_rate in [0, 10, 25, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("ErrorProcessing", error_rate), error_rate, |b, &error_rate| {
            b.iter(|| {
                for i in 0..1000 {
                    if i % 100 < error_rate {
                        // 模拟错误情况
                        let result: Result<Value, ValidationError> = Err(ValidationError::TypeMismatch {
                            expected: ValueType::I32,
                            actual: ValueType::I64,
                        });
                        let _ = black_box(result);
                    } else {
                        // 正常情况
                        let result: Result<Value, ValidationError> = Ok(Value::I32(42));
                        let _ = black_box(result);
                    }
                }
            });
        });
    }
    
    group.finish();
}

/// 辅助函数：常规函数调用（用于对比测试）
/// Helper function: Regular function call (for comparison testing)
fn regular_function_call(a: i32, b: i64) -> i32 {
    (a + b as i32) * 2
}

// 基准测试配置
// Benchmark Configuration
criterion_group!(
    benches,
    bench_simd_operations,
    bench_bulk_memory_operations,
    bench_tail_call_optimization,
    bench_interface_types,
    bench_host_bindings,
    bench_wasm2_runtime,
    bench_memory_efficiency,
    bench_concurrent_performance,
    bench_error_handling
);

criterion_main!(benches);
