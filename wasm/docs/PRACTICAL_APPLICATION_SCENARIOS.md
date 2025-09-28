# WebAssembly 2.0 + Rust 1.90 实际应用场景指南

## 📚 概述

本指南提供了 WebAssembly 2.0 + Rust 1.90 在实际项目中的应用场景，包括具体的实现方案、最佳实践和性能优化策略。

## 🎯 应用场景分类

### 1. 前端应用场景
- 高性能图像处理
- 实时数据可视化
- 游戏引擎
- 音视频处理

### 2. 后端应用场景
- 微服务架构
- 边缘计算
- 数据处理管道
- API 网关

### 3. 跨平台应用场景
- 桌面应用
- 移动应用
- 嵌入式系统
- IoT 设备

## 🖼️ 场景一：高性能图像处理

### 应用背景
在 Web 应用中实现实时图像处理，包括滤镜、缩放、格式转换等操作。

### 技术方案

```rust
// 图像处理引擎
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

#[wasm_bindgen]
pub struct ImageProcessor {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl ImageProcessor {
    /// 创建图像处理器
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }
    
    /// 从 ImageData 加载图像
    pub fn from_image_data(image_data: &ImageData) -> Self {
        let width = image_data.width();
        let height = image_data.height();
        let data = image_data.data().to_vec();
        
        Self { width, height, data }
    }
    
    /// 应用灰度滤镜
    pub fn apply_grayscale_filter(&mut self) {
        for i in (0..self.data.len()).step_by(4) {
            let r = self.data[i] as f32;
            let g = self.data[i + 1] as f32;
            let b = self.data[i + 2] as f32;
            
            // 使用标准灰度转换公式
            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            
            self.data[i] = gray;     // R
            self.data[i + 1] = gray; // G
            self.data[i + 2] = gray; // B
            // Alpha 通道保持不变
        }
    }
    
    /// 应用模糊滤镜
    pub fn apply_blur_filter(&mut self, radius: u32) {
        let mut temp_data = self.data.clone();
        let radius = radius as i32;
        
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut count = 0u32;
                
                // 计算邻域平均值
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let nx = x + dx;
                        let ny = y + dy;
                        
                        if nx >= 0 && nx < self.width as i32 && 
                           ny >= 0 && ny < self.height as i32 {
                            let idx = ((ny * self.width as i32 + nx) * 4) as usize;
                            r_sum += self.data[idx] as u32;
                            g_sum += self.data[idx + 1] as u32;
                            b_sum += self.data[idx + 2] as u32;
                            count += 1;
                        }
                    }
                }
                
                let idx = ((y * self.width as i32 + x) * 4) as usize;
                temp_data[idx] = (r_sum / count) as u8;
                temp_data[idx + 1] = (g_sum / count) as u8;
                temp_data[idx + 2] = (b_sum / count) as u8;
            }
        }
        
        self.data = temp_data;
    }
    
    /// 调整图像大小
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let mut new_data = vec![0; (new_width * new_height * 4) as usize];
        
        let x_ratio = self.width as f32 / new_width as f32;
        let y_ratio = self.height as f32 / new_height as f32;
        
        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio) as u32;
                let src_y = (y as f32 * y_ratio) as u32;
                
                let src_idx = ((src_y * self.width + src_x) * 4) as usize;
                let dst_idx = ((y * new_width + x) * 4) as usize;
                
                if src_idx + 3 < self.data.len() && dst_idx + 3 < new_data.len() {
                    new_data[dst_idx] = self.data[src_idx];
                    new_data[dst_idx + 1] = self.data[src_idx + 1];
                    new_data[dst_idx + 2] = self.data[src_idx + 2];
                    new_data[dst_idx + 3] = self.data[src_idx + 3];
                }
            }
        }
        
        self.width = new_width;
        self.height = new_height;
        self.data = new_data;
    }
    
    /// 转换为 ImageData
    pub fn to_image_data(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&self.data),
            self.width,
            self.height,
        ).unwrap()
    }
    
    /// 获取图像数据
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
```

