// ============================================================================
// 异步运行时 - Tokio 基础
// ============================================================================
//
// Tokio 是 Rust 最流行的异步运行时。
//
// 主要特点：
// 1. 异步 I/O - 高性能网络编程
// 2. 任务调度器 - M:N 线程模型
// 3. 定时器 - sleep, interval, timeout
// 4. 同步原语 - Mutex, RwLock, Channel, Barrier, Semaphore
// 5. 信号处理 - Unix 信号
//
// 依赖：tokio = { version = "1", features = ["full"] }

// 注意：这些示例需要在 Cargo.toml 中添加 tokio 依赖才能运行
// [dependencies]
// tokio = { version = "1", features = ["full"] }

// ============================================================================
// 示例 1: 基本 async 函数
// ============================================================================
/*
async fn example1_basic_async() {
    println!("开始异步任务");

    // async 关键字定义异步函数
    async say_hello() {
        println!("你好！");
    }

    // 等待异步任务完成
    say_hello().await;
    println!("异步任务完成");
}
*/

// ============================================================================
// 示例 2: 使用 Tokio 运行时
// ============================================================================
/*
#[tokio::main]
async fn example2_tokio_main() {
    println!("Tokio 运行时已启动");

    // tokio::main 宏自动创建运行时
    // 等效于：
    // fn main() {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(async {
    //         // async 代码
    //     });
    // }
}
*/

// ============================================================================
// 示例 3: 创建任务（tokio::spawn）
// ============================================================================
/*
#[tokio::main]
async fn example3_spawn() {
    println!("主任务");

    // 创建新任务（并发执行）
    let task = tokio::spawn(async {
        println!("任务 1 开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("任务 1 完成");
    });

    let task2 = tokio::spawn(async {
        println!("任务 2 开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("任务 2 完成");
    });

    println!("主任务继续");

    // 等待任务完成
    task.await.unwrap();
    task2.await.unwrap();

    println!("所有任务完成");
}
*/

// ============================================================================
// 示例 4: 异步定时器
// ============================================================================
/*
#[tokio::main]
async fn example4_timer() {
    use tokio::time::{sleep, Duration};

    println!("开始计时");

    // sleep - 异步等待
    sleep(Duration::from_secs(1)).await;
    println!("1 秒后");

    // interval - 周期性执行
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for i in 0..3 {
        interval.tick().await;
        println!("定时器 tick {}", i + 1);
    }

    // timeout - 超时控制
    match tokio::time::timeout(
        Duration::from_secs(2),
        sleep(Duration::from_secs(3))
    ).await {
        Ok(_) => println!("任务完成"),
        Err(_) => println!("任务超时"),
    }
}
*/

// ============================================================================
// 示例 5: 异步通道
// ============================================================================
/*
use tokio::sync::mpsc;

#[tokio::main]
async fn example5_channel() {
    let (tx, mut rx) = mpsc::channel(100);

    // 发送者任务
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            println!("发送: {}", i);
        }
    });

    // 接收者任务
    while let Some(msg) = rx.recv().await {
        println!("接收: {}", msg);
    }

    println!("通道关闭");
}
*/

// ============================================================================
// 示例 6: 异步 Mutex
// ============================================================================
/*
use tokio::sync::Mutex;

#[tokio::main]
async fn example6_async_mutex() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
            println!("计数: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("最终计数: {}", *data.lock().await);
}
*/

// ============================================================================
// 示例 7: Join! - 并发执行多个任务
// ============================================================================
/*
use tokio::time::{sleep, Duration};

async fn task1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    1
}

async fn task2() -> i32 {
    sleep(Duration::from_secs(2)).await;
    2
}

#[tokio::main]
async fn example7_join() {
    // 并发执行多个任务
    let (result1, result2) = tokio::join!(task1(), task2());

    println!("结果: {} {}", result1, result2);
}
*/

// ============================================================================
// 示例 8: Select! - 等待多个任务中的任意一个
// ============================================================================
/*
use tokio::sync::mpsc;

#[tokio::main]
async fn example8_select() {
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);

    // 两个发送者
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx1.send("来自通道 1").await.unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx2.send("来自通道 2").await.unwrap();
    });

    // select! 等待任意一个
    tokio::select! {
        msg = rx1.recv() => {
            println!("通道 1 收到: {:?}", msg);
        }
        msg = rx2.recv() => {
            println!("通道 2 收到: {:?}", msg);
        }
    }
}
*/

