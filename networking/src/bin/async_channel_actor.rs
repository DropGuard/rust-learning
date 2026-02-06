use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::task::JoinSet;
#[tokio::main]
async fn main() {
    println!("=== 异步 Channel (Actor模式) 示例 ===");
    actor().await
}
async fn actor() {
    let (tx, rx) = mpsc::channel::<String>(100);
    // 先启动消费者（后台启动）
    let actor_handle = spawn_actor(rx);

    // 直接在函数里 .await 是主线程
    // 主线程是生产者
    supplier(tx).await;
    let final_result = actor_handle.await.unwrap();
    println!("最终结果: {}", final_result);
}
async fn supplier(tx: mpsc::Sender<String>) {
    let mut worker_set = JoinSet::new();
    for i in 0..5 {
        let tx = tx.clone();
        worker_set.spawn(async move {
            let msg = format!(" world {}!", i);
            if tx.send(msg).await.is_err() {
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
}
fn spawn_actor(mut rx: mpsc::Receiver<String>) -> JoinHandle<String> {
    tokio::spawn(async move {
        let mut state = String::from("Hello,");

        while let Some(msg) = rx.recv().await {
            state.push_str(&msg);
            println!("Actor 更新状态: {}", state);
        }

        state
    })
}