### 性能优化

```rust
/// 使用 SIMD 优化的图像处理
impl ImageProcessor {
    /// SIMD 优化的灰度转换
    pub fn apply_grayscale_filter_simd(&mut self) {
        use std::arch::wasm32::*;
        
        // 使用 SIMD 指令进行批量处理
        for chunk in self.data.chunks_exact_mut(16) {
            if chunk.len() == 16 {
                unsafe {
                    let mut rgba = f32x4_load(chunk.as_ptr() as *const f32);
                    
                    // 应用灰度转换权重
                    let weights = f32x4(0.299, 0.587, 0.114, 0.0);
                    let gray = f32x4_mul(rgba, weights);
                    
                    // 水平求和
                    let sum = f32x4_extract_lane::<0>(gray) +
                             f32x4_extract_lane::<1>(gray) +
                             f32x4_extract_lane::<2>(gray);
                    
                    // 设置所有通道为灰度值
                    let gray_vec = f32x4_splat(sum);
                    f32x4_store(chunk.as_mut_ptr() as *mut f32, gray_vec);
                }
            }
        }
    }
}
```

### 使用示例

```javascript
// JavaScript 端使用
import { ImageProcessor } from './pkg/image_processor.js';

// 创建图像处理器
const processor = new ImageProcessor(800, 600);

// 从 Canvas 加载图像
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const imageData = ctx.getImageData(0, 0, 800, 600);

// 处理图像
processor.from_image_data(imageData);
processor.apply_grayscale_filter();
processor.apply_blur_filter(3);
processor.resize(400, 300);

// 显示结果
const resultData = processor.to_image_data();
ctx.putImageData(resultData, 0, 0);
```

## 📊 场景二：实时数据可视化

### 应用背景
在 Web 应用中实现高性能的实时数据图表和可视化组件。

### 技术方案