// ============================================================================
// 示例 9: 异步文件 I/O
// ============================================================================
/*
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn example9_async_io() -> io::Result<()> {
    // 异步写入文件
    let mut file = File::create("async_test.txt").await?;
    file.write_all(b"异步文件内容").await?;
    file.flush().await?;

    // 异步读取文件
    let mut content = String::new();
    let mut file = File::open("async_test.txt").await?;
    file.read_to_string(&mut content).await?;

    println!("内容: {}", content);

    // 清理
    fs::remove_file("async_test.txt").await?;

    Ok(())
}
*/

// ============================================================================
// 示例 10: 异步 TCP 服务端
// ============================================================================
/*
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn example10_async_tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("异步 TCP 服务端监听在: 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("新连接: {}", addr);

        // 为每个连接创建新任务
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("收到: {}", message);

                    socket.write_all(b"异步响应").await.unwrap();
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                }
            }
        });
    }
}
*/

// ============================================================================
// 示例 11: 异步 TCP 客户端
// ============================================================================
/*
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn example11_async_tcp_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("已连接");

    // 发送消息
    stream.write_all(b"你好").await?;

    // 读取响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("响应: {}", response);

    Ok(())
}
*/

// ============================================================================
// 示例 12: Barrier - 同步点
// ============================================================================
/*
use tokio::sync::Barrier;

#[tokio::main]
async fn example12_barrier() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            println!("任务 {} 准备中", i);
            barrier.wait().await;
            println!("任务 {} 继续", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
*/

// ============================================================================
// 示例 13: Semaphore - 限制并发
// ============================================================================
/*
use tokio::sync::Semaphore;

#[tokio::main]
async fn example13_semaphore() {
    // 最多允许 3 个并发
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    for i in 0..10 {
        let semaphore = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            // 获取许可
            let _permit = semaphore.acquire().await.unwrap();
            println!("任务 {} 开始", i);

            // 模拟工作
            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("任务 {} 完成", i);
            // permit 在这里被释放
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
*/

// ============================================================================
// 示例 14: Broadcast - 多消费者
// ============================================================================
/*
use tokio::sync::broadcast;

#[tokio::main]
async fn example14_broadcast() {
    let (tx, mut rx1) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();

    // 发送者
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).unwrap();
            println!("广播: {}", i);
        }
    });

    // 接收者 1
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("接收者 1: {}", msg);
        }
    });

    // 接收者 2
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("接收者 2: {}", msg);
        }
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
}
*/

// ============================================================================
// 示例 15: RwLock - 读写锁
// ============================================================================
/*
use tokio::sync::RwLock;

#[tokio::main]
async fn example15_rwlock() {
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 多个读者
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let r = data.read().await;
            println!("读者 {}: {}", i, *r);
        });
        handles.push(handle);
    }

    // 写者
    let data = Arc::clone(&data);
    let handle = tokio::spawn(async move {
        let mut w = data.write().await;
        *w = 100;
        println!("写者: 更新为 {}", *w);
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }

    println!("最终值: {}", *data.read().await);
}
*/

// ============================================================================
// 示例 16: OnceCell - 初始化一次
// ============================================================================
/*
use tokio::sync::OnceCell;

static CONFIG: OnceCell<String> = OnceCell::const_new();

#[tokio::main]
async fn example16_oncecell() {
    // 多次调用 get_or_init，只初始化一次
    let value = CONFIG.get_or_init(|| async {
        println!("初始化配置...");
        "配置值".to_string()
    }).await;

    println!("配置: {}", value);

    let value2 = CONFIG.get_or_init(|| async {
        "不会执行".to_string()
    }).await;

    println!("配置2: {}", value2);
}
*/

// ============================================================================
// 示例 17: 异步迭代器
// ============================================================================
/*
use tokio_stream::StreamExt;

#[tokio::main]
async fn example17_async_iter() {
    let mut stream = tokio_stream::iter(vec
![1, 2, 3, 4, 5]);

    while let Some(value) = stream.next().await {
        println!("异步迭代: {}", value);
    }
}
*/

// ============================================================================
// 示例 18: 异步超时和重试
// ============================================================================
/*
use tokio::time::{timeout, Duration};

async fn unreliable_operation() -> Result<i32, &'static str> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    if rand::random() {
        Ok(42)
    } else {
        Err("随机失败")
    }
}

async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration
) -> Result<T, E>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
{
    let mut delay = initial_delay;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                println!("尝试 {} 失败，{} 秒后重试", attempt + 1, delay.as_secs());
                tokio::time::sleep(delay).await;
                delay *= 2; // 指数退避
            }
            Err(e) => return Err(e),
        }
    }

    unreachable!()
}

#[tokio::main]
async fn example18_retry() {
    match timeout(Duration::from_secs(5), unreliable_operation()).await {
        Ok(Ok(result)) => println!("成功: {}", result),
        Ok(Err(e)) => println!("失败: {}", e),
        Err(_) => println!("超时"),
    }
}
*/

