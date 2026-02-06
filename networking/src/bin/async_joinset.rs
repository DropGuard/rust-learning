use std::error::Error;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn worker(task_id: i32) -> Result<String, String> {
    let delay = (6 - task_id) as u64;
    sleep(Duration::from_millis(delay * 200)).await;

    if task_id == 3 {
        return Err(format!("任务 {} 发生了错误", task_id));
    }

    Ok(format!("任务 {} 完成 (耗时 {}ms)", task_id, delay * 200))
}

async fn demonstrate_batch_processing() {
    // 一个任务失败不会影响其他正在运行的任务。
    let mut set = JoinSet::new();
    for i in 1..=5 {
        set.spawn(worker(i));
    }

    println!("等待任务结果 (按完成顺序):");
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(msg)) => println!("成功: {}", msg),
            Ok(Err(e)) => eprintln!("业务逻辑错误: {}", e),
            Err(e) => eprintln!("任务 Join 错误 (可能是 Panic): {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Tokio JoinSet 示例 ===");
    demonstrate_batch_processing().await;
    Ok(())
}