```rust
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

#[wasm_bindgen]
pub struct DataVisualizer {
    data_points: VecDeque<DataPoint>,
    max_points: usize,
    width: u32,
    height: u32,
}

#[derive(Clone)]
struct DataPoint {
    timestamp: f64,
    value: f64,
    category: String,
}

#[wasm_bindgen]
impl DataVisualizer {
    /// 创建数据可视化器
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, max_points: usize) -> Self {
        Self {
            data_points: VecDeque::with_capacity(max_points),
            max_points,
            width,
            height,
        }
    }
    
    /// 添加数据点
    pub fn add_data_point(&mut self, timestamp: f64, value: f64, category: &str) {
        let data_point = DataPoint {
            timestamp,
            value,
            category: category.to_string(),
        };
        
        self.data_points.push_back(data_point);
        
        // 保持最大点数限制
        if self.data_points.len() > self.max_points {
            self.data_points.pop_front();
        }
    }
    
    /// 批量添加数据点
    pub fn add_batch_data_points(&mut self, timestamps: &[f64], values: &[f64], categories: &[String]) {
        for ((&timestamp, &value), category) in timestamps.iter().zip(values.iter()).zip(categories.iter()) {
            self.add_data_point(timestamp, value, category);
        }
    }
    
    /// 计算统计信息
    pub fn calculate_statistics(&self) -> Statistics {
        if self.data_points.is_empty() {
            return Statistics::default();
        }
        
        let values: Vec<f64> = self.data_points.iter().map(|p| p.value).collect();
        let sum: f64 = values.iter().sum();
        let count = values.len() as f64;
        let mean = sum / count;
        
        let variance: f64 = values.iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / count;
        let std_dev = variance.sqrt();
        
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        Statistics {
            count: values.len(),
            mean,
            std_dev,
            min,
            max,
            sum,
        }
    }
    
    /// 生成折线图数据
    pub fn generate_line_chart_data(&self) -> LineChartData {
        let mut points = Vec::new();
        
        for (i, data_point) in self.data_points.iter().enumerate() {
            let x = (i as f64 / self.data_points.len() as f64) * self.width as f64;
            let y = self.height as f64 - (data_point.value / 100.0) * self.height as f64;
            
            points.push(Point { x, y });
        }
        
        LineChartData { points }
    }
    
    /// 生成柱状图数据
    pub fn generate_bar_chart_data(&self, num_bins: usize) -> BarChartData {
        let stats = self.calculate_statistics();
        let bin_width = (stats.max - stats.min) / num_bins as f64;
        let mut bins = vec![0; num_bins];
        
        for data_point in &self.data_points {
            let bin_index = ((data_point.value - stats.min) / bin_width) as usize;
            let bin_index = bin_index.min(num_bins - 1);
            bins[bin_index] += 1;
        }
        
        let bars: Vec<Bar> = bins.iter().enumerate().map(|(i, &count)| {
            let x = (i as f64 / num_bins as f64) * self.width as f64;
            let height = (count as f64 / self.data_points.len() as f64) * self.height as f64;
            
            Bar {
                x,
                width: self.width as f64 / num_bins as f64,
                height,
                count,
            }
        }).collect();
        
        BarChartData { bars }
    }
    
    /// 数据平滑处理
    pub fn smooth_data(&mut self, window_size: usize) {
        if self.data_points.len() < window_size {
            return;
        }
        
        let mut smoothed_points = VecDeque::new();
        let window = window_size as f64;
        
        for i in 0..self.data_points.len() {
            let start = i.saturating_sub(window_size / 2);
            let end = (i + window_size / 2).min(self.data_points.len());
            
            let window_values: Vec<f64> = self.data_points.iter()
                .skip(start)
                .take(end - start)
                .map(|p| p.value)
                .collect();
            
            let smoothed_value = window_values.iter().sum::<f64>() / window_values.len() as f64;
            
            if let Some(original_point) = self.data_points.get(i) {
                smoothed_points.push_back(DataPoint {
                    timestamp: original_point.timestamp,
                    value: smoothed_value,
                    category: original_point.category.clone(),
                });
            }
        }
        
        self.data_points = smoothed_points;
    }
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub count: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub sum: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            std_dev: 0.0,
            min: 0.0,
            max: 0.0,
            sum: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineChartData {
    pub points: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct BarChartData {
    pub bars: Vec<Bar>,
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub x: f64,
    pub width: f64,
    pub height: f64,
    pub count: usize,
}
```

### 使用示例

```javascript
// JavaScript 端使用
import { DataVisualizer } from './pkg/data_visualizer.js';

// 创建数据可视化器
const visualizer = new DataVisualizer(800, 600, 1000);

// 模拟实时数据
setInterval(() => {
    const timestamp = Date.now();
    const value = Math.random() * 100;
    const category = 'sensor1';
    
    visualizer.add_data_point(timestamp, value, category);
    
    // 更新图表
    updateChart(visualizer);
}, 100);

function updateChart(visualizer) {
    // 获取统计信息
    const stats = visualizer.calculate_statistics();
    console.log('统计信息:', stats);
    
    // 生成图表数据
    const lineData = visualizer.generate_line_chart_data();
    const barData = visualizer.generate_bar_chart_data(20);
    
    // 更新 Canvas 显示
    drawLineChart(lineData);
    drawBarChart(barData);
}
```

## 🎮 场景三：游戏引擎

### 应用背景
在 Web 浏览器中实现高性能的 2D/3D 游戏引擎。

### 技术方案

