//! # Component Model 演示
//!
//! 本示例展示了 WebAssembly Component Model 的特性：
//! - WIT 接口定义
//! - 组件创建
//! - 组件组合
//! - 多语言互操作

use wasm::component_model::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        WebAssembly Component Model Demo                    ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // 1. WIT 接口演示
    demo_wit_interface();
    
    // 2. 组件创建演示
    demo_component_creation();
    
    // 3. 组件组合演示
    demo_component_composition();
    
    // 4. WIT 解析和生成演示
    demo_wit_parsing();
    
    // 5. 实际场景演示
    demo_real_world_scenario();
    
    println!("\n✅ All Component Model demos completed successfully!");
}

fn demo_wit_interface() {
    println!("▶ Demo 1: WIT Interface Definition");
    println!("  Defining interfaces with WebAssembly Interface Types\n");
    
    // 创建计算器接口
    let mut calculator = WitInterface::new("calculator");
    
    calculator.add_function(
        WitFunction::new("add")
            .with_param("a", WitType::S32)
            .with_param("b", WitType::S32)
            .with_result(WitType::S32)
    );
    
    calculator.add_function(
        WitFunction::new("subtract")
            .with_param("a", WitType::S32)
            .with_param("b", WitType::S32)
            .with_result(WitType::S32)
    );
    
    calculator.add_function(
        WitFunction::new("multiply")
            .with_param("a", WitType::S32)
            .with_param("b", WitType::S32)
            .with_result(WitType::S32)
    );
    
    calculator.add_function(
        WitFunction::new("divide")
            .with_param("a", WitType::F64)
            .with_param("b", WitType::F64)
            .with_result(WitType::Result(
                Box::new(WitType::F64),
                Box::new(WitType::String)
            ))
    );
    
    // 添加类型定义
    calculator.add_type(WitTypeDef {
        name: "CalculatorError".to_string(),
        definition: WitType::Variant(vec![
            ("DivideByZero".to_string(), None),
            ("InvalidInput".to_string(), Some(WitType::String)),
        ]),
        docs: "Errors that can occur in calculator operations".to_string(),
    });
    
    println!("  Interface: {}", calculator.name);
    println!("  Functions:");
    for func in &calculator.functions {
        let async_marker = if func.is_async { "async " } else { "" };
        println!("    - {}{}({} params)", 
            async_marker, 
            func.name, 
            func.params.len()
        );
    }
    println!("  Types: {}", calculator.types.len());
    
    // 生成 WIT 文本
    println!("\n  Generated WIT:\n");
    let wit_text = WitParser::generate(&calculator);
    for line in wit_text.lines() {
        println!("    {}", line);
    }
    
    println!();
}

fn demo_component_creation() {
    println!("▶ Demo 2: Component Creation");
    println!("  Creating WebAssembly components\n");
    
    // 创建计算器组件
    let calculator_component = Component::new("calculator")
        .with_version("1.0.0")
        .with_wit(examples::calculator_interface());
    
    println!("  Component: {} v{}", 
        calculator_component.name, 
        calculator_component.version
    );
    println!("  ID: {}...", &calculator_component.id.0[..8]);
    
    if let Some(ref wit) = calculator_component.wit {
        println!("  WIT interface: {} ({} functions)", 
            wit.name, 
            wit.functions.len()
        );
    }
    
    // 创建 HTTP 组件
    let http_component = Component::new("http-client")
        .with_version("0.5.0")
        .with_wit(examples::http_interface());
    
    println!("\n  Component: {} v{}", 
        http_component.name, 
        http_component.version
    );
    
    // 添加导入和导出
    let mut storage_component = Component::new("storage");
    storage_component.add_export(ComponentExport {
        name: "read".to_string(),
        interface: examples::http_interface(),
        as_name: None,
    });
    storage_component.add_export(ComponentExport {
        name: "write".to_string(),
        interface: examples::http_interface(),
        as_name: None,
    });
    
    println!("  Component: {} ({} exports)", 
        storage_component.name, 
        storage_component.exports.len()
    );
    
    println!();
}

