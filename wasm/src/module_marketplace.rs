//! # 模块市场和生态系统
//!
//! 本模块提供了 WebAssembly 模块市场、生态系统管理和模块分发功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{ SystemTime};
use thiserror::Error;

/// 模块市场管理器
/// Module Marketplace Manager
#[derive(Debug)]
pub struct ModuleMarketplaceManager {
    /// 模块注册表
    pub registry: Arc<Mutex<HashMap<String, ModuleEntry>>>,
    /// 用户管理
    pub user_manager: UserManager,
    /// 评分系统
    pub rating_system: RatingSystem,
    /// 下载统计
    pub download_stats: Arc<Mutex<HashMap<String, DownloadStats>>>,
    /// 市场配置
    pub config: MarketplaceConfig,
}

/// 模块条目
/// Module Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEntry {
    /// 模块ID
    pub id: String,
    /// 模块名称
    pub name: String,
    /// 版本
    pub version: String,
    /// 描述
    pub description: String,
    /// 作者
    pub author: String,
    /// 许可证
    pub license: String,
    /// 标签
    pub tags: Vec<String>,
    /// 分类
    pub category: ModuleCategory,
    /// 下载URL
    pub download_url: String,
    /// 文档URL
    pub documentation_url: Option<String>,
    /// 源码URL
    pub source_url: Option<String>,
    /// 创建时间
    pub created_at: SystemTime,
    /// 更新时间
    pub updated_at: SystemTime,
    /// 下载次数
    pub download_count: u64,
    /// 评分
    pub rating: f64,
    /// 评分数量
    pub rating_count: u32,
    /// 模块大小
    pub size: u64,
    /// 依赖关系
    pub dependencies: Vec<ModuleDependency>,
    /// 兼容性
    pub compatibility: CompatibilityInfo,
    /// 安全扫描结果
    pub security_scan: Option<SecurityScanResult>,
}

/// 模块分类
/// Module Category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleCategory {
    /// 数学计算
    Mathematics,
    /// 图像处理
    ImageProcessing,
    /// 机器学习
    MachineLearning,
    /// 加密安全
    Cryptography,
    /// 网络通信
    Networking,
    /// 数据库
    Database,
    /// 工具库
    Utilities,
    /// 游戏引擎
    GameEngine,
    /// 其他
    Other,
}

/// 模块依赖
/// Module Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    /// 依赖模块ID
    pub module_id: String,
    /// 版本要求
    pub version_requirement: String,
    /// 是否必需
    pub required: bool,
}

/// 兼容性信息
/// Compatibility Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// 支持的 WebAssembly 版本
    pub wasm_versions: Vec<String>,
    /// 支持的 Rust 版本
    pub rust_versions: Vec<String>,
    /// 支持的目标平台
    pub target_platforms: Vec<String>,
    /// 最小内存要求
    pub min_memory: u64,
    /// 推荐内存
    pub recommended_memory: u64,
}

/// 安全扫描结果
/// Security Scan Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    /// 扫描时间
    pub scan_time: SystemTime,
    /// 安全级别
    pub security_level: SecurityLevel,
    /// 漏洞列表
    pub vulnerabilities: Vec<Vulnerability>,
    /// 扫描工具
    pub scan_tools: Vec<String>,
}

/// 安全级别
/// Security Level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// 低风险
    Low,
    /// 中等风险
    Medium,
    /// 高风险
    High,
    /// 严重风险
    Critical,
}

/// 漏洞
/// Vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// 漏洞ID
    pub id: String,
    /// 严重程度
    pub severity: SecurityLevel,
    /// 描述
    pub description: String,
    /// CVE编号
    pub cve_id: Option<String>,
    /// 修复建议
    pub fix_suggestion: Option<String>,
}

/// 用户管理器
/// User Manager
#[derive(Debug)]
pub struct UserManager {
    /// 用户存储
    pub users: Arc<Mutex<HashMap<String, User>>>,
    /// 权限管理
    pub permission_manager: PermissionManager,
}

/// 用户
/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub id: String,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后登录时间
    pub last_login: Option<SystemTime>,
    /// 角色
    pub roles: Vec<UserRole>,
    /// 统计信息
    pub statistics: UserStatistics,
}