```rust
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct GameEngine {
    entities: HashMap<u32, Entity>,
    next_entity_id: u32,
    physics_world: PhysicsWorld,
    renderer: Renderer,
    input_handler: InputHandler,
}

#[derive(Clone)]
struct Entity {
    id: u32,
    position: Vector2,
    velocity: Vector2,
    rotation: f32,
    scale: f32,
    sprite: Option<Sprite>,
    collider: Option<Collider>,
}

#[derive(Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Sprite {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[derive(Clone)]
struct Collider {
    width: f32,
    height: f32,
    is_trigger: bool,
}

#[wasm_bindgen]
impl GameEngine {
    /// 创建游戏引擎
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            entities: HashMap::new(),
            next_entity_id: 0,
            physics_world: PhysicsWorld::new(),
            renderer: Renderer::new(width, height),
            input_handler: InputHandler::new(),
        }
    }
    
    /// 创建实体
    pub fn create_entity(&mut self, x: f32, y: f32) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        
        let entity = Entity {
            id,
            position: Vector2 { x, y },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: 1.0,
            sprite: None,
            collider: None,
        };
        
        self.entities.insert(id, entity);
        id
    }
    
    /// 设置实体精灵
    pub fn set_entity_sprite(&mut self, entity_id: u32, width: u32, height: u32, data: &[u8]) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.sprite = Some(Sprite {
                width,
                height,
                data: data.to_vec(),
            });
        }
    }
    
    /// 设置实体碰撞器
    pub fn set_entity_collider(&mut self, entity_id: u32, width: f32, height: f32, is_trigger: bool) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.collider = Some(Collider {
                width,
                height,
                is_trigger,
            });
        }
    }
    
    /// 设置实体速度
    pub fn set_entity_velocity(&mut self, entity_id: u32, x: f32, y: f32) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.velocity = Vector2 { x, y };
        }
    }
    
    /// 更新游戏状态
    pub fn update(&mut self, delta_time: f32) {
        // 更新物理
        self.physics_world.update(&mut self.entities, delta_time);
        
        // 处理输入
        self.input_handler.update();
        
        // 更新实体
        for entity in self.entities.values_mut() {
            // 更新位置
            entity.position.x += entity.velocity.x * delta_time;
            entity.position.y += entity.velocity.y * delta_time;
            
            // 边界检查
            if entity.position.x < 0.0 || entity.position.x > self.renderer.width as f32 {
                entity.velocity.x *= -1.0;
            }
            if entity.position.y < 0.0 || entity.position.y > self.renderer.height as f32 {
                entity.velocity.y *= -1.0;
            }
        }
    }
    
    /// 渲染游戏
    pub fn render(&mut self) -> Vec<u8> {
        self.renderer.clear();
        
        for entity in self.entities.values() {
            if let Some(sprite) = &entity.sprite {
                self.renderer.draw_sprite(
                    &entity.position,
                    entity.rotation,
                    entity.scale,
                    sprite,
                );
            }
        }
        
        self.renderer.get_pixel_data()
    }
    
    /// 处理键盘输入
    pub fn handle_key_down(&mut self, key: &str) {
        self.input_handler.handle_key_down(key);
    }
    
    /// 处理键盘释放
    pub fn handle_key_up(&mut self, key: &str) {
        self.input_handler.handle_key_up(key);
    }
    
    /// 获取实体数量
    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }
}

struct PhysicsWorld {
    gravity: Vector2,
}

impl PhysicsWorld {
    fn new() -> Self {
        Self {
            gravity: Vector2 { x: 0.0, y: 9.8 },
        }
    }
    
    fn update(&self, entities: &mut HashMap<u32, Entity>, delta_time: f32) {
        for entity in entities.values_mut() {
            // 应用重力
            entity.velocity.y += self.gravity.y * delta_time;
            
            // 应用摩擦力
            entity.velocity.x *= 0.99;
            entity.velocity.y *= 0.99;
        }
    }
}

struct Renderer {
    width: u32,
    height: u32,
    pixel_data: Vec<u8>,
}

impl Renderer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixel_data: vec![0; (width * height * 4) as usize],
        }
    }
    
    fn clear(&mut self) {
        self.pixel_data.fill(0);
    }
    
    fn draw_sprite(&mut self, position: &Vector2, rotation: f32, scale: f32, sprite: &Sprite) {
        // 简化的精灵渲染实现
        let start_x = (position.x * scale) as i32;
        let start_y = (position.y * scale) as i32;
        
        for y in 0..sprite.height {
            for x in 0..sprite.width {
                let pixel_x = start_x + x as i32;
                let pixel_y = start_y + y as i32;
                
                if pixel_x >= 0 && pixel_x < self.width as i32 &&
                   pixel_y >= 0 && pixel_y < self.height as i32 {
                    let sprite_idx = ((y * sprite.width + x) * 4) as usize;
                    let pixel_idx = ((pixel_y * self.width as i32 + pixel_x) * 4) as usize;
                    
                    if sprite_idx + 3 < sprite.data.len() && pixel_idx + 3 < self.pixel_data.len() {
                        self.pixel_data[pixel_idx] = sprite.data[sprite_idx];
                        self.pixel_data[pixel_idx + 1] = sprite.data[sprite_idx + 1];
                        self.pixel_data[pixel_idx + 2] = sprite.data[sprite_idx + 2];
                        self.pixel_data[pixel_idx + 3] = sprite.data[sprite_idx + 3];
                    }
                }
            }
        }
    }
    
    fn get_pixel_data(&self) -> Vec<u8> {
        self.pixel_data.clone()
    }
}

struct InputHandler {
    pressed_keys: std::collections::HashSet<String>,
}

impl InputHandler {
    fn new() -> Self {
        Self {
            pressed_keys: std::collections::HashSet::new(),
        }
    }
    
    fn update(&mut self) {
        // 处理输入状态更新
    }
    
    fn handle_key_down(&mut self, key: &str) {
        self.pressed_keys.insert(key.to_string());
    }
    
    fn handle_key_up(&mut self, key: &str) {
        self.pressed_keys.remove(key);
    }
    
    fn is_key_pressed(&self, key: &str) -> bool {
        self.pressed_keys.contains(key)
    }
}
```

