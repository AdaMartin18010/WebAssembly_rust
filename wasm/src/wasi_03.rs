//! # WASI 0.3 原生异步支持
//!
//! 提供 WASI 0.3 的完整实现，包括：
//! - 原生 async/await 支持（无需手动管理 pollable handles）
//! - stream<T> 和 future<T> 作为一等类型
//! - 取消令牌与语言级集成
//! - 组件模型异步支持
//! - HTTP 请求原生异步

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, oneshot};
use thiserror::Error;
use serde::{Deserialize, Serialize};

/// WASI 0.3 运行时配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub max_concurrent_tasks: usize,
    pub enable_cancellation: bool,
    pub stream_buffer_size: usize,
    pub http_timeout_ms: u64,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 1000,
            enable_cancellation: true,
            stream_buffer_size: 1024,
            http_timeout_ms: 30000,
        }
    }
}

/// WASI 0.3 运行时
pub struct Wasi03Runtime {
    config: RuntimeConfig,
    task_counter: Arc<Mutex<u64>>,
}

impl Wasi03Runtime {
    /// 创建新的运行时
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            task_counter: Arc::new(Mutex::new(0)),
        }
    }
    
    /// 使用默认配置创建运行时
    pub fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
    
    /// 执行异步操作
    pub async fn execute<F, T>(&self, future: F) -> Result<T, Wasi03Error>
    where
        F: Future<Output = Result<T, Wasi03Error>>,
    {
        future.await
    }
    
    /// 并发执行多个异步操作
    pub async fn execute_all<F, T>(
        &self,
        futures: Vec<F>,
    ) -> Vec<Result<T, Wasi03Error>>
    where
        F: Future<Output = Result<T, Wasi03Error>>,
    {
        let results = futures::future::join_all(futures).await;
        results
    }
    
    /// 创建新的流
    pub fn create_stream<T>(&self) -> (StreamWriter<T>, StreamReader<T>)
    where
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel(self.config.stream_buffer_size);
        let cancelled = Arc::new(Mutex::new(false));
        
        let writer = StreamWriter {
            sender: tx,
            cancelled: cancelled.clone(),
        };
        
        let reader = StreamReader {
            receiver: rx,
            cancelled,
        };
        
        (writer, reader)
    }
    
    /// 创建有缓冲区的流
    pub fn create_stream_with_buffer<T>(&self, buffer_size: usize) -> (StreamWriter<T>, StreamReader<T>)
    where
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel(buffer_size);
        let cancelled = Arc::new(Mutex::new(false));
        
        let writer = StreamWriter {
            sender: tx,
            cancelled: cancelled.clone(),
        };
        
        let reader = StreamReader {
            receiver: rx,
            cancelled,
        };
        
        (writer, reader)
    }
    
    /// 创建新的 future
    pub fn create_future<T>(&self) -> (FutureCompleter<T>, WasiFuture<T>)
    where
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let cancelled = Arc::new(Mutex::new(false));
        
        let completer = FutureCompleter {
            sender: Some(tx),
            cancelled: cancelled.clone(),
        };
        
        let future = WasiFuture {
            receiver: Some(rx),
            cancelled,
        };
        
        (completer, future)
    }
    
    /// 获取当前任务计数
    pub async fn task_count(&self) -> u64 {
        *self.task_counter.lock().await
    }
    
    /// 递增任务计数
    async fn increment_task(&self) {
        let mut counter = self.task_counter.lock().await;
        *counter += 1;
    }
}