// ============================================================================
// 示例 19: 异步信号处理（Unix）
// ============================================================================
/*
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn example19_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    println!("等待 SIGTERM 信号...");

    sigterm.recv().await;

    println!("收到 SIGTERM，优雅关闭");
}

#[cfg(windows)]
#[tokio::main]
async fn example19_signal() {
    println!("Unix 信号处理仅在 Linux/Mac 上可用");
}
*/

// ============================================================================
// 示例 20: 异步 HTTP 客户端（简化版）
// ============================================================================
/*
#[tokio::main]
async fn example20_simple_http() -> io::Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect("example.com:80").await?;

    let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request).await?;

    let mut response = vec
![0u8; 4096];
    let n = stream.read(&mut response).await?;

    println!("响应:\n{}", String::from_utf8_lossy(&response[..n]));

    Ok(())
}
*/

// ============================================================================
// 主函数（仅作为展示，实际代码需要取消注释）
// ============================================================================
fn main() {
    println!("=== Tokio 异步运行时示例 ===\n");

    println!("注意: 以下示例需要在 Cargo.toml 中添加 tokio 依赖:");
    println!("  [dependencies]");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!();

    println!("示例 1: 基本 async 函数");
    println!("  使用 async fn 定义异步函数");
    println!("  使用 .await 等待异步任务\n");

    println!("示例 2: 使用 Tokio 运行时");
    println!("  #[tokio::main] 宏自动创建运行时\n");

    println!("示例 3: 创建任务（tokio::spawn）");
    println!("  并发执行多个任务\n");

    println!("示例 4: 异步定时器");
    println!("  sleep, interval, timeout\n");

    println!("示例 5: 异步通道");
    println!("  mpsc::channel 发送和接收消息\n");

    println!("示例 6: 异步 Mutex");
    println!("  保护共享可变状态\n");

    println!("示例 7: Join! - 并发执行多个任务");
    println!("  tokio::join!(task1(), task2())\n");

    println!("示例 8: Select! - 等待多个任务中的任意一个");
    println!("  tokio::select! 宏\n");

    println!("示例 9: 异步文件 I/O");
    println!("  tokio::fs 读写文件\n");

    println!("示例 10: 异步 TCP 服务端");
    println!("  tokio::net::TcpListener\n");

    println!("示例 11: 异步 TCP 客户端");
    println!("  tokio::net::TcpStream\n");

    println!("示例 12: Barrier - 同步点");
    println!("  等待多个任务到达同步点\n");

    println!("示例 13: Semaphore - 限制并发");
    println!("  控制并发任务数量\n");

    println!("示例 14: Broadcast - 多消费者");
    println!("  广播消息给多个接收者\n");

    println!("示例 15: RwLock - 读写锁");
    println!("  多读者单写者\n");

    println!("示例 16: OnceCell - 初始化一次");
    println!("  单次初始化共享状态\n");

    println!("示例 17: 异步迭代器");
    println!("  tokio_stream 处理数据流\n");

    println!("示例 18: 异步超时和重试");
    println!("  超时控制和指数退避\n");

    println!("示例 19: 异步信号处理");
    println!("  Unix 信号处理\n");

    println!("示例 20: 异步 HTTP 客户端");
    println!("  简化的 HTTP 请求\n");

    println!("=== 总结 ===");
    println!("Tokio 特点:");
    println!("  - 异步 I/O 运行时");
    println!("  - M:N 线程模型");
    println!("  - 高性能网络编程");
    println!("  - 丰富的同步原语");
    println!("  - 定时器和超时");
    println!("  - 生态系统完善");
    println!("\n常用宏:");
    println!("  - #[tokio::main]");
    println!("  - #[tokio::test]");
    println!("  - tokio::join!()");
    println!("  - tokio::select!()");
    println!("\n模块:");
    println!("  - tokio::spawn: 创建任务");
    println!("  - tokio::time: 定时器");
    println!("  - tokio::net: 网络 I/O");
    println!("  - tokio::fs: 文件 I/O");
    println!("  - tokio::sync: 同步原语");
    println!("  - tokio::io: 通用 I/O");
}