### 使用示例

```javascript
// JavaScript 端使用
import { GameEngine } from './pkg/game_engine.js';

// 创建游戏引擎
const gameEngine = new GameEngine(800, 600);

// 创建游戏对象
const player = gameEngine.create_entity(100, 100);
gameEngine.set_entity_sprite(player, 32, 32, playerSpriteData);
gameEngine.set_entity_collider(player, 32, 32, false);

// 游戏循环
function gameLoop() {
    const deltaTime = 1/60; // 60 FPS
    
    // 处理输入
    if (keys['ArrowLeft']) {
        gameEngine.set_entity_velocity(player, -100, 0);
    }
    if (keys['ArrowRight']) {
        gameEngine.set_entity_velocity(player, 100, 0);
    }
    if (keys['ArrowUp']) {
        gameEngine.set_entity_velocity(player, 0, -100);
    }
    if (keys['ArrowDown']) {
        gameEngine.set_entity_velocity(player, 0, 100);
    }
    
    // 更新游戏
    gameEngine.update(deltaTime);
    
    // 渲染
    const pixelData = gameEngine.render();
    updateCanvas(pixelData);
    
    requestAnimationFrame(gameLoop);
}

// 启动游戏
gameLoop();
```

## 🔧 场景四：微服务架构

### 应用背景
在微服务架构中使用 WebAssembly 实现高性能的数据处理服务。

### 技术方案

