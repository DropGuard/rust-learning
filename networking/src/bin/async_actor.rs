use tokio::sync::mpsc;
use tokio::task::JoinSet;
use std::error::Error;

#[tokio::main]
async fn main() {
    println!("=== 异步 Channel (Actor模式) 示例 ===");

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // 这个任务独占资源，不需要锁，因为独自串行处理所有消息
    let manager_handle = tokio::spawn(async move {
        let mut shared_data = String::from("Hello,");

        while let Some(msg) = rx.recv().await {
            shared_data.push_str(&msg);
            println!("更新数据: {}", shared_data);
        }
        shared_data
    });

    if let Err(e) = supplier(tx.clone()).await {
        eprintln!("Supplier 错误: {}", e);
    }

    // 关闭发送端，这样接收端会收到 None，退出循环
    drop(tx);

    let final_result = manager_handle.await.unwrap();
    println!("最终结果: {}", final_result);
}

async fn supplier(tx: mpsc::UnboundedSender<String>) -> Result<(), Box<dyn Error>> {
    let mut worker_set = JoinSet::new();
    for i in 0..5 {
        let tx = tx.clone();
        worker_set.spawn(async move {
            let msg = format!(" world {}!", i);
            // 发送消息。这里不需要等待"拿锁"，只要通道没满，瞬间完成。
            // 能不能不.await？
            if tx.send(msg).is_err() {
                eprintln!("接收端挂了");
            }
        });
    }

    // 等待所有工作完成
    while let Some(res) = worker_set.join_next().await {
        if let Err(e) = res {
            eprintln!("工作线程出错: {}", e);
        }
    }

    Ok(())
}