/// 用户角色
/// User Role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    /// 普通用户
    User,
    /// 开发者
    Developer,
    /// 维护者
    Maintainer,
    /// 管理员
    Administrator,
}

/// 用户统计信息
/// User Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    /// 发布的模块数
    pub published_modules: u32,
    /// 下载的模块数
    pub downloaded_modules: u32,
    /// 评分数量
    pub rating_count: u32,
    /// 贡献分数
    pub contribution_score: u32,
}

/// 权限管理器
/// Permission Manager
#[derive(Debug)]
pub struct PermissionManager {
    /// 权限规则
    pub rules: Arc<Mutex<Vec<PermissionRule>>>,
}

/// 权限规则
/// Permission Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    /// 规则ID
    pub id: String,
    /// 角色
    pub role: UserRole,
    /// 资源
    pub resource: String,
    /// 操作
    pub action: PermissionAction,
    /// 是否允许
    pub allowed: bool,
}

/// 权限操作
/// Permission Action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionAction {
    /// 读取
    Read,
    /// 写入
    Write,
    /// 删除
    Delete,
    /// 发布
    Publish,
    /// 评分
    Rate,
    /// 下载
    Download,
}

/// 评分系统
/// Rating System
#[derive(Debug)]
pub struct RatingSystem {
    /// 评分存储
    pub ratings: Arc<Mutex<HashMap<String, Vec<Rating>>>>,
    /// 评分配置
    pub config: RatingConfig,
}

/// 评分
/// Rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    /// 评分ID
    pub id: String,
    /// 模块ID
    pub module_id: String,
    /// 用户ID
    pub user_id: String,
    /// 评分 (1-5)
    pub score: u8,
    /// 评论
    pub comment: Option<String>,
    /// 评分时间
    pub rated_at: SystemTime,
    /// 有用性评分
    pub helpfulness: Option<u8>,
}

/// 评分配置
/// Rating Configuration
#[derive(Debug, Clone)]
pub struct RatingConfig {
    /// 最小评分
    pub min_score: u8,
    /// 最大评分
    pub max_score: u8,
    /// 是否需要评论
    pub require_comment: bool,
    /// 最大评论长度
    pub max_comment_length: usize,
}

/// 下载统计
/// Download Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    /// 模块ID
    pub module_id: String,
    /// 总下载次数
    pub total_downloads: u64,
    /// 今日下载次数
    pub today_downloads: u64,
    /// 本周下载次数
    pub week_downloads: u64,
    /// 本月下载次数
    pub month_downloads: u64,
    /// 最后下载时间
    pub last_download: Option<SystemTime>,
    /// 下载趋势
    pub download_trend: DownloadTrend,
}

/// 下载趋势
/// Download Trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadTrend {
    /// 上升
    Increasing,
    /// 下降
    Decreasing,
    /// 稳定
    Stable,
    /// 波动
    Volatile,
}

/// 市场配置
/// Marketplace Configuration
#[derive(Debug, Clone)]
pub struct MarketplaceConfig {
    /// 是否启用市场
    pub enabled: bool,
    /// 最大模块大小
    pub max_module_size: u64,
    /// 允许的许可证
    pub allowed_licenses: Vec<String>,
    /// 自动安全扫描
    pub auto_security_scan: bool,
    /// 评分权重
    pub rating_weights: RatingWeights,
}

/// 评分权重
/// Rating Weights
#[derive(Debug, Clone)]
pub struct RatingWeights {
    /// 功能完整性权重
    pub functionality_weight: f64,
    /// 性能权重
    pub performance_weight: f64,
    /// 安全性权重
    pub security_weight: f64,
    /// 文档质量权重
    pub documentation_weight: f64,
    /// 易用性权重
    pub usability_weight: f64,
}