```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: String,
    pub operation: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceResponse {
    pub id: String,
    pub status: String,
    pub data: serde_json::Value,
    pub processing_time: u64,
}

#[wasm_bindgen]
pub struct MicroService {
    service_id: String,
    handlers: HashMap<String, Box<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>>,
    metrics: ServiceMetrics,
}

struct ServiceMetrics {
    request_count: u64,
    total_processing_time: u64,
    error_count: u64,
}

#[wasm_bindgen]
impl MicroService {
    /// 创建微服务
    #[wasm_bindgen(constructor)]
    pub fn new(service_id: &str) -> Self {
        let mut service = Self {
            service_id: service_id.to_string(),
            handlers: HashMap::new(),
            metrics: ServiceMetrics {
                request_count: 0,
                total_processing_time: 0,
                error_count: 0,
            },
        };
        
        // 注册默认处理器
        service.register_default_handlers();
        service
    }
    
    /// 处理请求
    pub fn process_request(&mut self, request_json: &str) -> String {
        let start_time = std::time::Instant::now();
        
        match serde_json::from_str::<ServiceRequest>(request_json) {
            Ok(request) => {
                self.metrics.request_count += 1;
                
                match self.handle_request(&request) {
                    Ok(response) => {
                        let processing_time = start_time.elapsed().as_millis() as u64;
                        self.metrics.total_processing_time += processing_time;
                        
                        let response = ServiceResponse {
                            id: request.id,
                            status: "success".to_string(),
                            data: response,
                            processing_time,
                        };
                        
                        serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string())
                    }
                    Err(error) => {
                        self.metrics.error_count += 1;
                        
                        let response = ServiceResponse {
                            id: request.id,
                            status: "error".to_string(),
                            data: serde_json::json!({ "error": error }),
                            processing_time: start_time.elapsed().as_millis() as u64,
                        };
                        
                        serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string())
                    }
                }
            }
            Err(error) => {
                let response = ServiceResponse {
                    id: "unknown".to_string(),
                    status: "error".to_string(),
                    data: serde_json::json!({ "error": error.to_string() }),
                    processing_time: start_time.elapsed().as_millis() as u64,
                };
                
                serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string())
            }
        }
    }
    
    /// 注册处理器
    pub fn register_handler(&mut self, operation: &str, handler: Box<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>) {
        self.handlers.insert(operation.to_string(), handler);
    }
    
    /// 获取服务指标
    pub fn get_metrics(&self) -> String {
        let avg_processing_time = if self.metrics.request_count > 0 {
            self.metrics.total_processing_time / self.metrics.request_count
        } else {
            0
        };
        
        let metrics = serde_json::json!({
            "service_id": self.service_id,
            "request_count": self.metrics.request_count,
            "error_count": self.metrics.error_count,
            "average_processing_time": avg_processing_time,
            "error_rate": if self.metrics.request_count > 0 {
                self.metrics.error_count as f64 / self.metrics.request_count as f64
            } else {
                0.0
            }
        });
        
        serde_json::to_string(&metrics).unwrap_or_else(|_| "{}".to_string())
    }
    
    /// 处理请求
    fn handle_request(&self, request: &ServiceRequest) -> Result<serde_json::Value, String> {
        if let Some(handler) = self.handlers.get(&request.operation) {
            handler(&request.data)
        } else {
            Err(format!("Unknown operation: {}", request.operation))
        }
    }
    
    /// 注册默认处理器
    fn register_default_handlers(&mut self) {
        // 数据处理处理器
        self.handlers.insert("process_data".to_string(), Box::new(|data| {
            if let Some(array) = data.as_array() {
                let processed: Vec<f64> = array.iter()
                    .filter_map(|v| v.as_f64())
                    .map(|x| x * 2.0)
                    .collect();
                Ok(serde_json::json!(processed))
            } else {
                Err("Invalid data format".to_string())
            }
        }));
        
        // 数学计算处理器
        self.handlers.insert("calculate".to_string(), Box::new(|data| {
            if let Some(expression) = data.get("expression").and_then(|v| v.as_str()) {
                match evaluate_expression(expression) {
                    Ok(result) => Ok(serde_json::json!({ "result": result })),
                    Err(error) => Err(error),
                }
            } else {
                Err("Missing expression".to_string())
            }
        }));
        
        // 数据验证处理器
        self.handlers.insert("validate".to_string(), Box::new(|data| {
            let validation_result = validate_data(data);
            Ok(serde_json::json!(validation_result))
        }));
    }
}

/// 表达式求值器
fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // 简化的表达式求值实现
    if expression.contains('+') {
        let parts: Vec<&str> = expression.split('+').collect();
        if parts.len() == 2 {
            let a: f64 = parts[0].trim().parse().map_err(|_| "Invalid number")?;
            let b: f64 = parts[1].trim().parse().map_err(|_| "Invalid number")?;
            return Ok(a + b);
        }
    }
    
    if expression.contains('-') {
        let parts: Vec<&str> = expression.split('-').collect();
        if parts.len() == 2 {
            let a: f64 = parts[0].trim().parse().map_err(|_| "Invalid number")?;
            let b: f64 = parts[1].trim().parse().map_err(|_| "Invalid number")?;
            return Ok(a - b);
        }
    }
    
    if expression.contains('*') {
        let parts: Vec<&str> = expression.split('*').collect();
        if parts.len() == 2 {
            let a: f64 = parts[0].trim().parse().map_err(|_| "Invalid number")?;
            let b: f64 = parts[1].trim().parse().map_err(|_| "Invalid number")?;
            return Ok(a * b);
        }
    }
    
    if expression.contains('/') {
        let parts: Vec<&str> = expression.split('/').collect();
        if parts.len() == 2 {
            let a: f64 = parts[0].trim().parse().map_err(|_| "Invalid number")?;
            let b: f64 = parts[1].trim().parse().map_err(|_| "Invalid number")?;
            if b == 0.0 {
                return Err("Division by zero".to_string());
            }
            return Ok(a / b);
        }
    }
    
    // 尝试直接解析数字
    expression.trim().parse().map_err(|_| "Invalid expression".to_string())
}

/// 数据验证器
fn validate_data(data: &serde_json::Value) -> serde_json::Value {
    let mut validation_result = serde_json::json!({
        "valid": true,
        "errors": []
    });
    
    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            match key.as_str() {
                "email" => {
                    if let Some(email) = value.as_str() {
                        if !email.contains('@') {
                            validation_result["valid"] = serde_json::Value::Bool(false);
                            validation_result["errors"].as_array_mut().unwrap().push(
                                serde_json::json!({ "field": "email", "message": "Invalid email format" })
                            );
                        }
                    }
                }
                "age" => {
                    if let Some(age) = value.as_i64() {
                        if age < 0 || age > 150 {
                            validation_result["valid"] = serde_json::Value::Bool(false);
                            validation_result["errors"].as_array_mut().unwrap().push(
                                serde_json::json!({ "field": "age", "message": "Age must be between 0 and 150" })
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    validation_result
}
```

