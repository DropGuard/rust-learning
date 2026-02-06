use std::error::Error;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

// 1. 定义一个具体的任务函数，签名即契约
async fn worker(name: &'static str) -> Result<&'static str, String> {
    sleep(Duration::from_millis(100)).await;
    // 因为函数签名限制了返回值，这里编译器完全知道 Ok 代表什么，不需要 ::<>
    Ok(name)
}

async fn demonstrate_dynamic_addition() -> Result<(), Box<dyn Error>> {
    let mut dynamic_set = JoinSet::new();

    // 2. Spawn 的时候直接调用函数，代码意图非常清晰
    dynamic_set.spawn(worker("动态任务 1"));

    if let Some(res) = dynamic_set.join_next().await {
        // res 是 Result<Result<&str, String>, JoinError>
        // 第一层 Result 是 JoinSet 的结果（任务是否panic），第二层是你业务的 Result
        let task_result = res?;
        println!("第一个任务完成: {:?}", task_result);

        // 这里的类型已经被 worker 函数定死了，非常安全
        dynamic_set.spawn(worker("动态任务 2"));
    }

    while let Some(res) = dynamic_set.join_next().await {
        println!("处理剩余任务: {:?}", res?);
    }

    Ok(())
}
#[tokio::main]
async fn main() {
    println!("--- 演示动态添加任务 ---");
    demonstrate_dynamic_addition().await.unwrap();
}
