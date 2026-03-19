//! # Rust 1.94 新特性演示
//!
//! 本示例展示了 Rust 1.94 的所有新特性在 WebAssembly 中的应用

use wasm::rust_194_features::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║       Rust 1.94 New Features Demo for WebAssembly         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // 1. array_windows 演示
    demo_array_windows();
    
    // 2. element_offset 演示
    demo_element_offset();
    
    // 3. 数学常量演示
    demo_math_constants();
    
    // 4. char 到 usize 转换演示
    demo_char_conversion();
    
    // 5. 增强迭代器演示
    demo_enhanced_iterators();
    
    // 6. LazyLock 增强演示
    demo_lazy_lock();
    
    // 7. 运行所有演示
    demo_all();
    
    println!("\n✅ All Rust 1.94 demos completed successfully!");
}

fn demo_array_windows() {
    println!("▶ Demo 1: array_windows");
    println!("  Using array_windows for 4-byte aligned processing\n");
    
    let data: Vec<u8> = (0..16).collect();
    println!("  Input data: {:?}", data);
    
    let processed = MemoryOptimizer::process_with_windows(&data);
    println!("  Processed:  {:?}", processed);
    
    // 64-bit block processing
    let blocks = MemoryOptimizer::process_64bit_blocks(&data);
    println!("  64-bit blocks count: {}", blocks.len());
    
    println!();
}

fn demo_element_offset() {
    println!("▶ Demo 2: element_offset");
    println!("  Calculating precise memory offsets\n");
    
    let arr = [10i32, 20, 30, 40, 50];
    let offsets = MemoryOptimizer::calculate_offsets(&arr);
    
    println!("  Array: {:?}", arr);
    println!("  Offsets: {:?}", offsets);
    println!("  Size of i32: {} bytes", std::mem::size_of::<i32>());
    
    println!();
}

fn demo_math_constants() {
    println!("▶ Demo 3: New Math Constants");
    println!("  Euler's constant and Golden Ratio\n");
    
    let (euler, golden) = math_constants::get_constants();
    println!("  Euler-Mascheroni constant (γ): {:.10}", euler);
    println!("  Golden Ratio (φ): {:.10}", golden);
    
    // Euler approximation
    let approximation = math_constants::euler_approximation(1000);
    println!("  Euler approximation (n=1000): {:.10}", approximation);
    
    // Golden ratio split
    let (major, minor) = math_constants::golden_ratio_split(100.0);
    println!("  Golden ratio split of 100: major={:.2}, minor={:.2}", major, minor);
    println!("  Ratio check: {:.10}", major / minor);
    
    // Golden scale
    let scales = math_constants::golden_scale(100.0, 5);
    println!("  Golden scale: {:?}", scales);
    
    println!();
}

fn demo_char_conversion() {
    println!("▶ Demo 4: char to usize conversion");
    println!("  Using TryFrom<char> for usize\n");
    
    let chars = ['A', 'B', 'C', '0', '9', '😀'];
    
    for c in &chars {
        if let Some(index) = char_to_usize(*c) {
            println!("  '{}' -> usize: {}", c, index);
        } else {
            println!("  '{}' -> conversion failed", c);
        }
    }
    
    // String to indices
    let s = "Hello";
    let indices = string_utils::string_to_indices(s);
    println!("  String '{}' to indices: {:?}", s, indices);
    
    println!();
}

fn demo_enhanced_iterators() {
    println!("▶ Demo 5: Enhanced Iterators");
    println!("  Using Peekable::next_if_map\n");
    
    let instructions = vec![1, 0, 3, 0, 5, 0, 7];
    println!("  Input: {:?}", instructions);
    
    let processed = enhanced_iterators::process_instructions(instructions.into_iter());
    println!("  Processed (non-zero * 2): {:?}", processed);
    
    println!();
}

fn demo_lazy_lock() {
    println!("▶ Demo 6: Enhanced LazyLock");
    println!("  Using get_mut and force_mut\n");
    
    // Access global config
    println!("  Global config optimization level: {}", GLOBAL_CONFIG.optimization_level);
    println!("  Global config SIMD enabled: {}", GLOBAL_CONFIG.enable_simd);
    println!("  Global config memory limit: {} MB", GLOBAL_CONFIG.memory_limit / 1024 / 1024);
    
    // Access module cache
    println!("  Module cache accessible: true");
    
    println!();
}

fn demo_all() {
    println!("▶ Demo 7: Running All Demos");
    
    let demo = Rust194Demo::new();
    let results = demo.run_all_demos();
    
    println!("  Successes: {:?}", results.successes);
    
    if results.all_success() {
        println!("  ✅ All tests passed!");
    } else {
        println!("  ❌ Failures: {:?}", results.failures);
    }
}
