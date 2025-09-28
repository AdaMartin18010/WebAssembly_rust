//! # 性能监控模块 / Performance Monitoring Module
//!
//! 本模块提供了统一的性能监控和统计功能。
//! This module provides unified performance monitoring and statistics functionality.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// 性能统计 / Performance Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    /// 总执行时间 / Total execution time
    pub total_execution_time: Duration,
    /// 执行次数 / Execution count
    pub execution_count: u64,
    /// 平均执行时间 / Average execution time
    pub average_execution_time: Duration,
    /// 最大执行时间 / Maximum execution time
    pub max_execution_time: Duration,
    /// 最小执行时间 / Minimum execution time
    pub min_execution_time: Duration,
    /// 内存使用峰值 / Peak memory usage
    pub peak_memory_usage: u64,
    /// 当前内存使用 / Current memory usage
    pub current_memory_usage: u64,
    /// 缓存命中率 / Cache hit rate
    pub cache_hit_rate: f64,
    /// 错误率 / Error rate
    pub error_rate: f64,
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            total_execution_time: Duration::ZERO,
            execution_count: 0,
            average_execution_time: Duration::ZERO,
            max_execution_time: Duration::ZERO,
            min_execution_time: Duration::MAX,
            peak_memory_usage: 0,
            current_memory_usage: 0,
            cache_hit_rate: 0.0,
            error_rate: 0.0,
        }
    }
}

impl PerformanceStats {
    /// 创建新的性能统计 / Create new performance statistics
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 记录执行时间 / Record execution time
    pub fn record_execution(&mut self, execution_time: Duration) {
        self.total_execution_time += execution_time;
        self.execution_count += 1;
        
        // 计算平均执行时间
        if self.execution_count > 0 {
            let total_millis = self.total_execution_time.as_millis() as u64;
            self.average_execution_time = Duration::from_millis(total_millis / self.execution_count);
        }
        
        // 更新最大和最小执行时间
        if execution_time > self.max_execution_time {
            self.max_execution_time = execution_time;
        }
        
        if execution_time < self.min_execution_time {
            self.min_execution_time = execution_time;
        }
    }
    
    /// 更新内存使用 / Update memory usage
    pub fn update_memory_usage(&mut self, memory_usage: u64) {
        self.current_memory_usage = memory_usage;
        if memory_usage > self.peak_memory_usage {
            self.peak_memory_usage = memory_usage;
        }
    }
    
    /// 更新缓存命中率 / Update cache hit rate
    pub fn update_cache_hit_rate(&mut self, hits: u64, total: u64) {
        if total > 0 {
            self.cache_hit_rate = hits as f64 / total as f64;
        }
    }
    
    /// 更新错误率 / Update error rate
    pub fn update_error_rate(&mut self, errors: u64, total: u64) {
        if total > 0 {
            self.error_rate = errors as f64 / total as f64;
        }
    }
    
    /// 重置统计 / Reset statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    /// 获取性能摘要 / Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            total_executions: self.execution_count,
            average_execution_time: self.average_execution_time,
            peak_memory_usage: self.peak_memory_usage,
            cache_hit_rate: self.cache_hit_rate,
            error_rate: self.error_rate,
        }
    }
}

/// 性能摘要 / Performance Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// 总执行次数 / Total executions
    pub total_executions: u64,
    /// 平均执行时间 / Average execution time
    pub average_execution_time: Duration,
    /// 内存使用峰值 / Peak memory usage
    pub peak_memory_usage: u64,
    /// 缓存命中率 / Cache hit rate
    pub cache_hit_rate: f64,
    /// 错误率 / Error rate
    pub error_rate: f64,
}

/// 性能监控器 / Performance Monitor
pub struct PerformanceMonitor {
    /// 统计信息 / Statistics
    stats: Arc<Mutex<PerformanceStats>>,
    /// 函数级统计 / Function-level statistics
    function_stats: Arc<Mutex<HashMap<String, PerformanceStats>>>,
    /// 模块级统计 / Module-level statistics
    module_stats: Arc<Mutex<HashMap<String, PerformanceStats>>>,
}

