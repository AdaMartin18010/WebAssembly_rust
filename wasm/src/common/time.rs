//! # 时间处理模块 / Time Handling Module
//!
//! 本模块提供了统一的时间处理功能，包括时间戳、持续时间和时间格式化。
//! This module provides unified time handling functionality, including timestamps, durations, and time formatting.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local, TimeZone};

/// 时间戳类型 / Timestamp Type
///
/// 使用 chrono::DateTime<Utc> 作为统一的时间戳类型，支持序列化。
/// Uses chrono::DateTime<Utc> as the unified timestamp type with serialization support.
pub type Timestamp = DateTime<Utc>;

/// 本地时间戳类型 / Local Timestamp Type
pub type LocalTimestamp = DateTime<Local>;

/// 时间工具 / Time Utilities
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前UTC时间戳 / Get current UTC timestamp
    pub fn now() -> Timestamp {
        Utc::now()
    }
    
    /// 获取当前本地时间戳 / Get current local timestamp
    pub fn now_local() -> LocalTimestamp {
        Local::now()
    }
    
    /// 从系统时间创建时间戳 / Create timestamp from system time
    pub fn from_system_time(system_time: SystemTime) -> Timestamp {
        system_time.duration_since(UNIX_EPOCH)
            .map(|duration| Utc.timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()).single().unwrap_or_else(|| Utc::now()))
            .unwrap_or_else(|_| Utc::now())
    }
    
    /// 转换为系统时间 / Convert to system time
    pub fn to_system_time(timestamp: Timestamp) -> SystemTime {
        let duration = Duration::from_secs(timestamp.timestamp() as u64);
        UNIX_EPOCH + duration
    }
    
    /// 格式化时间戳 / Format timestamp
    pub fn format_timestamp(timestamp: Timestamp, format: &str) -> String {
        timestamp.format(format).to_string()
    }
    
    /// 格式化时间戳为ISO 8601格式 / Format timestamp as ISO 8601
    pub fn format_iso8601(timestamp: Timestamp) -> String {
        timestamp.to_rfc3339()
    }
    
    /// 解析ISO 8601时间戳 / Parse ISO 8601 timestamp
    pub fn parse_iso8601(s: &str) -> Result<Timestamp, chrono::ParseError> {
        DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
    }
    
    /// 计算时间差 / Calculate time difference
    pub fn duration_between(start: Timestamp, end: Timestamp) -> Duration {
        let diff = end.signed_duration_since(start);
        Duration::from_millis(diff.num_milliseconds().max(0) as u64)
    }
    
    /// 检查时间戳是否过期 / Check if timestamp is expired
    pub fn is_expired(timestamp: Timestamp, ttl: Duration) -> bool {
        let now = Utc::now();
        let expiry = timestamp + chrono::Duration::from_std(ttl).unwrap_or_default();
        now > expiry
    }
    
    /// 获取时间戳的年龄 / Get age of timestamp
    pub fn age(timestamp: Timestamp) -> Duration {
        Self::duration_between(timestamp, Utc::now())
    }
}

/// 时间范围 / Time Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// 开始时间 / Start time
    pub start: Timestamp,
    /// 结束时间 / End time
    pub end: Timestamp,
}

impl TimeRange {
    /// 创建新的时间范围 / Create new time range
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }
    
    /// 创建从指定时间开始的时间范围 / Create time range starting from specified time
    pub fn from_start(start: Timestamp, duration: Duration) -> Self {
        let end = start + chrono::Duration::from_std(duration).unwrap_or_default();
        Self::new(start, end)
    }
    
    /// 创建到指定时间结束的时间范围 / Create time range ending at specified time
    pub fn until_end(end: Timestamp, duration: Duration) -> Self {
        let start = end - chrono::Duration::from_std(duration).unwrap_or_default();
        Self::new(start, end)
    }
    
    /// 检查时间戳是否在范围内 / Check if timestamp is within range
    pub fn contains(&self, timestamp: Timestamp) -> bool {
        timestamp >= self.start && timestamp <= self.end
    }
    
    /// 获取时间范围的持续时间 / Get duration of time range
    pub fn duration(&self) -> Duration {
        TimeUtils::duration_between(self.start, self.end)
    }
    
    /// 检查时间范围是否重叠 / Check if time ranges overlap
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    
    /// 获取重叠部分 / Get overlap
    pub fn intersection(&self, other: &TimeRange) -> Option<TimeRange> {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            Some(TimeRange::new(start, end))
        } else {
            None
        }
    }
}

/// 时间窗口 / Time Window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// 窗口大小 / Window size
    pub size: Duration,
    /// 滑动步长 / Sliding step
    pub step: Duration,
    /// 当前窗口开始时间 / Current window start time
    pub current_start: Timestamp,
}

impl TimeWindow {
    /// 创建新的时间窗口 / Create new time window
    pub fn new(size: Duration, step: Duration) -> Self {
        Self {
            size,
            step,
            current_start: TimeUtils::now(),
        }
    }
    
    /// 创建固定时间窗口 / Create fixed time window
    pub fn fixed(size: Duration) -> Self {
        Self::new(size, size)
    }
    
    /// 创建滑动时间窗口 / Create sliding time window
    pub fn sliding(size: Duration, step: Duration) -> Self {
        Self::new(size, step)
    }
    
    /// 获取当前窗口 / Get current window
    pub fn current_window(&self) -> TimeRange {
        TimeRange::from_start(self.current_start, self.size)
    }
    
    /// 滑动到下一个窗口 / Slide to next window
    pub fn slide_next(&mut self) {
        self.current_start += chrono::Duration::from_std(self.step).unwrap_or_default();
    }
    