impl ModuleMarketplaceManager {
    /// 创建新的模块市场管理器
    pub fn new(config: MarketplaceConfig) -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
            user_manager: UserManager::new(),
            rating_system: RatingSystem::new(),
            download_stats: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// 发布模块
    pub fn publish_module(&self, module: ModuleEntry, user_id: &str) -> Result<String, MarketplaceError> {
        // 检查用户权限
        if !self.user_manager.has_permission(user_id, "module", PermissionAction::Publish) {
            return Err(MarketplaceError::PermissionDenied);
        }

        // 验证模块
        self.validate_module(&module)?;

        // 安全扫描
        if self.config.auto_security_scan {
            let security_scan = self.perform_security_scan(&module)?;
            // 如果安全级别过高，拒绝发布
            if security_scan.security_level >= SecurityLevel::High {
                return Err(MarketplaceError::SecurityRiskTooHigh);
            }
        }

        // 添加到注册表
        let module_id = module.id.clone();
        let mut registry = self.registry.lock().unwrap();
        registry.insert(module_id.clone(), module);

        // 初始化下载统计
        let mut download_stats = self.download_stats.lock().unwrap();
        download_stats.insert(module_id.clone(), DownloadStats {
            module_id: module_id.clone(),
            total_downloads: 0,
            today_downloads: 0,
            week_downloads: 0,
            month_downloads: 0,
            last_download: None,
            download_trend: DownloadTrend::Stable,
        });

        Ok(module_id)
    }

    /// 搜索模块
    pub fn search_modules(&self, query: &SearchQuery) -> Result<Vec<ModuleEntry>, MarketplaceError> {
        let registry = self.registry.lock().unwrap();
        let mut results: Vec<ModuleEntry> = registry.values().cloned().collect();

        // 应用搜索过滤器
        if let Some(category) = &query.category {
            results.retain(|module| module.category == *category);
        }

        if let Some(tags) = &query.tags {
            results.retain(|module| {
                tags.iter().any(|tag| module.tags.contains(tag))
            });
        }

        if let Some(min_rating) = query.min_rating {
            results.retain(|module| module.rating >= min_rating);
        }

        // 排序
        match query.sort_by {
            SortBy::Rating => results.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap()),
            SortBy::Downloads => results.sort_by(|a, b| b.download_count.cmp(&a.download_count)),
            SortBy::Recent => results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)),
            SortBy::Name => results.sort_by(|a, b| a.name.cmp(&b.name)),
        }

        // 分页
        let start = query.page * query.page_size;
        let end = start + query.page_size;
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results.clear();
        }

        Ok(results)
    }

    /// 下载模块
    pub fn download_module(&self, module_id: &str, user_id: &str) -> Result<ModuleEntry, MarketplaceError> {
        // 检查用户权限
        if !self.user_manager.has_permission(user_id, "module", PermissionAction::Download) {
            return Err(MarketplaceError::PermissionDenied);
        }

        // 获取模块
        let mut registry = self.registry.lock().unwrap();
        let module = registry.get_mut(module_id)
            .ok_or_else(|| MarketplaceError::ModuleNotFound)?;

        // 更新下载统计
        module.download_count += 1;
        let mut download_stats = self.download_stats.lock().unwrap();
        if let Some(stats) = download_stats.get_mut(module_id) {
            stats.total_downloads += 1;
            stats.today_downloads += 1;
            stats.week_downloads += 1;
            stats.month_downloads += 1;
            stats.last_download = Some(SystemTime::now());
        }

        Ok(module.clone())
    }

    /// 评分模块
    pub fn rate_module(&self, module_id: &str, user_id: &str, rating: Rating) -> Result<(), MarketplaceError> {
        // 检查用户权限
        if !self.user_manager.has_permission(user_id, "module", PermissionAction::Rate) {
            return Err(MarketplaceError::PermissionDenied);
        }

        // 验证评分
        if rating.score < self.rating_system.config.min_score || 
           rating.score > self.rating_system.config.max_score {
            return Err(MarketplaceError::InvalidRating);
        }

        // 添加评分
        self.rating_system.add_rating(module_id, rating)?;

        // 更新模块评分
        self.update_module_rating(module_id)?;

        Ok(())
    }

    /// 验证模块
    fn validate_module(&self, module: &ModuleEntry) -> Result<(), MarketplaceError> {
        // 检查模块大小
        if module.size > self.config.max_module_size {
            return Err(MarketplaceError::ModuleTooLarge);
        }

        // 检查许可证
        if !self.config.allowed_licenses.contains(&module.license) {
            return Err(MarketplaceError::LicenseNotAllowed);
        }

        // 检查必需字段
        if module.name.is_empty() || module.description.is_empty() {
            return Err(MarketplaceError::InvalidModule);
        }

        Ok(())
    }

    /// 执行安全扫描
    fn perform_security_scan(&self, _module: &ModuleEntry) -> Result<SecurityScanResult, MarketplaceError> {
        // 简化的安全扫描实现
        // 实际应用中应该集成真实的安全扫描工具
        Ok(SecurityScanResult {
            scan_time: SystemTime::now(),
            security_level: SecurityLevel::Low,
            vulnerabilities: Vec::new(),
            scan_tools: vec!["wasmati".to_string(), "custom_scanner".to_string()],
        })
    }

    /// 更新模块评分
    fn update_module_rating(&self, module_id: &str) -> Result<(), MarketplaceError> {
        let ratings = self.rating_system.get_ratings(module_id)?;
        if ratings.is_empty() {
            return Ok(());
        }

        let total_score: f64 = ratings.iter().map(|r| r.score as f64).sum();
        let avg_rating = total_score / ratings.len() as f64;

        let mut registry = self.registry.lock().unwrap();
        if let Some(module) = registry.get_mut(module_id) {
            module.rating = avg_rating;
            module.rating_count = ratings.len() as u32;
        }

        Ok(())
    }
}