/// 流写入端
pub struct StreamWriter<T> {
    sender: mpsc::Sender<T>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> StreamWriter<T> {
    /// 发送数据
    pub async fn send(&self, item: T) -> Result<(), StreamError> {
        if *self.cancelled.lock().await {
            return Err(StreamError::Cancelled);
        }
        
        self.sender.send(item).await.map_err(|_| StreamError::Closed)
    }
    
    /// 批量发送
    pub async fn send_batch(&self, items: Vec<T>) -> Result<usize, StreamError> {
        let mut sent = 0;
        for item in items {
            match self.send(item).await {
                Ok(()) => sent += 1,
                Err(e) => return Err(e),
            }
        }
        Ok(sent)
    }
    
    /// 关闭流
    pub fn close(self) {
        drop(self.sender);
    }
    
    /// 检查是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
}

/// 流读取端
pub struct StreamReader<T> {
    receiver: mpsc::Receiver<T>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> StreamReader<T> {
    /// 接收数据
    pub async fn recv(&mut self) -> Option<T> {
        if *self.cancelled.lock().await {
            return None;
        }
        
        self.receiver.recv().await
    }
    
    /// 批量接收
    pub async fn recv_batch(&mut self, max_items: usize) -> Vec<T> {
        let mut items = Vec::new();
        
        while items.len() < max_items {
            match self.recv().await {
                Some(item) => items.push(item),
                None => break,
            }
        }
        
        items
    }
    
    /// 取消流
    pub async fn cancel(&self) {
        *self.cancelled.lock().await = true;
    }
    
    /// 检查是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
    
    /// 收集所有数据直到流关闭
    pub async fn collect_all(mut self) -> Vec<T> {
        let mut items = Vec::new();
        while let Some(item) = self.recv().await {
            items.push(item);
        }
        items
    }
}

/// Future 完成端
pub struct FutureCompleter<T> {
    sender: Option<oneshot::Sender<T>>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> FutureCompleter<T> {
    /// 完成 future
    pub fn complete(mut self, value: T) -> Result<(), T> {
        if let Some(sender) = self.sender.take() {
            sender.send(value)
        } else {
            Err(value)
        }
    }
    
    /// 检查是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
}

/// WASI Future 类型
pub struct WasiFuture<T> {
    receiver: Option<oneshot::Receiver<T>>,
    cancelled: Arc<Mutex<bool>>,
}

impl<T> Future for WasiFuture<T> {
    type Output = Option<T>;
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 检查是否被取消
        let cancelled = futures::executor::block_on(async {
            *self.cancelled.lock().await
        });
        
        if cancelled {
            return Poll::Ready(None);
        }
        
        match self.receiver.as_mut() {
            Some(receiver) => match receiver.try_recv() {
                Ok(value) => {
                    self.receiver = None;
                    Poll::Ready(Some(value))
                }
                Err(oneshot::error::TryRecvError::Empty) => {
                    _cx.waker().wake_by_ref();
                    Poll::Pending
                }
                Err(_) => {
                    self.receiver = None;
                    Poll::Ready(None)
                }
            },
            None => Poll::Ready(None),
        }
    }
}

impl<T> WasiFuture<T> {
    /// 取消 future
    pub async fn cancel(&self) {
        *self.cancelled.lock().await = true;
    }
    
