//! # WebAssembly 3.0 特性演示
//!
//! 本示例展示了 WebAssembly 3.0 的新特性：
//! - Memory64 (64-bit memory addressing)
//! - WasmGC (Garbage Collection)
//! - Exception Handling (exnref)

use wasm::webassembly_3_0::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          WebAssembly 3.0 Features Demo                     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // 1. Memory64 演示
    demo_memory64();
    
    // 2. WasmGC 演示
    demo_wasmgc();
    
    // 3. 异常处理演示
    demo_exception_handling();
    
    // 4. 模块创建演示
    demo_module_creation();
    
    // 5. 运行时演示
    demo_runtime();
    
    println!("\n✅ All WebAssembly 3.0 demos completed successfully!");
}

fn demo_memory64() {
    println!("▶ Demo 1: Memory64 (64-bit Memory Addressing)");
    println!("  Breaking the 4GB barrier with 64-bit memory\n");
    
    // 创建 64 位内存（1 页 = 64KB）
    let mut memory = Memory64::new(10, Some(100));
    
    println!("  Initial pages: 10");
    println!("  Maximum pages: 100");
    println!("  Current size: {} bytes ({} MB)", 
        memory.size_bytes(), 
        memory.size_bytes() / 1024 / 1024
    );
    
    // 写入和读取测试
    memory.write_u64(0, 0x123456789ABCDEF0).unwrap();
    memory.write_u64(8, 0xFEDCBA9876543210).unwrap();
    
    let val1 = memory.read_u64(0).unwrap();
    let val2 = memory.read_u64(8).unwrap();
    
    println!("  Written/Read test 1: 0x{:016X}", val1);
    println!("  Written/Read test 2: 0x{:016X}", val2);
    
    // 内存增长测试
    let old_pages = memory.grow(5).unwrap();
    println!("  Grew memory: {} -> {} pages", old_pages, memory.size_pages());
    
    // 批量操作测试
    memory.bulk_fill(100, 0x42, 50).unwrap();
    let byte = memory.read_u8(125).unwrap();
    println!("  Bulk fill test at offset 125: 0x{:02X}", byte);
    
    memory.bulk_copy(100, 200, 50).unwrap();
    let copied = memory.read_u8(225).unwrap();
    println!("  Bulk copy test at offset 225: 0x{:02X}", copied);
    
    println!();
}

fn demo_wasmgc() {
    println!("▶ Demo 2: WasmGC (Garbage Collection)");
    println!("  Managed types for high-level languages\n");
    
    // 创建结构类型
    let point_struct = GcType::Struct(GcStruct {
        name: "Point".to_string(),
        fields: vec![
            GcField {
                name: "x".to_string(),
                value_type: ValueType::F64,
                mutable: true,
            },
            GcField {
                name: "y".to_string(),
                value_type: ValueType::F64,
                mutable: true,
            },
        ],
        super_type: None,
    });
    
    let person_struct = GcType::Struct(GcStruct {
        name: "Person".to_string(),
        fields: vec![
            GcField {
                name: "name".to_string(),
                value_type: ValueType::Ref(RefType::Any),
                mutable: false,
            },
            GcField {
                name: "age".to_string(),
                value_type: ValueType::I32,
                mutable: true,
            },
        ],
        super_type: None,
    });
    
    // 创建数组类型
    let int_array = GcType::Array(GcArray {
        element_type: ValueType::I32,
        mutable: true,
        min_size: 0,
        max_size: Some(10000),
    });
    
    println!("  Created struct type: Point (2 fields)");
    println!("  Created struct type: Person (2 fields)");
    println!("  Created array type: i32[] (max 10000 elements)");
    
    // 创建模块并添加 GC 类型
    let mut module = WebAssembly3Module::new("gc_demo");
    module.add_gc_type(point_struct);
    module.add_gc_type(person_struct);
    module.add_gc_type(int_array);
    
    println!("  Module '{}' has {} GC types", module.name, module.gc_types.len());
    
    // 演示引用类型
    let ref_types = vec![
        RefType::Any,
        RefType::Eq,
        RefType::I31,
        RefType::Struct(0),
        RefType::Array(2),
    ];
    
    println!("  Supported reference types:");
    for (i, rt) in ref_types.iter().enumerate() {
        println!("    {}. {:?}", i + 1, rt);
    }
    
    println!();
}