/// 搜索查询
/// Search Query
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// 关键词
    pub keywords: Option<String>,
    /// 分类
    pub category: Option<ModuleCategory>,
    /// 标签
    pub tags: Option<Vec<String>>,
    /// 最小评分
    pub min_rating: Option<f64>,
    /// 排序方式
    pub sort_by: SortBy,
    /// 页码
    pub page: usize,
    /// 页面大小
    pub page_size: usize,
}

/// 排序方式
/// Sort By
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    /// 按评分排序
    Rating,
    /// 按下载量排序
    Downloads,
    /// 按更新时间排序
    Recent,
    /// 按名称排序
    Name,
}

impl UserManager {
    /// 创建新的用户管理器
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            permission_manager: PermissionManager::new(),
        }
    }

    /// 检查用户权限
    pub fn has_permission(&self, user_id: &str, resource: &str, action: PermissionAction) -> bool {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(user_id) {
            self.permission_manager.check_permission(&user.roles, resource, action)
        } else {
            false
        }
    }
}

impl PermissionManager {
    /// 创建新的权限管理器
    pub fn new() -> Self {
        Self {
            rules: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 检查权限
    pub fn check_permission(&self, roles: &[UserRole], resource: &str, action: PermissionAction) -> bool {
        let rules = self.rules.lock().unwrap();
        for role in roles {
            for rule in rules.iter() {
                if rule.role == *role && rule.resource == resource && rule.action == action {
                    return rule.allowed;
                }
            }
        }
        false
    }
}

impl RatingSystem {
    /// 创建新的评分系统
    pub fn new() -> Self {
        Self {
            ratings: Arc::new(Mutex::new(HashMap::new())),
            config: RatingConfig {
                min_score: 1,
                max_score: 5,
                require_comment: false,
                max_comment_length: 1000,
            },
        }
    }

    /// 添加评分
    pub fn add_rating(&self, module_id: &str, rating: Rating) -> Result<(), MarketplaceError> {
        let mut ratings = self.ratings.lock().unwrap();
        let module_ratings = ratings.entry(module_id.to_string()).or_insert_with(Vec::new);
        module_ratings.push(rating);
        Ok(())
    }

    /// 获取评分
    pub fn get_ratings(&self, module_id: &str) -> Result<Vec<Rating>, MarketplaceError> {
        let ratings = self.ratings.lock().unwrap();
        Ok(ratings.get(module_id).cloned().unwrap_or_default())
    }
}

/// 错误类型定义
/// Error Type Definitions

#[derive(Debug, Error)]
pub enum MarketplaceError {
    /// 模块未找到
    #[error("模块未找到")]
    ModuleNotFound,
    /// 权限被拒绝
    #[error("权限被拒绝")]
    PermissionDenied,
    /// 模块过大
    #[error("模块过大")]
    ModuleTooLarge,
    /// 许可证不允许
    #[error("许可证不允许")]
    LicenseNotAllowed,
    /// 无效模块
    #[error("无效模块")]
    InvalidModule,
    /// 安全风险过高
    #[error("安全风险过高")]
    SecurityRiskTooHigh,
    /// 无效评分
    #[error("无效评分")]
    InvalidRating,
    /// 用户未找到
    #[error("用户未找到")]
    UserNotFound,
}
