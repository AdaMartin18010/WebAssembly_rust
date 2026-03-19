//! # Rust 1.94 新特性性能基准测试
//!
//! 测试 Rust 1.94 新特性在 WebAssembly 场景中的性能表现

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use wasm::rust_194_features::*;

/// 测试 array_windows 性能
fn bench_array_windows(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_windows");
    
    for size in [100, 1000, 10000].iter() {
        let data: Vec<u8> = (0..*size).map(|i| (i % 256) as u8).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                MemoryOptimizer::process_with_windows(black_box(&data))
            });
        });
    }
    
    group.finish();
}

/// 测试 element_offset 性能
fn bench_element_offset(c: &mut Criterion) {
    let mut group = c.benchmark_group("element_offset");
    
    for size in [100, 1000, 10000].iter() {
        let data: Vec<u64> = (0..*size).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                MemoryOptimizer::calculate_offsets(black_box(&data))
            });
        });
    }
    
    group.finish();
}

/// 测试 64-bit 块处理性能
fn bench_64bit_blocks(c: &mut Criterion) {
    let mut group = c.benchmark_group("64bit_blocks");
    
    for size in [128, 1024, 8192].iter() {
        let data: Vec<u8> = (0..*size).map(|i| (i % 256) as u8).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                MemoryOptimizer::process_64bit_blocks(black_box(&data))
            });
        });
    }
    
    group.finish();
}

/// 测试数学常量计算性能
fn bench_math_constants(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_constants");
    
    group.bench_function("euler_approximation", |b| {
        b.iter(|| {
            math_constants::euler_approximation(black_box(1000))
        });
    });
    
    group.bench_function("golden_ratio_split", |b| {
        b.iter(|| {
            math_constants::golden_ratio_split(black_box(1000.0))
        });
    });
    
    group.bench_function("golden_scale", |b| {
        b.iter(|| {
            math_constants::golden_scale(black_box(100.0), black_box(10))
        });
    });
    
    group.finish();
}

/// 测试 char 转换性能
fn bench_char_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("char_conversion");
    
    let chars: Vec<char> = ('A'..='Z').chain('a'..='z').chain('0'..='9').collect();
    
    group.bench_function("char_to_usize", |b| {
        b.iter(|| {
            for c in &chars {
                black_box(char_to_usize(*c));
            }
        });
    });
    
    group.bench_function("string_to_indices", |b| {
        let s = "HelloWorld123";
        b.iter(|| {
            black_box(string_utils::string_to_indices(s))
        });
    });
    
    group.finish();
}

/// 测试增强迭代器性能
fn bench_enhanced_iterators(c: &mut Criterion) {
    let mut group = c.benchmark_group("enhanced_iterators");
    
    for size in [100, 1000, 10000].iter() {
        let data: Vec<u32> = (0..*size).map(|i| if i % 3 == 0 { 0 } else { i as u32 }).collect();
        
        group.bench_with_input(
            BenchmarkId::new("process_instructions", size), 
            size, 
            |b, _| {
                b.iter(|| {
                    enhanced_iterators::process_instructions(
                        black_box(data.clone().into_iter())
                    )
                });
            }
        );
    }
    
    group.finish();
}

/// 测试 LazyLock 访问性能
fn bench_lazy_lock(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_lock");
    
    group.bench_function("global_config_access", |b| {
        b.iter(|| {
            black_box(GLOBAL_CONFIG.optimization_level);
            black_box(GLOBAL_CONFIG.enable_simd);
            black_box(GLOBAL_CONFIG.memory_limit);
        });
    });
    
    group.bench_function("module_cache_access", |b| {
        b.iter(|| {
            black_box(WASM_MODULE_CACHE.get_module_mut("test"));
        });
    });
    
    group.finish();
}

/// 综合性能测试
fn bench_comprehensive(c: &mut Criterion) {
    c.bench_function("rust_194_demo_run", |b| {
        b.iter(|| {
            let demo = Rust194Demo::new();
            black_box(demo.run_all_demos())
        });
    });
}

criterion_group!(
    rust_194_benches,
    bench_array_windows,
    bench_element_offset,
    bench_64bit_blocks,
    bench_math_constants,
    bench_char_conversion,
    bench_enhanced_iterators,
    bench_lazy_lock,
    bench_comprehensive
);

criterion_main!(rust_194_benches);
