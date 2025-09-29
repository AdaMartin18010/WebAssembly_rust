// 导出所有公共函数和类型
pub use crate::main::*;

mod main {
    use wasm_bindgen::prelude::*;
    use web_sys::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

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

    // 简化的WebAssembly运行时
    pub struct WasmRuntime {
        modules: HashMap<String, Vec<u8>>,
    }

    impl WasmRuntime {
        pub fn new() -> Result<Self, String> {
            Ok(Self {
                modules: HashMap::new(),
            })
        }
        
        pub fn load_module(&mut self, name: &str, wasm_bytes: &[u8]) -> Result<(), String> {
            self.modules.insert(name.to_string(), wasm_bytes.to_vec());
            Ok(())
        }
        
        pub fn call_function(&self, _module_name: &str, _function_name: &str, _args: &[f64]) -> Result<f64, String> {
            // 简化的实现
            Ok(0.0)
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

    // 高级图像处理结构体
    #[wasm_bindgen]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ImageProcessor {
        width: u32,
        height: u32,
        data: Vec<u8>,
    }

    #[wasm_bindgen]
    impl ImageProcessor {
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32) -> ImageProcessor {
            let size = (width * height * 4) as usize; // RGBA
            ImageProcessor {
                width,
                height,
                data: vec![255; size], // 初始化为白色
            }
        }

        #[wasm_bindgen]
        pub fn apply_filter(&mut self, filter_type: &str) -> Result<(), JsValue> {
            match filter_type {
                "grayscale" => self.grayscale_filter(),
                "blur" => self.blur_filter(),
                "sharpen" => self.sharpen_filter(),
                "edge_detect" => self.edge_detect_filter(),
                _ => Err(JsValue::from_str("Unknown filter type")),
            }
        }

        fn grayscale_filter(&mut self) -> Result<(), JsValue> {
            for i in (0..self.data.len()).step_by(4) {
                let r = self.data[i] as f32;
                let g = self.data[i + 1] as f32;
                let b = self.data[i + 2] as f32;
                
                let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                
                self.data[i] = gray;     // R
                self.data[i + 1] = gray; // G
                self.data[i + 2] = gray; // B
                // Alpha channel remains unchanged
            }
            Ok(())
        }

        fn blur_filter(&mut self) -> Result<(), JsValue> {
            let mut new_data = self.data.clone();
            let width = self.width as usize;
            let height = self.height as usize;
            
            for y in 1..height-1 {
                for x in 1..width-1 {
                    let mut r = 0u32;
                    let mut g = 0u32;
                    let mut b = 0u32;
                    
                    // 3x3 blur kernel
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let idx = ((y as i32 + dy) * width as i32 + (x as i32 + dx)) as usize * 4;
                            r += self.data[idx] as u32;
                            g += self.data[idx + 1] as u32;
                            b += self.data[idx + 2] as u32;
                        }
                    }
                    
                    let idx = (y * width + x) * 4;
                    new_data[idx] = (r / 9) as u8;
                    new_data[idx + 1] = (g / 9) as u8;
                    new_data[idx + 2] = (b / 9) as u8;
                }
            }
            
