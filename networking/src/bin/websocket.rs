// ============================================================================ 
// WebSocket - 实时双向通信
// ============================================================================ 
//
// 本文件演示了一个完整的 WebSocket 流程：
// 1. 启动一个 Echo Server (服务端)。
// 2. 启动一个 Client 连接服务端并发送消息。
// 3. 演示全双工通信效果。
//
// 核心依赖：tokio, tokio-tungstenite, futures-util

use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};

// 定义服务器地址
const SERVER_ADDR: &str = "127.0.0.1:9001";

// ============================================================================ 
// 部分 1: WebSocket 服务端 (Echo Server)
// ============================================================================ 
async fn run_server() {
    let listener = TcpListener::bind(SERVER_ADDR).await.expect("Failed to bind");
    println!("Server: 监听于 ws://{}", SERVER_ADDR);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(async move {
            println!("Server: 接受连接来自 {}", addr);
            handle_connection(stream).await;
            println!("Server: 连接断开 {}", addr);
        });
    }
}

async fn handle_connection(raw_stream: TcpStream) {
    // 将 TCP 流升级为 WebSocket 流
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake");

    // 将流拆分为 发送端(write) 和 接收端(read) 
    let (mut write, mut read) = ws_stream.split();

    // 循环处理接收到的每一条消息
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Server: 收到消息: {}", msg);
                    // Echo: 原样发回给客户端
                    if let Err(e) = write.send(msg).await {
                        eprintln!("Server: 发送失败: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Server: 连接错误: {}", e);
                break;
            }
        }
    }
}

// ============================================================================ 
// 部分 2: WebSocket 客户端
// ============================================================================ 
async fn run_client() {
    // 等待一小会儿确保 Server 已经启动
    sleep(Duration::from_millis(500)).await;

    let url = format!("ws://{}", SERVER_ADDR);
    println!("Client: 正在连接到 {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Client: 连接成功！");

    let (mut write, mut read) = ws_stream.split();

    // 1. 启动一个任务用于接收消息 (并在后台打印)
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => println!("Client: 收到回显 -> {}", msg),
                Err(e) => eprintln!("Client: 接收错误: {}", e),
            }
        }
    });

    // 2. 发送几条测试消息
    let messages = vec![
        "Hello, WebSocket!",
        "Rust is awesome!",
        "Bye bye!",
    ];

    for msg in messages {
        println!("Client: 发送 -> \"{}\"", msg);
        write.send(Message::Text(msg.to_string())).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    // 3. 发送关闭帧
    println!("Client: 发送关闭请求");
    write.close().await.unwrap();

    // 等待接收任务结束 (Server 关闭连接后 read 会返回 None)
    let _ = recv_task.await;
    println!("Client: 任务结束");
}

// ============================================================================ 
// 部分 3: 主函数 (将两者跑在一起)
// ============================================================================ 
#[tokio::main] 
async fn main() {
    println!("=== WebSocket 实战演示 ===");
    println!("本程序将同时运行 Server 和 Client 以演示通信。\n");

    // 在同一个 runtime 中并发运行 server 和 client
    tokio::select! {
        _ = run_server() => {
            eprintln!("Server unexpected exit");
        }
        _ = run_client() => {
            println!("\n=== 演示完成 ===");
            // Client 跑完后，为了让程序退出，我们直接返回
            // (在实际应用中，Server 通常会一直运行)
            println!("(关闭 Server 并退出程序)");
        }
    }
}

// ============================================================================ 
// 附录: 进阶模式 - 广播聊天室 (核心逻辑参考)
// ============================================================================ 
/* 
// 如果你想实现群聊，可以使用 tokio::sync::broadcast
use tokio::sync::broadcast;

async fn chat_server_example() {
    let (tx, _rx) = broadcast::channel::<String>(100);
    
    // ... 在 handle_connection 中 ...
    let mut rx = tx.subscribe();
    
    // 接收消息并广播给所有人
    // tx.send(msg).unwrap();
    
    // 从广播频道接收消息并发送给当前 WebSocket 客户端
    // socket.send(msg).await;
}
*/