fn demo_exception_handling() {
    println!("▶ Demo 3: Exception Handling (exnref)");
    println!("  Advanced exception handling with exnref\n");
    
    // 创建异常标签
    let error_tag = ExnrefTag {
        index: 0,
        params: vec![ValueType::I32, ValueType::I32],
        name: "Error".to_string(),
    };
    
    let panic_tag = ExnrefTag {
        index: 1,
        params: vec![ValueType::I32],
        name: "Panic".to_string(),
    };
    
    // 创建 catch 块
    let catch_blocks = vec![
        CatchBlock {
            tag_index: 0,
            label: 1,
            handler_instructions: vec![
                Instruction::LocalGet(0),
                Instruction::Return,
            ],
            rethrow: false,
        },
        CatchBlock {
            tag_index: 1,
            label: 2,
            handler_instructions: vec![
                Instruction::LocalGet(0),
                Instruction::Return,
            ],
            rethrow: true,
        },
    ];
    
    let handler = ExnrefHandler {
        tag: error_tag.clone(),
        catch_blocks,
    };
    
    println!("  Exception tag: {} ({} params)", error_tag.name, error_tag.params.len());
    println!("  Exception tag: {} ({} params)", panic_tag.name, panic_tag.params.len());
    println!("  Handler has {} catch blocks", handler.catch_blocks.len());
    
    // 演示 TryTable 块
    let try_table = TryTableBlock {
        block_type: BlockType::Value(ValueType::I32),
        catches: vec![
            CatchClause::Catch(0, 1),
            CatchClause::CatchAll(2),
        ],
    };
    
    println!("  TryTable block with {} catch clauses", try_table.catches.len());
    
    // 演示指令
    let instructions = vec![
        Instruction::TryTable(try_table),
        Instruction::Throw(0),
        Instruction::Rethrow(0),
        Instruction::RefNull(RefType::Exn),
    ];
    
    println!("  Exception-related instructions: {}", instructions.len());
    
    println!();
}

fn demo_module_creation() {
    println!("▶ Demo 4: WebAssembly 3.0 Module Creation");
    println!("  Creating a complete WebAssembly 3.0 module\n");
    
    // 创建模块
    let mut module = WebAssembly3Module::new("my_app");
    module.version = "1.0.0".to_string();
    
    // 启用特性
    module.enable_feature(WebAssembly3Feature::Memory64);
    module.enable_feature(WebAssembly3Feature::WasmGC);
    module.enable_feature(WebAssembly3Feature::ExceptionHandlingExnref);
    module.enable_feature(WebAssembly3Feature::ReferenceTypes);
    
    println!("  Module: {} v{}", module.name, module.version);
    println!("  Enabled features:");
    for feature in &module.features {
        println!("    - {:?}", feature);
    }
    
    // 初始化内存
    module.init_memory64(1, Some(10));
    println!("  Memory64 initialized: 1 page (64KB)");
    
    // 添加函数
    let func = WebAssembly3Function {
        index: 0,
        name: Some("main".to_string()),
        params: vec![],
        results: vec![ValueType::I32],
        locals: vec![ValueType::I32],
        body: vec![
            Instruction::I32Const(42),
            Instruction::Return,
        ],
    };
    
    println!("  Function '{}' added", func.name.as_ref().unwrap());
    
    // 添加导出
    let export = Export {
        name: "main".to_string(),
        kind: ExportKind::Function,
        index: 0,
    };
    println!("  Export: {} ({:?})", export.name, export.kind);
    
    // 验证模块
    let validation = module.validate();
    println!("  Validation result: {}", if validation.valid { "✅ Valid" } else { "❌ Invalid" });
    
    if !validation.errors.is_empty() {
        for error in &validation.errors {
            println!("    Error: {:?}", error);
        }
    }
    
    println!("\n  Module ID: {}", module.id.0);
    
    println!();
}

fn demo_runtime() {
    println!("▶ Demo 5: WebAssembly 3.0 Runtime");
    println!("  Runtime with GC support\n");
    
    let mut runtime = WebAssembly3Runtime::new();
    
    // 创建模块
    let module = WebAssembly3Module::new("runtime_demo");
    let module_id = runtime.load_module(module);
    
    println!("  Loaded module with ID: {}...", &module_id.0[..8]);
    
    // 分配 GC 对象
    let struct_type = GcType::Struct(GcStruct {
        name: "TestObject".to_string(),
        fields: vec![],
        super_type: None,
    });
    
    let obj_id = runtime.allocate_gc_object(struct_type, vec![1, 2, 3, 4, 5]);
    println!("  Allocated GC object with ID: {}", obj_id);
    
    println!("  Total GC objects: {}", runtime.gc_object_count());
    
    if let Some(data) = runtime.get_gc_object_data(obj_id) {
        println!("  Object data: {:?}", data);
    }
    
    println!();
}