### 使用示例

```javascript
// JavaScript 端使用
import { MicroService } from './pkg/micro_service.js';

// 创建微服务
const dataService = new MicroService('data-processor');

// 处理数据请求
const request = {
    id: 'req-001',
    operation: 'process_data',
    data: [1, 2, 3, 4, 5],
    timestamp: Date.now()
};

const response = dataService.process_request(JSON.stringify(request));
console.log('Response:', JSON.parse(response));

// 处理计算请求
const calcRequest = {
    id: 'req-002',
    operation: 'calculate',
    data: { expression: '10 + 20 * 2' },
    timestamp: Date.now()
};

const calcResponse = dataService.process_request(JSON.stringify(calcRequest));
console.log('Calculation Response:', JSON.parse(calcResponse));

// 获取服务指标
const metrics = dataService.get_metrics();
console.log('Service Metrics:', JSON.parse(metrics));
```

## 📋 最佳实践总结

### 1. 性能优化
- 使用 SIMD 指令进行批量处理
- 实现内存池减少分配开销
- 使用无锁数据结构提高并发性能
- 优化数据结构和算法

### 2. 错误处理
- 实现完善的错误处理机制
- 使用 Result 类型处理可能失败的操作
- 提供详细的错误信息
- 实现优雅的错误恢复

### 3. 内存管理
- 使用 RAII 模式管理资源
- 避免内存泄漏
- 优化内存使用模式
- 实现高效的数据结构

### 4. 并发处理
- 使用线程安全的数据结构
- 实现工作窃取调度
- 避免数据竞争
- 优化锁的使用

### 5. 测试和调试
- 编写全面的单元测试
- 实现性能基准测试
- 使用调试工具
- 监控运行时性能

---

**注意**: 这些应用场景展示了 WebAssembly 2.0 + Rust 1.90 在实际项目中的强大能力。建议根据具体需求选择合适的实现方案，并遵循最佳实践进行开发。