fn demo_component_composition() {
    println!("▶ Demo 3: Component Composition");
    println!("  Composing multiple components together\n");
    
    let mut composer = ComponentComposer::new();
    
    // 创建组件
    let calc = Component::new("calculator");
    let api = Component::new("api");
    let db = Component::new("database");
    
    let calc_id = calc.id.clone();
    let api_id = api.id.clone();
    let db_id = db.id.clone();
    
    composer.add_component(calc);
    composer.add_component(api);
    composer.add_component(db);
    
    // 连接组件
    composer.connect(&calc_id, &api_id, "calculate");
    composer.connect(&api_id, &db_id, "query");
    
    println!("  Components: 3");
    println!("  Connections: 2");
    println!("  Graph nodes: {}", composer.graph_node_count());
    println!("  Graph edges: {}", composer.graph_edge_count());
    
    // 尝试组合
    match composer.compose() {
        Ok(composed) => {
            println!("  ✅ Composition successful!");
            println!("     Entry point: {}...", &composed.entry_point.0[..8]);
            println!("     Total components: {}", composed.components.len());
        }
        Err(e) => {
            println!("  ❌ Composition failed: {:?}", e);
        }
    }
    
    println!();
}

fn demo_wit_parsing() {
    println!("▶ Demo 4: WIT Parsing and Generation");
    println!("  Parsing and generating WIT definitions\n");
    
    let wit_text = r#"
interface parser-demo {
  type position = record {
    line: u32,
    column: u32,
  }
  
  type parse-error = variant {
    syntax-error(string),
    unexpected-eof,
  }
  
  func parse: func(input: string) -> result<list<token>, parse-error>
  func get-position: func() -> position
}
"#;
    
    println!("  Input WIT:");
    for line in wit_text.lines() {
        println!("    {}", line);
    }
    
    match WitParser::parse(wit_text) {
        Ok(interface) => {
            println!("\n  ✅ Parsed successfully!");
            println!("     Interface name: {}", interface.name);
            println!("     Functions: {}", interface.functions.len());
            
            // 重新生成
            let generated = WitParser::generate(&interface);
            println!("\n  Regenerated WIT:\n");
            for line in generated.lines() {
                println!("    {}", line);
            }
        }
        Err(e) => {
            println!("\n  ❌ Parse error: {:?}", e);
        }
    }
    
    println!();
}

fn demo_real_world_scenario() {
    println!("▶ Demo 5: Real-World Scenario");
    println!("  Building a complete application with Component Model\n");
    
    // 定义电商系统组件
    let mut composer = ComponentComposer::new();
    
    // 1. 用户服务
    let user_service = Component::new("user-service");
    let user_id = user_service.id.clone();
    composer.add_component(user_service);
    
    // 2. 订单服务
    let order_service = Component::new("order-service");
    let order_id = order_service.id.clone();
    composer.add_component(order_service);
    
    // 3. 支付服务
    let payment_service = Component::new("payment-service");
    let payment_id = payment_service.id.clone();
    composer.add_component(payment_service);
    
    // 4. 库存服务
    let inventory_service = Component::new("inventory-service");
    let inventory_id = inventory_service.id.clone();
    composer.add_component(inventory_service);
    
    // 5. API 网关
    let api_gateway = Component::new("api-gateway");
    let gateway_id = api_gateway.id.clone();
    composer.add_component(api_gateway);
    
    // 建立连接
    composer.connect(&gateway_id, &user_id, "auth");
    composer.connect(&gateway_id, &order_id, "orders");
    composer.connect(&order_id, &payment_id, "payment");
    composer.connect(&order_id, &inventory_id, "inventory");
    
    println!("  E-commerce System Architecture:");
    println!("    API Gateway -> User Service (auth)");
    println!("    API Gateway -> Order Service (orders)");
    println!("    Order Service -> Payment Service (payment)");
    println!("    Order Service -> Inventory Service (inventory)");
    
    match composer.compose() {
        Ok(composed) => {
            println!("\n  ✅ E-commerce system composed successfully!");
            println!("     Components: {}", composed.components.len());
            println!("     Entry point: API Gateway");
            
            // 列出所有组件
            println!("\n     Component list:");
            for (i, comp) in composed.components.iter().enumerate() {
                println!("       {}. {} ({})", 
                    i + 1, 
                    comp.name, 
                    &comp.id.0[..8]
                );
            }
        }
        Err(e) => {
            println!("\n  ❌ Composition failed: {:?}", e);
        }
    }
    
    println!();
}