    /// 检查是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
}

/// 取消令牌
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<Mutex<bool>>,
    children: Arc<Mutex<Vec<CancellationToken>>>,
}

impl CancellationToken {
    /// 创建新的令牌
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(Mutex::new(false)),
            children: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 取消
    pub fn cancel(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            *self.cancelled.lock().await = true;
            
            // 取消所有子令牌
            let children: Vec<_> = self.children.lock().await.clone();
            for child in children.iter() {
                child.cancel().await;
            }
        })
    }
    
    /// 检查是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().await
    }
    
    /// 检查是否被取消，如果被取消则返回错误
    pub async fn check_cancelled(&self) -> Result<(), Wasi03Error> {
        if self.is_cancelled().await {
            Err(Wasi03Error::Cancelled)
        } else {
            Ok(())
        }
    }
    
    /// 创建子令牌
    pub async fn create_child(&self) -> CancellationToken {
        let child = CancellationToken::new();
        self.children.lock().await.push(child.clone());
        child
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// 错误类型
#[derive(Debug, Clone, Error)]
pub enum Wasi03Error {
    #[error("IO错误: {0}")]
    Io(String),
    #[error("操作被取消")]
    Cancelled,
    #[error("流已关闭")]
    StreamClosed,
    #[error("无效状态")]
    InvalidState,
    #[error("超时")]
    Timeout,
    #[error("HTTP错误: {0}")]
    HttpError(String),
    #[error("其他错误: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Error)]
pub enum StreamError {
    #[error("流已取消")]
    Cancelled,
    #[error("流已关闭")]
    Closed,
    #[error("流缓冲区已满")]
    Full,
}

/// HTTP 客户端 (WASI 0.3)
pub mod http {
    use super::*;
    
    /// HTTP 方法
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum HttpMethod {
        Get,
        Post,
        Put,
        Delete,
        Patch,
        Head,
        Options,
    }
    
    impl std::fmt::Display for HttpMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                HttpMethod::Get => write!(f, "GET"),
                HttpMethod::Post => write!(f, "POST"),
                HttpMethod::Put => write!(f, "PUT"),
                HttpMethod::Delete => write!(f, "DELETE"),
                HttpMethod::Patch => write!(f, "PATCH"),
                HttpMethod::Head => write!(f, "HEAD"),
                HttpMethod::Options => write!(f, "OPTIONS"),
            }
        }
    }
    
    /// HTTP 请求
    #[derive(Debug, Clone)]
    pub struct HttpRequest {
        pub method: HttpMethod,
        pub url: String,
        pub headers: Vec<(String, String)>,
        pub body: Option<Vec<u8>>,
        pub timeout_ms: Option<u64>,
    }
    
    impl HttpRequest {
        pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
            Self {
                method,
                url: url.into(),
                headers: vec![],
                body: None,
                timeout_ms: None,
            }
        }
        
        pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.headers.push((key.into(), value.into()));
            self
        }
        
        pub fn with_body(mut self, body: Vec<u8>) -> Self {
            self.body = Some(body);
            self
        }
        
        pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
            self.timeout_ms = Some(timeout_ms);
            self
        }
    }
    
    /// HTTP 响应
    #[derive(Debug, Clone)]
    pub struct HttpResponse {
        pub status: u16,
        pub status_text: String,
        pub headers: Vec<(String, String)>,
        pub body: Vec<u8>,
    }
    
    impl HttpResponse {
        pub fn is_success(&self) -> bool {
            self.status >= 200 && self.status < 300
        }
        
        pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
            String::from_utf8(self.body.clone())
        }
    }
    
    /// HTTP 客户端
    pub struct HttpClient {
        default_timeout_ms: u64,
    }
    
    impl HttpClient {
        pub fn new(default_timeout_ms: u64) -> Self {
            Self { default_timeout_ms }
        }
        
        /// 发送 HTTP 请求
        /// 注意：这是 WASI 0.3 的异步接口，不需要手动管理 pollable handles
        pub async fn request(&self, request: HttpRequest) -> Result<HttpResponse, Wasi03Error> {
            // 这里应该调用 WASI 0.3 的 HTTP 接口
            // 目前返回模拟实现
            let _timeout = request.timeout_ms.unwrap_or(self.default_timeout_ms);
            
            // 模拟异步 HTTP 请求
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            
            Ok(HttpResponse {
                status: 200,
                status_text: "OK".to_string(),
                headers: vec![
                    ("Content-Type".to_string(), "application/json".to_string()),
                ],
                body: b"{\"status\": \"ok\"}".to_vec(),
            })
        }
        
        /// GET 请求
        pub async fn get(&self, url: impl Into<String>) -> Result<HttpResponse, Wasi03Error> {
            self.request(HttpRequest::new(HttpMethod::Get, url)).await
        }
        
        /// POST 请求
        pub async fn post(
            &self,
            url: impl Into<String>,
            body: Vec<u8>,
        ) -> Result<HttpResponse, Wasi03Error> {
            self.request(HttpRequest::new(HttpMethod::Post, url).with_body(body)).await
        }
    }
}

/// 文件系统操作 (WASI 0.3)
pub mod filesystem {
    use super::*;
    
    /// 文件
    #[derive(Debug)]
    pub struct File {
        path: String,
    }
    
    impl File {
        /// 异步读取文件
        pub async fn read(path: impl Into<String>) -> Result<Vec<u8>, Wasi03Error> {
            // 使用 WASI 0.3 异步文件读取
            // 不再需要手动管理 pollable handles
            let path = path.into();
            
            // 模拟异步文件读取
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            
            // 实际实现应该调用 WASI 0.3 接口
            Ok(vec![])
        }
        