    /// 滑动到上一个窗口 / Slide to previous window
    pub fn slide_previous(&mut self) {
        self.current_start -= chrono::Duration::from_std(self.step).unwrap_or_default();
    }
    
    /// 重置到指定时间 / Reset to specified time
    pub fn reset_to(&mut self, timestamp: Timestamp) {
        self.current_start = timestamp;
    }
}

/// 时间序列数据点 / Time Series Data Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint<T> {
    /// 时间戳 / Timestamp
    pub timestamp: Timestamp,
    /// 数据值 / Data value
    pub value: T,
}

impl<T> TimeSeriesPoint<T> {
    /// 创建新的时间序列数据点 / Create new time series data point
    pub fn new(timestamp: Timestamp, value: T) -> Self {
        Self { timestamp, value }
    }
    
    /// 创建当前时间的数据点 / Create data point with current time
    pub fn now(value: T) -> Self {
        Self::new(TimeUtils::now(), value)
    }
}

/// 时间序列 / Time Series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries<T> {
    /// 数据点 / Data points
    pub points: Vec<TimeSeriesPoint<T>>,
    /// 最大数据点数量 / Maximum number of data points
    pub max_points: Option<usize>,
}

impl<T> TimeSeries<T> {
    /// 创建新的时间序列 / Create new time series
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            max_points: None,
        }
    }
    
    /// 创建带最大数据点限制的时间序列 / Create time series with max points limit
    pub fn with_max_points(max_points: usize) -> Self {
        Self {
            points: Vec::new(),
            max_points: Some(max_points),
        }
    }
    
    /// 添加数据点 / Add data point
    pub fn add_point(&mut self, point: TimeSeriesPoint<T>) {
        self.points.push(point);
        
        // 如果超过最大数据点数量，移除最旧的数据点
        if let Some(max_points) = self.max_points {
            if self.points.len() > max_points {
                self.points.remove(0);
            }
        }
    }
    
    /// 添加当前时间的数据点 / Add data point with current time
    pub fn add_now(&mut self, value: T) {
        self.add_point(TimeSeriesPoint::now(value));
    }
    
    /// 获取指定时间范围内的数据点 / Get data points within specified time range
    pub fn get_points_in_range(&self, time_range: &TimeRange) -> Vec<&TimeSeriesPoint<T>> {
        self.points.iter()
            .filter(|point| time_range.contains(point.timestamp))
            .collect()
    }
    
    /// 获取最新的数据点 / Get latest data point
    pub fn get_latest(&self) -> Option<&TimeSeriesPoint<T>> {
        self.points.last()
    }
    
    /// 获取最旧的数据点 / Get oldest data point
    pub fn get_oldest(&self) -> Option<&TimeSeriesPoint<T>> {
        self.points.first()
    }
    
    /// 清空数据点 / Clear data points
    pub fn clear(&mut self) {
        self.points.clear();
    }
    
    /// 获取数据点数量 / Get number of data points
    pub fn len(&self) -> usize {
        self.points.len()
    }
    
    /// 检查是否为空 / Check if empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

impl<T> Default for TimeSeries<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 时间格式化器 / Time Formatter
pub struct TimeFormatter;

impl TimeFormatter {
    /// 格式化持续时间 / Format duration
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let milliseconds = duration.subsec_millis();
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else if seconds > 0 {
            format!("{}s {}ms", seconds, milliseconds)
        } else {
            format!("{}ms", milliseconds)
        }
    }
    
    /// 格式化相对时间 / Format relative time
    pub fn format_relative(timestamp: Timestamp) -> String {
        let now = TimeUtils::now();
        let duration = TimeUtils::duration_between(timestamp, now);
        
        if duration.as_secs() < 60 {
            format!("{}秒前", duration.as_secs())
        } else if duration.as_secs() < 3600 {
            format!("{}分钟前", duration.as_secs() / 60)
        } else if duration.as_secs() < 86400 {
            format!("{}小时前", duration.as_secs() / 3600)
        } else {
            format!("{}天前", duration.as_secs() / 86400)
        }
    }
    
    /// 格式化人类可读的时间 / Format human readable time
    pub fn format_human_readable(timestamp: Timestamp) -> String {
        timestamp.format("%Y年%m月%d日 %H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_time_utils() {
        let now = TimeUtils::now();
        let formatted = TimeUtils::format_iso8601(now);
        let parsed = TimeUtils::parse_iso8601(&formatted).unwrap();
        assert_eq!(now.timestamp(), parsed.timestamp());
    }
    
    #[test]
    fn test_time_range() {
        let start = TimeUtils::now();
        let end = start + chrono::Duration::hours(1);
        let range = TimeRange::new(start, end);
        
        assert!(range.contains(start));
        assert!(range.contains(end));
        assert!(!range.contains(start - chrono::Duration::minutes(1)));
    }
    
    #[test]
    fn test_time_window() {
        let mut window = TimeWindow::fixed(Duration::from_secs(60));
        let current = window.current_window();
        assert_eq!(current.duration(), Duration::from_secs(60));
        
        window.slide_next();
        let next = window.current_window();
        assert!(next.start > current.start);
    }
    
    #[test]
    fn test_time_series() {
        let mut series = TimeSeries::with_max_points(3);
        series.add_now(1);
        series.add_now(2);
        series.add_now(3);
        series.add_now(4); // 应该移除第一个数据点
        
        assert_eq!(series.len(), 3);
        assert_eq!(series.get_latest().unwrap().value, 4);
    }
    
    #[test]
    fn test_time_formatter() {
        let duration = Duration::from_secs(3661);
        let formatted = TimeFormatter::format_duration(duration);
        assert!(formatted.contains("1h"));
    }
}
