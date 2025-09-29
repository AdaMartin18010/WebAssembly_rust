// 导出所有公共函数和类型
pub use crate::main::*;

mod main {
    use wasm_bindgen::prelude::*;
    use web_sys::console;
    use serde::{Deserialize, Serialize};

    // 当 `wee_alloc` 特性被启用时，使用 `wee_alloc` 作为全局分配器
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    // 定义一个 `console.log` 的宏，类似于 `println!` 宏
    macro_rules! log {
        ( $( $t:tt )* ) => {
            console::log_1(&format!( $( $t )* ).into());
        }
    }

    // 导出一个 `greet` 函数到 JavaScript
    #[wasm_bindgen]
    pub fn greet(name: &str) {
        log!("Hello, {}!", name);
    }

    // 导出一个结构体到 JavaScript
    #[wasm_bindgen]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Person {
        name: String,
        age: u32,
    }

    #[wasm_bindgen]
    impl Person {
        #[wasm_bindgen(constructor)]
        pub fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }

        #[wasm_bindgen(getter)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn age(&self) -> u32 {
            self.age
        }

        #[wasm_bindgen]
        pub fn greet(&self) -> String {
            format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
        }

        #[wasm_bindgen]
        pub fn to_json(&self) -> Result<String, JsValue> {
            serde_wasm_bindgen::to_value(self)
                .map(|v| v.as_string().unwrap_or_default())
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
        }

        #[wasm_bindgen]
        pub fn from_json(json: &str) -> Result<Person, JsValue> {
            let value = JsValue::from_str(json);
            serde_wasm_bindgen::from_value(value)
                .map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))
        }
    }

    // 导出一个计算函数
    #[wasm_bindgen]
    pub fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    // 导出一个更高效的计算函数
    #[wasm_bindgen]
    pub fn fibonacci_fast(n: u32) -> u32 {
        if n <= 1 {
            return n;
        }
        
        let mut a = 0;
        let mut b = 1;
        
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        b
    }

    // 导出一个数组处理函数
    #[wasm_bindgen]
    pub fn process_array(data: &[f64]) -> Vec<f64> {
        data.iter()
            .map(|x| x * x)  // 计算平方
            .collect()
    }

    // 导出一个字符串处理函数
    #[wasm_bindgen]
    pub fn process_string(input: &str) -> String {
        input.to_uppercase()
    }

    // 导出一个性能测试函数
    #[wasm_bindgen]
    pub fn performance_test(iterations: u32) -> f64 {
        let start = js_sys::Date::now();
        
        let mut sum = 0.0;
        for i in 0..iterations {
            sum += (i as f64).sin();
        }
        
        let end = js_sys::Date::now();
        let duration = end - start;
        
        log!("Performance test completed in {}ms", duration);
        
        sum
    }

    // 初始化函数，在模块加载时调用
    #[wasm_bindgen(start)]
    pub fn main() {
        // 设置 panic hook 以提供更好的错误信息
        console_error_panic_hook::set_once();
        
        log!("WebAssembly 2.0 + Rust 1.90 Basic Example loaded!");
    }

    // 导出一个清理函数
    #[wasm_bindgen]
    pub fn cleanup() {
        log!("Cleaning up WebAssembly module...");
    }
}