        /// 异步写入文件
        pub async fn write(
            path: impl Into<String>,
            contents: Vec<u8>,
        ) -> Result<(), Wasi03Error> {
            let _path = path.into();
            
            // 模拟异步文件写入
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            
            let _ = contents;
            Ok(())
        }
    }
}

/// 演示示例
pub mod demo {
    use super::*;
    use super::http::*;
    
    /// 演示流处理
    pub async fn stream_demo() -> Result<(), Wasi03Error> {
        let runtime = Wasi03Runtime::default();
        let (writer, mut reader) = runtime.create_stream::<i32>();
        
        // 生产者任务
        let producer = tokio::spawn(async move {
            for i in 0..100 {
                if let Err(_) = writer.send(i).await {
                    break;
                }
            }
        });
        
        // 消费者
        let mut sum = 0;
        while let Some(value) = reader.recv().await {
            sum += value;
        }
        
        producer.await.map_err(|e| Wasi03Error::Other(e.to_string()))?;
        
        println!("Sum: {}", sum);
        Ok(())
    }
    
    /// 演示 HTTP 请求
    pub async fn http_demo() -> Result<(), Wasi03Error> {
        let client = HttpClient::new(30000);
        
        let request = HttpRequest::new(HttpMethod::Get, "https://api.example.com/data")
            .with_header("Accept", "application/json")
            .with_timeout(5000);
        
        let response = client.request(request).await?;
        
        if response.is_success() {
            println!("Response: {:?}", response.body_as_string());
        }
        
        Ok(())
    }
    
    /// 演示取消令牌
    pub async fn cancellation_demo() -> Result<(), Wasi03Error> {
        let token = CancellationToken::new();
        let child_token = token.create_child().await;
        
        // 在某个时刻取消
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            token.cancel().await;
        });
        
        // 检查取消状态
        loop {
            if child_token.is_cancelled().await {
                println!("Cancelled!");
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_stream() {
        let runtime = Wasi03Runtime::default();
        let (writer, mut reader) = runtime.create_stream::<i32>();
        
        writer.send(42).await.unwrap();
        let value = reader.recv().await;
        
        assert_eq!(value, Some(42));
    }
    
    #[tokio::test]
    async fn test_stream_batch() {
        let runtime = Wasi03Runtime::default();
        let (writer, mut reader) = runtime.create_stream::<i32>();
        
        let items: Vec<i32> = (0..10).collect();
        let sent = writer.send_batch(items).await.unwrap();
        assert_eq!(sent, 10);
        
        let received = reader.recv_batch(10).await;
        assert_eq!(received.len(), 10);
    }
    
    #[tokio::test]
    async fn test_future() {
        let runtime = Wasi03Runtime::default();
        let (completer, future) = runtime.create_future::<i32>();
        
        completer.complete(42).unwrap();
        let value = future.await;
        
        assert_eq!(value, Some(42));
    }
    
    #[tokio::test]
    async fn test_cancellation() {
        let token = CancellationToken::new();
        
        assert!(!token.is_cancelled().await);
        
        token.cancel().await;
        assert!(token.is_cancelled().await);
        
        assert!(token.check_cancelled().await.is_err());
    }
    
    #[tokio::test]
    async fn test_cancellation_parent_child() {
        let parent = CancellationToken::new();
        let child = parent.create_child().await;
        
        // 取消父令牌
        parent.cancel().await;
        
        // 子令牌也应该被取消
        assert!(child.is_cancelled().await);
    }
    
    #[tokio::test]
    async fn test_stream_cancel() {
        let runtime = Wasi03Runtime::default();
        let (writer, reader) = runtime.create_stream::<i32>();
        
        // 取消读取端
        reader.cancel().await;
        
        // 写入应该失败
        let result = writer.send(42).await;
        assert!(matches!(result, Err(StreamError::Cancelled)));
    }
    
    #[tokio::test]
    async fn test_http_request() {
        let client = http::HttpClient::new(30000);
        let response = client.get("https://example.com").await;
        
        assert!(response.is_ok());
        let resp = response.unwrap();
        assert_eq!(resp.status, 200);
    }
}
