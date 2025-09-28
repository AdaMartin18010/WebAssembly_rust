//! # 公共组件模块 / Common Components Module
//!
//! 本模块提供了项目中使用的公共组件和工具函数。
//! This module provides common components and utility functions used throughout the project.

pub mod error;
pub mod performance;
pub mod time;
pub mod config;
pub mod logging;
pub mod serialization;

// 重新导出公共类型
// Re-export common types
pub use error::*;
pub use performance::*;
pub use time::*;
pub use config::*;
pub use logging::*;
pub use serialization::*;
