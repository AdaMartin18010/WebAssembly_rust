//! # WASI 0.3 原生异步演示
//!
//! 本示例展示了 WASI 0.3 的异步特性：
//! - 原生 async/await
//! - stream<T> 和 future<T>
//! - 取消令牌
//! - HTTP 请求

use wasm::wasi_03::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║              WASI 0.3 Async Features Demo                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // 1. 流演示
    demo_stream().await?;
    
    // 2. Future 演示
    demo_future().await?;
    
    // 3. 取消令牌演示
    demo_cancellation().await?;
    
    // 4. HTTP 请求演示
    demo_http().await?;
    
    // 5. 并发执行演示
    demo_concurrent().await?;
    
    println!("\n✅ All WASI 0.3 demos completed successfully!");
    Ok(())
}

async fn demo_stream() -> Result<(), Wasi03Error> {
    println!("▶ Demo 1: stream<T> - Native Streaming");
    println!("  WASI 0.3 introduces stream<T> as a first-class type\n");
    
    let runtime = Wasi03Runtime::default();
    let (writer, mut reader) = runtime.create_stream::<i32>();
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            if let Err(e) = writer.send(i).await {
                println!("  Producer error: {:?}", e);
                break;
            }
            println!("  Produced: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        println!("  Producer finished");
    });
    
    // 消费者
    let mut sum = 0;
    while let Some(value) = reader.recv().await {
        sum += value;
        println!("  Consumed: {}, Running sum: {}", value, sum);
    }
    
    producer.await.map_err(|e| Wasi03Error::Other(e.to_string()))?;
    
    println!("  ✅ Final sum: {}", sum);
    println!();
    Ok(())
}

async fn demo_future() -> Result<(), Wasi03Error> {
    println!("▶ Demo 2: future<T> - Native Futures");
    println!("  WASI 0.3 introduces future<T> as a first-class type\n");
    
    let runtime = Wasi03Runtime::default();
    let (completer, future) = runtime.create_future::<String>();
    
    // 异步完成任务
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let result = "Hello from future!".to_string();
        completer.complete(result).unwrap();
        println!("  Future completed");
    });
    
    // 等待结果
    println!("  Waiting for future...");
    if let Some(result) = future.await {
        println!("  ✅ Future result: {}", result);
    }
    
    println!();
    Ok(())
}

async fn demo_cancellation() -> Result<(), Wasi03Error> {
    println!("▶ Demo 3: Cancellation Tokens");
    println!("  WASI 0.3 provides integrated cancellation support\n");
    
    let token = CancellationToken::new();
    let child_token = token.create_child().await;
    
    // 启动一个长时间运行的任务
    let task = tokio::spawn({
        let token = child_token.clone();
        async move {
            for i in 0..100 {
                if token.is_cancelled().await {
                    println!("  Task cancelled at iteration {}", i);
                    return i;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
            100
        }
    });
    
    // 50ms 后取消
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    println!("  Cancelling parent token...");
    token.cancel().await;
    
    let iterations = task.await.unwrap();
    println!("  ✅ Task stopped after {} iterations", iterations);
    println!("  Child token cancelled: {}", child_token.is_cancelled().await);
    
    println!();
    Ok(())
}

async fn demo_http() -> Result<(), Wasi03Error> {
    println!("▶ Demo 4: Native Async HTTP");
    println!("  WASI 0.3 HTTP: No more pollable handles!\n");
    
    let client = http::HttpClient::new(30000);
    
    // GET 请求
    println!("  Sending GET request...");
    let response = client.get("https://api.example.com/data").await?;
    println!("  ✅ Status: {}", response.status);
    println!("  Headers: {:?}", response.headers);
    
    // POST 请求
    println!("\n  Sending POST request...");
    let request = http::HttpRequest::new(http::HttpMethod::Post, "https://api.example.com/data")
        .with_header("Content-Type", "application/json")
        .with_body(b"{\"key\": \"value\"}".to_vec())
        .with_timeout(5000);
    
    let response = client.request(request).await?;
    println!("  ✅ Status: {}", response.status);
    
    if let Ok(body) = response.body_as_string() {
        println!("  Body: {}", body);
    }
    
    println!();
    Ok(())
}

async fn demo_concurrent() -> Result<(), Wasi03Error> {
    println!("▶ Demo 5: Concurrent Execution");
    println!("  Running multiple async tasks concurrently\n");
    
    let runtime = Wasi03Runtime::default();
    
    // 创建多个 future
    let futures: Vec<_> = (0..5)
        .map(|i| {
            let rt = &runtime;
            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(50 * (5 - i))).await;
                println!("  Task {} completed", i);
                Ok::<i32, Wasi03Error>(i * i)
            }
        })
        .collect();
    
    println!("  Running 5 tasks concurrently...");
    let results = runtime.execute_all(futures).await;
    
    println!("  Results:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("    Task {}: {}", i, value),
            Err(e) => println!("    Task {}: Error {:?}", i, e),
        }
    }
    
    println!("\n  ✅ All concurrent tasks completed");
    println!();
    Ok(())
}