            self.data = new_data;
            Ok(())
        }

        fn sharpen_filter(&mut self) -> Result<(), JsValue> {
            let mut new_data = self.data.clone();
            let width = self.width as usize;
            let height = self.height as usize;
            
            // Sharpen kernel
            let kernel = [
                [0, -1, 0],
                [-1, 5, -1],
                [0, -1, 0],
            ];
            
            for y in 1..height-1 {
                for x in 1..width-1 {
                    let mut r = 0i32;
                    let mut g = 0i32;
                    let mut b = 0i32;
                    
                    for ky in 0..3 {
                        for kx in 0..3 {
                            let idx = ((y + ky - 1) * width + (x + kx - 1)) * 4;
                            let weight = kernel[ky][kx];
                            
                            r += (self.data[idx] as i32) * weight;
                            g += (self.data[idx + 1] as i32) * weight;
                            b += (self.data[idx + 2] as i32) * weight;
                        }
                    }
                    
                    let idx = (y * width + x) * 4;
                    new_data[idx] = r.max(0).min(255) as u8;
                    new_data[idx + 1] = g.max(0).min(255) as u8;
                    new_data[idx + 2] = b.max(0).min(255) as u8;
                }
            }
            
            self.data = new_data;
            Ok(())
        }

        fn edge_detect_filter(&mut self) -> Result<(), JsValue> {
            let mut new_data = vec![0u8; self.data.len()];
            let width = self.width as usize;
            let height = self.height as usize;
            
            // Sobel edge detection kernels
            let gx = [
                [-1, 0, 1],
                [-2, 0, 2],
                [-1, 0, 1],
            ];
            
            let gy = [
                [-1, -2, -1],
                [0, 0, 0],
                [1, 2, 1],
            ];
            
            for y in 1..height-1 {
                for x in 1..width-1 {
                    let mut gx_r = 0i32;
                    let mut gy_r = 0i32;
                    
                    for ky in 0..3 {
                        for kx in 0..3 {
                            let idx = ((y + ky - 1) * width + (x + kx - 1)) * 4;
                            let gray = (self.data[idx] as f32 * 0.299 + 
                                       self.data[idx + 1] as f32 * 0.587 + 
                                       self.data[idx + 2] as f32 * 0.114) as i32;
                            
                            gx_r += gray * gx[ky][kx];
                            gy_r += gray * gy[ky][kx];
                        }
                    }
                    
                    let magnitude = ((gx_r * gx_r + gy_r * gy_r) as f32).sqrt() as u8;
                    let idx = (y * width + x) * 4;
                    
                    new_data[idx] = magnitude;
                    new_data[idx + 1] = magnitude;
                    new_data[idx + 2] = magnitude;
                    new_data[idx + 3] = 255; // Alpha
                }
            }
            
            self.data = new_data;
            Ok(())
        }
    }

    // 高级数学计算器
    #[wasm_bindgen]
    pub struct MathCalculator {
        memory: HashMap<String, f64>,
    }

    #[wasm_bindgen]
    impl MathCalculator {
        #[wasm_bindgen(constructor)]
        pub fn new() -> MathCalculator {
            MathCalculator {
                memory: HashMap::new(),
            }
        }

        #[wasm_bindgen]
        pub fn store(&mut self, key: &str, value: f64) {
            self.memory.insert(key.to_string(), value);
        }

        #[wasm_bindgen]
        pub fn recall(&self, key: &str) -> Option<f64> {
            self.memory.get(key).copied()
        }

        #[wasm_bindgen]
        pub fn matrix_multiply(
            &self,
            a: &[f64],
            b: &[f64],
            rows_a: usize,
            cols_a: usize,
            cols_b: usize,
        ) -> Vec<f64> {
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
    }

    // 网络请求处理器
    #[wasm_bindgen]
    pub struct NetworkManager {
        base_url: String,
    }

    #[wasm_bindgen]
    impl NetworkManager {
        #[wasm_bindgen(constructor)]
        pub fn new(base_url: &str) -> NetworkManager {
            NetworkManager {
                base_url: base_url.to_string(),
            }
        }

        #[wasm_bindgen]
        pub async fn fetch_data(&self, endpoint: &str) -> Result<JsValue, JsValue> {
            let url = format!("{}/{}", self.base_url, endpoint);
            
            let init = RequestInit::new();
            init.set_method("GET");
            init.set_mode(RequestMode::Cors);
            let request = Request::new_with_str_and_init(&url, &init)
                .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;
            
            let window = web_sys::window().unwrap();
            let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|e| JsValue::from_str(&format!("Request failed: {:?}", e)))?;
            
            let response = response.dyn_into::<Response>()
                .map_err(|_| JsValue::from_str("Invalid response type"))?;
            
            if !response.ok() {
                return Err(JsValue::from_str("HTTP error"));
            }
            
            let json_promise = response.json()
                .map_err(|e| JsValue::from_str(&format!("Failed to get JSON promise: {:?}", e)))?;
            let json = wasm_bindgen_futures::JsFuture::from(json_promise)
                .await
                .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {:?}", e)))?;
            
            Ok(json)
        }

        #[wasm_bindgen]
        pub async fn post_data(&self, endpoint: &str, data: &str) -> Result<JsValue, JsValue> {
            let url = format!("{}/{}", self.base_url, endpoint);
            
            let headers = Headers::new().unwrap();
            headers.set("Content-Type", "application/json").unwrap();
            
            let init = RequestInit::new();
            init.set_method("POST");
            init.set_headers(&headers);
            let body = JsValue::from_str(data);
            init.set_body(&body);
            init.set_mode(RequestMode::Cors);
            
            let request = Request::new_with_str_and_init(&url, &init)
                .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;
            
            let window = web_sys::window().unwrap();
            let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|e| JsValue::from_str(&format!("Request failed: {:?}", e)))?;
            
            let response = response.dyn_into::<Response>()
                .map_err(|_| JsValue::from_str("Invalid response type"))?;
            
            if !response.ok() {
                return Err(JsValue::from_str("HTTP error"));
            }
            
            let json_promise = response.json()
                .map_err(|e| JsValue::from_str(&format!("Failed to get JSON promise: {:?}", e)))?;
            let json = wasm_bindgen_futures::JsFuture::from(json_promise)
                .await
                .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {:?}", e)))?;
            
            Ok(json)
        }
    }

    // WebAssembly运行时管理器
    #[wasm_bindgen]
    pub struct WasmRuntimeManager {
        runtime: WasmRuntime,
        modules: HashMap<String, Vec<u8>>,
    }

    #[wasm_bindgen]
    impl WasmRuntimeManager {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Result<WasmRuntimeManager, JsValue> {
            let runtime = WasmRuntime::new()
                .map_err(|e| JsValue::from_str(&format!("Failed to create runtime: {:?}", e)))?;
            
            Ok(WasmRuntimeManager {
                runtime,
                modules: HashMap::new(),
            })
        }

        #[wasm_bindgen]
        pub fn load_module(&mut self, name: &str, wasm_bytes: &[u8]) -> Result<(), JsValue> {
            self.runtime
                .load_module(name, wasm_bytes)
                .map_err(|e| JsValue::from_str(&format!("Failed to load module: {:?}", e)))?;
            
            self.modules.insert(name.to_string(), wasm_bytes.to_vec());
            Ok(())
        }

        #[wasm_bindgen]
        pub fn call_function(&self, module_name: &str, function_name: &str, args: &[f64]) -> Result<f64, JsValue> {
            self.runtime
                .call_function(module_name, function_name, args)
                .map_err(|e| JsValue::from_str(&format!("Failed to call function: {:?}", e)))
        }

        #[wasm_bindgen]
        pub fn get_module_info(&self, name: &str) -> Result<JsValue, JsValue> {
            let config = WasmModuleConfig::new(name, "1.0.0");
            serde_wasm_bindgen::to_value(&config)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
        }
    }

    // 初始化函数
    #[wasm_bindgen(start)]
    pub fn main() {
        console_error_panic_hook::set_once();
        log!("WebAssembly 2.0 + Rust 1.90 Advanced Example loaded!");
    }

    // 导出一个清理函数
    #[wasm_bindgen]
    pub fn cleanup() {
        log!("Cleaning up advanced WebAssembly module...");
    }
}
