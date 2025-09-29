-- WebAssembly 2.0 + Rust 1.90 数据库初始化脚本
-- PostgreSQL数据库结构和初始数据

-- 创建数据库
CREATE DATABASE wasm_db;
\c wasm_db;

-- 创建用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建WebAssembly模块表
CREATE TABLE wasm_modules (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    version VARCHAR(20) NOT NULL,
    description TEXT,
    wasm_file_path VARCHAR(255) NOT NULL,
    js_file_path VARCHAR(255),
    size_bytes BIGINT,
    hash VARCHAR(64),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, version)
);

-- 创建性能指标表
CREATE TABLE performance_metrics (
    id SERIAL PRIMARY KEY,
    module_id INTEGER REFERENCES wasm_modules(id),
    metric_name VARCHAR(50) NOT NULL,
    metric_value DECIMAL(10,4) NOT NULL,
    execution_time_ms INTEGER,
    memory_usage_mb DECIMAL(8,2),
    cpu_usage_percent DECIMAL(5,2),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建测试结果表
CREATE TABLE test_results (
    id SERIAL PRIMARY KEY,
    module_id INTEGER REFERENCES wasm_modules(id),
    test_name VARCHAR(100) NOT NULL,
    test_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    execution_time_ms INTEGER,
    memory_usage_mb DECIMAL(8,2),
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建部署记录表
CREATE TABLE deployments (
    id SERIAL PRIMARY KEY,
    module_id INTEGER REFERENCES wasm_modules(id),
    environment VARCHAR(50) NOT NULL,
    version VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL,
    deployed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deployed_by INTEGER REFERENCES users(id)
);

-- 创建索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_wasm_modules_name ON wasm_modules(name);
CREATE INDEX idx_wasm_modules_version ON wasm_modules(version);
CREATE INDEX idx_performance_metrics_module_id ON performance_metrics(module_id);
CREATE INDEX idx_performance_metrics_timestamp ON performance_metrics(timestamp);
CREATE INDEX idx_test_results_module_id ON test_results(module_id);
CREATE INDEX idx_test_results_status ON test_results(status);
CREATE INDEX idx_deployments_module_id ON deployments(module_id);
CREATE INDEX idx_deployments_environment ON deployments(environment);

-- 创建触发器函数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 创建触发器
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_wasm_modules_updated_at BEFORE UPDATE ON wasm_modules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 插入初始数据
INSERT INTO users (username, email, password_hash) VALUES
('admin', 'admin@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/8Kz8KzK'),
('developer', 'dev@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/8Kz8KzK'),
('tester', 'test@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/8Kz8KzK');

INSERT INTO wasm_modules (name, version, description, wasm_file_path, js_file_path, size_bytes, hash) VALUES
('basic', '1.0.0', '基础WebAssembly示例', '/pkg/basic/basic_bg.wasm', '/pkg/basic/basic.js', 1024, 'abc123def456'),
('advanced', '1.0.0', '高级WebAssembly示例', '/pkg/advanced/advanced_bg.wasm', '/pkg/advanced/advanced.js', 2048, 'def456ghi789'),
('performance', '1.0.0', '性能测试WebAssembly示例', '/pkg/performance/performance_bg.wasm', '/pkg/performance/performance.js', 1536, 'ghi789jkl012');

INSERT INTO performance_metrics (module_id, metric_name, metric_value, execution_time_ms, memory_usage_mb, cpu_usage_percent) VALUES
(1, 'fibonacci_40', 102334155, 15, 2.5, 12.5),
(1, 'fibonacci_50', 12586269025, 25, 3.2, 18.7),
(2, 'matrix_multiply_100', 1000000, 45, 5.8, 25.3),
(2, 'matrix_multiply_200', 4000000, 180, 12.4, 45.6),
(3, 'sort_1000', 1000, 8, 1.8, 8.9),
(3, 'sort_10000', 10000, 85, 4.2, 22.1);

INSERT INTO test_results (module_id, test_name, test_type, status, execution_time_ms, memory_usage_mb) VALUES
(1, 'fibonacci_test', 'unit', 'passed', 15, 2.5),
(1, 'memory_test', 'integration', 'passed', 5, 1.2),
(2, 'matrix_test', 'unit', 'passed', 45, 5.8),
(2, 'performance_test', 'benchmark', 'passed', 180, 12.4),
(3, 'sort_test', 'unit', 'passed', 8, 1.8),
(3, 'stress_test', 'load', 'passed', 85, 4.2);

INSERT INTO deployments (module_id, environment, version, status, deployed_by) VALUES
(1, 'development', '1.0.0', 'deployed', 1),
(2, 'staging', '1.0.0', 'deployed', 2),
(3, 'production', '1.0.0', 'deployed', 1);

-- 创建视图
CREATE VIEW module_stats AS
SELECT 
    wm.name,
    wm.version,
    wm.size_bytes,
    COUNT(DISTINCT pm.id) as metric_count,
    AVG(pm.execution_time_ms) as avg_execution_time,
    AVG(pm.memory_usage_mb) as avg_memory_usage,
    COUNT(DISTINCT tr.id) as test_count,
    COUNT(DISTINCT CASE WHEN tr.status = 'passed' THEN tr.id END) as passed_tests
FROM wasm_modules wm
LEFT JOIN performance_metrics pm ON wm.id = pm.module_id
LEFT JOIN test_results tr ON wm.id = tr.module_id
GROUP BY wm.id, wm.name, wm.version, wm.size_bytes;

-- 创建存储过程
CREATE OR REPLACE FUNCTION get_module_performance(module_name VARCHAR, days_back INTEGER DEFAULT 7)
RETURNS TABLE (
    metric_name VARCHAR,
    avg_value DECIMAL,
    min_value DECIMAL,
    max_value DECIMAL,
    sample_count BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        pm.metric_name,
        AVG(pm.metric_value) as avg_value,
        MIN(pm.metric_value) as min_value,
        MAX(pm.metric_value) as max_value,
        COUNT(*) as sample_count
    FROM performance_metrics pm
    JOIN wasm_modules wm ON pm.module_id = wm.id
    WHERE wm.name = module_name
    AND pm.timestamp >= CURRENT_TIMESTAMP - INTERVAL '1 day' * days_back
    GROUP BY pm.metric_name
    ORDER BY pm.metric_name;
END;
$$ LANGUAGE plpgsql;

-- 创建权限
GRANT ALL PRIVILEGES ON DATABASE wasm_db TO wasm_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO wasm_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO wasm_user;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO wasm_user;