impl PerformanceMonitor {
    /// 创建新的性能监控器 / Create new performance monitor
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(PerformanceStats::new())),
            function_stats: Arc::new(Mutex::new(HashMap::new())),
            module_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 记录全局执行时间 / Record global execution time
    pub fn record_global_execution(&self, execution_time: Duration) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.record_execution(execution_time);
        }
    }
    
    /// 记录函数执行时间 / Record function execution time
    pub fn record_function_execution(&self, function_name: &str, execution_time: Duration) {
        if let Ok(mut function_stats) = self.function_stats.lock() {
            let stats = function_stats.entry(function_name.to_string()).or_insert_with(PerformanceStats::new);
            stats.record_execution(execution_time);
        }
    }
    
    /// 记录模块执行时间 / Record module execution time
    pub fn record_module_execution(&self, module_name: &str, execution_time: Duration) {
        if let Ok(mut module_stats) = self.module_stats.lock() {
            let stats = module_stats.entry(module_name.to_string()).or_insert_with(PerformanceStats::new);
            stats.record_execution(execution_time);
        }
    }
    
    /// 更新内存使用 / Update memory usage
    pub fn update_memory_usage(&self, memory_usage: u64) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.update_memory_usage(memory_usage);
        }
    }
    
    /// 获取全局统计 / Get global statistics
    pub fn get_global_stats(&self) -> PerformanceStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// 获取函数统计 / Get function statistics
    pub fn get_function_stats(&self, function_name: &str) -> Option<PerformanceStats> {
        self.function_stats.lock().unwrap().get(function_name).cloned()
    }
    
    /// 获取模块统计 / Get module statistics
    pub fn get_module_stats(&self, module_name: &str) -> Option<PerformanceStats> {
        self.module_stats.lock().unwrap().get(module_name).cloned()
    }
    
    /// 获取所有函数统计 / Get all function statistics
    pub fn get_all_function_stats(&self) -> HashMap<String, PerformanceStats> {
        self.function_stats.lock().unwrap().clone()
    }
    
    /// 获取所有模块统计 / Get all module statistics
    pub fn get_all_module_stats(&self) -> HashMap<String, PerformanceStats> {
        self.module_stats.lock().unwrap().clone()
    }
    
    /// 重置所有统计 / Reset all statistics
    pub fn reset_all(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.reset();
        }
        if let Ok(mut function_stats) = self.function_stats.lock() {
            function_stats.clear();
        }
        if let Ok(mut module_stats) = self.module_stats.lock() {
            module_stats.clear();
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 性能计时器 / Performance Timer
pub struct PerformanceTimer {
    start_time: Instant,
    name: String,
}

impl PerformanceTimer {
    /// 开始计时 / Start timing
    pub fn start(name: impl Into<String>) -> Self {
        Self {
            start_time: Instant::now(),
            name: name.into(),
        }
    }
    
    /// 结束计时并返回持续时间 / End timing and return duration
    pub fn end(self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// 获取当前经过的时间 / Get current elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// 获取计时器名称 / Get timer name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// 性能分析器 / Performance Analyzer
pub struct PerformanceAnalyzer {
    /// 性能数据 / Performance data
    data: Vec<PerformanceDataPoint>,
    /// 分析结果 / Analysis results
    results: Option<PerformanceAnalysis>,
}

/// 性能数据点 / Performance Data Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    /// 时间戳 / Timestamp
    pub timestamp: std::time::SystemTime,
    /// 执行时间 / Execution time
    pub execution_time: Duration,
    /// 内存使用 / Memory usage
    pub memory_usage: u64,
    /// 函数名称 / Function name
    pub function_name: String,
    /// 模块名称 / Module name
    pub module_name: String,
}

/// 性能分析结果 / Performance Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    /// 平均执行时间 / Average execution time
    pub average_execution_time: Duration,
    /// 执行时间标准差 / Execution time standard deviation
    pub execution_time_std_dev: f64,
    /// 内存使用趋势 / Memory usage trend
    pub memory_usage_trend: f64,
    /// 性能瓶颈 / Performance bottlenecks
    pub bottlenecks: Vec<String>,
    /// 优化建议 / Optimization suggestions
    pub suggestions: Vec<String>,
}

impl PerformanceAnalyzer {
    /// 创建新的性能分析器 / Create new performance analyzer
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            results: None,
        }
    }
    
    /// 添加性能数据点 / Add performance data point
    pub fn add_data_point(&mut self, data_point: PerformanceDataPoint) {
        self.data.push(data_point);
    }
    
    /// 分析性能数据 / Analyze performance data
    pub fn analyze(&mut self) -> &PerformanceAnalysis {
        let mut analysis = PerformanceAnalysis {
            average_execution_time: Duration::ZERO,
            execution_time_std_dev: 0.0,
            memory_usage_trend: 0.0,
            bottlenecks: Vec::new(),
            suggestions: Vec::new(),
        };
        
        if self.data.is_empty() {
            self.results = Some(analysis);
            return self.results.as_ref().unwrap();
        }
        
        // 计算平均执行时间
        let total_time: Duration = self.data.iter().map(|d| d.execution_time).sum();
        analysis.average_execution_time = Duration::from_millis(
            total_time.as_millis() as u64 / self.data.len() as u64
        );
        
        // 计算标准差
        let mean = analysis.average_execution_time.as_millis() as f64;
        let variance: f64 = self.data.iter()
            .map(|d| {
                let diff = d.execution_time.as_millis() as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / self.data.len() as f64;
        analysis.execution_time_std_dev = variance.sqrt();
        
        // 分析内存使用趋势
        if self.data.len() > 1 {
            let first_memory = self.data[0].memory_usage as f64;
            let last_memory = self.data.last().unwrap().memory_usage as f64;
            analysis.memory_usage_trend = (last_memory - first_memory) / first_memory;
        }
        
        // 识别性能瓶颈
        let slow_functions: Vec<_> = self.data.iter()
            .filter(|d| d.execution_time > analysis.average_execution_time * 2)
            .map(|d| d.function_name.clone())
            .collect();
        analysis.bottlenecks = slow_functions;
        
        // 生成优化建议
        if analysis.execution_time_std_dev > mean * 0.5 {
            analysis.suggestions.push("执行时间变化较大，建议检查算法复杂度".to_string());
        }
        if analysis.memory_usage_trend > 0.1 {
            analysis.suggestions.push("内存使用呈上升趋势，建议检查内存泄漏".to_string());
        }
        if !analysis.bottlenecks.is_empty() {
            analysis.suggestions.push("发现性能瓶颈，建议优化相关函数".to_string());
        }
        
        self.results = Some(analysis);
        self.results.as_ref().unwrap()
    }
    
    /// 获取分析结果 / Get analysis results
    pub fn get_results(&self) -> Option<&PerformanceAnalysis> {
        self.results.as_ref()
    }
    
    /// 清除数据 / Clear data
    pub fn clear(&mut self) {
        self.data.clear();
        self.results = None;
    }
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_stats() {
        let mut stats = PerformanceStats::new();
        stats.record_execution(Duration::from_millis(100));
        stats.record_execution(Duration::from_millis(200));
        
        assert_eq!(stats.execution_count, 2);
        assert_eq!(stats.average_execution_time, Duration::from_millis(150));
        assert_eq!(stats.max_execution_time, Duration::from_millis(200));
        assert_eq!(stats.min_execution_time, Duration::from_millis(100));
    }
    
    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start("test");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }
    
    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        monitor.record_global_execution(Duration::from_millis(100));
        monitor.record_function_execution("test_function", Duration::from_millis(50));
        
        let global_stats = monitor.get_global_stats();
        assert_eq!(global_stats.execution_count, 1);
        
        let function_stats = monitor.get_function_stats("test_function");
        assert!(function_stats.is_some());
        assert_eq!(function_stats.unwrap().execution_count, 1);
    }
}
